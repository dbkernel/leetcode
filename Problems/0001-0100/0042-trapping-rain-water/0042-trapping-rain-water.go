/*
============================================================================
42. 接雨水（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/trapping-rain-water/
分类：前缀后缀统计 · 双指针（同题另有单调栈解）

给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，
下雨之后能接多少雨水。

示例 1：
输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
输出：6
解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况
下，可以接 6 个单位的雨水（蓝色部分表示雨水）。

示例 2：
输入：height = [4,2,0,3,2,5]
输出：9

提示：
- n == height.length
- 1 <= n <= 2 * 10^4
- 0 <= height[i] <= 10^5
*/
package main

import "fmt"

/*
解法一：按列暴力（每列向两侧找最高）【非最优】

为什么慢：按列算的公式本身是对的 —— 第 i 列的水位等于
min(左侧最高, 右侧最高) - height[i]，因为水往低处流，只能被较矮的那侧挡住。
但若每列都向左右各扫一遍求这两个最高值，单列 O(n)，总计 O(n^2)；
n = 2 * 10^4 时是四亿次比较，必然超时。

换个角度：瓶颈不是公式，而是「左右最高值被重复计算」—— 相邻两列的左最高
只差一个元素。把这些最高值提前存下来就是解法二，能在一趟遍历里只维护两个
变量就是解法四。

关键点：
  · 首尾两列一定接不到水（外侧没有柱子挡水），循环直接取 1 到 n-2
  · 左最高取闭区间 [0, i] 的最大值，右最高取 [i, n-1]，两边都要含 i 本身
  · 水量是逐列累加，不是取最大值：每列的水是独立的一份
  · 另一种按行暴力（逐层数被围住的格子）复杂度是 O(n * maxHeight)，
    方向不同但同样超时，这里实现的是按列这一种

手推一遍：
以示例 1（height = [0,1,0,2,1,0,1,3,2,1,2,1]）为例：
  i = 2 时左最高 1、右最高 3，min = 1，减去 height[2] = 0 → 接 1
  i = 5 时左最高 2、右最高 3，min = 2，减去 0 → 接 2
  逐列相加得 6 ✔

复杂度分析：
  时间 O(n^2)：每列向两侧各扫一次
  空间 O(1)：只用几个临时变量

解法二：动态规划预处理左右最高【非最优】

为什么慢：解法一的重复计算可以一次性摊平 —— leftMax[i] 只比 leftMax[i-1]
多考虑一个 height[i]，rightMax 反着推同理。两趟 O(n) 预处理之后每列 O(1)
查询，时间降到 O(n)。让出来的代价是空间：两个长度 n 的数组。

换个角度：第 i 列的水只由 height[i]、leftMax[i]、rightMax[i] 三个数决定，
两个最高值数组就是一张查表。

关键点：
  · leftMax[0] = height[0]、rightMax[n-1] = height[n-1] 作为递推起点
  · 两趟预处理方向相反：leftMax 从前往后，rightMax 从后往前
  · 水量是 min(leftMax[i], rightMax[i]) - height[i]，非负，不必再取 max
  · 本解法是解法四的展开版：双指针只是把两个数组压缩成两个变量

手推一遍：
同一组数，i = 5 时 leftMax[5] = 2、rightMax[5] = 3，2 - 0 = 2，
逐列累加同样得 6 ✔

复杂度分析：
  时间 O(n)：三次线性扫描
  空间 O(n)：两个长度 n 的数组，最坏时都存满

解法三：单调栈（按行结算凹槽）【非最优】

为什么慢：这里的短板是空间而不是时间 —— 时间同样是 O(n)，但栈要额外
O(n)。单调栈按行算水：栈内下标对应的高度保持单调递减，一旦遇到比栈顶高的
柱子，就说明栈顶那一格被左右两根更高的柱子夹住，可以结算一层水。

换个角度：把接水看成「找每个凹槽的左右边界」。某个下标只有遇到右侧比自己
高的柱子时才可能形成凹槽，而它的左边界正是它出栈后新的栈顶。

关键点：
  · 栈里存下标而不是高度：结算要用下标算宽度
  · 弹出栈顶后若栈为空，说明左侧没有更高的柱子，这一格的水会流出边界，
    必须立刻结束本层，否则会用到不存在的左边界
  · 结算式：(i - left - 1) * (min(height[left], height[i]) - height[bottom])，
    bottom 是刚弹出的下标，left 是新的栈顶，i 是当前下标
  · 一个 i 可能连续结算多层，所以弹出用的是 while 而不是 if
  · Go 用 int 切片当栈，只靠 append 与切片截断维护栈顶

手推一遍：
以示例 2（height = [4,2,0,3,2,5]）为例：
  下标 0(4)、1(2)、2(0) 依次入栈
  遇 3(3)：弹出 2，left = 1，宽 1，高 min(2,3) - 0 = 2 → 接 2
          再弹 1，left = 0，宽 2，高 min(4,3) - 2 = 1 → 接 2
  遇 5(5)：依次弹出 3、2 并结算，共 5
  合计 2 + 2 + 5 = 9 ✔

复杂度分析：
  时间 O(n)：每个下标入栈、出栈各一次
  空间 O(n)：栈最多存 n 个下标

解法四：双指针【最优解】

为什么慢：解法二的两个数组只是在回答「i 左边最高多少、右边最高多少」，
但这两个答案不必同时知道。若 leftMax < rightMax，那么 min(leftMax, rightMax)
已经确定就是 leftMax —— 右边具体多高都不影响这一列的结算。

换个角度：左右指针从两端向中间收，各自维护本侧的历史最高。每次比较
leftMax 与 rightMax，哪一侧小就结算哪一侧，因为那一侧的上界已被另一侧更高
的柱子担保。

关键点：
  · 每步先更新本侧最高，再结算，否则当前柱子会被当成自己的边界
  · 结算左边时用 leftMax - height[left]，这一步合法只在 leftMax <= rightMax 时
  · 一次只移动一侧指针，移动后立刻在下一轮更新该侧最高
  · 循环条件是 left < right，相遇的那一列不必再算，两侧最高已覆盖它
  · 时间 O(n)、空间 O(1)，是本题唯一同时压到两个下界的写法

手推一遍：
以示例 1 为例：
  起初 leftMax = 0、rightMax = 1，左边更小，结算 i=0 得 0，left 右移
  随后 rightMax 涨到 2：i=2 结算 1，right 侧的 i=9 结算 1
  right 移到下标 7（高度 3）后 rightMax = 3，而 leftMax 只有 2
  于是 i=4、5、6 依次结算 1、2、1
  合计 0 + 1 + 1 + 1 + 2 + 1 = 6 ✔

复杂度分析：
  时间 O(n)：两个指针各走一遍数组，合计 n 步
  空间 O(1)：四个整型变量
*/

// 解法一：按列暴力
func trapBrute(height []int) int {
	n := len(height)
	ans := 0
	// 首尾两列没有外侧柱子挡水，必然为 0
	for i := 1; i < n-1; i++ {
		leftMax := 0
		rightMax := 0
		for j := 0; j <= i; j++ {
			if height[j] > leftMax {
				leftMax = height[j]
			}
		}
		for j := i; j < n; j++ {
			if height[j] > rightMax {
				rightMax = height[j]
			}
		}
		// 高度非负，min 一定不小于 height[i]，不用再取 max
		if leftMax < rightMax {
			ans += leftMax - height[i]
		} else {
			ans += rightMax - height[i]
		}
	}
	return ans
}

// 解法二：预处理左右最高
func trapDP(height []int) int {
	n := len(height)
	leftMax := make([]int, n)
	rightMax := make([]int, n)

	leftMax[0] = height[0]
	for i := 1; i < n; i++ {
		leftMax[i] = leftMax[i-1]
		if height[i] > leftMax[i] {
			leftMax[i] = height[i]
		}
	}

	rightMax[n-1] = height[n-1]
	for i := n - 2; i >= 0; i-- {
		rightMax[i] = rightMax[i+1]
		if height[i] > rightMax[i] {
			rightMax[i] = height[i]
		}
	}

	ans := 0
	for i := 1; i < n-1; i++ {
		if leftMax[i] < rightMax[i] {
			ans += leftMax[i] - height[i]
		} else {
			ans += rightMax[i] - height[i]
		}
	}
	return ans
}

// 解法三：单调栈，栈里存下标
func trapStack(height []int) int {
	n := len(height)
	st := []int{} // 对应高度从栈底到栈顶单调递减
	ans := 0

	for i := 0; i < n; i++ {
		for len(st) > 0 && height[st[len(st)-1]] < height[i] {
			bottom := st[len(st)-1]
			st = st[:len(st)-1]
			// 左侧没有更高的柱子，水会流出边界，这一层结算不了
			if len(st) == 0 {
				break
			}
			left := st[len(st)-1]
			width := i - left - 1
			h := height[left]
			if height[i] < h {
				h = height[i]
			}
			ans += width * (h - height[bottom])
		}
		st = append(st, i)
	}
	return ans
}

// 解法四：双指针
func trap(height []int) int {
	left := 0
	right := len(height) - 1
	leftMax := 0
	rightMax := 0
	ans := 0

	for left < right {
		if height[left] > leftMax {
			leftMax = height[left]
		}
		if height[right] > rightMax {
			rightMax = height[right]
		}

		// 左侧上界更小，右侧必然有一根不低于它的柱子兜底，可直接结算
		if leftMax < rightMax {
			ans += leftMax - height[left]
			left++
		} else {
			ans += rightMax - height[right]
			right--
		}
	}
	return ans
}

// ===== 测试辅助：四个解法共用一份输入，逐个断言 =====
func check(height []int, want int) {
	if got := trapBrute(append([]int(nil), height...)); got != want {
		panic(fmt.Sprintf("brute(%v) = %d, want %d", height, got, want))
	}
	if got := trapDP(append([]int(nil), height...)); got != want {
		panic(fmt.Sprintf("dp(%v) = %d, want %d", height, got, want))
	}
	if got := trapStack(append([]int(nil), height...)); got != want {
		panic(fmt.Sprintf("stack(%v) = %d, want %d", height, got, want))
	}
	if got := trap(append([]int(nil), height...)); got != want {
		panic(fmt.Sprintf("two-pointers(%v) = %d, want %d", height, got, want))
	}
}

func main() {
	// 示例 1：中间高两侧低，多个凹槽
	check([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}, 6)

	// 示例 2：两端高、中间一列凹到底
	check([]int{4, 2, 0, 3, 2, 5}, 9)

	// 边界：全 0，没有柱子也没有水
	check([]int{0, 0, 0}, 0)

	// 边界：单调递增，水全部往右流走
	check([]int{0, 1, 2, 3, 4}, 0)

	// 边界：单调递减，水全部往左流走
	check([]int{4, 3, 2, 1, 0}, 0)

	// 边界：只有一列
	check([]int{5}, 0)
	check([]int{0}, 0)

	// 边界：等高的平顶，中间不存水
	check([]int{3, 3, 3}, 0)

	// 反直觉：最短的凹槽，宽 1 深 2
	check([]int{2, 0, 2}, 2)

	// 反直觉：两端等高，中间逐级下降再上升
	check([]int{2, 1, 0, 1, 2}, 4)

	// 反直觉：中间有一根柱子比两端都高，左右各自成池
	check([]int{5, 4, 1, 2, 1, 5}, 12)

	// 反直觉：最高柱不在端点，右侧水位由更矮的那侧决定
	check([]int{0, 7, 1, 4, 6}, 7)

	// 反直觉：两侧高、中心低，且内部还有高低起伏
	check([]int{4, 2, 3, 2, 4}, 5)

	fmt.Println("all tests passed")
}
