"""
============================================================================
104. 二叉树的最大深度（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-depth-of-binary-tree/
分类：二叉树 · 深度（DFS / BFS）

给定一个二叉树 root ，返回其最大深度。

二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：3

示例 2：
输入：root = [1,null,2]
输出：2

提示：
- 树中节点的数量在 [0, 10^4] 区间内。
- -100 <= Node.val <= 100
"""
from collections import deque
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None, right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法一：DFS（后序递归取较大值）【最优解】

为什么慢：如果按层去数，必须真的把每一层都走完、并在每层结束时计数；
虽然也是 O(n)，但要多维护一个队列，空间随层宽增长。

换个角度：最大深度满足递归关系 —— 空树深度 0，非空树深度是
`1 + max(左子树深度, 右子树深度)`。把问题拆成两个子问题再合并，
一次后序遍历即可。

关键点：
  · 空节点返回 0，这是递归的边界，也是叶子返回 1 的来源
  · 必须取 max 而不是固定走某一侧；写成 `left + right` 就变成求直径了
  · 后序位置才相加：先拿到左右子树的深度，再 +1 计当前节点
  · 递归深度 = 结果高度，最坏（斜链）O(n)；本题上限 10^4，
    本地自测压到 200，避免触及 Python 默认递归上限

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  叶子 9、15、7 返回 1；节点 20 返回 1 + max(1,1) = 2；
  根 3 返回 1 + max(1,2) = 3

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈；最坏（斜链）O(n)

解法二：BFS 按层计数【非最优】

官方方法二。层序遍历时每处理完一层就把层数加一，循环结束时层数就是最大深度。
  · 用队列，每轮先记下本层节点数 size；
  · 弹出 size 个节点并把孩子入队，然后 depth += 1。

关键点：
  · size 必须在 for 循环之前锁定，循环中队列还会增长
  · 空树返回 0：不要入队再靠循环兜底
  · 每层结束才 depth += 1，不能每弹出一个节点就加
  · 空间 O(w)：宽树时队列可能存下整层，比 DFS 更费

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
"""


class Solution1:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))


class Solution2:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        depth = 0
        q = deque([root])
        while q:
            size = len(q)
            for _ in range(size):
                node = q.popleft()
                if node.left is not None:
                    q.append(node.left)
                if node.right is not None:
                    q.append(node.right)
            depth += 1
        return depth


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


def check(level_order: list, want: int) -> None:
    assert Solution1().maxDepth(build_tree(level_order)) == want
    assert Solution2().maxDepth(build_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([3, 9, 20, None, None, 15, 7], 3)

    # 示例 2：只有右孩子
    check([1, None, 2], 2)

    # 边界：空树
    check([], 0)

    # 边界：单节点
    check([1], 1)

    # 边界：左斜链 / 右斜链，深度等于节点数
    check([1, 2, None, 3, None, 4], 4)
    check([1, None, 2, None, 3], 3)

    # 边界：完全二叉树，深度 3
    check([1, 2, 3, 4, 5, 6, 7], 3)

    # 边界：左子树深、右子树浅（不能只看右边）
    check([1, 2, 3, 4, None, None, None, 5], 4)

    # 边界：负数值不影响结构
    check([-100, -50, 100, 0], 3)

    # 边界：深链 200 层
    level_order = []
    for i in range(1, 201):
        level_order.extend([i, None])
    check(level_order, 200)

    print("all tests passed")
