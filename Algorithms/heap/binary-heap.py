"""
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
  面试常要求手写堆，而不是直接调用 heapq。

核心思想：
  把完全二叉树按层序塞进列表，父子关系用下标算出来，省掉指针。
  push 只在末尾追加再上浮；pop 把末尾元素搬到根再下沉，两者都只沿着
  一条从根到叶的路径比较，所以是 O(log n)。

关键点（不变量）：
  · 大根堆：每个父节点 >= 它的两个子节点；小根堆方向相反。
  · 0-based 下标：parent = (i-1)//2，children = 2*i+1、2*i+2。
  · 上浮看到父节点已满足堆序就停；下沉看到两个孩子都不更优就停。
  · build 必须从最后一个非叶节点 n//2-1 倒着下沉，才是 O(n)。

手推一遍：
  以小根堆为例，依次 push 3、1、4：
  1. push 3 -> [3]。
  2. push 1 -> [3,1]，1 比父 3 小，交换 -> [1,3]。
  3. push 4 -> [1,3,4]，4 比父 3 大，停。
  接着 pop：根 1 取出，末尾 4 补到根 -> [4,3]，
  4 与唯一的（较小的）孩子 3 交换 -> [3,4]。

复杂度分析：
  时间 O(log n)：push/pop 各沿树高走一条链，树高 log2(n)。
  空间 O(n)：一个列表存 n 个元素，不额外拷贝。
  稳定性：不稳定，等值元素的相对次序由交换决定。
"""
import random


class BinaryHeap:
    def __init__(self, is_max_heap: bool = True) -> None:
        self._data = []
        self._is_max = is_max_heap

    def size(self) -> int:
        return len(self._data)

    def empty(self) -> bool:
        return not self._data

    def clear(self) -> None:
        self._data.clear()

    def push(self, value: int) -> None:
        self._data.append(value)
        self._sift_up(len(self._data) - 1)

    # 空堆时返回 None，调用方不会被塞一个假值。
    def pop(self):
        if not self._data:
            return None
        top = self._data[0]
        last = self._data.pop()
        if self._data:
            self._data[0] = last
            self._sift_down(0)
        return top

    def peek(self):
        if not self._data:
            return None
        return self._data[0]

    # 自底向上建堆：叶子不用下沉，从最后一个非叶节点往回扫，
    # 每个节点下沉的高度之和是 O(n)，比逐个 push 的 O(n log n) 更快。
    def build(self, values) -> None:
        self._data = list(values)
        for i in range(len(self._data) // 2 - 1, -1, -1):
            self._sift_down(i)

    # 仅供自测：导出内部列表的副本，用于直接检查不变量。
    def snapshot(self):
        return list(self._data)

    # 大根堆里 a 该在上面；小根堆里 b 该在上面。
    def _higher(self, a: int, b: int) -> bool:
        return a > b if self._is_max else a < b

    def _sift_up(self, i: int) -> None:
        while i > 0:
            parent = (i - 1) // 2
            if not self._higher(self._data[i], self._data[parent]):
                break
            self._data[i], self._data[parent] = (
                self._data[parent],
                self._data[i],
            )
            i = parent

    def _sift_down(self, i: int) -> None:
        n = len(self._data)
        while True:
            best = i
            left = 2 * i + 1
            right = 2 * i + 2
            if left < n and self._higher(self._data[left], self._data[best]):
                best = left
            if right < n and self._higher(self._data[right], self._data[best]):
                best = right
            # best 还是自己，说明两个孩子都不比它更优，再走就破坏堆序。
            if best == i:
                break
            self._data[i], self._data[best] = self._data[best], self._data[i]
            i = best


def heap_ok(a, is_max: bool) -> bool:
    """直接扫一遍列表，验证「父节点与两个孩子」的堆序不变量。"""
    n = len(a)
    for i in range(n):
        left = 2 * i + 1
        right = 2 * i + 2
        if left < n and (a[left] > a[i] if is_max else a[left] < a[i]):
            return False
        if right < n and (a[right] > a[i] if is_max else a[right] < a[i]):
            return False
    return True


def drain(heap: BinaryHeap):
    out = []
    while True:
        value = heap.pop()
        if value is None:
            return out
        out.append(value)


def test_normal() -> None:
    heap = BinaryHeap(True)
    for v in [5, 3, 8, 1, 9, 2]:
        heap.push(v)
    assert heap.size() == 6

    assert heap.peek() == 9
    assert heap.pop() == 9
    assert heap.pop() == 8
    assert heap.size() == 4

    heap.push(7)
    assert drain(heap) == [7, 5, 3, 2, 1]
    assert heap.empty()

    small = BinaryHeap(False)
    for v in [5, 3, 8, 1, 9, 2]:
        small.push(v)
    assert drain(small) == [1, 2, 3, 5, 8, 9]


def test_empty_and_single() -> None:
    heap = BinaryHeap(True)
    assert heap.pop() is None
    assert heap.peek() is None
    assert heap.empty() and heap.size() == 0

    heap.push(-7)
    assert heap.size() == 1
    assert heap.pop() == -7
    assert heap.empty()
    assert heap.pop() is None


def test_build_property() -> None:
    heap = BinaryHeap(True)
    heap.build([7, 2, 9, 4, 4, 1, 8])
    assert heap.size() == 7
    assert heap_ok(heap.snapshot(), True)

    # 重复 build 必须覆盖旧内容，不能把上一次的元素留在后面。
    heap.build([3, 1, 2])
    assert heap.size() == 3
    assert heap_ok(heap.snapshot(), True)
    assert drain(heap) == [3, 2, 1]

    # 空列表 build 也不能越界（n//2-1 = -1，循环一次都不进）。
    heap.build([])
    assert heap.empty() and heap_ok(heap.snapshot(), True)


def test_duplicates() -> None:
    small = BinaryHeap(False)
    for _ in range(6):
        small.push(6)
    assert small.size() == 6
    got = drain(small)
    assert got == [6, 6, 6, 6, 6, 6]

    many = BinaryHeap(True)
    many.build([4] * 16)
    assert heap_ok(many.snapshot(), True)
    assert len(drain(many)) == 16


def test_random_cross_check() -> None:
    rng = random.Random(20240501)
    for round_index in range(5):
        n = 20 + round_index * 37
        raw = [rng.randint(-1000, 1000) for _ in range(n)]
        big = BinaryHeap(True)
        small = BinaryHeap(False)
        for v in raw:
            big.push(v)
            small.push(v)
        assert big.size() == n and small.size() == n

        # 仅用于校验：实现部分没有使用内置排序。
        asc = sorted(raw)
        assert drain(big) == asc[::-1]
        assert drain(small) == asc
        assert big.empty() and small.empty()

    # 交错 push/pop：size 始终等于有效元素个数。
    heap = BinaryHeap(True)
    heap.push(4)
    heap.push(10)
    assert heap.pop() == 10
    heap.push(1)
    heap.push(7)
    assert heap.size() == 3
    assert heap.pop() == 7
    heap.clear()
    assert heap.empty() and heap.size() == 0


def main() -> None:
    test_normal()
    test_empty_and_single()
    test_build_property()
    test_duplicates()
    test_random_cross_check()
    print("all tests passed")


if __name__ == "__main__":
    main()
