"""
============================================================================
145. 二叉树的后序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-postorder-traversal/
分类：二叉树 · 遍历（递归 / 栈 / Morris）

给你一棵二叉树的根节点 root ，返回其节点值的 后序遍历 。

示例 1：
输入：root = [1,null,2,3]
输出：[3,2,1]

示例 2：
输入：root = [1,2,3,4,5,null,8,null,null,6,7,9]
输出：[4,6,7,5,2,9,8,3,1]

示例 3：
输入：root = []
输出：[]

示例 4：
输入：root = [1]
输出：[1]

提示：
树中节点的数目在范围 [0, 100] 内
-100 <= Node.val <= 100

进阶：递归算法很简单，你可以通过迭代算法完成吗？
"""
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None, right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法一：递归【非最优】

官方方法一。后序遍历的定义是「左 → 右 → 根」，直接按定义写递归：
  · 空节点返回；
  · 先递归左子树、再递归右子树，最后把当前节点值加入答案。

关键点：
  · 与前序/中序只差「访问语句的位置」：后序放在两个递归之后
  · 后序的典型用途：「自底向上」汇总信息（子树深度、子树和、是否平衡等），
    因为处理自己时孩子的结果都已经拿到了
  · 递归深度 = 树高，最坏 O(n)；Python 默认递归上限 1000，本题 n ≤ 100 安全

复杂度分析：
  时间 O(n)：每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高，平均 O(log n)、最坏 O(n)

解法二：迭代（辅助栈 + prev 指针）【非最优】

官方方法二。后序比前序、中序都难写：访问一个节点之前，必须确认「右子树已经处理完」。
官方用 prev 记录「上一个被输出的节点」：
  · 一路向左压栈，直到空；
  · 取栈顶（先不弹出），若它没有右孩子、或右孩子就是 prev（说明右子树刚处理完），
    才输出它，并置 prev、把当前指针置空（表示这一支结束）；
  · 否则说明右子树还没处理，把它重新压栈，转去处理右子树。

关键点：
  · 判定条件必须是 `node.right is None or node.right is prev`，两个分支缺一不可
  · 判断完「还不能访问」时要把节点重新压回去，这是与中序最大的不同
  · prev 的作用是区分「右孩子没处理」和「右孩子处理完了」，只看 right 是否为空不够
  · 每个节点会进栈两次（第一次为了左子树，第二次为了右子树）

复杂度分析：
  时间 O(n)
  空间 O(h)：显式栈

解法三：Morris 遍历 + 逆序输出右脊【最优解】

官方方法三。后序的 Morris 与中序共用「线索」骨架，但访问时机变成「拆除线索时」，
并且不是逐个输出，而是把「当前节点左子树的最右一条链（右脊）」整体倒序输出：
  · 若当前节点 p1 有左孩子：找它的中序前驱 p2（左子树最右节点）。
      - p2.right 为空 → 建立线索 p2.right = p1，p1 走向左孩子；
      - p2.right 是 p1 → 拆线索，并把「p1 的左孩子到 p2 这条右链」倒序加到答案里，
        然后 p1 走向右孩子。
  · 若没有左孩子：p1 直接走向右孩子（注意这里不输出）。
  · 主循环结束后，还要把「从根开始的最右一条链」倒序输出一次，收尾。

关键点：
  · add_path 的作用：把一条只往右走的链收集起来再反转，正好等价于后序里
    「先左、再右、最后根」的顺序
  · 主循环里不输出任何单个节点，全部靠 add_path 批量输出，这是后序 Morris 与
    前序/中序 Morris 最本质的区别
  · 最后那次 add_path(root) 不能漏，否则根节点和整条最右链都会丢
  · 线索同样必须拆除，保证不改动原树结构

以 root = [1,null,2,3] 为例（1 的右孩子 2，2 的左孩子 3）：
  递归：左(空) → 右子树：3 的左(空) → 右(空) → 3；然后 2；最后 1 → [3,2,1]
  迭代：同递归的展开顺序，prev 保证 2 在 3 之后输出
  Morris：1 无左孩子 → 走向 2；2 有左孩子 3，3 无右孩子 → 建立线索 3.right=2，
          走向 3；3 无左孩子 → 走向右孩子（线索 2）→ 2 的前驱 p2 = 3 已指向 2 →
          拆线索，倒序输出链 3（即 [3]），走向 2 的右孩子（空）；
          循环结束，add_path(root=1)：链 1 -> 2 倒序 → [2,1]
          合并得到 [3,2,1]

复杂度分析：
  时间 O(n)：每个节点至多被访问两次，add_path 的每条边也只走一次
  空间 O(1)：只用常数个指针（答案数组不计）
"""


class Solution:
    # ---------- 解法一：递归 ----------
    def postorderTraversal_recursive(self, root: Optional[TreeNode]) -> list:
        res = []

        def dfs(node: Optional[TreeNode]) -> None:
            if node is None:
                return
            dfs(node.left)
            dfs(node.right)
            res.append(node.val)

        dfs(root)
        return res

    # ---------- 解法二：迭代（栈 + prev） ----------
    def postorderTraversal_iterative(self, root: Optional[TreeNode]) -> list:
        res = []
        if root is None:
            return res

        stk = []
        prev = None
        while root is not None or stk:
            # 一路向左压栈
            while root is not None:
                stk.append(root)
                root = root.left
            root = stk.pop()
            # 右子树为空或已处理完，才输出当前节点
            if root.right is None or root.right is prev:
                res.append(root.val)
                prev = root
                root = None  # 这一支结束，下一轮从栈里取
            else:
                # 右子树还没处理，把当前节点压回去，转去处理右子树
                stk.append(root)
                root = root.right
        return res

    # ---------- 解法三：Morris 遍历 + 逆序输出右脊（最优解） ----------
    @staticmethod
    def add_path(res: list, node: Optional[TreeNode]) -> None:
        """把从 node 开始一直往右的链收集起来并倒序追加到 res"""
        start = len(res)
        while node is not None:
            res.append(node.val)
            node = node.right
        res[start:] = res[start:][::-1]

    def postorderTraversal(self, root: Optional[TreeNode]) -> list:
        res = []
        if root is None:
            return res

        p1 = root
        while p1 is not None:
            p2 = p1.left
            if p2 is not None:
                # 找到 p1 的中序前驱：左子树的最右节点
                while p2.right is not None and p2.right is not p1:
                    p2 = p2.right

                if p2.right is None:
                    # 第一次到达：建立线索，转向左子树
                    p2.right = p1
                    p1 = p1.left
                    continue
                # 第二次到达：拆线索，并把左子树的右脊倒序输出
                p2.right = None
                self.add_path(res, p1.left)
            p1 = p1.right

        # 收尾：根节点所在的最右链
        self.add_path(res, root)
        return res


# ===== 测试辅助：按层序数组建树（None 表示空） =====
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
    solution = Solution()
    # 每跑一种解法都重新建树：Morris 会临时改动指针
    assert solution.postorderTraversal_recursive(build_tree(level_order)) == want
    assert solution.postorderTraversal_iterative(build_tree(level_order)) == want
    assert solution.postorderTraversal(build_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([1, None, 2, 3], [3, 2, 1])

    # 示例 2
    check([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9], [4, 6, 7, 5, 2, 9, 8, 3, 1])

    # 示例 3：空树
    check([], [])

    # 示例 4：单节点
    check([1], [1])

    # 边界：完全二叉树
    check([1, 2, 3, 4, 5, 6, 7], [4, 5, 2, 6, 7, 3, 1])

    # 边界：只有左子树链 / 只有右子树链
    check([3, 2, None, 1], [1, 2, 3])
    check([1, None, 2, None, 3], [3, 2, 1])

    # 边界：负数值（根 -100，左 -50（其左 0），右 100）
    check([-100, -50, 100, 0], [0, -50, 100, -100])

    # 边界：深层右斜链（深度 100，后序是降序）
    right = []
    for i in range(1, 101):
        right.extend([i, None])
    check(right, list(range(100, 0, -1)))

    # 边界：深层左斜链（深度 100，后序是升序）
    left = [100]
    for i in range(99, 0, -1):
        left.extend([i, None])
    check(left, list(range(1, 101)))

    print("all tests passed")
