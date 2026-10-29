/*
============================================================================
循环队列（Circular Queue）
分类：手写数据结构 · 线性结构

操作复杂度：
  enqueue   均摊 O(1)  满时 2 倍扩容并重排下标
  dequeue   O(1)       只动 head，不搬元素
  front     O(1)       队首元素
  rear      O(1)       队尾元素，由 (head+size-1) 算出
  empty     O(1)
  full      O(1)
  size      O(1)
  clear     O(1)
============================================================================

用途：
  BFS 的待访问队列、任务调度、滑动窗口最值、生产者/消费者之间的环形
  缓冲（音频流、日志缓冲都是这个形状）。

核心思想：
  一块环形数组，只维护 head 和 size 两个量：队尾就是
  (head + size) % capacity，不用单独存 tail。出队只把 head 往前走一格，
  元素一个都不搬，于是 dequeue 是 O(1)，而不是数组删除的 O(n)。

关键点（不变量）：
  · 队首逻辑下标是 head，队尾逻辑下标是 (head + size - 1) % capacity，
    一律用公式算，不要另存一个 tail 指针去维护两套状态；
  · 用 size 区分空满，而不是「留一个空槽」：留空槽法最多只能存
    capacity-1 个元素，且每次判满都要绕一圈算 (tail+1)%cap == head；
  · head 和 size 都用 usize，回绕只做加法取模，不存在负下标；
  · 扩容时必须先按「旧 head、旧 capacity」把元素按逻辑顺序搬到新 Vec 的
    0..size-1，循环结束后才能把 head 置 0；先改 head，映射就全错。

手推一遍：
  容量 4，依次 enqueue 1、2、3、4 后 size=4、head=0，队满。
  1. dequeue 两次，取走 1、2，head 走到 2，size=2，逻辑内容 [3, 4]；
  2. 再 enqueue 5：写入下标 (2+2)%4 = 0；enqueue 6：写入 (2+3)%4 = 1；
     物理上是 [5, 6, 3, 4]，逻辑上仍是 [3, 4, 5, 6]，这就是回绕；
  3. 此时再 enqueue 7 就满了，触发扩容：按逻辑顺序搬出 3、4、5、6、7
     到新数组下标 0..4，head 置 0，capacity 变 8。

复杂度分析：
  时间：enqueue 均摊 O(1)（扩容搬移摊到每次入队，n 次入队总搬移小于 2n），
    dequeue/front/rear 都是 O(1)，因为不搬元素只挪下标。
  空间：O(n)，capacity 最多是 size 的 2 倍。
  稳定性：队列是 FIFO，谈不到稳定性。
  与「数组模拟队列」的对比：不做循环就得每次出队整体左移，那是 O(n)；
  循环队列把这份 O(n) 摊成 O(1)，代价只是多一次取模。
*/

#![allow(dead_code)]

const QUEUE_INIT_CAP: usize = 4;

struct CircularQueue {
    data: Vec<i32>,
    head: usize, // 队首所在的物理下标
    size: usize, // 元素个数，空满全靠它区分
}

impl CircularQueue {
    fn new() -> Self {
        CircularQueue {
            data: vec![0; QUEUE_INIT_CAP],
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
        // 关键：这一步用的是旧 head、旧 capacity，映射关系一定正确。
        for i in 0..self.size {
            nd[i] = self.data[(self.head + i) % old_cap];
        }
        self.data = nd;
        // 搬完之后才能改 head：先改就再也算不出旧下标了。
        self.head = 0;
    }

    fn enqueue(&mut self, v: i32) {
        if self.size == self.data.len() {
            self.grow(); // 先扩容，再写入
        }
        // 队尾逻辑位置：head 往右数 size 格，取模把尾巴绕回头部
        let idx = (self.head + self.size) % self.data.len();
        self.data[idx] = v;
        self.size += 1;
    }

    // 空队列返回 None。
    fn dequeue(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let v = self.data[self.head];
        self.head = (self.head + 1) % self.data.len(); // 只挪下标，不搬元素
        self.size -= 1;
        Some(v)
    }

    fn front(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        Some(self.data[self.head])
    }

    fn rear(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        // size-1 是队尾的偏移量；size >= 1 在这里已由判空保证
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
    // 用例 1：出两个进两个，反复 6 轮，让 head 回绕多圈（容量保持 4）。
    let mut a = CircularQueue::new();
    let mut model: Vec<i32> = Vec::new(); // 仅用于校验：当作参考记录
    let mut next = 1;
    for _ in 0..4 {
        a.enqueue(next);
        model.push(next);
        next += 1;
    }
    assert!(a.full() && a.size() == 4);
    for round in 0..6 {
        for _ in 0..2 {
            let got = a.dequeue().expect("dequeue");
            assert_eq!(got, model.remove(0)); // 回绕后出队顺序依然正确
        }
        for _ in 0..2 {
            a.enqueue(next);
            model.push(next);
            next += 1;
        }
        assert!(a.full() && a.size() == 4);
        assert_eq!(a.to_vec(), model);
        assert_eq!(a.head_index(), ((round + 1) * 2) % 4); // head 回绕
    }
    assert_eq!(a.capacity(), 4); // 全程没超过容量，没触发扩容
    a.print();

    // 用例 2：半满时再入队，物理下标会绕回头部。
    let mut b = CircularQueue::new();
    for i in 1..=4 {
        b.enqueue(i); // 满，head=0
    }
    assert_eq!(b.dequeue(), Some(1));
    assert_eq!(b.dequeue(), Some(2)); // head 走到 2
    b.enqueue(5); // 写到下标 0
    b.enqueue(6); // 写到下标 1
    assert_eq!(b.to_vec(), vec![3, 4, 5, 6]);
    assert_eq!(b.head_index(), 2);
    assert!(b.full());
    b.print();

    // 用例 3：满队列再入队触发扩容，head 归零且顺序不变。
    b.enqueue(7);
    assert_eq!(b.capacity(), 8);
    assert_eq!(b.head_index(), 0);
    assert_eq!(b.to_vec(), vec![3, 4, 5, 6, 7]);
    assert_eq!(b.front(), Some(3)); // 扩容后队首没变
    assert_eq!(b.rear(), Some(7));
    b.print();
    // 扩容后继续绕：出两个再进两个，不应该再扩容。
    assert_eq!(b.dequeue(), Some(3));
    assert_eq!(b.dequeue(), Some(4));
    b.enqueue(8);
    b.enqueue(9);
    assert_eq!(b.capacity(), 8);
    assert_eq!(b.to_vec(), vec![5, 6, 7, 8, 9]);

    // 用例 4：空队列的每个出口。
    let mut e = CircularQueue::new();
    assert!(e.empty() && !e.full() && e.size() == 0);
    assert_eq!(e.dequeue(), None);
    assert_eq!(e.front(), None);
    assert_eq!(e.rear(), None);
    assert!(e.to_vec().is_empty());
    e.enqueue(1);
    assert_eq!(e.front(), Some(1)); // 单元素时队首队尾相同
    assert_eq!(e.rear(), Some(1));
    e.clear();
    assert!(e.empty() && e.size() == 0 && e.head_index() == 0);
    e.enqueue(2); // clear 之后还能继续用
    assert_eq!(e.to_vec(), vec![2]);

    // 用例 5：长操作序列，size 与出队顺序逐步核对（参考模型是 Vec）。
    let mut q = CircularQueue::new();
    let mut model: Vec<i32> = Vec::new();
    let mut pushed = 0;
    for step in 0..200 {
        if step % 3 != 2 {
            q.enqueue(pushed);
            model.push(pushed);
            pushed += 1;
        } else if !model.is_empty() {
            let want = model.remove(0); // 参考模型可以直接 O(n) 删
            assert_eq!(q.dequeue(), Some(want));
        }
        assert_eq!(q.size(), model.len());
    }
    assert_eq!(q.to_vec(), model);

    println!("all tests passed");
}
