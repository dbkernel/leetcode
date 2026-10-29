/*
============================================================================
74. 搜索二维矩阵（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/search-a-2d-matrix/
分类：二分查找 · 二维矩阵展平 / 两次二分

给你一个满足下述两条属性的 m x n 整数矩阵：
- 每行中的整数从左到右按非严格递增顺序排列。
- 每行的第一个整数大于前一行的最后一个整数。

给你一个整数 target ，如果 target 在矩阵中，返回 true ；否则，返回 false 。

你必须编写一个时间复杂度为 O(log(m * n)) 的解决方案。

示例 1：
输入：matrix = [[1,3,5,7],[10,11,16,20],
              [23,30,34,60]], target = 3
输出：true

示例 2：
输入：matrix = [[1,3,5,7],[10,11,16,20],
              [23,30,34,60]], target = 13
输出：false

提示：
- m == matrix.length
- n == matrix[i].length
- 1 <= m, n <= 100
- -10^4 <= matrix[i][j], target <= 10^4
*/
package main

import "fmt"

/*
解法一：把矩阵看成一条升序数组，整体二分【最优解】

为什么慢：逐行扫描要 O(m * n)；就算每行内部二分，也得先线性找到目标行，
瓶颈仍在「先选行」这一步。题目要求的 O(log(m * n)) 不允许任何一层线性扫描。

换个角度：第二条性质「下一行首元素大于上一行末元素」说明按行首尾相接后，
整条序列严格升序。元素 (i, j) 恰好是这条序列里的第 i * n + j 个，
二维查找因此退化成一次普通的一维二分。

关键点：
  · 一维下标 mid 映射回矩阵：行 mid / n，列 mid % n
    —— 先除后模，写反会取到完全错位的元素
  · 左闭右闭区间用 lo <= hi；mid 取 lo + (hi - lo) / 2 防止 lo + hi 溢出
  · 矩阵可能为空（自测覆盖），进循环前先判 matrix 和 matrix[0] 非空
  · 命中就直接返回，不要在循环外再用一次下标访问，空矩阵会越界

手推一遍：
  以示例 1（3 行 4 列，target = 3）为例：
  一维长度 12，lo = 0，hi = 11
  mid = 5 → 行 1 列 1 → 11 > 3 → hi = 4
  mid = 2 → 行 0 列 2 → 5 > 3 → hi = 1
  mid = 0 → 行 0 列 0 → 1 < 3 → lo = 1
  mid = 1 → 行 0 列 1 → 3 == target → true ✔

复杂度分析：
  时间 O(log(m * n))：每次把候选区间折半
  空间 O(1)：只用了 lo、hi、mid 三个下标
*/
func searchMatrix(matrix [][]int, target int) bool {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return false
	}

	n := len(matrix[0])
	lo, hi := 0, len(matrix)*n-1

	for lo <= hi {
		mid := lo + (hi-lo)/2
		// 先除后模：行号由高位决定，列号是余数
		value := matrix[mid/n][mid%n]

		if value == target {
			return true
		}
		if value < target {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return false
}

/*
解法二：先二分定位行，再在行内二分【最优解】

为什么慢：解法一一次二分就能命中；本解法要两轮，第一轮在首列上找
「最后一个首元素 ≤ target 的行」，第二轮在该行内找 target。

换个角度：第一条性质保证行内有序，第二条性质保证首列有序，于是「选行」
和「选列」可以拆成两个互不干扰的二分，各自只依赖一维有序性。

关键点：
  · 首列二分维护的是「最后一个 ≤ target 的行号」row，不是 lower_bound；
    用 lower_bound 会落到第一个 > target 的行，还得回退一格
  · target 比首列最小值还小时 row 保持 -1，直接返回 false
  · 行内二分的区间是 [0, n-1]，不要沿用首列二分剩下的边界
  · 与解法一并列最优：两轮长度分别是 m 与 n，log m + log n 与
    log(m * n) 同阶，没有渐近差别；它只在「必须按行列定位」的场景才更
    直观，日常仍推荐解法一

手推一遍：
  示例 1（target = 3）：
  首列 [1, 10, 23]：1 ≤ 3 → row = 0；10 > 3 停 → row = 0
  第 0 行 [1, 3, 5, 7] 二分 → 命中 3 → true
  示例 2（target = 13）：
  首列：1 ≤ 13 → row = 0；10 ≤ 13 → row = 1；23 > 13 停
  第 1 行 [10, 11, 16, 20] 二分 → 没有 13 → false ✔

复杂度分析：
  时间 O(log m + log n)：首列二分 O(log m)，行内二分 O(log n)
  空间 O(1)
*/
func searchMatrixTwoStep(matrix [][]int, target int) bool {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return false
	}

	m, n := len(matrix), len(matrix[0])

	// 找最后一个首元素 ≤ target 的行；都不满足时 row 保持 -1
	row := -1
	lo, hi := 0, m-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		if matrix[mid][0] <= target {
			row = mid
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	if row < 0 {
		return false
	}

	lo, hi = 0, n-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		value := matrix[row][mid]
		if value == target {
			return true
		}
		if value < target {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return false
}

// 两套解法吃同样的矩阵；题目不修改输入，这里直接复用
func check(matrix [][]int, target int, want bool) {
	if got := searchMatrix(matrix, target); got != want {
		panic(fmt.Sprintf("flatten(%v, %d) = %v, want %v", matrix, target, got, want))
	}
	if got := searchMatrixTwoStep(matrix, target); got != want {
		panic(fmt.Sprintf("twoStep(%v, %d) = %v, want %v", matrix, target, got, want))
	}
}

func main() {
	sample := [][]int{
		{1, 3, 5, 7},
		{10, 11, 16, 20},
		{23, 30, 34, 60},
	}

	// 示例 1：目标在中间行
	check(sample, 3, true)

	// 示例 2：目标落在两行之间的空隙
	check(sample, 13, false)

	// 边界：空矩阵与只有空行的矩阵
	check([][]int{}, 0, false)
	check([][]int{{}}, 0, false)

	// 边界：1 x 1 矩阵，命中 / 偏小 / 偏大
	check([][]int{{5}}, 5, true)
	check([][]int{{5}}, 4, false)
	check([][]int{{5}}, 6, false)

	// 边界：只有一行，首尾元素都要能命中
	oneRow := [][]int{{1, 2, 3, 4, 5}}
	check(oneRow, 1, true)
	check(oneRow, 5, true)
	check(oneRow, 3, true)
	check(oneRow, 0, false)
	check(oneRow, 6, false)

	// 边界：只有一列，首列二分退化成在单列上找行
	oneCol := [][]int{{1}, {2}, {5}}
	check(oneCol, 5, true)
	check(oneCol, 3, false)
	check(oneCol, 0, false)

	// 边界：行内允许重复值，二分仍要返回 true
	dup := [][]int{{1, 3, 3}, {5, 7, 8}}
	check(dup, 3, true)
	check(dup, 4, false)
	check(dup, 8, true)

	// 边界：负数与 0，验证比较没有用无符号整数
	neg := [][]int{{-10, -5, -1}, {0, 2, 9}}
	check(neg, -10, true)
	check(neg, -5, true)
	check(neg, 10, false)
	check(neg, -11, false)

	fmt.Println("all tests passed")
}
