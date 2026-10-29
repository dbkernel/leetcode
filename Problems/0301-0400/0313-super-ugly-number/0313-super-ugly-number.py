"""
============================================================================
313. 超级丑数（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/super-ugly-number/
分类：堆 · 多指针动态规划（生成有序序列）
============================================================================

超级丑数 是一个正整数，并满足其所有质因数都出现在质数数组 primes 中。

给你一个整数 n 和一个整数数组 primes ，返回第 n 个 超级丑数 。

题目数据保证第 n 个 超级丑数 在 32-bit 带符号整数范围内。

示例 1：
输入：n = 12, primes = [2,7,13,19]
输出：32
解释：给定长度为 4 的质数数组 primes = [2,7,13,19]，前 12 个超级丑数序列
为：[1,2,4,7,8,13,14,16,19,26,28,32] 。

示例 2：
输入：n = 1, primes = [2,3,5]
输出：1
解释：1 不含质因数，因此它的所有质因数都在质数数组 primes = [2,3,5] 中。

提示：
1 <= n <= 10^5
1 <= primes.length <= 100
2 <= primes[i] <= 1000
题目数据 保证 primes[i] 是一个质数
primes 中的所有值都 互不相同 ，且按 递增顺序 排列
"""
import heapq
from typing import List

# 题目保证第 n 个超级丑数不超过 32 位有符号整数的最大值
MAX_INT32 = 2**31 - 1


"""
解法一：最小堆 + 集合去重【非最优】

为什么慢：最直白的生成方式是「把当前最小的丑数分别乘每个质数，把结果丢进
一个能自动排序的容器」。为了拿到第 i 个，堆里必须先攒住所有已生成但还没
取出的候选，规模是 O(n·k)；每次取最小与插入都要 O(log(n·k))，总共
O(n·k·log(n·k))。慢在「生成了远超所需的候选」—— 第 n 个之前的每个数都会
再乘上 k 个质数，绝大多数乘出来都排在很后面，白占内存。

换个角度：与其「生成后排序」，不如「按顺序生成」。第 i 个丑数一定等于某个
更早的丑数乘以某个质数，只要为每个质数维护一个「轮到哪个丑数去乘」的指针，
就能直接按序产出。

关键点：
  · 同一个数可能被多条路径生成（例如 2*3 与 3*2），必须用集合去重，
    否则堆里会出现重复，取出的第 n 个会偏小
  · 乘出来的结果要判上界：题目保证第 n 个不超过 2^31 - 1，超过的候选
    一律不需要（Python 整数不会溢出，判上界只为省内存）
  · heapq 就是最小堆，不需要取反
  · 第 1 个丑数是 1，循环从它开始弹，弹出第 n 次的就是答案

以示例 1（n = 12, primes = [2,7,13,19]）为例：
  弹出 1，生成 2、7、13、19
  弹出 2，生成 4、14、26、38
  弹出 4，生成 8、28、52、76
  依次弹出 1,2,4,7,8,13,14,16,19,26,28,32，第 12 次弹出得到 32 ✔

复杂度分析：
  时间 O(n·k·log(n·k))：每个弹出的数都要生成 k 个候选并入堆
  空间 O(n·k)：堆与去重集合最多各存这么多个候选
"""


class Solution1:
    def nthSuperUglyNumber(self, n: int, primes: List[int]) -> int:
        heap = [1]
        seen = {1}

        cur = 1
        for _ in range(n):
            cur = heapq.heappop(heap)

            for p in primes:
                nxt = cur * p
                # 超过 int 上界的候选不可能是前 n 个之一，直接丢弃
                if nxt > MAX_INT32:
                    continue
                # 同一数值只入堆一次，否则第 n 个会被重复值顶偏
                if nxt not in seen:
                    seen.add(nxt)
                    heapq.heappush(heap, nxt)

        return cur


"""
解法二：多指针动态规划【最优解】

为什么慢：解法一的问题在于「先把候选全生成出来，再按大小挑」。实际上
第 i 个丑数只可能来自「某个更早的丑数 × 某个质数」，而每个质数对应的
那个「更早的丑数」是单调前进的，用不着把它们全放进堆里。

换个角度：给每个质数 p 配一个指针 ptr[p]，它指向 dp 里第一个「还没有被
p 乘过」的位置。第 i 个丑数就是 min over p of dp[ptr[p]] * p。选出来之后，
把所有「乘出来正好等于它」的指针都往前推一格 —— 这一步同时完成了去重。

关键点：
  · 必须一次性推进所有乘出当前最小值的指针，只推一个就会让同一个数在
    下一轮被再次产出，序列里出现重复
  · 第 0 个丑数是 1，循环从 i = 1 开始写
  · 与解法三并列最优：同为 O(n·k)、同一空间量级，本解法每轮要做两遍
    k 次乘法（一遍找最小、一遍推进指针），解法三把这一步缓存掉

以示例 1（n = 12, primes = [2,7,13,19]）为例：
  初始四个指针都指向 dp[0] = 1，候选为 2、7、13、19，最小 2 → dp[1] = 2
  只有 2 的指针前进，候选变成 dp[1]*2 = 4、7、13、19，最小 4 → dp[2] = 4
  继续下去得到 1,2,4,7,8,13,14,16,19,26,28,32，第 12 个是 32 ✔

复杂度分析：
  时间 O(n·k)：每个位置扫两遍 k 个候选
  空间 O(n + k)：dp 数组与前缀指针
"""


class Solution2:
    def nthSuperUglyNumber(self, n: int, primes: List[int]) -> int:
        k = len(primes)
        dp = [0] * n
        dp[0] = 1

        ptr = [0] * k
        for i in range(1, n):
            best = min(dp[ptr[j]] * primes[j] for j in range(k))
            dp[i] = best

            # 所有乘出 best 的指针一起前进，这就是去重
            for j in range(k):
                if dp[ptr[j]] * primes[j] == best:
                    ptr[j] += 1

        return dp[n - 1]


"""
解法三：动态规划 + 索引数组（缓存候选值）【最优解，与解法二并列】

为什么慢：解法二的思路已经正确，浪费在常数上：每轮把 k 个候选算两遍。
dp[ptr[j]] 与 primes[j] 都没变的情况下，把乘积再算一次纯属重复劳动。

换个角度：为每个质数缓存「它当前能产出的最小值」cand[j] = dp[idx[j]] * p。
每轮只比较 k 个缓存值；选中 best 之后，只把那些缓存值恰好等于 best 的项
重新计算一次。乘法的次数从每轮 2k 次降到「选中最小的那些项才需要」。

关键点：
  · cand 必须与 idx 同步维护：一旦推进了 idx[j]，立刻刷新 cand[j]，
    否则下一轮会拿旧值参与比较
  · 去重仍然靠「缓存值等于 best 的全部推进」，漏掉任何一个都会产生重复
  · 判定用相等而不是 <= ：小于 best 的候选不可能存在（best 是它们的最小值）
  · n == 1 时循环一次都不进，直接返回 dp[0] = 1

以示例 1（n = 12, primes = [2,7,13,19]）为例：
  cand = [2,7,13,19]，idx = [0,0,0,0]
  第 1 轮：best = 2 → 只有 cand[0] 命中，idx[0] = 1，cand[0] = dp[1]*2 = 4
  第 2 轮：cand = [4,7,13,19]，best = 4 → idx[0] = 2，cand[0] = 8
  继续到第 12 轮，best = 32 ✔

复杂度分析：
  时间 O(n·k)：每轮扫一遍 k 个缓存值；缓存刷新只在命中时发生
  空间 O(n + k)：dp 数组 O(n)，cand 与 idx 各 O(k)
"""


class Solution3:
    def nthSuperUglyNumber(self, n: int, primes: List[int]) -> int:
        k = len(primes)
        dp = [0] * n
        dp[0] = 1

        idx = [0] * k
        # cand[j] 是「第 idx[j] 个丑数乘 primes[j]」，初始时都乘 dp[0] = 1
        cand = list(primes)

        for i in range(1, n):
            best = min(cand)
            dp[i] = best

            # 缓存值等于 best 的项全部推进并刷新，一步完成去重
            for j in range(k):
                if cand[j] == best:
                    idx[j] += 1
                    cand[j] = dp[idx[j]] * primes[j]

        return dp[n - 1]


# ===== 测试辅助 =====
def check(n: int, primes: List[int], want: int) -> None:
    """三种解法必须给出同一个数；顺带检查与已知答案一致。"""
    got1 = Solution1().nthSuperUglyNumber(n, primes)
    got2 = Solution2().nthSuperUglyNumber(n, primes)
    got3 = Solution3().nthSuperUglyNumber(n, primes)

    assert got1 == want, f"heap({n}, {primes}) = {got1}, want {want}"
    assert got2 == want, f"dp({n}, {primes}) = {got2}, want {want}"
    assert got3 == want, f"indexed({n}, {primes}) = {got3}, want {want}"


def check_agree(n: int, primes: List[int]) -> None:
    """只比较三种解法是否一致，用于不希望手算答案的大规模用例。"""
    got1 = Solution1().nthSuperUglyNumber(n, primes)
    got2 = Solution2().nthSuperUglyNumber(n, primes)
    got3 = Solution3().nthSuperUglyNumber(n, primes)

    assert got1 == got2 == got3, f"n={n} primes={primes}: {got1} {got2} {got3}"


if __name__ == "__main__":
    # 示例 1：4 个质数，第 12 个是 32
    check(12, [2, 7, 13, 19], 32)

    # 示例 2：n = 1 时无论 primes 是什么都是 1
    check(1, [2, 3, 5], 1)

    # 边界：只有 2 一个质数，序列就是 1、2、4、8……
    check(10, [2], 512)

    # 边界：经典丑数序列（2、3、5），第 10 个是 12
    check(10, [2, 3, 5], 12)

    # 边界：primes 里含较大的质数，验证小的质数仍然主导前缀
    check(5, [2, 7, 13, 19], 8)

    # 边界：n = 2 时答案就是最小的质数
    check(2, [7, 11], 7)

    # 反直觉：两个质数都能生成 6（2*3 与 3*2），去重写错会在这里露馅
    check(5, [2, 3], 6)

    # 规模：n = 20000，10 个质数，三种解法必须一致
    check_agree(20000, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])

    print("all tests passed")
