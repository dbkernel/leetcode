#![allow(dead_code)]
/*
============================================================================
手写二叉堆（Binary Heap）
分类：手写数据结构 · 堆与优先队列
操作复杂度：
  push   O(log n)  新元素放末尾后上浮，最多走一条树高
  pop    O(log n)  末尾元素补到根后下沉，最多走一条树高
  peek   O(1)      根就是最值
  build  O(n)      自底向上 heapify，不是逐个 push 的 O(n log n)
============================================================================

用途：
  需要反复取最值的场景：优先队列、TopK、Dijkstra、合并 K 个有序链表。
  面试常要求手写堆，而不是直接调用 BinaryHeap。

核心思想：
  把完全二叉树按层序塞进 Vec，父子关系用下标算出来，省掉指针。
  push 只在末尾追加再上浮；pop 把末尾元素搬到根再下沉，两者都只沿着
  一条从根到叶的路径比较，所以是 O(log n)。

关键点（不变量）：
  · 大根堆：每个父节点 >= 它的两个子节点；小根堆方向相反。
  · 0-based 下标：parent = (i-1)/2，children = 2*i+1、2*i+2。
  · 上浮看到父节点已满足堆序就停；下沉看到两个孩子都不更优就停。
  · build 必须从最后一个非叶节点 n/2-1 倒着下沉，才是 O(n)。

手推一遍：
  以小根堆为例，依次 push 3、1、4：
  1. push 3 -> [3]。
  2. push 1 -> [3,1]，1 比父 3 小，交换 -> [1,3]。
  3. push 4 -> [1,3,4]，4 比父 3 大，停。
  接着 pop：根 1 取出，末尾 4 补到根 -> [4,3]，
  4 与唯一的（较小的）孩子 3 交换 -> [3,4]。

复杂度分析：
  时间 O(log n)：push/pop 各沿树高走一条链，树高 log2(n)。
  空间 O(n)：一个 Vec 存 n 个元素，不额外拷贝。
  稳定性：不稳定，等值元素的相对次序由交换决定。
*/

struct BinaryHeap {
    data: Vec<i32>,
    is_max: bool,
}

impl BinaryHeap {
    fn new(is_max: bool) -> Self {
        BinaryHeap {
            data: Vec::new(),
            is_max,
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
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    // 空堆返回 None，调用方不会拿到一个假值。
    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let top = self.data[0];
        let last = self.data.pop().expect("non-empty");
        if !self.data.is_empty() {
            self.data[0] = last;
            self.sift_down(0);
        }
        Some(top)
    }

    fn peek(&self) -> Option<i32> {
        self.data.first().copied()
    }

    // 自底向上建堆：叶子不用下沉，从最后一个非叶节点往回扫，
    // 每个节点下沉的高度之和是 O(n)，比逐个 push 的 O(n log n) 更快。
    fn build(&mut self, values: &[i32]) {
        self.data.clear();
        self.data.extend_from_slice(values);
        let n = self.data.len();
        // n/2 是第一个叶子的下标；这里从它前一个节点往前扫。
        let mut i = n / 2;
        while i > 0 {
            i -= 1;
            self.sift_down(i);
        }
    }

    // 仅供自测：导出内部切片，用于直接检查不变量。
    fn as_slice(&self) -> &[i32] {
        &self.data
    }

    // 大根堆里 a 该在上面；小根堆里 b 该在上面。
    fn higher(&self, a: i32, b: i32) -> bool {
        if self.is_max {
            a > b
        } else {
            a < b
        }
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if !self.higher(self.data[i], self.data[parent]) {
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
            if left < n && self.higher(self.data[left], self.data[best]) {
                best = left;
            }
            if right < n && self.higher(self.data[right], self.data[best]) {
                best = right;
            }
            // best 还是自己，说明两个孩子都不比它更优，再走就破坏堆序。
            if best == i {
                break;
            }
            self.data.swap(i, best);
            i = best;
        }
    }
}

// 孩子比父亲更优先（大根堆更大、小根堆更小）就是违反堆序。
fn violates(child: i32, parent: i32, is_max: bool) -> bool {
    if is_max {
        child > parent
    } else {
        child < parent
    }
}

// 直接扫一遍切片，验证「父节点与两个孩子」的堆序不变量。
fn heap_ok(a: &[i32], is_max: bool) -> bool {
    let n = a.len();
    for i in 0..n {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        if left < n && violates(a[left], a[i], is_max) {
            return false;
        }
        if right < n && violates(a[right], a[i], is_max) {
            return false;
        }
    }
    true
}

fn drain(heap: &mut BinaryHeap) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(v) = heap.pop() {
        out.push(v);
    }
    out
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

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        let span = (hi - lo + 1) as u32;
        lo + (self.next_u32() % span) as i32
    }
}

fn test_normal() {
    let mut heap = BinaryHeap::new(true);
    for v in [5, 3, 8, 1, 9, 2] {
        heap.push(v);
    }
    assert_eq!(heap.size(), 6);

    assert_eq!(heap.peek(), Some(9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.size(), 4);

    heap.push(7);
    assert_eq!(drain(&mut heap), vec![7, 5, 3, 2, 1]);
    assert!(heap.empty());

    let mut small = BinaryHeap::new(false);
    for v in [5, 3, 8, 1, 9, 2] {
        small.push(v);
    }
    assert_eq!(drain(&mut small), vec![1, 2, 3, 5, 8, 9]);
}

fn test_empty_and_single() {
    let mut heap = BinaryHeap::new(true);
    assert_eq!(heap.pop(), None);
    assert_eq!(heap.peek(), None);
    assert!(heap.empty() && heap.size() == 0);

    heap.push(-7);
    assert_eq!(heap.size(), 1);
    assert_eq!(heap.pop(), Some(-7));
    assert!(heap.empty());
    assert_eq!(heap.pop(), None);
}

fn test_build_property() {
    let mut heap = BinaryHeap::new(true);
    heap.build(&[7, 2, 9, 4, 4, 1, 8]);
    assert_eq!(heap.size(), 7);
    assert!(heap_ok(heap.as_slice(), true));

    // 重复 build 必须覆盖旧内容，不能把上一次的元素留在后面。
    heap.build(&[3, 1, 2]);
    assert_eq!(heap.size(), 3);
    assert!(heap_ok(heap.as_slice(), true));
    assert_eq!(drain(&mut heap), vec![3, 2, 1]);

    // 空切片 build 也不能越界（n/2 = 0，循环一次都不进）。
    heap.build(&[]);
    assert!(heap.empty() && heap_ok(heap.as_slice(), true));
}

fn test_duplicates() {
    let mut small = BinaryHeap::new(false);
    for _ in 0..6 {
        small.push(6);
    }
    assert_eq!(small.size(), 6);
    assert_eq!(drain(&mut small), vec![6, 6, 6, 6, 6, 6]);

    let mut many = BinaryHeap::new(true);
    many.build(&[4; 16]);
    assert!(heap_ok(many.as_slice(), true));
    assert_eq!(drain(&mut many).len(), 16);
}

fn test_random_cross_check() {
    let mut rng = Rng(20240501);
    for round in 0..5 {
        let n = 20 + round * 37;
        let mut raw = Vec::new();
        let mut big = BinaryHeap::new(true);
        let mut small = BinaryHeap::new(false);
        for _ in 0..n {
            let v = rng.range(-1000, 1000);
            raw.push(v);
            big.push(v);
            small.push(v);
        }
        assert_eq!(big.size(), n);
        assert_eq!(small.size(), n);

        // 仅用于校验：实现部分没有使用内置排序。
        raw.sort();
        let mut desc = raw.clone();
        desc.reverse();
        assert_eq!(drain(&mut big), desc);
        assert_eq!(drain(&mut small), raw);
        assert!(big.empty() && small.empty());
    }

    // 交错 push/pop：size 始终等于有效元素个数。
    let mut heap = BinaryHeap::new(true);
    heap.push(4);
    heap.push(10);
    assert_eq!(heap.pop(), Some(10));
    heap.push(1);
    heap.push(7);
    assert_eq!(heap.size(), 3);
    assert_eq!(heap.pop(), Some(7));
    heap.clear();
    assert!(heap.empty() && heap.size() == 0);
}

fn main() {
    test_normal();
    test_empty_and_single();
    test_build_property();
    test_duplicates();
    test_random_cross_check();
    println!("all tests passed");
}
