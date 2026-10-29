/*
============================================================================
200. 岛屿数量（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/number-of-islands/
分类：网格 · 深度优先搜索（淹岛）

给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中
岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的
陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。

示例 1：
输入：grid = [
  ['1','1','1','1','0'],
  ['1','1','0','1','0'],
  ['1','1','0','0','0'],
  ['0','0','0','0','0']
]
输出：1

示例 2：
输入：grid = [
  ['1','1','0','0','0'],
  ['1','1','0','0','0'],
  ['0','0','1','0','0'],
  ['0','0','0','1','1']
]
输出：3

提示：
- m == grid.length
- n == grid[i].length
- 1 <= m, n <= 300
- grid[i][j] 的值为 '0' 或 '1'
*/
package main

import "fmt"

/*
解法一：DFS 淹岛【最优解】

为什么慢：把"岛屿"翻译成"任意两块陆地是否连通"，就要对每一对陆地
判断一次，O(mn) 个格子两两比较是 O((mn)^2)；即便想到从一个陆地出发
扩散，如果只做标记而不敢改图，扩散结束后仍要再扫一遍找下一块起点，
多出一整轮全图扫描。

换个角度：既然一整块岛屿最终只记 1，它的形状就不必保留。每发现一个
还没被淹掉的 '1'，就地把它和它能到达的所有 '1' 全改成 '0'。此后网格里
剩下的 '1' 只可能是尚未计数的岛，扫到的每个 '1' 都是一块新岛。

关键点：
  · 计数和淹没必须绑死：扫到 '1' 先 count++，紧接着从这格淹完整块
  · 淹的动作放在进入递归之前（先置 '0' 再扩散），否则同一格会被多个
    邻居重复进入，复杂度退化
  · 相邻只看上下左右四向，对角相邻不算连通
  · 用改 grid 代替 visited 表：题面允许原地修改，省掉 O(mn) 的标记数组
  · 递归深度最坏是 mn（300×300 全陆 → 90000 层），Go 的 goroutine 栈
    按需增长，本题量级不会爆；Python 则需要显式调高递归上限

手推一遍：以示例 2 为例：
  (0,0) 是 '1' → count = 1，淹没左上角 2×2 的陆地
  继续扫，到 (2,2) 是 '1' → count = 2，淹掉它自己
  到 (3,3) 是 '1' → count = 3，顺手淹掉 (3,4)
  扫完，答案 3 ✔

复杂度分析：
  时间 O(mn)：每格最多被访问一次，淹过就变 '0'
  空间 O(mn)：递归栈最深 mn，最坏是整张图都是陆地

解法二：BFS 淹岛【最优解，与解法一并列】

把解法一的递归换成显式队列：扫到 '1' 时 count++，该格入队，然后反复
弹出队首、把四邻中仍是 '1' 的格子置 '0' 后入队，直到队列为空。
  · 时间同为 O(mn)：每格入队至多一次
  · 空间同为 O(mn)：队列最坏同时装下整块陆地的格子
  · 取舍：BFS 不占递归栈，深度只受堆内存限制，长条形网格里比 DFS 稳；
    DFS 不用队列、少一层循环。同量级，两者并列最优
  · 入队时就要置 '0'，不能等出队再置：否则同一格会被多个邻居重复入队，
    队列膨胀到 O(4mn)
  · Go 用 [2]int 数组当坐标，切片头不会像 pair 结构那样多一次分配

复杂度分析：
  时间 O(mn)
  空间 O(mn)：队列最多同时装 O(mn) 个格子
*/

// 解法一：DFS 淹岛
func numIslands(grid [][]byte) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	count := 0
	for r := 0; r < len(grid); r++ {
		for c := 0; c < len(grid[0]); c++ {
			// 扫到 '1'：它一定属于尚未计数的岛，先计数再淹
			if grid[r][c] != '1' {
				continue
			}
			count++
			sink(grid, r, c)
		}
	}
	return count
}

// 先置 '0' 再递归：同一格不会被两个邻居重复访问
func sink(grid [][]byte, r, c int) {
	if r < 0 || r >= len(grid) || c < 0 || c >= len(grid[0]) {
		return
	}
	if grid[r][c] != '1' {
		return
	}
	grid[r][c] = '0'
	sink(grid, r-1, c)
	sink(grid, r+1, c)
	sink(grid, r, c-1)
	sink(grid, r, c+1)
}

// 解法二：BFS 淹岛
func numIslandsBFS(grid [][]byte) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	rows, cols := len(grid), len(grid[0])
	dr := [4]int{-1, 1, 0, 0}
	dc := [4]int{0, 0, -1, 1}

	count := 0
	var queue [][2]int
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if grid[r][c] != '1' {
				continue
			}
			count++
			// 入队前就淹掉：否则会被相邻格子重复入队
			grid[r][c] = '0'
			queue = append(queue, [2]int{r, c})

			for head := 0; head < len(queue); head++ {
				cr, cc := queue[head][0], queue[head][1]
				for d := 0; d < 4; d++ {
					nr, nc := cr+dr[d], cc+dc[d]
					if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
						continue
					}
					if grid[nr][nc] != '1' {
						continue
					}
					grid[nr][nc] = '0'
					queue = append(queue, [2]int{nr, nc})
				}
			}
			queue = queue[:0]
		}
	}
	return count
}

// ===== 测试辅助：字符串行转网格、双解法对照 =====
// 解法会原地淹岛，所以每条用例都要现造一份新网格
func makeGrid(rows []string) [][]byte {
	grid := make([][]byte, len(rows))
	for r, row := range rows {
		grid[r] = []byte(row)
	}
	return grid
}

func check(rows []string, want int) {
	if got := numIslands(makeGrid(rows)); got != want {
		panic(fmt.Sprintf("dfs(%v) = %d, want %d", rows, got, want))
	}
	if got := numIslandsBFS(makeGrid(rows)); got != want {
		panic(fmt.Sprintf("bfs(%v) = %d, want %d", rows, got, want))
	}
}

func main() {
	// 示例 1：一整块大陆，中间有凹陷的水
	check([]string{"11110", "11010", "11000", "00000"}, 1)

	// 示例 2：三座岛，(2,2) 与右下角互不相连
	check([]string{"11000", "11000", "00100", "00011"}, 3)

	// 边界：全是水，一座岛也没有
	check([]string{"000", "000", "000"}, 0)

	// 边界：全是陆地，只有一座岛
	check([]string{"111", "111", "111"}, 1)

	// 边界：单格网格
	check([]string{"1"}, 1)
	check([]string{"0"}, 0)

	// 对角相邻不算连通：一行里两块陆地隔着水
	check([]string{"101"}, 2)

	// 环形岛：中间的水不参与计数，但整圈仍是一座岛
	check([]string{"111", "101", "111"}, 1)

	// 边界：竖条与横条各成一体，只在角上相遇也不算连通
	check([]string{"10", "10", "01", "01"}, 2)

	// 边界：100×100 全陆，逼递归深度到 10000 层
	rows := make([]string, 100)
	for i := range rows {
		buf := make([]byte, 100)
		for j := range buf {
			buf[j] = '1'
		}
		rows[i] = string(buf)
	}
	check(rows, 1)

	fmt.Println("all tests passed")
}
