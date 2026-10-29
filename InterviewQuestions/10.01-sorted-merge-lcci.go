/*
============================================================================
面试题 10.01. 合并排序的数组（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sorted-merge-lcci/
分类：数组 · 双指针（归并）

给定两个排序后的数组 A 和 B，其中 A 的末端有足够的缓冲空间容纳 B。 编写一个方法，
将 B 合并入 A 并排序。

初始化 A 和 B 的元素数量分别为 m 和 n。

示例：
输入：
A = [1,2,3,0,0,0], m = 3
B = [2,5,6],       n = 3

输出： [1,2,2,3,5,6]

说明：
A.length == n + m
*/
package main

import (
	"fmt"
	"reflect"
	"sort"
)

/*
解法一：直接合并后排序【非最优】

官方方法一。把 B 直接搬到 A 的尾部，然后对整条 A 排序。

关键点：
  · 写法最短，但没有利用「两个数组各自有序」这一条件
  · 排序 A 的代价是 O((m+n)log(m+n))，比线性归并慢

复杂度分析：
  时间 O((m+n)log(m+n))，空间 O(log(m+n))（排序栈开销）

解法二：双指针（借助临时数组）【非最优】

官方方法二。把 A、B 看成两个队列，每次从两队的队首取较小者放进结果数组。
用一个长度为 m+n 的临时数组接住结果，最后整体拷回 A。

关键点：
  · 利用了「已排序」的性质，时间降为线性
  · 但需要一个额外的长度为 m+n 的数组，空间 O(m+n)
  · 指针 pa 走到 m 或 pb 走到 n 之一时，另一侧要整体搬完（别漏了尾巴）

复杂度分析：
  时间 O(m+n)，空间 O(m+n)

解法三：逆向双指针（原地）【最优解】

官方方法三。既然 A 的后半段本来就是空的，那就不该「从头往尾填」而应该「从尾往头填」：
比较 A 与 B 的最后一个元素，把较大的那个放到 A 的最后一位，然后指针前移。
这样写入位置永远在还没读取的位置之后，不会覆盖 A 中尚未处理的数据。

为什么不会覆盖？设 pa、pb 为当前待比较的下标，tail 为待写入下标。此时
A 中已搬到后半部分的有 m-pa-1 个、B 中已搬的有 n-pb-1 个，而 pa 之后还剩
m+n-pa-1 个位置。由 m+n-pa-1 >= (m-pa-1)+(n-pb-1) 等价于 pb >= -1 恒成立，
所以位置永远够用 —— 这就是「从后往前」能原地完成的原因。

关键点：
  · 从后往前比较、从后往前写入，是大数组归并的标准技巧
  · pa < 0 时只能搬 B，pb < 0 时只能搬 A
  · 若 B 先被搬完，A 前面剩下的元素本来就在正确位置上，什么都不用做
  · 比较用 A[pa] > B[pb]，相等时先取 B（或 A）都可以，结果都正确

以示例（A = [1,2,3,0,0,0], m = 3；B = [2,5,6], n = 3）为例：
  pa = 2 (A[2]=3)、pb = 2 (B[2]=6)、tail = 5
  3 < 6 → A[5] = 6，pb = 1，tail = 4
  3 < 5 → A[4] = 5，pb = 0，tail = 3
  3 > 2 → A[3] = 3，pa = 1，tail = 2
  2 与 2 相等 → 取 B：A[2] = 2，pb = -1，tail = 1
  pb < 0 → A[1] = A[1] = 2，pa = 0，tail = 0
  pb < 0 → A[0] = A[0] = 1，pa = -1，结束
  结果 [1,2,2,3,5,6]

复杂度分析：
  时间 O(m+n)：每个元素恰好搬一次
  空间 O(1)：原地完成，不用临时数组
*/

// 解法一：直接合并后排序
func merge1(A []int, m int, B []int, n int) {
	copy(A[m:], B)
	sort.Ints(A)
}

// 解法二：双指针 + 临时数组
func merge2(A []int, m int, B []int, n int) {
	sorted := make([]int, 0, m+n)
	pa, pb := 0, 0
	for pa < m || pb < n {
		if pa == m {
			sorted = append(sorted, B[pb])
			pb++
		} else if pb == n {
			sorted = append(sorted, A[pa])
			pa++
		} else if A[pa] < B[pb] {
			sorted = append(sorted, A[pa])
			pa++
		} else {
			sorted = append(sorted, B[pb])
			pb++
		}
	}
	copy(A, sorted)
}

// 解法三：逆向双指针（原地，最优解）
func merge(A []int, m int, B []int, n int) {
	pa, pb := m-1, n-1
	tail := m + n - 1
	for pa >= 0 || pb >= 0 {
		if pa == -1 {
			A[tail] = B[pb]
			pb--
		} else if pb == -1 {
			A[tail] = A[pa]
			pa--
		} else if A[pa] > B[pb] {
			A[tail] = A[pa]
			pa--
		} else {
			A[tail] = B[pb]
			pb--
		}
		tail--
	}
}

// 由「有效元素」和 B 构造 A（尾部补 0）
func build(values []int, extra int) []int {
	A := make([]int, len(values), len(values)+extra)
	copy(A, values)
	for i := 0; i < extra; i++ {
		A = append(A, 0)
	}
	return A
}

func check(aValues []int, bValues []int, want []int) {
	m := len(aValues)
	n := len(bValues)

	cases := []struct {
		name string
		fn   func([]int, int, []int, int)
	}{
		{"merge1", merge1},
		{"merge2", merge2},
		{"merge", merge},
	}
	for _, c := range cases {
		a := build(aValues, n)
		b := append([]int(nil), bValues...)
		c.fn(a, m, b, n)
		if !reflect.DeepEqual(a, want) {
			panic(fmt.Sprintf("%s(%v, %d, %v, %d) = %v, want %v", c.name, aValues, m, bValues, n, a, want))
		}
	}
}

func main() {
	// 示例
	check([]int{1, 2, 3}, []int{2, 5, 6}, []int{1, 2, 2, 3, 5, 6})

	// 边界：A 为空（m = 0），B 整体搬入
	check([]int{}, []int{1, 2, 3}, []int{1, 2, 3})

	// 边界：B 为空（n = 0），A 不变
	check([]int{1, 2, 3}, []int{}, []int{1, 2, 3})

	// 边界：两者都为空
	check([]int{}, []int{}, []int{})

	// 边界：B 全部小于 A
	check([]int{4, 5, 6}, []int{1, 2, 3}, []int{1, 2, 3, 4, 5, 6})

	// 边界：B 全部大于 A
	check([]int{1, 2, 3}, []int{4, 5, 6}, []int{1, 2, 3, 4, 5, 6})

	// 边界：大量重复元素
	check([]int{1, 1, 1}, []int{1, 1}, []int{1, 1, 1, 1, 1})

	// 边界：交错合并
	check([]int{1, 3, 5, 7}, []int{2, 4, 6, 8}, []int{1, 2, 3, 4, 5, 6, 7, 8})

	// 边界：含负数
	check([]int{-5, -1, 3}, []int{-4, 0, 9}, []int{-5, -4, -1, 0, 3, 9})

	fmt.Println("all tests passed")
}
