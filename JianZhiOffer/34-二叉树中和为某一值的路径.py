"""
============================================================================
LCR 153 / 剑指 Offer 34. 二叉树中和为某一值的路径（中等）
来源：剑指 Offer（第 2 版）
链接：https://leetcode.cn/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/
分类：二叉树 · DFS 回溯（根到叶子路径）

给你二叉树的根节点 root 和一个整数目标和 targetSum ，找出所有 从根节点到
叶子节点 路径总和等于给定目标和的路径。

叶子节点 是指没有子节点的节点。

示例 1：
输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
输出：[[5,4,11,2],[5,8,4,5]]

示例 2：
输入：root = [1,2,3], targetSum = 5
输出：[]

示例 3：
输入：root = [1,2], targetSum = 0
输出：[]

提示：
- 树中节点总数在范围 [0, 5000] 内
- -1000 <= Node.val <= 1000
- -1000 <= targetSum <= 1000
"""
from typing import List, Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None,
                 right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法：DFS 回溯（进入压栈，离开弹栈）【最优解】

为什么慢：要枚举所有根到叶子的路径，路径条数最坏是 O(n)（叶子数），每条
路径长 O(h)。若每到一个叶子都从头重建一次路径，或者每层都把 path 拷贝一份
往下传，重复付出的代价会到 O(n*h)；而答案本身最多也只有 O(n*h) 个元素。

换个角度：所有根到叶子的路径共享前缀。维护一条 path，进入节点时压入、
离开时弹出，它自然就等于"当前根到节点的链"。命中时只拷贝一次，
没命中就原地复用，前缀部分不再重复构造。

关键点：
  · 弹出 path 末尾元素必须做，且放在左右递归之后 —— 漏掉它，兄弟分支
    会带着上一分支的尾巴，路径越积越长
  · 只在该节点是叶子（左右都空）且剩余和恰为 0 时收集；在非叶子处收集，
    会把"根到中间节点"误当成一条完整路径
  · 剩余和用 targetSum 沿途递减，避免每次重新求和
  · 值可以为负，remain < 0 不能剪枝，remain == 0 也不能提前返回 ——
    后面接着负数仍可能把和拉回 0
  · 收集时用 path.copy()，不能存 path 本身，否则后续增删会改掉已存答案
  · path 是外层列表，闭包里用 append/pop 原地修改，不需要 nonlocal

手推一遍：以示例 1（root = [5,4,8,11,null,13,4,7,2,null,null,5,1]，
targetSum = 22）为例：
  5 → 4 → 11 → 7，remain = -5，是叶子但归不了零，弹出 7
  回到 11 → 2，remain = 0，是叶子 → 收集 [5,4,11,2]，再弹出 2、11、4
  5 → 8 → 13，remain = -4，是叶子，弹出
  回到 8 → 4 → 5，remain = 0，是叶子 → 收集 [5,8,4,5]
  回到根，path 清空 → 共两条 ✔

复杂度分析：
  时间 O(n^2)：节点进出各一次是 O(n)，但每次命中要拷贝长度 O(h) 的路径，
    最坏（斜链且处处命中）为 O(n^2)，输出本身也有这么多元素
  空间 O(h)：path 与递归栈都只保存当前一条根到叶子的链，最坏 O(n)
"""


class Solution:
    def pathSum(self, root: Optional[TreeNode],
                targetSum: int) -> List[List[int]]:
        path = []
        ans = []

        def dfs(node: Optional[TreeNode], remain: int) -> None:
            if node is None:
                return

            path.append(node.val)
            remain -= node.val

            # 叶子且差值归零，才是一条符合题意的路径
            if node.left is None and node.right is None and remain == 0:
                ans.append(path.copy())

            # 负值存在，两条分支都必须走，不能按 remain 剪枝
            dfs(node.left, remain)
            dfs(node.right, remain)

            # 撤销压栈：兄弟分支不能继承这条路径的尾巴
            path.pop()

        dfs(root, targetSum)
        return ans


# ===== 测试辅助：按层序数组（None 表示空）建树、路径归一化 =====
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


def normalize(paths: list) -> list:
    # 题面没有约定路径顺序，统一排序后再比较
    return sorted(sorted(p) for p in paths)


def check(level_order: list, target_sum: int, want: list) -> None:
    got = Solution().pathSum(build_tree(level_order), target_sum)
    assert normalize(got) == normalize(want), \
        f"pathSum({level_order}, {target_sum}) = {got}, want {want}"


if __name__ == "__main__":
    # 示例 1
    check([5, 4, 8, 11, None, 13, 4, 7, 2, None, None, 5, 1], 22,
          [[5, 4, 11, 2], [5, 8, 4, 5]])

    # 示例 2
    check([1, 2, 3], 5, [])

    # 示例 3：目标和为 0，但两条路径分别是 1 和 3，都不等于 0
    check([1, 2], 0, [])

    # 边界：空树
    check([], 0, [])

    # 反直觉：单个 0 既是叶子又等于目标和，应收集一条路径
    check([0], 0, [[0]])

    # 反直觉：正负抵消，1 + (-1) = 0，路径跨两层
    check([1, -1], 0, [[1, -1]])

    # 反直觉：全负值，目标和也为负
    check([-2, None, -3], -5, [[-2, -3]])

    # 关键点：根节点和已等于 1，但它不是叶子，不能收集
    check([1, 2, 2], 1, [])

    # 关键点：remain 中途归零后还要继续走（后续节点是负数）
    check([1, -1, -1], 0, [[1, -1], [1, -1]])

    # 关键点：左分支结束后必须弹干净，右分支路径不能带左分支的值
    check([1, 2, 3, 4], 7, [[1, 2, 4]])

    # 边界：200 层斜链全为 1，目标和 200 只有一整条路径
    chain = []
    for i in range(1, 201):
        chain.append(1)
        if i < 200:
            chain.append(None)
    check(chain, 200, [[1] * 200])

    print("all tests passed")
