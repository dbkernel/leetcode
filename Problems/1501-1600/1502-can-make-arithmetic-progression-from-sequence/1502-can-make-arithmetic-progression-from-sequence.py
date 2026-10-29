"""
============================================================================
1502. 判断能否形成等差数列（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/can-make-arithmetic-progression-from-sequence/
分类：排序 · 模拟

给你一个数字数组 arr 。

如果一个数列中，任意相邻两项的差总等于同一个常数，那么这个数列就称为 等差数列 。

如果可以重新排列数组形成等差数列，请返回 true ；否则，返回 false 。

示例 1：
输入：arr = [3,5,1]
输出：true
解释：对数组重新排序得到 [1,3,5] 或者 [5,3,1] ，任意相邻两项的差分别为 2 或 -2 ，
可以形成等差数列。

示例 2：
输入：arr = [1,2,4]
输出：false
解释：无法通过重新排序得到等差数列。

提示：
2 <= arr.length <= 1000
-10^6 <= arr[i] <= 10^6
"""


"""
解法一：排序后模拟【最优解】

官方题解只给出这一种方法，也就是本题的最优解：时间 O(n log n)、空间 O(log n)。

为什么慢：如果直接按「原顺序」去比较相邻两项的差，是不对的——题目允许重新排列，
原顺序相邻的差相等并不代表整个数组能排成等差数列。必须先把顺序「规整」一下。

换个角度：能排成等差数列的数组，排序之后必然是等差数列。所以排序就是本题的
「归约」步骤：一旦有序，等差数列的性质就变成一条可以逐项检验的局部条件。

关键点：
  · 有序后，等差数列 ⇔ 对每个 i ∈ [1, n-2] 都有 a[i] - a[i-1] == a[i+1] - a[i]；
    也等价于官方写的 a[i] * 2 == a[i-1] + a[i+1]
  · 只检查相邻的三个数就够，因为公差只需一致一次就对全局一致（相邻区间共享端点）
  · 用「减去前一项」而不是「乘 2」，写法更直观且不涉及溢出的顾虑
  · 建议先 sorted(arr) 复制一份再排序，避免改动调用方传入的列表

以 arr = [3,5,1] 为例：
  排序 → [1,3,5]
  i=1：3-1=2，5-3=2 → 相等
  返回 True

以 arr = [1,2,4] 为例：
  排序 → [1,2,4]
  i=1：2-1=1，4-2=2 → 不等 → 返回 False

复杂度分析：
  时间 O(n log n)：排序 O(n log n) + 线性扫描 O(n)
  空间 O(n)：sorted() 复制出的新列表（就地排序则只需 O(log n) 栈空间）
"""


class Solution:
    def canMakeArithmeticProgression(self, arr: list[int]) -> bool:
        """解法一：排序后模拟（最优解）"""
        a = sorted(arr)
        for i in range(1, len(a) - 1):
            if a[i] - a[i - 1] != a[i + 1] - a[i]:
                return False
        return True


def check(arr: list[int], want: bool) -> None:
    original = list(arr)
    got = Solution().canMakeArithmeticProgression(arr)
    assert got == want, f"canMakeArithmeticProgression({original}) = {got}"


if __name__ == "__main__":
    # 示例 1
    check([3, 5, 1], True)

    # 示例 2
    check([1, 2, 4], False)

    # 边界：只有两个元素，一定能形成等差数列
    check([1, 2], True)
    check([-1000000, 1000000], True)

    # 边界：已有序 / 逆序
    check([1, 2, 3, 4], True)
    check([4, 3, 2, 1], True)

    # 边界：公差为 0（全部相等）
    check([7, 7, 7, 7], True)

    # 边界：中间某项破坏公差
    check([1, 3, 7], False)
    check([0, 2, 4, 7], False)

    # 较大规模：0..999 打乱顺序（公差为 1）
    check([(i * 617) % 1000 for i in range(1000)], True)

    # 较大规模但破坏一项
    broken = [(i * 617) % 1000 for i in range(1000)]
    broken[500] = 1000000
    check(broken, False)

    print("all tests passed")
