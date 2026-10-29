/*
============================================================================
79. 单词搜索（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/word-search/
分类：网格 · 回溯（原地标记代替 visited）

给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word
存在于网格中，返回 true ；否则，返回 false 。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格
是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

示例 1：
输入：board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']],
      word = "ABCCED"
输出：true

示例 2：
输入：board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']],
      word = "SEE"
输出：true

示例 3：
输入：board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']],
      word = "ABCB"
输出：false

提示：
- m == board.length
- n = board[i].length
- 1 <= m, n <= 6
- 1 <= word.length <= 15
- board 和 word 仅由大小写英文字母组成

进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下
可以更快解决问题？
*/
package main

import "fmt"

/*
解法：回溯 DFS（原地占位代替 visited）【最优解】

为什么慢：不做回溯的暴力是"对每个起点单独枚举一条长度 L 的路径"，同一
段前缀会被不同分支反复展开；而为了让"同一格不许重复使用"生效，最省事的
做法是额外开一张 m×n 的 visited 表，把空间顶到 O(mn)——可路径长度只有 L，
多出来的标记与路径无关。

换个角度：枚举"当前站在 (r,c)、已匹配到 word[k]"的所有走法，匹配失败就
退回上一步换方向。走到 (r,c) 时它必然属于当前路径，于是不需要独立的标记
数组：把 board[r][c] 改成一个绝不会与字母相等的占位符，"字符是否相等"的
检查就自动替我们挡住了重复使用。

关键点：
  · 进入 (r,c) 后立刻写占位符，退出前必须改回原字符，改回点在"四条
    方向都试完"之后，只有一处，别在中间提前 return 漏掉回滚
  · 为什么没有 visited 也能工作：占位符是"本路径已占用"的唯一凭证，
    下一层看到 board[r][c] != word[k] 就剪掉，语义与 visited 完全一致，
    而它只作用于当前路径，天然带"撤销"
  · 为什么必须回滚：占位符是全局副作用。若某条分支失败后不还原，
    这个字母对之后所有起点、所有分支都消失了，本该成立的路径会被判成
    不存在（示例外的经典用例 "ABCESEEEFS" 就是靠回滚才返回 true）
  · 终止条件放在"当前字符已匹配"之后：k 走到 word 末尾就返回 true，
    不必再去找邻居
  · 起点只从 board[r][c] == word[0] 处进入，省掉必然失败的递归
  · 进阶剪枝：先统计 board 与 word 的字符频次，word 里某字符比 board
    里还多就直接 false；也可以从"两端出现次数更少"的那端开始搜
  · 官方题解额外开了 visited 表（O(mn) 空间）；这里用占位符代替，
    额外空间降到 O(L)，时间不变
  · Go 用闭包递归（先把闭包声明成变量再赋值）以捕获 board 与 word，
    避免为解法单独建一个结构体

手推一遍：以示例 3（word = "ABCB"）为例：
  从 (0,0) 的 'A' 进 → (0,1) 的 'B' 占位成 '#' → (0,2) 的 'C' 占位
  还差一个 'B'：右邻 (0,3) 是 'E'，下邻 (1,2) 是 'C'，而唯一相邻的
  'B'(0,1) 已是占位符——正是这一步让"同一格不许重用"自动生效
  四个方向都不成，逐层把 'C'、'B'、'A' 改回来，返回 false ✔

复杂度分析：
  时间 O(mn·3^L)：mn 个起点；除第一步外每步最多 3 个未回头的方向，
    单条路径展开不超过 3^(L-1)，L 为 word 长度
  空间 O(L)：递归深度不超过 L；board 原地改写，不额外开 visited 表
*/
func exist(board [][]byte, word string) bool {
	if len(board) == 0 || len(board[0]) == 0 || word == "" {
		return false
	}
	rows, cols := len(board), len(board[0])
	dr := [4]int{-1, 1, 0, 0}
	dc := [4]int{0, 0, -1, 1}

	// 站在 (r,c)、已匹配到 word[k]，返回能否走完剩余后缀
	var search func(r, c, k int) bool
	search = func(r, c, k int) bool {
		if r < 0 || r >= rows || c < 0 || c >= cols {
			return false
		}
		if board[r][c] != word[k] {
			return false
		}
		// 最后一个字符都对上了，路径已成立，不必再找邻居
		if k+1 == len(word) {
			return true
		}

		saved := board[r][c]
		board[r][c] = '#' // 本路径占用该格，作用等同于 visited = true

		found := false
		for d := 0; d < 4 && !found; d++ {
			found = search(r+dr[d], c+dc[d], k+1)
		}

		// 唯一回滚点：不管成败都把字符放回，供其它分支继续使用
		board[r][c] = saved
		return found
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			// 只从首字母处起搜：其它起点必然在第一层就被剪掉
			if board[r][c] != word[0] {
				continue
			}
			if search(r, c, 0) {
				return true
			}
		}
	}
	return false
}

// ===== 测试辅助：字符串行转网格、单解法断言 =====
// 回溯会就地改 board，每条用例都要现造一份，避免污染下一条
func makeBoard(rows []string) [][]byte {
	board := make([][]byte, len(rows))
	for r, row := range rows {
		board[r] = []byte(row)
	}
	return board
}

func check(rows []string, word string, want bool) {
	if got := exist(makeBoard(rows), word); got != want {
		panic(fmt.Sprintf("exist(%v, %q) = %v, want %v", rows, word, got, want))
	}
}

func main() {
	board := []string{"ABCE", "SFCS", "ADEE"}

	// 示例 1：一条折线路径 ABCCED
	check(board, "ABCCED", true)

	// 示例 2：'S' 出现两次，只有 (1,3) 那个起点能拼出 SEE
	check(board, "SEE", true)

	// 示例 3：能拼出 "ABC"，但第四步要回头用同一个 'B'，必须失败
	check(board, "ABCB", false)

	// 回滚用例：(0,0) 起搜先走错一条分支，不回滚就会丢掉 'E' 而误判
	check([]string{"ABCE", "SFES", "ADEE"}, "ABCESEEEFS", true)

	// 反直觉：'a' 有三个起点，只有最上面那个能走到结尾
	check([]string{"abc", "axc", "abd"}, "abc", true)

	// 边界：单格网格
	check([]string{"a"}, "a", true)
	check([]string{"a"}, "b", false)
	// 同一个字母不许用两次，单格拼不出两个字符
	check([]string{"a"}, "aa", false)

	// 边界：单词比格子总数还长，必然不存在
	check([]string{"ab"}, "aba", false)

	// 边界：1×4 直线，只有顺序连着的读法才成立
	check([]string{"abcd"}, "abcd", true)
	check([]string{"abcd"}, "dcba", true)
	check([]string{"abcd"}, "abdc", false)

	fmt.Println("all tests passed")
}
