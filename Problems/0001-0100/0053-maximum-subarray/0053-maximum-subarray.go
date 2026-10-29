/*
============================================================================
53. 最大子数组和（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-subarray/
分类：前缀后缀统计 · 动态规划（Kadane）

给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组
（子数组最少包含一个元素），返回其最大和。

子数组 是数组中的一个连续部分。

示例 1：
输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
输出：6
解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。

示例 2：
输入：nums = [1]
输出：1

示例 3：
输入：nums = [5,4,-1,7,8]
输出：23

提示：
- 1 <= nums.length <= 10^5
- -10^4 <= nums[i] <= 10^4

进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
*/
package main

import "fmt"

/*
解法一：动态规划（Kadane，一次遍历）【最优解】

为什么慢：最直接的做法是枚举左右端点，再用前缀和 O(1) 求出这段的和，
一共 O(n^2) 个子数组；换一种省法也只是固定右端点后枚举左端点，仍是 O(n^2)。
瓶颈在于「每换一个右端点就重新考虑所有左端点」，而绝大多数左端点早已被
前一个右端点的最优值淘汰。

换个角度：把答案拆成「以 i 结尾」的子问题。设 f[i] 为以 nums[i] 结尾的最大
子数组和，它要么单独取 nums[i]，要么接在 f[i-1] 之后 —— 因为「以 i 结尾」
这个约束把前面所有历史压缩成了一个数，递推立刻成立。

关键点：
  · f[i] 的定义必须以 i 结尾，不能是「前 i 个元素里的最大和」，否则写不出递推
  · 每一步都用 f[i] 更新全局答案；只返回 f[n-1] 会在全负数时漏掉中间的最大值
  · f[i-1] + x 与 x 取大，等价于「前缀一旦为负就丢弃」，不需要显式判断正负
  · 初值取 nums[0]，不能取 0，否则全负数数组会错误地返回 0

手推一遍：
以示例 1（nums = [-2,1,-3,4,-1,2,1,-5,4]）为例：
  f = -2 → 1（丢弃 -2）→ -2（1-3 不如单取 -3，但仍记 1 作答案）→ 4 → 3
    → 5 → 6 → 1 → 5
  途中最大值 6，对应子数组 [4,-1,2,1] ✔

复杂度分析：
  时间 O(n)：每个元素一次加法、两次比较
  空间 O(1)：只保留 f 与 best，不需要长度为 n 的 f 数组

解法二：分治（维护区间四值）【非最优】

为什么慢：一种常见的分治写法会在合并时线性扫描求「跨中线的最大和」，
每次合并 O(区间长度)，总时间退化成 O(n log n)。扫描本身就是浪费：
跨中线的最大和只需要左右区间各一个数。

换个角度：让每个区间自己汇报四个数，合并只做常数次运算：
  lSum 以左端点为起点的最大和、rSum 以右端点为终点的最大和、
  mSum 区间内最大子段和、iSum 区间总和。
跨中线的最大和一定是「左区间的后缀 + 右区间的前缀」，即 a.rSum + b.lSum。

关键点：
  · lSum 必须包含左端点，否则合并时接不上右半段，这个字段就失去了意义
  · mSum 取三者最大：全在左、全在右、跨中线（a.rSum + b.lSum）
  · 长度为 1 的区间四值都等于该元素，这是递归出口
  · Go 没有元组，四值用一个结构体返回，省得四个返回值到处拆包

手推一遍：
以 nums = [4,-1,2,1] 为例，拆成 [4,-1] 与 [2,1]：
  [4,-1] → lSum 4、rSum 3、mSum 4、iSum 3
  [2,1]  → lSum 3、rSum 3、mSum 3、iSum 3
  合并：iSum = 6；lSum = max(4, 3+3) = 6；rSum = max(3, 3+3) = 6
        mSum = max(4, 3, 3+3) = 6 ✔（正好是整个数组）

复杂度分析：
  时间 O(n)：共 2n-1 个区间节点，每个节点常数次运算；四值写法不是 O(n log n)
  空间 O(log n)：递归栈深度为划分层数，四值随返回值传递，不额外开数组

解法三：前缀和的最值差（一次遍历）【最优解，与解法一并列】

为什么慢：子数组和写作 S[j] - S[i]，枚举 i、j 又是 O(n^2)。瓶颈仍是
「每个右端点都回头看所有左端点」，而真正有用的只有其中最小的那个前缀和。

换个角度：固定右端点 j 后，S[j] - S[i] 要最大，只需让 S[i] 取
S[0..j-1] 的最小值。于是问题变成「一边右移，一边维护历史最小前缀和」。

关键点：
  · 先用当前前缀和减「此前」的最小前缀和，再把当前前缀和纳入最小值；
    顺序一颠倒就允许了空子数组，全负数数组会返回 0
  · minS 初值取 0，代表空前缀，因此 [1] 这样的单元素数组也能得到 1
  · 答案初值不能设 0，理由同上：全负数数组必须返回最大的那个负数
  · 与解法一的关系：f[i] = S[i] - min(S[0..i-1])，两条式子是同一件事

手推一遍：
以示例 1 为例，前缀和（含 S[0] = 0）是
  0, -2, -1, -4, 0, -1, 1, 2, -3, 1：
  j = 4 时 S[j] = 0，此前最小 -4，差 4
  j = 7 时 S[j] = 2，此前最小 -4，差 6 ✔
  之后 S 再没超过 2，6 就是答案

复杂度分析：
  时间 O(n)：一次遍历
  空间 O(1)：cur 与 minS 两个变量，不必保存前缀和数组
*/

// 解法一：动态规划
func maxSubArray(nums []int) int {
	f := nums[0]
	best := nums[0]
	for i := 1; i < len(nums); i++ {
		// 接上负数前缀一定更小，取大即自动选择从头开始
		if f+nums[i] > nums[i] {
			f = f + nums[i]
		} else {
			f = nums[i]
		}
		if f > best {
			best = f
		}
	}
	return best
}

// 解法二：分治所需的区间四值
type seg struct {
	lSum int
	rSum int
	mSum int
	iSum int
}

func maxSubArrayDivide(nums []int) int {
	return rangeMax(nums, 0, len(nums)-1).mSum
}

func rangeMax(nums []int, left, right int) seg {
	// 单元素区间四值都等于它自己
	if left == right {
		x := nums[left]
		return seg{x, x, x, x}
	}
	mid := left + (right-left)/2
	a := rangeMax(nums, left, mid)
	b := rangeMax(nums, mid+1, right)

	// 跨中线的最大和只能由「a 的后缀 + b 的前缀」构成
	cross := a.rSum + b.lSum
	out := seg{
		iSum: a.iSum + b.iSum,
		lSum: max2(a.lSum, a.iSum+b.lSum),
		rSum: max2(b.rSum, b.iSum+a.rSum),
		mSum: max2(max2(a.mSum, b.mSum), cross),
	}
	return out
}

func max2(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// 解法三：前缀和的最值差
func maxSubArrayPrefix(nums []int) int {
	// minS 初值 0 代表空前缀，覆盖「子数组从 0 开始」的情况
	minS := 0
	cur := 0
	best := nums[0]
	for _, x := range nums {
		cur += x
		// 先算差再更新 minS，否则 i == j 会让空子数组参与比较
		if v := cur - minS; v > best {
			best = v
		}
		if cur < minS {
			minS = cur
		}
	}
	return best
}

// ===== 测试辅助：三个解法共用一份输入，逐个断言 =====
func check(nums []int, want int) {
	if got := maxSubArray(append([]int(nil), nums...)); got != want {
		panic(fmt.Sprintf("dp(%v) = %d, want %d", nums, got, want))
	}
	if got := maxSubArrayDivide(append([]int(nil), nums...)); got != want {
		panic(fmt.Sprintf("divide(%v) = %d, want %d", nums, got, want))
	}
	if got := maxSubArrayPrefix(append([]int(nil), nums...)); got != want {
		panic(fmt.Sprintf("prefix(%v) = %d, want %d", nums, got, want))
	}
}

func main() {
	// 示例 1：最优子数组在中间，前后都有更大的正数被负数隔开
	check([]int{-2, 1, -3, 4, -1, 2, 1, -5, 4}, 6)

	// 示例 2：单元素数组
	check([]int{1}, 1)

	// 示例 3：整段都取
	check([]int{5, 4, -1, 7, 8}, 23)

	// 边界：全负数，答案必须是最大的那个负数而不是 0
	check([]int{-3, -1, -2}, -1)
	check([]int{-2, -3, -1, -5}, -1)

	// 边界：单个负数
	check([]int{-1}, -1)

	// 边界：0 与负数混排，答案为 0
	check([]int{0}, 0)
	check([]int{-1, 0, -2}, 0)

	// 反直觉：整个数组的和是负数，但中间一段是正的
	check([]int{-5, 4, -1, 2, 1, -3}, 6)

	// 反直觉：最优子数组是单个中间元素
	check([]int{-2, 1, -3}, 1)

	// 反直觉：前缀最大和出现在中途，末尾被负数拖低
	check([]int{8, -19, 5, -4, 20}, 21)

	// 规模边界：1 到 100 全部为正，答案 5050
	nums := []int{}
	for i := 1; i <= 100; i++ {
		nums = append(nums, i)
	}
	check(nums, 5050)

	fmt.Println("all tests passed")
}
