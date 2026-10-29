/*
============================================================================
桶排序（Bucket Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 平均 O(n)、最坏 O(n^2)；空间 O(n + k)；稳定性：稳定
        k = ceil(sqrt(n)) 为桶数
============================================================================

用途：
  数据在值域上大致均匀时，用「先分桶、桶内小规模排序」换掉全局比较；
  面试常问「k 怎么取」「为什么平均是线性」。

核心思想：
  按值域把 [min, max] 均分成 k 个区间，每个元素按值落进对应桶；
  桶内元素只可能落在本桶的值域里，所以桶内排好后按桶序拼接即可。

关键点（不变量）：
  · 桶下标 idx = (v - min) * k / (max - min + 1)，用 64 位算；
    分母是值域宽度，因此 v = max 时 idx 最大也只会到 k - 1；
  · 同值元素必然进同一个桶，桶内插入排序用严格大于（>）比较，
    相等元素不交换，稳定性的另一半靠这里；
  · 桶按值域区间从左到右拼接，桶间不需要再比较；
  · 全相同或值域极窄时所有元素挤进少数桶，会退化。

手推一遍：
  以 [7, 1, 5, 3] 为例：n = 4，k = 2，值域 [1, 7]，宽度 7。
  1. idx = (v - 1) * 2 / 7：7→1、1→0、5→1、3→0；
  2. 桶 0 = [1, 3]，桶 1 = [5, 7]（桶内插入排序）；
  3. 按桶序拼接得 [1, 3, 5, 7]。

复杂度分析：
  时间：分桶 O(n) + 桶内插入排序 sum(mi^2) + 拼接 O(n)。
    平均 O(n)：均匀输入下每个桶的期望规模 m = n / k；当 k 取到
      n 的量级时 m ≈ 1，桶内 O(1)，总计 O(n)，这是「平均线性」的
      准确前提。本例按题面取 k = ceil(sqrt(n))，m ≈ sqrt(n)，
      桶内约 O(n)、k 个桶合计 O(n sqrt(n))，常数很小、实测很快。
    最坏 O(n^2)：数据全挤进一个桶，退化成一次插入排序。
  空间 O(n + k)：桶里存全部元素共 n 个，桶对象 k 个（k <= n）。
  稳定性：稳定。同值元素同桶 + 桶内插入排序稳定 + 桶序即值域序。
*/

package main

import (
	"fmt"
	"sort" // 仅用于对拍校验：实现部分没有使用内置排序
)

// 固定种子的 xorshift32：四个语言跑同一条序列，对拍结果可复现。
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

// 桶内排序用的插入排序。用严格大于比较：相等元素不交换 => 稳定。
func insertionSort(b []int) {
	m := len(b)
	for i := 1; i < m; i++ {
		key := b[i]
		j := i - 1
		for j >= 0 && b[j] > key { // 注意是 >，不是 >=
			b[j+1] = b[j]
			j--
		}
		b[j+1] = key
	}
}

func bucketSort(a []int) {
	n := len(a)
	if n < 2 {
		return
	}

	lo, hi := a[0], a[0]
	for _, v := range a {
		if v < lo {
			lo = v
		}
		if v > hi {
			hi = v
		}
	}
	// 值域宽度用 64 位：hi - lo + 1 在 int32 上可能溢出
	span := int64(hi) - int64(lo) + 1

	k := 1
	for int64(k)*int64(k) < int64(n) {
		k++ // ceil(sqrt(n))
	}

	buckets := make([][]int, k)
	for _, v := range a {
		// 乘以 k 后再除，先乘后除保证桶边界均匀；大值域下必须 64 位
		idx := (int64(v) - int64(lo)) * int64(k) / span
		buckets[idx] = append(buckets[idx], v)
	}

	w := 0
	for i := 0; i < k; i++ {
		insertionSort(buckets[i])
		// 桶 i 里的值都落在第 i 段值域，按桶序写回即全局有序
		for _, v := range buckets[i] {
			a[w] = v
			w++
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

func checkSorted(src []int) {
	want := append([]int(nil), src...)
	sort.Ints(want) // 仅用于校验：实现部分没有使用内置排序
	got := append([]int(nil), src...)
	bucketSort(got)
	check(equalVec(got, want), "bucketSort")
}

func main() {
	// 1. 正常用例：与上面「手推一遍」的数组一致
	{
		a := []int{7, 1, 5, 3}
		bucketSort(a)
		fmt.Print("sorted: ")
		printVec(a)
		fmt.Println()
		check(equalVec(a, []int{1, 3, 5, 7}), "主用例")
	}

	// 2. 边界：空数组、单元素
	{
		var empty []int
		bucketSort(empty)
		check(len(empty) == 0, "空数组")

		one := []int{7}
		bucketSort(one)
		check(equalVec(one, []int{7}), "单元素")
	}

	// 3. 全相同：所有元素挤进一个桶，退化成一个桶的插入排序
	{
		same := make([]int, 500)
		for i := range same {
			same[i] = 42
		}
		checkSorted(same)
	}

	// 4. 已有序 / 逆序 / 含负数 / 值域极窄
	{
		asc := make([]int, 500)
		desc := make([]int, 500)
		for i := 0; i < 500; i++ {
			asc[i] = i
			desc[i] = 500 - i
		}
		checkSorted(asc)
		checkSorted(desc)
		checkSorted([]int{-7, -100, 0, 5, -3})
		checkSorted([]int{-1, 0, 1, -1, 0, 1, 0})
	}

	// 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数
	{
		resetRng()
		for _, n := range []int{0, 1, 2, 3, 10, 100, 1000} {
			for t := 0; t < 20; t++ {
				a := make([]int, n)
				for i := range a {
					a[i] = nextInt(-50, 50)
				}
				checkSorted(a)
			}
		}
	}

	// 6. 值域偏斜：绝大多数元素集中在 [0, 5]，少数跑到 10^6
	{
		resetRng()
		for t := 0; t < 20; t++ {
			a := make([]int, 0, 302)
			for i := 0; i < 300; i++ {
				a = append(a, nextInt(0, 5))
			}
			a = append(a, 1000000, -1000000)
			checkSorted(a)
		}
	}

	fmt.Println("all tests passed")
}
