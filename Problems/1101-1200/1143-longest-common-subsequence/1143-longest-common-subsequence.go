/*
============================================================================
1143. 最长公共子序列（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/longest-common-subsequence/
分类：动态规划 · 双序列 DP

给定两个字符串 text1 和 text2，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。

一个字符串的 子序列 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。

- 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。

两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。

示例 1：
输入：text1 = "abcde", text2 = "ace"
输出：3
解释：最长公共子序列是 "ace" ，它的长度为 3 。

示例 2：
输入：text1 = "abc", text2 = "abc"
输出：3
解释：最长公共子序列是 "abc" ，它的长度为 3 。

示例 3：
输入：text1 = "abc", text2 = "def"
输出：0
解释：两个字符串没有公共子序列，返回 0 。

提示：
- 1 <= text1.length, text2.length <= 1000
- text1 和 text2 仅由小写英文字符组成。
*/
package main

import (
	"fmt"
	"strings"
)

/*
解法一：二维 DP【非最优】

为什么慢：枚举 text1 的所有子序列再逐个到 text2 里找，是 2^m 量级；
即便只看两个前缀的公共部分，同一对前缀也会从不同路径重复到达。

换个角度：把"两个前缀"当成状态。dp[i][j] 表示 text1 前 i 个字符与
text2 前 j 个字符的 LCS 长度。末尾字符相同时它们一定可以配对，不同时
至少有一个末尾字符不在答案里，于是转移到两处之一。

关键点：
  · 下标从 1 开始，dp[0][*] 与 dp[*][0] 全为 0，正好表示"空前缀"
  · 字符相等时接的是 dp[i-1][j-1]+1，不是 dp[i-1][j] 或 dp[i][j-1]
  · 不相等时取 max(上, 左)，两条路都必须算，不能只挑一条
  · 返回 dp[m][n]；不是 max 整张表，空前缀已经覆盖了"没有公共子序列"

手推一遍：以示例 1（text1 = "abcde", text2 = "ace"）为例：
  a 与 a 命中：dp[1][1] = 1
  b、c 与 a 不匹配，dp[2][1] = dp[3][1] = 1
  c 与 c 命中：dp[3][2] = dp[2][1] + 1 = 2
  d、e 与 c 保持 2；e 与 e 命中：dp[5][3] = dp[4][2] + 1 = 3
  → 答案 3 ✔

复杂度分析：
  时间 O(m*n)：每对前缀算一次
  空间 O(m*n)：整张表都留着，但每个格子只被右下方的格子用到
*/
func longestCommonSubsequence2D(text1 string, text2 string) int {
	m, n := len(text1), len(text2)
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if text1[i-1] == text2[j-1] {
				// 末尾字符相同，让它们配对一定不亏
				dp[i][j] = dp[i-1][j-1] + 1
			} else {
				// 至少有一个末尾字符不属于答案
				dp[i][j] = dp[i-1][j]
				if dp[i][j-1] > dp[i][j] {
					dp[i][j] = dp[i][j-1]
				}
			}
		}
	}
	return dp[m][n]
}

/*
解法二：记忆化搜索【非最优】

为什么慢：直接写 dfs(i, j) 自顶向下，同一对 (i, j) 会被多条路径反复
到达，最坏指数级；不记忆就等于把同一张表的格子重算了很多遍。

换个角度：状态与解法一完全同构，只是把填表顺序反过来——先问终点的答案，
递归去要子问题的答案，算过一次就记下来。

关键点：
  · memo 的"未计算"必须用 -1 之类的标记，不能用 0：LCS 为 0 是合法结果
  · 字符相等时只递归 (i-1, j-1)，不必再试 (i-1,j) 与 (i,j-1)，配对不会更差
  · i == 0 或 j == 0 是递归出口，返回 0
  · 递归深度最坏 m+n（一路斜着走会更快），1000+1000 在 Go 的可增长栈上安全
  · 空间比解法三多一倍：memo 是 O(m*n)，另有递归栈 O(m+n)

手推一遍：以示例 1（text1 = "abcde", text2 = "ace"）为例：
  dfs(5,3)：e == e → 1 + dfs(4,2)
  dfs(4,2)：d != c → max(dfs(3,2), dfs(4,1))
  dfs(3,2)：c == c → 1 + dfs(2,1)；dfs(2,1)：b != a → max(dfs(1,1), 0) = 1
  dfs(4,1)：d != a → dfs(3,1) = 1
  → 1 + max(2, 1) = 3 ✔

复杂度分析：
  时间 O(m*n)：每个状态最多算一次
  空间 O(m*n)：memo 表，另有 O(m+n) 的递归栈
*/
func longestCommonSubsequenceMemo(text1 string, text2 string) int {
	m, n := len(text1), len(text2)
	// -1 表示没算过；0 是合法答案，不能拿来当哨兵
	memo := make([][]int, m+1)
	for i := range memo {
		memo[i] = make([]int, n+1)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(i int, j int) int
	dfs = func(i int, j int) int {
		if i == 0 || j == 0 {
			return 0
		}
		if memo[i][j] >= 0 {
			return memo[i][j]
		}

		var res int
		if text1[i-1] == text2[j-1] {
			res = dfs(i-1, j-1) + 1
		} else {
			a, b := dfs(i-1, j), dfs(i, j-1)
			if a > b {
				res = a
			} else {
				res = b
			}
		}

		memo[i][j] = res
		return res
	}
	return dfs(m, n)
}

/*
解法三：一维滚动数组【最优解】

为什么慢：解法一保留整张表，可 dp[i][j] 只读 dp[i-1][j]、dp[i-1][j-1]
和 dp[i][j-1]。算完第 i 行，第 i-2 行及更早的数据再也用不到，O(m*n)
的空间里绝大多数是垃圾。

换个角度：抹掉行号，只留一维 dp[j]。覆盖 dp[j] 之前它是 dp[i-1][j]；
dp[j-1] 已被本轮更新成 dp[i][j-1]；还缺一个 dp[i-1][j-1]，用一个变量
prev 随身带着即可。

关键点：
  · 先把较短的串换到"列"的位置，数组长度才是 min(m,n)，而不是 m 或 n
  · prev 存的是 dp[i-1][j-1]，必须在 dp[j] 被覆盖之前把它取走
  · 字符相等时只能写 prev + 1，写 dp[j] + 1 会把同一对字符重复用一次
  · 不等时 dp[j] = max(旧 dp[j], dp[j-1])，顺序是从左到右
  · 空串直接得 0：dp 长度是 n+1，外层循环一次都不进

手推一遍：以示例 1（text1 = "abcde", text2 = "ace"）为例（列为 "ace"）：
  处理 'a'：dp = [0,1,1,1]
  处理 'b'：dp 不变 [0,1,1,1]
  处理 'c'：dp = [0,1,2,2]
  处理 'd'：dp 不变 [0,1,2,2]
  处理 'e'：dp = [0,1,2,3] → 答案 3 ✔

复杂度分析：
  时间 O(m*n)：每个字符对算一次，与解法一相同
  空间 O(min(m,n))：只有长度 min(m,n)+1 的一行，外加常量个标量
*/
func longestCommonSubsequence(text1 string, text2 string) int {
	m, n := len(text1), len(text2)

	// 让 text2 成为较短的串，滚动数组的长度就是 min(m, n)
	if n > m {
		text1, text2 = text2, text1
		m, n = n, m
	}

	dp := make([]int, n+1)

	for i := 1; i <= m; i++ {
		// prev：dp[i-1][j-1]，在 dp[j] 被覆盖前保存下来
		prev := 0
		for j := 1; j <= n; j++ {
			up := dp[j]
			if text1[i-1] == text2[j-1] {
				dp[j] = prev + 1
			} else {
				best := up
				if dp[j-1] > best {
					best = dp[j-1]
				}
				dp[j] = best
			}
			prev = up
		}
	}
	return dp[n]
}

// 暴力参考解：不带记忆的递归，只适合短串
func bruteLCS(a string, b string, i int, j int) int {
	if i == 0 || j == 0 {
		return 0
	}
	if a[i-1] == b[j-1] {
		return bruteLCS(a, b, i-1, j-1) + 1
	}
	x, y := bruteLCS(a, b, i-1, j), bruteLCS(a, b, i, j-1)
	if x > y {
		return x
	}
	return y
}

// 三种解法必须给出同一个答案
func check(text1 string, text2 string, want int) {
	got := []int{
		longestCommonSubsequence2D(text1, text2),
		longestCommonSubsequenceMemo(text1, text2),
		longestCommonSubsequence(text1, text2),
	}
	for _, g := range got {
		if g != want {
			panic(fmt.Sprintf("lcs(%q,%q) = %d, want %d", text1, text2, g, want))
		}
	}

	// 短串再与无记忆的暴力递归对拍；长串的暴力是指数级，只做解法间比对
	if len(text1) <= 12 && len(text2) <= 12 {
		if got := bruteLCS(text1, text2, len(text1), len(text2)); got != want {
			panic(fmt.Sprintf("brute(%q,%q) = %d, want %d", text1, text2, got, want))
		}
	}
}

func main() {
	// 示例 1
	check("abcde", "ace", 3)

	// 示例 2：完全相同
	check("abc", "abc", 3)

	// 示例 3：没有任何公共字符
	check("abc", "def", 0)

	// 边界：空串（题面约束至少 1 个字符，这里只验证健壮性）
	check("", "", 0)
	check("", "abc", 0)
	check("abc", "", 0)

	// 边界：单字符相等 / 不等
	check("a", "a", 1)
	check("a", "b", 0)

	// 边界：长短悬殊，短的整个是长的子序列
	check("abcdefgh", "aeh", 3)

	// 边界：顺序颠倒不算公共子序列
	check("ab", "ba", 1)

	// 边界：参数顺序对调后答案不变
	check("ace", "abcde", 3)

	// 边界：1000 长度的串，压测滚动数组与记忆化搜索的栈深
	check(strings.Repeat("a", 1000), strings.Repeat("a", 1000), 1000)

	// 随机对拍：长度 0..8、字母表 {a,b,c}，逐组与暴力递归比对
	seed := uint32(20260812)
	next := func(mod uint32) int {
		seed = seed*1103515245 + 12345
		return int((seed >> 16) % mod)
	}
	for t := 0; t < 60; t++ {
		m := next(9)
		n := next(9)
		x := make([]byte, m)
		y := make([]byte, n)
		for i := range x {
			x[i] = byte('a' + next(3))
		}
		for j := range y {
			y[j] = byte('a' + next(3))
		}
		check(string(x), string(y), bruteLCS(string(x), string(y), m, n))
	}

	fmt.Println("all tests passed")
}
