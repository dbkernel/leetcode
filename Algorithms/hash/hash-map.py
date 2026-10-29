"""
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
"""

MASK32 = 0xFFFFFFFF
MIN_CAPACITY = 8


class _Node:
    __slots__ = ("key", "value", "next")

    def __init__(self, key: int, value: int, nxt=None) -> None:
        self.key = key
        self.value = value
        self.next = nxt


def mix(key: int) -> int:
    """对 int 的 32 位模式做乘法散列（murmur 风格的 finalizer）。

    先截成 32 位无符号看，符号位不再有特殊含义，负数 key 也能散匀。
    """
    h = key & MASK32
    h ^= h >> 16
    h = (h * 0x7FEB352D) & MASK32
    h ^= h >> 15
    h = (h * 0x846CA68B) & MASK32
    h ^= h >> 16
    return h


class HashMap:
    def __init__(self) -> None:
        self._buckets = [None] * MIN_CAPACITY
        self._cap = MIN_CAPACITY
        self._size = 0

    def size(self) -> int:
        return self._size

    def capacity(self) -> int:
        return self._cap

    def put(self, key: int, value: int) -> None:
        """已存在就覆盖值，不算新键，也不触发扩容。"""
        index = self._index_of(key)
        node = self._buckets[index]
        while node is not None:
            if node.key == key:
                node.value = value
                return
            node = node.next

        # 先判满再插：size+1 个元素必须装得下 0.75 的负载因子。
        if (self._size + 1) * 4 > self._cap * 3:
            self._rehash(self._cap * 2)
            index = self._index_of(key)
        # 头插，O(1)；同一个桶里的顺序会与插入顺序相反。
        self._buckets[index] = _Node(key, value, self._buckets[index])
        self._size += 1

    def get(self, key: int):
        node = self._buckets[self._index_of(key)]
        while node is not None:
            if node.key == key:
                return node.value
            node = node.next
        return None

    def contains(self, key: int) -> bool:
        node = self._buckets[self._index_of(key)]
        while node is not None:
            if node.key == key:
                return True
            node = node.next
        return False

    def remove(self, key: int) -> bool:
        index = self._index_of(key)
        prev = None
        node = self._buckets[index]
        while node is not None:
            if node.key == key:
                if prev is None:
                    self._buckets[index] = node.next
                else:
                    prev.next = node.next
                self._size -= 1
                self._maybe_shrink()
                return True
            prev = node
            node = node.next
        return False

    def clear(self) -> None:
        self._buckets = [None] * self._cap
        self._size = 0

    def check_invariants(self) -> bool:
        """仅供自测：检查每个节点都在正确的桶里、节点数等于 size。"""
        counted = 0
        for i in range(self._cap):
            node = self._buckets[i]
            while node is not None:
                counted += 1
                if self._index_of(node.key) != i:
                    return False
                node = node.next
        if counted != self._size:
            return False
        return self._size * 4 <= self._cap * 3

    def bucket_lengths(self):
        """仅供自测：返回每个桶的链长，用来观察撞桶情况。"""
        lengths = []
        for i in range(self._cap):
            node = self._buckets[i]
            count = 0
            while node is not None:
                count += 1
                node = node.next
            lengths.append(count)
        return lengths

    def _index_of(self, key: int) -> int:
        # 容量是 2 的幂，用 & 代替 %。
        return mix(key) & (self._cap - 1)

    def _rehash(self, new_cap: int) -> None:
        fresh = [None] * new_cap
        mask = new_cap - 1
        for i in range(self._cap):
            node = self._buckets[i]
            while node is not None:
                nxt = node.next
                # mask 变了，下标必须重算；沿用旧下标会把节点放错桶。
                index = mix(node.key) & mask
                node.next = fresh[index]
                fresh[index] = node
                node = nxt
        self._buckets = fresh
        self._cap = new_cap

    def _maybe_shrink(self) -> None:
        # 低于 0.25 才缩，且不低于最小容量；否则 put/remove 交替抖动。
        if self._cap > MIN_CAPACITY and self._size * 4 < self._cap:
            self._rehash(self._cap // 2)


def is_power_of_two(value: int) -> bool:
    return value > 0 and value & (value - 1) == 0


def test_basics() -> None:
    m = HashMap()
    assert m.size() == 0 and m.capacity() == 8

    m.put(42, 420)
    m.put(7, 70)
    assert m.size() == 2
    assert m.get(42) == 420
    assert m.get(7) == 70
    assert m.contains(7)
    assert not m.contains(8)
    assert m.get(8) is None

    m.put(42, 999)
    assert m.size() == 2
    assert m.get(42) == 999
    assert m.check_invariants()


def test_extremes() -> None:
    m = HashMap()
    extremes = [0, -1, -1024, 1024, -2147483648, 2147483647, -2147483647]
    for key in extremes:
        # 用异或造值，避开 Python 之外的溢出问题，也顺手测负数。
        m.put(key, key ^ 0x5A5A)
    assert m.size() == len(extremes)
    for key in extremes:
        assert m.get(key) == key ^ 0x5A5A
    assert m.get(1) is None
    assert m.check_invariants()


def test_collision_chain() -> None:
    # 容量 8 时 mix(3)、mix(10)、mix(17) 的低三位都是 5，必然同桶。
    m = HashMap()
    assert m.capacity() == 8
    m.put(3, 30)
    m.put(10, 100)
    m.put(17, 170)
    assert m.capacity() == 8
    assert m.size() == 3
    assert m.check_invariants()

    # 三个 key 落在同一个桶里，这条链必须完整。
    lengths = m.bucket_lengths()
    assert lengths[5] == 3
    assert sum(lengths) == 3

    assert m.get(3) == 30
    assert m.get(10) == 100
    assert m.get(17) == 170

    # 10 在链中间，摘掉它必须把前后接上。
    assert m.remove(10)
    assert not m.contains(10)
    assert m.size() == 2
    assert m.get(3) == 30
    assert m.get(17) == 170
    assert m.check_invariants()

    # remove 不存在的 key 必须返回 False 且不动 size。
    assert not m.remove(10)
    assert not m.remove(12345)
    assert m.size() == 2
    assert m.check_invariants()


def test_grow_and_shrink() -> None:
    m = HashMap()
    total = 5000
    for i in range(total):
        m.put(i, i * 3 + 1)
    assert m.size() == total
    assert is_power_of_two(m.capacity())
    assert m.capacity() >= 8192  # 5000/8192 < 0.75
    assert m.check_invariants()

    for i in range(total):
        assert m.get(i) == i * 3 + 1

    # 删掉一半，剩下的必须还能查到（扩容/缩容都要重算下标）。
    for i in range(0, total, 2):
        assert m.remove(i)
    assert m.size() == total // 2
    for i in range(total):
        if i % 2 == 1:
            assert m.get(i) == i * 3 + 1
        else:
            assert m.get(i) is None
    assert m.check_invariants()

    big_capacity = m.capacity()
    for i in range(1, total, 2):
        if i >= 1000:
            assert m.remove(i)
    assert m.size() == 500
    assert m.capacity() < big_capacity
    assert is_power_of_two(m.capacity())
    for i in range(1, total, 2):
        assert m.contains(i) == (i < 1000)
    assert m.check_invariants()

    # clear 之后 size 归零、旧 key 全部查不到，且能继续使用。
    m.clear()
    assert m.size() == 0
    assert not m.contains(1)
    assert m.check_invariants()

    m.put(123, 456)
    assert m.size() == 1
    assert m.get(123) == 456
    assert m.check_invariants()


def main() -> None:
    test_basics()
    test_extremes()
    test_collision_chain()
    test_grow_and_shrink()
    print("all tests passed")


if __name__ == "__main__":
    main()
