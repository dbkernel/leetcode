"""
============================================================================
334. 递增的三元子序列（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/increasing-triplet-subsequence/
分类：贪心 · 双变量维护最小递增对

给你一个整数数组 nums ，判断这个数组中是否存在长度为 3 的递增子序列。

如果存在这样的三元组下标 (i, j, k) 且满足 i < j < k ，使得
nums[i] < nums[j] < nums[k] ，返回 true ；否则，返回 false 。

示例 1：
输入：nums = [1,2,3,4,5]
输出：true
解释：任何 i < j < k 的三元组都满足题意

示例 2：
输入：nums = [5,4,3,2,1]
输出：false
解释：不存在满足题意的三元组

示例 3：
输入：nums = [2,1,5,0,4,6]
输出：true
解释：其中一个满足题意的三元组是 (1, 4, 5)，因为
nums[1] == 1 < nums[4] == 4 < nums[5] == 6

提示：
1 <= nums.length <= 5 * 10^5
-2^31 <= nums[i] <= 2^31 - 1

进阶：你能实现时间复杂度为 O(n) ，空间复杂度为 O(1) 的解决方案吗？
"""
from typing import List


"""
解法一：贪心维护 first / second【最优解】

为什么慢：直接枚举 i<j<k 是 O(n^3)；固定 i、j 后找右侧最大值可以做到
O(n^2)，仍然超时。瓶颈是「每换一个 i 或 j 就重新扫一遍后缀」。

换个角度：从左到右扫描时，只关心两件事 ——「目前见过的最小元素」和
「目前见过的最短递增对的第二个元素」。只要后面出现比 second 大的数，
三元组就凑齐了。

关键点：
  · first 存最小元素，second 存递增对 (i,j) 中 nums[j] 的最小值
  · 判断顺序必须是「先比 second，再比 first」：等于 second 不能算三元组
  · 用严格大于 / 严格小于，distinct 要求写成 >= 会漏掉相等元素的情况
  · 哨兵取比任何 nums[i] 都大的值（这里用 float("inf")）

为什么贪心是对的（不变式，不看两个数是否同源）：
  · 扫描到任意位置时维护：
    first  = 已扫描元素的最小值；
    second = 所有已出现的递增对 (i,j)（i<j 且 nums[i]<nums[j]）中
             nums[j] 的最小值。
  · 关键：first 与 second 不需要来自同一个前缀，也不必满足
    「first 的下标 < second 的下标」。两者携带的信息彼此独立：
    - second 确定的那一刻，就已经记住了一对真实存在的 (i,j)；
      后来 first 被更小的数覆盖，不会让这对 (i,j) 失效。
    - first 只用来判断「当前数能否成为新的第二元素」，它的下标一定
      早于当前数，所以 (first, 当前数) 是合法的递增对。
  · 于是对读到的 x：
    x > second           → second 背后的 (i,j) 加当前下标构成三元组；
    first < x <= second  → (first, x) 是新的递增对，把 second 降到 x
                           只会让之后更容易命中；
    x <= first           → 更新 first，second 与其背后的对保持不变。
  · 反证完整性：若存在三元组 (i,j,k)，处理到 k 时 second <= nums[j]
    （second 是所有第二元素的最小值），于是 nums[k] > nums[j] >= second，
    算法早已返回 true。与「返回 false」矛盾。

以示例 3（nums = [2,1,5,0,4,6]）为例：
  2 → first=2
  1 → first=1
  5 → 1 < 5 <= inf → second=5（对是 (1,5)）
  0 → first=0（second 背后的 (1,5) 仍然有效）
  4 → 0 < 4 <= 5 → second=4（对是 (0,4)）
  6 → 6 > 4 → 返回 true ✔

复杂度分析：
  时间 O(n)：每个元素只比较常数次
  空间 O(1)：只用了 first / second 两个变量
"""


class Solution1:
    def increasingTriplet(self, nums: List[int]) -> bool:
        # 哨兵比任何 nums[i] 都大，保证真实元素一定能覆盖它
        first = float("inf")
        second = float("inf")

        for x in nums:
            # 命中 third：second 背后的那对 (i,j) 的下标都早于当前位置
            if x > second:
                return True

            # 能接在 first 后面：更新第二元素的最小值
            if x > first:
                second = x
            # 比 first 还小：只换第一元素，旧递增对仍然有效
            else:
                first = x
        return False


"""
解法二：前后缀最值数组（枚举中间元素）【非最优】

为什么慢：贪心版已经把信息压到两个变量里，本解法把「左边最小值」和
「右边最大值」都显式存下来，多用了 O(n) 空间。

换个角度：三元组必须有一个中间元素 nums[j]。只要它左边存在比它小的数、
右边存在比它大的数，就存在以它为中间的三元组。于是预处理前缀最小值和
后缀最大值，再枚举中间位置即可。

关键点：
  · 中间下标只能取 1..n-2，首尾不能当中间元素
  · 前缀最小值要先取 nums[0]，再边扫边更新；后缀最大值同理从右往左
  · 判断用严格不等号，相等不算递增

复杂度分析：
  时间 O(n)：预处理两趟，枚举一趟
  空间 O(n)：suffix_max 数组；prefix_min 用滚动变量省掉一个数组
"""


class Solution2:
    def increasingTriplet(self, nums: List[int]) -> bool:
        n = len(nums)
        if n < 3:
            return False

        # suffix_max[i]：nums[i..n-1] 的最大值
        suffix_max = [0] * n
        suffix_max[n - 1] = nums[n - 1]
        for i in range(n - 2, -1, -1):
            suffix_max[i] = max(suffix_max[i + 1], nums[i])

        # prefix_min 滚动保存 nums[0..i-1] 的最小值
        prefix_min = nums[0]
        for i in range(1, n - 1):
            if prefix_min < nums[i] < suffix_max[i + 1]:
                return True
            prefix_min = min(prefix_min, nums[i])
        return False


# 两种解法必须给出一致答案，任一失败都能从参数定位用例
def check(nums: List[int], want: bool) -> None:
    assert Solution1().increasingTriplet(nums) == want
    assert Solution2().increasingTriplet(nums) == want


if __name__ == "__main__":
    # 示例 1：严格递增，任意三个下标都行
    check([1, 2, 3, 4, 5], True)

    # 示例 2：严格递减，不存在递增对
    check([5, 4, 3, 2, 1], False)

    # 示例 3：first 被 0 覆盖后，second 背后的旧递增对仍能凑出答案
    check([2, 1, 5, 0, 4, 6], True)

    # 边界：全相等，严格递增不成立
    check([1, 1, 1], False)

    # 边界：长度不足 3
    check([1, 2], False)

    # 关键用例：first=0 来自 second=2 之后，两个变量不同源
    check([1, 2, 0, 3], True)

    # 反例：second 已出现但 first 被更小数覆盖后没有第三个数
    check([2, 3, 1], False)

    # 边界：只有两档数值反复出现，凑不出第三档
    check([1, 1, 2, 2, 1, 1], False)

    # 反直觉：递增三元组不一定包含全局最小值
    check([20, 100, 10, 12, 5, 13], True)

    print("all tests passed")
