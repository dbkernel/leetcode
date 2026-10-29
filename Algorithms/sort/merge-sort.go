/*
============================================================================
归并排序（Merge Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n log n)；空间 O(n)；稳定性：稳定
============================================================================

用途：
  给整数数组做稳定排序，也是「分治 + 双指针合并」的入门题。
  面试里常以「排序链表」「求逆序对」的形式出现。

核心思想：
  把数组一分为二，两半各自有序后用两个指针从头各取较小者写进
  辅助数组，一趟线性扫描完成合并；长度 1 的区间天然有序。

关键点（不变量）：
  · 合并时相等取左半（用 <= 判断），稳定性就靠这一处；
  · 辅助数组只分配一次，递归各层复用同一块；
  · 子区间用左闭右开 [lo, hi)，hi - lo 即长度，空区间直接返回；
  · 自底向上版让 width 从 1 翻倍，按层合并，不需要递归栈。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：
  1. 拆成 [3, 1] 和 [4, 1, 5]；
  2. [3, 1] 拆成 [3]、[1]，合并成 [1, 3]；
  3. [4, 1, 5] 拆成 [4]、[1, 5]，[1, 5] 已有序；
  4. 合并 [4] 与 [1, 5]：先取 1，再取 4，再取 5，得 [1, 4, 5]；
  5. 合并 [1, 3] 与 [1, 4, 5]：两个 1 相等，先取左半的 1
     （稳定性），再取 3、4、5，得 [1, 1, 3, 4, 5]。

复杂度分析：
  时间 O(n log n)：递归树高 log n，每层合并共 O(n)，好坏平均一致。
  空间 O(n)：辅助数组；递归栈 O(log n) 被 O(n) 吸收。
  稳定性：稳定。相等时先取左半，而左半元素原本就在前面；
    自底向上版同样只在合并处比较，所以也稳定。
*/

package main

import (
	"fmt"
	"sort" // 仅用于对拍校验：实现部分没有使用内置排序
)

// ---------------------------------------------------------------------------
// 固定种子的 xorshift32：四个语言跑同一条序列，对拍结果可复现。
// ---------------------------------------------------------------------------
var rngState uint32 = 20260101

func resetRng() { rngState = 20260101 }

func nextU32() uint32 {
	s := rngState
	s ^= s << 13
	s ^= s >> 17
	s ^= s << 5
	rngState = s
	return s
}

// 返回闭区间 [lo, hi] 内的整数。
func nextInt(lo, hi int) int {
	span := uint32(hi - lo + 1)
	return lo + int(nextU32()%span)
}

// 合并 a[lo, mid) 与 a[mid, hi)：先写进 buf，再整体搬回 a。
// 只有 a[i] <= a[j] 时才取左半，相等元素的先后次序因此不变。
func mergeRange(a, buf []int, lo, mid, hi int) {
	i, j, k := lo, mid, lo
	for i < mid && j < hi {
		if a[i] <= a[j] {
			buf[k] = a[i]
			i++
		} else {
			buf[k] = a[j]
			j++
		}
		k++
	}
	for i < mid {
		buf[k] = a[i]
		i++
		k++
	}
	for j < hi {
		buf[k] = a[j]
		j++
		k++
	}
	for t := lo; t < hi; t++ {
		a[t] = buf[t]
	}
}

// 自顶向下：先排两半再合并；buf 由调用方分配，各层共用。
func msort(a, buf []int, lo, hi int) {
	if hi-lo < 2 {
		return // 空或单元素即有序，也是递归出口
	}
	mid := lo + (hi-lo)/2
	msort(a, buf, lo, mid)
	msort(a, buf, mid, hi)
	mergeRange(a, buf, lo, mid, hi)
}

func mergeSort(a []int) {
	n := len(a)
	if n < 2 {
		return
	}
	buf := make([]int, n) // 只分配这一次，各层复用
	msort(a, buf, 0, n)
}

// 自底向上：width = 1, 2, 4 ...，每轮合并相邻两段长度 width 的区间。
// 好处是没有递归栈；链表排序用同一套按层合并的思路，只改 next 指针。
func mergeSortBottomUp(a []int) {
	n := len(a)
	if n < 2 {
		return
	}
	buf := make([]int, n)
	for width := 1; width < n; width *= 2 {
		for lo := 0; lo < n; lo += 2 * width {
			mid := lo + width
			if mid >= n {
				break // 只剩一段，本轮无需合并
			}
			hi := lo + 2*width
			if hi > n {
				hi = n
			}
			mergeRange(a, buf, lo, mid, hi)
		}
	}
}

// ---------------------------------------------------------------------------
// 自测
// ---------------------------------------------------------------------------
func printVec(a []int) {
	fmt.Print("[")
	for i, v := range a {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(v)
	}
	fmt.Print("]")
}

func check(cond bool, msg string) {
	if !cond {
		panic("断言失败: " + msg)
	}
}

func equalVec(a, b []int) bool {
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

// 同一组输入分别喂给两个版本，都与内置排序的结果比对。
func checkBoth(src []int) {
	want := append([]int(nil), src...)
	sort.Ints(want) // 仅用于校验：实现部分没有使用内置排序

	top := append([]int(nil), src...)
	mergeSort(top)
	check(equalVec(top, want), "mergeSort")

	bottom := append([]int(nil), src...)
	mergeSortBottomUp(bottom)
	check(equalVec(bottom, want), "mergeSortBottomUp")
}

func main() {
	// 1. 正常用例：与上面「手推一遍」的数组一致
	{
		a := []int{3, 1, 4, 1, 5}
		mergeSort(a)
		fmt.Print("sorted: ")
		printVec(a)
		fmt.Println()
		check(equalVec(a, []int{1, 1, 3, 4, 5}), "主用例")
	}

	// 2. 边界：空数组、单元素，两个版本都不能越界
	{
		var empty []int
		mergeSort(empty)
		mergeSortBottomUp(empty)
		check(len(empty) == 0, "空数组")

		one := []int{7}
		mergeSort(one)
		mergeSortBottomUp(one)
		check(equalVec(one, []int{7}), "单元素")
	}

	// 3. 逆序：每层都要真的搬运，漏合并的写法会在这里挂
	{
		rev := make([]int, 0, 1000)
		for i := 1000; i >= 1; i-- {
			rev = append(rev, i)
		}
		checkBoth(rev)
	}

	// 4. 大量重复：全相同 + 只有三种取值
	{
		same := make([]int, 500)
		for i := range same {
			same[i] = 42
		}
		checkBoth(same)

		few := make([]int, 300)
		for i := range few {
			few[i] = i % 3
		}
		checkBoth(few)
	}

	// 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
	{
		resetRng()
		lens := []int{0, 1, 2, 3, 10, 100, 1000}
		for _, n := range lens {
			for t := 0; t < 20; t++ {
				a := make([]int, n)
				for i := range a {
					a[i] = nextInt(-50, 50)
				}
				checkBoth(a)
			}
		}
	}

	fmt.Println("all tests passed")
}
