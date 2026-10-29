"""
============================================================================
循环队列（Circular Queue）
分类：手写数据结构 · 线性结构

操作复杂度：
  enqueue   均摊 O(1)  满时 2 倍扩容并重排下标
  dequeue   O(1)       只动 head，不搬元素
  front     O(1)       队首元素
  rear      O(1)       队尾元素，由 (head+size-1) 算出
  empty     O(1)
  full      O(1)
  size      O(1)
  clear     O(1)
============================================================================

用途：
  BFS 的待访问队列、任务调度、滑动窗口最值、生产者/消费者之间的环形
  缓冲（音频流、日志缓冲都是这个形状）。

核心思想：
  一块环形数组，只维护 head 和 size 两个量：队尾就是
  (head + size) % capacity，不用单独存 tail。出队只把 head 往前走一格，
  元素一个都不搬，于是 dequeue 是 O(1)，而不是 list 删除的 O(n)。

关键点（不变量）：
  · 队首逻辑下标是 head，队尾逻辑下标是 (head + size - 1) % capacity，
    一律用公式算，不要另存一个 tail 指针去维护两套状态；
  · 用 size 区分空满，而不是「留一个空槽」：留空槽法最多只能存
    capacity-1 个元素，且每次判满都要绕一圈算 (tail+1)%cap == head；
  · 下标回绕统一写 (i + 1) % capacity 这类形式，往左走必须写成
    (i - 1 + capacity) % capacity，Python 的 % 恰好返回非负余数，
    但为了和 C++/Go/Rust 写法一一对应，这里仍然显式加上 capacity；
  · 扩容时必须先按「旧 head、旧 capacity」把元素按逻辑顺序搬到新数组的
    0..size-1，循环结束后才能把 head 置 0、capacity 翻倍；先改 head
    或先改 capacity，取模映射就全错，元素顺序会乱。

手推一遍：
  容量 4，依次 enqueue 1、2、3、4 后 size=4、head=0，队满。
  1. dequeue 两次，取走 1、2，head 走到 2，size=2，逻辑内容 [3, 4]；
  2. 再 enqueue 5：写入下标 (2+2)%4 = 0；enqueue 6：写入 (2+3)%4 = 1；
     物理上是 [5, 6, 3, 4]，逻辑上仍是 [3, 4, 5, 6]，这就是回绕；
  3. 此时再 enqueue 7 就满了，触发扩容：按逻辑顺序搬出 3、4、5、6、7
     到新数组下标 0..4，head 置 0，capacity 变 8。

复杂度分析：
  时间：enqueue 均摊 O(1)（扩容搬移摊到每次入队，n 次入队总搬移小于 2n），
    dequeue/front/rear 都是 O(1)，因为不搬元素只挪下标。
  空间：O(n)，capacity 最多是 size 的 2 倍。
  稳定性：队列是 FIFO，谈不到稳定性。
  与「数组模拟队列」的对比：不做循环就得每次出队整体左移，那是 O(n)；
  循环队列把这份 O(n) 摊成 O(1)，代价只是多一次取模。
"""

QUEUE_INIT_CAP = 4


class CircularQueue:
    def __init__(self):
        self._data = [0] * QUEUE_INIT_CAP
        self._head = 0  # 队首所在的物理下标
        self._size = 0  # 元素个数，空满全靠它区分

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
        # 关键：这一步用的是旧 _head、旧 capacity，映射关系一定正确。
        for i in range(self._size):
            nd[i] = self._data[(self._head + i) % len(self._data)]
        self._data = nd
        # 搬完之后才能改 head：先改就再也算不出旧下标了。
        self._head = 0

    def enqueue(self, v):
        if self._size == len(self._data):
            self._grow()  # 先扩容，再写入
        # 队尾逻辑位置：head 往右数 size 格，取模把尾巴绕回头部
        self._data[(self._head + self._size) % len(self._data)] = v
        self._size += 1
        return True

    def dequeue(self):
        # 空队列抛 IndexError，调用方自己决定降级还是报错。
        if self._size == 0:
            raise IndexError("dequeue from empty queue")
        v = self._data[self._head]
        self._head = (self._head + 1) % len(self._data)  # 只挪下标，不搬元素
        self._size -= 1
        return v

    def front(self):
        if self._size == 0:
            raise IndexError("front from empty queue")
        return self._data[self._head]

    def rear(self):
        if self._size == 0:
            raise IndexError("rear from empty queue")
        # size-1 是队尾的偏移量；size >= 1 在这里已由判空保证
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
    # 用例 1：出两个进两个，反复 6 轮，让 head 回绕多圈（容量保持 4）。
    a = CircularQueue()
    m = []  # 仅用于校验：当作 FIFO 的参考记录
    nxt = 1
    for _ in range(4):
        assert a.enqueue(nxt)
        m.append(nxt)
        nxt += 1
    assert a.full() and a.size() == 4
    for rnd in range(6):
        for _ in range(2):
            v = a.dequeue()
            assert v == m[0]  # 回绕之后出队顺序依然正确
            m.pop(0)
        for _ in range(2):
            assert a.enqueue(nxt)
            m.append(nxt)
            nxt += 1
        assert a.full() and a.size() == 4
        expect(a, m)
        assert a.head() == ((rnd + 1) * 2) % 4  # head 每轮前进两格并回绕
    assert a.capacity() == 4  # 全程没超过容量，没触发扩容
    a.print()

    # 用例 2：半满时再入队，物理下标会绕回头部。
    b = CircularQueue()
    for i in range(1, 5):
        b.enqueue(i)  # 满，head=0
    assert b.dequeue() == 1
    assert b.dequeue() == 2  # head 走到 2
    assert b.enqueue(5)      # 写到下标 0
    assert b.enqueue(6)      # 写到下标 1
    expect(b, [3, 4, 5, 6])
    assert b.head() == 2 and b.full()
    b.print()

    # 用例 3：满队列再入队触发扩容，head 归零且顺序不变。
    assert b.enqueue(7)
    assert b.capacity() == 8
    assert b.head() == 0
    expect(b, [3, 4, 5, 6, 7])
    assert b.front() == 3  # 扩容后队首没变
    assert b.rear() == 7
    b.print()
    # 扩容后继续绕：出两个再进两个，不应该再扩容。
    assert b.dequeue() == 3
    assert b.dequeue() == 4
    assert b.enqueue(8)
    assert b.enqueue(9)
    assert b.capacity() == 8
    expect(b, [5, 6, 7, 8, 9])

    # 用例 4：空队列的每个出口。
    e = CircularQueue()
    assert e.empty() and not e.full() and e.size() == 0
    for op in (e.dequeue, e.front, e.rear):
        try:
            op()
            raise AssertionError("empty queue op should raise IndexError")
        except IndexError:
            pass
    expect(e, [])
    assert e.enqueue(1)
    assert e.front() == 1 and e.rear() == 1  # 单元素时队首队尾相同
    e.clear()
    assert e.empty() and e.size() == 0 and e.head() == 0
    assert e.enqueue(2)  # clear 之后还能继续用
    expect(e, [2])

    # 用例 5：长操作序列，size 与出队顺序逐步核对（参考模型是 list）。
    q = CircularQueue()
    model = []
    pushed = 0
    for step in range(200):
        if step % 3 != 2:
            assert q.enqueue(pushed)
            model.append(pushed)
            pushed += 1
        elif model:
            want = model.pop(0)  # 参考模型可以直接 O(n) 删
            assert q.dequeue() == want
        assert q.size() == len(model)
    expect(q, model)

    print("all tests passed")


if __name__ == "__main__":
    main()
