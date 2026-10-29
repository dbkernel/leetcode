'''
============================================================================
LeetCode 19. 删除链表的倒数第 N 个结点（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
分类：链表 · 倒数定位（长度 / 栈 / 快慢指针）
============================================================================

给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。

示例 1：
输入：head = [1,2,3,4,5], n = 2
输出：[1,2,3,5]

示例 2：
输入：head = [1], n = 1
输出：[]

示例 3：
输入：head = [1,2], n = 1
输出：[1]

提示：
链表中结点的数目为 sz
1 <= sz <= 30
0 <= Node.val <= 100
1 <= n <= sz

进阶：你能尝试使用一趟扫描实现吗？
'''

from typing import List, Optional


# 平台预置的链表节点定义；本地运行 python3 时需要补上
class ListNode:
    def __init__(self, val: int = 0, next: Optional["ListNode"] = None):
        self.val = val
        self.next = next


# ============================================================================
# 解法一：计算链表长度（两趟扫描）【非最优】
#
# 为什么慢：倒数第 n 个，换成正数就是第 L - n + 1 个；而 L 事先不知道，
# 必须先把链表走一遍数长度，再走第二遍删节点。两趟扫描本身仍是 O(L)，但
# 题目在进阶里明确问「能否一趟扫描」。
#
# 换个角度：要删的节点必须知道它的前驱。长度为 L 时，前驱是正数第
# L - n 个。加上哨兵后，从头数 L - n 步就停在前驱上，把 next 跨过去即可。
#
# 关键点：
#   · 必须用哨兵 —— 否则删的正好是头节点（L == n）时，没有前驱可改
#   · 步数是 L - n，不是 L - n + 1 —— 从哨兵出发，走 L - n 步落点就是前驱
#   · 删除只改指针；Python 垃圾回收会自动处理被摘下的节点
#
# 以示例 1（head = [1,2,3,4,5], n = 2）为例：
#   L = 5，需要删除正数第 4 个节点（值 4），其前驱是第 3 个（值 3）
#   从哨兵走 L - n = 3 步，停在 3 上
#   把 3 的 next 指向 5 → [1,2,3,5]
#
# 复杂度分析：
#   时间 O(L)：数长度一趟，定位一趟
#   空间 O(1)：只用哨兵和两个整数
# ============================================================================


class Solution1:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        # 哨兵让「删除头节点」和「删除中间节点」走同一套逻辑
        dummy = ListNode(0, head)

        length = 0
        p = head
        while p is not None:
            length += 1
            p = p.next

        # 停在待删节点的前驱上：从哨兵走 length - n 步
        prev = dummy
        for _ in range(length - n):
            prev = prev.next

        # 跨越被删节点
        prev.next = prev.next.next

        return dummy.next


# ============================================================================
# 解法二：栈（边走边压，弹出 n 个后栈顶即前驱）【非最优】
#
# 为什么慢：为了「从后往前数 n 个」这种反向需求，把整条链的节点地址都压进
# 栈里，额外占用 O(L) 空间。时间和一趟扫描一样是 O(L)，但空间不如快慢指针。
#
# 换个角度：栈天然就是「倒序访问」。把哨兵和所有节点依次压栈后，栈顶是
# 最后一个节点；弹出 n 个，栈顶就正好是待删节点的前驱。
#
# 关键点：
#   · 哨兵也要入栈 —— 否则删除头节点时链路会断（栈里没有前驱可退）
#   · 弹出 n 个之后不要再弹，此时的栈顶就是前驱
#   · 删除后要返回哨兵的 next，而不是原来的 head —— head 可能已经被删掉
#
# 以示例 1（head = [1,2,3,4,5], n = 2）为例：
#   栈内自底向上为：哨兵、1、2、3、4、5
#   弹出 2 个（5、4），栈顶是 3
#   3 的 next 指向 3.next.next，即 5 → [1,2,3,5]
#
# 复杂度分析：
#   时间 O(L)：每个节点入栈一次、出栈 n 次
#   空间 O(L)：栈里存放全部节点地址
# ============================================================================


class Solution2:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        nodes: List[ListNode] = []

        # 哨兵一同入栈，删除头节点时才有前驱可用
        p = dummy
        while p is not None:
            nodes.append(p)
            p = p.next

        # 弹出 n 个节点后，栈顶就是待删节点的前驱
        for _ in range(n):
            nodes.pop()

        prev = nodes[-1]
        prev.next = prev.next.next

        return dummy.next


# ============================================================================
# 解法三：双指针（快指针先走 n 步，一趟扫描）【最优解】
#
# 官方三解中时间空间都最优的一个，也是本题的最优解：一趟扫描 O(L)，
# 额外空间 O(1)，正好回答题目进阶里的问题。
#
# 为什么慢：解法一要两趟，解法二要多占 O(L) 空间。二者的问题都是「必须先把
# 整条链看一遍才知道前驱在哪」。
#
# 换个角度：让 fast 先走 n 步，之后 fast 和 slow 同步前进。当 fast 走到
# 末尾时，slow 与 fast 相隔 n 个节点，正好停在待删节点的前驱上。
# 「倒数第 n 个」这个距离关系，可以直接用两条指针的脚步差来表达。
#
# 关键点：
#   · fast 必须先走 n 步再开始同步 —— 顺序反了差值就变成 0
#   · 走 n 步的起点是哨兵而不是 head —— 这样 n == L 时 fast 恰好走到尾节点，
#     slow 停在哨兵上，删除头节点不需要额外分支
#   · 循环条件是 fast.next is not None，不是 fast is not None —— 后者会让
#     slow 多走一格，落到待删节点本身而不是它的前驱上
#
# 以示例 1（head = [1,2,3,4,5], n = 2）为例：
#   fast 从哨兵走 2 步，落在节点 2 上
#   fast.next 非空就同步走：fast 到 3 时 slow 到哨兵；fast 到 4 时 slow 到 1；
#   fast 到 5 时 slow 到 2；fast.next 为空时停止，slow 停在 3
#   3 是待删节点 4 的前驱 → [1,2,3,5]
#
# 复杂度分析：
#   时间 O(L)：fast 恰好走完一遍链表
#   空间 O(1)：只用哨兵和两条指针
# ============================================================================


class Solution3:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)

        # fast 先走 n 步：从哨兵出发，保证 n == 链表长度时 slow 落在哨兵上
        fast = dummy
        slow = dummy
        for _ in range(n):
            fast = fast.next

        # 同步前进，fast 走到尾节点时 slow 恰好是待删节点的前驱
        while fast.next is not None:
            fast = fast.next
            slow = slow.next

        slow.next = slow.next.next

        return dummy.next


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


def check(name: str, values: List[int], n: int, want: List[int]) -> None:
    """一个用例喂给三种解法：三种写法必须得到同一条链表"""
    # 每种解法都会改链表结构，各用一份新建的节点
    solutions = [Solution1(), Solution2(), Solution3()]
    for i, solution in enumerate(solutions, start=1):
        got = to_list(solution.removeNthFromEnd(build(values), n))
        assert got == want, f"{name}: solution{i} got {got}, want {want}"


def main() -> None:
    # 示例 1：删除中间节点
    check("example 1", [1, 2, 3, 4, 5], 2, [1, 2, 3, 5])

    # 示例 2：只剩一个节点且要删它，结果必须为空
    check("example 2", [1], 1, [])

    # 示例 3：删除尾节点
    check("example 3", [1, 2], 1, [1])

    # 边界：删除头节点（n == 链表长度），专防「没有前驱」的写法
    check("remove head", [1, 2, 3], 3, [2, 3])

    # 边界：n == 1 且链表较长，验证快慢指针不会删错尾部
    check("remove tail", [1, 2, 3, 4], 1, [1, 2, 3])

    print("all tests passed")


if __name__ == "__main__":
    main()
