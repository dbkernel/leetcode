/*
============================================================================
1122. 数组的相对排序（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/relative-sort-array/
分类：哈希表 · 计数排序（按值域下标还原顺序）

给你两个数组，arr1 和 arr2，arr2 中的元素各不相同，arr2 中的每个元素都出现在
arr1 中。

对 arr1 中的元素进行排序，使 arr1 中项的相对顺序和 arr2 中的相对顺序相同。
未在 arr2 中出现过的元素需要按照升序放在 arr1 的末尾。

示例 1：
输入：arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
输出：[2,2,2,1,4,3,3,9,6,7,19]

示例 2：
输入：arr1 = [28,6,22,8,44,17], arr2 = [22,28,8,6]
输出：[22,28,8,6,17,44]

提示：
- 1 <= arr1.length, arr2.length <= 1000
- 0 <= arr1[i], arr2[i] <= 1000
- arr2 中的元素 arr2[i] 各不相同
- arr2 中的每个元素 arr2[i] 都出现在 arr1 中
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法一：自定义排序（用哈希表记名次）【非最优】

为什么慢：要给每个元素定义「谁在前」，最直接的是比较函数里判断「在不在 arr2」的
三种组合，但这种写法容易漏掉一支，也容易把「名次」和「数值」混在一起比较。而且
比较排序本身要 O(m log m)，比按值域直接还原顺序慢一个 log。

换个角度：把「arr2 里的名次」当主键、「自身数值」当次键，问题就退化成普通的二元
组排序：属于 arr2 的元素统一拿类别 0、名次作键，其余统一拿类别 1、自身值作键，
两类元素自动分开，同类内部各自的顺序也唯一确定。

关键点：
  · 排序键取 (类别, 键值)：arr2 中的元素为 (0, 名次)，其余为 (1, 自身值)
  · 名次来自 arr2 的下标，与数值大小无关
  · 类别位不能省：名次和数值量纲不同，直接混比会得出错误的全序
  · 排序原地改动 arr1，直接返回它即可

以示例 2（arr1 = [28,6,22,8,44,17]，arr2 = [22,28,8,6]）为例：
  名次表 {22:0, 28:1, 8:2, 6:3}
  排序键：28→(0,1)、6→(0,3)、22→(0,0)、8→(0,2)、44→(1,44)、17→(1,17)
  按二元组升序 → 22、28、8、6、17、44 ✔

复杂度分析：
  时间 O(m log m + n)：建名次表 O(n)，排序 O(m log m)
  空间 O(n + log m)：名次表存 arr2 的全部元素，排序另有递归栈

解法二：计数排序【最优解】

为什么慢：arr1 的值域只有 [0, 1000]，比较排序的 O(m log m) 完全没必要 —— 名次
信息可以直接由数组下标给出，不需要两两比较。

换个角度：先用频次数组统计 arr1 里每个值出现几次，再按 arr2 的顺序把每个值「整批
倒出」并清零；剩下的值按频次数组下标从 0 到 upper 扫一遍补在末尾，天然就是升序。
两段合起来正好满足题面的全部要求。

关键点：
  · 频次数组长度取 arr1 最大值 + 1，而不是固定 1001，省掉无用的高位
  · 倒出后必须把频次清零，否则同一个值会在第二段再输出一遍
  · 第二段按下标升序扫描就完成了「未出现过的元素升序放末尾」，不需要再排序
  · 依赖输入非负且值域可控（本题 0 <= arr1[i] <= 1000），否则下标不能当键

以示例 1（arr1 = [2,3,1,3,2,4,6,7,9,2,19]，arr2 = [2,1,4,3,9,6]）为例：
  频次：2→3、1→1、3→2、4→1、6→1、7→1、9→1、19→1
  按 arr2 倒出：2,2,2 | 1 | 4 | 3,3 | 9 | 6
  按下标补剩余：7、19
  结果 [2,2,2,1,4,3,3,9,6,7,19] ✔

复杂度分析：
  时间 O(m + n + upper)：统计 O(m)，倒出 O(m + n)，扫值域 O(upper)，
    upper <= 1000 是常数上界，故整体是线性的
  空间 O(upper)：只有频次数组，答案数组不计入（可覆盖写回 arr1）
*/

// 解法一：自定义排序
func relativeSortArray(arr1 []int, arr2 []int) []int {
	rank := make(map[int]int, len(arr2))
	for i, x := range arr2 {
		rank[x] = i
	}

	// 排序键：arr2 里的元素用 [0, 名次]，其余用 [1, 自身值]
	key := func(x int) [2]int {
		if r, ok := rank[x]; ok {
			return [2]int{0, r}
		}
		return [2]int{1, x}
	}

	sort.Slice(arr1, func(i, j int) bool {
		a, b := key(arr1[i]), key(arr1[j])
		if a[0] != b[0] {
			return a[0] < b[0]
		}
		return a[1] < b[1]
	})
	return arr1
}

// 解法二：计数排序
func relativeSortArrayByCount(arr1 []int, arr2 []int) []int {
	upper := 0
	for _, x := range arr1 {
		if x > upper {
			upper = x
		}
	}
	frequency := make([]int, upper+1)
	for _, x := range arr1 {
		frequency[x]++
	}

	ans := make([]int, 0, len(arr1))
	// 按 arr2 的顺序把对应值整批倒出，倒完清零避免重复输出
	for _, x := range arr2 {
		for ; frequency[x] > 0; frequency[x]-- {
			ans = append(ans, x)
		}
	}
	// 剩下的值没有额外约束，按下标升序补在末尾即可
	for x := 0; x <= upper; x++ {
		for ; frequency[x] > 0; frequency[x]-- {
			ans = append(ans, x)
		}
	}
	return ans
}

// ===== 测试辅助 =====
func check(arr1, arr2, want []int) {
	got := relativeSortArray(append([]int(nil), arr1...), arr2)
	if !equal(got, want) {
		panic(fmt.Sprintf("sort(%v, %v) = %v, want %v", arr1, arr2, got, want))
	}

	got = relativeSortArrayByCount(arr1, arr2)
	if !equal(got, want) {
		panic(fmt.Sprintf("count(%v, %v) = %v, want %v", arr1, arr2, got, want))
	}
}

func equal(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func main() {
	// 示例 1
	check([]int{2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19}, []int{2, 1, 4, 3, 9, 6},
		[]int{2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19})

	// 示例 2：arr2 的顺序与数值大小无关
	check([]int{28, 6, 22, 8, 44, 17}, []int{22, 28, 8, 6},
		[]int{22, 28, 8, 6, 17, 44})

	// 边界：arr2 只有一个元素，其余按升序补在后面
	check([]int{5, 3, 1}, []int{3}, []int{3, 1, 5})

	// 边界：arr1 全是同一个值
	check([]int{7, 7, 7}, []int{7}, []int{7, 7, 7})

	// 边界：值域下限 0 也要能进频次数组
	check([]int{0, 2, 0}, []int{0}, []int{0, 0, 2})

	// 反直觉：arr2 是降序，结果必须跟着 arr2 逆序走
	check([]int{1, 2}, []int{2, 1}, []int{2, 1})

	// 反直觉：未出现的元素里含 0，仍要排在升序首位
	check([]int{1, 0}, []int{1}, []int{1, 0})

	// 边界：值域上限 1000
	check([]int{1000, 0}, []int{1000}, []int{1000, 0})

	fmt.Println("all tests passed")
}
