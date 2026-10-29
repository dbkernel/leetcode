"""
============================================================================
102. 二叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-level-order-traversal/
分类：二叉树 · 层序遍历（BFS / 队列）

给你二叉树的根节点 root ，返回其节点值的 层序遍历 。
（即逐层地，从左到右访问所有节点）。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：[[3],[9,20],[15,7]]

示例 2：
输入：root = [1]
输出：[[1]]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目在范围 [0, 2000] 内
- -1000 <= Node.val <= 1000
"""
from collections import deque
from typing import List, Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None, right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法：BFS + 按层定长出队【最优解】

为什么慢：最朴素的 BFS 只知道「下一个是谁」，并不知道「这一层到哪儿结束」。
如果给每个节点额外挂一个层号，再用哈希表按层号分组，就要多存 O(n) 个层号；
最后还要按下标顺序遍历哈希表，空间和实现都变重。

换个角度：BFS 的队列有一个几乎白送的性质 —— 某一时刻队列里的元素，恰好就是
「当前这一层剩下的节点 + 下一层已经入队的节点」。因此只要在每轮开始时先记下
`size = len(q)`，就锁定了本层的边界：接下来连续出队 size 个，得到的就是完整
的一层，而这 size 个节点新入队的恰好全是下一层。

关键点：
  · size 必须在 for 循环之前取，循环中队列长度会增长，不能写成 `len(q)`
  · 每轮新建一个 level 列表，循环结束后再 append 到答案，避免共用同一个列表
  · 入队仍要「先左后右」，并且跳过空孩子，才能保证同层从左到右
  · 根为空直接返回空列表：不要入队再靠循环兜底
  · 用 collections.deque：popleft 是 O(1)，list 的 pop(0) 是 O(n)

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  第 1 轮 size=1 → 出 3，入 9、20 → [[3]]
  第 2 轮 size=2 → 出 9，入空；出 20，入 15、7 → [[3],[9,20]]
  第 3 轮 size=2 → 出 15、7 → [[3],[9,20],[15,7]]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案数组外，队列最多同时存两层节点
"""


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        res = []
        if root is None:
            return res

        q = deque([root])
        while q:
            # 本层节点个数必须在出队前锁定，循环中队列还会增长
            size = len(q)
            level = []
            for _ in range(size):
                node = q.popleft()
                level.append(node.val)
                if node.left is not None:
                    q.append(node.left)
                if node.right is not None:
                    q.append(node.right)
            res.append(level)
        return res


# ===== 测试辅助：按层序数组（None 表示空）建树 =====
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


def check(level_order: list, want: list) -> None:
    assert Solution().levelOrder(build_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([3, 9, 20, None, None, 15, 7], [[3], [9, 20], [15, 7]])

    # 示例 2：单节点
    check([1], [[1]])

    # 示例 3：空树
    check([], [])

    # 边界：完全二叉树，层宽递增
    check([1, 2, 3, 4, 5, 6, 7], [[1], [2, 3], [4, 5, 6, 7]])

    # 边界：左斜链 / 右斜链，每层只有一个节点
    check([1, 2, None, 3], [[1], [2], [3]])
    check([1, None, 2, None, 3], [[1], [2], [3]])

    # 边界：缺左孩子，层内顺序不能乱
    check([1, 2, 3, None, 4, 5], [[1], [2, 3], [4, 5]])

    # 边界：负数值
    check([-1000, -500, 1000, 0], [[-1000], [-500, 1000], [0]])

    # 边界：满二叉树四层
    want = []
    start = 1
    width = 1
    while width <= 8:
        want.append(list(range(start, start + width)))
        start += width
        width <<= 1
    check(list(range(1, 16)), want)

    print("all tests passed")
