"""
============================================================================
148. 排序链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-list/
分类：链表 · 归并排序 / 堆排序

给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。

示例 1：
输入：head = [4,2,1,3]
输出：[1,2,3,4]

示例 2：
输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]

示例 3：
输入：head = []
输出：[]

提示：
- 链表中节点的数目在范围 [0, 5 * 10^4] 内
- -10^5 <= Node.val <= 10^5

进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序
吗？
"""
from typing import Optional


# 力扣预置的节点定义
class ListNode:
    def __init__(self, val: int = 0, next: "Optional[ListNode]" = None):
        self.val = val
        self.next = next


"""
解法一：自顶向下归并排序（官方方法一）【非最优】

为什么慢：插入排序在链表上定位插入点要 O(n)，总共 O(n^2)。要达到
O(n log n)，适合链表的是归并排序：合并只改指针，不需要随机访问。

换个角度：快慢指针找中点，把链表切成两半，递归排好左右两半，再合并两个
有序链表。递归深度是 O(log n)，这部分栈空间就是它比解法二多出来的开销。

关键点：
  · fast 从 head.next 起步，偶数长度时 slow 落在左半段末尾，两半长度差
    不超过 1
  · 切分时必须令 slow.next = None，否则递归不终止
  · 递归终止条件是空表或单节点
  · 合并用 dummy 承接，比较用 <= 保持稳定

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归调用栈；节点本身复用，没有新建链表

解法二：自底向上归并排序（官方方法二）【最优解】

为什么慢：解法一时间已经最优，但递归栈还要 O(log n)。进阶要求常数级空间，
就差在这层栈上。

换个角度：把「递归切半」换成「按固定步长切段」。sub_length 从 1 开始，每次
把链表切成若干长度不超过 sub_length 的段，两两合并，再让 sub_length 翻倍，
直到超过链表长度。整个过程只用了几个游标指针。

关键点：
  · 先把链表长度数出来，否则不知道 sub_length 何时停止
  · 每轮用 prev 记住「已合并部分的尾节点」，合并完接回去
  · 切出 head2 后必须把 head1 段尾的 next 置空，否则 merge 会越过段边界
  · 右半段可能不足 sub_length，甚至为空；所有推进都要判空
  · 与解法一同为 O(n log n)，但辅助空间从 O(log n) 降到 O(1)，因此最优

复杂度分析：
  时间 O(n log n)：共 O(log n) 轮，每轮合并 O(n)
  空间 O(1)：只用游标指针，节点原地重连

解法三：堆排序【非最优】

为什么慢：堆排序时间 O(n log n)，与两种归并同阶，本身不慢；慢在空间。
  · 数组上的堆排序可以做到 O(1) 辅助空间，因为下标能直接定位父子节点
  · 链表没有随机访问，必须先遍历一次把节点值收集到列表，才能建堆和下沉；
    这一步就是 O(n) 额外空间
  · 排完后还要把值写回原节点，节点身份不变，只有 val 被改写

换个角度：把问题降维成「对一个长为 n 的列表做堆排序」，再写回链表。

关键点：
  · 建堆从最后一个非叶节点 n//2 - 1 开始向前下沉，才是 O(n) 建堆
  · 每轮把堆顶换到末尾，堆大小减一后再从根下沉
  · 下沉时先比较左右孩子取较大者，再和父节点比较；相等不交换
  · 与解法二同为 O(n log n) 时间，但多出 O(n) 列表，故不是最优解

复杂度分析：
  时间 O(n log n)：建堆 O(n)，n 次取堆顶并下沉共 O(n log n)
  空间 O(n)：values 列表保存全部节点值；链表本身没有额外分配
"""


class Solution1:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head

        # fast 从 head.next 起步，偶数长度时 slow 落在左半段末尾
        slow, fast = head, head.next
        while fast is not None and fast.next is not None:
            slow = slow.next
            fast = fast.next.next

        mid = slow.next
        slow.next = None

        return self._merge(self.sortList(head), self.sortList(mid))

    # 合并两个有序链表，返回新表头；节点全部复用
    def _merge(self, a: Optional[ListNode],
               b: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0)
        tail = dummy
        while a is not None and b is not None:
            if a.val <= b.val:
                tail.next = a
                a = a.next
            else:
                tail.next = b
                b = b.next
            tail = tail.next
        tail.next = a if a is not None else b
        return dummy.next


class Solution2:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None

        length = 0
        p = head
        while p is not None:
            length += 1
            p = p.next

        dummy = ListNode(0)
        dummy.next = head
        sub_length = 1
        while sub_length < length:
            prev, curr = dummy, dummy.next
            while curr is not None:
                # 切出长度不超过 sub_length 的左半段
                head1 = curr
                for _ in range(1, sub_length):
                    if curr.next is None:
                        break
                    curr = curr.next

                head2 = curr.next
                curr.next = None
                curr = head2

                # 再切出长度不超过 sub_length 的右半段
                for _ in range(1, sub_length):
                    if curr is None or curr.next is None:
                        break
                    curr = curr.next

                nxt = None
                if curr is not None:
                    nxt = curr.next
                    curr.next = None

                prev.next = self._merge(head1, head2)
                while prev.next is not None:
                    prev = prev.next
                curr = nxt
            sub_length <<= 1

        return dummy.next

    # 合并两个有序链表，返回新表头；节点全部复用
    def _merge(self, a: Optional[ListNode],
               b: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0)
        tail = dummy
        while a is not None and b is not None:
            if a.val <= b.val:
                tail.next = a
                a = a.next
            else:
                tail.next = b
                b = b.next
            tail = tail.next
        tail.next = a if a is not None else b
        return dummy.next


class Solution3:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        values = []
        p = head
        while p is not None:
            values.append(p.val)
            p = p.next

        n = len(values)
        # 从最后一个非叶节点开始下沉，建大根堆是 O(n)
        for i in range(n // 2 - 1, -1, -1):
            self._sift_down(values, i, n)

        # 每轮把堆顶换到末尾，缩小堆后重新下沉
        for end in range(n - 1, 0, -1):
            values[0], values[end] = values[end], values[0]
            self._sift_down(values, 0, end)

        # 节点顺序不变，只把排好的值写回
        p = head
        for v in values:
            p.val = v
            p = p.next
        return head

    @staticmethod
    def _sift_down(a: list, i: int, n: int) -> None:
        while True:
            left = 2 * i + 1
            right = left + 1
            largest = i
            if left < n and a[left] > a[largest]:
                largest = left
            if right < n and a[right] > a[largest]:
                largest = right
            if largest == i:
                break
            a[i], a[largest] = a[largest], a[i]
            i = largest


# ===== 测试辅助：列表建表、链表转列表 =====
def build_list(values: list) -> Optional[ListNode]:
    dummy = ListNode(0)
    tail = dummy
    for v in values:
        tail.next = ListNode(v)
        tail = tail.next
    return dummy.next


def to_list(head: Optional[ListNode]) -> list:
    values = []
    while head is not None:
        values.append(head.val)
        head = head.next
    return values


def check(input_values: list, want: list) -> None:
    assert to_list(Solution1().sortList(build_list(input_values))) == want
    assert to_list(Solution2().sortList(build_list(input_values))) == want
    assert to_list(Solution3().sortList(build_list(input_values))) == want


if __name__ == "__main__":
    # 示例 1
    check([4, 2, 1, 3], [1, 2, 3, 4])

    # 示例 2
    check([-1, 5, 3, 4, 0], [-1, 0, 3, 4, 5])

    # 示例 3：空链表
    check([], [])

    # 边界：单节点
    check([7], [7])

    # 边界：已经有序
    check([1, 2, 3, 4, 5], [1, 2, 3, 4, 5])

    # 边界：完全逆序
    check([5, 4, 3, 2, 1], [1, 2, 3, 4, 5])

    # 边界：全部相同，验证合并不会丢掉重复节点
    check([3, 3, 3, 3], [3, 3, 3, 3])

    # 边界：重复值夹杂在乱序里
    check([2, 1, 2, 1, 2], [1, 1, 2, 2, 2])

    # 边界：两个节点，验证自底向上时右半段可能为空
    check([2, 1], [1, 2])

    # 边界：负数与正数混合
    check([0, -100000, 100000, -1, 1], [-100000, -1, 0, 1, 100000])

    # 边界：奇数长度，验证按长度切分的最后一段不足 sub_length
    check([5, 1, 4, 2, 3], [1, 2, 3, 4, 5])

    # 规模用例：1000 个乱序节点，与内置排序结果对照
    big = [(i * 37) % 1000 - 500 for i in range(1000)]
    check(big, sorted(big))

    print("all tests passed")
