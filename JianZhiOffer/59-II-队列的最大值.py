"""
============================================================================
LCR 184 / 剑指 Offer 59 - II. 设计自助结算系统（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/dui-lie-de-zui-da-zhi-lcof/
分类：队列 · 设计（单调双端队列）

请设计一个自助结账系统，该系统需要通过一个队列来模拟顾客通过购物车的结算
过程，需要实现的功能有：

- get_max()：获取结算商品中的最高价格，如果队列为空，则返回 -1
- add(value)：将价格为 value 的商品加入待结算商品队列的尾部
- remove()：移除第一个待结算的商品价格，如果队列为空，则返回 -1

注意，为保证该系统运转高效性，以上函数的均摊时间复杂度均为 O(1)

示例 1：
输入: 
["Checkout","add","add","get_max","remove","get_max"]
[[],[4],[7],[],[],[]]
输出: [null,null,null,7,4,7]

示例 2：
输入: 
["Checkout","remove","get_max"]
[[],[],[]]
输出: [null,-1,-1]

提示：
- 1 <= get_max, add, remove 的总操作数 <= 10000
- 1 <= value <= 10^5

说明：本题在 leetcode.cn 上已改号为 LCR 184，题名由「队列的最大值」改为
「设计自助结算系统」，平台方法名也从 max_value / push_back / pop_front
改为 get_max / add / remove，下面按平台给出的签名实现。
"""
from collections import deque
from typing import List


"""
解法一：普通队列 + 线性扫描求最大值【非最优】

为什么慢：队列只允许两头操作，没有任何"局部信息"能直接给出最大值，于是
get_max 每次都得从头扫到尾，单次 O(n)。m 次操作里有大量 get_max 时退化
成 O(m·n)。

换个角度：先把 FIFO 语义做对 —— 用一个只前进的 head 下标记录队首，避免
remove 时整体搬移元素；最大值则老老实实扫描。这是正确但慢的基线，后面
的单调队列正是在它的基础上把"扫描"换成了"增量维护"。

关键点：
  · remove 返回被移除的那个价格，而不是 None；空队列返回 -1
  · get_max 在空队列上也返回 -1，与"最大值恰好是 -1"不会混淆（价格 ≥ 1）
  · 用 head 下标而不是 list.pop(0)：后者每次 O(n)，remove 就不再是 O(1)
  · head 只前进，data 只追加，所以扫描区间是 data[head:]

以示例 1（add 4、add 7、get_max、remove、get_max）为例：
  add 4：data = [4]
  add 7：data = [4, 7]
  get_max：扫描得 7
  remove：返回 4，head 移到 1
  get_max：扫描 data[1:] 得 7 ✔

复杂度分析：
  时间：add O(1)、remove O(1)、get_max O(n)（n 为当前队列长度）
  空间 O(n)：保存所有尚未出队的元素；head 只前进，最坏存下全部历史元素

解法二：单调双端队列，与普通队列同步进出【最优解】

为什么慢：解法一每次都重扫，但两次 get_max 之间队列只可能"尾部加一个"或
"头部删一个"，真正影响最大值的信息变化极小。重扫等于把没变的部分又算一遍。

换个角度：若 i 排在 j 前面且 value[i] <= value[j]，那么 j 一定比 i 晚出队，
i 在 j 存在期间永远当不了最大值，可以直接丢弃。反复丢弃后，候选集合里剩
下的值必然"从队首到队尾严格递减"，队首就是当前最大值。

关键点：
  · add 时从队尾弹掉所有**严格小于**新值的元素：相等必须保留 —— 重复的
    最大值要一个一个等着被 remove 走，弹掉就少了一份
  · remove 时先按 FIFO 弹出队首元素，只有它恰好等于单调队列的队首，才同步
    弹出单调队列的队首；不相等说明它早被"更大的后来者"压掉了
  · 单调队列只存值就够了，不需要下标：remove 的顺序与值无关，靠"相等"就能
    对齐两个队列
  · 两个队列都空时 remove / get_max 返回 -1；只要普通队列非空，单调队列必非空
  · Python 用 collections.deque，popleft / pop 都是 O(1)

以示例 1（add 4、add 7、get_max、remove、get_max）为例：
  add 4：普通队列 [4]，单调队列 [4]
  add 7：7 比队尾 4 大，先弹掉 4；普通队列 [4,7]，单调队列 [7]
  get_max：队首 7
  remove：普通队列弹出 4，与单调队列队首 7 不等，单调队列不动 → 返回 4
  get_max：仍是 7 ✔（这正是示例输出 4、7 的来源）

复杂度分析：
  时间：add / remove / get_max 均摊 O(1)：每个值最多进、出单调队列各一次
  空间 O(n)：普通队列存所有待结算价格，单调队列最坏也存 n 个（全相等时）
"""


class CheckoutBrute:
    def __init__(self):
        self.data: List[int] = []
        self.head = 0  # 队首位置，只前进

    def get_max(self) -> int:
        best = -1
        for i in range(self.head, len(self.data)):
            if self.data[i] > best:
                best = self.data[i]
        return best

    def add(self, value: int) -> None:
        self.data.append(value)

    def remove(self) -> int:
        if self.head == len(self.data):
            return -1
        value = self.data[self.head]
        self.head += 1
        return value


class Checkout:
    def __init__(self):
        # q 保证 remove 的先后顺序，d 的队首恒为当前最大值
        self.q = deque()
        self.d = deque()

    def get_max(self) -> int:
        return self.d[0] if self.d else -1

    def add(self, value: int) -> None:
        # 严格小于才弹：相等要保留，否则重复的最大值会少一份
        while self.d and self.d[-1] < value:
            self.d.pop()
        self.d.append(value)
        self.q.append(value)

    def remove(self) -> int:
        if not self.q:
            return -1
        value = self.q.popleft()
        # 只有真正等于最大值时才同步弹出单调队列的队首
        if value == self.d[0]:
            self.d.popleft()
        return value


# 按题目给的调用序列回放；void 方法在结果里留 None
def run(ops: List[tuple], use_brute: bool = False) -> list:
    shelf = CheckoutBrute() if use_brute else Checkout()
    out = []
    for name, value in ops:
        if name == "add":
            shelf.add(value)
            out.append(None)
        elif name == "get_max":
            out.append(shelf.get_max())
        else:
            out.append(shelf.remove())
    return out


def check(ops: List[tuple], want: list) -> None:
    for use_brute in (False, True):
        got = run(ops, use_brute)
        assert got == want, f"use_brute={use_brute}: got {got}, want {want}"


# 随机交叉验证：用 deque 当队列，最大值直接扫描，作为独立基准
def cross_check() -> None:
    shelf = Checkout()
    brute = CheckoutBrute()
    reference = deque()
    seed = 20240607
    for step in range(3000):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        r = (seed >> 16) % 10
        if r < 5:
            seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
            # 值域 1..5，专门制造大量重复值
            value = (seed >> 16) % 5 + 1
            shelf.add(value)
            brute.add(value)
            reference.append(value)
        elif r < 8:
            want = reference.popleft() if reference else -1
            assert shelf.remove() == want
            assert brute.remove() == want
        else:
            want = max(reference) if reference else -1
            assert shelf.get_max() == want
            assert brute.get_max() == want


if __name__ == "__main__":
    # 示例 1：remove 返回被移除的价格 4，之后最大值仍是 7
    check([("add", 4), ("add", 7), ("get_max", 0), ("remove", 0),
           ("get_max", 0)],
          [None, None, 7, 4, 7])

    # 示例 2：空队列上的 remove 与 get_max 都返回 -1
    check([("remove", 0), ("get_max", 0)], [-1, -1])

    # 边界：取空后继续操作，状态不能残留
    check([("add", 5), ("remove", 0), ("get_max", 0), ("remove", 0)],
          [None, 5, -1, -1])

    # 反直觉：最大值被取走后要回落到次大值，而不是继续报 5
    check([("add", 5), ("add", 3), ("add", 4), ("get_max", 0),
           ("remove", 0), ("get_max", 0), ("remove", 0), ("get_max", 0),
           ("remove", 0), ("get_max", 0)],
          [None, None, None, 5, 5, 4, 3, 4, 4, -1])

    # 反直觉：两个相同的最大值必须分别等着被 remove，弹队尾用 < 而不是 <=
    check([("add", 5), ("add", 5), ("get_max", 0), ("remove", 0),
           ("get_max", 0), ("remove", 0), ("get_max", 0)],
          [None, None, 5, 5, 5, 5, -1])

    # 反直觉：新来的更大值要立刻接管最大值，且旧最大值已不在单调队列里
    check([("add", 3), ("get_max", 0), ("add", 9), ("get_max", 0),
           ("remove", 0), ("get_max", 0), ("remove", 0), ("get_max", 0)],
          [None, 3, None, 9, 3, 9, 9, -1])

    # 边界：单个元素反复 get_max，不应改变状态
    check([("add", 1), ("get_max", 0), ("get_max", 0), ("remove", 0),
           ("get_max", 0)],
          [None, 1, 1, 1, -1])

    # 随机交叉验证
    cross_check()

    print("all tests passed")
