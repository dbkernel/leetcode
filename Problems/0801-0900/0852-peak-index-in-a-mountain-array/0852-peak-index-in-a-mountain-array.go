/*
============================================================================
852. 山脉数组的峰顶索引（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/peak-index-in-a-mountain-array/
分类：二分查找 · 单峰数组

符合下列属性的数组 arr 称为 山脉数组 ：
- arr.length >= 3
- 存在 i（0 < i < arr.length - 1）使得：
  - arr[0] < arr[1] < ... arr[i-1] < arr[i]
  - arr[i] > arr[i+1] > ... arr[arr.length - 1]

给你由整数组成的山脉数组 arr ，返回满足
arr[0] < arr[1] < ... < arr[i - 1] < arr[i] > arr[i + 1] > ...
> arr[arr.length - 1] 的下标 i 。

你必须设计并实现时间复杂度为 O(log(n)) 的解决方案。

示例 1：
输入：arr = [0,1,0]
输出：1

示例 2：
输入：arr = [0,2,1,0]
输出：1

示例 3：
输入：arr = [0,10,5,2]
输出：1

提示：
- 3 <= arr.length <= 10^5
- 0 <= arr[i] <= 10^6
- 题目数据保证 arr 是一个山脉数组

进阶：
很容易想到时间复杂度 O(n) 的解决方案，你可以设计一个 O(log(n)) 的解决方案吗？
*/
package main

import "fmt"

/*
解法一：线性扫描，找第一个下降的位置【非最优】

为什么慢：峰顶是"最后一个仍严格上升的位置"，从下标 0 开始逐个比较
arr[i] 与 arr[i+1]，第一次出现 arr[i] > arr[i+1] 时停下即可，时间 O(n)。
n 可以到 10^5，而题目明确要求 O(log n)，线性扫描只能当下界对照。

换个角度：山脉数组先严格升、再严格降，这种单峰结构本身就是二分的前提，
不用完整扫一遍也能定位转折点。

关键点：
  · 从 i = 0 开始，只要 arr[i] < arr[i+1] 就右移
  · 山脉数组保证峰顶严格在内部（0 < i < n-1），所以 i+1 不会越界
  · 停下时的 i 满足 arr[i] > arr[i+1]，且之前一路上升，正是峰顶

手推一遍：arr = [0,2,1,0]
  i=0：0 < 2 → i=1
  i=1：2 < 1 不成立，停 → 返回 1 ✔

复杂度分析：
  时间 O(n)：最坏要一路扫到峰顶
  空间 O(1)：只用一个下标
*/
func peakIndexInMountainArrayLinear(arr []int) int {
	i := 0
	for arr[i] < arr[i+1] {
		i++
	}
	return i
}

/*
解法二：二分比较中点与右邻元素【最优解】

为什么慢：线性扫描只用到"当前这一步是否上升"，每步只排除一个位置；
单峰结构其实一次就能排除一半。

换个角度：取 mid，比较 arr[mid] 与 arr[mid+1]：
  · arr[mid] < arr[mid+1]：还在上升段，峰顶严格在 mid 右侧
  · arr[mid] > arr[mid+1]：已经在下坡，峰顶不晚于 mid
每轮砍掉一半，O(log n)。

关键点：
  · 候选区间取左闭右闭 [left, right]，right 初始为 n-2：
    峰顶不可能是最后一个元素，这一格永远不必作为答案
  · 用 arr[mid] 与 arr[mid+1] 比较，mid < right <= n-2，所以 mid+1 不越界
  · arr[mid] < arr[mid+1] 时 left = mid + 1；否则 right = mid（mid 仍可能是峰）
  · 循环条件 left < right，退出时 left == right == 峰顶下标

手推一遍：arr = [0,10,5,2]
  left=0,right=2,mid=1，arr[1]=10 > arr[2]=5 → right=1
  left=0,right=1,mid=0，arr[0]=0 < arr[1]=10 → left=1
  left==right==1 → 返回 1 ✔

复杂度分析：
  时间 O(log n)：每轮候选区间减半
  空间 O(1)：只用 left / right / mid 三个下标
*/
func peakIndexInMountainArray(arr []int) int {
	left, right := 0, len(arr)-2

	for left < right {
		mid := left + (right-left)/2
		// 上升段意味着峰顶不可能停在 mid 或它左边
		if arr[mid] < arr[mid+1] {
			left = mid + 1
		} else {
			right = mid
		}
	}
	return left
}

// 两个解法都要跑同一组用例，答案只比较返回值
func check(arr []int, want int) {
	if got := peakIndexInMountainArrayLinear(arr); got != want {
		panic(fmt.Sprintf("linear(%v) = %d, want %d", arr, got, want))
	}
	if got := peakIndexInMountainArray(arr); got != want {
		panic(fmt.Sprintf("binary(%v) = %d, want %d", arr, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{0, 1, 0}, 1)

	// 示例 2
	check([]int{0, 2, 1, 0}, 1)

	// 示例 3
	check([]int{0, 10, 5, 2}, 1)

	// 边界：最短的合法山脉，峰顶只能在下标 1
	check([]int{3, 4, 2}, 1)
	check([]int{1, 2, 1}, 1)

	// 说明：山脉数组长度至少 3，单元素 / 两元素都不是合法输入，
	// 最短形态由上面的 [3,4,2] 覆盖

	// 边界：峰顶尽量靠右，只在末尾留一个下降元素
	check([]int{0, 1, 2, 3, 2}, 3)
	check([]int{5, 6, 7, 8, 4}, 3)

	// 边界：长数组，峰顶正好在正中，验证二分真的每轮减半
	arr := []int{}
	for i := 0; i <= 5000; i++ {
		arr = append(arr, i)
	}
	for i := 4999; i >= 0; i-- {
		arr = append(arr, i)
	}
	check(arr, 5000)

	// 边界：取到上限值的山脉，比较不能溢出或其他类型问题
	check([]int{0, 1000000, 0}, 1)

	// 说明：整体升序 / 整体降序都不是山脉数组（峰顶必须在内部），
	// 不能作为合法输入，因此不在这里断言

	fmt.Println("all tests passed")
}
