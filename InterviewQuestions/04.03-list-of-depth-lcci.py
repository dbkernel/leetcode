r"""
============================================================================
面试题 04.03. 特定深度节点链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/list-of-depth-lcci/
分类：二叉树 · 层序遍历（BFS）

给定一棵二叉树，设计一个算法，创建含有某一深度上所有节点的链表（比如，若
一棵树的深度为 D，则会创建出 D 个链表）。返回一个包含所有深度的链表的数组。

示例：
输入：[1,2,3,4,5,null,7,8]

        1
       /  \
      2    3
     / \    \
    4   5    7
   /
  8

输出：[[1],[2,3],[4,5,7],[8]]
"""
from collections import deque
from typing import List, Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int):
        self.val = val
        self.left = None
        self.right = None


class ListNode:
    def __init__(self, val: int):
        self.val = val
        self.next = None


"""
解法：BFS 逐层建链表【最优解】

为什么慢：要找“每一层”，最直接的想法是 DFS 记录深度，把节点塞进 depth 对应
的数组，最后每层再建链表。但 DFS 先访问的节点不一定来自同一层，必须额外维护
depth 数组；它把“分层”这件事交给调用方，函数参数里到处是深度。本质上缺的是
“同一层节点连续处理”的顺序。

换个角度：层序遍历本身就把同一层节点排在一起。BFS 队列里当前这批节点正好是
一整层：先记下队列长度 size，只弹出 size 个节点建一条链表，同时把它们的左右
孩子入队，下一轮队列里就是下一层。每轮产出一条链表，自然对应一个深度。

关键点：
  · 每轮开始先固定 size = len(q)，循环中用 i < size 而不是 len(q)，
    否则本层入队的孩子会把下一层节点卷进来
  · 用哨兵 dummy 建链表：dummy.next 就是本层头节点，tail 负责尾插，
    省掉“第一个节点特殊处理”
  · 孩子入队前判空，空孩子不入队，否则会凭空多出一层
  · 空树直接返回空列表，不能进循环，否则会把 None 入队
  · 返回的是每层链表头节点组成的列表，顺序从浅到深
  · deque 的 popleft 是 O(1)，list 的 pop(0) 是 O(n)，层数多时差距明显

以示例（[1,2,3,4,5,null,7,8]）为例：
  第一轮 size = 1：弹出 1，建 [1]，2、3 入队；
  第二轮 size = 2：弹出 2、3，建 [2,3]，4、5、7 入队；
  第三轮 size = 3：弹出 4、5、7，建 [4,5,7]，只有 4 带左孩子 8；
  第四轮 size = 1：弹出 8，建 [8]，无孩子，队列空；
  返回 [[1],[2,3],[4,5,7],[8]] ✔

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次，每个节点新建一个链表节点
  空间 O(n)：队列最宽一层最多 n/2 个节点，输出本身也占 O(n)
"""


class Solution:
    def listOfDepth(self, tree: Optional[TreeNode]) -> List[ListNode]:
        levels: List[ListNode] = []
        if tree is None:
            return levels

        q = deque([tree])
        while q:
            # 先固定本层大小：本层孩子入队后 len(q) 会变大
            size = len(q)
            dummy = ListNode(0)
            tail = dummy

            for _ in range(size):
                node = q.popleft()

                tail.next = ListNode(node.val)
                tail = tail.next

                if node.left is not None:
                    q.append(node.left)
                if node.right is not None:
                    q.append(node.right)

            # dummy.next 是本层链表头，空层不会出现（size >= 1）
            levels.append(dummy.next)
        return levels


# ===== 测试辅助：按层序数组（None 表示空）建树、收集每层值 =====
def build_tree(level_order: list) -> Optional[TreeNode]:
    if not level_order or level_order[0] is None:
        return None

    root = TreeNode(level_order[0])
    nodes = [root]
    idx = 1
    i = 0
    while i < len(nodes) and idx < len(level_order):
        if level_order[idx] is not None:
            nodes[i].left = TreeNode(level_order[idx])
            nodes.append(nodes[i].left)
        idx += 1
        if idx >= len(level_order):
            break
        if level_order[idx] is not None:
            nodes[i].right = TreeNode(level_order[idx])
            nodes.append(nodes[i].right)
        idx += 1
        i += 1
    return root


def collect_levels(levels: List[ListNode]) -> List[List[int]]:
    got = []
    for head in levels:
        values = []
        cur = head
        while cur is not None:
            values.append(cur.val)
            cur = cur.next
        got.append(values)
    return got


def check(level_order: list, want: List[List[int]]) -> None:
    got = collect_levels(Solution().listOfDepth(build_tree(level_order)))
    assert got == want, f"listOfDepth({level_order}) = {got}, want {want}"


if __name__ == "__main__":
    # 示例：第四层只有一个节点，最容易漏层
    check([1, 2, 3, 4, 5, None, 7, 8], [[1], [2, 3], [4, 5, 7], [8]])

    # 边界：空树，返回空列表而不是含一个空链表的列表
    check([], [])

    # 边界：单节点，只有一层
    check([1], [[1]])

    # 满二叉树，最宽一层有 4 个节点
    check([1, 2, 3, 4, 5, 6, 7], [[1], [2, 3], [4, 5, 6, 7]])

    # 边界：只有左孩子的斜链，每层一个节点
    check([1, 2, None, 3, None, 4], [[1], [2], [3], [4]])

    # 边界：只有右孩子的斜链，每层一个节点
    check([1, None, 2, None, 3], [[1], [2], [3]])

    # 反直觉：中间层缺孩子，但下一层仍有节点
    check([1, 2, 3, None, 4, None, 5], [[1], [2, 3], [4, 5]])

    # 反直觉：左右孩子交叉分布，层内顺序必须从左到右
    check([1, 2, 3, 4, None, None, 5], [[1], [2, 3], [4, 5]])

    # 边界：负数
    check([-1, -2, -3], [[-1], [-2, -3]])

    print("all tests passed")
