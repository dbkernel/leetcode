"""
============================================================================
面试题 04.06. 后继者（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/successor-lcci/
分类：二叉搜索树 · 中序后继

设计一个算法，找出二叉搜索树中指定节点的“下一个”节点（也即中序后继）。

如果指定节点没有对应的“下一个”节点，则返回 null。

示例 1:
输入: root = [2,1,3], p = 1
输出: 2

示例 2:
输入: root = [5,3,6,2,4,null,null,1], p = 6
输出: null
"""
from typing import List, Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, x: int):
        self.val = x
        self.left = None
        self.right = None


"""
解法一：中序遍历收集全部节点【非最优】

为什么慢：中序序列天然升序，找到 p 之后的下一个就是答案，思路零门槛；
但要把整棵树存进数组才能取"下一个"，时间 O(n)、额外空间 O(n)，
而且明明只需要一条从根到 p 的路径信息。

换个角度：真正需要的只是"p 之后第一个被访问的节点"。如果能让遍历在
访问到 p 时记下状态，下一个被访问的节点就是答案，就无需存整棵序列。

关键点：
  · 中序遍历的访问顺序是左 → 根 → 右，p 的后继就是 p 之后第一个访问到的节点
  · 用一个 found 标记记录"是否已经访问过 p"
  · 找到后继后要能提前收手，否则会被后续节点覆盖答案
  · 递归写法最直观，但额外 O(n) 的数组是它的主要缺点

以示例 1（root = [2,1,3]，p = 1）为例：
  中序为 1、2、3，p = 1 之后是 2 ✔

复杂度分析：
  时间 O(n)：完整中序遍历
  空间 O(n)：保存中序序列（若改成"边遍历边判断"可以降到 O(h)）

解法二：利用 BST 性质（不遍历全树）【最优解】

官方方法二。分两种情况，都不需要访问整棵树：
  1. p 有右子树：p 的中序后继一定是右子树里的**最小**节点，
     也就是右子树一路向左走到底的那个节点；
  2. p 没有右子树：后继在"祖先"里 —— 从根往下找，每当遇到比 p 大的节点，
     它就有可能是答案（先记下来），然后向左子树继续找更近的；
     遇到比 p 小或相等的节点则向右走。走到底记下的最后一个"更大的祖先"
     就是答案。
情况 2 的本质是：后继是"大于 p 的最小祖先"。

关键点：
  · 情况 1 别忘了先判断 p.right 是否为空
  · 情况 2 里 successor 要在"往左走"之前更新，往右走时不更新
  · 比较用值即可（题目保证值互不相同）；若用 `is` 比较对象，注意 Python
    里我们直接持有节点引用，更严谨也更直观
  · 都没找到说明 p 是整棵树的最大值，返回 None
  · 只沿一条路径走，时间 O(h)、空间 O(1)，明显优于解法一

以示例 2（root = [5,3,6,2,4,null,null,1]，p = 6）为例：
  6 没有右子树 → 从根找"大于 6 的最小祖先"：
  5 < 6 往右到 6；6 不大于 6 往右到空 → 没有更大的祖先 → 返回 None ✔
以 p = 4 为例：
  4 没有右子树 → 5 > 4，successor = 5，往左到 3；3 < 4 往右到 4；
  4 不大于 4 → 往右到空，返回 successor = 5 ✔

复杂度分析：
  时间 O(h)：h 为树高，最坏（斜链）O(n)
  空间 O(1)：只用几个指针，连递归栈都不需要
"""


class Solution1:
    def inorderSuccessor(self, root: TreeNode, p: TreeNode) -> Optional[TreeNode]:
        order: List[TreeNode] = []

        def dfs(node: Optional[TreeNode]) -> None:
            if node is None:
                return
            dfs(node.left)
            order.append(node)
            dfs(node.right)

        dfs(root)
        for i in range(len(order) - 1):
            if order[i] is p:
                return order[i + 1]
        return None


class Solution2:
    def inorderSuccessor(self, root: TreeNode, p: TreeNode) -> Optional[TreeNode]:
        # 情况 1：有右子树，后继是右子树中的最小节点
        if p.right is not None:
            node = p.right
            while node.left is not None:
                node = node.left
            return node

        # 情况 2：没有右子树，后继是"大于 p 的最小祖先"
        successor = None
        node = root
        while node is not None:
            if node.val > p.val:
                successor = node  # 先记下这个候选
                node = node.left
            else:
                node = node.right
        return successor


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


def find_node(root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
    while root is not None and root.val != val:
        root = root.left if val < root.val else root.right
    return root


def check(level_order: list, p_val: int, want: int) -> None:
    tree1 = build_tree(level_order)
    got1 = Solution1().inorderSuccessor(tree1, find_node(tree1, p_val))
    assert (got1.val if got1 is not None else -1) == want

    tree2 = build_tree(level_order)
    got2 = Solution2().inorderSuccessor(tree2, find_node(tree2, p_val))
    assert (got2.val if got2 is not None else -1) == want


if __name__ == "__main__":
    # 示例 1
    check([2, 1, 3], 1, 2)

    # 示例 2
    check([5, 3, 6, 2, 4, None, None, 1], 6, -1)

    # 同一个树上的其它后继
    check([5, 3, 6, 2, 4, None, None, 1], 1, 2)
    check([5, 3, 6, 2, 4, None, None, 1], 2, 3)
    check([5, 3, 6, 2, 4, None, None, 1], 3, 4)
    check([5, 3, 6, 2, 4, None, None, 1], 4, 5)
    check([5, 3, 6, 2, 4, None, None, 1], 5, 6)

    # 边界：单节点，没有后继
    check([1], 1, -1)

    # 边界：有右子树时，后继是右子树的最左节点
    check([2, 1, 3], 2, 3)

    # 边界：左斜链 [3,2,null,1]，降序中序 1,2,3
    check([3, 2, None, 1], 1, 2)
    check([3, 2, None, 1], 2, 3)
    check([3, 2, None, 1], 3, -1)

    # 边界：负数值
    check([0, -1, 1], -1, 0)
    check([0, -1, 1], 1, -1)

    # 边界：100 层右斜链 1..100
    chain = [1]
    for i in range(2, 101):
        chain.extend([None, i])
    check(chain, 1, 2)
    check(chain, 99, 100)
    check(chain, 100, -1)

    print("all tests passed")
