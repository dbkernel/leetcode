#![allow(dead_code)]
/*
============================================================================
手写哈希表（Hash Map，链地址法）
分类：手写数据结构 · 哈希表
操作复杂度：
  put      O(1) 均摊  负载因子超过 0.75 就 2 倍扩容 rehash
  get      O(1) 均摊  最坏 O(n)，所有 key 挤在同一个桶
  remove   O(1) 均摊  单链表摘除，最坏 O(n)
  clear    O(n)       逐个丢弃节点
============================================================================

用途：
  需要 O(1) 增删查的 key-value 场景。面试里常要求手写链地址法，
  并追问扩容、rehash、负载因子、为什么容量取 2 的幂。

核心思想：
  桶数组的每个位置挂一条单链表，哈希值决定挂在哪个桶。容量是 2 的幂，
  所以取下标用 h & (cap-1) 代替取模（取模是整数除法，慢一个量级）。
  元素多了就 2 倍扩容并把每个节点重新挂到新桶，元素少了就缩容。

关键点（不变量）：
  · 任何时刻 size <= cap * 0.75，超了就扩容，保证链不太长。
  · 扩容后每个节点必须重新算下标：mask 从 cap-1 变成 2*cap-1，
    同一个 key 的低位变了，旧桶号完全可能失效，照搬就会丢数据。
  · 头插会让同一个桶里的顺序与插入顺序相反，但查找是「整条链扫完」，
    顺序不影响正确性；只有需要遍历有序性时才要改成尾插。
  · remove 必须拿到前驱再改 next，别把后面的整条链一起丢掉。

手推一遍：
  初始容量 8，mask = 7，依次 put(1,10)、put(3,30)、put(10,100)：
  1. mix(1) & 7 = 0，桶 0 空，挂上 -> 桶 0: 1。
  2. mix(3) & 7 = 5，桶 5 空，挂上 -> 桶 5: 3。
  3. mix(10) & 7 = 5，与 3 撞桶，头插 -> 桶 5: 10 -> 3。
  get(3)：先算到桶 5，沿链比 10 不等，再比 3 命中，返回 30。
  注意链上是 10 在前、3 在后，与插入顺序相反，但结果一样。

复杂度分析：
  时间 O(1) 均摊：负载因子封顶 0.75，链长期望长度是常数；扩容代价
  分摊到每次 put，等价 O(1)。最坏 O(n) 是所有 key 同桶。
  空间 O(n)：桶数组 cap 个指针 + 每个元素一个节点，cap <= 4n/3 左右。
  稳定性：不涉及顺序，遍历顺序由哈希值决定，不可依赖。
*/

const MIN_CAPACITY: usize = 8;

struct Node {
    key: i32,
    value: i32,
    next: Option<Box<Node>>,
}

struct HashMap {
    buckets: Vec<Option<Box<Node>>>,
    size: usize,
}

impl HashMap {
    fn new() -> Self {
        HashMap {
            buckets: (0..MIN_CAPACITY).map(|_| None).collect(),
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.buckets.len()
    }

    // 已存在就覆盖值，不算新键，也不触发扩容。
    fn put(&mut self, key: i32, value: i32) {
        let index = self.index_of(key);
        if Self::put_in(&mut self.buckets[index], key, value) {
            return;
        }
        // 先判满再插：size+1 个元素必须装得下 0.75 的负载因子。
        if (self.size + 1) * 4 > self.capacity() * 3 {
            self.rehash(self.capacity() * 2);
        }
        // 扩容之后 mask 变了，下标必须重新算，不能沿用上面的 index。
        let index = self.index_of(key);
        let head = self.buckets[index].take();
        self.buckets[index] = Some(Box::new(Node {
            key,
            value,
            next: head,
        }));
        self.size += 1;
    }

    fn get(&self, key: i32) -> Option<i32> {
        Self::get_in(&self.buckets[self.index_of(key)], key)
    }

    fn contains(&self, key: i32) -> bool {
        self.get(key).is_some()
    }

    fn remove(&mut self, key: i32) -> bool {
        let index = self.index_of(key);
        let found = Self::remove_in(&mut self.buckets[index], key);
        if found {
            self.size -= 1;
            self.maybe_shrink();
        }
        found
    }

    fn clear(&mut self) {
        for slot in self.buckets.iter_mut() {
            *slot = None;
        }
        self.size = 0;
    }

    // 仅供自测：检查每个节点都在正确的桶里、节点数等于 size。
    fn check_invariants(&self) -> bool {
        let mut counted = 0;
        for (i, slot) in self.buckets.iter().enumerate() {
            let mut cur = slot;
            while let Some(node) = cur {
                counted += 1;
                if self.index_of(node.key) != i {
                    return false;
                }
                cur = &node.next;
            }
        }
        counted == self.size && self.size * 4 <= self.capacity() * 3
    }

    /// 仅供自测：返回每个桶的链长，用来观察撞桶情况。
    fn bucket_lengths(&self) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(self.capacity());
        for slot in self.buckets.iter() {
            let mut count = 0;
            let mut cur = slot;
            while let Some(node) = cur {
                count += 1;
                cur = &node.next;
            }
            lengths.push(count);
        }
        lengths
    }

    // 对 int 的 32 位模式做乘法散列（murmur 风格的 finalizer）。
    // 先当无符号数看，符号位不再有特殊含义，负数 key 也能散匀。
    fn mix(key: i32) -> u32 {
        let mut h = key as u32;
        h ^= h >> 16;
        h = h.wrapping_mul(0x7feb_352d);
        h ^= h >> 15;
        h = h.wrapping_mul(0x846c_a68b);
        h ^= h >> 16;
        h
    }

    // 容量是 2 的幂，用 & 代替 %。
    fn index_of(&self, key: i32) -> usize {
        (Self::mix(key) as usize) & (self.capacity() - 1)
    }

    fn rehash(&mut self, new_cap: usize) {
        let mut fresh: Vec<Option<Box<Node>>> =
            (0..new_cap).map(|_| None).collect();
        let mask = new_cap - 1;
        for slot in self.buckets.iter_mut() {
            let mut cur = slot.take();
            while let Some(mut node) = cur {
                let next = node.next.take();
                // mask 变了，下标必须重算；沿用旧下标会把节点放错桶。
                let index = (Self::mix(node.key) as usize) & mask;
                node.next = fresh[index].take();
                fresh[index] = Some(node);
                cur = next;
            }
        }
        self.buckets = fresh;
    }

    fn maybe_shrink(&mut self) {
        // 低于 0.25 才缩，且不低于最小容量；否则 put/remove 交替抖动。
        if self.capacity() > MIN_CAPACITY && self.size * 4 < self.capacity() {
            let new_cap = self.capacity() / 2;
            self.rehash(new_cap);
        }
    }

    // 命中已有 key 就覆盖值并返回 true；否则返回 false 由调用方头插。
    fn put_in(link: &mut Option<Box<Node>>, key: i32, value: i32) -> bool {
        match link {
            None => false,
            Some(node) => {
                if node.key == key {
                    node.value = value;
                    true
                } else {
                    Self::put_in(&mut node.next, key, value)
                }
            }
        }
    }

    fn get_in(link: &Option<Box<Node>>, key: i32) -> Option<i32> {
        match link {
            None => None,
            Some(node) => {
                if node.key == key {
                    Some(node.value)
                } else {
                    Self::get_in(&node.next, key)
                }
            }
        }
    }

    fn remove_in(link: &mut Option<Box<Node>>, key: i32) -> bool {
        match link {
            None => false,
            Some(node) => {
                if node.key == key {
                    // 用后继顶替自己，前驱的 next 自然接上，不会断链。
                    *link = node.next.take();
                    true
                } else {
                    Self::remove_in(&mut node.next, key)
                }
            }
        }
    }
}

fn is_power_of_two(value: usize) -> bool {
    value > 0 && value & (value - 1) == 0
}

fn test_basics() {
    let mut map = HashMap::new();
    assert_eq!(map.size(), 0);
    assert_eq!(map.capacity(), 8);

    map.put(42, 420);
    map.put(7, 70);
    assert_eq!(map.size(), 2);
    assert_eq!(map.get(42), Some(420));
    assert_eq!(map.get(7), Some(70));
    assert!(map.contains(7));
    assert!(!map.contains(8));
    assert_eq!(map.get(8), None);

    map.put(42, 999);
    assert_eq!(map.size(), 2);
    assert_eq!(map.get(42), Some(999));
    assert!(map.check_invariants());
}

fn test_extremes() {
    let mut map = HashMap::new();
    let extremes = [
        0,
        -1,
        -1024,
        1024,
        i32::MIN,
        i32::MAX,
        -2147483647,
    ];
    for &key in extremes.iter() {
        // 用异或造值，避免 i32::MIN 再乘 2 这种溢出。
        map.put(key, key ^ 0x5a5a);
    }
    assert_eq!(map.size(), extremes.len());
    for &key in extremes.iter() {
        assert_eq!(map.get(key), Some(key ^ 0x5a5a));
    }
    assert_eq!(map.get(1), None);
    assert!(map.check_invariants());
}

fn test_collision_chain() {
    // 容量 8 时 mix(3)、mix(10)、mix(17) 的低三位都是 5，必然同桶。
    let mut map = HashMap::new();
    assert_eq!(map.capacity(), 8);
    map.put(3, 30);
    map.put(10, 100);
    map.put(17, 170);
    assert_eq!(map.capacity(), 8);
    assert_eq!(map.size(), 3);
    assert!(map.check_invariants());

    // 三个 key 落在同一个桶里，这条链必须完整。
    let lengths = map.bucket_lengths();
    assert_eq!(lengths[5], 3);
    assert_eq!(lengths.iter().sum::<usize>(), 3);

    assert_eq!(map.get(3), Some(30));
    assert_eq!(map.get(10), Some(100));
    assert_eq!(map.get(17), Some(170));

    // 10 在链中间，摘掉它必须把前后接上。
    assert!(map.remove(10));
    assert!(!map.contains(10));
    assert_eq!(map.size(), 2);
    assert_eq!(map.get(3), Some(30));
    assert_eq!(map.get(17), Some(170));
    assert!(map.check_invariants());

    // remove 不存在的 key 必须返回 false 且不动 size。
    assert!(!map.remove(10));
    assert!(!map.remove(12345));
    assert_eq!(map.size(), 2);
    assert!(map.check_invariants());
}

fn test_grow_and_shrink() {
    let mut map = HashMap::new();
    let total = 5000;
    for i in 0..total {
        map.put(i as i32, i as i32 * 3 + 1);
    }
    assert_eq!(map.size() as i32, total);
    assert!(is_power_of_two(map.capacity()));
    assert!(map.capacity() >= 8192); // 5000/8192 < 0.75
    assert!(map.check_invariants());

    for i in 0..total {
        assert_eq!(map.get(i as i32), Some(i as i32 * 3 + 1));
    }

    // 删掉一半，剩下的必须还能查到（扩容/缩容都要重算下标）。
    for i in (0..total).step_by(2) {
        assert!(map.remove(i as i32));
    }
    assert_eq!(map.size() as i32, total / 2);
    for i in 0..total {
        if i % 2 == 1 {
            assert_eq!(map.get(i as i32), Some(i as i32 * 3 + 1));
        } else {
            assert_eq!(map.get(i as i32), None);
        }
    }
    assert!(map.check_invariants());

    let big_capacity = map.capacity();
    for i in (1..total).step_by(2) {
        if i >= 1000 {
            assert!(map.remove(i as i32));
        }
    }
    assert_eq!(map.size(), 500);
    assert!(map.capacity() < big_capacity);
    assert!(is_power_of_two(map.capacity()));
    for i in (1..total).step_by(2) {
        assert_eq!(map.contains(i as i32), i < 1000);
    }
    assert!(map.check_invariants());

    // clear 之后 size 归零、旧 key 全部查不到，且能继续使用。
    map.clear();
    assert_eq!(map.size(), 0);
    assert!(!map.contains(1));
    assert!(map.check_invariants());

    map.put(123, 456);
    assert_eq!(map.size(), 1);
    assert_eq!(map.get(123), Some(456));
    assert!(map.check_invariants());
}

fn main() {
    test_basics();
    test_extremes();
    test_collision_chain();
    test_grow_and_shrink();
    println!("all tests passed");
}
