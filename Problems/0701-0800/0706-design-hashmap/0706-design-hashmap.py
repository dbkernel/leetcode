"""
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
"""

"""
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
  · remove 记住前驱节点，命中时把 prev.next 指向 cur.next；前驱为空说明删的是
    桶头，直接改 buckets[h]
  · get 与 remove 必须沿 next 走完整条链才能断定 key 不存在
  · 空映射时每个桶头都是 None，哈希函数只做取模，不依赖 key 是否出现过

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
"""


class _Entry:
    """链地址法里的链表节点，同时存 key 与 value"""

    __slots__ = ("key", "value", "next")

    def __init__(self, key: int, value: int, nxt: "_Entry | None"):
        self.key = key
        self.value = value
        self.next = nxt


class MyHashMap:
    # 质数：key 在 0..10^6 之间，取合数做模更容易让一批 key 挤进同几个桶
    BASE = 769

    def __init__(self):
        self._buckets = [None] * self.BASE

    def _hash(self, key: int) -> int:
        return key % self.BASE

    def put(self, key: int, value: int) -> None:
        h = self._hash(key)
        cur = self._buckets[h]
        while cur is not None:
            if cur.key == key:
                # 命中也只改值：key 在表里必须唯一
                cur.value = value
                return
            cur = cur.next
        # 头插：新节点放在链首，省掉找链尾的遍历
        self._buckets[h] = _Entry(key, value, self._buckets[h])

    def get(self, key: int) -> int:
        cur = self._buckets[self._hash(key)]
        while cur is not None:
            if cur.key == key:
                return cur.value
            cur = cur.next
        # 链走完仍未命中，才说明 key 不存在
        return -1

    def remove(self, key: int) -> None:
        h = self._hash(key)
        prev = None
        cur = self._buckets[h]
        while cur is not None:
            if cur.key == key:
                if prev is None:
                    # 删的是桶头，直接改桶指针
                    self._buckets[h] = cur.next
                else:
                    prev.next = cur.next
                return
            prev, cur = cur, cur.next


# ===== 测试辅助 =====
class NaiveMap:
    """测试对照：用线性查找的键值对列表模拟，只负责核对结果"""

    def __init__(self):
        self.items = []

    def put(self, key: int, value: int) -> None:
        for kv in self.items:
            if kv[0] == key:
                kv[1] = value
                return
        self.items.append([key, value])

    def get(self, key: int) -> int:
        for kv in self.items:
            if kv[0] == key:
                return kv[1]
        return -1

    def remove(self, key: int) -> None:
        for i, kv in enumerate(self.items):
            if kv[0] == key:
                del self.items[i]
                return


def replay_example() -> None:
    m = MyHashMap()
    m.put(1, 1)
    m.put(2, 2)
    assert m.get(1) == 1
    assert m.get(3) == -1
    m.put(2, 1)
    assert m.get(2) == 1
    m.remove(2)
    assert m.get(2) == -1


def check_collisions() -> None:
    m = MyHashMap()
    # 三个 key 对 769 同余，必然落在同一个桶里
    m.put(0, 10)
    m.put(769, 20)
    m.put(1538, 30)
    assert m.get(0) == 10
    assert m.get(769) == 20
    assert m.get(1538) == 30

    # 删桶头
    m.remove(0)
    assert m.get(0) == -1
    assert m.get(769) == 20
    assert m.get(1538) == 30

    # 删链中间
    m.remove(769)
    assert m.get(769) == -1
    assert m.get(1538) == 30

    # 删不存在的 key 不应报错，也不应影响其它节点
    m.remove(769)
    m.remove(1000000)
    assert m.get(1538) == 30


def cross_check() -> None:
    fast = MyHashMap()
    slow = NaiveMap()
    seed = 20240607
    for _ in range(3000):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        op = (seed >> 16) % 3
        # 让 key 集中在几个同余类上，制造长链
        key = ((seed >> 8) % 8) * 769 + (seed >> 20) % 3
        value = (seed >> 4) % 1000

        if op == 0:
            fast.put(key, value)
            slow.put(key, value)
        elif op == 1:
            assert fast.get(key) == slow.get(key), f"get({key})"
        else:
            fast.remove(key)
            slow.remove(key)


if __name__ == "__main__":
    # 示例：按题目给的调用序列回放
    replay_example()

    # 边界：同桶多节点的增删
    check_collisions()

    # 边界：value 为 0 与「不存在」的 -1 必须区分开
    m = MyHashMap()
    m.put(1000000, 0)
    assert m.get(1000000) == 0
    assert m.get(0) == -1

    # 边界：同一个 key 反复覆盖，值只保留最后一次
    m = MyHashMap()
    for v in range(5):
        m.put(42, v)
    assert m.get(42) == 4

    # 随机交叉验证
    cross_check()

    print("all tests passed")
