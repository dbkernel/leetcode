"""
============================================================================
面试题 03.05. 栈排序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-of-stacks-lcci/
分类：栈 · 设计（单调栈 / 一个辅助栈排序）

栈排序。 编写程序，对栈进行排序使最小元素位于栈顶。最多只能使用一个其他的临时栈
存放数据，但不得将元素复制到别的数据结构（如数组）中。该栈支持如下操作：push、
pop、peek 和 isEmpty。当栈为空时，peek 返回 -1。

示例 1：
输入：
["SortedStack", "push", "push", "peek", "pop", "peek"]
[[], [1], [2], [], [], []]
输出：
[null,null,null,1,null,2]

示例 2：
输入：
["SortedStack", "pop", "pop", "push", "pop", "isEmpty"]
[[], [], [], [1], [], []]
输出：
[null,null,null,null,null,true]

提示：
栈中的元素数目在[0, 5000]范围内。
"""
from typing import List


"""
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出两种标准实现（都只用一个临时栈，
不借助数组等其他数据结构），并把实践中更常用的「push 时维护有序」标为最优解。

解法一：push 时用辅助栈插入排序【最优解】

保持主栈 s1 从栈顶到栈底单调递增 —— 即栈顶永远是最小元素。于是 pop / peek /
isEmpty 全都可以直接 O(1) 完成，代价压在 push 上：
  1. 把 s1 中所有「比 val 小」的元素挪到辅助栈 s2（它们会被 val 压得更深）；
  2. 把 val 压入 s1；
  3. 再把 s2 里的元素全部搬回 s1。
搬完后 s1 依然「顶小底大」，不变量得以维持。

关键点：
  · 比较用 s1[-1] < val（严格小于）：相等时停下即可，相等元素谁在上都不破坏有序
  · 第 3 步必须把 s2 全部搬回，否则 s2 里还留着数据，下次 push 会乱序
  · 主栈为空时 peek 返回 -1，pop 什么也不做（题面要求）
  · 全程只用两个栈，没有借助数组，符合题面限制

以 push(1)、push(2) 为例：
  push(1)：s1 空 → s1 = [1]
  push(2)：1 < 2，把 1 挪到 s2；s1 空 → 压入 2，s1 = [2]；搬回 1 → s1 = [2, 1]
           （栈顶 1 是最小值）
  peek → 1；pop → 弹出 1；peek → 2

复杂度分析：
  时间：push 最坏 O(n)（要挪动并搬回 n 个元素），pop / peek / isEmpty 均 O(1)
  空间 O(n)：两个栈合计存放 n 个元素

解法二：pop / peek 时才整理（惰性排序）【非最优】

不变量不变（栈顶最小），但把排序推迟到真正需要读数据的时候：push 只入栈并打上
「脏」标记；pop / peek 前若为脏，就用辅助栈 s2 做一次插入排序，把 s1 整理有序。

关键点：
  · 整理时先把 s1 全部倒进 s2（顺序被反转），再逐个从 s2 取出，边取边插入到 s1 的
    正确位置：比当前元素小的一律先挪回 s2，插入后再搬回来
  · 整理完 s1 有序、s2 必为空，脏标记清除
  · 排序后连续 pop 不需要重排 —— 从有序栈顶弹元素不会破坏有序性

复杂度分析：
  时间：push O(1)，pop / peek 最坏 O(n^2)（一次整理要插入 n 个元素）
  空间 O(n)
  由于 pop / peek 往往比 push 调得更频繁，这种「把代价推到读操作」的做法整体更差，
  因此解法一才是最常用的写法。
"""


class SortedStack:
    def __init__(self):
        self.s1: List[int] = []  # 主栈，列表末尾是栈顶，恒为最小
        self.s2: List[int] = []  # 临时栈

    def push(self, val: int) -> None:
        # 比 val 小的元素要被压得更深，先挪到辅助栈
        while self.s1 and self.s1[-1] < val:
            self.s2.append(self.s1.pop())
        self.s1.append(val)
        # 辅助栈里的元素全部搬回，维持「顶小底大」
        while self.s2:
            self.s1.append(self.s2.pop())

    def pop(self) -> None:
        if self.s1:
            self.s1.pop()

    def peek(self) -> int:
        return self.s1[-1] if self.s1 else -1

    def isEmpty(self) -> bool:
        return not self.s1


class SortedStackLazy:
    def __init__(self):
        self.s1: List[int] = []
        self.s2: List[int] = []
        self.dirty = False

    def push(self, val: int) -> None:
        self.s1.append(val)
        self.dirty = True

    def pop(self) -> None:
        if not self.s1:
            return
        self._sort_if_needed()
        self.s1.pop()

    def peek(self) -> int:
        if not self.s1:
            return -1
        self._sort_if_needed()
        return self.s1[-1]

    def isEmpty(self) -> bool:
        return not self.s1

    def _sort_if_needed(self) -> None:
        if not self.dirty:
            return
        # 先把 s1 全部倒进 s2（顺序反转）
        while self.s1:
            self.s2.append(self.s1.pop())
        # 再逐个插入到 s1 的正确位置：比 t 小的元素先挪回 s2
        while self.s2:
            t = self.s2.pop()
            while self.s1 and self.s1[-1] < t:
                self.s2.append(self.s1.pop())
            self.s1.append(t)
        self.dirty = False


def run(stack, ops: List[tuple]) -> List[object]:
    out: List[object] = []
    for name, value in ops:
        if name == "push":
            stack.push(value)
            out.append(None)
        elif name == "pop":
            stack.pop()
            out.append(None)
        elif name == "peek":
            out.append(stack.peek())
        else:
            out.append(stack.isEmpty())
    return out


def check(stack, ops: List[tuple], want: List[object]) -> None:
    got = run(stack, ops)
    assert got == want, f"run({ops}) = {got}, want {want}"


# 随机交叉验证：每次 peek / pop 都和「排序后的 list」比对
def cross_check(stack) -> None:
    expect: List[int] = []  # 降序排列，末尾是最小值（即栈顶）
    seed = 20240607
    for _ in range(3000):
        seed = (seed * 1103515245 + 12345) % (2**32)
        r = (seed >> 16) % 10
        if r < 5 or not expect:
            value = (seed >> 8) % 20
            stack.push(value)
            expect.append(value)
            expect.sort(reverse=True)
        elif r < 8:
            stack.pop()
            expect.pop()
        elif r < 9:
            assert stack.peek() == expect[-1]
        else:
            assert stack.isEmpty() == (not expect)


if __name__ == "__main__":
    # 示例 1
    ops1 = [("push", 1), ("push", 2), ("peek", 0), ("pop", 0), ("peek", 0)]
    want1 = [None, None, 1, None, 2]
    check(SortedStack(), ops1, want1)
    check(SortedStackLazy(), ops1, want1)

    # 示例 2：空栈 pop 不报错，最终为空
    ops2 = [("pop", 0), ("pop", 0), ("push", 1), ("pop", 0), ("isEmpty", 0)]
    want2 = [None, None, None, None, True]
    check(SortedStack(), ops2, want2)
    check(SortedStackLazy(), ops2, want2)

    # 边界：空栈 peek 返回 -1
    check(SortedStack(), [("peek", 0)], [-1])
    check(SortedStackLazy(), [("peek", 0)], [-1])

    # 边界：含重复元素
    ops_dup = [
        ("push", 3),
        ("push", 1),
        ("push", 3),
        ("push", 1),
        ("peek", 0),
        ("pop", 0),
        ("peek", 0),
        ("pop", 0),
        ("peek", 0),
        ("pop", 0),
        ("peek", 0),
    ]
    want_dup = [None, None, None, None, 1, None, 1, None, 3, None, 3]
    check(SortedStack(), ops_dup, want_dup)
    check(SortedStackLazy(), ops_dup, want_dup)

    # 边界：连续入栈的逆序数据，每次 peek 都是当前最小值
    ops_desc = [
        ("push", 5),
        ("push", 4),
        ("push", 3),
        ("push", 2),
        ("push", 1),
        ("peek", 0),
        ("pop", 0),
        ("peek", 0),
        ("pop", 0),
        ("peek", 0),
    ]
    want_desc = [None, None, None, None, None, 1, None, 2, None, 3]
    check(SortedStack(), ops_desc, want_desc)
    check(SortedStackLazy(), ops_desc, want_desc)

    # 随机交叉验证
    cross_check(SortedStack())
    cross_check(SortedStackLazy())

    print("all tests passed")
