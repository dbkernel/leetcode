"""
============================================================================
LCR 193 / 剑指 Offer 68 - I. 二叉搜索树的最近公共祖先（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-zui-jin-gong-gong-zu-xian-lcof/
分类：二叉搜索树 · 一次遍历 / 路径比较

给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个结点 p、q，最近公共
祖先表示为一个结点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点
也可以是它自己的祖先）。”

例如，给定如下二叉搜索树:  root = [6,2,8,0,4,7,9,null,null,3,5]

示例 1：
输入：root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
输出：6
解释：节点 2 和节点 8 的最近公共祖先是 6。

示例 2：
输入：root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
输出：2
解释：节点 2 和节点 4 的最近公共祖先是 2, 因为根据定义最近公共祖先节点可以
为节点本身。

说明：
- 所有节点的值都是唯一的。
- p、q 为不同节点且均存在于给定的二叉搜索树中。

注意：本题与主站 235 题相同：
https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/
"""
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, x: int):
        self.val = x
        self.left = None
        self.right = None


"""
解法一：两次遍历求出根到 p、q 的路径，取最后一个公共前缀【非最优】

为什么慢：时间上两趟下降各 O(h)，并不算慢；代价在空间和多余信息：要各存
一条长度最多 h 的路径（斜树上 O(n)），可题目只关心"两条路径从哪里分开"，
中间那些公共前缀节点存下来之后只用得到最后一个。

换个角度：从根走到某个节点的路径，在 BST 上唯一。p、q 的最近公共祖先就是
两条路径的最后一个相同节点 —— 分叉发生的位置。既然路径本身是单调下降的，
就可以边比较边丢掉前缀，只保留"当前还相同"的那个节点。

关键点：
  · 每走一步先把当前节点压入路径，再与 target 比较 —— 先比较再压入会漏掉
    target 自己，p 是 q 的祖先时会退回上一层
  · 路径必须从根开始，否则公共前缀不完整
  · 两条路径从头比，遇到第一个不同的节点就停，答案取最后一个相同的节点
  · 题目保证节点存在，走空只可能出现在非法输入上

以示例 2（root = [6,2,8,0,4,7,9,null,null,3,5]，p = 2，q = 4）为例：
  到 2 的路径：[6, 2]（2 就是目标，压入后立刻停）
  到 4 的路径：[6, 2, 4]
  公共前缀 6、2，最后一个相同节点是 2 ✔
  （若压入时不包含 target，路径变成 [6] 和 [6,2]，答案会错成 6。）

复杂度分析：
  时间 O(h)：两条路径各下降一次，h 为树高（最坏 n）
  空间 O(h)：两个列表保存路径，最坏（斜树）O(n)

解法二：一次遍历，利用 BST 的比较性质直接找分叉点【最优解】

为什么慢：解法一存了两条完整路径，而 BST 上根本不需要知道路径长什么样：
当前节点的值就能告诉我们 p、q 在哪一侧。

换个角度：从根往下走，p、q 的走向只有三种可能：
  · 两个都小于当前值 → 二者都在左子树，LCA 在左边，向左走；
  · 两个都大于当前值 → 二者都在右子树，LCA 在右边，向右走；
  · 一大一小，或其中之一等于当前值 → 二者从这里分到两侧（或当前节点
    就是其中一个），当前节点就是 LCA，不能再往下走。

关键点：
  · 两个判断都要写成"严格小于 / 严格大于当前值"：只要有一个等于当前节点，
    当前节点就是答案，把等号放进下沉条件会多走一层，返回更深的节点
  · 必须拿 p、q 分别与当前节点比较，不能先比较 p 与 q 的大小再定方向 ——
    "p < q" 说明不了它们相对当前节点在左边还是右边
  · p、q 的先后顺序不可假设，p 完全可以比 q 大，两个条件都要同时成立
  · 走到 None 说明输入不合法；题目保证 p、q 存在，所以正常循环内必返回

以示例 2（root = [6,2,8,0,4,7,9,null,null,3,5]，p = 2，q = 4）为例：
  root = 6：2 < 6 且 4 < 6，向左走到 2；
  root = 2：p 等于当前值，不满足"两个都严格小于/大于"，返回 2 ✔

复杂度分析：
  时间 O(h)：每步只下降一层，h 为树高；平衡时 O(log n)，斜树 O(n)
  空间 O(1)：只用了一个游标指针，没有栈也没有列表
"""


class Solution1:
    def lowestCommonAncestor(self, root: "TreeNode", p: "TreeNode",
                             q: "TreeNode") -> "TreeNode":
        path_p = self._collect_path(root, p)
        path_q = self._collect_path(root, q)

        ans = None
        # 公共前缀一路相同，最后一个相同节点就是分叉前的位置
        for a, b in zip(path_p, path_q):
            if a is not b:
                break
            ans = a
        return ans

    def _collect_path(self, node: "TreeNode",
                      target: "TreeNode") -> list:
        # 路径包含 target 本身：否则"某节点是另一个的祖先"会被漏掉
        path = []
        while node is not None:
            path.append(node)
            if node is target:
                return path
            node = node.left if target.val < node.val else node.right
        return path


class Solution2:
    def lowestCommonAncestor(self, root: "TreeNode", p: "TreeNode",
                             q: "TreeNode") -> "TreeNode":
        cur = root
        while cur is not None:
            # 两者都严格小于当前值，说明都在左子树，分叉点不可能在这
            if p.val < cur.val and q.val < cur.val:
                cur = cur.left
            # 两者都严格大于当前值，说明都在右子树
            elif p.val > cur.val and q.val > cur.val:
                cur = cur.right
            else:
                # 分居两侧，或其中之一正是当前节点
                return cur
        # 题目保证 p、q 都在树上，只有非法输入才会走到这里
        return None


# ===== 测试辅助：按层序数组建树、按值找节点 =====
NULL_NODE = -(1 << 62)  # 节点值为 int，不会等于这个哨兵


def build_tree(level_order: list) -> Optional[TreeNode]:
    if not level_order or level_order[0] == NULL_NODE:
        return None

    root = TreeNode(level_order[0])
    nodes = [root]
    idx = 1
    i = 0
    while i < len(nodes) and idx < len(level_order):
        if level_order[idx] != NULL_NODE:
            nodes[i].left = TreeNode(level_order[idx])
            nodes.append(nodes[i].left)
        idx += 1
        if idx >= len(level_order):
            break
        if level_order[idx] != NULL_NODE:
            nodes[i].right = TreeNode(level_order[idx])
            nodes.append(nodes[i].right)
        idx += 1
        i += 1
    return root


def find_node(root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
    if root is None:
        return None
    if root.val == val:
        return root
    found = find_node(root.left, val)
    return found if found is not None else find_node(root.right, val)


def check(level_order: list, p_val: int, q_val: int, want: int) -> None:
    for cls in (Solution1, Solution2):
        root = build_tree(level_order)
        p = find_node(root, p_val)
        q = find_node(root, q_val)
        want_node = find_node(root, want)
        assert p is not None and q is not None and want_node is not None
        # 比对象身份而不是值：is 成立才证明返回的是那个节点
        assert cls().lowestCommonAncestor(root, p, q) is want_node


if __name__ == "__main__":
    sample = [6, 2, 8, 0, 4, 7, 9, NULL_NODE, NULL_NODE, 3, 5]

    # 示例 1：p、q 分居根的两侧
    check(sample, 2, 8, 6)

    # 示例 2：p 是 q 的祖先，答案必须是 p 自己（路径必须含 target）
    check(sample, 2, 4, 2)

    # 参数顺序无关：p 比 q 大
    check(sample, 8, 2, 6)
    check(sample, 5, 3, 4)

    # 边界：根就是其中一个节点
    check(sample, 6, 0, 6)

    # 反直觉：分叉点不在根，而在左边的 2 上
    check(sample, 0, 5, 2)

    # 反直觉：两个都在 2 的右子树里，分叉点是 4
    check(sample, 3, 5, 4)

    # 边界：右子树内部
    check(sample, 7, 9, 8)

    # 边界：最小规模，p、q 是根的两个孩子
    check([2, 1, 3], 1, 3, 2)

    # 边界：最小规模，p 是根
    check([2, 1, 3], 2, 1, 2)

    # 边界：200 层右斜链，值 1..200
    chain = [1]
    for i in range(2, 201):
        chain.extend([NULL_NODE, i])
    check(chain, 1, 200, 1)
    check(chain, 150, 200, 150)
    check(chain, 120, 130, 120)

    print("all tests passed")
