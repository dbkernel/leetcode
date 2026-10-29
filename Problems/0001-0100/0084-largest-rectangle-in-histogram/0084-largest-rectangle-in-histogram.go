/*
============================================================================
84. 柱状图中最大的矩形（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/largest-rectangle-in-histogram/
分类：单调栈 · 最近更小元素

给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且
宽度为 1 。

求在该柱状图中，能够勾勒出来的矩形的最大面积。

示例 1：
输入：heights = [2,1,5,6,2,3]
输出：10
解释：最大的矩形为图中红色区域，面积为 10

示例 2：
输入： heights = [2,4]
输出： 4

提示：
- 1 <= heights.length <= 10^5
- 0 <= heights[i] <= 10^4
*/
package main

import (
	"fmt"
	"math"
)

/*
解法一：枚举左右边界（宽），边走边维护区间最小高度【非最优】

为什么慢：矩形的面积由"左边界 + 右边界 + 区间最小高度"决定，三者里任意
两个确定后第三个已经被确定。要枚举全部 O(n^2) 个区间，本身就是平方级的
工作量，无法再优化。

换个角度：固定左端点 left，向右推进 right 时区间最小值只会变小，可以顺手
维护，不必每次回头重扫区间（那样是 O(n^3)）。这样一次双重循环就能覆盖所有
区间，把常数压到最小。

关键点：
  · 每进入一个新的 left，minHeight 都要重置为"无穷大"，不能沿用上一轮的
  · 必须先用 heights[right] 更新 minHeight，再算面积 —— 顺序反了会把当前
    柱子漏掉，第一个区间（right == left）会算成 0
  · minHeight 只会单调不增，所以"当前区间的最小高度"恰好就是能铺满该区间的
    最大矩形高度，不必再考虑更矮的取值

以示例 2（heights = [2,4]）为例：
  left=0：right=0，minHeight=2，面积 2；right=1，minHeight=2，面积 4
  left=1：right=1，minHeight=4，面积 4
  答案 4 ✔

复杂度分析：
  时间 O(n^2)：枚举 O(n^2) 个区间，每个区间 O(1) 更新
  空间 O(1)：只用了几个游标，没有额外数组

解法二：枚举高度（柱），向两侧扩展到第一根更矮的柱子【非最优】

为什么慢：解法一枚举的是区间，而真正决定答案的其实是"以某根柱子为最低点"
的那一族区间。枚举高度只有 n 种可能，但每次向两侧扩展都要一格一格走，
最坏（等高或单调）走满 n 格，总时间仍是 O(n^2)。

换个角度：定住高度 h = heights[mid]，矩形能向右铺多远，只取决于右侧第一根
严格矮于 h 的柱子在哪儿；向左同理。也就是说答案 = max over mid of
h * (右边界 - 左边界 + 1)，问题从"枚举区间"变成 2n 次"找最近的更矮元素"。

关键点：
  · 扩展条件用 heights[..] >= height：等高柱子拼在一起仍然合法，用 > 会
    把等高的邻居挡在外面，宽度偏小
  · 用 for 一路扩到底，不能只看紧邻的一格 —— 更矮的柱子可能隔了好几格
  · 左右边界都取闭区间，宽度是 right - left + 1，不是 right - left
  · 这是下一节单调栈的原型：把"向两侧线性扫描"换成"用栈维护候选边界"

以示例 1（heights = [2,1,5,6,2,3]）为例：
  mid=2（高 5）：左扩到 1（heights[1]=1 更矮），右扩到 3（heights[4]=2 更矮）
                 宽度 2，面积 10
  mid=3（高 6）：左右都扩不动，面积 6
  mid=4（高 2）：左到 1、右到 5，宽度 4，面积 8
  其余柱子面积都更小，答案 10 ✔

复杂度分析：
  时间 O(n^2)：n 根柱子，每根最多向两侧各走 n 格
  空间 O(1)：只有左右游标

解法三：两遍单调栈，分别求左右边界【最优解】

为什么慢：解法二的瓶颈是"向两侧走"重复扫了同一段后缀。关键观察是：若
j0 < j1 且 heights[j0] >= heights[j1]，那么 j1 右侧的柱子 i 找"左边最近的
更矮"时，j0 一定被 j1 挡住，j0 从此不可能再作为答案的一部分。于是候选边界
只需要保留一个高度严格递增的下标序列 —— 这就是单调栈。

换个角度：从左往右扫一遍得到每个位置"左边最近的严格更矮"，再从右往左扫
一遍得到"右边最近的严格更矮"，两个边界之间（开区间）就是该高度能铺满的
范围。

关键点：
  · 弹栈条件必须是 heights[栈顶] >= heights[i]（连等高都弹）：等高柱子若
    留在栈里，左侧边界会互相挡住，宽度算出来偏小；连等高一起弹掉后，每个
    位置拿到的是"最近的严格更矮"，两侧口径一致
  · 栈空表示这一侧没有更矮的柱子：左边界记 -1，右边界记 n
  · 面积宽度是 right[i] - left[i] - 1：左右边界都是"第一根更矮的"，要排除
  · 第二遍之前必须把栈清空，否则左边界残留的下标会污染右边界
  · 高度为 0 的柱子面积为 0，不必特判；返回类型 int 也够用，因为
    10^5 × 10^4 = 10^9 仍在 32 位整数范围内
  · Go 用切片模拟栈，左边界用 -1 表示"没有更矮的柱子"

以示例 1（heights = [2,1,5,6,2,3]）为例：
  左边界 left  = [-1, -1, 1, 2, 1, 4]
  右边界 right = [ 1,  6, 4, 4, 6, 6]
  逐根算面积：2, 6, 10, 6, 8, 3 → 最大 10 ✔
  （下标 2、3 的柱子高 5、6，右边界都是 4 号的矮柱 2，左边界分别是 1 号、
    2 号，所以只有 2 号能铺出宽 2 的矩形）

复杂度分析：
  时间 O(n)：每个下标在每一遍里进出栈各一次，两遍是 2n 次栈操作
  空间 O(n)：left / right 各 n 个下标，栈最坏 n 个

解法四：一遍单调栈，出栈时同时确定右边界【最优解，与解法三并列】

为什么慢：解法三为了求右边界又倒着扫了一遍。其实右边界在"出栈那一刻"就
已知：位置 i 把栈顶 top 弹出，说明 i 是 top 右侧第一根高度 <= heights[top]
的柱子。

换个角度：入栈前确定的左边界 + 出栈时确定的右边界，一次从左到右就能同时
拿到两侧信息。对最终答案没有影响的原因：右侧用的是"小于等于"，等高的一段
里最靠右的那根会最后被弹出，它的右边界恰好是真正的"第一根严格更矮"，而它
的左边界也是正确的，整块等高矩形由它算出来。

关键点：
  · 左边界在入栈前确定（弹完之后栈顶），右边界在出栈时确定（当前 i）
  · right 数组初始化成 n：遍历结束后还在栈里的柱子，右侧没有更矮的，边界
    就是虚拟的 n
  · 弹栈条件 heights[栈顶] >= heights[i] 让等高柱子立刻出栈，靠"最右那根"
    兜住正确宽度 —— 这也是为什么公式仍是 right - left - 1
  · 先记录 right 再弹栈：反了就取不到出栈元素的下标

以示例 1（heights = [2,1,5,6,2,3]）为例：
  i=1（高 1）弹出 0，right[0]=1
  i=4（高 2）连续弹出 3、2，right[3]=right[2]=4
  结束时 right = [1, 6, 4, 4, 6, 6]，与解法三完全一致 → 答案仍是 10 ✔

复杂度分析：
  时间 O(n)：只扫一遍，每个下标进出栈各一次
  空间 O(n)：left / right 两个数组加一个栈；与解法三相比少扫一遍，同阶
*/

// 解法一：枚举左右边界（宽）
func largestRectangleAreaEnumWidth(heights []int) int {
	n := len(heights)
	ans := 0
	for left := 0; left < n; left++ {
		// 区间最小高度：进入新的左端点时必须重置
		minHeight := math.MaxInt
		for right := left; right < n; right++ {
			// 先并入当前柱子，再算面积，否则 right == left 的区间会漏算
			if heights[right] < minHeight {
				minHeight = heights[right]
			}
			area := (right - left + 1) * minHeight
			if area > ans {
				ans = area
			}
		}
	}
	return ans
}

// 解法二：枚举高度（柱），向两侧扩展到第一根更矮的柱子
func largestRectangleAreaEnumHeight(heights []int) int {
	n := len(heights)
	ans := 0
	for mid := 0; mid < n; mid++ {
		height := heights[mid]
		left, right := mid, mid
		// 等高也算能铺满，所以用 >= 而不是 >
		for left-1 >= 0 && heights[left-1] >= height {
			left--
		}
		for right+1 < n && heights[right+1] >= height {
			right++
		}
		if area := (right - left + 1) * height; area > ans {
			ans = area
		}
	}
	return ans
}

// 解法三：两遍单调栈，分别求左右边界（平台入口签名）
func largestRectangleArea(heights []int) int {
	n := len(heights)
	left := make([]int, n)
	right := make([]int, n)
	var monoStack []int

	for i := 0; i < n; i++ {
		// 连等高一起弹，保证拿到的是"最近的严格更矮"
		for len(monoStack) > 0 && heights[monoStack[len(monoStack)-1]] >= heights[i] {
			monoStack = monoStack[:len(monoStack)-1]
		}
		if len(monoStack) == 0 {
			left[i] = -1
		} else {
			left[i] = monoStack[len(monoStack)-1]
		}
		monoStack = append(monoStack, i)
	}

	// 第二遍之前清空，否则上一遍的下标会污染右边界
	monoStack = nil
	for i := n - 1; i >= 0; i-- {
		for len(monoStack) > 0 && heights[monoStack[len(monoStack)-1]] >= heights[i] {
			monoStack = monoStack[:len(monoStack)-1]
		}
		if len(monoStack) == 0 {
			right[i] = n
		} else {
			right[i] = monoStack[len(monoStack)-1]
		}
		monoStack = append(monoStack, i)
	}

	ans := 0
	for i := 0; i < n; i++ {
		// 两侧都是"第一根更矮"，宽度要排除它们
		if area := (right[i] - left[i] - 1) * heights[i]; area > ans {
			ans = area
		}
	}
	return ans
}

// 解法四：一遍单调栈，出栈时同时确定右边界
func largestRectangleAreaOnePass(heights []int) int {
	n := len(heights)
	left := make([]int, n)
	// 留在栈里的柱子右侧没有更矮的，右边界就是虚拟的 n
	right := make([]int, n)
	for i := range right {
		right[i] = n
	}
	var monoStack []int

	for i := 0; i < n; i++ {
		for len(monoStack) > 0 && heights[monoStack[len(monoStack)-1]] >= heights[i] {
			// 出栈的这一刻才知道它的右边界，所以先写 right 再弹出
			top := monoStack[len(monoStack)-1]
			right[top] = i
			monoStack = monoStack[:len(monoStack)-1]
		}
		if len(monoStack) == 0 {
			left[i] = -1
		} else {
			left[i] = monoStack[len(monoStack)-1]
		}
		monoStack = append(monoStack, i)
	}

	ans := 0
	for i := 0; i < n; i++ {
		if area := (right[i] - left[i] - 1) * heights[i]; area > ans {
			ans = area
		}
	}
	return ans
}

// 独立的枚举左右边界答案，只用于交叉验证，不参与题解
func bruteForceHistogram(heights []int) int {
	n := len(heights)
	ans := 0
	for i := 0; i < n; i++ {
		minHeight := math.MaxInt
		for j := i; j < n; j++ {
			if heights[j] < minHeight {
				minHeight = heights[j]
			}
			if area := (j - i + 1) * minHeight; area > ans {
				ans = area
			}
		}
	}
	return ans
}

// 拷贝一份输入，避免某个解法改写切片后影响其他解法
func cloneInts(values []int) []int {
	return append([]int{}, values...)
}

// 四个解法都要过同一组用例；期望值由题面定义独立给出
func checkHistogram(heights []int, want int) {
	cases := []struct {
		name string
		got  int
	}{
		{"enumWidth", largestRectangleAreaEnumWidth(cloneInts(heights))},
		{"enumHeight", largestRectangleAreaEnumHeight(cloneInts(heights))},
		{"twoPass", largestRectangleArea(cloneInts(heights))},
		{"onePass", largestRectangleAreaOnePass(cloneInts(heights))},
	}
	for _, c := range cases {
		if c.got != want {
			panic(fmt.Sprintf("%s(%v) = %d, want %d", c.name, heights, c.got, want))
		}
	}
}

// 随机交叉验证：小规模用暴力基准，四个解法一起比
func crossCheckHistogram() {
	seed := uint32(20240607)
	for round := 0; round < 200; round++ {
		seed = seed*1103515245 + 12345
		n := int((seed>>16)%40) + 1
		heights := make([]int, n)
		for i := 0; i < n; i++ {
			seed = seed*1103515245 + 12345
			// 高度取 0..5，专门制造大量等高段与 0 柱子
			heights[i] = int((seed >> 16) % 6)
		}
		checkHistogram(heights, bruteForceHistogram(heights))
	}
}

func main() {
	// 示例 1：中段的 5、6 与右端的 3 一起决定答案
	checkHistogram([]int{2, 1, 5, 6, 2, 3}, 10)

	// 示例 2：两根柱子，取更高的那根
	checkHistogram([]int{2, 4}, 4)

	// 边界：只有一根柱子
	checkHistogram([]int{1}, 1)

	// 边界：高度为 0，面积为 0
	checkHistogram([]int{0}, 0)

	// 反直觉：中间的 0 把两侧切开，答案是 2 而不是 4
	checkHistogram([]int{2, 0, 2}, 2)

	// 反直觉：等高柱子必须合并宽度，答案是 2 而不是 1
	checkHistogram([]int{1, 1}, 2)

	// 反直觉：三根等高，宽度 3 才是最优
	checkHistogram([]int{5, 5, 5}, 15)

	// 边界：单调递增，最优高度藏在中间（3×3）
	checkHistogram([]int{1, 2, 3, 4, 5}, 9)

	// 边界：单调递减，最优高度藏在中间（3×3）
	checkHistogram([]int{5, 4, 3, 2, 1}, 9)

	// 边界：取高度上限，面积 2×10^4
	checkHistogram([]int{10000, 10000}, 20000)

	// 官方手推例子：所有柱子都不低于 2，高 2 全铺开得到 16
	checkHistogram([]int{6, 7, 5, 2, 4, 5, 9, 3}, 16)

	// 随机交叉验证
	crossCheckHistogram()

	// 大规模用例只喂给 O(n) 的两个解法：面积 10^5 × 10^4 = 10^9
	{
		const n = 100000
		heights := make([]int, n)
		for i := range heights {
			heights[i] = 10000
		}
		if got := largestRectangleArea(heights); got != 1000000000 {
			panic(fmt.Sprintf("twoPass large = %d, want 1000000000", got))
		}
		if got := largestRectangleAreaOnePass(heights); got != 1000000000 {
			panic(fmt.Sprintf("onePass large = %d, want 1000000000", got))
		}
	}

	fmt.Println("all tests passed")
}
