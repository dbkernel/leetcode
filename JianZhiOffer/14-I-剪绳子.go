/*
============================================================================
LCR 131 / 剑指 Offer 14- I. 砍竹子 I（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/jian-sheng-zi-lcof/
分类：动态规划 / 贪心 · 整数拆分（最大乘积）

说明：本题原名「剪绳子」，leetcode.cn 现已改写为「砍竹子 I」：参数名为
bamboo_len、函数名为 cuttingBamboo，题面只保留「切成若干段使乘积最大」
这一层意思，示例换成 bamboo_len = 12 → 81。数学内容与原名完全一致。
============================================================================

现需要将一根长为正整数 bamboo_len 的竹子砍为若干段，每段长度均为正整数。
请返回每段竹子长度的最大乘积是多少。

示例 1：
输入: bamboo_len = 12
输出: 81

提示：
2 <= bamboo_len <= 58

注意：本题与主站 343 题相同：
https://leetcode.cn/problems/integer-break/
*/
package main

import "fmt"

/*
解法一：动态规划【非最优】

为什么慢：直接枚举「切成几段、每段多长」是整数拆分，方案数随长度指数增长
（等价于枚举所有分拆）。但只要固定了某一段的长度，剩下的部分是一个规模
更小的同类问题，重复枚举的都是同样的子问题。

换个角度：dp[i] 记「长度 i 的最大乘积」。切第一刀留下 j，剩下 i-j 有两种
命运 —— 不再切（贡献 i-j 本身）或继续切（贡献 dp[i-j]），取两者较大值。

关键点：
  · 转移必须写 max(i-j, dp[i-j])，只写 dp[i-j] 就漏掉「最后一段不切」
    的情形；只写 i-j 又漏掉后面还能切
  · j 从 1 循环到 i-1，已经覆盖所有切法；dp[1] = 1 是唯一需要的初值
  · 题目要求至少切一刀，所以答案是 dp[bamboo_len]，不能写成
    max(bamboo_len, dp[bamboo_len])
  · i 从 2 开始推，dp[i] 只依赖比它小的下标，顺序不能乱

以 bamboo_len = 4 为例：
  dp[1] = 1
  dp[2]：j=1 → 1 * max(1, dp[1]) = 1
  dp[3]：j=1 → 2，j=2 → 2 * max(1, 1) = 2 → 2
  dp[4]：j=1 → 3，j=2 → 2 * max(2, dp[2]) = 4，j=3 → 3 → 4 ✔

复杂度分析：
  时间 O(n^2)：每个长度都要试所有切点
  空间 O(n)：dp 数组
*/
func cuttingBambooDP(bambooLen int) int {
	// dp[i]：长度 i 的竹子切至少一刀后的最大乘积
	dp := make([]int, bambooLen+1)
	dp[1] = 1

	for i := 2; i <= bambooLen; i++ {
		for j := 1; j < i; j++ {
			// 剩下 i-j 这一段可以不再切，也可以继续切，取较大的
			rest := dp[i-j]
			if i-j > rest {
				rest = i - j
			}
			if cur := j * rest; cur > dp[i] {
				dp[i] = cur
			}
		}
	}
	return dp[bambooLen]
}

/*
解法二：贪心（尽可能多切 3）【最优解】

为什么慢：解法一把每个长度的每种切法都试了一遍，但「每段切多长」其实有
很强的结构：只有 2 和 3 值得作为段长，而且 3 越多越好。把这些结构算清楚，
就不需要任何表格。

换个角度：只问一个问题 —— 一段的长度取多少最划算？
  · 长度 x >= 5 的一段可以拆成 3 与 x-3，而 3(x-3) = 3x-9 >= x 当且仅当
    x >= 4.5，所以拆开不会变小 → 最优解里每段长度 <= 4
  · 段长 1 一定是浪费：把它和相邻的 a 合并成 a+1，而 a+1 > a*1
    → 最优解里没有 1
  · 4 与 2+2 的乘积都是 4，所以只需在 2 和 3 之间选
  · 三段 2 的总长是 6、乘积是 8；两段 3 的总长也是 6、乘积是 9
    → 2 最多出现两次
结论：尽可能切 3，再按余数收尾。

关键点：
  · bamboo_len 为 2 时只能切 1+1（乘积 1），为 3 时只能切 1+2（乘积 2），
    这两个必须先单独返回 —— 它们连一段都拆不出 3
  · 余数为 1 时不能直接乘 1（那等于白送一段 1），要从已切的 3 里退一个
    出来凑成 2+2，也就是乘 4、总长度减去 4
  · 余数为 2 时直接乘 2
  · 余数为 0 时一路乘 3
  · bamboo_len <= 58 时最大乘积是 3^18 * 4 = 1549681956，仍在 int 范围内，
    不需要 64 位

以 bamboo_len = 10 为例：
  10 % 3 = 1 → 先取出 4（也就是 2+2），剩下 6
  6 切成 3+3
  乘积 2 * 2 * 3 * 3 = 36 ✔

复杂度分析：
  时间 O(n)：循环每轮把长度减 3，最多 n/3 轮
  空间 O(1)：只有乘积与剩余长度两个变量
*/
func cuttingBamboo(bambooLen int) int {
	if bambooLen == 2 {
		return 1
	}
	if bambooLen == 3 {
		return 2
	}

	ans := 1
	rest := bambooLen
	if rest%3 == 1 {
		// 退一个 3 出来凑成 2+2，否则最后会剩一段长度 1
		ans = 4
		rest -= 4
	} else if rest%3 == 2 {
		ans = 2
		rest -= 2
	}

	for rest > 0 {
		ans *= 3
		rest -= 3
	}
	return ans
}

/*
解法三：记忆化搜索【非最优】

沿解法一的状态定义写递归：best(len) 返回「长度 len 这一段的最大贡献」，
它可以是整段不切（值就是 len），也可以再切一刀。
  · 与解法一的时间、空间同阶（O(n^2) / O(n)），但省掉了「哪些切法不必
    再算」的重复展开
  · 真正的收益只在 len <= 3 这一层：这三种长度作为「一段」时原样返回最赚，
    所以递归到它们就停
  · 顶层仍要特判：题目要求至少切一刀，len 为 2、3 时不能直接返回 len

复杂度分析：
  时间 O(n^2)：n 个状态，每个状态试 O(n) 个切点
  空间 O(n)：备忘录，另加 O(n) 的递归栈
*/
func cuttingBambooMemo(bambooLen int) int {
	if bambooLen <= 3 {
		return bambooLen - 1 // 2 -> 1，3 -> 2
	}

	memo := make([]int, bambooLen+1)

	// 长度 len 这一段的最大贡献：整段不切就是 len，切则取所有切法的较大值
	var best func(length int) int
	best = func(length int) int {
		if length <= 3 {
			return length
		}
		if memo[length] != 0 {
			return memo[length]
		}

		res := 0
		for cut := 1; cut < length; cut++ {
			if cur := cut * best(length-cut); cur > res {
				res = cur
			}
		}
		memo[length] = res
		return res
	}
	return best(bambooLen)
}

// 把所有切法按「段长非降」枚举一遍：乘积与顺序无关，每种切法只出现一次
func enumerate(remain, minSeg, parts int, prod int64, best *int64) {
	if remain == 0 {
		if parts >= 2 && prod > *best {
			*best = prod
		}
		return
	}
	for seg := minSeg; seg <= remain; seg++ {
		enumerate(remain-seg, seg, parts+1, prod*int64(seg), best)
	}
}

func bruteMaxProduct(bambooLen int) int {
	var best int64
	enumerate(bambooLen, 1, 0, 1, &best)
	return int(best)
}

func check(bambooLen, want int) {
	if got := cuttingBambooDP(bambooLen); got != want {
		panic(fmt.Sprintf("dp(%d) = %d, want %d", bambooLen, got, want))
	}
	if got := cuttingBamboo(bambooLen); got != want {
		panic(fmt.Sprintf("greedy(%d) = %d, want %d", bambooLen, got, want))
	}
	if got := cuttingBambooMemo(bambooLen); got != want {
		panic(fmt.Sprintf("memo(%d) = %d, want %d", bambooLen, got, want))
	}
	// 与暴力枚举对拍，三种解法都要和「真值」一致
	if got := bruteMaxProduct(bambooLen); got != want {
		panic(fmt.Sprintf("brute(%d) = %d, want %d", bambooLen, got, want))
	}
}

func main() {
	// 示例：bamboo_len = 12 → 81
	check(12, 81)

	// 边界：最小长度 2，只能切成 1+1
	check(2, 1)

	// 边界：长度 3，切成 1+2
	check(3, 2)

	// 关键点用例：余数为 1，必须退一个 3 出来凑 2+2
	check(4, 4)
	check(10, 36)
	check(16, 324)

	// 关键点用例：余数为 2，直接乘 2
	check(5, 6)
	check(8, 18)

	// 余数为 0，一路乘 3
	check(6, 9)
	check(9, 27)

	// 规模上界：3^18 * 4，验证 int 不会溢出
	check(58, 1549681956)

	// 对拍：n = 2..20 的全部切法暴力枚举
	for n := 2; n <= 20; n++ {
		check(n, bruteMaxProduct(n))
	}

	fmt.Println("all tests passed")
}
