"""
============================================================================
面试题 02.01. 移除重复节点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicate-node-lcci/
分类：哈希表 · 链表去重

编写代码，移除未排序链表中的重复节点。保留最开始出现的节点。

示例1：
 输入：[1, 2, 3, 3, 2, 1]
 输出：[1, 2, 3]

示例2：
 输入：[1, 1, 1, 1, 2]
 输出：[1, 2]

提示：
- 链表长度在[0, 20000]范围内。
- 链表元素在[0, 20000]范围内。

进阶：
如果不得使用临时缓冲区，该怎么解决？
"""
from typing import List, Optional


# 力扣预置的节点定义
class ListNode:
    def __init__(self, val: int = 0, next: "Optional[ListNode]" = None):
        self.val = val
        self.next = next


"""
解法一：哈希集合记录出现过的值（一次遍历）【最优解】

为什么慢：最直接的做法是对每个节点再扫一遍后面的链表，删掉值相同的
节点，时间 O(n^2)。链表未排序，无法像数组那样先排序再相邻去重；
而重复判断只关心"这个值以前出现过没有"，不需要保留先后关系之外的信息。

换个角度：遍历时用一个哈希集合记录已经保留的值。每个节点只问一次
"见过吗"：见过就删，没见过就留下并登记。一次遍历即可完成。

关键点：
  · 集合里存值而不是节点地址：链表节点按地址比较，值相同的节点地址不同
  · 枚举前驱节点 pos，待删节点是 pos.next，删除时只需改 pos.next
  · 只有保留节点时才把 pos 前移；删除节点时 pos 原地不动
  · 头节点一定保留，先把它登记进集合，循环从 head.next 开始
  · Python 有垃圾回收，被删节点不需要手动释放

以示例 1（[1,2,3,3,2,1]）为例：
  保留 1，seen={1}；保留 2，seen={1,2}；保留 3，seen={1,2,3}
  下一个 3 已见过，删；下一个 2 已见过，删；下一个 1 已见过，删
  得到 [1,2,3] ✔

复杂度分析：
  时间 O(n)：每个节点最多访问一次
  空间 O(n)：最坏情况下所有值都不同，集合要存 n 个值

解法二：两重循环（时间换空间，对应进阶）【非最优】

不借助缓冲区时，只能让每个保留节点去"清理"它后面所有同值节点：
外层枚举保留节点，内层从该节点开始枚举待删节点的前驱，遇到同值就删。
  · 空间降到 O(1)，但时间升到 O(n^2)
  · 保留的仍是"最开始出现"的节点，因为外层从左往右处理
  · 官方提示 Python 用此解法会超时，这里只作为空间受限时的对照

复杂度分析：
  时间 O(n^2)：最坏（全不相同）每个外层节点都要扫完剩余链表
  空间 O(1)：只用了两个遍历指针
"""


class Solution1:
    def removeDuplicateNodes(
        self, head: Optional[ListNode]
    ) -> Optional[ListNode]:
        if head is None:
            return None

        seen = {head.val}  # 头节点一定保留
        pos = head
        while pos.next is not None:
            # 只在保留时前移 pos；删除时 pos 停在原处继续检查新后继
            if pos.next.val not in seen:
                seen.add(pos.next.val)
                pos = pos.next
            else:
                pos.next = pos.next.next
        return head


class Solution2:
    def removeDuplicateNodes(
        self, head: Optional[ListNode]
    ) -> Optional[ListNode]:
        outer = head
        while outer is not None:
            # inner 是待删节点的前驱，删除时不必回退
            inner = outer
            while inner.next is not None:
                if inner.next.val == outer.val:
                    inner.next = inner.next.next
                else:
                    inner = inner.next
            outer = outer.next
        return head


def build_list(vals: List[int]) -> Optional[ListNode]:
    dummy = ListNode()
    tail = dummy
    for v in vals:
        tail.next = ListNode(v)
        tail = tail.next
    return dummy.next


def to_list(head: Optional[ListNode]) -> List[int]:
    vals = []
    while head is not None:
        vals.append(head.val)
        head = head.next
    return vals


def check(input_vals: List[int], want: List[int]) -> None:
    got1 = to_list(Solution1().removeDuplicateNodes(build_list(input_vals)))
    assert got1 == want, f"hash({input_vals}) = {got1}, want {want}"

    got2 = to_list(Solution2().removeDuplicateNodes(build_list(input_vals)))
    assert got2 == want, f"brute({input_vals}) = {got2}, want {want}"


if __name__ == "__main__":
    # 示例 1
    check([1, 2, 3, 3, 2, 1], [1, 2, 3])

    # 示例 2
    check([1, 1, 1, 1, 2], [1, 2])

    # 边界：空链表
    check([], [])

    # 边界：单节点 / 两个相同节点
    check([1], [1])
    check([1, 1], [1])

    # 边界：重复值分散出现，保留第一次出现的顺序
    check([1, 2, 1, 3, 2, 4], [1, 2, 3, 4])
    check([1, 3, 2, 3, 1, 2, 4], [1, 3, 2, 4])

    # 边界：全部相同
    check([7, 7, 7, 7], [7])

    # 边界：本来就没有重复
    check([1, 2, 3, 4], [1, 2, 3, 4])

    # 规模：200 个节点，值循环出现 0..9
    check([i % 10 for i in range(200)], list(range(10)))

    print("all tests passed")
