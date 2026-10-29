"""
============================================================================
手写优先队列（Priority Queue）
分类：手写数据结构 · 堆与优先队列
操作复杂度：
  push   O(log n)  追加到末尾后上浮
  pop    O(log n)  末尾补根后下沉
  peek   O(1)      根就是当前最优任务
  size/empty O(1)
============================================================================

用途：
  任务调度、事件模拟、带权最短路的取最小边。面试里常接着问一句
  「同优先级怎么保证先来先服务」，这就是下面的稳定化写法。

核心思想：
  和二叉堆一样用列表存完全二叉树，但比较的是二元组 (优先级, 插入序号)：
  优先级大的先出；优先级相同时插入序号小的先出。序号随 push 单调递增，
  于是同优先级天然按入队顺序出队，不需要额外队列。

关键点（不变量）：
  · 比较必须两级：先比 priority，相等再比 seq，且 seq 小的优先。
    只比 priority 时，等值元素的先后由堆的交换决定，顺序不稳定。
  · seq 由队列自己发号，调用方不能自己传，否则稳定性无从保证。
  · 上浮/下沉只走一条根到叶的路径，停的条件是「父不再更优」。
  · pop 后必须先把末尾元素搬到根再下沉，不能留空洞。

手推一遍：
  任务依次入队：(2,a)、(3,b)、(3,c)、(1,d)，序号分别是 0..3。
  1. a 进 -> 根 a。
  2. b 优先级 3 比 a 的 2 大，b 上浮成根。
  3. c 与 b 同级但序号 2 更大，b 仍在上，c 挂在下面。
  4. d 优先级最小，沉在叶子上。
  出队顺序：b(3,#1)、c(3,#2)、a(2,#0)、d(1,#3)，即 b c a d。

复杂度分析：
  时间 O(log n)：每次 push/pop 只沿一条链走，链长不超过树高 log2(n)。
  空间 O(n)：一个任务列表，元素为 (priority, seq, id) 三个 int。
  稳定性：稳定（靠 seq 次关键字），不是排序算法本身稳定。
"""
import random


class PriorityQueue:
    def __init__(self) -> None:
        # 每个元素是 (priority, seq, id)，只用元组当记录，不借用堆。
        self._data = []
        self._next_seq = 0

    def size(self) -> int:
        return len(self._data)

    def empty(self) -> bool:
        return not self._data

    def clear(self) -> None:
        self._data.clear()
        self._next_seq = 0

    def push(self, priority: int, task_id: int) -> None:
        self._data.append((priority, self._next_seq, task_id))
        self._next_seq += 1
        self._sift_up(len(self._data) - 1)

    # 空队列返回 None，调用方不用猜一个默认优先级。
    def pop(self):
        if not self._data:
            return None
        top = self._data[0]
        last = self._data.pop()
        if self._data:
            self._data[0] = last
            self._sift_down(0)
        return top[0], top[2]

    def peek(self):
        if not self._data:
            return None
        return self._data[0][0], self._data[0][2]

    # 先比优先级，再比插入序号：这就是「堆 + 稳定化」的全部秘密。
    @staticmethod
    def _higher(a, b) -> bool:
        if a[0] != b[0]:
            return a[0] > b[0]
        return a[1] < b[1]

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
            # 两个孩子都不更优就停，再往下扫只会破坏堆序。
            if best == i:
                break
            self._data[i], self._data[best] = self._data[best], self._data[i]
            i = best


def schedule(priorities, names):
    """按优先级从高到低输出任务名，同级保持入队顺序。"""
    queue = PriorityQueue()
    for index, priority in enumerate(priorities):
        queue.push(priority, index)

    order = []
    while True:
        item = queue.pop()
        if item is None:
            return order
        order.append(names[item[1]])


def test_normal() -> None:
    got = schedule(
        [3, 1, 3, 2],
        ["write", "coffee", "review", "test"],
    )
    assert got == ["write", "review", "test", "coffee"]
    print("schedule order:", " ".join(got))


def test_empty_and_single() -> None:
    queue = PriorityQueue()
    assert queue.pop() is None
    assert queue.peek() is None
    assert queue.empty() and queue.size() == 0

    queue.push(5, 100)
    assert queue.size() == 1
    assert queue.pop() == (5, 100)
    assert queue.empty()
    assert queue.pop() is None

    # 清空后可继续使用，序号从 0 重新发，顺序依然正确。
    queue.push(1, 0)
    queue.push(2, 1)
    assert queue.pop() == (2, 1)
    queue.clear()
    assert queue.empty() and queue.size() == 0


def test_stability_on_key_point() -> None:
    # 只比优先级会挂：五个同级任务的顺序必须原样。
    queue = PriorityQueue()
    for i in range(5):
        queue.push(7, i)
    assert queue.size() == 5
    for i in range(5):
        assert queue.pop() == (7, i)
    assert queue.empty()

    # 交错 push/pop：序号继续递增，后进的同级任务排在后面。
    queue.push(4, 10)
    queue.push(4, 11)
    assert queue.pop() == (4, 10)
    queue.push(4, 12)
    assert queue.pop() == (4, 11)
    assert queue.pop() == (4, 12)
    assert queue.empty()


def test_random_priorities() -> None:
    rng = random.Random(20240501)
    n = 120
    priorities = [rng.randint(0, 5) for _ in range(n)]
    names = ["task-%d" % i for i in range(n)]

    # 期望值：优先级从高到低，同级按入队顺序扫一遍，不用任何排序。
    expect = []
    for p in range(max(priorities), min(priorities) - 1, -1):
        for i in range(n):
            if priorities[i] == p:
                expect.append(names[i])

    got = schedule(priorities, names)
    assert got == expect

    # 弹出来的优先级必须单调不增，这一条独立于期望序列。
    # 名字里带了下标，直接回查原序列。
    last = 1 << 30
    for name in got:
        index = int(name[len("task-"):])
        assert priorities[index] <= last
        last = priorities[index]


def main() -> None:
    test_normal()
    test_empty_and_single()
    test_stability_on_key_point()
    test_random_priorities()
    print("all tests passed")


if __name__ == "__main__":
    main()
