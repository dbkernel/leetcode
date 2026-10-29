"""
============================================================================
543. 二叉树的直径（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/diameter-of-binary-tree/
分类：二叉树 · 后序递归

给你一棵二叉树的根节点，返回该树的 直径 。

二叉树的 直径 是指树中任意两个节点之间最长路径的 长度 。这条路径可能经过也可能不经过根节点 root 。

两节点之间路径的 长度 由它们之间边数表示。

示例 1：
输入：root = [1,2,3,4,5]
输出：3
解释：3 ，取路径 [4,2,1,3] 或 [5,2,1,3] 的长度。

示例 2：
输入：root = [1,2]
输出：1

提示：
- 树中节点数目在范围 [1, 10^4] 内
- -100 <= Node.val <= 100
"""
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None, right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法一：后序递归，边求深度边更新直径【最优解】

为什么慢：最直接的想法是"枚举每个节点，把它当作路径的最高点（拐点），
用左右子树的深度算出经过它的最长路径，取全局最大"。但若每遇到一个节点都
重新递归求一次左右深度，同一条链会被反复数很多遍，最坏（斜链）退化成
O(n^2)。

换个角度：求某个节点的左右深度本身就是后序遍历的一部分。一次后序遍历
返回"以该节点为端点的最长向下路径（边数）"，同时顺手用 `left + right + 2`
更新全局答案 —— 这条和正是以当前节点为拐点的最长路径边数。
深度计算和答案更新共用同一次遍历，信息不浪费。

关键点：
  · 统一用"边数"计量：dfs 返回从该节点向下到叶子的最长**边数**，
    空树返回 -1，叶子返回 0，则 `1 + max(左, 右)`
  · 这样经过当前节点的最长路径边数恰好是 `左 + 右 + 2`
    （两条向下的边各自 +1 才接到当前节点）
  · 答案必须在后序位置（拿到左右结果之后）更新，不能先序
  · 直径不一定经过根：全局最大值要在每个节点处都比较，而不是只算根
  · 空树是最小单位，返回 -1 让叶子的 +2 归零，公式无需特判
  · Python 用 `nonlocal` 更新外层答案，避免类成员或列表包裹

以示例 1（root = [1,2,3,4,5]）为例：
  叶子 4、5：更新 -1 + -1 + 2 = 0，返回 0
  节点 2：更新 0 + 0 + 2 = 2（路径 4-2-5），返回 1
  节点 3：更新 0，返回 0
  根 1：更新 1 + 0 + 2 = 3（路径 4-2-1-3）→ 答案 3 ✔

复杂度分析：
  时间 O(n)：每个节点只访问一次
  空间 O(h)：递归栈深度等于树高，最坏（斜链）O(n)

解法二：枚举每个节点求左右深度（暴力）【非最优】

对每个节点都调用一次 `max_depth` 求左右深度，取 `ld + rd + 2` 的最大值，
再递归到左右孩子继续枚举。
  · 思路最直白，完全对应"直径 = max over x of 左深度 + 右深度 + 2"
  · 但每个节点被重复计算 O(h) 次，斜链上退化为 O(n^2)
  · 只适合用来对照验证解法一

复杂度分析：
  时间 O(n^2)：平均 O(n log n)，斜链最坏 O(n^2)
  空间 O(h)：递归栈
"""


class Solution1:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        ans = 0

        # 空树约定为 -1，叶子为 0
        def depth(node: Optional[TreeNode]) -> int:
            nonlocal ans
            if node is None:
                return -1
            left = depth(node.left)
            right = depth(node.right)
            ans = max(ans, left + right + 2)
            return 1 + max(left, right)

        depth(root)
        return ans


class Solution2:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        best = self.max_depth(root.left) + self.max_depth(root.right) + 2
        best = max(best, self.diameterOfBinaryTree(root.left))
        best = max(best, self.diameterOfBinaryTree(root.right))
        return best

    # 空树 -1，叶子 0，与解法一保持同一约定
    def max_depth(self, node: Optional[TreeNode]) -> int:
        if node is None:
            return -1
        return 1 + max(self.max_depth(node.left), self.max_depth(node.right))


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
    assert Solution1().diameterOfBinaryTree(build_tree(level_order)) == want
    assert Solution2().diameterOfBinaryTree(build_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([1, 2, 3, 4, 5], 3)

    # 示例 2
    check([1, 2], 1)

    # 边界：单节点，没有边
    check([1], 0)

    # 边界：空树
    check([], 0)

    # 边界：右斜链 / 左斜链
    check([1, None, 2, None, 3], 2)
    check([1, 2, None, 3], 2)

    # 边界：直径不经过根 —— 长路径藏在左子树内部
    check([1, 2, 3, 4, 5, None, None, 6, None, 7, None, 8, None, 9, None], 6)

    # 边界：满二叉树，直径穿过根
    check([1, 2, 3, 4, 5, 6, 7], 4)

    # 边界：负数值不影响结构
    check([-100, -50, 100], 2)

    # 边界：深链 200 层，答案 199
    level_order = []
    for i in range(1, 201):
        level_order.extend([i, None])
    check(level_order, 199)

    print("all tests passed")
