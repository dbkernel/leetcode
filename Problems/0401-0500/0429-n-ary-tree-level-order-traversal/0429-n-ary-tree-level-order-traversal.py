"""
============================================================================
429. N 叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-level-order-traversal/
分类：N 叉树 · 层序遍历（BFS / 队列）

给定一个 N 叉树，返回其节点值的层序遍历。（即从左到右，逐层遍历）。

树的序列化输入是用层序遍历，每组子节点都由 null 值分隔（参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[[1],[3,2,4],[5,6]]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]

提示：
- 树的高度不会超过 1000
- 树的节点总数在 [0, 10^4] 之间
"""
from collections import deque
from typing import List, Optional


# 力扣预置的节点定义（多叉树，注意孩子是一个列表）
class Node:
    def __init__(self, val: int = None, children: "List[Node]" = None):
        self.val = val
        self.children = children


"""
解法：BFS + 按层定长出队【最优解】

为什么慢：层序遍历要求「同层从左到右、不同层分开」。如果只用一个普通队列
逐个弹出，虽然节点顺序是对的，但不知道哪几个属于同一层；若给每个节点挂
层号再按层号分组，就要多存 O(n) 个层号。

换个角度：BFS 队列在某一时刻恰好是「当前层剩余节点 + 下一层已入队节点」。
每轮开始先记下 cnt = len(q)，再连续弹出 cnt 个节点，这批就是完整的一层；
而这 cnt 个节点入队的孩子，恰好全部是下一层，并且按父节点的顺序排好。

关键点：
  · cnt 必须在 for 循环之前取，循环中队列还会增长
  · 每轮新建 level 再 append 到答案，避免各层共用同一个列表
  · N 叉树没有「先左后右」，只要按 children 的下标顺序入队，层内自然有序
  · 空树要单独返回：不要入队再靠循环兜底，否则会多出一个空层
  · 用 collections.deque：popleft 是 O(1)，list 的 pop(0) 是 O(n)

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  第 1 轮 cnt=1 → 出 1，入孩子 3、2、4 → [[1]]
  第 2 轮 cnt=3 → 出 3（入 5、6）、2、4 → [[1],[3,2,4]]
  第 3 轮 cnt=2 → 出 5、6 → [[1],[3,2,4],[5,6]]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案外，队列最多同时存两层节点
"""


class Solution:
    def levelOrder(self, root: Optional["Node"]) -> List[List[int]]:
        ans = []
        if root is None:
            return ans

        q = deque([root])
        while q:
            # 本层节点个数必须在出队前锁定
            cnt = len(q)
            level = []
            for _ in range(cnt):
                cur = q.popleft()
                level.append(cur.val)
                q.extend(cur.children)
            ans.append(level)
        return ans


# ===== 测试辅助：按层序数组建 N 叉树（None 表示空） =====
def build_nary_tree(level_order: list) -> Optional["Node"]:
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
    assert Solution().levelOrder(build_nary_tree(level_order)) == want


if __name__ == "__main__":
    # 示例 1
    check([1, None, 3, 2, 4, None, 5, 6], [[1], [3, 2, 4], [5, 6]])

    # 示例 2
    check([1, None, 2, 3, 4, 5, None, None, 6, 7, None, 8, None, 9, 10,
           None, None, 11, None, 12, None, 13, None, None, 14],
          [[1], [2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13], [14]])

    # 边界：空树
    check([], [])

    # 边界：单节点
    check([1], [[1]])

    # 边界：根节点没有孩子
    check([1, None], [[1]])

    # 边界：星形树，第二层顺序就是孩子数组顺序
    check([1, None, 2, 3, 4, 5, 6], [[1], [2, 3, 4, 5, 6]])

    # 边界：某层节点有的有孩子、有的没有（None 占位）
    check([1, None, 2, 3, None, None, 4], [[1], [2, 3], [4]])

    # 边界：深链（每层一个节点），深度 200
    level_order = []
    want = []
    for i in range(1, 201):
        level_order.extend([i, None])
        want.append([i])
    check(level_order, want)

    print("all tests passed")
