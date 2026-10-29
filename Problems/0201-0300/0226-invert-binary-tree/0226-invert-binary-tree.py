"""
============================================================================
226. 翻转二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/invert-binary-tree/
分类：二叉树 · 递归 / 层序遍历

给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。

示例 1：
输入：root = [4,2,7,1,3,6,9]
输出：[4,7,2,9,6,3,1]

示例 2：
输入：root = [2,1,3]
输出：[2,3,1]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目范围在 [0, 100] 内
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
解法一：递归（交换左右孩子 + 递归翻转）【最优解】

为什么慢：按"新建一棵镜像树"的思路写也能过，但要额外构造一整棵树，
空间翻倍；而且原树被丢弃，白白浪费。层序迭代也能做，但要显式维护队列，
代码并不比递归短。

换个角度：翻转是一棵子树一个动作 —— 对当前节点，先交换它的左右孩子，
再分别翻转这两棵子树即可。由于"翻转左右子树"与"交换左右孩子"可交换顺序，
先序、后序写都成立；中序则会因为交换后访问的子树已变而需要小心，一般不用。

关键点：
  · 每次递归只需做两件事：交换 left / right，然后递归两个孩子
  · 空节点直接返回，作为递归出口
  · 交换后递归时的 left / right 已是"翻转前"的另一侧，注意别重复交换
  · 递归深度 = 树高，本题节点数 ≤ 100，绝对安全
  · 若题目不允许修改原树，才需要新建节点；本题允许原地翻转

以示例 1（root = [4,2,7,1,3,6,9]）为例：
  节点 1、3、6、9 无孩子，返回自身；
  节点 2 交换左右（1、3）→ 变 (3,1)；节点 7 同理变 (9,6)；
  根 4 交换左右（2、7）→ (7,2)，结果 [4,7,2,9,6,3,1] ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：BFS 层序迭代【非最优】

官方方法二。用队列做层序遍历，每弹出一个节点就交换它的左右孩子，
再把非空孩子入队 —— 因为交换只依赖节点自身，与访问顺序无关。

关键点：
  · 交换发生在"弹出"时，孩子入队顺序无所谓，反正后面还会各自交换
  · 空树直接返回 root（None），不要先入队
  · 不需要按层分批处理，本题没有"按层"语义，一层一层反而多余
  · 空间 O(w)：宽树时要存下整层节点

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
"""


class Solution1:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None:
            return None

        root.left, root.right = root.right, root.left
        self.invertTree(root.left)
        self.invertTree(root.right)
        return root


class Solution2:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None:
            return None

        q = deque([root])
        while q:
            node = q.popleft()
            node.left, node.right = node.right, node.left
            if node.left is not None:
                q.append(node.left)
            if node.right is not None:
                q.append(node.right)
        return root


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


def same_tree(a: Optional[TreeNode], b: Optional[TreeNode]) -> bool:
    if a is None or b is None:
        return a is b
    return a.val == b.val and same_tree(a.left, b.left) and same_tree(a.right, b.right)


def check(input_data: list, want: list) -> None:
    assert same_tree(Solution1().invertTree(build_tree(input_data)), build_tree(want))
    assert same_tree(Solution2().invertTree(build_tree(input_data)), build_tree(want))


if __name__ == "__main__":
    # 示例 1
    check([4, 2, 7, 1, 3, 6, 9], [4, 7, 2, 9, 6, 3, 1])

    # 示例 2
    check([2, 1, 3], [2, 3, 1])

    # 示例 3：空树
    check([], [])

    # 边界：单节点
    check([1], [1])

    # 边界：只有左孩子 → 翻转后变成右孩子
    check([1, 2], [1, None, 2])

    # 边界：只有右孩子
    check([1, None, 2], [1, 2])

    # 边界：叶子层的孩子也要换位
    check([1, 2, 3, 4], [1, 3, 2, None, None, None, 4])

    # 边界：负数不影响结构
    check([-100, -50, 100], [-100, 100, -50])

    # 边界：左斜链 100 层，翻转后变成同深度的右斜链
    chain = [1]
    for i in range(2, 101):
        chain.extend([i, None])
    want = []
    for i in range(1, 101):
        want.extend([i, None])
    check(chain, want)

    print("all tests passed")
