"""
============================================================================
106. 从中序与后序遍历序列构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
分类：二叉树 · 分治构造（中序 + 后序）

给定两个整数数组 inorder 和 postorder ，其中 inorder 是二叉树的中序遍历，
postorder 是同一棵树的后序遍历，请你构造并返回这颗 二叉树 。

示例 1：
输入：inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
输出：[3,9,20,null,null,15,7]

示例 2：
输入：inorder = [-1], postorder = [-1]
输出：[-1]

提示：
- 1 <= inorder.length <= 3000
- postorder.length == inorder.length
- -3000 <= inorder[i], postorder[i] <= 3000
- inorder 和 postorder 都由 不同 的值组成
- postorder 中每一个值都在 inorder 中
- inorder 保证是树的中序遍历
- postorder 保证是树的后序遍历
"""
import sys
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None,
                 right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


"""
解法一：递归分治 + 中序下标哈希表【最优解】

为什么慢：后序的最后一个元素就是根，这一步 O(1)；开销全在"根在中序的哪个
位置"。若每层递归都线性扫描中序区间找根，每层花 O(区间长)，左斜链与右斜链
都有 O(n) 层，累计退化成 O(n^2)。

换个角度：根的位置由"值 → 中序下标"决定，而题目保证值互不重复。先建一张表，
每次定位 O(1)，整棵树 O(n) 次访问就够。

关键点：
  · 后序的根在区间**右端**（`pr`），先序题在左端，这是两题唯一的切换点
  · 右子树节点数 `right_len = 中序区间右端 - 中序根下标`；后序里右半段紧贴
    根之前，即 [pl + left_len, pr - 1]
  · 左半段是 [pl, pl + left_len - 1]，left_len 与中序左段长度必须相等
  · 递归出口是区间为空（`pl > pr`），长度 1 时仍要建出叶子
  · 值域含负数，只能用 dict 存下标
  · Python 用闭包递归，dict 与两个列表在闭包外捕获，避免层层传参

以示例 1（inorder = [9,3,15,20,7]，postorder = [9,15,7,20,3]）为例：
  根 3（后序末尾），中序下标 1 → left_len = 1
  左：中序 [9]、后序 [9] → 叶子 9
  右：中序 [15,20,7]、后序 [15,7,20]，根 20（后序末尾），left_len = 1
      左边中序 [15]、后序 [15] → 15；右边中序 [7]、后序 [7] → 7
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，每个节点恰被创建一次
  空间 O(n)：dict 存 n 项；递归栈 O(h)，斜链时也是 O(n)

解法二：迭代栈，从后序末尾倒着走【最优解】

为什么慢：解法一要递归 O(h) 层，再额外占一张 O(n) 的哈希表；树深时递归栈
同样到 O(n)，时间上也不比解法二省。

换个角度：后序是「左 → 右 → 根」，倒着读就是「根 → 右 → 左」，等价于把
先序题的树做左右镜像后照抄。于是栈维护的是当前未完成的**右链**，中序指针
从末尾往前扫：
  · 栈顶与 `inorder[in_idx]` 不等：新节点是栈顶的右孩子，压栈继续往右下走
  · 相等：右链到底，连续弹栈跳过已在中序出现过的祖先，新节点挂在最后弹出的
    节点的**左边**

关键点：
  · in_idx 从 `n - 1` 开始、只减不增，方向与栈的「右链」一致
  · 相等时必须连续弹栈，弹到「不相等或栈空」再挂左孩子
  · 挂左孩子用的是最后弹出的节点，不是弹完后的栈顶
  · 后序末元素先建根、先入栈，循环从下标 n - 2 往下走
  · 与解法一并列最优：时间同为 O(n)，但不再需要哈希表，空间从 O(n) 降到
    O(h)；它是解法一的镜像写法，可用来对照「根在末尾」这一侧的边界
  · Python 用 list 当栈，`stack[-1]` 取栈顶，`pop()` 弹栈

以示例 1（inorder = [9,3,15,20,7]，postorder = [9,15,7,20,3]）为例：
  i=3，值 20：栈顶 3 ≠ in[4]=7 → 3.right = 20，压栈
  i=2，值 7：栈顶 20 ≠ in[4]=7 → 20.right = 7，压栈
  i=1，值 15：栈顶 7 = in[4] → 弹 7、弹 20，in_idx 走到 2；15 挂 20 左边
  i=0，值 9：栈顶 15 = in[2] → 弹 15、弹 3，in_idx 走到 0；9 挂 3 左边
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：每个节点进出栈各一次
  空间 O(h)：栈里只保存当前右链，不再需要哈希表
"""


class Solution1:
    def buildTree(self, inorder: list, postorder: list) -> Optional[TreeNode]:
        # pos：值 → 中序下标，题目保证无重复值
        pos = {v: i for i, v in enumerate(inorder)}

        def build(pl: int, pr: int, il: int, ir: int) -> Optional[TreeNode]:
            # 空区间才是出口：长度为 1 时仍需建出那个叶子
            if pl > pr:
                return None
            root_val = postorder[pr]
            k = pos[root_val]
            left_len = k - il

            node = TreeNode(root_val)
            # 左半段在前，右半段紧贴根，两段长度由 left_len 唯一确定
            node.left = build(pl, pl + left_len - 1, il, k - 1)
            node.right = build(pl + left_len, pr - 1, k + 1, ir)
            return node

        n = len(postorder)
        return build(0, n - 1, 0, n - 1)


class Solution2:
    def buildTree(self, inorder: list, postorder: list) -> Optional[TreeNode]:
        n = len(postorder)
        root = TreeNode(postorder[n - 1])
        stack = [root]
        # 中序指针从末尾往前：与「根 → 右 → 左」的倒序后序同向
        in_idx = n - 1

        for i in range(n - 2, -1, -1):
            node = stack[-1]
            if node.val != inorder[in_idx]:
                # 栈顶尚未在中序出现，倒序后序的这一项只能是它的右孩子
                node.right = TreeNode(postorder[i])
                stack.append(node.right)
            else:
                # 弹到不相等为止：最后弹出的节点正是新节点的父节点
                while stack and stack[-1].val == inorder[in_idx]:
                    node = stack.pop()
                    in_idx -= 1
                node.left = TreeNode(postorder[i])
                stack.append(node.left)

        return root


# ===== 测试辅助：层序序列化（None 表示空） =====
NULL_NODE = -2 ** 31  # 节点值在 [-3000, 3000]，不会撞上哨兵


def to_level_order(root: Optional[TreeNode]) -> list:
    if root is None:
        return []

    out = []
    # 用队列逐个展开：空节点也入队一次，保证位置不塌陷
    queue = [root]
    i = 0
    while i < len(queue):
        node = queue[i]
        i += 1
        if node is None:
            out.append(NULL_NODE)
            continue
        out.append(node.val)
        queue.append(node.left)
        queue.append(node.right)

    # 末尾的空孩子不改变结构，裁掉才能和题面输出逐项比较
    while out and out[-1] == NULL_NODE:
        out.pop()
    return out


def check(inorder: list, postorder: list, want: list) -> None:
    got = to_level_order(Solution1().buildTree(inorder[:], postorder[:]))
    assert got == want, (inorder, postorder, got, want)
    got = to_level_order(Solution2().buildTree(inorder[:], postorder[:]))
    assert got == want, (inorder, postorder, got, want)


if __name__ == "__main__":
    # 示例 1
    check([9, 3, 15, 20, 7], [9, 15, 7, 20, 3],
          [3, 9, 20, NULL_NODE, NULL_NODE, 15, 7])

    # 示例 2：单节点，两个解法都不该进循环体
    check([-1], [-1], [-1])

    # 边界：左斜链，后序升序
    check([1, 2, 3], [1, 2, 3], [3, 2, NULL_NODE, 1])

    # 边界：右斜链，后序降序
    check([1, 2, 3], [3, 2, 1], [1, NULL_NODE, 2, NULL_NODE, 3])

    # 关键点用例：根在末尾且右子树非空，右半段偏移写错时会挂到左边
    check([4, 2, 3, 1], [4, 3, 2, 1], [1, 2, NULL_NODE, 4, 3])

    # 满二叉树
    check([4, 2, 5, 1, 6, 3, 7], [4, 5, 2, 6, 7, 3, 1], [1, 2, 3, 4, 5, 6, 7])

    # 边界：含 0 与负数，验证按值定位
    check([-3000, 0, 3000], [-3000, 3000, 0], [0, -3000, 3000])

    # 边界：1000 层斜链，递归深度超过默认上限，先放宽
    sys.setrecursionlimit(10000)

    # 左斜链：中序升序、后序也升序
    ino = list(range(1, 1001))
    post = list(range(1, 1001))
    want = [1000]
    for v in range(999, 0, -1):
        want.append(v)
        if v > 1:
            want.append(NULL_NODE)
    check(ino, post, want)

    # 右斜链：中序升序、后序降序
    ino = list(range(1, 1001))
    post = list(range(1000, 0, -1))
    want = [1]
    for v in range(2, 1001):
        want.extend([NULL_NODE, v])
    check(ino, post, want)

    print("all tests passed")
