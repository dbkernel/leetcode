"""
============================================================================
LCR 152 / 剑指 Offer 33. 验证二叉搜索树的后序遍历序列（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-hou-xu-bian-li-xu-lie-lcof/
分类：二叉搜索树 · 递归分治（最差 O(n^2)）/ 单调栈 O(n)

请实现一个函数来判断整数数组 postorder 是否为二叉搜索树的后序遍历结果。

示例 1：
输入: postorder = [4,9,6,5,8]
输出: false
解释：从上图可以看出这不是一颗二叉搜索树

示例 2：
输入: postorder = [4,6,5,9,8]
输出: true
解释：可构建的二叉搜索树如上图

提示：
- 数组长度 <= 1000
- postorder 中无重复数字
"""
import sys

"""
解法一：递归分治（按「左 < 根 < 右」切段）【非最优】

为什么慢：后序的最后一个元素是根，左子树的值全小于根、右子树全大于根，于是
分段位置唯一。但分段只能靠扫描得到：每层从左往右找到第一个大于根的位置，
代价是这一段的长度。当树退化成一条左斜链（后序恰为升序 1..n）时，每层只剥掉
根，一共 O(n) 层，累计 O(n^2)。

换个角度：既然"根的左边一段全小、其余全大"是充要条件，那就递归地验证每一段
是否满足它。长度 <= 1 的段天然满足。

关键点：
  · 根永远是区间右端 `post[r]`，扫描范围到 `r` 的前一位为止
  · 先扫出左段边界 i：从 l 起连续满足 `post[i] < root` 的最大下标加一；
    再扫右段 i..r-1，只要出现一个小于根的值就说明这段不可能是右子树
  · 两段的递归区间是 `[l, i-1]` 与 `[i, r-1]`，右段要排除根本身
  · 题目保证无重复数字，所以用严格小于 / 大于即可；出现相等同样非法
  · 最差 O(n^2)，本题长度上限 1000 才勉强够用，这也是它不是最优解的原因
  · Python 里 1000 层递归会碰到默认上限，自测里先 setrecursionlimit

以示例 2（postorder = [4,6,5,9,8]）为例：
  根 8，扫描左段：4 < 8、6 < 8、5 < 8、9 > 8 → i = 3
  右段 [9] 全大于 8，通过 → 递归 [4,6,5] 与 [9]
  [4,6,5]：根 5，左段 [4]，右段 [6]，都合规；[9] 长度 1 直接通过
  全部合规 → true ✔

再以示例 1（postorder = [4,9,6,5,8]）为例：
  根 8，左段只取到 [4]（9 > 8），右段 [9,6,5] 里出现 6 < 8 → false ✔

复杂度分析：
  时间 O(n^2)：最差（左斜链）每层只缩短 1，逐层扫描累计 n + (n-1) + …
  空间 O(n)：递归栈深度等于树高，最差 O(n)

解法二：单调栈（倒序扫描，维护祖先上界）【最优解】

为什么慢：解法一把"值的上下界"交给递归层层传递，同一个元素会被不同层反复扫
到。如果能把"当前允许的最大值"用一个变量记住，每个元素就只需看一次。

换个角度：后序倒着读是「根 → 右 → 左」。右子树的节点值沿祖先链单调递减，所以
用单调递减栈保存当前这条链；一旦倒序序列出现上升，说明刚走完某个右子树、
进入它父节点的左子树，被弹掉的那个父节点就是此后所有元素的上界。

关键点：
  · root 表示"当前所在左子树的上界"，初值为 +∞（第一个节点没有上界）
  · 先判 `post[i] > root` 直接返回 false —— 上界是父节点，左子树不可能更大
  · 弹栈条件写在 `post[i] < 栈顶`，因为相等只可能来自重复值，本题不存在
  · 新 root 必须取**最后一个**弹出的值：弹栈是从最近的祖先往上走，最后弹出的
    才是当前区间的父节点
  · 弹完才把当前值压栈，保持栈从底到顶递减
  · 空数组视为合法（空树也是二叉搜索树）
  · Python 用 list 当栈，`stk[-1]` 取栈顶，`pop()` 弹栈

以示例 2（postorder = [4,6,5,9,8]）为例：
  i=4，值 8：8 < ∞，栈空 → 压栈，栈 [8]
  i=3，值 9：9 < ∞，栈顶 8 < 9 不弹 → 压栈，栈 [8,9]
  i=2，值 5：栈顶 9 > 5 → root = 9 弹出；栈顶 8 > 5 → root = 8 弹出；
             压栈，栈 [5]
  i=1，值 6：6 < root=8 通过，栈顶 5 < 6 → 压栈，栈 [5,6]
  i=0，值 4：4 < 8，栈顶 6 > 4 → root = 6；栈顶 5 > 4 → root = 5 → 压栈
  全程未越界 → true ✔

再以示例 1（postorder = [4,9,6,5,8]）为例：
  i=4，8 入栈；i=3，5 弹出 8、root = 8，压 5；i=2，6 < 8 压栈；
  i=1，值 9 > root=8 → false ✔

复杂度分析：
  时间 O(n)：每个元素进出栈各一次
  空间 O(n)：栈最深保存一条右链
"""


class Solution1:
    def verifyPostorder(self, postorder: list) -> bool:
        def verify(l: int, r: int) -> bool:
            # 空区间与单节点都没有「左 < 根 < 右」可违反
            if l >= r:
                return True

            root = postorder[r]

            # 左段：从 l 起连续小于根的部分，i 停在第一个不小于根的位置
            i = l
            while i < r and postorder[i] < root:
                i += 1

            # 右段必须全部大于根；出现更小的值说明这段被切错了位置
            for j in range(i, r):
                if postorder[j] < root:
                    return False

            # 右段区间是 [i, r-1]，r 位置的根不属于任何一段
            return verify(l, i - 1) and verify(i, r - 1)

        return verify(0, len(postorder) - 1)


class Solution2:
    def verifyPostorder(self, postorder: list) -> bool:
        stack = []
        # root 是「当前所在左子树的上界」，初值 +∞ 表示最外层没有上界
        root = float("inf")

        for i in range(len(postorder) - 1, -1, -1):
            if postorder[i] > root:
                return False

            # 弹出的是刚走完的右子树的祖先，最后弹出的那个成为新的上界
            while stack and stack[-1] > postorder[i]:
                root = stack.pop()
            stack.append(postorder[i])
        return True


# ===== 测试辅助：两个解法跑同一组断言 =====
def check(postorder: list, want: bool) -> None:
    got = Solution1().verifyPostorder(postorder[:])
    assert got == want, ("recursive", postorder, got, want)
    got = Solution2().verifyPostorder(postorder[:])
    assert got == want, ("monotonic", postorder, got, want)


if __name__ == "__main__":
    # 1000 个节点的链会到 1000 层，递归解法需要放宽上限
    sys.setrecursionlimit(10000)

    # 示例 1：右段里混进比根小的值
    check([4, 9, 6, 5, 8], False)

    # 示例 2
    check([4, 6, 5, 9, 8], True)

    # 边界：空数组，空树也是二叉搜索树
    check([], True)

    # 边界：单节点
    check([1], True)

    # 边界：只有左孩子 / 只有右孩子，两条序列互为镜像
    check([1, 2], True)
    check([2, 1], True)

    # 边界：左斜链（升序）与右斜链（降序），两者都合法
    check([1, 2, 3], True)
    check([3, 2, 1], True)

    # 关键点用例：根的最大值必须由「最后一个弹出」决定
    check([1, 4, 3, 2], True)

    # 关键点用例：左段合法但右段混入更小的值
    check([1, 2, 5, 3, 4], False)

    # 关键点用例：没有左段可切，值却全在根的右侧排不成 BST
    check([7, 4, 6, 5], False)

    # 边界：1000 个节点的左斜链，递归解法的最差形态
    check(list(range(1, 1001)), True)

    # 边界：1000 个节点的右斜链
    check(list(range(1000, 0, -1)), True)

    # 边界：上面那条升序链把一个值改大，制造"根之后还有更大值"的违例
    broken = list(range(1, 1001))
    broken[500] = 1001
    check(broken, False)

    print("all tests passed")
