/*
============================================================================
33. 搜索旋转排序数组（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/search-in-rotated-sorted-array/
分类：二分查找 · 旋转数组

整数数组 nums 按升序排列，数组中的值 互不相同 。

在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行
了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0],
nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如，
[0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。

给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值
target ，则返回它的下标，否则返回 -1 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。

示例 1：
输入：nums = [4,5,6,7,0,1,2], target = 0
输出：4

示例 2：
输入：nums = [4,5,6,7,0,1,2], target = 3
输出：-1

示例 3：
输入：nums = [1], target = 0
输出：-1

提示：
- 1 <= nums.length <= 5000
- -10^4 <= nums[i] <= 10^4
- nums 中的每个值都 独一无二
- 题目数据保证 nums 在预先未知的某个下标上进行了旋转
- -10^4 <= target <= 10^4
*/
package main

import "fmt"

/*
解法一：一次二分，先判有序的那一半【最优解】

为什么慢：旋转之后整体不再有序，普通二分的中点左右两侧不一定都升序，
直接按 nums[mid] 与 target 的大小关系选边会把答案区间扔错。若因此退回
线性扫描，时间是 O(n)，不满足题目要求。

换个角度：任何时刻 nums[left..mid] 和 nums[mid..right] 里至少有一半是
升序的。升序的那一半可以用两个端点直接判断 target 在不在里面：
落在里面，答案只可能在这一半；不落在里面，答案只能在另一半。未排序的
那一半不参与判断，二分照样每轮砍掉一半区间。

关键点：
  · 区间取左闭右闭 [left, right]，循环条件 left <= right；收缩时写
    right = mid - 1 / left = mid + 1，两端都不再保留
  · 判有序用 nums[left] <= nums[mid]：left == mid 时等号成立，
    让长度为 1 的区间也走"左半有序"分支
  · 目标区间必须是闭开区间 nums[left] <= target && target < nums[mid]，
    mid 处已经比较过，不放进区间
  · 右半有序时对称写 nums[mid] < target && target <= nums[right]

手推一遍：nums = [4,5,6,7,0,1,2]，target = 0
  left=0, right=6, mid=3，nums[3]=7，nums[0]=4 <= 7，左半有序；
  0 不在 [4,7) 内 → left=4
  left=4, right=6, mid=5，nums[5]=1，nums[4]=0 <= 1，左半有序；
  0 在 [0,1) 内 → right=4
  left=4, right=4, mid=4，nums[4]=0 == target → 返回 4 ✔

复杂度分析：
  时间 O(log n)：每轮区间长度减半，与普通二分相同
  空间 O(1)：只用了 left / right / mid 三个下标
*/
func search(nums []int, target int) int {
	left, right := 0, len(nums)-1

	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] == target {
			return mid
		}

		// 左半有序：命中区间时右半整体被排除，反之左半被排除
		if nums[left] <= nums[mid] {
			if nums[left] <= target && target < nums[mid] {
				right = mid - 1
			} else {
				left = mid + 1
			}
		} else if nums[mid] < target && target <= nums[right] {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	// 区间排空仍未命中，说明 target 不在数组里
	return -1
}

/*
解法二：先二分找旋转点，再在有序段上二分【非最优】

为什么慢：把一件事拆成两次二分 —— 先找最小值下标 pivot，再判断 target
落在 [0, pivot) 还是 [pivot, n)，最后在选中的有序段上做普通二分。两次
都是 O(log n)，渐进复杂度与解法一相同，但比较次数约为两倍，还多了一次
选段分支，常数更大。

换个角度：旋转数组可以看成"两段各自升序的段"，pivot 正是第二段的起点。
找到 pivot 之后，问题退化成两段互不重叠的有序数组上的普通二分。

关键点：
  · 找 pivot 用 nums[mid] 与 nums[right] 比较：nums[mid] > nums[right]
    时最小值在 mid 右侧（不含 mid），否则在 mid 或它左侧（含 mid）
  · 退出时 left == right，交点就是最小值下标，也是第二段起点
  · 选段用 target >= nums[0]：第一段的值都 >= nums[0]，第二段都小于它
  · pivot == 0 表示数组未旋转，此时两段合并成一整段 [0, n-1]
  · 两段内的查找都用左闭右闭区间，第二段右端是 n-1

手推一遍：nums = [4,5,6,7,0,1,2]，target = 0
  找 pivot：left=0,right=6,mid=3，7 > nums[6]=2 → left=4
  left=4,right=6,mid=5，1 <= nums[6]=2 → right=5
  left=4,right=5,mid=4，0 <= nums[5]=1 → right=4 → pivot=4
  target=0 < nums[0]=4，在第二段 [4,6] 二分，命中下标 4 ✔

复杂度分析：
  时间 O(log n)：两次二分各自减半，常数约为解法一的两倍
  空间 O(1)：只用下标变量
*/
func searchByPivot(nums []int, target int) int {
	n := len(nums)

	// 找最小值下标：它同时是第一段的终点边界和第二段的起点
	left, right := 0, n-1
	for left < right {
		mid := left + (right-left)/2
		// mid 落在第一段，最小值不可能在 [left, mid]
		if nums[mid] > nums[right] {
			left = mid + 1
		} else {
			right = mid
		}
	}
	pivot := left

	if pivot == 0 {
		// 未旋转：两段连成一整段，直接全区间二分
		left, right = 0, n-1
	} else if target >= nums[0] {
		// 第一段每个值都不小于 nums[0]，第二段每个值都小于它
		left, right = 0, pivot-1
	} else {
		left, right = pivot, n-1
	}

	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] == target {
			return mid
		}
		if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return -1
}

// 两个解法都要跑同一组用例，答案只比较返回值
func check(nums []int, target, want int) {
	if got := search(nums, target); got != want {
		panic(fmt.Sprintf("optimal(%v,%d) = %d, want %d", nums, target, got, want))
	}
	if got := searchByPivot(nums, target); got != want {
		panic(fmt.Sprintf("pivot(%v,%d) = %d, want %d", nums, target, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{4, 5, 6, 7, 0, 1, 2}, 0, 4)

	// 示例 2：目标不存在
	check([]int{4, 5, 6, 7, 0, 1, 2}, 3, -1)

	// 示例 3：单元素且未命中
	check([]int{1}, 0, -1)

	// 边界：单元素命中
	check([]int{1}, 1, 0)

	// 边界：完全升序（旋转点在首，相当于旋转了 n 次）
	check([]int{1, 2, 3, 4, 5}, 3, 2)
	check([]int{1, 2, 3, 4, 5}, 0, -1)
	check([]int{1, 2, 3, 4, 5}, 5, 4)

	// 边界：只旋转一次，最小值落在末尾（旋转点在尾）
	check([]int{2, 3, 4, 5, 1}, 1, 4)
	check([]int{2, 3, 4, 5, 1}, 2, 0)

	// 边界：旋转点在 n/2 附近，左右两段长度相等
	check([]int{4, 5, 0, 1, 2, 3}, 3, 5)
	check([]int{4, 5, 0, 1, 2, 3}, 0, 2)

	// 边界：两个元素，旋转点在尾
	check([]int{3, 1}, 1, 1)
	check([]int{3, 1}, 3, 0)

	// 边界：目标是最小值或最大值，两者都在段边界上
	check([]int{6, 7, 1, 2, 3, 4, 5}, 1, 2)
	check([]int{6, 7, 1, 2, 3, 4, 5}, 7, 1)

	// 说明：整体降序（如 [5,4,3,2,1]）不是升序数组的旋转结果，
	// 不能作为合法输入；降序片段由"只旋转一次"的用例覆盖

	fmt.Println("all tests passed")
}
