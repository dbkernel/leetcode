/*
============================================================================
计数排序（Counting Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n + K)；空间 O(n + K)；稳定性：稳定
        K 为值域宽度 max - min + 1，不是元素个数
============================================================================

用途：
  值域不大（K 与 n 同量级）时能突破比较排序的 O(n log n) 下界；
  也是基数排序每一位的稳定子过程。面试常问「为什么不用比较」。

核心思想：
  数出每个值出现几次，再做前缀和得到「该值应该放在哪一段」，
  最后按原数组顺序把元素填进输出数组：值本身决定位置，不用比较。

关键点（不变量）：
  · 统一减去 min 做偏移，负数也能当数组下标用；
  · 前缀和之后 cnt[i] 表示「值 <= i + min 的元素个数」，也是右边界；
  · 回填必须倒序遍历原数组：这样同值元素按原次序落在连续区间的
    后半段，前一个同值元素再填到它前面，稳定性由此保证；
  · K 是值域宽度：值域极宽时计数数组比数据还大，会慢且费内存。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：min = 1，K = 5。
  1. 计数：值 1、2、3、4、5 依次出现 2、0、1、1、1 次，
     cnt = [2, 0, 1, 1, 1]；
  2. 前缀和：cnt = [2, 2, 3, 4, 5]；
  3. 倒序回填：a[4] = 5 放到 out[4]；a[3] = 1 放到 out[1]；
     a[2] = 4 放到 out[3]；a[1] = 1 放到 out[0]；a[0] = 3 放到
     out[2]；
  4. out = [1, 1, 3, 4, 5]，搬回 a。

复杂度分析：
  时间 O(n + K)：计数一趟、前缀和 K 趟、回填一趟，与比较无关。
    K 远大于 n 时（比如 n = 100、值域跨 10^9），光清零计数数组就要
    10^9 步，比 O(n log n) 的比较排序慢得多，也更费内存。
  空间 O(n + K)：输出数组 n、计数数组 K。
  稳定性：稳定。倒序回填让同值元素保持原有的相对次序。
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

func countingSort(a []int) {
	n := len(a)
	if n < 2 {
		return
	}

	lo, hi := a[0], a[0]
	for _, v := range a { // 先求值域，负数靠偏移 v - lo 变成合法下标
		if v < lo {
			lo = v
		}
		if v > hi {
			hi = v
		}
	}
	k := hi - lo + 1 // 值域宽度
	cnt := make([]int, k)
	for _, v := range a {
		cnt[v-lo]++
	}

	// 前缀和：cnt[i] 变成「值 <= i + lo 的元素个数」，即该值右边界
	for i := 1; i < k; i++ {
		cnt[i] += cnt[i-1]
	}

	out := make([]int, n)
	// 倒序回填：同值元素里靠后的先落位，靠前的再填到它前面，
	// 于是同值元素保持了原数组中的先后次序（稳定）
	for i := n - 1; i >= 0; i-- {
		v := a[i]
		cnt[v-lo]--
		out[cnt[v-lo]] = v
	}
	for i := range a {
		a[i] = out[i]
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
	countingSort(got)
	check(equalVec(got, want), "countingSort")
}

func main() {
	// 1. 正常用例：与上面「手推一遍」的数组一致
	{
		a := []int{3, 1, 4, 1, 5}
		countingSort(a)
		fmt.Print("sorted: ")
		printVec(a)
		fmt.Println()
		check(equalVec(a, []int{1, 1, 3, 4, 5}), "主用例")
	}

	// 2. 含负数：偏移写错（漏减 min）就会越界
	{
		a := []int{-7, 3, -1, 0, -7, 2}
		countingSort(a)
		fmt.Print("with negatives: ")
		printVec(a)
		fmt.Println()
		check(equalVec(a, []int{-7, -7, -1, 0, 2, 3}), "负数用例")
		checkSorted([]int{-1000000, -999999, -1000001})
	}

	// 3. 边界：空数组、单元素、全相同
	{
		var empty []int
		countingSort(empty)
		check(len(empty) == 0, "空数组")

		one := []int{-42}
		countingSort(one)
		check(equalVec(one, []int{-42}), "单元素")

		same := make([]int, 300)
		for i := range same {
			same[i] = 5
		}
		checkSorted(same)
	}

	// 4. 值域极窄：只有两个取值，计数数组长度为 2
	{
		a := make([]int, 400)
		for i := range a {
			a[i] = i % 2
		}
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

	fmt.Println("all tests passed")
}
