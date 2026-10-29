#![allow(dead_code)]
/*
============================================================================
手写优先队列（Priority Queue）
分类：手写数据结构 · 堆与优先队列
操作复杂度：
  push   O(log n)  追加到末尾后上浮
  pop    O(log n)  末尾补根后下沉
  peek   O(1)      根就是当前最优任务
  size/empty O(1)
============================================================================

用途：
  任务调度、事件模拟、带权最短路的取最小边。面试里常接着问一句
  「同优先级怎么保证先来先服务」，这就是下面的稳定化写法。

核心思想：
  和二叉堆一样用 Vec 存完全二叉树，但比较的是二元组 (优先级, 插入序号)：
  优先级大的先出；优先级相同时插入序号小的先出。序号随 push 单调递增，
  于是同优先级天然按入队顺序出队，不需要额外队列。

关键点（不变量）：
  · 比较必须两级：先比 priority，相等再比 seq，且 seq 小的优先。
    只比 priority 时，等值元素的先后由堆的交换决定，顺序不稳定。
  · seq 由队列自己发号，调用方不能自己传，否则稳定性无从保证。
  · 上浮/下沉只走一条根到叶的路径，停的条件是「父不再更优」。
  · pop 后必须先把末尾元素搬到根再下沉，不能留空洞。

手推一遍：
  任务依次入队：(2,a)、(3,b)、(3,c)、(1,d)，序号分别是 0..3。
  1. a 进 -> 根 a。
  2. b 优先级 3 比 a 的 2 大，b 上浮成根。
  3. c 与 b 同级但序号 2 更大，b 仍在上，c 挂在下面。
  4. d 优先级最小，沉在叶子上。
  出队顺序：b(3,#1)、c(3,#2)、a(2,#0)、d(1,#3)，即 b c a d。

复杂度分析：
  时间 O(log n)：每次 push/pop 只沿一条链走，链长不超过树高 log2(n)。
  空间 O(n)：一个任务数组，元素为 (priority, seq, id) 三个 int。
  稳定性：稳定（靠 seq 次关键字），不是排序算法本身稳定。
*/
#[derive(Clone, Copy)]
struct Task {
    priority: i32,
    seq: i32,
    id: i32,
}

struct PriorityQueue {
    data: Vec<Task>,
    next_seq: i32,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            data: Vec::new(),
            next_seq: 0,
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }

    fn clear(&mut self) {
        self.data.clear();
        self.next_seq = 0;
    }

    fn push(&mut self, priority: i32, id: i32) {
        let task = Task {
            priority,
            seq: self.next_seq,
            id,
        };
        self.next_seq += 1;
        self.data.push(task);
        self.sift_up(self.data.len() - 1);
    }

    // 空队列返回 None，调用方不用猜一个默认优先级。
    fn pop(&mut self) -> Option<(i32, i32)> {
        if self.data.is_empty() {
            return None;
        }
        let top = self.data[0];
        let last = self.data.pop().expect("non-empty");
        if !self.data.is_empty() {
            self.data[0] = last;
            self.sift_down(0);
        }
        Some((top.priority, top.id))
    }

    fn peek(&self) -> Option<(i32, i32)> {
        self.data.first().map(|t| (t.priority, t.id))
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if !higher(&self.data[i], &self.data[parent]) {
                break;
            }
            self.data.swap(i, parent);
            i = parent;
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let mut best = i;
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < n && higher(&self.data[left], &self.data[best]) {
                best = left;
            }
            if right < n && higher(&self.data[right], &self.data[best]) {
                best = right;
            }
            // 两个孩子都不更优就停，再往下扫只会破坏堆序。
            if best == i {
                break;
            }
            self.data.swap(i, best);
            i = best;
        }
    }
}

// 先比优先级，再比插入序号：这就是「堆 + 稳定化」的全部秘密。
fn higher(a: &Task, b: &Task) -> bool {
    if a.priority != b.priority {
        return a.priority > b.priority;
    }
    a.seq < b.seq
}

// 小应用：按优先级从高到低输出任务名，同级保持入队顺序。
fn schedule<'a>(priorities: &[i32], names: &[&'a str]) -> Vec<&'a str> {
    let mut queue = PriorityQueue::new();
    for (i, p) in priorities.iter().enumerate() {
        queue.push(*p, i as i32);
    }

    let mut order = Vec::with_capacity(priorities.len());
    while let Some((_, id)) = queue.pop() {
        order.push(names[id as usize]);
    }
    order
}

// 固定种子的 xorshift32：同一个种子永远得到同一串数，失败可复现。
struct Rng(u32);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

fn test_normal() {
    let priorities = [3, 1, 3, 2];
    let names = ["write", "coffee", "review", "test"];
    let got = schedule(&priorities, &names);
    assert_eq!(got, vec!["write", "review", "test", "coffee"]);
    println!("schedule order: {}", got.join(" "));
}

fn test_empty_and_single() {
    let mut queue = PriorityQueue::new();
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.peek(), None);
    assert!(queue.empty() && queue.size() == 0);

    queue.push(5, 100);
    assert_eq!(queue.size(), 1);
    assert_eq!(queue.pop(), Some((5, 100)));
    assert!(queue.empty());
    assert_eq!(queue.pop(), None);

    // 清空后可继续使用，序号从 0 重新发，顺序依然正确。
    queue.push(1, 0);
    queue.push(2, 1);
    assert_eq!(queue.pop(), Some((2, 1)));
    queue.clear();
    assert!(queue.empty() && queue.size() == 0);
}

fn test_stability_on_key_point() {
    // 只比优先级会挂：五个同级任务的顺序必须原样。
    let mut queue = PriorityQueue::new();
    for i in 0..5 {
        queue.push(7, i);
    }
    assert_eq!(queue.size(), 5);
    for i in 0..5 {
        assert_eq!(queue.pop(), Some((7, i)));
    }
    assert!(queue.empty());

    // 交错 push/pop：序号继续递增，后进的同级任务排在后面。
    queue.push(4, 10);
    queue.push(4, 11);
    assert_eq!(queue.pop(), Some((4, 10)));
    queue.push(4, 12);
    assert_eq!(queue.pop(), Some((4, 11)));
    assert_eq!(queue.pop(), Some((4, 12)));
    assert!(queue.empty());
}

fn test_random_priorities() {
    let mut rng = Rng(20240501);
    let n = 120;
    let mut priorities = Vec::with_capacity(n);
    let mut names: Vec<String> = Vec::with_capacity(n);
    for i in 0..n {
        priorities.push((rng.next_u32() % 6) as i32);
        names.push(format!("task-{}", i));
    }
    let name_refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();

    // 期望值：优先级从高到低，同级按入队顺序扫一遍，不用任何排序。
    let mut max_priority = i32::MIN;
    let mut min_priority = i32::MAX;
    for &p in &priorities {
        max_priority = max_priority.max(p);
        min_priority = min_priority.min(p);
    }
    let mut expect = Vec::with_capacity(n);
    let mut p = max_priority;
    while p >= min_priority {
        for (i, &value) in priorities.iter().enumerate() {
            if value == p {
                expect.push(i);
            }
        }
        p -= 1;
    }

    let mut queue = PriorityQueue::new();
    for (i, &value) in priorities.iter().enumerate() {
        queue.push(value, i as i32);
    }
    assert_eq!(queue.size(), n);

    let mut got = Vec::with_capacity(n);
    while let Some((_, id)) = queue.pop() {
        got.push(id as usize);
    }
    assert_eq!(got, expect);

    // 弹出来的优先级必须单调不增，这一条独立于期望序列。
    let mut last = i32::MAX;
    for &id in &got {
        let p = priorities[id];
        assert!(p <= last, "priority must be non-increasing");
        last = p;
    }
    assert!(queue.empty());

    // 名字序列也要对得上：第 k 个出队的名字是 names[got[k]]。
    let order = schedule(&priorities, &name_refs);
    for (k, name) in order.iter().enumerate() {
        assert_eq!(*name, names[got[k]]);
    }
}

fn main() {
    test_normal();
    test_empty_and_single();
    test_stability_on_key_point();
    test_random_priorities();
    println!("all tests passed");
}
