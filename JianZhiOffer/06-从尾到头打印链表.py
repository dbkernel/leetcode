"""
============================================================================
LCR 123 / 剑指 Offer 06. 图书整理 I（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/
分类：链表 · 递归 / 辅助栈

书店店员有一张链表形式的书单，每个节点代表一本书，节点中的值表示书的
编号。为更方便整理书架，店员需要将书单倒过来排列，就可以从最后一本书
开始整理，逐一将书放回到书架上。请倒序返回这个书单链表。

示例 1：
输入：head = [3,6,4,1]
输出：[1,4,6,3]

提示：
- 0 <= 链表长度 <= 10000

说明：本题在 leetcode.cn 已改号为 LCR 123，题名由「从尾到头打印链表」
改为「图书整理 I」，函数签名由 `reversePrint(ListNode*)` 改为
`reverseBookList(ListNode*)`，返回值仍为节点值组成的数组。
"""
import sys
from typing import List, Optional


# 力扣预置的节点定义
class ListNode:
    def __init__(self, val: int = 0, next: "Optional[ListNode]" = None):
        self.val = val
        self.next = next


"""
解法一：递归到表尾，回溯时写值【最优解】

为什么慢：链表只能从头往后走，如果先顺序存值再整体反转，确实也能做，
但那只是把「逆序」这一步推给了另一个数组；而且同样要 O(n) 的额外空间。
递归则利用函数调用栈，把逆序这件事交给回溯顺序完成。

换个角度：要输出「最后一个节点、倒数第二个……」，就先递归处理 next，
等 next 全部处理完，再把当前节点值写进结果。递归基是空节点。

关键点：
  · append 必须写在递归调用之后，顺序反了就变成正序
  · 递归基覆盖 head is None，空链表直接得到空列表
  · 递归深度等于链表长度，n = 10000 时要先调高 recursionlimit，
    否则默认的 1000 层会直接抛 RecursionError

以 [3,6,4,1] 为例：
  递归到 1，回溯写入 1；再回溯写入 4、6、3
  结果 [1,4,6,3]  ✔

复杂度分析：
  时间 O(n)：每个节点进入递归一次、写结果一次
  空间 O(n)：递归栈最深 n 层，另有 O(n) 的返回列表
"""


class Solution1:
    def reverseBookList(self, head: Optional[ListNode]) -> List[int]:
        res = []

        def dfs(node: Optional[ListNode]) -> None:
            if node is None:
                return
            # 先处理后面的节点，回溯时当前节点才轮到写入
            dfs(node.next)
            res.append(node.val)

        dfs(head)
        return res


"""
解法二：辅助栈【最优解，与解法一并列】

为什么慢：递归写法代码短，但每一层调用都要压一次函数栈；如果语言对
递归深度有限制，长链表会直接报错。显式栈把「后进先出」写出来，不受
递归深度限制。

换个角度：顺序遍历链表，把节点值压栈；遍历结束后从栈顶依次弹出，弹出
顺序就是链表的逆序。

关键点：
  · 先压后弹，弹出顺序才是逆序；边遍历边输出仍是正序
  · 用 list 当栈即可，append 入栈、pop 出栈，不必引入额外容器
  · 空链表时栈为空，返回空列表

以 [3,6,4,1] 为例：
  压栈顺序 3、6、4、1，栈顶是 1
  弹出顺序 1、4、6、3  ✔

复杂度分析：
  时间 O(n)：一次遍历压栈，一次弹栈
  空间 O(n)：栈里最多 n 个值，另有 O(n) 的返回列表
  与解法一时间和空间同阶：递归短、显式栈不受递归深度限制，按需取舍
"""


class Solution2:
    def reverseBookList(self, head: Optional[ListNode]) -> List[int]:
        stk = []
        node = head
        while node is not None:
            stk.append(node.val)
            node = node.next

        res = []
        while stk:
            res.append(stk.pop())
        return res


# 按数组顺序建链表，返回头节点；空数组返回 None
def build_list(values: List[int]) -> Optional[ListNode]:
    dummy = ListNode()
    tail = dummy
    for value in values:
        tail.next = ListNode(value)
        tail = tail.next
    return dummy.next


# 两个解法各自建一条新链表，避免前一个解法改动影响后一个
def check(values: List[int], want: List[int]) -> None:
    assert Solution1().reverseBookList(build_list(values)) == want
    assert Solution2().reverseBookList(build_list(values)) == want


if __name__ == "__main__":
    # 递归解法按题目上限 10000 层，默认限制不够
    sys.setrecursionlimit(100000)

    # 示例 1
    check([3, 6, 4, 1], [1, 4, 6, 3])

    # 边界：空链表，两个解法都应返回空列表
    check([], [])

    # 边界：单节点，逆序后还是自己
    check([5], [5])

    # 反直觉：两个节点，不能只交换值而漏掉 next
    check([1, 2], [2, 1])

    # 边界：负数和零不影响逆序
    check([-5000, 0, 5000], [5000, 0, -5000])

    # 边界：最长 10000 个节点，验证递归深度与栈实现的一致性
    values = list(range(1, 10001))
    check(values, list(reversed(values)))

    print("all tests passed")
