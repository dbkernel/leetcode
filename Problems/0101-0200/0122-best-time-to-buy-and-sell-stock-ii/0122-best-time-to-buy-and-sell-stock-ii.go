/*
============================================================================
122. 买卖股票的最佳时机 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/
分类：动态规划 · 状态机 / 贪心累加涨幅

给你一个整数数组 prices ，其中 prices[i] 表示某支股票第 i 天的价格。

在每一天，你可以决定是否购买和/或出售股票。你在任何时候 最多 只能持有 一股 股票。你也可以先购买，然后在 同一天 出售。

返回 你能获得的 最大 利润 。

示例 1：
输入：prices = [7,1,5,3,6,4]
输出：7
解释：在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5 - 1 = 4 。
随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6 - 3 = 3 。
总利润为 4 + 3 = 7 。

示例 2：
输入：prices = [1,2,3,4,5]
输出：4
解释：在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5 - 1 = 4 。
总利润为 4 。

示例 3：
输入：prices = [7,6,4,3,1]
输出：0
解释：在这种情况下, 交易无法获得正利润，所以不参与交易可以获得最大利润，最大利润为 0 。

提示：
- 1 <= prices.length <= 3 * 10^4
- 0 <= prices[i] <= 10^4
*/
package main

import (
	"fmt"
	"math/rand"
)

/*
解法一：DP 状态机（收盘持股 / 收盘空仓）【最优解】

为什么慢：最直白的做法是枚举每一笔交易的买入日和卖出日。交易笔数不受
限制，数对本身就随 n 平方增长，而相邻两笔交易还要满足「先卖后买」的
先后关系，枚举组合根本压不到多项式以内。

换个角度：过去的所有交易细节只影响一件事 —— 今天收盘时手里有没有股票。
于是把「历史」压成两个数：持股时的最大收益、空仓时的最大收益。

关键点：
  · hold 表示收盘持股的最大收益，cash 表示收盘空仓的最大收益
  · hold[i] = max(hold[i-1], cash[i-1] - p[i])：用昨天的空仓收益买入
  · cash[i] = max(cash[i-1], hold[i-1] + p[i])：把昨天的持股卖出
  · 两个转移必须都读第 i-1 天的旧值，用临时变量收住再统一覆盖，
    否则「今天卖完又用今天的钱买」会被错误地算进来
  · 答案取 cash：收盘还持股等于没卖出，这部分收益不成立

手推一遍
以示例 1（prices = [7,1,5,3,6,4]）为例：
  第 1 天：cash = 0，hold = -7
  第 2 天：cash = max(0, -6) = 0，hold = max(-7, -1) = -1（改在第 2 天买）
  第 3 天：cash = max(0, 4) = 4，hold = max(-1, -5) = -1
  第 4 天：cash = max(4, 2) = 4，hold = max(-1, 1) = 1（第 2 天买、第 3
          天卖，第 4 天再用利润买入）
  第 5 天：cash = max(4, 7) = 7，hold = max(1, -2) = 1
  第 6 天：cash = max(7, 5) = 7，hold = 1 → 答案 7

复杂度分析：
  时间 O(n)：每天只做常数次比较
  空间 O(1)：只留两个状态变量

解法二：贪心，累加所有相邻上涨【最优解，与解法一并列】

为什么慢：照搬 121 的思路只做一笔交易、找最大的一段涨幅，就会漏掉
「先跌再涨」带来的第二次机会（示例 1 的 1→5 与 3→6 必须分成两笔）。

换个角度：一笔从第 a 天买到第 b 天的交易，收益 p[b] - p[a] 恰好等于
这段区间里每一对相邻差之和。既然交易次数不限，任何一天只要比前一天贵，
就能在前一天买入、当天卖出，把这天的涨幅落袋；遇到下跌就空仓跳过。

关键点：
  · 只累加 p[i] - p[i-1] > 0 的部分，负差额直接丢弃（空仓躲过下跌）
  · 拆出来的每个正向区间都是一笔真实交易，区间之间必然已经卖出，
    不会出现「同时持有两股」的违规状态
  · 同一天先卖再买不产生额外收益，所以「一段长涨势」与「逐日买卖」
    收益完全相等，贪心不吃亏

手推一遍
以示例 1（prices = [7,1,5,3,6,4]）为例：
  (7→1) 下跌，跳过
  (1→5) 涨 4，累加 4
  (5→3) 下跌，跳过
  (3→6) 涨 3，累加 7
  (6→4) 下跌，跳过 → 答案 7

复杂度分析：
  时间 O(n)：扫描一遍相邻差
  空间 O(1)：只用一个累加器

与解法一的关系：两解时间同为 O(n)、空间同为 O(1)，并列最优。取舍是
贪心代码更短；DP 更通用 —— 122 加上手续费或冷冻期后只剩改转移方程
一条路，而「累加涨幅」的贪心在那些题里不再成立。
*/

// 解法一：DP 状态机
func maxProfit(prices []int) int {
	// 收盘空仓 / 收盘持股的最大收益，买入的支出从 cash 里扣
	cash := 0
	hold := -prices[0]
	for i := 1; i < len(prices); i++ {
		// 先按昨天的状态算出两个新值，再一起覆盖，避免自引用
		nextCash := max(cash, hold+prices[i])
		nextHold := max(hold, cash-prices[i])
		cash = nextCash
		hold = nextHold
	}
	// 收盘还持股等于没卖，收益不落袋
	return cash
}

// 解法二：贪心累加相邻上涨
func maxProfitGreedy(prices []int) int {
	total := 0
	for i := 1; i < len(prices); i++ {
		// 只吃上涨段：跌的日子空仓跳过，总收益不会被拉低
		if prices[i] > prices[i-1] {
			total += prices[i] - prices[i-1]
		}
	}
	return total
}

// 对照用 O(n^2) DP：dp[i] 是前 i 天的最大收益，枚举最后一笔的买入日
func maxProfitBrute(prices []int) int {
	n := len(prices)
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = dp[i-1]
		// 第 j 天买入、第 i-1 天卖出，前 j 天的最优收益已知
		for j := 0; j < i; j++ {
			if v := dp[j] + prices[i-1] - prices[j]; v > dp[i] {
				dp[i] = v
			}
		}
	}
	return dp[n]
}

func check(prices []int, want int) {
	if got := maxProfit(prices); got != want {
		panic(fmt.Sprintf("dp(%v) = %d, want %d", prices, got, want))
	}
	if got := maxProfitGreedy(prices); got != want {
		panic(fmt.Sprintf("greedy(%v) = %d, want %d", prices, got, want))
	}
	if got := maxProfitBrute(prices); got != want {
		panic(fmt.Sprintf("brute(%v) = %d, want %d", prices, got, want))
	}
}

// 三路对拍：暴力 DP 是定义式，状态机若漏掉「先卖后买」会在这里挂
func randomCheck() {
	rng := rand.New(rand.NewSource(20240102))
	for t := 0; t < 3000; t++ {
		n := 1 + rng.Intn(22)
		prices := make([]int, n)
		for i := range prices {
			prices[i] = rng.Intn(21)
		}
		want := maxProfitBrute(prices)
		if maxProfit(prices) != want || maxProfitGreedy(prices) != want {
			panic(fmt.Sprintf("mismatch on %v", prices))
		}
	}
}

func main() {
	// 示例 1：两段上涨，中间必须卖出才能换手
	check([]int{7, 1, 5, 3, 6, 4}, 7)

	// 示例 2：单调上涨，整段持有与逐日买卖等价
	check([]int{1, 2, 3, 4, 5}, 4)

	// 示例 3：单调下跌，不交易
	check([]int{7, 6, 4, 3, 1}, 0)

	// 边界：只有一天，没有可卖的第二日
	check([]int{5}, 0)

	// 边界：两天一涨一跌，只吃第一段
	check([]int{1, 5}, 4)

	// 反直觉：先跌后涨，不能拿前面更高的价格当买入价
	check([]int{9, 1, 5}, 4)

	// 边界：价格全相同，涨跌都为零
	check([]int{4, 4, 4, 4}, 0)

	// 边界：锯齿形，答案等于每个峰谷差之和
	check([]int{1, 3, 2, 5, 4, 7}, 8)

	// 边界：规模上限，只跑两个 O(n) 解
	prices := make([]int, 30000)
	for i := range prices {
		prices[i] = i * 37 % 10001
	}
	if maxProfit(prices) != maxProfitGreedy(prices) {
		panic("large case mismatch")
	}

	randomCheck()

	fmt.Println("all tests passed")
}
