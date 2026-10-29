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
#include <cassert>
#include <iostream>
#include <unordered_map>
#include <utility>
#include <vector>
using namespace std;

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
  · 淘汰判据是「插入之后」的 size > capacity；更新已有 key 不改变 size，
    绝不能触发淘汰
  · 节点必须同时存 key：淘汰时手里只有节点指针，要靠它反查哈希表里的项
  · removeNode 只改前后邻居的指针，不清零 node 自己的 prev / next；
    紧接着的 addToHead 会覆盖这两个字段
  · 先 addToHead 再进表、或先进表再 addToHead 都可以，但必须保证失败路径
    不会留下半成品；这里先挂链表再写表，两步都不会失败

以题面示例（capacity = 2，put 1→1、put 2→2、get 1、put 3→3、get 2、
put 4→4、get 1、get 3、get 4）为例：
  put(1,1)、put(2,2)：链表 [2,1]（头为最近），表 {1,2}
  get(1) → 1，链表变 [1,2]
  put(3,3)：插入后 size=3 > 2，淘汰尾部 2 → 链表 [3,1]
  get(2) → -1
  put(4,4)：淘汰尾部 1 → 链表 [4,3]
  get(1) → -1，get(3) → 3，get(4) → 4 ✔

复杂度分析：
  时间：get / put 都是 O(1)，哈希定位一次、链表摘挂各一次
  空间 O(capacity)：哈希表与链表各存最多 capacity 个节点
*/

// 双向链表节点：key 用于淘汰时反查哈希表，prev / next 维护使用顺序
struct DLinkedNode
{
    int key = 0;
    int value = 0;
    DLinkedNode *prev = nullptr;
    DLinkedNode *next = nullptr;

    DLinkedNode() = default;
    DLinkedNode(int k, int v) : key(k), value(v) {}
};

class LRUCache
{
public:
    explicit LRUCache(int capacity) : capacity_(capacity)
    {
        // 伪头伪尾：链表的增删不用判空，头尾永远存在
        head_ = new DLinkedNode();
        tail_ = new DLinkedNode();
        head_->next = tail_;
        tail_->prev = head_;
    }

    ~LRUCache()
    {
        DLinkedNode *cur = head_;
        while (cur != nullptr)
        {
            DLinkedNode *next = cur->next;
            delete cur;
            cur = next;
        }
    }

    // 平台只构造一个对象；禁用拷贝，避免两个对象共享同一条链
    LRUCache(const LRUCache &) = delete;
    LRUCache &operator=(const LRUCache &) = delete;

    int get(int key)
    {
        auto it = cache_.find(key);
        if (it == cache_.end()) return -1;

        // 读一次就算「刚用过」，顺序必须立刻更新
        moveToHead(it->second);
        return it->second->value;
    }

    void put(int key, int value)
    {
        auto it = cache_.find(key);
        if (it != cache_.end())
        {
            // 已存在：只改值并刷新位置，元素个数不变，不触发淘汰
            it->second->value = value;
            moveToHead(it->second);
            return;
        }

        DLinkedNode *node = new DLinkedNode(key, value);
        addToHead(node);
        cache_[key] = node;

        if (static_cast<int>(cache_.size()) > capacity_)
        {
            // 伪尾的前一个就是最久未使用的节点
            DLinkedNode *removed = tail_->prev;
            removeNode(removed);
            cache_.erase(removed->key);
            delete removed;
        }
    }

private:
    // 头插：伪头与原来的首节点之间
    void addToHead(DLinkedNode *node)
    {
        node->prev = head_;
        node->next = head_->next;
        head_->next->prev = node;
        head_->next = node;
    }

    // 摘链：只改邻居指针，node 自身的 prev / next 交给下一次插入覆盖
    void removeNode(DLinkedNode *node)
    {
        node->prev->next = node->next;
        node->next->prev = node->prev;
    }

    void moveToHead(DLinkedNode *node)
    {
        removeNode(node);
        addToHead(node);
    }

    unordered_map<int, DLinkedNode *> cache_;
    DLinkedNode *head_;
    DLinkedNode *tail_;
    int capacity_;
};

namespace
{
// 测试对照：用「最近使用在前」的列表模拟，容量小、只用于核对
class NaiveLRU
{
public:
    explicit NaiveLRU(int capacity) : capacity_(capacity) {}

    int get(int key)
    {
        for (size_t i = 0; i < order_.size(); ++i)
        {
            if (order_[i].first == key)
            {
                pair<int, int> hit = order_[i];
                order_.erase(order_.begin() + static_cast<long>(i));
                order_.insert(order_.begin(), hit);
                return hit.second;
            }
        }
        return -1;
    }

    void put(int key, int value)
    {
        for (size_t i = 0; i < order_.size(); ++i)
        {
            if (order_[i].first == key)
            {
                order_.erase(order_.begin() + static_cast<long>(i));
                break;
            }
        }
        order_.insert(order_.begin(), {key, value});
        if (static_cast<int>(order_.size()) > capacity_) order_.pop_back();
    }

private:
    int capacity_;
    vector<pair<int, int>> order_;
};

void replayExample()
{
    LRUCache c(2);
    c.put(1, 1);
    c.put(2, 2);
    assert(c.get(1) == 1);
    c.put(3, 3);
    assert(c.get(2) == -1);
    c.put(4, 4);
    assert(c.get(1) == -1);
    assert(c.get(3) == 3);
    assert(c.get(4) == 4);
}

void checkCapacityOne()
{
    LRUCache c(1);
    c.put(1, 1);
    assert(c.get(1) == 1);
    c.put(2, 2);
    // 容量 1：每次插入都必须挤掉上一个
    assert(c.get(1) == -1);
    assert(c.get(2) == 2);
}

void checkUpdateDoesNotEvict()
{
    LRUCache c(2);
    c.put(1, 1);
    c.put(2, 2);
    c.put(1, 10);          // 更新已有 key，不该新增元素
    assert(c.get(2) == 2); // 2 还在
    c.put(3, 3);           // 此时最久未使用的是 1
    assert(c.get(1) == -1);
    assert(c.get(2) == 2);
    assert(c.get(3) == 3);
}

void checkGetRefreshesOrder()
{
    LRUCache c(2);
    c.put(2, 1);
    c.put(1, 1);
    c.get(2);     // 2 变成最近使用
    c.put(3, 3);  // 被淘汰的应是 1 而不是 2
    assert(c.get(1) == -1);
    assert(c.get(2) == 1);
    assert(c.get(3) == 3);
}

void crossCheck()
{
    unsigned seed = 20240607;
    for (int round = 0; round < 5; ++round)
    {
        seed = seed * 1103515245 + 12345;
        int capacity = 1 + static_cast<int>((seed >> 16) % 4);
        LRUCache fast(capacity);
        NaiveLRU slow(capacity);

        for (int step = 0; step < 600; ++step)
        {
            seed = seed * 1103515245 + 12345;
            int key = static_cast<int>((seed >> 8) % 8);
            int value = static_cast<int>((seed >> 4) % 100);

            if ((seed >> 20) % 2 == 0)
            {
                fast.put(key, value);
                slow.put(key, value);
            }
            else
            {
                assert(fast.get(key) == slow.get(key));
            }
        }
    }
}
} // namespace

int main()
{
    // 示例：按题目给的调用序列回放
    replayExample();

    // 边界：容量为 1
    checkCapacityOne();

    // 边界：更新已有 key 不能触发淘汰
    checkUpdateDoesNotEvict();

    // 反直觉：get 会刷新顺序，被淘汰的不是最早插入的那个
    checkGetRefreshesOrder();

    // 边界：容量 1 时连续插入，只剩最后一个
    {
        LRUCache c(1);
        for (int k = 0; k < 5; ++k) c.put(k, k * 10);
        assert(c.get(4) == 40);
        assert(c.get(0) == -1);
    }

    // 随机交叉验证
    crossCheck();

    cout << "all tests passed" << endl;
    return 0;
}
