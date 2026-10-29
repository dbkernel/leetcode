"""
============================================================================
147. 对链表进行插入排序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/insertion-sort-list/
分类：链表 · 插入排序 / 归并排序

给定单个链表的头 head ，使用 插入排序 对链表进行排序，并返回 排序后链表
的头 。

插入排序 算法的步骤:

1. 插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的
   输出列表。
2. 每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列
   中适当的位置，并将其插入。
3. 重复直到所有输入数据插入完为止。

下面是插入排序算法的一个图形示例。部分排序的列表(黑色)最初只包含列表中的
第一个元素。每次迭代时，从输入数据中删除一个元素(红色)，并就地插入已排序
的列表中。

对链表进行插入排序。

示例 1：
输入: head = [4,2,1,3]
输出: [1,2,3,4]

示例 2：
输入: head = [-1,5,3,4,0]
输出: [-1,0,3,4,5]

提示：
- 列表中的节点数在 [1, 5000] 范围内
- -5000 <= Node.val <= 5000
"""
from typing import Optional


# 力扣预置的节点定义
class ListNode:
    def __init__(self, val: int = 0, next: "Optional[ListNode]" = None):
        self.val = val
        self.next = next


"""
解法一：链表插入排序（官方方法，题面指定算法）【最优解】

为什么慢：数组插入排序的代价在「搬移元素」，链表插入只需改指针，插入本身
是 O(1)；但单链表没有前驱指针，每次都要从表头走一遍才能找到插入点，单次
定位 O(n)，n 次插入累计 O(n^2)。输入已经有序时，last_sorted 一次比较就能
跳过，退化成 O(n)，但逆序输入每次都要从头扫。

换个角度：把链表分成「已排序段」和「待插入段」。已排序段用 last_sorted
标记尾节点，待插入节点是 last_sorted.next。如果它不小于尾节点，直接接到
尾部；否则从 dummy 开始找第一个严格大于它的位置插进去。

关键点：
  · 先比较 last_sorted.val <= curr.val：相等时留在后面，插入排序才是稳定的
  · 插入前必须先把 last_sorted.next 接到 curr.next，否则 curr 原来的后继
    会丢
  · curr 插到 last_sorted 前面时，last_sorted 不动；所以循环末尾统一用
    curr = last_sorted.next 取下一个待排节点，而不是 curr = curr.next
  · dummy 节点让 curr 可以插到头节点之前，返回 dummy.next

以示例 1（head = [4,2,1,3]）为例：
  已排序段 [4]，curr = 2：2 < 4，插到 4 前面 → [2,4]
  curr = 1：1 < 4，插到最前 → [1,2,4]
  curr = 3：3 < 4，插到 4 前面 → [1,2,3,4]

复杂度分析：
  时间 O(n^2)：每插入一个节点最多从头扫描 O(n)
  空间 O(1)：只用了 dummy、last_sorted、curr、prev 几个引用

解法二：自顶向下归并排序【非最优】

为什么慢：解法一的瓶颈是「每次插入都要重新定位」。链表不能随机访问，插入
排序无法用二分把定位降到 O(log n)。

换个角度：既然最终只要求升序，可以放弃插入排序，改用分治：快慢指针把链表
切成两半，递归排好左右两半，再合并两个有序链表。合并只改指针，不需要额外
节点。

关键点：
  · fast 从 head.next 起步，偶数长度时 slow 停在左半段的最后一个节点，
    两半长度差不超过 1
  · 切分时必须令 slow.next = None，否则递归不会终止
  · 合并时用 dummy 节点承接，比较用 <= 保持稳定
  · 渐近上比插入排序更优，但题面明确要求「使用插入排序」，所以它不算本题
    的解；此处保留只为对照，本题以解法一为准

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归栈深度为 log n；节点本身复用，没有新建链表
"""


class Solution1:
    def insertionSortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None

        dummy = ListNode(0)
        dummy.next = head
        last_sorted = head
        curr = head.next

        while curr is not None:
            if last_sorted.val <= curr.val:
                # 已排序段整体不大于 curr，直接把它并入尾部
                last_sorted = last_sorted.next
            else:
                # 从 dummy 起找第一个严格大于 curr 的节点，插在它前面
                prev = dummy
                while prev.next.val <= curr.val:
                    prev = prev.next

                # 先摘下 curr，再接到 prev 后面，顺序反了会丢后继
                last_sorted.next = curr.next
                curr.next = prev.next
                prev.next = curr
            curr = last_sorted.next
        return dummy.next


class Solution2:
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

        left = self.sortList(head)
        right = self.sortList(mid)
        return self._merge(left, right)

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


# ===== 测试辅助：数组建表、链表转数组 =====
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
    got1 = to_list(Solution1().insertionSortList(build_list(input_values)))
    got2 = to_list(Solution2().sortList(build_list(input_values)))
    assert got1 == want
    assert got2 == want


if __name__ == "__main__":
    # 示例 1
    check([4, 2, 1, 3], [1, 2, 3, 4])

    # 示例 2
    check([-1, 5, 3, 4, 0], [-1, 0, 3, 4, 5])

    # 边界：空链表（题面保证至少 1 个节点，这里验证健壮性）
    check([], [])

    # 边界：单节点
    check([7], [7])

    # 边界：已经有序，插入排序应只比较尾节点
    check([1, 2, 3, 4, 5], [1, 2, 3, 4, 5])

    # 边界：完全逆序，每次都要插到最前面
    check([5, 4, 3, 2, 1], [1, 2, 3, 4, 5])

    # 边界：全部相同，验证稳定性与不会死循环
    check([3, 3, 3, 3], [3, 3, 3, 3])

    # 边界：重复值夹杂在乱序里
    check([2, 1, 2, 1, 2], [1, 1, 2, 2, 2])

    # 边界：负数与正数混合
    check([0, -5000, 5000, -1, 1], [-5000, -1, 0, 1, 5000])

    print("all tests passed")
