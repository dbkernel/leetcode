/*
============================================================================
双端队列（Deque）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_front  均摊 O(1)  满时 2 倍扩容并重排下标
  push_back   均摊 O(1)  同循环队列的入队
  pop_front   O(1)       只挪 head
  pop_back    O(1)       只改 size，队尾由公式算出
  front       O(1)
  back        O(1)
  empty/size  O(1)
  clear       O(1)
============================================================================

用途：
  两端都要进出的场景：滑动窗口最值（单调队列）、0-1 BFS、回文判断、
  撤销栈、以及工作窃取调度器的本地队列。

核心思想：
  循环队列再加一个「往左长」的能力。队尾仍然是 (head + size - 1)，
  队头插就是把 head 往左挪一格再写；挪的时候要 (head - 1 + capacity)
  再取模，否则下标会变成 -1。于是两端增删都只动下标，不搬元素。

关键点（不变量）：
  · 队首物理下标是 head，队尾物理下标是 (head + size - 1) % capacity；
  · push_front 必须「先左移 head，再写 data[head]」；pop_front 必须
    「先读 data[head]，再右移 head」，两步顺序反了就会写错/读错位置；
  · head 与 size 都是 usize，头部下标写成 (head + capacity - 1) % capacity，
    等价于减一取模，但完全避开了 usize 下溢（debug 模式会直接 panic）；
  · 扩容时按逻辑顺序把 head..head+size-1 搬到新 Vec 的 0..size-1，最后才
    把 head 置 0；容量翻倍后旧的取模关系全部失效。

手推一遍：
  容量 4，push_back 1、2、3、4 后 head=0、size=4，队满。
  1. pop_front 取走 1，head 变 1，size=3，逻辑内容 [2, 3, 4]；
  2. push_front(0)：head = (1 + 4 - 1) % 4 = 0，写 data[0]=0，size 回到 4，
     逻辑内容 [0, 2, 3, 4]，物理上下标 0..3 全占满；
  3. 再 push_back(5)：size==capacity 先扩容到 8，按逻辑顺序搬成
     [0, 2, 3, 4] 占下标 0..3、head 归 0，再写 data[4]=5。

复杂度分析：
  时间：两端插入/删除都是均摊 O(1)；单次触发扩容是 O(n)，但一次扩容至少
    带来 capacity 次入队，摊下来仍是 O(1)。
  空间：O(n)，capacity 最多是 size 的 2 倍，且总是 2 的幂。
  取舍：相对 Vec 的优势是两端 O(1)（Vec 头插是 O(n)），代价是随机访问
    变成一次取模。
*/

#![allow(dead_code)]

const DEQUE_INIT_CAP: usize = 4;

struct Deque {
    data: Vec<i32>,
    head: usize, // 队首物理下标
    size: usize, // 元素个数
}

impl Deque {
    fn new() -> Self {
        Deque {
            data: vec![0; DEQUE_INIT_CAP],
            head: 0,
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }

    // 只为自测观察回绕，不参与逻辑。
    fn head_index(&self) -> usize {
        self.head
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    fn full(&self) -> bool {
        self.size == self.data.len()
    }

    fn grow(&mut self) {
        let old_cap = self.data.len();
        let mut nd = vec![0; old_cap * 2];
        // 用旧 head、旧 capacity 按逻辑顺序搬；搬完才能改动这两个量。
        for i in 0..self.size {
            nd[i] = self.data[(self.head + i) % old_cap];
        }
        self.data = nd;
        self.head = 0;
    }

    fn push_front(&mut self, v: i32) {
        if self.size == self.data.len() {
            self.grow();
        }
        // head 减一再用 usize 取模会下溢，所以写成 + capacity - 1
        let cap = self.data.len();
        self.head = (self.head + cap - 1) % cap;
        self.data[self.head] = v;
        self.size += 1;
    }

    fn push_back(&mut self, v: i32) {
        if self.size == self.data.len() {
            self.grow();
        }
        let idx = (self.head + self.size) % self.data.len();
        self.data[idx] = v;
        self.size += 1;
    }

    // 头删：先读再右移，否则读到的是新 head 上的元素。
    fn pop_front(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let v = self.data[self.head];
        self.head = (self.head + 1) % self.data.len();
        self.size -= 1;
        Some(v)
    }

    // 尾删：只需 size 减一，把那一格「逻辑上」丢掉。
    fn pop_back(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let idx = (self.head + self.size - 1) % self.data.len();
        self.size -= 1;
        Some(self.data[idx])
    }

    fn front(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        Some(self.data[self.head])
    }

    fn back(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let idx = (self.head + self.size - 1) % self.data.len();
        Some(self.data[idx])
    }

    fn clear(&mut self) {
        self.head = 0;
        self.size = 0;
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::new();
        for i in 0..self.size {
            out.push(self.data[(self.head + i) % self.data.len()]);
        }
        out
    }

    fn print(&self) {
        let parts: Vec<String> =
            self.to_vec().iter().map(|x| x.to_string()).collect();
        println!(
            "[{}] size={} cap={} head={}",
            parts.join(", "),
            self.size,
            self.data.len(),
            self.head
        );
    }
}

fn main() {
    // 用例 1：两端交替操作，顺序与两端取值都要对。
    let mut a = Deque::new();
    a.push_back(3);
    a.push_front(2);
    a.push_back(4);
    a.push_front(1);
    assert_eq!(a.to_vec(), vec![1, 2, 3, 4]);
    assert_eq!(a.front(), Some(1));
    assert_eq!(a.back(), Some(4));
    assert_eq!(a.pop_front(), Some(1));
    assert_eq!(a.pop_back(), Some(4));
    assert_eq!(a.to_vec(), vec![2, 3]);
    a.push_front(1);
    a.push_back(4);
    assert_eq!(a.to_vec(), vec![1, 2, 3, 4]);
    a.print();

    // 用例 2：头插到底，跨多次扩容，顺序必须是整体倒序。
    let mut b = Deque::new();
    for i in 0..100 {
        b.push_front(i);
    }
    assert_eq!(b.size(), 100);
    assert_eq!(b.capacity(), 128);
    let want: Vec<i32> = (0..100).rev().collect();
    assert_eq!(b.to_vec(), want);
    assert_eq!(b.front(), Some(99));
    assert_eq!(b.back(), Some(0));

    // 用例 3：关键点用例——head 不为 0 时扩容，两端数据顺序不能乱。
    let mut c = Deque::new();
    for i in 1..=4 {
        c.push_back(i); // 满，head=0
    }
    assert_eq!(c.pop_front(), Some(1));
    assert_eq!(c.pop_front(), Some(2)); // head 走到 2
    c.push_back(5); // 写到下标 0
    c.push_back(6); // 写到下标 1
    assert_eq!(c.to_vec(), vec![3, 4, 5, 6]);
    assert_eq!(c.head_index(), 2);
    assert!(c.full());
    c.push_back(7); // 触发扩容
    assert_eq!(c.capacity(), 8);
    assert_eq!(c.head_index(), 0);
    assert_eq!(c.to_vec(), vec![3, 4, 5, 6, 7]);
    // 扩容后立刻头插：head 必须回绕到 7，而不是下溢成 -1。
    c.push_front(2);
    assert_eq!(c.head_index(), 7);
    assert_eq!(c.to_vec(), vec![2, 3, 4, 5, 6, 7]);
    assert_eq!(c.front(), Some(2));
    assert_eq!(c.back(), Some(7));
    c.print();

    // 用例 4：空队列的每个出口。
    let mut e = Deque::new();
    assert!(e.empty() && !e.full() && e.size() == 0);
    assert_eq!(e.pop_front(), None);
    assert_eq!(e.pop_back(), None);
    assert_eq!(e.front(), None);
    assert_eq!(e.back(), None);
    assert!(e.to_vec().is_empty());
    e.push_front(1);
    assert_eq!(e.front(), Some(1)); // 单元素时两端相同
    assert_eq!(e.back(), Some(1));
    e.clear();
    assert!(e.empty() && e.size() == 0 && e.head_index() == 0);
    e.push_back(2); // clear 之后还能继续用
    assert_eq!(e.to_vec(), vec![2]);

    // 用例 5：长操作序列对拍，两端交替进出，逐步核对 size 与内容。
    let mut q = Deque::new();
    let mut model: Vec<i32> = Vec::new(); // 仅用于校验：参考记录
    for step in 0..300 {
        match step % 4 {
            0 => {
                q.push_back(step);
                model.push(step);
            }
            1 => {
                q.push_front(step);
                model.insert(0, step);
            }
            2 => {
                if !model.is_empty() {
                    let want = model.remove(0);
                    assert_eq!(q.pop_front(), Some(want));
                }
            }
            _ => {
                if !model.is_empty() {
                    let want = model.pop().unwrap();
                    assert_eq!(q.pop_back(), Some(want));
                }
            }
        }
        assert_eq!(q.size(), model.len());
        if step % 37 == 0 {
            assert_eq!(q.to_vec(), model);
        }
    }
    assert_eq!(q.to_vec(), model);

    println!("all tests passed");
}
