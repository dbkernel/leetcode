/*
============================================================================
面试题 16.04. 井字游戏（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/tic-tac-toe-lcci/
分类：矩阵 · 行列与对角线的整线检查

设计一个算法，判断玩家是否赢了井字游戏。输入是一个 N x N 的数组棋盘，
由字符" "，"X"和"O"组成，其中字符" "代表一个空位。

以下是井字游戏的规则：
- 玩家轮流将字符放入空位（" "）中。
- 第一个玩家总是放字符"O"，且第二个玩家总是放字符"X"。
- "X"和"O"只允许放置在空位中，不允许对已放有字符的位置进行填充。
- 当有N个相同（且非空）的字符填充任何行、列或对角线时，游戏结束，
  对应该字符的玩家获胜。
- 当所有位置非空时，也算为游戏结束。
- 如果游戏结束，玩家不允许再放置字符。

如果游戏存在获胜者，就返回该游戏的获胜者使用的字符（"X"或"O"）；
如果游戏以平局结束，则返回 "Draw"；如果仍会有行动（游戏未结束），
则返回 "Pending"。

示例 1：
输入：board = ["O X"," XO","X O"]
输出："X"

示例 2：
输入：board = ["OOX","XXO","OXO"]
输出："Draw"
解释：没有玩家获胜且不存在空位

示例 3：
输入：board = ["OOX","XXO","OX "]
输出："Pending"
解释：没有玩家获胜且仍存在空位

提示：
- 1 <= board.length == board[i].length <= 100
- 输入一定遵循井字棋规则
*/
package main

import (
	"fmt"
	"strings"
)

// sameLine 判断线上的 N 个格子是否同为一个非空字符，供行、列、对角线共用
func sameLine(board []string, row, col, dr, dc int) bool {
	mark := board[row][col]
	if mark == ' ' {
		return false
	}

	n := len(board)
	for k := 1; k < n; k++ {
		// 调用方只从边上出发：行、列从 0 开始，对角线从角格开始，
		// 因此 row + dr*k 与 col + dc*k 都不会越界
		if board[row+dr*k][col+dc*k] != mark {
			return false
		}
	}
	return true
}

/*
解法：逐行、逐列、两条对角线检查【最优解】

为什么慢：想判断胜负，容易先把 2N+2 条线各拷成一个字符串或集合，再判断
「去重后是否只剩一个非空字符」。拷贝所有线要 O(N^2) 额外空间，而且同一批
格子被反复读，只是把比较藏进了容器的实现里。

换个角度：一条线是否被同一个玩家占满，只取决于两件事 —— 起点字符是谁，
以及后面有没有出现与它不同的非空字符。于是拿起点做基准、边走边比即可，
不需要任何辅助数组。

关键点：
  · 基准格是空格时整条线直接判否：题面要求「相同（且非空）」才算获胜
  · 判定顺序固定为「已胜 > 未满 > 平局」：先查完所有线再统计空格，
    否则遇到「某方已连成 N 子但盘面还有空格」的情况会错答 Pending
  · 线长恰好是 N，N = 1 时单个非空格子就已经获胜
  · 行、列、两条对角线共 2N+2 条，每条 O(N)，总体 O(N^2)
  · Go 里 string 可按字节下标访问，'X'、'O'、' ' 都是单字节 ASCII

以示例 1（board = ["O X"," XO","X O"]）为例：
  行 "O X" 的首格是 O，第三格是 X，不成立
  列 0 是 O、空格、X，含空格，不成立
  反对角线 (0,2)、(1,1)、(2,0) 都是 X → 返回 "X" ✔
  此时 O 也有 3 子，但没占满任何一条线，所以「先查线」不会误判

以示例 3（board = ["OOX","XXO","OX "]）为例：
  四条方向都没有被占满；扫到 (2,2) 是空格 → 返回 "Pending" ✔

复杂度分析：
  时间 O(N^2)：2N+2 条线各扫 N 格，再加一次全盘空格扫描
  空间 O(1)：只用基准字符和几个下标，没有辅助数组
*/
func tictactoe(board []string) string {
	n := len(board)

	// 先判胜负：任意一条线成立就定下胜者
	var winner byte
	for i := 0; i < n && winner == 0; i++ {
		if sameLine(board, i, 0, 0, 1) {
			winner = board[i][0]
		} else if sameLine(board, 0, i, 1, 0) {
			winner = board[0][i]
		}
	}
	if winner == 0 && sameLine(board, 0, 0, 1, 1) {
		winner = board[0][0]
	}
	if winner == 0 && sameLine(board, 0, n-1, 1, -1) {
		winner = board[0][n-1]
	}
	if winner != 0 {
		return string(winner)
	}

	// 已胜优先于未满：走到这里才允许用空格判定 Pending
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if board[i][j] == ' ' {
				return "Pending"
			}
		}
	}
	return "Draw"
}

// ===== 测试辅助 =====
func check(board []string, want string) {
	if got := tictactoe(board); got != want {
		panic(fmt.Sprintf("tictactoe(%v) = %q, want %q", board, got, want))
	}
}

func main() {
	// 示例 1：反对角线由 X 占满
	check([]string{"O X", " XO", "X O"}, "X")

	// 示例 2：盘面已满且无人连成线
	check([]string{"OOX", "XXO", "OXO"}, "Draw")

	// 示例 3：无人获胜且仍有空格
	check([]string{"OOX", "XXO", "OX "}, "Pending")

	// 关键点：O 已连成一行但盘面还有空格，必须返回 O 而不是 Pending
	check([]string{"OOO", "XX ", "   "}, "O")

	// 覆盖列：第 0 列三个 O
	check([]string{"OX ", "OX ", "O  "}, "O")

	// 覆盖主对角线
	check([]string{"OX ", "XO ", "  O"}, "O")

	// 边界：N = 1，单个非空格子即获胜
	check([]string{"X"}, "X")
	check([]string{"O"}, "O")
	check([]string{" "}, "Pending")

	// 边界：N = 4，整行获胜且盘面未满
	check([]string{"OOOO", "XXX ", "    ", "    "}, "O")
	check([]string{"XXXX", "OOO ", "O   ", "    "}, "X")

	// 反直觉：4 x 4 棋盘两两交错，任何方向都凑不齐 4 个同色
	check([]string{"XXOO", "OOXX", "XOXO", "OXOX"}, "Draw")

	// 边界：N = 100 全空，只走一遍空格扫描
	board := make([]string, 100)
	for i := range board {
		board[i] = strings.Repeat(" ", 100)
	}
	check(board, "Pending")

	fmt.Println("all tests passed")
}
