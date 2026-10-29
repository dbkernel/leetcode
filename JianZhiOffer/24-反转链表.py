"""
============================================================================
LCR 141 / 剑指 Offer 24. 训练计划 III（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/fan-zhuan-lian-biao-lcof/
分类：链表 · 三指针迭代 / 递归

给定一个头节点为 head 的单链表用于记录一系列核心肌群训练编号，请将该
系列训练编号 倒序 记录于链表并返回。

示例 1：
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]

示例 2：
输入：head = [1,2]
输出：[2,1]

示例 3：
输入：head = []
输出：[]

提示：
- 链表中节点的数目范围是 [0, 5000]
- -5000 <= Node.val <= 5000

注意：本题与主站 206 题相同：
https://leetcode.cn/problems/reverse-linked-list/

说明：本题在 leetcode.cn 已改号为 LCR 141，题名由「反转链表」改为
「训练计划 III」，函数签名由 `reverseList(ListNode*)` 改为
`trainningPlan(ListNode*)`（官方 slug 与函数名均保留 trainning 拼写）。
"""
import sys
from typing import List, Optional


# 力扣预置的节点定义
class ListNode:
    def __init__(self, val: int = 0, next: "Optional[ListNode]" = None):
        self.val = val
        self.next = next


"""
解法一：三指针原地迭代【最优解】

为什么慢：如果新建一条链表做头插，时间仍是 O(n)，但每个节点都要复制
一份，额外空间 O(n)。题目只要求把原来的 next 指针反过来，不需要动节点值。

换个角度：遍历时把当前节点的 next 指向前一个节点。为了断链后还能继续，
需要三个指针：prev（已反转部分的头）、curr（当前节点）、next（尚未处理
部分的头）。

关键点：
  · 必须先保存 curr.next，再改 curr.next；顺序反了，后半条链表就丢了
  · 循环结束时 curr 为 None，prev 才是新链表的头，不能返回 head
  · 空链表或单节点时循环不执行/只执行一次，返回 prev 正好是原头

以 1 -> 2 -> 3 为例：
  初始 prev = None，curr = 1
  存 next = 2，1.next = None，prev = 1，curr = 2
  存 next = 3，2.next = 1，prev = 2，curr = 3
  存 next = None，3.next = 2，prev = 3，curr = None
  返回 prev = 3，链表 3 -> 2 -> 1  ✔

复杂度分析：
  时间 O(n)：每个节点改一次 next 指针
  空间 O(1)：只用了三个指针，没有新建节点
"""


class Solution1:
    def trainningPlan(self, head: Optional[ListNode]) -> Optional[ListNode]:
        prev = None
        curr = head

        while curr is not None:
            # 先记住剩余部分，改完 next 后就找不到它了
            next_node = curr.next
            curr.next = prev
            prev = curr
            curr = next_node

        # curr 为 None 时，prev 指向原链表的尾，也就是新头
        return prev


"""
解法二：递归反转【非最优】

为什么慢：递归写法代码短，但每层调用都要保存当前节点和返回地址，
空间是 O(n) 的调用栈；链表很长时可能爆栈，做不到迭代版本的 O(1)。

换个角度：假设 head.next 之后的链表已经反转好，返回的新头是 new_head。
此时 head.next 是反转后链表的尾节点，让它指回 head，再把 head.next
置空，整条链表就接上了。

关键点：
  · 递归基是空链表或单节点，直接返回 head
  · `head.next.next = head` 必须在 `head.next = None` 之前；
    顺序反了，尾节点就找不到了
  · 必须把 head.next 置空，否则原头和新尾会形成环
  · 递归深度等于链表长度，Python 默认 1000 层不够，自测里先调高

以 1 -> 2 -> 3 为例：
  递归到 3，返回 3
  回到 2：2.next.next = 2，即 3->2，2.next = None，返回 3
  回到 1：1.next.next = 1，即 2->1，1.next = None，返回 3
  链表 3 -> 2 -> 1  ✔

复杂度分析：
  时间 O(n)：每个节点做常数次指针操作
  空间 O(n)：递归栈深度等于链表长度
"""


class Solution2:
    def trainningPlan(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head

        new_head = self.trainningPlan(head.next)
        head.next.next = head
        head.next = None
        return new_head


# 按数组顺序建链表，返回头节点；空数组返回 None
def build_list(values: List[int]) -> Optional[ListNode]:
    dummy = ListNode()
    tail = dummy
    for value in values:
        tail.next = ListNode(value)
        tail = tail.next
    return dummy.next


def to_list(head: Optional[ListNode]) -> List[int]:
    values = []
    node = head
    while node is not None:
        values.append(node.val)
        node = node.next
    return values


# 两个解法各自建一条新链表，反转后按顺序收集再比较
def check(values: List[int], want: List[int]) -> None:
    assert to_list(Solution1().trainningPlan(build_list(values))) == want
    assert to_list(Solution2().trainningPlan(build_list(values))) == want


if __name__ == "__main__":
    # 递归解法按题目上限 5000 层，默认限制不够
    sys.setrecursionlimit(100000)

    # 示例 1
    check([1, 2, 3, 4, 5], [5, 4, 3, 2, 1])

    # 示例 2
    check([1, 2], [2, 1])

    # 示例 3：空链表
    check([], [])

    # 边界：单节点，反转后还是自己
    check([7], [7])

    # 边界：负数和零
    check([-5000, 0, 5000], [5000, 0, -5000])

    # 反直觉：两个节点时，递归版若忘记把 head.next 置空会成环
    check([1, 2], [2, 1])

    # 边界：最长 5000 个节点，验证递归深度
    values = list(range(1, 5001))
    check(values, list(reversed(values)))

    print("all tests passed")
