'''
============================================================================
LeetCode 83. 删除排序链表中的重复元素（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-list/
分类：链表 · 一次遍历去重
============================================================================

给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。
返回 已排序的链表 。

示例 1：
输入：head = [1,1,2]
输出：[1,2]

示例 2：
输入：head = [1,1,2,3,3]
输出：[1,2,3]

提示：
链表中节点数目在范围 [0, 300] 内
-100 <= Node.val <= 100
题目数据保证链表已经按升序 排列
'''

from typing import List, Optional


# 平台预置的链表节点定义；本地运行 python3 时需要补上
class ListNode:
    def __init__(self, val: int = 0, next: Optional["ListNode"] = None):
        self.val = val
        self.next = next


# ============================================================================
# 解法：一次遍历（当前节点与下一个节点比较）【最优解】
#
# 官方题解只给出这一种方法，它同时也就是本题的最优解：每个节点访问一次，
# 时间 O(n) 已是下界，且完全不需要额外空间。
#
# 为什么慢：想「收集所有不同值再重建链表」就要 O(n) 额外空间；想「发现重复
# 就回头找前驱」又得 O(n^2)。而链表本身已经有序，重复元素必然相邻，这个
# 性质足以让一趟扫描解决问题。
#
# 换个角度：站在当前节点上，只需要看它的下一个节点值是否相同：
#   · 相同 → 把当前节点的 next 跨过下一个节点（丢弃重复的那个），当前节点
#     本身不动，继续和新的 next 比
#   · 不同 → 当前节点前进一步
# 因为链表有序，每个值只会连续出现一段，处理完这段就再也不会遇到它。
#
# 关键点：
#   · 删除时当前节点不动 —— 否则 [1,1,1] 这种连续重复会漏删
#   · 只有值不同才前进 —— 这一步是「不重复时才走」，容易写反
#   · 不需要哨兵 —— 头节点永远会被保留，这与 203 题不同
#   · 循环条件要同时判 cur 和 cur.next —— 拿到 next 才能比较
#
# 以示例 2（head = [1,1,2,3,3]）为例：
#   cur = 1：next 也是 1 → 跨过 → [1,2,3,3]（cur 仍是第 1 个 1）
#   cur = 1：next 是 2 → 不同，cur 前进到 2
#   cur = 2：next 是 3 → 不同，cur 前进到 3
#   cur = 3：next 也是 3 → 跨过 → [1,2,3]
#   cur = 3：next 为空 → 结束
#
# 复杂度分析：
#   时间 O(n)：每个节点最多比较一次、被跨越一次
#   空间 O(1)：只用一条指针
# ============================================================================


class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        cur = head

        # 需要拿到 next 才能比较，所以两个条件都要判
        while cur is not None and cur.next is not None:
            if cur.val == cur.next.val:
                # 跨过重复节点；cur 不动，可能后面还有相同的值
                cur.next = cur.next.next
            else:
                cur = cur.next

        return head


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


def check(name: str, values: List[int], want: List[int]) -> None:
    """一个用例 = 输入 + 期望输出"""
    got = to_list(Solution().deleteDuplicates(build(values)))
    assert got == want, f"{name}: got {got}, want {want}"


def main() -> None:
    # 示例 1：一对重复
    check("example 1", [1, 1, 2], [1, 2])

    # 示例 2：两组重复，验证跨过之后 cur 不动
    check("example 2", [1, 1, 2, 3, 3], [1, 2, 3])

    # 边界：空链表，不能解引用
    check("empty", [], [])

    # 边界：只有一个节点，cur.next 为空，循环一次都不进
    check("single node", [1], [1])

    # 边界：全部相同，连续跨越三次
    check("all equal", [1, 1, 1, 1], [1])

    # 边界：没有重复，指针一路前进，链表不应被改动
    check("all distinct", [1, 2, 3], [1, 2, 3])

    print("all tests passed")


if __name__ == "__main__":
    main()
