/*
============================================================================
15. 三数之和（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/3sum/
分类：哈希表 · 排序 + 双指针

给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]]
满足 i != j、i != k 且 j != k ，同时还满足
nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。

注意：答案中不可以包含重复的三元组。

示例 1：
输入：nums = [-1,0,1,2,-1,-4]
输出：[[-1,-1,2],[-1,0,1]]
解释：
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
注意，输出的顺序和三元组的顺序并不重要。

示例 2：
输入：nums = [0,1,1]
输出：[]
解释：唯一可能的三元组和不为 0 。

示例 3：
输入：nums = [0,0,0]
输出：[[0,0,0]]
解释：唯一可能的三元组和为 0 。

提示：
- 3 <= nums.length <= 3000
- -10^5 <= nums[i] <= 10^5
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法：排序 + 双指针（固定第一个数，后两数向中间收）【最优解】

为什么慢：直接三重循环枚举下标，再对三元组排序去重，最坏（全 0）会产生
O(n^3) 个重复结果。固定第一个数 a 后，问题变成在剩余元素里找
b + c = -a；若对每个 a 再暴力枚举 b、c，总时间仍是 O(n^3)。改用哈希表
固定 a 后把 b 放进表里查 -a-b，时间可降到 O(n^2)，但额外占 O(n) 空间，
而且同一组 (a,b,c) 会因下标不同被反复枚举，最后还要用集合去重，
空间和常数都更差。

换个角度：排序后三元组天然满足 a <= b <= c，只要每一层枚举都跳过相邻
重复值，就不会产生重复三元组。固定 a 后，b 从左向右增大时，满足
b + c = -a 的 c 只会从右向左减小，两个指针可以单调地相向移动。

关键点：
  · 先排序，让 a <= b <= c 成为天然顺序，重复值相邻
  · 第一层遇到 nums[first] == nums[first-1] 直接跳过
  · 第二层遇到 nums[second] == nums[second-1] 直接跳过
  · 内层循环只在 second < third 时收缩 third，保证 b 在 c 左侧
  · 指针重合就 break：b 再变大，需要的 c 更小，已无合法位置
  · 找到答案后 second 继续右移，由第二层的跳过逻辑负责去重

以示例 1（nums = [-1,0,1,2,-1,-4]）为例：
  排序后 [-4,-1,-1,0,1,2]
  a = -4：需要 b+c=4，双指针无解
  a = -1（第一个）：b=-1,c=2 得 [-1,-1,2]；b=0,c=1 得 [-1,0,1]
  a = -1（第二个）：与上一个 a 相同，跳过
  a = 0/1/2：b+c 无法再凑出 0，没有新答案 → 两组 ✔

复杂度分析：
  时间 O(n^2)：排序 O(n log n)，第一层 O(n) 乘双指针 O(n)
  空间 O(1)：除返回值外只用常数个指针；排序本身是原地进行的
*/
func threeSum(nums []int) [][]int {
	sort.Ints(nums)
	n := len(nums)
	ans := [][]int{}

	for first := 0; first < n; first++ {
		// 跳过重复的 a：同一组 (a,b,c) 只可能由第一个 a 枚举到
		if first > 0 && nums[first] == nums[first-1] {
			continue
		}

		// third 是递减指针：b 增大后，能配对的 c 只会更小
		third := n - 1
		target := -nums[first]

		for second := first + 1; second < n; second++ {
			// 跳过重复的 b：固定 a 时，重复 b 只会得到同一组答案
			if second > first+1 && nums[second] == nums[second-1] {
				continue
			}

			// 只在 b 仍在 c 左侧时收缩，避免把同一个下标用两次
			for second < third && nums[second]+nums[third] > target {
				third--
			}

			// 两指针重合：b 再增大时 c 只会更小，不可能再等于 target
			if second == third {
				break
			}

			if nums[second]+nums[third] == target {
				ans = append(ans,
					[]int{nums[first], nums[second], nums[third]})
			}
		}
	}
	return ans
}

// 题面允许任意顺序，先把每个三元组内部和三元组之间都排好再比较
func normalizeTriplets(triplets [][]int) [][]int {
	for _, t := range triplets {
		sort.Ints(t)
	}
	sort.Slice(triplets, func(i, j int) bool {
		for k := 0; k < len(triplets[i]) && k < len(triplets[j]); k++ {
			if triplets[i][k] != triplets[j][k] {
				return triplets[i][k] < triplets[j][k]
			}
		}
		return len(triplets[i]) < len(triplets[j])
	})
	return triplets
}

func sameTriplets(a, b [][]int) bool {
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

func checkTriplets(nums []int, want [][]int) {
	input := append([]int(nil), nums...)
	got := normalizeTriplets(threeSum(input))
	expected := normalizeTriplets(want)
	if !sameTriplets(got, expected) {
		panic(fmt.Sprintf("threeSum(%v) = %v, want %v", nums, got, expected))
	}
}

func main() {
	// 示例 1
	checkTriplets([]int{-1, 0, 1, 2, -1, -4},
		[][]int{{-1, -1, 2}, {-1, 0, 1}})

	// 示例 2
	checkTriplets([]int{0, 1, 1}, [][]int{})

	// 示例 3
	checkTriplets([]int{0, 0, 0}, [][]int{{0, 0, 0}})

	// 边界：空输入（题面保证至少 3 个数，这里只验证健壮性）
	checkTriplets([]int{}, [][]int{})

	// 边界：全正 / 全负，都不可能凑出 0
	checkTriplets([]int{1, 2, 3, 4}, [][]int{})
	checkTriplets([]int{-4, -3, -2, -1}, [][]int{})

	// 边界：四个 0 只能产出一个 [0,0,0]
	checkTriplets([]int{0, 0, 0, 0}, [][]int{{0, 0, 0}})

	// 边界：重复元素与一正一负的配对
	checkTriplets([]int{-2, 0, 1, 1, 2},
		[][]int{{-2, 0, 2}, {-2, 1, 1}})
	checkTriplets([]int{-1, -1, 0, 0, 2, 2}, [][]int{{-1, -1, 2}})

	// 反直觉：a 重复三次、b/c 各有多个选择，仍只能输出 6 组
	checkTriplets([]int{-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6},
		[][]int{{-4, -2, 6}, {-4, 0, 4}, {-4, 1, 3}, {-4, 2, 2},
			{-2, -2, 4}, {-2, 0, 2}})

	fmt.Println("all tests passed")
}
