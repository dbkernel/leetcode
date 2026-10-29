/*
============================================================================
64. 最小路径和（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/minimum-path-sum/
分类：动态规划 · 网格最短路径和

给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

说明：每次只能向下或者向右移动一步。

示例 1：
输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
输出：7
解释：因为路径 1→3→1→1→1 的总和最小。

示例 2：
输入：grid = [[1,2,3],[4,5,6]]
输出：12

提示：
- m == grid.length
- n == grid[i].length
- 1 <= m, n <= 200
- 0 <= grid[i][j] <= 200
*/
package main

import "fmt"

/*
解法一：二维 DP【非最优】

为什么慢：把每条路径都走一遍，路径数与 C(m+n-2, m-1) 同阶，200x200 时
是天文数字；而且大量路径共享同一段后缀，重复计算完全无法复用。

换个角度：每条路径的最后一步只能从上方 (i-1,j) 或左方 (i,j-1) 走进
(i,j)，所以"到 (i,j) 的最小和"就是这两处较小者加上 grid[i][j]。状态
dp[i][j] 只和上一行、本行左侧有关。

关键点：
  · 第一行只能从左边来、第一列只能从上边来，要单独按前缀和初始化
  · 其余格子取 min(上方, 左方)，这里是 min 不是 max，写反必错
  · 网格非负，所以不需要考虑"走远路反而更小"的情况，递推方向无歧义
  · 答案在 dp[m-1][n-1]，不是 dp 的最小值

手推一遍：以示例 1（grid = [[1,3,1],[1,5,1],[4,2,1]]）为例：
  第一行 → [1,4,5]
  第一列 → 1+1=2，2+4=6
  中心 (1,1)：min(4,2)+5 = 7；(1,2)：min(5,7)+1 = 6
  末行：dp[2][1] = min(7,6)+2 = 8；dp[2][2] = min(6,8)+1 = 7
  → 答案 7 ✔

复杂度分析：
  时间 O(m*n)：每个格子一次转移
  空间 O(m*n)：完整保存 m 行；题目只要右下角的值，这些中间结果其实用不上
*/
func minPathSum2D(grid [][]int) int {
	m := len(grid)
	n := len(grid[0])
	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
	}

	dp[0][0] = grid[0][0]
	// 第一行只能一路向右，是前缀和
	for j := 1; j < n; j++ {
		dp[0][j] = dp[0][j-1] + grid[0][j]
	}
	// 第一列只能一路向下，同样是前缀和
	for i := 1; i < m; i++ {
		dp[i][0] = dp[i-1][0] + grid[i][0]
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			best := dp[i-1][j]
			if dp[i][j-1] < best {
				best = dp[i][j-1]
			}
			dp[i][j] = best + grid[i][j]
		}
	}
	return dp[m-1][n-1]
}

/*
解法二：一维滚动数组【最优解】

为什么慢：解法一保留了整张表，可 dp[i][j] 只读 dp[i-1][j]（上一行同列）
和 dp[i][j-1]（本行左邻）。等第 i 行算完，第 i-2 行及更早的数据再也用不到，
O(m*n) 的空间里有一整片是垃圾。

换个角度：把行号压掉，只留一维 dp[j]。在覆盖 dp[j] 之前它正好是"上一行
同列"的值，而 dp[j-1] 已经在本次扫描里被更新成"本行左邻"，两个前驱都在。

关键点：
  · 内层 j 必须从小到大：dp[j-1] 要先被本行更新，才能当左邻用
  · 覆盖 dp[0] 之前先加上 grid[i][0]：第一列只能从上边来
  · 第一行要单独初始化成前缀和，否则 dp[j-1] 是 0 会把答案拉低
  · 不要直接改写 grid：题目给的是输入，覆盖它会改变调用方的数据

手推一遍：以示例 1（grid = [[1,3,1],[1,5,1],[4,2,1]]）为例：
  第一行 → dp = [1,4,5]
  第二行：dp[0] = 1+1 = 2；dp[1] = min(4,2)+5 = 7；dp[2] = min(5,7)+1 = 6
  第三行：dp[0] = 2+4 = 6；dp[1] = min(7,6)+2 = 8；dp[2] = min(6,8)+1 = 7
  → 答案 7 ✔

复杂度分析：
  时间 O(m*n)：每个格子一次转移，与解法一相同
  空间 O(n)：只保留一行；若按列方向滚动则压到 O(min(m,n))
*/
func minPathSum(grid [][]int) int {
	m := len(grid)
	n := len(grid[0])

	// dp[j]：到达当前行第 j 列的最小路径和，先铺成第一行的前缀和
	dp := make([]int, n)
	dp[0] = grid[0][0]
	for j := 1; j < n; j++ {
		dp[j] = dp[j-1] + grid[0][j]
	}

	for i := 1; i < m; i++ {
		// 第一列没有左邻，只能承接上一行
		dp[0] += grid[i][0]

		for j := 1; j < n; j++ {
			// dp[j] 此时仍是上一行，dp[j-1] 已是本行左邻
			best := dp[j]
			if dp[j-1] < best {
				best = dp[j-1]
			}
			dp[j] = best + grid[i][j]
		}
	}
	return dp[n-1]
}

// 暴力参考解：把所有路径走一遍取最小和，只适合小网格
func bruteMin(grid [][]int, i int, j int) int {
	m := len(grid)
	n := len(grid[0])
	if i == m-1 && j == n-1 {
		return grid[i][j]
	}

	best := int(^uint(0) >> 1) // 最大 int，作为"还没有候选"的哨兵
	if i+1 < m {
		if v := bruteMin(grid, i+1, j); v < best {
			best = v
		}
	}
	if j+1 < n {
		if v := bruteMin(grid, i, j+1); v < best {
			best = v
		}
	}
	return best + grid[i][j]
}

// 两种解法必须给出同一个答案
func check(grid [][]int, want int) {
	if got := minPathSum2D(grid); got != want {
		panic(fmt.Sprintf("2d = %d, want %d", got, want))
	}
	if got := minPathSum(grid); got != want {
		panic(fmt.Sprintf("rolling = %d, want %d", got, want))
	}
}

// 小网格额外走一遍暴力枚举；大网格的路径数太多，只做解法间比对
func checkBrute(grid [][]int) {
	check(grid, bruteMin(grid, 0, 0))
}

func main() {
	// 示例 1
	check([][]int{{1, 3, 1}, {1, 5, 1}, {4, 2, 1}}, 7)

	// 示例 2
	check([][]int{{1, 2, 3}, {4, 5, 6}}, 12)

	// 边界：单行，只能一路向右，答案就是整行之和
	check([][]int{{1, 2, 3}}, 6)

	// 边界：单列，只能一路向下
	check([][]int{{1}, {2}, {3}}, 6)

	// 边界：单格
	check([][]int{{5}}, 5)

	// 边界：全 0，结果是 0 而不是"没有路径"
	check([][]int{{0, 0}, {0, 0}}, 0)

	// 边界：贪心"每步取较小邻格"会先撞进 9，必须靠 DP 比较两条来路
	check([][]int{{1, 9, 1}, {1, 9, 1}, {1, 1, 1}}, 5)

	// 边界：数值全为 200，200x200 的结果仍在 int 内
	big := make([][]int, 200)
	for i := range big {
		big[i] = make([]int, 200)
		for j := range big[i] {
			big[i][j] = 200
		}
	}
	check(big, 200*(200+200-1))

	// 随机对拍：1..5 的网格、取值 0..9，逐组与暴力枚举比对
	seed := uint32(20260810)
	next := func(mod uint32) int {
		seed = seed*1103515245 + 12345
		return int((seed >> 16) % mod)
	}
	for t := 0; t < 60; t++ {
		m := 1 + next(5)
		n := 1 + next(5)
		grid := make([][]int, m)
		for i := range grid {
			grid[i] = make([]int, n)
			for j := range grid[i] {
				grid[i][j] = next(10)
			}
		}
		checkBrute(grid)
	}

	fmt.Println("all tests passed")
}
