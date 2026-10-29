/*
============================================================================
堆排序（Heap Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n log n)；空间 O(1)；稳定性：不稳定
============================================================================

用途：
  原地、最坏也能保证 O(n log n) 的排序；面试里常和「第 k 大」
  「数据流中位数」一起出现，考的是 heapify 和下沉写不写对。

核心思想：
  数组天然是一棵完全二叉树（下标 i 的孩子是 2i+1、2i+2）。
  先把数组调成大根堆，堆顶就是最大值；把它换到末尾，堆的规模减一，
  再让新堆顶下沉复位，重复 n-1 次就得到升序数组。

关键点（不变量）：
  · 下沉时只在「比最大的孩子还小」时才交换，否则立刻停；
  · 建堆从最后一个非叶结点 n/2 - 1 往前走，叶子不必下沉；
  · 交换后堆的有效长度变成 end，下沉的边界必须用 end 而不是 n；
  · 大根堆 + 堆顶换到末尾 = 升序；想降序就建小根堆。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例（n = 5）：
  1. i = 1 下沉：1 的孩子是 a[3]=1、a[4]=5，与 5 交换，
     得 [3, 5, 4, 1, 1]；
  2. i = 0 下沉：3 的孩子是 5、4，与 5 交换得 [5, 3, 4, 1, 1]，
     3 落到下标 1 后孩子都是 1，停下，堆建成；
  3. 堆顶 5 换到末尾，前 4 个下沉复位得 [4, 3, 1, 1 | 5]；
  4. 再换 4、3，最后两个 1 交换，整体 [1, 1, 3, 4, 5]。

复杂度分析：
  时间 O(n log n)：建堆 O(n)，之后 n-1 次下沉，每次最多走树高
    O(log n)，合计 O(n log n)，与输入是否有序无关。
  空间 O(1)：原地交换，下沉写成循环，不占递归栈。
  稳定性：不稳定。堆顶换到末尾、孩子与父结点交换都是远距离搬运，
    相等元素的前后次序无法保持。
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

// 让 a[i] 在「有效长度 n」的堆里下沉到合适位置。
// 循环版：不递归，额外空间才是 O(1)。
func siftDown(a []int, i, n int) {
	for {
		l := 2*i + 1
		if l >= n {
			break // 没有孩子，已经落到叶子
		}
		big := l
		if r := l + 1; r < n && a[r] > a[l] {
			big = r // 挑较大的孩子
		}
		if a[i] >= a[big] {
			break // 父结点已经不小于孩子
		}
		a[i], a[big] = a[big], a[i]
		i = big // 继续检查被换下去的元素
	}
}

// 自底向上建大根堆：从最后一个非叶结点往前扫。
// 为什么是 O(n)：高度 h 的结点最多 n/2^(h+1) 个，下沉代价 h，
// 总和 sum(h / 2^h) 收敛到常数，故为 O(n) 而不是 O(n log n)。
func buildMaxHeap(a []int) {
	n := len(a)
	for i := n/2 - 1; i >= 0; i-- {
		siftDown(a, i, n)
	}
}

func heapSort(a []int) {
	n := len(a)
	if n < 2 {
		return
	}
	buildMaxHeap(a)
	for end := n - 1; end > 0; end-- {
		a[0], a[end] = a[end], a[0] // 当前最大值放到它最终的位置
		siftDown(a, 0, end)         // 边界是 end：末尾已归位，不再参与
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

// 逐结点检查大根堆性质：每个父结点都不小于它的孩子。
func isMaxHeap(a []int) bool {
	n := len(a)
	for i := 0; i < n; i++ {
		l := 2*i + 1
		if l < n && a[i] < a[l] {
			return false
		}
		if r := l + 1; r < n && a[i] < a[r] {
			return false
		}
	}
	return true
}

func checkSorted(src []int) {
	want := append([]int(nil), src...)
	sort.Ints(want) // 仅用于校验：实现部分没有使用内置排序
	got := append([]int(nil), src...)
	heapSort(got)
	check(equalVec(got, want), "heapSort")
}

func main() {
	// 1. 正常用例：与上面「手推一遍」的数组一致
	{
		a := []int{3, 1, 4, 1, 5}
		heapSort(a)
		fmt.Print("sorted: ")
		printVec(a)
		fmt.Println()
		check(equalVec(a, []int{1, 1, 3, 4, 5}), "主用例")
	}

	// 2. 边界：空数组、单元素
	{
		var empty []int
		heapSort(empty)
		check(len(empty) == 0, "空数组")

		one := []int{7}
		heapSort(one)
		check(equalVec(one, []int{7}), "单元素")
	}

	// 3. 建堆后数组必须满足大根堆性质（关键点写错时这条会挂）
	{
		resetRng()
		for t := 0; t < 100; t++ {
			n := 1 + int(nextU32()%200)
			a := make([]int, n)
			for i := range a {
				a[i] = nextInt(-100, 100)
			}
			buildMaxHeap(a)
			check(isMaxHeap(a), "建堆后的堆性质")
		}
		sorted := []int{1, 2, 3, 4, 5}
		buildMaxHeap(sorted) // 有序输入也要能建成大根堆
		check(isMaxHeap(sorted), "有序输入建堆")
	}

	// 4. 已有序 / 逆序 / 全相同
	{
		asc := make([]int, 500)
		desc := make([]int, 500)
		for i := 0; i < 500; i++ {
			asc[i] = i
			desc[i] = 500 - i
		}
		checkSorted(asc)
		checkSorted(desc)
		same := make([]int, 500)
		for i := range same {
			same[i] = 9
		}
		checkSorted(same)
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
