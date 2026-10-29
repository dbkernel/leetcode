/*
============================================================================
130. 被围绕的区域（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/surrounded-regions/
分类：并查集 · 边界洪水填充

给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' 组成，捕获 所有
被围绕的区域：

  - 连接：一个单元格与水平或垂直方向上相邻的单元格连接。
  - 区域：连接所有 'O' 的单元格来形成一个区域。
  - 围绕：如果一个区域中的所有 'O' 单元格都不在棋盘的边缘，则该区域被
    包围。这样的区域 完全 被 'X' 单元格包围。

通过 原地 将输入矩阵中的所有 'O' 替换为 'X' 来 捕获被围绕的区域。你
不需要返回任何值。

示例 1：
输入：board = [['X','X','X','X'],['X','O','O','X'],['X','X','O','X'],['X','O','X','X']]
输出：[['X','X','X','X'],['X','X','X','X'],['X','X','X','X'],['X','O','X','X']]
解释：在上图中，底部的区域没有被捕获，因为它在 board 的边缘并且不能被围绕。

示例 2：
输入：board = [['X']]
输出：[['X']]

提示：
- m == board.length
- n == board[i].length
- 1 <= m, n <= 200
- board[i][j] 为 'X' 或 'O'
*/
package main

import "fmt"

// 并查集：路径压缩 + 按大小合并。
// 路径压缩把查找路径上的点直接挂到根，缩短后续查找；
// 按大小合并把元素少的树挂到元素多的树下，防止树退化成链。
// 两者合起来让单次操作的均摊代价接近 O(α(n))。
type DSU struct {
	parent []int
	size   []int
}

func newDSU(n int) *DSU {
	d := &DSU{parent: make([]int, n), size: make([]int, n)}
	for i := 0; i < n; i++ {
		d.parent[i] = i
		d.size[i] = 1
	}
	return d
}

func (d *DSU) Find(x int) int {
	if d.parent[x] != x {
		d.parent[x] = d.Find(d.parent[x])
	}
	return d.parent[x]
}

func (d *DSU) Unite(x, y int) {
	rx, ry := d.Find(x), d.Find(y)
	if rx == ry {
		return
	}
	if d.size[rx] < d.size[ry] {
		rx, ry = ry, rx
	}
	d.parent[ry] = rx
	d.size[rx] += d.size[ry]
}

/*
解法一：从边界多源 BFS，标记安全格后回写【最优解】

为什么慢：直接对每个 'O' 判断“它所在区域是否被围”，要么反复扫描同一块
区域，要么从内部出发根本无从知道它有没有连到边界。

换个角度：逆向思考 —— 不被围的反面是“与边界连通”。从四条边上的每个
'O' 同时出发做洪水填充，凡是能淹到的 'O' 都不会被围，剩下的才是被围的。

关键点：

	· 必须从边界出发：边界上的 'O' 天然安全，从它扩散到的整块 'O' 都安全
	· 标记用第三个字符 '#'，先标记后回写，避免边标记边改写导致漏填或死循环
	· 四条边都要扫，角落会被重复入队，靠 board 上的 '#' 判重
	· 最后一律把剩余 'O' 改 'X'、把 '#' 改回 'O'，全 'X' 或全 'O' 都成立

手推一遍：以示例 1 为例（4x4，底部 (3,1) 是 'O'）：

	(3,1) 在边界，入队并标 '#'
	它四周都是 'X'，无法继续扩散
	回写时 (1,1)(1,2)(2,2) 仍是 'O' → 全部改 'X'；(3,1) 的 '#' 还原成 'O' ✔

复杂度分析：

	时间 O(mn)：每个格子至多入队一次，最后回写再扫一遍
	空间 O(mn)：队列最坏装下所有 'O'，原地 '#' 标记省掉了 visited

解法二：并查集 + 虚拟边界节点【最优解，与解法一并列】

为什么慢：洪水填充要真的走到每个格子；若只想回答“这块 'O' 是否触边”，
并查集把问题变成一次集合查找。

换个角度：多设一个虚拟节点代表“棋盘外”。把所有边界 'O' 与它合并，再把
相邻的 'O' 两两合并；最后不在虚拟节点集合里的 'O' 就是被围的。

关键点：

	· 格子编号 r*n+c，虚拟节点编号 m*n，并查集开 m*n+1 个位置
	· 四条边上的 'O' 都要与虚拟节点合并，漏掉一条边就会把贴边的 'O' 误判
	· 相邻关系只需向右、向下合并，这两条方向已覆盖全部无向相邻边
	· 回写时比较 Find(格子) 与 Find(虚拟节点)，不要把虚拟节点写进棋盘

手推一遍：仍以示例 1 为例：

	边界 'O' 只有 (3,1)，与虚拟节点合并成 {虚拟, (3,1)}
	(1,1)-(1,2)-(2,2) 相邻合并成另一集合，与虚拟节点不相通
	回写：(1,1)(1,2)(2,2) 改 'X'，(3,1) 保留 'O' ✔

复杂度分析：

	时间 O(mn α(mn))：每个格子常数次查找与合并
	空间 O(mn)：parent 与 size 各一个长度 m*n+1 的切片
*/
func solve(board [][]byte) {
	m, n := len(board), len(board[0])
	qr, qc := []int{}, []int{}

	// 入队即标记：'#' 表示“与边界连通的 O”，同时充当判重
	push := func(r, c int) {
		if board[r][c] != 'O' {
			return
		}
		board[r][c] = '#'
		qr = append(qr, r)
		qc = append(qc, c)
	}

	for r := 0; r < m; r++ {
		push(r, 0)
		push(r, n-1)
	}
	for c := 0; c < n; c++ {
		push(0, c)
		push(m-1, c)
	}

	dr := [4]int{1, -1, 0, 0}
	dc := [4]int{0, 0, 1, -1}
	for head := 0; head < len(qr); head++ {
		r, c := qr[head], qc[head]
		for k := 0; k < 4; k++ {
			nr, nc := r+dr[k], c+dc[k]
			if nr < 0 || nr >= m || nc < 0 || nc >= n {
				continue
			}
			push(nr, nc)
		}
	}

	// 先标记后回写：此刻 '#' 是安全的 O，剩余的 'O' 才是被围的
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			switch board[r][c] {
			case 'O':
				board[r][c] = 'X'
			case '#':
				board[r][c] = 'O'
			}
		}
	}
}

// 解法二：并查集 + 虚拟边界节点
func solveDSU(board [][]byte) {
	m, n := len(board), len(board[0])
	root := m * n // 虚拟节点代表“棋盘外”
	dsu := newDSU(m*n + 1)
	id := func(r, c int) int { return r*n + c }

	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if board[r][c] != 'O' {
				continue
			}
			cur := id(r, c)
			// 贴边的 O 与棋盘外连通，永远不会被围
			if r == 0 || r == m-1 || c == 0 || c == n-1 {
				dsu.Unite(cur, root)
			}
			// 只看右、下：每条无向边恰好被一个格子负责一次
			if r+1 < m && board[r+1][c] == 'O' {
				dsu.Unite(cur, id(r+1, c))
			}
			if c+1 < n && board[r][c+1] == 'O' {
				dsu.Unite(cur, id(r, c+1))
			}
		}
	}

	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if board[r][c] == 'O' && dsu.Find(id(r, c)) != dsu.Find(root) {
				board[r][c] = 'X'
			}
		}
	}
}

func cloneBoard(board [][]byte) [][]byte {
	out := make([][]byte, len(board))
	for i := range board {
		out[i] = append([]byte(nil), board[i]...)
	}
	return out
}

func equalBoard(a, b [][]byte) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if len(a[i]) != len(b[i]) {
			return false
		}
		for j := range a[i] {
			if a[i][j] != b[i][j] {
				return false
			}
		}
	}
	return true
}

func check(board [][]byte, want [][]byte) {
	got1 := cloneBoard(board)
	solve(got1)
	if !equalBoard(got1, want) {
		panic(fmt.Sprintf("bfs(%v) = %v, want %v", board, got1, want))
	}
	got2 := cloneBoard(board)
	solveDSU(got2)
	if !equalBoard(got2, want) {
		panic(fmt.Sprintf("dsu(%v) = %v, want %v", board, got2, want))
	}
}

func main() {
	// 示例 1：逐一比对整个矩阵
	check([][]byte{
		{'X', 'X', 'X', 'X'},
		{'X', 'O', 'O', 'X'},
		{'X', 'X', 'O', 'X'},
		{'X', 'O', 'X', 'X'},
	}, [][]byte{
		{'X', 'X', 'X', 'X'},
		{'X', 'X', 'X', 'X'},
		{'X', 'X', 'X', 'X'},
		{'X', 'O', 'X', 'X'},
	})

	// 示例 2：单格 X
	check([][]byte{{'X'}}, [][]byte{{'X'}})

	// 边界：单格 O 在边界上，保留
	check([][]byte{{'O'}}, [][]byte{{'O'}})

	// 边界：全 X，原样返回
	check([][]byte{
		{'X', 'X', 'X'},
		{'X', 'X', 'X'},
		{'X', 'X', 'X'},
	}, [][]byte{
		{'X', 'X', 'X'},
		{'X', 'X', 'X'},
		{'X', 'X', 'X'},
	})

	// 边界：全 O，全部贴边，原样返回
	check([][]byte{
		{'O', 'O', 'O'},
		{'O', 'O', 'O'},
		{'O', 'O', 'O'},
	}, [][]byte{
		{'O', 'O', 'O'},
		{'O', 'O', 'O'},
		{'O', 'O', 'O'},
	})

	// 反直觉：内圈 O 被 X 包围，整块都要变成 X
	check([][]byte{
		{'X', 'X', 'X', 'X', 'X'},
		{'X', 'O', 'O', 'O', 'X'},
		{'X', 'O', 'X', 'O', 'X'},
		{'X', 'O', 'O', 'O', 'X'},
		{'X', 'X', 'X', 'X', 'X'},
	}, [][]byte{
		{'X', 'X', 'X', 'X', 'X'},
		{'X', 'X', 'X', 'X', 'X'},
		{'X', 'X', 'X', 'X', 'X'},
		{'X', 'X', 'X', 'X', 'X'},
		{'X', 'X', 'X', 'X', 'X'},
	})

	// 反直觉：只在对角相邻不算连通，中心 O 会被改写
	check([][]byte{
		{'O', 'X', 'O'},
		{'X', 'O', 'X'},
		{'O', 'X', 'O'},
	}, [][]byte{
		{'O', 'X', 'O'},
		{'X', 'X', 'X'},
		{'O', 'X', 'O'},
	})

	// 有通道连到边界：整块 O 都保留
	check([][]byte{
		{'X', 'X', 'X', 'X', 'X'},
		{'X', 'O', 'O', 'O', 'X'},
		{'X', 'O', 'X', 'O', 'X'},
		{'X', 'O', 'O', 'O', 'X'},
		{'X', 'O', 'X', 'X', 'X'},
	}, [][]byte{
		{'X', 'X', 'X', 'X', 'X'},
		{'X', 'O', 'O', 'O', 'X'},
		{'X', 'O', 'X', 'O', 'X'},
		{'X', 'O', 'O', 'O', 'X'},
		{'X', 'O', 'X', 'X', 'X'},
	})

	// 单行棋盘：两端都是边界，全部保留
	check([][]byte{{'O', 'X', 'O'}}, [][]byte{{'O', 'X', 'O'}})

	fmt.Println("all tests passed")
}
