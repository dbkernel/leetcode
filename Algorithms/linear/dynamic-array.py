"""
============================================================================
动态数组（Dynamic Array）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_back  均摊 O(1)  满时容量翻倍，n 次 push 累计搬移小于 2n
  pop_back   O(1)       缩容只在 size <= capacity/4 时发生
  insert     O(n)       index 及其后的元素整体右移一位
  erase      O(n)       index 之后的元素整体左移一位
  at         O(1)       带越界检查，越界抛 IndexError
  find       O(n)       线性扫描，返回下标，找不到返回 -1
  clear      O(1)       把 size 置 0，必要时顺带缩容
============================================================================

用途：
  手写一切「长度可变」容器的底座：栈、队列、哈希表的桶都能建在它上面。
  面试常以「实现一个 vector」或「这行为什么退化到 O(n^2)」的形式出现。

核心思想：
  一块预分配的 list + size / capacity 两个量。list 的长度当 capacity 用，
  size 之前的格子才是有效元素。写满就换成两倍长；空出四分之三就减半。

关键点（不变量）：
  · 0 <= size <= capacity，capacity 从 4 开始，每次只翻倍或减半，
    所以它永远是 2 的幂；
  · push_back 必须先扩容再写 _data[size]，顺序反了就是越界写；
  · 缩容条件是 size <= capacity/4 且 capacity >= 8。若在 size 刚低于
    capacity/2 时就减半，同一个位置反复 push/pop 会来回搬移；
  · insert 从后往前搬，erase 从前往后搬，方向反了会覆盖未读元素。

手推一遍：
  以 capacity=4、空数组开始连续 push 0..4：
  1. push 0..3 一路顺序写，写完 size=4，正好等于 capacity；
  2. push 4 时 size==capacity，先换 8 长的 list 并拷走 4 个元素；
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
"""

INIT_CAP = 4


class DynamicArray:
    def __init__(self):
        # 长度当 capacity 用，前 _size 个是有效元素
        self._data = [0] * INIT_CAP
        self._size = 0

    def size(self):
        return self._size

    def capacity(self):
        return len(self._data)

    def empty(self):
        return self._size == 0

    def _grow(self):
        nd = [0] * (len(self._data) * 2)
        for i in range(self._size):
            nd[i] = self._data[i]
        self._data = nd

    def _shrink(self):
        # 一次可能连降两级（比如从 16 掉到 4），所以用 while。
        while len(self._data) >= 8 and self._size <= len(self._data) // 4:
            nd = [0] * (len(self._data) // 2)
            for i in range(self._size):
                nd[i] = self._data[i]
            self._data = nd

    def push_back(self, v):
        if self._size == len(self._data):
            self._grow()  # 先扩容，再写入
        self._data[self._size] = v
        self._size += 1

    def pop_back(self):
        if self._size == 0:
            return False
        self._size -= 1
        self._shrink()
        return True

    def insert(self, index, v):
        # index == size 等价于尾插，仍然合法；其余越界一律拒绝。
        if index < 0 or index > self._size:
            return False
        if self._size == len(self._data):
            self._grow()
        # 从后往前挪一位，给 index 腾位置；反向会覆盖还没搬的元素。
        for i in range(self._size, index, -1):
            self._data[i] = self._data[i - 1]
        self._data[index] = v
        self._size += 1
        return True

    def erase(self, index):
        if index < 0 or index >= self._size:
            return False
        # 从前往后挪，覆盖掉 index 这一格；i+1 < size 保证读不越界。
        for i in range(index, self._size - 1):
            self._data[i] = self._data[i + 1]
        self._size -= 1
        self._shrink()
        return True

    def at(self, index):
        if index < 0 or index >= self._size:
            raise IndexError("at: index out of range")
        return self._data[index]

    def find(self, v):
        for i in range(self._size):
            if self._data[i] == v:
                return i
        return -1

    def clear(self):
        self._size = 0
        self._shrink()  # 清空后通常不会马上塞满，顺手把内存还回去

    def to_list(self):
        return self._data[: self._size]

    def print(self):
        print("[", end="")
        for i in range(self._size):
            if i > 0:
                print(", ", end="")
            print(self._data[i], end="")
        print("]")


def expect(a, want):
    assert a.size() == len(want), "size mismatch"
    for i, v in enumerate(want):
        assert a.at(i) == v, "index %d mismatch" % i


def main():
    # 用例 1：连续 push 跨多次扩容，容量按 4/8/16/... 翻倍。
    a = DynamicArray()
    assert a.capacity() == 4 and a.empty()
    for i in range(100):
        a.push_back(i)
    assert a.size() == 100
    assert a.capacity() == 128  # 4 -> 8 -> 16 -> 32 -> 64 -> 128
    assert a.at(0) == 0 and a.at(99) == 99 and a.at(50) == 50
    assert a.find(42) == 42 and a.find(-1) == -1
    a.print()

    # 用例 2：insert 的头、尾、中间与越界。
    b = DynamicArray()
    b.push_back(1)
    b.push_back(2)
    b.push_back(3)
    assert b.insert(0, 0)    # 头插
    assert b.insert(4, 4)    # 尾插，index == size
    assert b.insert(2, 99)   # 中间插
    assert not b.insert(99, 7)
    assert not b.insert(-1, 7)
    expect(b, [0, 1, 99, 2, 3, 4])
    assert b.size() == 6

    # 用例 3：erase 的头、尾、中间与越界。
    assert b.erase(0)
    expect(b, [1, 99, 2, 3, 4])
    assert b.erase(4)  # 删尾（此时 size=4）
    expect(b, [1, 99, 2, 3])
    assert b.erase(1)
    expect(b, [1, 2, 3])
    assert not b.erase(3)  # 下标 == size 已越界
    assert not b.erase(-1)
    assert b.size() == 3

    # 用例 4：缩容后再 push，数据不能丢、容量要能重新长回来。
    c = DynamicArray()
    for i in range(16):
        c.push_back(i)
    assert c.capacity() == 16
    for _ in range(12):
        assert c.pop_back()
    assert c.size() == 4
    assert c.capacity() == 8  # 16 -> 8（size==4 命中 4<=16/4）
    expect(c, [0, 1, 2, 3])
    for i in range(4):
        c.push_back(100 + i)
    assert c.size() == 8 and c.capacity() == 8  # 占用率回到 1/2 不扩容
    c.push_back(200)                            # 第 9 个才扩容
    assert c.size() == 9 and c.capacity() == 16
    expect(c, [0, 1, 2, 3, 100, 101, 102, 103, 200])

    # 用例 5：空数组的每个出口。
    e = DynamicArray()
    assert e.empty() and e.size() == 0 and e.capacity() == 4
    assert not e.pop_back()  # 空数组 pop 返回 False，不抛
    assert e.find(1) == -1
    assert not e.erase(0)
    try:
        e.at(0)
        raise AssertionError("at on empty should raise IndexError")
    except IndexError:
        pass
    e.push_back(7)
    e.clear()
    assert e.size() == 0 and e.empty()
    e.push_back(8)  # 清空之后还能继续用
    expect(e, [8])

    print("all tests passed")


if __name__ == "__main__":
    main()
