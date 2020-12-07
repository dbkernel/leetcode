'''
给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例：
输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        # -> 是python3的特性，标记返回值信息
        head = ListNode(0)  # 头结点，无存储，指向链表第一个结点
        node = head  # 初始化链表结点
        carry = 0  # 进一 标记
        while l1 or l2:
            x = l1.val if l1 else 0
            y = l2.val if l2 else 0
            sum = x + y + carry  # 对每一位求和
            carry = sum // 10  # 求进位。" / " 就表示 浮点数除法，返回浮点结果; " // " 表示整数除法。
            node.next = ListNode(sum % 10)
            if l1:  # 判断是否为空，防止出现无后继结点
                l1 = l1.next
            if l2:  # 同上
                l2 = l2.next
            node = node.next  # 更新指针
        if carry != 0:  # 验证最后一位相加是否需 进一，比如 [9, 9] + [1] = [0, 0, 1]
            node.next = ListNode(1)
        return head.next  # 返回头结点的下一个结点，即链表的第一个结点
