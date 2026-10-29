"""
============================================================================
LCR 127 / 剑指 Offer 10- II. 跳跃训练（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/qing-wa-tiao-tai-jie-wen-ti-lcof/
分类：动态规划 · 滚动数组（线性递推）

今天的有氧运动训练内容是在一个长条形的平台上跳跃。平台有 num 个小格子，每次可以选择
跳 一个格子 或者 两个格子。请返回在训练过程中，学员们共有多少种不同的跳跃方式。

结果可能过大，因此结果需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，
请返回 1。

示例 1：
输入：n = 2
输出：2

示例 2：
输入：n = 5
输出：8

提示：
0 <= n <= 100

注意：本题与主站 70 题相同：https://leetcode.cn/problems/climbing-stairs/
"""


"""
解法：动态规划 + 滚动数组【最优解】

本题在 leetcode.cn 上没有官方题解文章（它与主站 70「爬楼梯」是同一道题），标准做法
就是动态规划：时间 O(n)、空间 O(1)，也是本题的最优解。

为什么慢：直接递归枚举所有跳跃方案会重复展开大量相同子问题，调用次数随 n 指数增长，
n 到 30 以上就明显卡住。

换个角度：把注意力放到「最后一步」上 —— 走到第 i 格，要么是从第 i-1 格跳一格上来，
要么是从第 i-2 格跳两格上来，没有第三种可能，于是：
  f(i) = f(i-1) + f(i-2)
边界上，站着不动也算一种方案，所以 f(0) = 1；只有一个格子时 f(1) = 1。

关键点：
  · 边界容易错：f(0) = 1（空方案算 1 种），f(1) = 1；示例 n = 2 得 2 正好验证
  · num = 0 时直接返回 1，不要进入循环
  · 只依赖前两项 → 三个变量滚动即可，不必开长度为 n+1 的数组
  · 每步取模 1e9+7，避免溢出
  · 这题与斐波那契的区别只在初值：这里 f(0) = 1，斐波那契 F(0) = 0

以 n = 5 为例：
  初值 p = 1（f(0)）、q = 1（f(1)）
  i=2：r = 2 → 往后滚动，p = 1, q = 2
  i=3：r = 3
  i=4：r = 5
  i=5：r = 8
  返回 8，与示例一致

复杂度分析：
  时间 O(n)：一次循环若干次加法
  空间 O(1)：只用三个变量
"""

MOD = 1000000007


class Solution:
    def trainWays(self, num: int) -> int:
        # f(0) = 1（什么都不跳也算一种方案），f(1) = 1
        if num < 2:
            return 1

        p, q, r = 1, 1, 1
        for _ in range(2, num + 1):
            r = (p + q) % MOD
            p = q
            q = r
        return r


def check(num: int, want: int) -> None:
    got = Solution().trainWays(num)
    assert got == want, f"trainWays({num}) = {got}, want {want}"


if __name__ == "__main__":
    # 示例 1
    check(2, 2)

    # 示例 2
    check(5, 8)

    # 边界：0 个格子，站着不动算一种方案
    check(0, 1)

    # 边界：1 个格子，只有一种跳法
    check(1, 1)

    # 常规值
    check(3, 3)
    check(7, 21)
    check(10, 89)

    # 边界：开始取模的规模
    check(45, 836311896)
    check(50, 365010934)
    check(100, 782204094)

    # 交叉验证：用长度为 num + 1 的 DP 数组算一遍
    dp = [1] * 101
    for i in range(2, 101):
        dp[i] = (dp[i - 1] + dp[i - 2]) % MOD
    for n in range(101):
        assert Solution().trainWays(n) == dp[n], f"trainWays({n}) 与 DP 数组结果不一致"

    print("all tests passed")
