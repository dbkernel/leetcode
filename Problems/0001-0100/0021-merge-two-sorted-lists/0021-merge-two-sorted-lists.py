'''
============================================================================
LeetCode 21. 合并两个有序链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/merge-two-sorted-lists/
分类：链表 · 双路归并（递归 / 迭代）
============================================================================

将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个
链表的所有节点组成的。

示例 1：
输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]

示例 2：
输入：l1 = [], l2 = []
输出：[]

示例 3：
输入：l1 = [], l2 = [0]
输出：[0]

提示：
两个链表的节点数目范围是 [0, 50]
-100 <= Node.val <= 100
l1 和 l2 均按 非递减顺序 排列
'''

from typing import List, Optional


# 平台预置的链表节点定义；本地运行 python3 时需要补上
class ListNode:
    def __init__(self, val: int = 0, next: Optional["ListNode"] = None):
        self.val = val
        self.next = next


# ============================================================================
# 解法一：递归（每次摘掉较小的那个头节点）【非最优】
#
# 为什么慢：思路本身不慢，时间仍是 O(n + m)；慢在空间 —— 每合并一个节点就
# 压一层调用栈，最坏深度 n + m。Python 默认递归上限只有 1000，链表一长就
# 直接 RecursionError。
#
# 换个角度：合并结果的头，一定是 l1 头与 l2 头里较小的那个。取走它之后，
# 剩下的部分仍然是「两条有序链表合并」—— 问题结构完全没变，只是规模小了一，
# 于是可以直接写成递归定义。
#
# 关键点：
#   · 两个空判断必须写在最前面 —— 任一链表为空时，剩余部分整段接上即可，
#     不能再比较大小
#   · 递归调用前先把自己的 next 指过去，再返回自己 —— 顺序反了会丢掉
#     子问题的结果
#   · 比较用 < 还是 <= 都能得到合法答案；相等时取哪个由实现自定
#
# 以示例 1（l1 = [1,2,4], l2 = [1,3,4]）为例：
#   l1 头 1 不小于 l2 头 1 → 取 l2 的 1，问题变成 merge([1,2,4], [3,4])
#   再比较 1 与 3 → 取 1，问题变成 merge([2,4], [3,4])
#   依此递归到某一侧为空，剩余链表整段接上
#
# 复杂度分析：
#   时间 O(n + m)：每个节点恰好被取出一次
#   空间 O(n + m)：递归深度等于合并后的节点数
# ============================================================================


class Solution1:
    def mergeTwoLists(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        # 任一侧走完，剩下那一段整体有序，直接接上
        if l1 is None:
            return l2
        if l2 is None:
            return l1

        if l1.val <= l2.val:
            # 先接好子问题的结果，再返回自己；顺序反了会丢掉后半段
            l1.next = self.mergeTwoLists(l1.next, l2)
            return l1

        l2.next = self.mergeTwoLists(l1, l2.next)
        return l2


# ============================================================================
# 解法二：迭代（哨兵节点 + 尾指针）【最优解】
#
# 官方两解中空间最优的一个，也是本题的最优解：时间 O(n + m) 已是下界
# （每个节点至少要被接一次），空间压到 O(1)，不占用调用栈。
#
# 为什么慢：解法一的额外开销全在调用栈上。只要把「谁接在谁后面」这件事
# 交给一个循环，栈就不需要了。
#
# 换个角度：维护一个尾指针，每轮挑出两个链表头中较小的那个接到尾部，然后把
# 对应链表往后拨一位。与递归相比，只是把「递归返回时再连」换成了「当场连好」。
#
# 关键点：
#   · 必须用哨兵（哑节点）—— 否则「结果链表的第一个节点」和「后续节点」
#     要写两套赋值逻辑，容易漏掉空结果的分支
#   · 尾指针每轮都要前进 —— 漏掉这一步会反复覆盖同一个 next，链表被截断
#   · 循环结束后把 prev.next 指向剩余非空链表，而不是再逐个搬运 ——
#     两条链表都有序，剩下那段整体都大于已合并的部分
#
# 以示例 1（l1 = [1,2,4], l2 = [1,3,4]）为例：
#   1 与 1 比较，取 l1 的 1；再取 l2 的 1
#   接着 2 与 3 比较取 2，4 与 3 比较取 3，4 与 4 比较取 l1 的 4
#   此时 l1 已空，把 l2 剩下的 4 整段接上 → [1,1,2,3,4,4]
#
# 复杂度分析：
#   时间 O(n + m)：每轮至少推进一条链表，循环次数不超过节点总数
#   空间 O(1)：只用哨兵和尾指针两个额外节点
# ============================================================================


class Solution2:
    def mergeTwoLists(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        # 哨兵节点让「第一个节点」和「后续节点」走同一套赋值逻辑
        prehead = ListNode(-1)
        prev = prehead

        while l1 is not None and l2 is not None:
            # 相等时优先取 l1，只是为了让结果稳定，题目不要求这一点
            if l1.val <= l2.val:
                prev.next = l1
                l1 = l1.next
            else:
                prev.next = l2
                l2 = l2.next
            # 漏掉这句会反复覆盖 prev.next，链表被截断
            prev = prev.next

        # 剩下那段整体有序，直接接上即可，不必逐节点搬运
        prev.next = l1 if l1 is not None else l2

        return prehead.next


# ---- 以下为自测代码，不属于题解主体 ----


def build(values: List[int]) -> Optional[ListNode]:
    """由列表构造链表，方便写断言"""
    dummy = ListNode(0)
    tail = dummy
    for v in values:
        tail.next = ListNode(v)
        tail = tail.next
    return dummy.next


def to_list(head: Optional[ListNode]) -> List[int]:
    """按链表顺序展开，用于和期望序列比较"""
    values: List[int] = []
    while head is not None:
        values.append(head.val)
        head = head.next
    return values


def check(name: str, a: List[int], b: List[int], want: List[int]) -> None:
    """一个用例喂给两种解法：递归与迭代必须得到同一条链表"""
    # 解法会消耗输入，所以每个解法各用一份新建的链表
    got1 = to_list(Solution1().mergeTwoLists(build(a), build(b)))
    got2 = to_list(Solution2().mergeTwoLists(build(a), build(b)))

    assert got1 == want, f"{name}: recursive got {got1}, want {want}"
    assert got2 == want, f"{name}: iterative got {got2}, want {want}"


def main() -> None:
    # 示例 1：两链表等长且互相交错
    check("example 1", [1, 2, 4], [1, 3, 4], [1, 1, 2, 3, 4, 4])

    # 示例 2：两侧都是空链表，结果必须为空而不是崩溃
    check("example 2", [], [], [])

    # 示例 3：只有一侧有节点，走「整段接上」分支
    check("example 3", [], [0], [0])

    # 边界：两侧元素完全相等，验证不会漏节点也不会重复
    check("all equal", [1, 1], [1, 1], [1, 1, 1, 1])

    # 边界：一侧整体小于另一侧，验证剩余段是整段拼接
    check("one side smaller", [1, 2], [3, 4, 5], [1, 2, 3, 4, 5])

    print("all tests passed")


if __name__ == "__main__":
    main()
