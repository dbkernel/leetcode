"""
============================================================================
155. 最小栈（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/min-stack/
分类：栈 · 设计（辅助栈同步记录最小值）

设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

实现 MinStack 类:
  · MinStack() 初始化堆栈对象。
  · void push(int value) 将元素 value 推入堆栈。
  · void pop() 删除堆栈顶部的元素。
  · int top() 获取堆栈顶部的元素。
  · int getMin() 获取堆栈中的最小元素。

示例 1:
输入：
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

输出：
[null,null,null,null,-3,null,0,-2]

解释：
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.

提示：
-2^31 <= val <= 2^31 - 1
pop、top 和 getMin 操作总是在 非空栈 上调用
push, pop, top, and getMin最多被调用 3 * 10^4 次
"""
import math
from typing import List


"""
解法：辅助栈【最优解】

官方题解只给出这一种方法，也就是本题的最优解：四个操作全部 O(1)，空间 O(n)。

为什么慢：如果每次 get_min 都遍历一遍栈，单次就是 O(n)；若用一个变量只记「当前
最小值」，那么当这个最小值被 pop 掉之后，就再也找不回「第二小」，除非重新扫一遍。

换个角度：注意栈的先进后出性质 —— 元素 a 入栈时栈里已有 b、c、d，那么只要 a 还在栈中，
b、c、d 就一定还在。换句话说，「a 在栈中的这段期间，栈内最小值是固定的」。
于是可以让每个元素随身携带「它入栈那一刻的栈内最小值」，再把这份信息按相同的顺序
存进一个辅助栈。

关键点：
  · 辅助栈与数据栈严格同步：push 一起 push，pop 一起 pop，时刻保持一一对应
  · push 时压入 min(当前最小值, 新元素)；新元素不比当前最小值更小，就沿用旧的最小值
  · 辅助栈预置一个 inf 作为哨兵，省掉第一次 push 的判空特判
  · get_min 直接返回辅助栈栈顶，无需任何遍历
  · 题面保证 pop / top / get_min 都在非空栈上调用，所以不必额外判空

以示例（push -2、push 0、push -3、getMin、pop、top、getMin）为例：
  push(-2)：数据栈 [-2]，辅助栈 [inf, -2]
  push(0) ：数据栈 [-2, 0]，辅助栈 [..., -2, min(-2, 0) = -2]
  push(-3)：数据栈 [-2, 0, -3]，辅助栈 [..., -2, min(-2, -3) = -3]
  getMin → 辅助栈栈顶 -3
  pop    ：两栈同时弹出 → 数据栈 [-2, 0]，辅助栈 [..., -2]
  top    → 数据栈栈顶 0
  getMin → 辅助栈栈顶 -2

复杂度分析：
  时间：push / pop / top / get_min 均为 O(1)
  空间 O(n)：辅助栈与数据栈等长
"""


class MinStack:
    def __init__(self):
        self.data_stack: List[int] = []
        # 哨兵：让第一次 push 不必特判空栈
        self.min_stack: List[float] = [math.inf]

    def push(self, value: int) -> None:
        self.data_stack.append(value)
        # 每个元素随身携带「入栈这一刻的栈内最小值」
        self.min_stack.append(min(self.min_stack[-1], value))

    def pop(self) -> None:
        self.data_stack.pop()
        # 两个栈必须同步弹出，才能保持一一对应
        self.min_stack.pop()

    def top(self) -> int:
        return self.data_stack[-1]

    def getMin(self) -> int:
        return int(self.min_stack[-1])


def run(ops: List[tuple]) -> List[object]:
    obj = MinStack()
    out: List[object] = []
    for name, value in ops:
        if name == "push":
            obj.push(value)
            out.append(None)
        elif name == "pop":
            obj.pop()
            out.append(None)
        elif name == "top":
            out.append(obj.top())
        else:
            out.append(obj.getMin())
    return out


def check(ops: List[tuple], want: List[object]) -> None:
    got = run(ops)
    assert got == want, f"run({ops}) = {got}, want {want}"


# 随机交叉验证：用 list 当对照，get_min 每次线性求最小
def cross_check() -> None:
    min_stack = MinStack()
    expect: List[int] = []
    seed = 20240607
    for _ in range(3000):
        seed = (seed * 1103515245 + 12345) % (2**32)
        r = (seed >> 16) % 10
        if r < 5 or not expect:
            value = (seed >> 8) % 200 - 100
            min_stack.push(value)
            expect.append(value)
        elif r < 7:
            min_stack.pop()
            expect.pop()
        elif r < 9:
            assert min_stack.top() == expect[-1]
        else:
            assert min_stack.getMin() == min(expect)


if __name__ == "__main__":
    # 示例 1
    check(
        [
            ("push", -2),
            ("push", 0),
            ("push", -3),
            ("getMin", 0),
            ("pop", 0),
            ("top", 0),
            ("getMin", 0),
        ],
        [None, None, None, -3, None, 0, -2],
    )

    # 边界：单元素，top 与 getMin 相同
    check([("push", 7), ("top", 0), ("getMin", 0)], [None, 7, 7])

    # 边界：重复最小值，pop 一个后最小值仍应不变
    check(
        [("push", 1), ("push", 1), ("getMin", 0), ("pop", 0), ("getMin", 0)],
        [None, None, 1, None, 1],
    )

    # 边界：最小值被弹出后要能恢复到第二小
    check(
        [
            ("push", 3),
            ("push", 1),
            ("push", 2),
            ("getMin", 0),
            ("pop", 0),
            ("pop", 0),
            ("getMin", 0),
        ],
        [None, None, None, 1, None, None, 3],
    )

    # 边界：极端值
    check(
        [
            ("push", 2**31 - 1),
            ("push", -(2**31)),
            ("getMin", 0),
            ("pop", 0),
            ("getMin", 0),
        ],
        [None, None, -(2**31), None, 2**31 - 1],
    )

    # 随机交叉验证
    cross_check()

    print("all tests passed")
