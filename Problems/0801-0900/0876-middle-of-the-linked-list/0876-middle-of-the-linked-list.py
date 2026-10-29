'''
============================================================================
LeetCode 876. 链表的中间结点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/middle-of-the-linked-list/
分类：链表 · 中间结点（数组 / 单指针 / 快慢指针）
============================================================================

给你单链表的头结点 head ，请你找出并返回链表的中间结点。

如果有两个中间结点，则返回第二个中间结点。

示例 1：
输入：head = [1,2,3,4,5]
输出：[3,4,5]
解释：链表只有一个中间结点，值为 3 。

示例 2：
输入：head = [1,2,3,4,5,6]
输出：[4,5,6]
解释：该链表有两个中间结点，值分别为 3 和 4 ，返回第二个结点。

提示：
链表的结点数范围是 [1, 100]
1 <= Node.val <= 100
'''

from typing import List, Optional


# 平台预置的链表节点定义；本地运行 python3 时需要补上
class ListNode:
    def __init__(self, val: int = 0, next: Optional["ListNode"] = None):
        self.val = val
        self.next = next


# ============================================================================
# 解法一：数组（把所有节点存下来，取下标 n // 2）【非最优】
#
# 为什么慢：为了「按下标取中间」这个便利，把全部节点塞进列表，
# 额外占用 O(n) 空间。时间仍是 O(n)，但空间没有必要地放大了。
#
# 换个角度：链表不支持随机访问，那就先用列表补上这个能力 —— 一次遍历后，
# 中间结点就是 nodes[n // 2]。这是最直白的「先转成数组」写法。
#
# 关键点：
#   · 下标是 n // 2 而不是 (n - 1) // 2 —— 题目要求两个中间结点时取后一个，
#     整数除法向下取整正好满足
#   · 列表里存的是节点对象，返回 nodes[n // 2] 就是原链表中的那个结点
#
# 以示例 2（head = [1,2,3,4,5,6]）为例：
#   n = 6，n // 2 = 3，nodes[3] 是值为 4 的结点 → [4,5,6]
#
# 复杂度分析：
#   时间 O(n)：遍历一遍收集，再 O(1) 取下标
#   空间 O(n)：列表存放全部节点
# ============================================================================


class Solution1:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        nodes: List[ListNode] = []
        p = head
        while p is not None:
            nodes.append(p)
            p = p.next
        return nodes[len(nodes) // 2]


# ============================================================================
# 解法二：单指针法（先数长度，再走一半）【非最优】
#
# 为什么慢：空间回到了 O(1)，但为了知道「一半」有多远，必须先完整遍历一遍
# 数长度，再从头走一遍，共两趟。题目本身只需要一趟就能定位。
#
# 换个角度：中间结点是正数第 n // 2 个（0 起算）。既然已经知道长度，
# 从 head 出发走 n // 2 步就落在它上面。
#
# 关键点：
#   · 步数是 n // 2，不是 (n - 1) // 2 —— 偶数长度要取后一个中间结点
#   · 计数与前进要分开写，不能让同一个指针身兼两职
#
# 以示例 2（head = [1,2,3,4,5,6]）为例：
#   先数出 n = 6；再从 head 走 6 // 2 = 3 步，落在值为 4 的结点 → [4,5,6]
#
# 复杂度分析：
#   时间 O(n)：数长度一趟，前进一趟
#   空间 O(1)：只用两个指针和一个计数
# ============================================================================


class Solution2:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        n = 0
        p = head
        while p is not None:
            n += 1
            p = p.next

        cur = head
        for _ in range(n // 2):
            cur = cur.next
        return cur


# ============================================================================
# 解法三：快慢指针（快走两步，慢走一步）【最优解】
#
# 官方三解中综合最优的一个，也是本题的最优解：一趟扫描 O(n)，额外空间 O(1)。
# 解法一多花 O(n) 空间，解法二要扫两趟，这一种两者都省。
#
# 为什么慢：前面两种写法都建立在「先知道长度」之上 —— 要么把节点全存下来，
# 要么先数一遍。可「中间」这个概念其实可以由速度差直接表达。
#
# 换个角度：让 fast 每次走两步、slow 每次走一步。fast 走到末尾时，
# slow 走过的路程正好是 fast 的一半，也就停在中间。不需要知道 n。
#
# 关键点：
#   · 循环条件是 fast is not None and fast.next is not None ——
#     少了后半个判断，偶数长度时 fast.next.next 会抛 AttributeError
#   · 返回 slow，不是 fast —— fast 停在末尾（或末尾的下一个空位）
#   · 偶数长度时 slow 恰好落在后一个中间结点，与题目要求一致
#
# 以示例 2（head = [1,2,3,4,5,6]）为例：
#   初始 slow = 1，fast = 1
#   第 1 轮：slow = 2，fast = 3
#   第 2 轮：slow = 3，fast = 5
#   第 3 轮：fast.next 是 6，非空 → slow = 4，fast = None
#   此时 fast 为空，停止，slow 停在 4 → [4,5,6]
#
# 复杂度分析：
#   时间 O(n)：fast 恰好走完全程
#   空间 O(1)：只用两条指针
# ============================================================================


class Solution3:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        slow = head
        fast = head

        # 两个条件都要判：fast.next 存在才敢再走两步
        while fast is not None and fast.next is not None:
            slow = slow.next
            fast = fast.next.next

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
    """从某个结点开始按顺序展开，用于和期望序列比较"""
    values: List[int] = []
    while head is not None:
        values.append(head.val)
        head = head.next
    return values


def check(name: str, values: List[int], want: List[int]) -> None:
    """一个用例喂给三种解法：三种写法必须指向同一个中间结点"""
    # 每种解法只读不写，可以共用同一份链表
    head = build(values)
    for i, solution in enumerate([Solution1(), Solution2(), Solution3()], start=1):
        got = to_list(solution.middleNode(head))
        assert got == want, f"{name}: solution{i} got {got}, want {want}"


def main() -> None:
    # 示例 1：奇数长度，中间结点唯一
    check("example 1", [1, 2, 3, 4, 5], [3, 4, 5])

    # 示例 2：偶数长度，必须返回后一个中间结点
    check("example 2", [1, 2, 3, 4, 5, 6], [4, 5, 6])

    # 边界：只有一个结点，slow 就是 head，循环一次都不进
    check("single node", [1], [1])

    # 边界：两个结点，返回第二个结点，专门验证 n // 2 的取整方向
    check("two nodes", [1, 2], [2])

    print("all tests passed")


if __name__ == "__main__":
    main()
