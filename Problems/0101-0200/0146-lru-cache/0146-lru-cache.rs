/*
============================================================================
146. LRU 缓存（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/lru-cache/
分类：哈希表 · 设计（哈希表 + 双向链表）

请你设计并实现一个满足 LRU (最近最少使用) 缓存 约束的数据结构。

实现 LRUCache 类：

- LRUCache(int capacity) 以 正整数 作为容量 capacity 初始化 LRU 缓存
- int get(int key) 如果关键字 key 存在于缓存中，则返回关键字的值，否则返回 -1 。
- void put(int key, int value) 如果关键字 key 已经存在，则变更其数据值 value ；
  如果不存在，则向缓存中插入该组 key-value 。如果插入操作导致关键字数量超过
  capacity ，则应该 逐出 最久未使用的关键字。

函数 get 和 put 必须以 O(1) 的平均时间复杂度运行。

示例：
输入
["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
输出
[null, null, null, 1, null, -1, null, -1, 3, 4]

解释
LRUCache lRUCache = new LRUCache(2);
lRUCache.put(1, 1); // 缓存是 {1=1}
lRUCache.put(2, 2); // 缓存是 {1=1, 2=2}
lRUCache.get(1);    // 返回 1
lRUCache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
lRUCache.get(2);    // 返回 -1 (未找到)
lRUCache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
lRUCache.get(1);    // 返回 -1 (未找到)
lRUCache.get(3);    // 返回 3
lRUCache.get(4);    // 返回 4

提示：
- 1 <= capacity <= 3000
- 0 <= key <= 10000
- 0 <= value <= 10^5
- 最多调用 2 * 10^5 次 get 和 put
*/
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/*
解法：哈希表 + 双向链表【最优解】

官方题解只给出这一种需要自己写链表的做法（另有一段用 OrderedDict /
LinkedHashMap 的示例，官方自己也说明它不符合面试要求，这里不实现）。
get / put 均为 O(1)，空间 O(capacity)。

接口契约：
  · get(key) 命中返回对应值，并把该 key 置为「最近使用」；未命中返回 -1
  · put(key, value) 已存在则改值并置为最近使用；不存在则插入并置为最近使用
  · 插入后元素个数超过 capacity 时，淘汰「最久未使用」的那个 key
  · value 可能为 0，所以不能用 0 当「不存在」的哨兵，只有 -1 可以

数据结构选型：
  · 只用链表：把节点挪到头部是 O(1)，但按 key 找节点要遍历，O(n)
  · 只用哈希表：定位 O(1)，但没法在 O(1) 内找出谁最久没用
  · 两者合起来：哈希表存「key → 节点位置」，链表按使用顺序排队。定位靠哈希表，
    调序靠链表，各管一半，合起来才是 O(1)
  · 链表用伪头、伪尾两个哨兵：插入与删除都不用判「前驱 / 后继是否存在」

不变式：
  · 链表头部是最近使用的，伪尾的前一个是最久未使用的
  · 每个在链表里的节点都恰好有一条哈希表记录指向它，反之亦然
  · 被 get 或 put 命中的节点必须立刻移到头部，否则「最近使用」的顺序就失真
  · 淘汰时先用节点的 key 反查表项、再摘链表节点，两步都不能省

关键点：
  · 淘汰判据是「插入之后」的元素个数 > capacity；更新已有 key 不改变个数，
    绝不能触发淘汰
  · 节点必须同时存 key：淘汰时手里只有节点指针，要靠它反查哈希表里的项
  · detach 只改前后邻居的指针，不清零 node 自己的 prev / next；
    紧接着的 push_front 会覆盖这两个字段
  · 前驱用 Weak 存：下一跳是强引用、上一跳是弱引用，链上不存在引用环，
    节点被淘汰后内存就能释放
  · 平台签名是 &self，所以哈希表与桶状态都包在 RefCell 里；每次 borrow 都
    只活一条语句，避免 detach 时重复借用同一个节点

以题面示例（capacity = 2，put 1→1、put 2→2、get 1、put 3→3、get 2、
put 4→4、get 1、get 3、get 4）为例：
  put(1,1)、put(2,2)：链表 [2,1]（头为最近），表 {1,2}
  get(1) → 1，链表变 [1,2]
  put(3,3)：插入后个数为 3 > 2，淘汰尾部 2 → 链表 [3,1]
  get(2) → -1
  put(4,4)：淘汰尾部 1 → 链表 [4,3]
  get(1) → -1，get(3) → 3，get(4) → 4 ✔

复杂度分析：
  时间：get / put 都是 O(1)，哈希定位一次、链表摘挂各一次
  空间 O(capacity)：哈希表与链表各存最多 capacity 个节点
*/

// 双向链表节点：key 用于淘汰时反查哈希表，prev / next 维护使用顺序。
// 力扣平台不提供该结构，本地单文件编译需要自己定义。
struct Node {
    key: i32,
    value: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

pub struct LRUCache {
    capacity: usize,
    cache: RefCell<HashMap<i32, Rc<RefCell<Node>>>>,
    head: Rc<RefCell<Node>>, // 伪头
    tail: Rc<RefCell<Node>>, // 伪尾
}

impl LRUCache {
    // 平台签名：fn new(capacity: i32) -> Self
    pub fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(Node::new(0, 0)));
        let tail = Rc::new(RefCell::new(Node::new(0, 0)));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));
        LRUCache {
            capacity: capacity as usize,
            cache: RefCell::new(HashMap::new()),
            head,
            tail,
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        let node = self.cache.borrow().get(&key).cloned();
        match node {
            None => -1,
            Some(n) => {
                let value = n.borrow().value;
                // 读一次就算「刚用过」，顺序必须立刻更新
                self.move_to_head(&n);
                value
            }
        }
    }

    pub fn put(&self, key: i32, value: i32) {
        let existing = self.cache.borrow().get(&key).cloned();
        if let Some(node) = existing {
            // 已存在：只改值并刷新位置，元素个数不变，不触发淘汰
            node.borrow_mut().value = value;
            self.move_to_head(&node);
            return;
        }

        let node = Rc::new(RefCell::new(Node::new(key, value)));
        self.push_front(Rc::clone(&node));
        self.cache.borrow_mut().insert(key, node);

        let over = self.cache.borrow().len() > self.capacity;
        if over {
            // 伪尾的前一个就是最久未使用的节点
            let last = self
                .tail
                .borrow()
                .prev
                .as_ref()
                .and_then(Weak::upgrade)
                .expect("容量至少为 1，插入后链必非空");
            let evicted_key = last.borrow().key;
            self.detach(&last);
            self.cache.borrow_mut().remove(&evicted_key);
        }
    }

    // 头插：伪头与原来的首节点之间
    fn push_front(&self, node: Rc<RefCell<Node>>) {
        let first = self.head.borrow().next.clone().expect("伪尾始终存在");
        {
            let mut n = node.borrow_mut();
            n.prev = Some(Rc::downgrade(&self.head));
            n.next = Some(Rc::clone(&first));
        }
        first.borrow_mut().prev = Some(Rc::downgrade(&node));
        self.head.borrow_mut().next = Some(node);
    }

    // 摘链：只改邻居指针，node 自身的 prev / next 交给下一次插入覆盖
    fn detach(&self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().and_then(Weak::upgrade);
        let next = node.borrow().next.clone();
        if let Some(p) = &prev {
            p.borrow_mut().next = next.clone();
        }
        if let Some(n) = &next {
            n.borrow_mut().prev = prev.as_ref().map(Rc::downgrade);
        }
    }

    fn move_to_head(&self, node: &Rc<RefCell<Node>>) {
        self.detach(node);
        self.push_front(Rc::clone(node));
    }
}

// ===== 测试辅助 =====
// 测试对照：用「最近使用在前」的列表模拟，容量小、只用于核对
struct NaiveLRU {
    capacity: usize,
    order: Vec<(i32, i32)>,
}

impl NaiveLRU {
    fn new(capacity: usize) -> Self {
        NaiveLRU {
            capacity,
            order: Vec::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(pos) = self.order.iter().position(|kv| kv.0 == key) {
            let hit = self.order.remove(pos);
            self.order.insert(0, hit);
            return hit.1;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(pos) = self.order.iter().position(|kv| kv.0 == key) {
            self.order.remove(pos);
        }
        self.order.insert(0, (key, value));
        self.order.truncate(self.capacity);
    }
}

fn replay_example() {
    let c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    assert_eq!(c.get(1), 1);
    c.put(3, 3);
    assert_eq!(c.get(2), -1);
    c.put(4, 4);
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(3), 3);
    assert_eq!(c.get(4), 4);
}

fn check_capacity_one() {
    let c = LRUCache::new(1);
    c.put(1, 1);
    assert_eq!(c.get(1), 1);
    c.put(2, 2);
    // 容量 1：每次插入都必须挤掉上一个
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(2), 2);
}

fn check_update_does_not_evict() {
    let c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    c.put(1, 10); // 更新已有 key，不该新增元素
    assert_eq!(c.get(2), 2);
    c.put(3, 3); // 此时最久未使用的是 1
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(2), 2);
    assert_eq!(c.get(3), 3);
}

fn check_get_refreshes_order() {
    let c = LRUCache::new(2);
    c.put(2, 1);
    c.put(1, 1);
    c.get(2); // 2 变成最近使用
    c.put(3, 3); // 被淘汰的应是 1 而不是 2
    assert_eq!(c.get(1), -1);
    assert_eq!(c.get(2), 1);
    assert_eq!(c.get(3), 3);
}

fn cross_check() {
    let mut seed: u32 = 20240607;
    for _ in 0..5 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let capacity = 1 + (seed >> 16) % 4;
        let fast = LRUCache::new(capacity as i32);
        let mut slow = NaiveLRU::new(capacity as usize);

        for _ in 0..600 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let key = ((seed >> 8) % 8) as i32;
            let value = ((seed >> 4) % 100) as i32;

            if (seed >> 20) % 2 == 0 {
                fast.put(key, value);
                slow.put(key, value);
            } else {
                assert_eq!(fast.get(key), slow.get(key), "get({key})");
            }
        }
    }
}

fn main() {
    // 示例：按题目给的调用序列回放
    replay_example();

    // 边界：容量为 1
    check_capacity_one();

    // 边界：更新已有 key 不能触发淘汰
    check_update_does_not_evict();

    // 反直觉：get 会刷新顺序，被淘汰的不是最早插入的那个
    check_get_refreshes_order();

    // 边界：容量 1 时连续插入，只剩最后一个
    let c = LRUCache::new(1);
    for k in 0..5 {
        c.put(k, k * 10);
    }
    assert_eq!(c.get(4), 40);
    assert_eq!(c.get(0), -1);

    // 随机交叉验证
    cross_check();

    println!("all tests passed");
}
