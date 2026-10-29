"""
============================================================================
双端队列（Deque）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_front  均摊 O(1)  满时 2 倍扩容并重排下标
  push_back   均摊 O(1)  同循环队列的入队
  pop_front   O(1)       只挪 head
  pop_back    O(1)       只改 size，队尾由公式算出
  front       O(1)
  back        O(1)
  empty/size  O(1)
  clear       O(1)
============================================================================

用途：
  两端都要进出的场景：滑动窗口最值（单调队列）、0-1 BFS、回文判断、
  撤销栈、以及工作窃取调度器的本地队列。

核心思想：
  循环队列再加一个「往左长」的能力。队尾仍然是 (head + size - 1)，
  队头插就是把 head 往左挪一格再写；挪的时候要 (head - 1 + capacity)
  再取模，否则下标会变成 -1。于是两端增删都只动下标，不搬元素。

关键点（不变量）：
  · 队首物理下标是 head，队尾物理下标是 (head + size - 1) % capacity；
  · push_front 必须「先左移 head，再写 _data[head]」；pop_front 必须
    「先读 _data[head]，再右移 head」，两步顺序反了就会写错/读错位置；
  · 头部的下标运算一律写成 (head - 1 + capacity) % capacity。Python 的 %
    恰好返回非负余数，但这里显式加上 capacity，和另外三种语言一一对应；
  · 扩容时按逻辑顺序把 head..head+size-1 搬到新数组 0..size-1，最后才把
    head 置 0；容量翻倍后旧的取模关系全部失效。

手推一遍：
  容量 4，push_back 1、2、3、4 后 head=0、size=4，队满。
  1. pop_front 取走 1，head 变 1，size=3，逻辑内容 [2, 3, 4]；
  2. push_front(0)：head = (1-1+4)%4 = 0，写 _data[0]=0，size 回到 4，
     逻辑内容 [0, 2, 3, 4]，物理上下标 0..3 全占满；
  3. 再 push_back(5)：size==capacity 先扩容到 8，按逻辑顺序搬成
     [0, 2, 3, 4] 占下标 0..3、head 归 0，再写 _data[4]=5。

复杂度分析：
  时间：两端插入/删除都是均摊 O(1)；单次触发扩容是 O(n)，但一次扩容至少
    带来 capacity 次入队，摊下来仍是 O(1)。
  空间：O(n)，capacity 最多是 size 的 2 倍，且总是 2 的幂。
  取舍：相对 list 的优势是两端 O(1)，而 list 头插要 O(n)；代价是随机
    访问变成一次取模。
"""

DEQUE_INIT_CAP = 4


class Deque:
    def __init__(self):
        self._data = [0] * DEQUE_INIT_CAP
        self._head = 0  # 队首物理下标
        self._size = 0  # 元素个数

    def size(self):
        return self._size

    def capacity(self):
        return len(self._data)

    def head(self):
        """只为自测观察回绕，不参与逻辑。"""
        return self._head

    def empty(self):
        return self._size == 0

    def full(self):
        return self._size == len(self._data)

    def _grow(self):
        nd = [0] * (len(self._data) * 2)
        # 用旧 _head、旧 capacity 按逻辑顺序搬；搬完才能改动这两个量。
        for i in range(self._size):
            nd[i] = self._data[(self._head + i) % len(self._data)]
        self._data = nd
        self._head = 0

    def push_front(self, v):
        if self._size == len(self._data):
            self._grow()
        # 0-1 会变成 -1，必须先加 capacity 再取模
        self._head = (self._head - 1 + len(self._data)) % len(self._data)
        self._data[self._head] = v
        self._size += 1

    def push_back(self, v):
        if self._size == len(self._data):
            self._grow()
        self._data[(self._head + self._size) % len(self._data)] = v
        self._size += 1

    def pop_front(self):
        # 头删：先读再右移，否则读到的是新 head 上的元素。
        if self._size == 0:
            raise IndexError("pop_front from empty deque")
        v = self._data[self._head]
        self._head = (self._head + 1) % len(self._data)
        self._size -= 1
        return v

    def pop_back(self):
        # 尾删：只需 size 减一，把那一格「逻辑上」丢掉。
        if self._size == 0:
            raise IndexError("pop_back from empty deque")
        v = self._data[(self._head + self._size - 1) % len(self._data)]
        self._size -= 1
        return v

    def front(self):
        if self._size == 0:
            raise IndexError("front from empty deque")
        return self._data[self._head]

    def back(self):
        if self._size == 0:
            raise IndexError("back from empty deque")
        return self._data[(self._head + self._size - 1) % len(self._data)]

    def clear(self):
        self._head = 0
        self._size = 0

    def to_list(self):
        return [
            self._data[(self._head + i) % len(self._data)]
            for i in range(self._size)
        ]

    def print(self):
        print(
            "[%s] size=%d cap=%d head=%d"
            % (
                ", ".join(str(v) for v in self.to_list()),
                self._size,
                len(self._data),
                self._head,
            )
        )


def expect(q, want):
    assert q.to_list() == want, "content mismatch"
    assert q.size() == len(want), "size mismatch"


def main():
    # 用例 1：两端交替操作，顺序与两端取值都要对。
    a = Deque()
    a.push_back(3)
    a.push_front(2)
    a.push_back(4)
    a.push_front(1)
    expect(a, [1, 2, 3, 4])
    assert a.front() == 1 and a.back() == 4
    assert a.pop_front() == 1
    assert a.pop_back() == 4
    expect(a, [2, 3])
    a.push_front(1)
    a.push_back(4)
    expect(a, [1, 2, 3, 4])
    a.print()

    # 用例 2：头插到底，跨多次扩容，顺序必须是整体倒序。
    b = Deque()
    for i in range(100):
        b.push_front(i)
    assert b.size() == 100 and b.capacity() == 128
    assert b.to_list() == list(range(99, -1, -1))
    assert b.front() == 99 and b.back() == 0

    # 用例 3：关键点用例——head 不为 0 时扩容，两端数据顺序不能乱。
    c = Deque()
    for i in range(1, 5):
        c.push_back(i)  # 满，head=0
    assert c.pop_front() == 1
    assert c.pop_front() == 2  # head 走到 2
    c.push_back(5)             # 写到下标 0
    c.push_back(6)             # 写到下标 1
    expect(c, [3, 4, 5, 6])
    assert c.head() == 2 and c.full()
    c.push_back(7)  # 触发扩容
    assert c.capacity() == 8 and c.head() == 0
    expect(c, [3, 4, 5, 6, 7])
    # 扩容后立刻头插：head 必须回绕到 7，而不是变成 -1。
    c.push_front(2)
    assert c.head() == 7
    expect(c, [2, 3, 4, 5, 6, 7])
    assert c.front() == 2 and c.back() == 7
    c.print()

    # 用例 4：空队列的每个出口。
    e = Deque()
    assert e.empty() and not e.full() and e.size() == 0
    for op in (e.pop_front, e.pop_back, e.front, e.back):
        try:
            op()
            raise AssertionError("empty deque op should raise IndexError")
        except IndexError:
            pass
    expect(e, [])
    e.push_front(1)
    assert e.front() == 1 and e.back() == 1  # 单元素时两端相同
    e.clear()
    assert e.empty() and e.size() == 0 and e.head() == 0
    e.push_back(2)  # clear 之后还能继续用
    expect(e, [2])

    # 用例 5：长操作序列对拍，两端交替进出，逐步核对 size 与内容。
    q = Deque()
    model = []  # 仅用于校验：当作双端队列的参考记录
    for step in range(300):
        kind = step % 4
        if kind == 0:
            q.push_back(step)
            model.append(step)
        elif kind == 1:
            q.push_front(step)
            model.insert(0, step)
        elif kind == 2:
            if model:
                want = model.pop(0)
                assert q.pop_front() == want
        else:
            if model:
                want = model.pop()
                assert q.pop_back() == want
        assert q.size() == len(model)
        if step % 37 == 0:
            expect(q, model)
    expect(q, model)

    print("all tests passed")


if __name__ == "__main__":
    main()
