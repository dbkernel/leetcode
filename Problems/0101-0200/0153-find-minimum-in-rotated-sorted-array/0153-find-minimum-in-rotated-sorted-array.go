/*
============================================================================
153. 寻找旋转排序数组中的最小值（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/
分类：二分查找 · 旋转数组 · 最小值

已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到
输入数组。例如，原数组 nums = [0,1,2,4,5,6,7] 在变化后可能得到：
若旋转 4 次，则可以得到 [4,5,6,7,0,1,2]
若旋转 7 次，则可以得到 [0,1,2,4,5,6,7]
注意，数组 [a[0], a[1], a[2], ..., a[n-1]] 旋转一次 的结果为数组
[a[n-1], a[0], a[1], a[2], ..., a[n-2]] 。

给你一个元素值 互不相同 的数组 nums ，它原来是一个升序排列的数组，并按上述
情形进行了多次旋转。请你找出并返回数组中的 最小元素 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。

示例 1：
输入：nums = [3,4,5,1,2]
输出：1
解释：原数组为 [1,2,3,4,5] ，旋转 3 次得到输入数组。

示例 2：
输入：nums = [4,5,6,7,0,1,2]
输出：0
解释：原数组为 [0,1,2,4,5,6,7] ，旋转 4 次得到输入数组。

示例 3：
输入：nums = [11,13,15,17]
输出：11
解释：原数组为 [11,13,15,17] ，旋转 4 次得到输入数组。

提示：
- n == nums.length
- 1 <= n <= 5000
- -5000 <= nums[i] <= 5000
- nums 中的所有整数 互不相同
- nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转
*/
package main

import "fmt"

/*
解法：与右端点比较收缩，退出时 left 就是最小值下标【最优解】

为什么慢：最小值是"第一个比前一个元素小的位置"，顺着扫一遍 O(n)。
旋转数组的结构已经保证两段各自升序，这份信息足够支撑二分，逐项扫描
等于把它浪费掉。

换个角度：把 nums[mid] 和右端点比，而不是和左端点比。右端点落在第二段
（未旋转时落在整段末尾），它一定不小于最小值，因此
  · nums[mid] > nums[right]：mid 还在第一段，最小值严格在 mid 右侧
  · nums[mid] < nums[right]：mid 与 right 同处第二段，最小值在 mid 左侧
    （可能就是 mid 本身）
每次都能判定最小值在 mid 的哪一侧，不需要额外判断"哪半有序"。

关键点：
  · 区间取左闭右闭 [left, right]，但循环条件写 left < right：
    right 自己可能就是答案，不能用 right = mid - 1 把它排除
  · nums[mid] > nums[right] 时 left = mid + 1，mid 一定不是答案
  · 否则 right = mid，保留 mid 作为候选
  · 退出时 left == right，最小值一定停在这个点上，返回 nums[left]
  · 单元素数组不进循环，返回 nums[0]，无需特判

手推一遍：nums = [3,4,5,1,2]
  left=0,right=4,mid=2，nums[2]=5 > nums[4]=2 → left=3
  left=3,right=4,mid=3，nums[3]=1 < nums[4]=2 → right=3
  left==right==3 → 返回 nums[3]=1 ✔

复杂度分析：
  时间 O(log n)：每轮区间长度减半，区间长度 1 时退出
  空间 O(1)：只用 left / right / mid 三个下标
*/
func findMin(nums []int) int {
	left, right := 0, len(nums)-1

	// 不变式：最小值始终落在闭区间 [left, right] 内
	for left < right {
		mid := left + (right-left)/2
		// 中点在上升段，最小值只能出现在它右边
		if nums[mid] > nums[right] {
			left = mid + 1
		} else {
			right = mid
		}
	}
	return nums[left]
}

// 单解法也要覆盖全部用例，失败时能看出是哪一组数据
func check(nums []int, want int) {
	if got := findMin(nums); got != want {
		panic(fmt.Sprintf("findMin(%v) = %d, want %d", nums, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{3, 4, 5, 1, 2}, 1)

	// 示例 2
	check([]int{4, 5, 6, 7, 0, 1, 2}, 0)

	// 示例 3：旋转点在首，数组整体升序
	check([]int{11, 13, 15, 17}, 11)

	// 边界：单元素
	check([]int{1}, 1)

	// 边界：完全升序（旋转 n 次），最小值在下标 0
	check([]int{1, 2, 3, 4, 5}, 1)

	// 边界：只旋转一次，最小值在末尾（旋转点在尾）
	check([]int{2, 3, 4, 5, 1}, 1)

	// 边界：两个元素，旋转点在尾
	check([]int{2, 1}, 1)
	check([]int{1, 2}, 1)

	// 边界：旋转点在正中
	check([]int{5, 6, 1, 2, 3, 4}, 1)

	// 边界：负数值参与比较，不能拿 0 当初始答案
	check([]int{-1, 2, -3}, -3)
	check([]int{-3, -2, -1}, -3)

	// 说明：整体降序（如 [5,4,3,2,1]）不是升序数组的旋转结果，
	// 不能作为合法输入；"局部降序"由只旋转一次的用例覆盖

	fmt.Println("all tests passed")
}
