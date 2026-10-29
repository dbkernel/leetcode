/*
============================================================================
动态数组（Dynamic Array）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_back  均摊 O(1)  满时容量翻倍，n 次 push 累计搬移小于 2n
  pop_back   O(1)       缩容只在 size <= capacity/4 时发生
  insert     O(n)       index 及其后的元素整体右移一位
  erase      O(n)       index 之后的元素整体左移一位
  at         O(1)       带越界检查，越界返回 None
  find       O(n)       线性扫描，返回下标，找不到返回 None
  clear      O(1)       把 size 置 0，必要时顺带缩容
============================================================================

用途：
  手写一切「长度可变」容器的底座：栈、队列、哈希表的桶都能建在它上面。
  面试常以「实现一个 vector」或「这行为什么退化到 O(n^2)」的形式出现。

核心思想：
  一块预分配的 Vec + size / capacity 两个量。Vec 的 len 当 capacity 用，
  size 之前的格子才是有效元素。写满就换成两倍长；空出四分之三就减半。

关键点（不变量）：
  · 0 <= size <= capacity，capacity 从 4 开始，每次只翻倍或减半，
    所以它永远是 2 的幂；
  · push_back 必须先扩容再写 data[size]，顺序反了就是越界写；
  · 缩容条件是 size <= capacity/4 且 capacity >= 8。若在 size 刚低于
    capacity/2 时就减半，同一个位置反复 push/pop 会来回搬移；
  · insert 从后往前搬，erase 从前往后搬，方向反了会覆盖未读元素。

手推一遍：
  以 capacity=4、空数组开始连续 push 0..4：
  1. push 0..3 一路顺序写，写完 size=4，正好等于 capacity；
  2. push 4 时 size==capacity，先换 8 长的 Vec 并拷走 4 个元素；
  3. 累计搬移 4 次，总写入 5 次，摊到每次 push 不到 1 次；
  4. 之后 pop 到 size=2（== 8/4）时减半回 4，此时占用率 1/2，
     再 push 两次才会再扩容，不会在两倍临界点上抖动。

复杂度分析：
  时间：单次 push_back 最坏 O(n)（扩容搬移），但 n 次连续 push 的总搬移
    是 4+8+16+... < 2n，把这份代价摊到每一次 push 上就是均摊 O(1)。
    insert/erase 要整体挪动，是 O(n)；at 是 O(1)。
  空间：O(n)，capacity 至多是 size 的 2 倍，浪费是常数倍。
  为什么不是 1.5 倍：1.5 倍同样能保证均摊 O(1)（总搬移仍是 n 的常数倍），
    只是常数更大、更省内存；选 2 倍纯粹是加法比乘法好写，不影响正确性。
*/

#![allow(dead_code)]

const INIT_CAP: usize = 4;

struct DynamicArray {
    data: Vec<i32>, // 长度当 capacity；vec![0; n] 已把槽位初始化好
    size: usize,
}

impl DynamicArray {
    fn new() -> Self {
        DynamicArray {
            data: vec![0; INIT_CAP],
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    fn grow(&mut self) {
        let mut nd = vec![0; self.data.len() * 2];
        nd[..self.size].copy_from_slice(&self.data[..self.size]);
        self.data = nd;
    }

    // 一次可能连降两级（比如从 16 掉到 4），所以用 while 而不是 if。
    fn shrink(&mut self) {
        while self.data.len() >= 8 && self.size <= self.data.len() / 4 {
            let mut nd = vec![0; self.data.len() / 2];
            nd[..self.size].copy_from_slice(&self.data[..self.size]);
            self.data = nd;
        }
    }

    fn push_back(&mut self, v: i32) {
        if self.size == self.data.len() {
            self.grow(); // 先扩容，再写入
        }
        self.data[self.size] = v;
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.shrink();
        Some(self.data[self.size])
    }

    fn insert(&mut self, index: usize, v: i32) -> bool {
        // index == size 等价于尾插，仍然合法（usize 天然排除负下标）。
        if index > self.size {
            return false;
        }
        if self.size == self.data.len() {
            self.grow();
        }
        // 从后往前挪一位：i 最小取到 index+1，所以 i-1 不会下溢。
        for i in (index + 1..=self.size).rev() {
            self.data[i] = self.data[i - 1];
        }
        self.data[index] = v;
        self.size += 1;
        true
    }

    fn erase(&mut self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }
        // 从前往后挪；上界 size-1 保证 i+1 不越界。
        for i in index..self.size - 1 {
            self.data[i] = self.data[i + 1];
        }
        self.size -= 1;
        self.shrink();
        true
    }

    fn at(&self, index: usize) -> Option<i32> {
        if index >= self.size {
            return None;
        }
        Some(self.data[index])
    }

    fn find(&self, v: i32) -> Option<usize> {
        for i in 0..self.size {
            if self.data[i] == v {
                return Some(i);
            }
        }
        None
    }

    fn clear(&mut self) {
        self.size = 0;
        self.shrink(); // 清空后通常不会马上塞满，顺手把内存还回去
    }

    fn to_vec(&self) -> Vec<i32> {
        self.data[..self.size].to_vec()
    }

    fn print(&self) {
        let parts: Vec<String> = self.data[..self.size]
            .iter()
            .map(|x| x.to_string())
            .collect();
        println!("[{}]", parts.join(", "));
    }
}

fn main() {
    // 用例 1：连续 push 跨多次扩容，容量按 4/8/16/... 翻倍。
    let mut a = DynamicArray::new();
    assert!(a.capacity() == 4 && a.empty());
    for i in 0..100 {
        a.push_back(i);
    }
    assert_eq!(a.size(), 100);
    assert_eq!(a.capacity(), 128); // 4 -> 8 -> 16 -> 32 -> 64 -> 128
    assert_eq!(a.at(0), Some(0));
    assert_eq!(a.at(99), Some(99));
    assert_eq!(a.at(50), Some(50));
    assert_eq!(a.at(100), None);
    assert_eq!(a.find(42), Some(42));
    assert_eq!(a.find(-1), None);
    a.print();

    // 用例 2：insert 的头、尾、中间与越界。
    let mut b = DynamicArray::new();
    b.push_back(1);
    b.push_back(2);
    b.push_back(3);
    assert!(b.insert(0, 0)); // 头插
    assert!(b.insert(4, 4)); // 尾插，index == size
    assert!(b.insert(2, 99)); // 中间插
    assert!(!b.insert(99, 7));
    assert_eq!(b.to_vec(), vec![0, 1, 99, 2, 3, 4]);
    assert_eq!(b.size(), 6);

    // 用例 3：erase 的头、尾、中间与越界。
    assert!(b.erase(0));
    assert_eq!(b.to_vec(), vec![1, 99, 2, 3, 4]);
    assert!(b.erase(4)); // 删尾（此时 size=4）
    assert_eq!(b.to_vec(), vec![1, 99, 2, 3]);
    assert!(b.erase(1));
    assert_eq!(b.to_vec(), vec![1, 2, 3]);
    assert!(!b.erase(3)); // 下标 == size 已越界
    assert_eq!(b.size(), 3);

    // 用例 4：缩容后再 push，数据不能丢、容量要能重新长回来。
    let mut c = DynamicArray::new();
    for i in 0..16 {
        c.push_back(i);
    }
    assert_eq!(c.capacity(), 16);
    for _ in 0..12 {
        assert!(c.pop_back().is_some());
    }
    assert_eq!(c.size(), 4);
    assert_eq!(c.capacity(), 8); // 16 -> 8（size==4 命中 4<=16/4）
    assert_eq!(c.to_vec(), vec![0, 1, 2, 3]);
    for i in 0..4 {
        c.push_back(100 + i);
    }
    assert_eq!(c.size(), 8);
    assert_eq!(c.capacity(), 8); // 占用率回到 1/2 不扩容
    c.push_back(200); // 第 9 个才扩容
    assert_eq!(c.size(), 9);
    assert_eq!(c.capacity(), 16);
    assert_eq!(c.to_vec(), vec![0, 1, 2, 3, 100, 101, 102, 103, 200]);

    // 用例 5：空数组的每个出口。
    let mut e = DynamicArray::new();
    assert!(e.empty() && e.size() == 0 && e.capacity() == 4);
    assert_eq!(e.pop_back(), None); // 空数组 pop 返回 None
    assert_eq!(e.find(1), None);
    assert!(!e.erase(0));
    assert_eq!(e.at(0), None);
    e.push_back(7);
    e.clear();
    assert!(e.size() == 0 && e.empty());
    e.push_back(8); // 清空之后还能继续用
    assert_eq!(e.to_vec(), vec![8]);

    println!("all tests passed");
}
