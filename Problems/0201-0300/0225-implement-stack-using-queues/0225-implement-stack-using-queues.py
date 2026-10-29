"""
============================================================================
225. 用队列实现栈（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/implement-stack-using-queues/
分类：队列 · 设计（队列模拟栈）

请你仅使用两个队列实现一个后入先出（LIFO）的栈，并支持普通栈的全部四种操作
（push、top、pop 和 empty）。

实现 MyStack 类：
  · void push(int x) 将元素 x 压入栈顶。
  · int pop() 移除并返回栈顶元素。
  · int top() 返回栈顶元素。
  · boolean empty() 如果栈是空的，返回 true ；否则，返回 false 。

注意：
  · 你只能使用队列的标准操作 —— 也就是 push to back、peek/pop from front、
    size 和 is empty 这些操作。
  · 你所使用的语言也许不支持队列。 你可以使用 list （列表）或者 deque
    （双端队列）来模拟一个队列 , 只要是标准的队列操作即可。

示例：
输入：
["MyStack", "push", "push", "top", "pop", "empty"]
[[], [1], [2], [], [], []]
输出：
[null, null, null, 2, 2, false]

解释：
MyStack myStack = new MyStack();
myStack.push(1);
myStack.push(2);
myStack.top(); // 返回 2
myStack.pop(); // 返回 2
myStack.empty(); // 返回 False

提示：
1 <= x <= 9
最多调用 100 次 push、pop、top 和 empty
每次调用 pop 和 top 都保证栈不为空

进阶：你能否仅用一个队列来实现栈。
"""
from collections import deque
from typing import Deque, List


"""
解法一：两个队列【非最优】

官方方法一。queue1 存栈内全部元素，queue2 作为入栈的辅助队列。
每来一个新元素，先把它塞进 queue2，再把 queue1 里的元素整体搬到 queue2 的尾巴上，
最后交换两个队列 —— 这样 queue1 的队首永远是栈顶，pop / top 直接取队首即可。

关键点：
  · 「搬运」是让新元素排到队首的唯一手段：先把新元素放进去，再把旧元素全部
    接到它后面
  · 交换 queue1 与 queue2 之后，queue2 天然变空，省去手动清理
  · 栈是否为空只需看 queue1

以 push(1)、push(2) 为例：
  push(1)：queue2 = [1]，queue1 空 → 交换 → queue1 = [1]，queue2 = []
  push(2)：queue2 = [2]，把 queue1 的 1 搬到 queue2 → queue2 = [2, 1] → 交换
           得 queue1 = [2, 1]，队首 2 正是栈顶
  pop：取 queue1 队首 2，剩 [1]

复杂度分析：
  时间：push O(n)，pop / top / empty O(1)
  空间 O(n)：两个队列合计存放 n 个元素（其中一个为空）
"""


class MyStackTwoQueues:
    def __init__(self):
        self.queue1: Deque[int] = deque()
        self.queue2: Deque[int] = deque()

    def push(self, x: int) -> None:
        # 新元素先进辅助队列，再把旧元素整个接到它后面
        self.queue2.append(x)
        while self.queue1:
            self.queue2.append(self.queue1.popleft())
        # 交换后 queue2 天然为空
        self.queue1, self.queue2 = self.queue2, self.queue1

    def pop(self) -> int:
        return self.queue1.popleft()

    def top(self) -> int:
        return self.queue1[0]

    def empty(self) -> bool:
        return not self.queue1


"""
解法二：一个队列【最优解】

官方方法二。既然每次入栈都要把「除了新元素之外的所有元素」轮转一遍，那么只需要
一个队列就够：记住入队前的元素个数 n，把新元素入队，再把这 n 个旧元素依次
出队再入队（相当于把队首搬到队尾），轮转结束后新元素自然来到了队首。

关键点：
  · n 一定要在入队之前取，否则新元素也会被算进去，多轮转一次就错位
  · 轮转 n 次（而不是 n + 1 次），刚好把新元素顶到队首
  · 与解法一相比时间同为 O(n) 入栈，但少了一个队列，空间常数更小

以 push(1)、push(2) 为例：
  push(1)：n = 0，入队后 q = [1]，轮转 0 次 → q = [1]
  push(2)：n = 1，入队后 q = [1, 2]，轮转 1 次（1 搬到队尾）→ q = [2, 1]
           队首 2 即栈顶
  pop：取队首 2，剩 [1]
  empty：q 非空 → False

复杂度分析：
  时间：push O(n)，pop / top / empty O(1)
  空间 O(n)：一个队列存放 n 个元素
"""


class MyStack:
    def __init__(self):
        self.q: Deque[int] = deque()

    def push(self, x: int) -> None:
        # n 必须在入队之前取，避免把新元素也轮转一遍
        n = len(self.q)
        self.q.append(x)
        for _ in range(n):
            self.q.append(self.q.popleft())

    def pop(self) -> int:
        return self.q.popleft()

    def top(self) -> int:
        return self.q[0]

    def empty(self) -> bool:
        return not self.q


# 一次操作：(name, value)，name 为 push/pop/top/empty
def run(stack, ops: List[tuple]) -> List[object]:
    out: List[object] = []
    for name, value in ops:
        if name == "push":
            stack.push(value)
            out.append(None)
        elif name == "pop":
            out.append(stack.pop())
        elif name == "top":
            out.append(stack.top())
        else:
            out.append(stack.empty())
    return out


def check(stack, ops: List[tuple], want: List[object]) -> None:
    got = run(stack, ops)
    assert got == want, f"run({ops}) = {got}, want {want}"


# 随机交叉验证：用 Python 列表当对照栈
def cross_check(stack) -> None:
    expect: List[int] = []
    seed = 20240607
    for _ in range(2000):
        seed = (seed * 1103515245 + 12345) % (2**32)
        r = (seed >> 16) % 10
        if r < 5 or not expect:
            value = (seed >> 8) % 9 + 1
            stack.push(value)
            expect.append(value)
        elif r < 8:
            assert stack.pop() == expect.pop()
        else:
            assert not stack.empty()
            assert stack.top() == expect[-1]
    assert stack.empty() == (not expect)


if __name__ == "__main__":
    # 示例
    sample = [("push", 1), ("push", 2), ("top", 0), ("pop", 0), ("empty", 0)]
    want = [None, None, 2, 2, False]
    check(MyStack(), sample, want)
    check(MyStackTwoQueues(), sample, want)

    # 边界：只判空一次
    check(MyStack(), [("empty", 0)], [True])
    check(MyStackTwoQueues(), [("empty", 0)], [True])

    # 边界：压满再逐个弹出，顺序必须是后进先出
    lifo = [
        ("push", 1),
        ("push", 2),
        ("push", 3),
        ("pop", 0),
        ("pop", 0),
        ("pop", 0),
        ("empty", 0),
    ]
    lifo_want = [None, None, None, 3, 2, 1, True]
    check(MyStack(), lifo, lifo_want)
    check(MyStackTwoQueues(), lifo, lifo_want)

    # 随机交叉验证
    cross_check(MyStack())
    cross_check(MyStackTwoQueues())

    print("all tests passed")
