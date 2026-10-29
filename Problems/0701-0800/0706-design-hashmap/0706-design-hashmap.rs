/*
============================================================================
706. 设计哈希映射（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/design-hashmap/
分类：哈希表 · 设计（链地址法，自实现桶数组与链表）

不使用任何内建的哈希表库设计一个哈希映射（HashMap）。

实现 MyHashMap 类：

- MyHashMap() 用空映射初始化对象
- void put(int key, int value) 向 HashMap 插入一个键值对 (key, value) 。
  如果 key 已经存在于映射中，则更新其对应的值 value 。
- int get(int key) 返回特定的 key 所映射的 value ；如果映射中不包含 key 的映射，
  返回 -1 。
- void remove(key) 如果映射中存在 key 的映射，则移除 key 和它所对应的 value 。

示例：
输入：
["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
[[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
输出：
[null, null, null, 1, -1, null, 1, null, -1]

解释：
MyHashMap myHashMap = new MyHashMap();
myHashMap.put(1, 1); // myHashMap 现在为 [[1,1]]
myHashMap.put(2, 2); // myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(1);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(3);    // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
myHashMap.put(2, 1); // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
myHashMap.get(2);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
myHashMap.remove(2); // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
myHashMap.get(2);    // 返回 -1（未找到），myHashMap 现在为 [[1,1]]

提示：
- 0 <= key, value <= 10^6
- 最多调用 10^4 次 put、get 和 remove 方法
*/
use std::cell::RefCell;

/*
解法：链地址法（自实现桶数组 + 单向链表）【最优解】

官方题解只给出这一种方法，也就是本题的最优解：单次操作平均 O(1)，空间 O(n+b)。

接口契约：
  · put 是「有则改、无则插」，不能插出重复 key
  · get 用 -1 表示不存在 —— value 非负，所以 -1 不会与合法值冲突
  · remove 只删 key，不存在时什么也不做，不能报错
  · 全程不许用语言内建的哈希表，只能用数组加自己写的链表

数据结构选型：
  · 直接把 key 当下标开数组，单次 O(1)，但要开 10^6 + 1 个格子，而调用最多
    10^4 次，空间利用率不到百分之一
  · 有序数组加二分查找，单次 O(log n)，插入还要搬动元素
  · 桶数组加链表（链地址法）：769 个桶，10^4 个 key 平均每桶约 13 个节点，
    单次 O(n/b)；桶数取质数是为了让 key % b 分布更均匀

不变式：
  · 同一个 key 在整张表里最多出现一次：put 先查后插，查不到才新建节点
  · 每个节点的 next 要么为空，要么指向同一个桶里的另一个节点
  · buckets[h] 始终指向该桶链表的头节点；删除时改的是「指向这条边的指针」
    而不是节点本身，于是头节点和中间节点可以共用一段代码

关键点：
  · put 命中已有 key 时只改 value，绝不新建节点，否则同一个 key 会出现两次
  · get 与 remove 必须沿 next 走完整条链才能断定 key 不存在
  · 空映射时每个桶头都是 None，哈希函数只做取模，不依赖 key 是否出现过
  · 平台的签名是 &self，桶数组用 RefCell 包一层，等价于其它语言的成员变量
  · Rust 里「一边持有前驱的可变引用、一边改后继」过不了借用检查，所以增删查
    都写成沿 next 递归的小函数：每一层只借用自己那一格 Option

以题面示例（put 1→1、put 2→2、get 1、get 3、put 2→1、get 2、remove 2、get 2）：
  put(1,1)、put(2,2)：两个桶各挂一个节点
  get(1) → 1
  get(3) → 该桶为空 → -1
  put(2,1)：命中已有节点，只把 value 改成 1
  get(2) → 1
  remove(2)：从链上摘掉该节点
  get(2) → -1 ✔

复杂度分析：
  时间：put / get / remove 平均 O(n/b)，b = 769 固定，10^4 次调用下平均每桶约
    13 个节点，可视为 O(1)；最坏（key 全部同余）退化为 O(n)
  空间 O(n + b)：节点数等于不同 key 的个数，另加 769 个桶头指针
*/

const BASE: i32 = 769;

// 链地址法里的链表节点，同时存 key 与 value
struct Entry {
    key: i32,
    value: i32,
    next: Option<Box<Entry>>,
}

pub struct MyHashMap {
    buckets: RefCell<Vec<Option<Box<Entry>>>>,
}

// 命中已有 key 就只改值；走到链尾才新建节点，保证 key 唯一
fn put_entry(head: &mut Option<Box<Entry>>, key: i32, value: i32) {
    match head {
        Some(node) if node.key == key => node.value = value,
        Some(node) => put_entry(&mut node.next, key, value),
        None => *head = Some(Box::new(Entry { key, value, next: None })),
    }
}

fn get_entry(head: &Option<Box<Entry>>, key: i32) -> i32 {
    match head {
        Some(node) if node.key == key => node.value,
        Some(node) => get_entry(&node.next, key),
        // 链走完仍未命中，才说明 key 不存在
        None => -1,
    }
}

// 先把整条链 take 出来：命中就丢掉这一节，不命中再原样放回去
fn remove_entry(head: &mut Option<Box<Entry>>, key: i32) {
    let Some(mut node) = head.take() else {
        return;
    };
    if node.key == key {
        *head = node.next.take();
    } else {
        remove_entry(&mut node.next, key);
        *head = Some(node);
    }
}

impl MyHashMap {
    // 平台签名：fn new() -> Self
    pub fn new() -> Self {
        let buckets: Vec<Option<Box<Entry>>> =
            (0..BASE).map(|_| None).collect();
        MyHashMap {
            buckets: RefCell::new(buckets),
        }
    }

    fn hash(key: i32) -> usize {
        (key % BASE) as usize
    }

    pub fn put(&self, key: i32, value: i32) {
        let mut buckets = self.buckets.borrow_mut();
        put_entry(&mut buckets[Self::hash(key)], key, value);
    }

    pub fn get(&self, key: i32) -> i32 {
        let buckets = self.buckets.borrow();
        get_entry(&buckets[Self::hash(key)], key)
    }

    pub fn remove(&self, key: i32) {
        let mut buckets = self.buckets.borrow_mut();
        remove_entry(&mut buckets[Self::hash(key)], key);
    }
}

impl Default for MyHashMap {
    fn default() -> Self {
        Self::new()
    }
}

// ===== 测试辅助 =====
// 测试对照：用线性查找的键值对列表模拟，只负责核对结果
struct NaiveMap {
    items: Vec<(i32, i32)>,
}

impl NaiveMap {
    fn put(&mut self, key: i32, value: i32) {
        for kv in self.items.iter_mut() {
            if kv.0 == key {
                kv.1 = value;
                return;
            }
        }
        self.items.push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        for kv in self.items.iter() {
            if kv.0 == key {
                return kv.1;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        if let Some(pos) = self.items.iter().position(|kv| kv.0 == key) {
            self.items.remove(pos);
        }
    }
}

fn replay_example() {
    let m = MyHashMap::new();
    m.put(1, 1);
    m.put(2, 2);
    assert_eq!(m.get(1), 1);
    assert_eq!(m.get(3), -1);
    m.put(2, 1);
    assert_eq!(m.get(2), 1);
    m.remove(2);
    assert_eq!(m.get(2), -1);
}

fn check_collisions() {
    let m = MyHashMap::new();
    // 三个 key 对 769 同余，必然落在同一个桶里
    m.put(0, 10);
    m.put(769, 20);
    m.put(1538, 30);
    assert_eq!(m.get(0), 10);
    assert_eq!(m.get(769), 20);
    assert_eq!(m.get(1538), 30);

    // 删桶头
    m.remove(0);
    assert_eq!(m.get(0), -1);
    assert_eq!(m.get(769), 20);
    assert_eq!(m.get(1538), 30);

    // 删链中间
    m.remove(769);
    assert_eq!(m.get(769), -1);
    assert_eq!(m.get(1538), 30);

    // 删不存在的 key 不应影响其它节点
    m.remove(769);
    m.remove(1000000);
    assert_eq!(m.get(1538), 30);
}

fn cross_check() {
    let fast = MyHashMap::new();
    let mut slow = NaiveMap { items: Vec::new() };
    let mut seed: u32 = 20240607;
    for _ in 0..3000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let op = (seed >> 16) % 3;
        // 让 key 集中在几个同余类上，制造长链
        let key = ((seed >> 8) % 8) as i32 * 769 + ((seed >> 20) % 3) as i32;
        let value = ((seed >> 4) % 1000) as i32;

        match op {
            0 => {
                fast.put(key, value);
                slow.put(key, value);
            }
            1 => assert_eq!(fast.get(key), slow.get(key), "get({key})"),
            _ => {
                fast.remove(key);
                slow.remove(key);
            }
        }
    }
}

fn main() {
    // 示例：按题目给的调用序列回放
    replay_example();

    // 边界：同桶多节点的增删
    check_collisions();

    // 边界：value 为 0 与「不存在」的 -1 必须区分开
    let m = MyHashMap::new();
    m.put(1000000, 0);
    assert_eq!(m.get(1000000), 0);
    assert_eq!(m.get(0), -1);

    // 边界：同一个 key 反复覆盖，值只保留最后一次
    let m = MyHashMap::new();
    for v in 0..5 {
        m.put(42, v);
    }
    assert_eq!(m.get(42), 4);

    // 随机交叉验证
    cross_check();

    println!("all tests passed");
}
