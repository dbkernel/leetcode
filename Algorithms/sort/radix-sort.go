/*
============================================================================
基数排序（Radix Sort，LSD 十进制）
分类：手写数据结构 · 排序算法
复杂度：时间 O(d(n + 10))；空间 O(n + 10)；稳定性：稳定
        d 为最大值的十进制位数
============================================================================

用途：
  值域很大但位数不多（32 位整数最多 10 位）时，用「按位分配」
  代替「按值比较」；面试常追问「为什么要求稳定」与「负数怎么办」。

核心思想：
  从最低位到最高位，每一位用一次稳定的计数排序把数组按该位排好；
  低位排好的相对次序会被高位排序保留，d 趟之后整体有序。

关键点（不变量）：
  · 子过程必须稳定：同一位数字相同的元素要保持上一趟的相对次序；
  · 负数单独处理：取绝对值排好再反转负数段，不能和非负数混排；
  · exp 每次乘 10，到最高位就停，多乘一次只会空跑还可能溢出；
  · 十进制每趟只有 10 个桶，所以每趟 O(n + 10)，不是 O(n + K)。

手推一遍：
  以 [13, 21, 4] 为例：
  1. 个位：13→3、21→1、4→4，按个位稳定排序得 [21, 13, 4]；
  2. 十位：21→2、13→1、4→0，按十位稳定排序得 [4, 13, 21]；
  3. 十位已是最高位，结束。
  负数同理：[-5, 21, -13] 取绝对值 [5, 13] 排好再反转成
  [-13, -5]，拼上 [21] 得 [-13, -5, 21]。

复杂度分析：
  时间 O(d(n + 10))：共 d 趟，每趟计数 + 前缀和 + 回填都是 O(n + 10)。
    d = 10 时相当于 10 倍常数；数据量小时不一定比快排快。
  空间 O(n + 10)：每趟的输出缓冲 n，计数桶 10 个。
    负数分段还额外占 O(n) 的临时数组，这是「按符号分段」的代价。
  稳定性：稳定，前提是每趟的计数排序都稳定（倒序回填保证）。
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

// 一趟：按 exp 位（1, 10, 100 ...）对非负数组做稳定计数排序。
func countingByDigit(a, buf []int64, exp int64) {
	var cnt [10]int
	n := len(a)
	for _, v := range a {
		cnt[(v/exp)%10]++
	}
	for i := 1; i < 10; i++ {
		cnt[i] += cnt[i-1]
	}
	// 倒序回填，同一位数字的元素保持原次序 => 这一趟稳定
	for i := n - 1; i >= 0; i-- {
		d := (a[i] / exp) % 10
		cnt[d]--
		buf[cnt[d]] = a[i]
	}
	for i := 0; i < n; i++ {
		a[i] = buf[i]
	}
}

// radixLsd：从个位到最高位逐位稳定排序。要求元素全为非负。
func radixLsd(a []int64) {
	n := len(a)
	if n < 2 {
		return
	}
	maxv := a[0]
	for _, v := range a {
		if v > maxv {
			maxv = v
		}
	}
	buf := make([]int64, n)
	exp := int64(1)
	for exp <= maxv {
		countingByDigit(a, buf, exp)
		// maxv / 10 之后再乘 10 就越过最高位了，提前收工避免溢出
		if exp > maxv/10 {
			break
		}
		exp *= 10
	}
}

func radixSort(a []int) {
	n := len(a)
	if n < 2 {
		return
	}

	// 负数单独处理：按绝对值排好，再把负数段反转（绝对值升序 =>
	// 数值降序）。代价是多一趟分组和 O(n) 临时空间，好处是不必
	// 整体偏移到非负，min 很负、max 很大时也不会因偏移而溢出。
	negMag := make([]int64, 0, n) // 负数的绝对值
	pos := make([]int64, 0, n)    // 非负数
	for _, v := range a {
		if v < 0 {
			negMag = append(negMag, -int64(v)) // 先转 64 位，INT_MIN 也不溢出
		} else {
			pos = append(pos, int64(v))
		}
	}
	radixLsd(pos)
	radixLsd(negMag)

	k := 0
	for i := len(negMag) - 1; i >= 0; i-- {
		a[k] = int(-negMag[i]) // 绝对值大的负数更小
		k++
	}
	for _, v := range pos {
		a[k] = int(v)
		k++
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
	radixSort(got)
	check(equalVec(got, want), "radixSort")
}

func main() {
	// 1. 正常用例：与上面「手推一遍」的数组一致
	{
		a := []int{13, 21, 4}
		radixSort(a)
		fmt.Print("sorted: ")
		printVec(a)
		fmt.Println()
		check(equalVec(a, []int{4, 13, 21}), "主用例")
	}

	// 2. 含负数、含 0：负数段反转与 0 的归属都在这里验
	{
		a := []int{-5, 21, -13, 0, 0, -5}
		radixSort(a)
		fmt.Print("with negatives: ")
		printVec(a)
		fmt.Println()
		check(equalVec(a, []int{-13, -5, -5, 0, 0, 21}), "负数用例")
	}

	// 3. 边界：空数组、单元素、全为 0
	{
		var empty []int
		radixSort(empty)
		check(len(empty) == 0, "空数组")

		one := []int{-9}
		radixSort(one)
		check(equalVec(one, []int{-9}), "单元素")

		zeros := make([]int, 200)
		checkSorted(zeros)
	}

	// 4. 大数值：10^9 级别，验证位数循环与 int 边界
	{
		a := []int{1000000000, -1000000000, 999999999, 1, -1, 0, 100000000}
		checkSorted(a)
	}

	// 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
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

	// 6. 大值域随机对拍：[-10^9, 10^9]，含负数
	{
		resetRng()
		for t := 0; t < 20; t++ {
			a := make([]int, 500)
			for i := range a {
				a[i] = nextInt(-1000000000, 1000000000)
			}
			checkSorted(a)
		}
	}

	fmt.Println("all tests passed")
}
