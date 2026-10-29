r"""
============================================================================
LCR 174 / 剑指 Offer 54. 寻找二叉搜索树中的目标节点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/
分类：二叉搜索树 · 反序中序遍历

某公司组织架构以二叉搜索树形式记录，节点值为处于该职位的员工编号。
请返回第 cnt 大的员工编号。

示例 1：
输入：root = [7, 3, 9, 1, 5], cnt = 2
       7
      / \
     3   9
    / \
   1   5
输出：7

示例 2：
输入: root = [10, 5, 15, 2, 7, null, 20, 1, null, 6, 8], cnt = 4
       10
      /  \
     5    15
    / \     \
   2   7     20
  /   / \
 1   6   8
输出：8

提示：
- 1 ≤ cnt ≤ 二叉搜索树元素个数
"""
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None, right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法一：反序中序遍历（右 → 根 → 左）【最优解】

为什么慢：最容易想到的是"中序遍历把整棵树存成升序数组，再取倒数第 cnt 个"，
时间是 O(n) 但要多开 O(n) 的数组；如果先求"总节点数 n，再找第 n-cnt+1 小"，
还得额外遍历一次。

换个角度：BST 的中序是升序，"右 → 根 → 左"的遍历就是降序。按降序数到
第 cnt 个就是答案，数到就可以停，不必访问剩余节点。
  · 递归时先走右子树，再处理当前节点，最后走左子树；
  · 每处理一个节点就 `cnt -= 1`，减到 0 时记下答案并停止。

关键点：
  · 顺序是"右、根、左"，写成"左、根、右"就变成第 cnt 小了
  · 计数必须在访问当前节点时做，不能在进入函数时做
  · 找到答案后要能及时收手：`cnt == 0` 时直接 return，
    否则会继续遍历把 cnt 减成负数
  · Python 用 nonlocal 更新外层 ans / cnt
  · 题目保证 1 ≤ cnt ≤ 节点数，因此一定能找到答案

以示例 1（root = [7,3,9,1,5]，cnt = 2）为例：
  右子树 9 → 访问 9，cnt 变 1；
  回到根 7 → 访问 7，cnt 变 0 → 答案是 7 ✔

复杂度分析：
  时间 O(n)：最坏要走到最小节点；平均只需访问前 cnt 大的一小部分
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：迭代中序（右根左）【非最优】

把解法一改成显式栈：先一路向右压栈，弹出时计数，再转向左子树。
  · 这样能避免递归，思路与解法一完全一致；
  · 代码稍长，好处是不受递归深度限制（虽然本题数据量也用不上）。

关键点：
  · 压栈方向是"向右"，弹栈时才是"访问"，别把方向写反
  · 计数到 0 即刻 return，栈里剩下的节点不用再管
  · 与解法一相同，顺序必须是右 → 根 → 左

复杂度分析：
  时间 O(n)
  空间 O(h)：栈保存当前路径
"""


class Solution1:
    def findTargetNode(self, root: Optional[TreeNode], cnt: int) -> int:
        ans = 0
        remain = cnt

        def dfs(node: Optional[TreeNode]) -> None:
            nonlocal ans, remain
            if node is None or remain == 0:
                return

            dfs(node.right)

            remain -= 1
            if remain == 0:
                ans = node.val
                return

            dfs(node.left)

        dfs(root)
        return ans


class Solution2:
    def findTargetNode(self, root: Optional[TreeNode], cnt: int) -> int:
        stk = []
        node = root

        while node is not None or stk:
            while node is not None:
                stk.append(node)
                node = node.right
            node = stk.pop()

            cnt -= 1
            if cnt == 0:
                return node.val

            node = node.left
        return -1  # 题目保证不会走到这里


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


def check(level_order: list, cnt: int, want: int) -> None:
    assert Solution1().findTargetNode(build_tree(level_order), cnt) == want
    assert Solution2().findTargetNode(build_tree(level_order), cnt) == want


if __name__ == "__main__":
    # 示例 1
    check([7, 3, 9, 1, 5], 2, 7)

    # 示例 2
    check([10, 5, 15, 2, 7, None, 20, 1, None, 6, 8], 4, 8)

    # 边界：单节点
    check([1], 1, 1)

    # 剑指 Offer 原示例 1：root = [3,1,4,null,2], k = 1
    check([3, 1, 4, None, 2], 1, 4)

    # 剑指 Offer 原示例 2：root = [5,3,6,2,4,null,null,1], k = 3
    check([5, 3, 6, 2, 4, None, None, 1], 3, 4)

    # 边界：取最小值（cnt = 节点总数）
    check([5, 3, 6, 2, 4, None, None, 1], 6, 1)

    # 边界：cnt = 1 就是最大值
    check([5, 3, 6, 2, 4, None, None, 1], 1, 6)

    # 边界：只有右孩子的 BST，降序就是 3、2、1
    check([1, None, 2, None, 3], 2, 2)

    # 边界：只有左孩子的 BST，降序就是 3、2、1
    check([3, 2, None, 1], 3, 1)

    # 边界：100 层右斜链，值 1..100
    chain = [1]
    for i in range(2, 101):
        chain.extend([None, i])
    check(chain, 1, 100)
    check(chain, 100, 1)
    check(chain, 50, 51)

    print("all tests passed")
