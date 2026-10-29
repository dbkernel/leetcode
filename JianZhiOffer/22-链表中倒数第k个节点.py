'''
============================================================================
LCR 140 / 剑指 Offer 22. 训练计划 II（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/
分类：链表 · 倒数第 k 个节点（两次遍历 / 快慢双指针）

说明：本题原名「链表中倒数第 k 个节点」，leetcode.cn 现已改写成场景题
「训练计划 II」，函数名为 trainingPlan，参数名改为 cnt。
============================================================================

给定一个头节点为 head 的链表用于记录一系列核心肌群训练项目编号，请查找并
返回倒数第 cnt 个训练项目编号对应的节点。

示例 1：
输入：head = [2,4,7,8], cnt = 1
输出：8

提示：
1 <= head.length <= 100
0 <= head[i] <= 100
1 <= cnt <= head.length
'''

from typing import List, Optional


# 平台预置的链表节点定义；本地运行 python3 时需要补上
class ListNode:
    def __init__(self, val: int = 0, next: Optional["ListNode"] = None):
        self.val = val
        self.next = next


# ============================================================================
# 解法一：顺序查找（先数长度，再走到第 n - cnt 个）【非最优】
#
# 为什么慢：要走两遍。第一遍只为了得到长度 n，第二遍才真正定位；对链表这种
# 只能单向遍历的结构，等于把前 n - cnt 个节点白扫了一次。时间仍是 O(n)，
# 但常数上明显多一倍。
#
# 换个角度：倒数第 cnt 个，就是正数第 n - cnt 个（从 0 开始计数）。只要知道
# n，一条 for 循环走过去即可。
#
# 关键点：
#   · 第 n - cnt 个是 0 基下标，所以 range(n - cnt) 才对；写成 n - cnt + 1
#     会多走一步
#   · cnt 是 1 基的（倒数第 1 个是尾节点）
#   · 题目保证 1 <= cnt <= n，无需处理越界
#
# 以示例 1（head = [2,4,7,8], cnt = 1）为例：
#   第一遍数出 n = 4
#   从 head 走 n - cnt = 3 步 → 停在 8
#
# 复杂度分析：
#   时间 O(n)：两遍遍历
#   空间 O(1)：只用两个指针
# ============================================================================


class Solution1:
    def trainingPlan(self, head: Optional[ListNode], cnt: int) -> Optional[ListNode]:
        node, n = head, 0
        while node is not None:
            node = node.next
            n += 1

        # 倒数第 cnt 个 = 正数第 n - cnt 个（0 基）
        node = head
        for _ in range(n - cnt):
            node = node.next

        return node


# ============================================================================
# 解法二：快慢双指针（只走一遍）【最优解】
#
# 官方两解中只需一次遍历的一个，因此是本题的最优解：时间 O(n)（每个节点只被
# 访问一次），空间 O(1)。
#
# 为什么慢：解法一的第二遍是从头重新走。其实「倒数第 cnt 个」这个位置信息，
# 可以用两指针之间的固定间距表达出来，不必先知道 n。
#
# 换个角度：让 fast 先走 cnt 步，此时 fast 与 slow 之间正好隔着 cnt 个节点。
# 之后两者同步前进，当 fast 越过尾节点变成 None 时，slow 与 fast 相隔 cnt 步，
# 而 fast 已经走完了整条链表 —— slow 停下的位置正是倒数第 cnt 个。
#
# 关键点：
#   · fast 先走 cnt 步，不是 cnt - 1 步；走完之后 fast 指向正数第 cnt + 1 个
#   · 同步循环的条件是 fast is not None（而非 fast.next is not None），
#     因为 fast 要走到「尾节点的下一个」
#   · 第一个循环要带 cnt > 0，避免 cnt 超过链长时把 fast 走穿
#   · cnt == n 时 fast 先走成 None，同步循环一次都不进，slow 就是 head
#
# 以示例 1（head = [2,4,7,8], cnt = 1）为例：
#   fast 走 1 步 → 指向 4（此时与 slow 相隔 1 个节点）
#   两者同步前进：fast 4→7→8→None，slow 2→4→7→8
#   fast 变 None 时 slow 停在 8 —— 倒数第 1 个
#
# 复杂度分析：
#   时间 O(n)：fast 走 n 步，slow 走 n - cnt 步，合计不到 2n
#   空间 O(1)：只用两个指针
# ============================================================================


class Solution2:
    def trainingPlan(self, head: Optional[ListNode], cnt: int) -> Optional[ListNode]:
        fast = slow = head

        # 先让 fast 与 slow 之间拉开 cnt 个节点的间距
        while fast is not None and cnt > 0:
            fast = fast.next
            cnt -= 1

        # fast 越过尾节点时，slow 正好停在倒数第 cnt 个
        while fast is not None:
            fast = fast.next
            slow = slow.next

        return slow


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
    """从某个节点开始展开，用于核对返回的节点及其后续"""
    values: List[int] = []
    while head is not None:
        values.append(head.val)
        head = head.next
    return values


def check(name: str, values: List[int], cnt: int, want: List[int]) -> None:
    """一个用例喂给两种解法：两次遍历与双指针必须返回同一个节点"""
    got1 = to_list(Solution1().trainingPlan(build(values), cnt))
    got2 = to_list(Solution2().trainingPlan(build(values), cnt))

    assert got1 == want, f"{name}: by_length got {got1}, want {want}"
    assert got2 == want, f"{name}: two_pointers got {got2}, want {want}"


def main() -> None:
    # 示例 1：cnt = 1，返回尾节点
    check("example 1", [2, 4, 7, 8], 1, [8])

    # 边界：cnt == n，返回头节点，验证 fast 先走成 None 时 slow 不动
    check("cnt equals n", [2, 4, 7, 8], 4, [2, 4, 7, 8])

    # 边界：单节点链表，cnt 只能是 1
    check("single node", [7], 1, [7])

    # 一般情况：返回中间节点，后续部分要一并正确
    check("middle node", [1, 2, 3, 4, 5], 2, [4, 5])

    # 边界：cnt == n - 1，验证间距算的是 cnt 而不是 cnt - 1
    check("cnt near end", [1, 2, 3], 2, [2, 3])

    print("all tests passed")


if __name__ == "__main__":
    main()
