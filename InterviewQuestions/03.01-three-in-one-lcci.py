"""
============================================================================
面试题 03.01. 三合一（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/three-in-one-lcci/
分类：栈 · 设计（一个数组实现三个栈）

三合一。描述如何只用一个数组来实现三个栈。

你应该实现 push(stackNum, value)、pop(stackNum)、isEmpty(stackNum)、
peek(stackNum) 方法。stackNum 表示栈下标，value 表示压入的值。

构造函数会传入一个 stackSize 参数，代表每个栈的大小。

示例 1：
输入：
["TripleInOne", "push", "push", "pop", "pop", "pop", "isEmpty"]
[[1], [0, 1], [0, 2], [0], [0], [0], [0]]
输出：
[null, null, null, 1, -1, -1, true]
说明：当栈为空时 pop, peek 返回 -1，当栈满时 push 不压入元素。

示例 2：
输入：
["TripleInOne", "push", "push", "push", "pop", "pop", "pop", "peek"]
[[2], [0, 1], [0, 2], [0, 3], [0], [0], [0], [0]]
输出：
[null, null, null, null, 2, 1, -1, -1]

提示：
0 <= stackNum <= 2
"""
from typing import List


"""
说明：本题在 leetcode.cn 上没有官方题解文章，下面按《程序员面试金典》的标准做法给出
两种实现，并把真正满足「只用一个数组」的那一种标为最优解。

解法一：三个独立数组【非最优，不满足题意】

最直白的写法就是开三个 list，每个独立管理自己的栈顶。push / pop / peek /
isEmpty 都是 O(1)，但用了三块数组，与题面「只用一个数组」的要求不符，这里只作为
对照，用来说明为什么需要「分区 + 下标换算」。

复杂度分析：
  时间：四个操作均 O(1)
  空间：总 O(3 * stackSize)，但分散在三个数组里

解法二：一个数组分三段 + 三个栈顶指针【最优解】

既然每个栈的大小固定为 stackSize，那就把长度为 3 * stackSize 的一个数组切成三段：
第 s 个栈独占 [s * stackSize, (s + 1) * stackSize) 这一段。
再维护三个「栈顶下标」top[s]，初值取 s * stackSize - 1 —— 这样偶数段时 top[s] 是
段首前一格，「top[s] >= s * stackSize」就等价于「第 s 个栈非空」。

关键点：
  · 判空用 top[s] >= s * stackSize（等价于 top[s] - s * stackSize >= 0），
    不要写成 top[s] >= 0，否则第 1、2 段会被误判为非空
  · 判满用 top[s] < (s + 1) * stackSize - 1，满了就丢弃新元素（题面要求「当栈满时
    push 不压入元素」），不要扩容
  · stackSize == 0 是合法输入：此时数组为空，push 一律被丢弃，pop / peek 一律返回 -1
  · 三个栈共用一块内存但互不越界，这就是「一个数组实现三个栈」的标准答案

以示例 2（stackSize = 2）为例，数组长 6：
  push(0, 1)：top[0] 由 -1 变 0，data[0] = 1
  push(0, 2)：top[0] 变 1，data[1] = 2（第 0 段 [0, 1] 已满）
  push(0, 3)：top[0] = 1 == (0+1)*2 - 1，已满 → 丢弃
  pop(0) → 返回 data[1] = 2，top[0] 变 0
  pop(0) → 返回 data[0] = 1，top[0] 变 -1（空）
  pop(0) → 空 → 返回 -1
  peek(0) → 空 → 返回 -1

复杂度分析：
  时间：四个操作均 O(1)
  空间 O(3 * stackSize)：一个数组 + 三个下标
"""


class TripleInOne:
    def __init__(self, stackSize: int):
        self.stack_size = stackSize
        # 唯一的数组，切三段给三个栈用
        self.data: List[int] = [0] * (3 * stackSize)
        # 每个栈的栈顶下标，空栈时是段首前一格
        self.top: List[int] = [s * stackSize - 1 for s in range(3)]

    def push(self, stackNum: int, value: int) -> None:
        # 满了就丢弃（题面要求），不扩容
        if self.top[stackNum] < (stackNum + 1) * self.stack_size - 1:
            self.top[stackNum] += 1
            self.data[self.top[stackNum]] = value

    def pop(self, stackNum: int) -> int:
        if self.isEmpty(stackNum):
            return -1
        value = self.data[self.top[stackNum]]
        self.top[stackNum] -= 1
        return value

    def peek(self, stackNum: int) -> int:
        if self.isEmpty(stackNum):
            return -1
        return self.data[self.top[stackNum]]

    def isEmpty(self, stackNum: int) -> bool:
        return self.top[stackNum] < stackNum * self.stack_size


"""
解法一：三个独立 list（仅作对照，不满足「只用一个数组」的要求）
"""


class TripleInOneThreeArrays:
    def __init__(self, stackSize: int):
        self.stack_size = stackSize
        self.data: List[List[int]] = [[], [], []]

    def push(self, stackNum: int, value: int) -> None:
        if len(self.data[stackNum]) < self.stack_size:
            self.data[stackNum].append(value)

    def pop(self, stackNum: int) -> int:
        if self.isEmpty(stackNum):
            return -1
        return self.data[stackNum].pop()

    def peek(self, stackNum: int) -> int:
        if self.isEmpty(stackNum):
            return -1
        return self.data[stackNum][-1]

    def isEmpty(self, stackNum: int) -> bool:
        return not self.data[stackNum]


def run(triple, ops: List[tuple]) -> List[object]:
    out: List[object] = []
    for name, stackNum, value in ops:
        if name == "push":
            triple.push(stackNum, value)
            out.append(None)
        elif name == "pop":
            out.append(triple.pop(stackNum))
        elif name == "peek":
            out.append(triple.peek(stackNum))
        else:
            out.append(triple.isEmpty(stackNum))
    return out


def check(triple, ops: List[tuple], want: List[object]) -> None:
    got = run(triple, ops)
    assert got == want, f"run({ops}) = {got}, want {want}"


# 随机交叉验证：用三个 list 当对照栈
def cross_check(triple, stack_size: int) -> None:
    expect: List[List[int]] = [[], [], []]
    seed = 20240607 + stack_size
    for _ in range(3000):
        seed = (seed * 1103515245 + 12345) % (2**32)
        r = (seed >> 16) % 10
        s = (seed >> 8) % 3
        if r < 5:
            value = (seed >> 4) % 100
            triple.push(s, value)
            # 对照栈同样按「满了丢弃」
            if len(expect[s]) < stack_size:
                expect[s].append(value)
        elif r < 8:
            want = expect[s].pop() if expect[s] else -1
            assert triple.pop(s) == want
        elif r < 9:
            want = expect[s][-1] if expect[s] else -1
            assert triple.peek(s) == want
        else:
            assert triple.isEmpty(s) == (not expect[s])


if __name__ == "__main__":
    # 示例 1：stackSize = 1，push 第二个元素被丢弃
    ops1 = [
        ("push", 0, 1),
        ("push", 0, 2),
        ("pop", 0, 0),
        ("pop", 0, 0),
        ("pop", 0, 0),
        ("isEmpty", 0, 0),
    ]
    want1 = [None, None, 1, -1, -1, True]
    check(TripleInOne(1), ops1, want1)
    check(TripleInOneThreeArrays(1), ops1, want1)

    # 示例 2：stackSize = 2，压三个只留两个
    ops2 = [
        ("push", 0, 1),
        ("push", 0, 2),
        ("push", 0, 3),
        ("pop", 0, 0),
        ("pop", 0, 0),
        ("pop", 0, 0),
        ("peek", 0, 0),
    ]
    want2 = [None, None, None, 2, 1, -1, -1]
    check(TripleInOne(2), ops2, want2)
    check(TripleInOneThreeArrays(2), ops2, want2)

    # 边界：stackSize = 0，任何 push 都丢弃、任何取数都返回 -1
    ops0 = [("push", 1, 7), ("pop", 1, 0), ("peek", 1, 0), ("isEmpty", 1, 0)]
    want0 = [None, -1, -1, True]
    check(TripleInOne(0), ops0, want0)
    check(TripleInOneThreeArrays(0), ops0, want0)

    # 边界：三个栈互不干扰，各自后进先出
    ops_mix = [
        ("push", 0, 1),
        ("push", 1, 2),
        ("push", 2, 3),
        ("push", 1, 4),
        ("pop", 1, 0),
        ("pop", 0, 0),
        ("pop", 2, 0),
        ("pop", 1, 0),
        ("pop", 0, 0),
    ]
    want_mix = [None, None, None, None, 4, 1, 3, 2, -1]
    check(TripleInOne(2), ops_mix, want_mix)
    check(TripleInOneThreeArrays(2), ops_mix, want_mix)

    # 随机交叉验证
    for stack_size in (0, 1, 3, 10):
        cross_check(TripleInOne(stack_size), stack_size)
        cross_check(TripleInOneThreeArrays(stack_size), stack_size)

    print("all tests passed")
