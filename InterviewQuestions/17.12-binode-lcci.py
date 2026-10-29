"""
============================================================================
面试题 17.12. BiNode（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binode-lcci/
分类：二叉搜索树 · 中序遍历 / Morris

二叉树数据结构TreeNode可用来表示单向链表（其中left置空，right为下一个链表
节点）。实现一个方法，把二叉搜索树转换为单向链表，要求依然符合二叉搜索树的
性质，转换操作应是原址的，也就是在原始的二叉搜索树上直接修改。

返回转换后的单向链表的头节点。

注意：本题相对原题稍作改动

示例：
输入： [4,2,5,1,3,null,6,0]
输出： [0,null,1,null,2,null,3,null,4,null,5,null,6]

提示：
- 节点数量不会超过 100000。
"""
import sys
from typing import Optional


# 力扣预置的节点定义
class TreeNode:
    def __init__(self, x: int):
        self.val = x
        self.left: Optional["TreeNode"] = None
        self.right: Optional["TreeNode"] = None


"""
解法一：中序遍历，递归返回 (头, 尾) 再拼接【非最优】

为什么慢：中序遍历本身 O(n)，慢的地方在"怎么原地拼"。如果只保存一个"上一
个访问的节点"，递归回溯时无法在 O(1) 里把左链表接到根上；如果先收集节点再
重连，又要 O(n) 数组。真正的瓶颈是信息不够：拼接需要同时知道一段链表的头
和尾。

换个角度：让递归函数直接返回展开后的 (头, 尾) 一对。根的中序序列就是
"左链表 + 根 + 右链表"，拿到左尾、右头之后，拼接只是两次指针赋值，返回时
把头尾合并即可。

关键点：
  · 返回值必须是 (头, 尾) 一对：只返回头就得 O(n) 找尾才能拼接
  · 尾的取法：右子树非空取右尾，否则取当前节点；不能一律取当前节点
  · 必须先递归完左右子树再改当前节点的指针，否则会丢失子树
  · 拼接后当前节点的 left 必须置空（题目要求单向链表）
  · 空树返回 (None, None)，作为递归出口
  · Python 用元组返回一对指针，调用处解包即可，不必定义额外结构体

以示例（root = [4,2,5,1,3,null,6,0]）为例：
  节点 0：无子树，返回 (0, 0)；
  节点 1：左尾 0 接到 1，返回 (0, 1)；
  节点 2：左链表 (0,1) 的尾 1 接到 2，右链表 (3,3) 接在 2 后，
          返回 (0, 3)；
  依此向上，最终头是最左的 0，右链为 0→1→…→6 ✔

复杂度分析：
  时间 O(n)：每个节点进入递归一次，拼接是常数次赋值
  空间 O(h)：递归栈，h 为树高；最坏（斜树）O(n)，这正是解法二要省的

解法二：Morris 中序遍历，原地调整指针【最优解】

为什么慢：解法一的递归栈是 O(h)。中序需要栈，只是为了"从左子树回来后知道
该访问谁"；这个"谁"其实就是当前节点，可以用左子树最右节点的空 right 指针
直接指回来（线索），不需要真正的栈。

换个角度：中序里，节点 cur 的前驱是"左子树最右节点"。让前驱的空 right 指向
cur，向左走；等左子树遍历完，顺着这条线索回到 cur，此时才访问 cur，并把
"上一个访问节点"的 right 接成 cur，同时清空 cur 的 left。遍历和重连在同一个
循环里完成。

关键点：
  · 有左孩子时不算访问：先找前驱 pred（左子树最右），pred.right = cur 建
    线索，清空 cur.left，再向左走
  · 没有左孩子时才是访问：prev.right = cur；prev 为空时 cur 就是链表头
  · 线索只在回到 cur 后被 prev.right = cur 覆盖，值相同，不必单独拆链
  · 每个被访问节点的 left 都必须为 None；有左孩子的节点在向左走之前就清空
  · 最后访问的节点是中序最大节点，它的 right 本来就是 None，无需额外处理

以示例（root = [4,2,5,1,3,null,6,0]）为例：
  一路向左：4 的前驱 3 指向 4，2 的前驱 1 指向 2，1 的前驱 0 指向 1；
  访问 0（头）、顺线索回 1 并访问、回 2 并访问、访问 3、回 4；
  再依次访问 5、6，右链 0→1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：每条线索（边）最多被走两次，建线索一次、沿线索返回一次
  空间 O(1)：只用 cur、prev、pred 三个引用，不借助栈
"""


class Solution1:
    def convertBiNode(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        head, _ = self._dfs(root)
        return head

    # 返回展开后的 (头, 尾)：只返回头就没法 O(1) 接上左链表
    def _dfs(self, node: Optional[TreeNode]):
        if node is None:
            return None, None

        # 先递归完左右子树，再动指针，否则会丢失子树
        left_head, left_tail = self._dfs(node.left)
        right_head, right_tail = self._dfs(node.right)

        node.left = None
        node.right = right_head
        if left_tail is not None:
            left_tail.right = node

        head = left_head if left_head is not None else node
        # 尾取右链的尾；没有右子树时当前节点就是尾
        tail = right_tail if right_tail is not None else node
        return head, tail


class Solution2:
    def convertBiNode(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        cur = root
        head = None
        prev = None

        while cur is not None:
            if cur.left is not None:
                # 前驱 = 左子树最右节点；它此刻还没有右孩子
                pred = cur.left
                while pred.right is not None:
                    pred = pred.right
                pred.right = cur  # 线索：左子树走完能回到 cur

                left = cur.left
                cur.left = None
                cur = left
            else:
                # 访问 cur：把上一个访问节点的 right 接过来
                if prev is not None:
                    prev.right = cur
                else:
                    head = cur  # 第一个访问的就是最左节点，即链表头
                prev = cur
                cur = cur.right  # 可能是线索，也可能是原右子树

        return head


# ===== 测试辅助：按层序数组（None 表示空）建树、按右链收集 =====
MAX_NODES = 100000  # 题目上限，用来兜住成环的错误


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


# 沿 right 收集：同时断言 left 全为空、右链没有环
def collect_right_chain(head: Optional[TreeNode]) -> list:
    values = []
    cur = head
    while cur is not None:
        assert cur.left is None, f"node {cur.val} 残留左孩子"
        values.append(cur.val)
        cur = cur.right
        assert len(values) <= MAX_NODES, "right 链成环"
    return values


# BST 的中序就是升序，所以期望值 = 输入里的非空值排序
def sorted_values(level_order: list) -> list:
    return sorted(v for v in level_order if v is not None)


def check(level_order: list) -> None:
    want = sorted_values(level_order)
    assert collect_right_chain(
        Solution1().convertBiNode(build_tree(level_order))
    ) == want
    assert collect_right_chain(
        Solution2().convertBiNode(build_tree(level_order))
    ) == want


if __name__ == "__main__":
    # 顶点数上限 2000，斜树时递归深度等于节点数，超过 Python 默认的 1000
    sys.setrecursionlimit(10000)

    # 示例：左右都有子树，且最左节点在第三层
    check([4, 2, 5, 1, 3, None, 6, 0])

    # 边界：空树
    check([])

    # 边界：单节点
    check([1])

    # 边界：只有左孩子的斜链，头是最深的那个
    check([3, 2, None, 1])

    # 边界：只有右孩子的斜链，头就是根
    check([1, None, 2, None, 3])

    # 反直觉：右子树里有比根小的节点 —— 结果仍必须是全升序
    check([1, None, 3, 2])

    # 反直觉：根的两个孩子都不是最值，最左/最右藏在深处
    check([10, 5, 15, 2, 7, 12, 20, 1, 3, 6, 8, 11, 13])

    # 满 BST
    check([4, 2, 6, 1, 3, 5, 7])

    # 边界：2000 层左斜链，验证递归与线索都不改变顺序
    chain = [2000]
    for i in range(1999, 0, -1):
        chain.extend([i, None])
    check(chain)

    print("all tests passed")
