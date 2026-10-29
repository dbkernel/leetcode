"""
============================================================================
416. 分割等和子集（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/partition-equal-subset-sum/
分类：动态规划 · 0-1 背包（子集和）
============================================================================

给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组
分割成两个子集，使得两个子集的元素和相等。

示例 1：
输入：nums = [1,5,11,5]
输出：true
解释：数组可以分割成 [1, 5, 5] 和 [11] 。

示例 2：
输入：nums = [1,2,3,5]
输出：false
解释：数组不能分割成两个元素和相等的子集。

提示：
- 1 <= nums.length <= 200
- 1 <= nums[i] <= 100
"""
import random
from typing import List


"""
解法一：二维 0-1 背包（逐件物品、逐个子集和）【非最优】

为什么慢：最直接的做法是枚举所有子集，看有没有哪一半的和等于总和的一半。
子集共 2^n 个，n = 200 时连边都摸不到。

换个角度：只关心「能不能凑出某个和」，不关心是哪些数凑的。设
dp[i][j] 表示「前 i 个数能否凑出和 j」，每件物品只有选与不选两条路，
这就是 0-1 背包的存在性问题。

关键点：
  · 先判总和：两半相等 ⇒ 总和必为偶数，奇数直接返回 false
  · 只需凑 half = sum/2：凑出 half 后另一半自动也是 half，不必再管
  · j < nums[i] 时只能走「不选」这一条，不能写成 dp[i-1][负数]
  · 第一维按「前 i 个数」而不是「下标 i」，取值用 nums[i-1]，
    否则第 0 行与第 1 件物品之间会错开一位
  · 初值 dp[0][0] = True（空集和为 0），dp[0][j>0] = False

以示例 1（nums = [1,5,11,5]，总和 22，half = 11）为例：
  前 1 个数：可达 {0,1}
  前 2 个数：可达 {0,1,5,6}
  前 3 个数：11 出现
  前 4 个数：11 仍可达 → true
  对应划分 {11} 与 {1,5,5}，两边各 11 ✔

复杂度分析：
  时间 O(n·S)：n 件物品 × S = half 个和值，每格 O(1)
  空间 O(n·S)：二维表整张保留，S 最大 100×200/2 = 10^4

解法二：一维滚动数组（倒序遍历容量）【最优解】

为什么慢：解法一留下整张二维表，但第 i 行只依赖第 i-1 行。多留的 n-1 行
都是为了「回看」，而回看只需要上一行还在。

换个角度：把物品维度压掉，dp[j] 当场表示「凑出 j 是否可行」，每加入一个数
就原地更新整行。只要读到的还是「加入之前」的值，结论就与二维版相同。

关键点：
  · 内层必须倒序：j 从 S 递减到 nums[i]。正序时 dp[j - nums[i]] 可能
    刚被当前这个数改过，同一个数会被重复使用，0-1 背包退化成完全背包
  · 倒序时 dp[j - nums[i]] 尚未被本轮写过，天然是「上一行」的值
  · j 只降到 nums[i]：更小的 j 放不下当前数，保持原值
  · dp[0] = True 是唯一初值，含义是「一个数都不选，和为 0」；
    没有它，任何比 0 大的和都推不出来
  · dp[S] 一旦为真即可提前返回，后面的数不可能把它改回假

手推一遍（nums = [1,5,11,5]，S = 11，只列被点亮的格子）：
  初值 dp[0] = T
  处理 1（j 由 11 降到 1）：dp[1] = dp[0] = T
  处理 5（j 由 11 降到 5）：dp[6] = dp[1] = T，dp[5] = dp[0] = T
  处理 11（j 取 11）：dp[11] = dp[0] = T → 返回 True
  最后一个 5 不必再处理
  ✔ 结论与解法一一致

复杂度分析：
  时间 O(n·S)：状态数与解法一相同，只是省掉行维度
  空间 O(S)：只留一行，比解法一少 n 倍
"""


class Solution1:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        # 奇数总和不可能被平分成两个整数和
        if total % 2 != 0:
            return False

        half = total // 2
        n = len(nums)
        # dp[i][j]：前 i 个数能否凑出 j；第 0 行只有 j = 0 可行
        dp = [[False] * (half + 1) for _ in range(n + 1)]
        dp[0][0] = True

        for i in range(1, n + 1):
            value = nums[i - 1]
            for j in range(half + 1):
                # 不选第 i 个数：沿用上一行的结论
                dp[i][j] = dp[i - 1][j]
                # 选第 i 个数：要求 j 放得下，且余下部分上一行能凑出
                if not dp[i][j] and j >= value:
                    dp[i][j] = dp[i - 1][j - value]
        return dp[n][half]


class Solution2:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2 != 0:
            return False

        half = total // 2
        # 只留一行：dp[j] 表示当前这批数能否凑出 j
        dp = [False] * (half + 1)
        dp[0] = True

        for value in nums:
            # 倒序：dp[j - value] 仍是加入 value 之前的值
            for j in range(half, value - 1, -1):
                if dp[j - value]:
                    dp[j] = True
            if dp[half]:
                return True
        return dp[half]


# ===== 测试辅助：小规模暴力枚举，作为对拍的独立真值来源 =====
def brute_force(nums: List[int]) -> bool:
    """2^n 枚举子集，看是否存在和为 half 的一部分"""
    total = sum(nums)
    if total % 2 != 0:
        return False

    half = total // 2
    n = len(nums)
    for mask in range(1 << n):
        s = 0
        for i in range(n):
            if mask >> i & 1:
                s += nums[i]
        if s == half:
            return True
    return False


def check(nums: List[int], want: bool) -> None:
    assert Solution1().canPartition(nums) == want, ("2D", nums, want)
    assert Solution2().canPartition(nums) == want, ("1D", nums, want)


def cross_check() -> None:
    rng = random.Random(20261030)
    for _ in range(400):
        n = rng.randint(1, 10)
        nums = [rng.randint(1, 7) for _ in range(n)]
        check(nums, brute_force(nums))


if __name__ == "__main__":
    # 示例 1：{11} 与 {1,5,5}
    check([1, 5, 11, 5], True)

    # 示例 2：总和 11 为奇数，直接否决
    check([1, 2, 3, 5], False)

    # 边界：单元素，half 比这个数还小，永远凑不出
    check([1], False)
    check([2], False)

    # 边界：总和为偶数但无解 —— 这组凑不出 8
    check([3, 3, 3, 3, 4], False)

    # 边界：总和为奇数，且最大元素已超过一半
    check([1, 2, 4], False)

    # 边界：最小的可行输入
    check([1, 1], True)
    check([2, 2], True)

    # 最大规模：200 个 100，half = 10000 由 100 个 100 凑出
    check([100] * 200, True)

    # 最大规模下的无解：全为偶数，half 却是奇数 9951，必然凑不出
    check([100] * 199 + [2], False)

    # 对拍：随机小数组与暴力枚举逐组比对
    cross_check()

    print("all tests passed")
