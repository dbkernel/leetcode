"""
============================================================================
589. N 叉树的前序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-preorder-traversal/
分类：N 叉树 · 遍历（递归 / 栈）

给定一个 n 叉树的根节点 root ，返回 其节点值的 前序遍历 。

n 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔
（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[1,3,5,6,2,4]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[1,2,3,6,7,11,14,4,8,12,5,9,13,10]

提示：
- 节点总数在范围 [0, 10^4]内
- 0 <= Node.val <= 10^4
- n 叉树的高度小于或等于 1000

进阶：递归法很简单，你可以使用迭代法完成此题吗?
"""
from typing import List


# 力扣预置的节点定义（多叉树，注意孩子是一个列表）
class Node:
    def __init__(self, val: int = None, children: "List[Node]" = None):
        self.val = val
        self.children = children


"""
解法一：递归【最优解】

官方方法一。N 叉树的前序遍历与二叉树完全一致，只是「左右两个孩子」换成了
「children 数组」：
  · 先访问当前节点；
  · 再按从左到右的顺序依次递归每个孩子。

为什么它是最优解：
  · 时间 O(m)，每个节点只访问一次，不可能更快；
  · 辅助空间只有递归栈 O(h)，不需要额外哈希表或「待处理兄弟」缓存，
    因此在空间上优于下面两种迭代写法。

关键点：
  · 访问语句必须在 for 循环之前，否则就变成后序了
  · children 为空时 for 循环自然跳过，不需要单独判空
  · 递归深度 = 树高 h，本题 h ≤ 1000，接近 Python 默认上限 1000；
    本地自测把深度压到 200，避免误触 RecursionError

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  访问 1 → 依次进孩子 3、2、4：
  访问 3 → 孩子 5、6：访问 5，访问 6；回到 2：访问 2；回到 4：访问 4
  得到 [1,3,5,6,2,4]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：迭代（显式栈 + 哈希表记录孩子下标）【非最优】

官方方法二。直接模拟递归：递归时「返回到父节点后该处理第几个孩子」这件事
由调用栈自动记住，手写栈就必须自己记，于是用 dict 存每个节点已处理到的下标。
  · 一路沿着第一个孩子向下，边下边访问、边把节点压栈，并在表中记 0；
  · 走到空后看栈顶 p：取它下一个未访问孩子的下标 index；
      - index 有效 → 更新表中下标，转到该孩子继续向下；
      - index 无效 → 说明 p 的孩子都处理完了，弹出 p、删掉表项，回到 p 的父亲。

关键点：
  · 「先访问再往下」决定了这是前序；如果改成出栈时才访问就变成后序
  · 表项要在节点弹出时删除，否则同一节点再次入栈会读到旧下标
  · 叶子的表项不存在，用 -1 兜底：加一后为 0，恰好判定为「已无孩子」
  · 额外的 dict 让常数和空间都变差，所以不是最优

复杂度分析：
  时间 O(m)
  空间 O(h)：栈与 dict 都只保存当前路径上的节点

解法三：迭代（逆序压栈，最简写法）【非最优】

官方方法三。前序要求「根 → 孩子从左到右」，而栈是后进先出：
  · 先弹出并访问当前节点；
  · 再把孩子「从右到左」压栈，这样左孩子最后入栈、最先出栈。

关键点：
  · 压栈必须逆序，正序压栈会让最右孩子先被访问，顺序就反了
  · 不需要 dict，也不需要「当前节点」指针，循环体只有三步
  · 空间最坏是 O(m)：星形树（根有 m-1 个孩子）时栈里同时存下所有兄弟节点
  · 与解法二同为 O(m) 时间，但写法短得多，是迭代法里的首选

复杂度分析：
  时间 O(m)
  空间 O(m)：最坏为星形树，栈同时保存 O(m) 个兄弟节点

-------- 三种写法对比 --------
  递归：时间 O(m)、空间 O(h)，最省空间，标记为【最优解】
  解法二：时间 O(m)、空间 O(h) + dict，常数最大，仅作为「模拟递归」的示范
  解法三：时间 O(m)、空间 O(m)，代码最短，迭代时优先选它
  本题 N 叉树没有 Morris 那样的 O(1) 空间遍历写法。
"""


class Solution:
    # ---------- 解法一：递归（最优解） ----------
    def preorder(self, root: "Node") -> List[int]:
        res = []

        def dfs(node: "Node") -> None:
            if node is None:
                return
            res.append(node.val)
            for child in node.children:
                dfs(child)

        dfs(root)
        return res

    # ---------- 解法二：迭代 + dict 记录孩子下标 ----------
    def preorderIterativeWithIndex(self, root: "Node") -> List[int]:
        res = []
        if root is None:
            return res

        next_index = {}
        stk = []
        node = root
        while stk or node is not None:
            # 沿着第一个孩子一路向下，同时访问沿途节点
            while node is not None:
                res.append(node.val)
                stk.append(node)
                if node.children:
                    next_index[node] = 0
                    node = node.children[0]
                else:
                    node = None
            node = stk[-1]
            # 叶子没有表项，-1 + 1 = 0，恰好越界
            index = next_index.get(node, -1) + 1
            if index < len(node.children):
                next_index[node] = index
                node = node.children[index]
            else:
                stk.pop()
                # 叶子本来就没有表项，pop 比 del 安全
                next_index.pop(node, None)
                node = None
        return res

    # ---------- 解法三：迭代 + 逆序压栈 ----------
    def preorderIterativeReverse(self, root: "Node") -> List[int]:
        res = []
        if root is None:
            return res

        stk = [root]
        while stk:
            node = stk.pop()
            res.append(node.val)
            # 逆序压栈：左孩子最后入栈，从而最先出栈
            for child in reversed(node.children):
                stk.append(child)
        return res


# ===== 测试辅助：按层序数组建 N 叉树（None 表示空） =====
def build_nary_tree(level_order: list) -> "Node":
    if not level_order or level_order[0] is None:
        return None

    root = Node(level_order[0], [])
    nodes = [root]
    idx = 2  # 跳过根与它后面的 None
    p = 0
    # 层序序列规则：根之后先来一个 None，然后是该节点的全部孩子；
    # 每个孩子按 BFS 顺序继续展开，孩子列表以 None 结尾。
    while idx < len(level_order) and p < len(nodes):
        parent = nodes[p]
        p += 1
        while idx < len(level_order) and level_order[idx] is not None:
            child = Node(level_order[idx], [])
            parent.children.append(child)
            nodes.append(child)
            idx += 1
        idx += 1  # 跳过孩子列表的 None 结束符
    return root


def check(level_order: list, want: list) -> None:
    solution = Solution()
    assert solution.preorder(build_nary_tree(level_order)) == want
    assert solution.preorderIterativeWithIndex(build_nary_tree(level_order)) == want
    assert solution.preorderIterativeReverse(build_nary_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([1, None, 3, 2, 4, None, 5, 6], [1, 3, 5, 6, 2, 4])

    # 示例 2
    check([1, None, 2, 3, 4, 5, None, None, 6, 7, None, 8, None, 9, 10,
           None, None, 11, None, 12, None, 13, None, None, 14],
          [1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10])

    # 边界：空树
    check([], [])

    # 边界：单节点
    check([1], [1])

    # 边界：根节点没有孩子
    check([1, None], [1])

    # 边界：叶子节点顺序 —— 根的 3 个孩子都没有孩子
    check([1, None, 2, 3, 4], [1, 2, 3, 4])

    # 边界：星形树，验证逆序压栈有没有把兄弟顺序写反
    check([1, None, 2, 3, 4, 5, 6], [1, 2, 3, 4, 5, 6])

    # 边界：孩子为空要用 None 占位（2 没有孩子，3 有孩子 4）
    check([1, None, 2, 3, None, None, 4], [1, 2, 3, 4])

    # 边界：深链（每个节点只有一个孩子），深度 200
    level_order = []
    for i in range(1, 201):
        level_order.extend([i, None])
    check(level_order, list(range(1, 201)))

    print("all tests passed")
