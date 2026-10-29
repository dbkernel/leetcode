/*
============================================================================
LCR 130 / 剑指 Offer 13. 衣橱整理（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/ji-qi-ren-de-yun-dong-fan-wei-lcof/
分类：网格 · 深度优先搜索（可达格子计数）
说明：本题即《剑指 Offer（第 2 版）》第 13 题「机器人的运动范围」。力扣
      现行题面改名为「衣橱整理」，并把移动方向限定为向右 / 向下，签名
      相应改为 wardrobeFinishing(m, n, cnt)（原为 movingCount）。

家居整理师将待整理衣橱划分为 m x n 的二维矩阵 grid，其中 grid[i][j]
代表一个需要整理的格子。整理师自 grid[0][0] 开始 逐行逐列 地整理每个
格子。

整理规则为：在整理过程中，可以选择 向右移动一格 或 向下移动一格，但不能
移动到衣柜之外。同时，不需要整理 digit(i) + digit(j) > cnt 的格子，其中
digit(x) 表示数字 x 的各数位之和。

请返回整理师 总共需要整理多少个格子。

示例 1：
输入：m = 4, n = 7, cnt = 5
输出：18

提示：
- 1 <= n, m <= 100
- 0 <= cnt <= 20
*/
package main

import "fmt"

// 数位和递推表：sum[x] = sum[x/10] + x%10，判定因此是 O(1)
func buildDigitSums(size int) []int {
	sum := make([]int, size)
	for x := 1; x < size; x++ {
		sum[x] = sum[x/10] + x%10
	}
	return sum
}

/*
解法一：DFS（只走右 / 下）【最优解】

为什么慢：不搜索就只能"逐行逐列"直接数格子，但满足 digit 约束的格子未必
与 (0,0) 连通——某个数位和超限的格子会把它右下方的整片格子挡在外面，
只看坐标算 digit 就会把不可达的格子也算进去。

换个角度：整理师只能右移或下移，所以"能否整理到 (i,j)"等价于"从 (0,0)
出发、只走右 / 下、且途经每格的 digit 和都不超过 cnt 能否到达 (i,j)"。
从 (0,0) 做一次图遍历，只往右、下两个方向扩展，访问到的格子数就是答案。

关键点：
  · 方向只有 (0,+1) 与 (+1,0)；写成四方向虽然答案一样，但与题面规则
    不符，也会白走回头路
  · 进入格子时立刻标记 visited 再扩展，否则同一格会被上方和左方各压一次
  · digit 和用递推表：sum[x] = sum[x/10] + x%10，先算好行、列两张表，
    每格的判定从 O(log) 降到 O(1)
  · 先判越界再判 visited，最后判 digit，顺序反过来会让下标越界
  · cnt = 0 时只有 (0,0) 可达（digit(0) + digit(0) = 0），无需特判
  · 递归深度只有 m + n - 1 ≤ 199，不必担心爆栈
  · Go 用闭包递归（先声明再赋值）捕获 visited 与两张 digit 表，
    省掉为解法单独定义结构体

手推一遍：以示例 1（m=4, n=7, cnt=5）为例：
  第 0 行 digit(i)=0，列 digit(0..6)=0..6 → j ≤ 5，6 格可达
  第 1 行 digit(i)=1 → j ≤ 4，5 格；第 2 行 j ≤ 3，4 格；第 3 行 3 格
  合计 6 + 5 + 4 + 3 = 18 ✔

复杂度分析：
  时间 O(mn)：每格至多进入一次，digit 表预处理 O(m + n)
  空间 O(mn)：visited 表；递归栈只有 O(m + n)

解法二：BFS（只把右 / 下入队）【最优解，与解法一并列】

把解法一的递归栈换成队列：出队时计数，并把右、下两个合法邻居入队。
  · 时间同为 O(mn)，空间同为 O(mn)
  · 取舍：BFS 的队列最坏装 O(mn) 个格子，但调用栈只有一层；DFS 反过来，
    递归栈 O(m + n) 而不用队列。两者同阶，并列最优
  · 入队时就要标记 visited，不能等出队再标记，否则同一格会被上方和左方
    重复入队，队列膨胀

复杂度分析：
  时间 O(mn)
  空间 O(mn)：visited 表加队列

解法三：按行递推（官方方法二）【最优解，与解法一、二并列】

既然只能右移、下移，任何到达 (i,j) 的路径最后一步只可能来自 (i-1,j) 或
(i,j-1)，于是"可达"具有最优子结构：
  reach[i][j] = 合法(i,j) && (reach[i-1][j] || reach[i][j-1])
  · 合法(i,j) 指 digit(i) + digit(j) ≤ cnt；第一行只能来自左方，
    第一列只能来自上方，(0,0) 恒为真
  · 按行、列顺序填表，每个格子的值只依赖已经算过的上方和左方
  · 与两个搜索解时间同为 O(mn)、空间同为 O(mn)（官方实现保留整张可达
    表），三者并列最优
  · 递推没有递归深度与队列开销，常数最小，格子规模大时最稳

复杂度分析：
  时间 O(mn)：每个状态 O(1) 转移
  空间 O(mn)：记录每格是否可达
*/

// 解法一：DFS，只走右 / 下
func wardrobeFinishing(m int, n int, cnt int) int {
	rowSum := buildDigitSums(m)
	colSum := buildDigitSums(n)
	visited := make([][]bool, m)
	for i := range visited {
		visited[i] = make([]bool, n)
	}

	// 返回从 (i,j) 出发还能整理到多少格（含 (i,j) 自己）
	var dfs func(i, j int) int
	dfs = func(i, j int) int {
		if i >= m || j >= n || visited[i][j] {
			return 0
		}
		// digit 超限的格子不整理，也不再往后传：它后面只能经它到达
		if rowSum[i]+colSum[j] > cnt {
			return 0
		}
		visited[i][j] = true
		return 1 + dfs(i+1, j) + dfs(i, j+1)
	}
	return dfs(0, 0)
}

// 解法二：BFS，只把右 / 下入队
func wardrobeFinishingBFS(m int, n int, cnt int) int {
	rowSum := buildDigitSums(m)
	colSum := buildDigitSums(n)
	visited := make([][]bool, m)
	for i := range visited {
		visited[i] = make([]bool, n)
	}

	visited[0][0] = true
	queue := [][2]int{{0, 0}}
	count := 0
	for head := 0; head < len(queue); head++ {
		i, j := queue[head][0], queue[head][1]
		count++

		// 两条边各自判一次，入队前标记，避免被两个前驱重复入队
		if j+1 < n && !visited[i][j+1] && rowSum[i]+colSum[j+1] <= cnt {
			visited[i][j+1] = true
			queue = append(queue, [2]int{i, j + 1})
		}
		if i+1 < m && !visited[i+1][j] && rowSum[i+1]+colSum[j] <= cnt {
			visited[i+1][j] = true
			queue = append(queue, [2]int{i + 1, j})
		}
	}
	return count
}

// 解法三：按行递推
func wardrobeFinishingDP(m int, n int, cnt int) int {
	rowSum := buildDigitSums(m)
	colSum := buildDigitSums(n)
	reach := make([][]bool, m)
	for i := range reach {
		reach[i] = make([]bool, n)
	}

	count := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if rowSum[i]+colSum[j] > cnt {
				continue
			}
			if i == 0 && j == 0 {
				reach[i][j] = true // 起点恒可达，digit(0)=0 ≤ cnt
			} else {
				fromUp := i > 0 && reach[i-1][j]
				fromLeft := j > 0 && reach[i][j-1]
				reach[i][j] = fromUp || fromLeft
			}
			if reach[i][j] {
				count++
			}
		}
	}
	return count
}

// ===== 测试辅助：三解法对照 =====
func check(m int, n int, cnt int, want int) {
	if got := wardrobeFinishing(m, n, cnt); got != want {
		panic(fmt.Sprintf("dfs(%d,%d,%d) = %d, want %d", m, n, cnt, got, want))
	}
	if got := wardrobeFinishingBFS(m, n, cnt); got != want {
		panic(fmt.Sprintf("bfs(%d,%d,%d) = %d, want %d", m, n, cnt, got, want))
	}
	if got := wardrobeFinishingDP(m, n, cnt); got != want {
		panic(fmt.Sprintf("dp(%d,%d,%d) = %d, want %d", m, n, cnt, got, want))
	}
}

func main() {
	// 示例 1
	check(4, 7, 5, 18)

	// 边界：只有起点一格，cnt 取最小和最大都不影响答案
	check(1, 1, 0, 1)
	check(1, 1, 20, 1)

	// 边界：2×2，cnt = 0 只能停在起点，cnt = 1 能走到两个邻居
	check(2, 2, 0, 1)
	check(2, 2, 1, 3)
	// 边界：cnt 足够大时整张图都能整理
	check(2, 2, 20, 4)

	// 边界：cnt = 2 时 (2,2) 的 digit 和为 4，被挡在外面
	check(3, 3, 2, 6)

	// 反直觉：数位和超限的格子会隔断后方，不能只按公式数格子
	check(16, 16, 6, 28)

	// 边界：规模放大，验证递推与两条搜索路径一致
	check(20, 20, 11, 255)
	check(100, 100, 20, 6117)

	fmt.Println("all tests passed")
}
