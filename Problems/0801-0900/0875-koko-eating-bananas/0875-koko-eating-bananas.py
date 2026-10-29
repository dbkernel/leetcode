"""
============================================================================
875. 爱吃香蕉的珂珂（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/koko-eating-bananas/
分类：二分查找 · 答案空间二分（单调可行性）

珂珂喜欢吃香蕉。这里有 n 堆香蕉，第 i 堆中有 piles[i] 根香蕉。警卫已经
离开了，将在 h 小时后回来。

珂珂可以决定她吃香蕉的速度 k （单位：根/小时）。每个小时，她将会选择一堆
香蕉，从中吃掉 k 根。如果这堆香蕉少于 k 根，她将吃掉这堆的所有香蕉，然后
这一小时内不会再吃更多的香蕉。

珂珂喜欢慢慢吃，但仍然想在警卫回来前吃掉所有的香蕉。

返回她可以在 h 小时内吃掉所有香蕉的最小速度 k （k 为整数）。

示例 1：
输入：piles = [3,6,7,11], h = 8
输出：4

示例 2：
输入：piles = [30,11,23,4,20], h = 5
输出：30

示例 3：
输入：piles = [30,11,23,4,20], h = 6
输出：23

提示：
- 1 <= piles.length <= 10^4
- piles.length <= h <= 10^9
- 1 <= piles[i] <= 10^9
"""
import random
from typing import List


"""
解法：二分答案（对速度做二分）【最优解】

为什么慢：速度 k 的合法范围是 1 .. max(piles)，逐个数到第一个可行解
最坏要尝试 10^9 个 k，每个 k 还要扫一遍 piles，无法接受。

换个角度：若速度 k 能在 h 小时内吃完，那么任何 k' > k 也一定能吃完
（每小时吃得多，耗时不增）。可行性关于 k 单调，于是可以在答案空间上
二分：找最小的可行 k。

关键点：
  · 下界取 1，不能取 0 —— 速度为 0 时除数为 0，且永远吃不完
  · 上界取 max(piles)：最大堆一小时吃完，其余堆也都一小时一株，
    总耗时不超过 piles.length ≤ h，所以上界一定可行
  · 单堆耗时是上取整 (p + k - 1) // k，写成 p // k 会漏掉不满一小时的尾巴
  · Python 整数不会溢出，不需要像 C++ / Go 那样显式转 64 位；
    判定时一旦超过 h 可以提前返回，省掉后续堆的计算
  · 可行时 hi = mid（保留 mid 作为候选），不可行时 lo = mid + 1；
    循环结束 lo == hi 就是最小可行速度

手推一遍：
  以示例 1（piles = [3,6,7,11], h = 8）为例：
  lo = 1，hi = 11
  mid = 6：耗时 1 + 1 + 2 + 2 = 6 ≤ 8 → hi = 6
  mid = 3：耗时 1 + 2 + 3 + 4 = 10 > 8 → lo = 4
  mid = 5：耗时 1 + 2 + 2 + 3 = 8 ≤ 8 → hi = 5
  mid = 4：耗时 1 + 2 + 2 + 3 = 8 ≤ 8 → hi = 4
  lo == hi == 4 → 答案 4

复杂度分析：
  时间 O(n log max(piles))：二分 log(max) 轮，每轮扫一遍 n 堆
  空间 O(1)：只用了上下界与累加变量
"""


class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        if not piles:
            return 0

        lo, hi = 1, max(piles)
        while lo < hi:
            mid = lo + (hi - lo) // 2
            hours = 0
            for p in piles:
                # 上取整：p 不够 k 根也要占满一小时
                hours += (p + mid - 1) // mid
                if hours > h:
                    break
            if hours <= h:
                hi = mid
            else:
                lo = mid + 1
        return lo


# 测试用暴力：从 1 开始逐个尝试速度，只在小数据上跑
def brute(piles: List[int], h: int) -> int:
    for k in range(1, max(piles) + 1):
        hours = 0
        for p in piles:
            hours += (p + k - 1) // k
        if hours <= h:
            return k
    return max(piles)


def check(piles: List[int], h: int, want: int) -> None:
    a = piles[:]
    assert Solution().minEatingSpeed(a, h) == want
    assert a == piles


if __name__ == "__main__":
    # 示例 1：效率上限是 8 小时
    check([3, 6, 7, 11], 8, 4)

    # 示例 2：h 恰好等于堆数，每堆只能一个小时，速度必须取最大堆
    check([30, 11, 23, 4, 20], 5, 30)

    # 示例 3：多一个小时，最大堆可以分两小时吃完
    check([30, 11, 23, 4, 20], 6, 23)

    # 边界：单堆，h = 1 时必须一口气吃完
    check([5], 1, 5)
    check([5], 5, 1)
    check([5], 100, 1)

    # 边界：全是 1，任何速度 1 都能吃完
    check([1, 1, 1], 3, 1)
    check([1, 1, 1], 100, 1)

    # 边界：h == n，每个堆都只能用一小时
    check([3, 6, 7, 11], 4, 11)

    # 边界：10^9 级别，验证上取整与求和
    check([1000000000], 1, 1000000000)
    check([1000000000, 1000000000], 2, 1000000000)
    check([1000000000, 1000000000], 1000000000, 2)
    check([805306368, 805306368, 805306368], 1000000000, 3)

    # 边界：空数组（题目保证至少一堆，这里只验证健壮性）
    check([], 1, 0)

    # 随机交叉验证：小数据上与暴力逐个尝试的结果对齐
    rng = random.Random(20240607)
    for _ in range(500):
        n = rng.randint(1, 6)
        piles = [rng.randint(1, 15) for _ in range(n)]
        h = rng.randint(n, 15 * n)
        check(piles, h, brute(piles, h))

    print("all tests passed")
