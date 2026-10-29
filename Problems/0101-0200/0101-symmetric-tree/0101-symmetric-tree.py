"""
============================================================================
101. 对称二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/symmetric-tree/
分类：二叉树 · 递归 / 队列

给你一个二叉树的根节点 root ， 检查它是否轴对称。

示例 1：
输入：root = [1,2,2,3,4,4,3]
输出：true

示例 2：
输入：root = [1,2,2,null,3,null,3]
输出：false

提示：
- 树中节点数目在范围 [1, 1000] 内
- -100 <= Node.val <= 100

进阶：你可以运用递归和迭代两种方法解决这个问题吗？
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
解法一：递归（对左右子树做镜像比较）【最优解】

为什么慢：直接的想法是"把树翻转一份，再和原树逐节点比较相等"，能过，
但要多存一整棵树；或者按层序把每层取出来、判回文，也很容易在 null
位置上出错（空位也要参与比较）。

换个角度：轴对称的本质不是"左右子树相同"，而是"左右子树互为镜像"。
于是问题变成：写一个 `mirror(a, b)`，判断 a 与 b 是否镜像：
  · 都空 → True；
  · 一个空一个不空 → False；
  · 值不同 → False；
  · 否则 a 的左要和 b 的右镜像、a 的右要和 b 的左镜像。
  从 `mirror(root.left, root.right)` 出发即可，不需要额外的翻转。

关键点：
  · 比较对象是"左对右"，不是"左对左"，这是全题唯一容易写错的地方
  · 两个为空的检查必须放在取 val 之前，否则抛 AttributeError
  · 叶子节点：镜像比较返回 (左空、右空) 的与，结果为 True
  · 根节点自身不需要比较，只需要它的左右子树互为镜像
  · 递归深度 = 树高，节点数 ≤ 1000，Python 默认递归上限够用

以示例 1（root = [1,2,2,3,4,4,3]）为例：
  mirror(2左, 2右)：值相等；比较 (3 与 右3)、(4 与 左4) 都成立；
  逐层向内全部满足 → True ✔
以示例 2（root = [1,2,2,null,3,null,3]）为例：
  mirror(2左, 2右)：值相等；2左的右是 3，而 2右的左是 None → False ✔

复杂度分析：
  时间 O(n)：每个节点只参与一次镜像比较
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：迭代（队列成对出队）【非最优】

官方方法二。用一个队列成对存放"应该镜像的两个节点"，初始放
`(root.left, root.right)`：
  · 弹出 (a, b)：都空则继续；一个空则 False；值不同则 False；
  · 依次压入镜像配对：(a.left, b.right)、(a.right, b.left)。
队列空时说明全部配对成功。

关键点：
  · 空节点也要入队（用 None 表示），否则无法判断"一边有孩子一边没有"
  · 入队顺序必须是"左对右、右对左"，不能写成本身
  · 队列里允许存 None，取出后再判空
  · 空间 O(w)：宽树时队列可能存下整层节点对

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
"""


class Solution1:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True

        def mirror(a: Optional[TreeNode], b: Optional[TreeNode]) -> bool:
            if a is None and b is None:
                return True
            if a is None or b is None:
                return False
            if a.val != b.val:
                return False
            return mirror(a.left, b.right) and mirror(a.right, b.left)

        return mirror(root.left, root.right)


class Solution2:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True

        q = deque([(root.left, root.right)])
        while q:
            a, b = q.popleft()

            if a is None and b is None:
                continue
            if a is None or b is None:
                return False
            if a.val != b.val:
                return False

            q.append((a.left, b.right))
            q.append((a.right, b.left))
        return True


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


def check(level_order: list, want: bool) -> None:
    assert Solution1().isSymmetric(build_tree(level_order)) == want
    assert Solution2().isSymmetric(build_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([1, 2, 2, 3, 4, 4, 3], True)

    # 示例 2
    check([1, 2, 2, None, 3, None, 3], False)

    # 边界：单节点
    check([1], True)

    # 边界：两个孩子的值不同
    check([1, 2, 3], False)

    # 边界：两个孩子的值相同
    check([1, 2, 2], True)

    # 边界：一边有内层孩子，另一边没有
    check([1, 2, 2, 3, None, None, None], False)

    # 边界：一侧是右孩子、另一侧是左孩子，仍然镜像
    check([1, 2, 2, None, 3, 3, None], True)

    # 边界：镜像层差 —— 左侧有两个 3，右侧只有一个 3
    check([1, 2, 2, 3, 3, 3, None], False)

    # 边界：完全镜像的四层满二叉树
    check([1, 2, 2, 3, 4, 4, 3, 5, 6, 7, 8, 8, 7, 6, 5], True)

    # 边界：满二叉树里改坏一个叶子
    check([1, 2, 2, 3, 4, 4, 3, 5, 6, 7, 8, 8, 7, 6, 9], False)

    # 边界：负数不影响结构
    check([-100, -50, -50, 0, 1, 1, 0], True)

    print("all tests passed")
