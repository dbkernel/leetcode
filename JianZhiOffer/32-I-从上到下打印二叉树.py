"""
============================================================================
LCR 149 / 剑指 Offer 32 - I. 从上到下打印二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/
分类：二叉树 · 层序遍历（BFS / 队列）

一棵圣诞树记作根节点为 root 的二叉树，节点值为该位置装饰彩灯的颜色编号。
请按照从 左 到 右 的顺序返回每一层彩灯编号。

说明：本题原名「从上到下打印二叉树」，leetcode.cn 现已改用
decorateRecord 作为函数名，题面意思不变。

示例 1：
输入：root = [8,17,21,18,null,null,6]
输出：[8,17,21,18,6]

提示：
- 节点总数 <= 1000
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
解法：BFS + 队列【最优解】

为什么慢：如果按「深度优先」去搜，虽然每个节点只访问一次，但得到的顺序是
一条路走到黑（根-左子树整条-右子树整条），要还原成层序还得额外记层号再排序，
反而把简单问题复杂化。

换个角度：题目要的正是「先来先访问」的顺序 —— 先遇到第 1 层，再遇到第 2 层。
这正是先进先出队列的语义：把根入队，每次从队头取出一个节点输出，并把它
非空的左右孩子依次接到队尾。

关键点：
  · 入队顺序必须是「先左后右」，否则同一层内的顺序会反
  · 孩子为空时不要入队，否则会把 None 带进队列，后续取值崩溃
  · 根为空要单独处理：直接返回空列表，不要入队
  · 队列里同时最多存两层的节点（当前层剩余 + 下一层），空间 O(w)，
    w 为最大层宽；本题不要求按层分组，所以不需要记录层大小
  · 用 collections.deque：popleft 是 O(1)，list 的 pop(0) 是 O(n)

以示例 1（root = [8,17,21,18,null,null,6]）为例：
  队列 [8] → 输出 8，入队 17、21
  输出 17，入队 18 → 输出 21，入队 6
  输出 18、6 → [8,17,21,18,6]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为树的最大层宽，最坏（完全二叉树最后一层）O(n)
"""


class Solution:
    def decorateRecord(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        if root is None:
            return res

        q = deque([root])
        while q:
            node = q.popleft()
            res.append(node.val)

            # 先左后右，保证同层内从左到右
            if node.left is not None:
                q.append(node.left)
            if node.right is not None:
                q.append(node.right)
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
    assert Solution().decorateRecord(build_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([8, 17, 21, 18, None, None, 6], [8, 17, 21, 18, 6])

    # 边界：空树
    check([], [])

    # 边界：单节点
    check([1], [1])

    # 边界：完全二叉树，验证同层从左到右
    check([1, 2, 3, 4, 5, 6, 7], [1, 2, 3, 4, 5, 6, 7])

    # 边界：左斜链 / 右斜链
    check([1, 2, None, 3], [1, 2, 3])
    check([1, None, 2, None, 3], [1, 2, 3])

    # 边界：右孩子缺失、左孩子存在（不能因为右空就提前结束本层）
    check([1, 2, 3, 4, None, None, 5], [1, 2, 3, 4, 5])

    # 边界：负数值
    check([-100, -50, 100, 0], [-100, -50, 100, 0])

    # 边界：满二叉树三层，层序就是 1..15
    check(list(range(1, 16)), list(range(1, 16)))

    print("all tests passed")
