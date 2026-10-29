"""
============================================================================
337. 打家劫舍 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/house-robber-iii/
分类：动态规划 · 树形 DP

小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为 root 。

除了 root 之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果 两个直接相连的房子在同一天晚上被打劫 ，房屋将自动报警。

给定二叉树的 root 。返回 在不触动警报的情况下 ，小偷能够盗取的最高金额 。

示例 1:
输入: root = [3,2,3,null,3,null,1]
输出: 7 
解释: 小偷一晚能够盗取的最高金额 3 + 3 + 1 = 7

示例 2:
输入: root = [3,4,5,1,3,null,1]
输出: 9
解释: 小偷一晚能够盗取的最高金额 4 + 5 = 9

提示：
- 树的节点数在 [1, 10^4] 范围内
- 0 <= Node.val <= 10^4
"""
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None, right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法一：树上 DP，后序一次返回 (选, 不选)【最优解】

为什么慢：把数组上的递推照搬到树上，最直白的式子是对每个节点比较
"选它 + 孙子们的最优" 和 "两个孩子的最优"。但"两个孩子的最优"本身又要
递归求孙子，同一个子树会在两条分支里各展开一次，节点被重复进入的次数
随深度指数增长。

换个角度：每个节点只需要向父节点汇报两个数——以它为根的子树里，
"选它"的最大值与"不选它"的最大值。父节点拿到孩子的这两个数就能自己
做决定，完全不需要知道子树内部怎么选的。于是后序一次遍历即可。

关键点：
  · 选 x 时孩子只能"不选"：take(x) = x.val + skip(左) + skip(右)
  · 不选 x 时孩子选不选都行：skip(x) = max(take(左), skip(左))
    + max(take(右), skip(右))
  · 返回二元组的两个分量顺序必须在整棵树里统一，父子两侧写反是这类题
    最常见的错
  · 空节点返回 (0, 0)：缺失的孩子既不能被选，也不会贡献金额
  · 答案是根节点两个值的 max，不能只看"选根"那一个
  · Python 直接返回二元组，解包比下标访问更不容易写反

以示例 1（root = [3,2,3,null,3,null,1]）为例，从叶子往上：
  val=3 的叶子： (3, 0)
  左孩子 2（右孩子是上面那个 3）：take = 2 + 0 + 0 = 2，
    skip = max(3,0) + 0 = 3 → (2, 3)
  右孩子 3（右孩子是 val=1 的叶子）：take = 3 + 0 + 0 = 3，
    skip = 0 + max(1,0) = 1 → (3, 1)
  根 3：take = 3 + skip(左)=3 + skip(右)=1 = 7；
    skip = max(2,3) + max(3,1) = 6 → max(7,6) = 7 ✔

复杂度分析：
  时间 O(n)：每个节点只被后序访问一次，汇报两个整数
  空间 O(h)：递归栈等于树高，斜树最坏 O(n)，平衡树 O(log n)
"""


class Solution1:
    def rob(self, root: Optional[TreeNode]) -> int:
        take, skip = self._dfs(root)
        return max(take, skip)

    # 返回 (选当前节点的最大值, 不选当前节点的最大值)
    def _dfs(self, node: Optional[TreeNode]) -> tuple:
        if node is None:
            return 0, 0

        left_take, left_skip = self._dfs(node.left)
        right_take, right_skip = self._dfs(node.right)

        # 选当前节点：两个孩子都只能跳过
        take = node.val + left_skip + right_skip
        # 不选当前节点：每个孩子都可以在"选"与"不选"之间取较优
        skip = max(left_take, left_skip) + max(right_take, right_skip)
        return take, skip


"""
解法二：记忆化递归，按节点缓存它的最优值【非最优】

为什么慢：解法一的式子在"不选 x"时要取孩子的 max，"选 x"时要取孙子的值，
如果直接递归而不缓存，同一个节点的最优值会被它的父、祖父、曾祖父反复
求解，深度为 h 的链上退化成指数级。

换个角度：把"以 x 为根的子树的最优值"当成唯一状态，算过就存。于是与解法
一一样每节点只算一次，但缓存的是"整棵子树的最优值"这一个数。

关键点：
  · 缓存键必须是节点对象本身，不能用 val —— 值相同的节点是不同子树
  · 进入函数要立刻查表，查表放在递归调用之后等于没缓存
  · 取孙子时要判空：node.left 本身可能为 None，再取 .left 会抛异常

复杂度分析：
  时间 O(n)：每个节点的最优值只计算一次
  空间 O(n)：字典给每个节点存一个整数，比解法一多一个数量级；
        递归栈 O(h) 相对次要
"""


class Solution2:
    def rob(self, root: Optional[TreeNode]) -> int:
        # 键是节点对象：同一棵树里对象身份唯一，值相同也不冲突
        memo = {}

        def best(node: Optional[TreeNode]) -> int:
            if node is None:
                return 0
            if node in memo:
                return memo[node]

            # 不选当前节点
            skip = best(node.left) + best(node.right)
            # 选当前节点：只能再从孙子们身上取
            take = node.val
            if node.left is not None:
                take += best(node.left.left) + best(node.left.right)
            if node.right is not None:
                take += best(node.right.left) + best(node.right.right)

            memo[node] = max(take, skip)
            return memo[node]

        return best(root)


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
    assert Solution1().rob(build_tree(level_order)) == want
    assert Solution2().rob(build_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1：3 + 3 + 1，根与右孩子、右孩子的右孩子互不相邻
    check([3, 2, 3, None, 3, None, 1], 7)

    # 示例 2：4 + 5 优于 3 + 1 + 3 + 1
    check([3, 4, 5, 1, 3, None, 1], 9)

    # 边界：单节点，必选
    check([1], 1)

    # 边界：值为 0 的节点，答案是 0 而不是 -1
    check([0], 0)

    # 关键点用例：隔代能同选，根 5 与两个孙子 4、6 一起拿共 15
    check([5, 1, None, 4, 6], 15)

    # 反直觉：3 与 4 分属不同分支，中间隔着一层，3+4=7 比 2+4=6 更好
    check([2, 1, 3, None, 4], 7)

    # 关键点用例：全零树，任何选法都是 0
    check([0, 0, 0, 0, 0], 0)

    # 边界：右斜链 200 个节点，等价于 198 处理 [1..200]，偶数位之和更大
    level_order = []
    for i in range(1, 201):
        level_order.extend([i, None])
    check(level_order, 10100)

    print("all tests passed")
