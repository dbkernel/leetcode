"""
============================================================================
617. 合并二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/merge-two-binary-trees/
分类：二叉树 · 递归 / BFS

给你两棵二叉树： root1 和 root2 。

想象一下，当你将其中一棵覆盖到另一棵之上时，两棵树上的一些节点将会重叠
（而另一些不会）。你需要将这两棵树合并成一棵新二叉树。合并的规则是：
如果两个节点重叠，那么将这两个节点的值相加作为合并后节点的新值；
否则，不为 null 的节点将直接作为新二叉树的节点。

返回合并后的二叉树。

注意: 合并过程必须从两个树的根节点开始。

示例 1：
输入：root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
输出：[3,4,5,5,4,null,7]

示例 2：
输入：root1 = [1], root2 = [1,2]
输出：[2,2]

提示：
- 两棵树中的节点数目在范围 [0, 2000] 内
- -10^4 <= Node.val <= 10^4
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
解法一：DFS 递归【最优解】

为什么慢：BFS 也能合并，但要显式维护两个队列，代码量和常数都更大；
而且合并本身是"同位置一对一"的结构化操作，用递归直接对应结构，最自然。

换个角度：把"合并"看成对两个根同时做一次前序/后序都无所谓的遍历：
  · 若 root1 为空，合并结果就是 root2；
  · 若 root2 为空，合并结果就是 root1；
  · 都不空时，新节点值 = 两者之和，左右孩子分别递归合并。
  "某一边为空就把另一边接回来"这一条，一次性处理了"不重叠节点直接保留"
  和"两边都空返回空"两种情况，不需要额外判断。

关键点：
  · 先把两个为空的出口写完，后面就不必再判空
  · 直接复用 root1 作为结果节点，能省一次构造；若不想改动输入，
    则新建节点再递归（下面复用写法，题目允许修改）
  · 递归顺序不影响结果，因为每个位置只合并一次
  · 递归深度 = 树高，节点数 ≤ 2000，不会爆栈
  · 时间复杂度是两棵树节点数之和，而不是较大者

以示例 1 为例（root1 = [1,3,2,5]，root2 = [2,1,3,null,4,null,7]）：
  根 1 + 2 = 3；
  左：3 + 1 = 4，其左 5 + 空 = 5，其右 空 + 4 = 4；
  右：2 + 3 = 5，其左 空，其右 空 + 7 = 7；
  结果 [3,4,5,5,4,null,7] ✔

复杂度分析：
  时间 O(m + n)：每个节点最多访问一次
  空间 O(h)：递归栈，h 为合并后树高；最坏（斜链）O(m + n)

解法二：BFS（队列成对出队）【非最优】

官方方法二。用一个队列存"待合并的节点对 (a, b)"，初始为 (root1, root2)：
  · 弹出 (a, b)，两者都不空；
  · 累加 a.val += b.val；
  · 左孩子：a 空 → 接 b 的左；b 空 → 保持 a 的左；都不空 → 成对入队；
  · 右孩子同理。
  这样任何时刻队列里都是已经存在的节点对，不会出现 None。

关键点：
  · 空树要先处理：root1 空直接返回 root2，反之亦然，否则后面无法成对
  · 只有当两个孩子都不空时才入队；一方为空时直接"嫁接"，不要入队
  · 队列里存的是节点对，注意别把已经合并过的节点重复入队
  · 空间 O(w)：宽树时队列可能存下整层节点对

复杂度分析：
  时间 O(m + n)
  空间 O(w)：w 为合并后树的最大层宽
"""


class Solution1:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if root1 is None:
            return root2
        if root2 is None:
            return root1

        root1.val += root2.val
        root1.left = self.mergeTrees(root1.left, root2.left)
        root1.right = self.mergeTrees(root1.right, root2.right)
        return root1


class Solution2:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if root1 is None:
            return root2
        if root2 is None:
            return root1

        q = deque([(root1, root2)])
        while q:
            a, b = q.popleft()
            a.val += b.val

            # 左孩子：都不空才入队，否则把非空的那一边嫁接过来
            if a.left is not None and b.left is not None:
                q.append((a.left, b.left))
            elif a.left is None:
                a.left = b.left

            # 右孩子同理
            if a.right is not None and b.right is not None:
                q.append((a.right, b.right))
            elif a.right is None:
                a.right = b.right

        return root1


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


def check(lhs: list, rhs: list, want: list) -> None:
    assert same_tree(Solution1().mergeTrees(build_tree(lhs), build_tree(rhs)), build_tree(want))
    assert same_tree(Solution2().mergeTrees(build_tree(lhs), build_tree(rhs)), build_tree(want))


if __name__ == "__main__":
    # 示例 1
    check([1, 3, 2, 5], [2, 1, 3, None, 4, None, 7], [3, 4, 5, 5, 4, None, 7])

    # 示例 2
    check([1], [1, 2], [2, 2])

    # 边界：两棵空树
    check([], [], [])

    # 边界：只有一棵为空 —— 另一棵原样返回
    check([1, 2, 3], [], [1, 2, 3])
    check([], [1, None, 2], [1, None, 2])

    # 边界：root2 在内部位置为空，root1 的节点要保留
    check([1, None, 2], [1, 3], [2, 3, 2])

    # 边界：root1 在内部位置为空，root2 的节点要接过来
    check([1, 3], [1, None, 2], [2, 3, 2])

    # 边界：负数相加
    check([-1], [-2], [-3])

    # 边界：一边深一边浅
    check([1, 2, 3, 4], [1, 2], [2, 4, 3, 4])

    # 边界：两棵 200 层斜链，值逐层翻倍
    chain = []
    want = []
    for i in range(1, 201):
        chain.extend([i, None])
        want.extend([2 * i, None])
    check(chain, chain, want)

    print("all tests passed")
