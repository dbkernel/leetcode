/*
============================================================================
插入排序（Insertion Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 最好 O(n)、平均 O(n^2)、最坏 O(n^2)；空间 O(1)；稳定性：稳定
============================================================================

用途：
  小数组或「基本有序」数据的首选排序。面试里常问「为什么它不靠交换」、
  「元素移动多少次」，它也常被当作快排小区间的收尾排序、希尔排序的内层。

核心思想：
  维护已排序前缀 a[0..i-1]，把 a[i] 取出来暂存为 key，然后在前缀里从右
  往左找 key 的落点：比 key 大的元素直接往后挪一格，空出来的位置最后放
  key。前缀逐步扩张，直到整个数组有序。

关键点（不变量）：
  · 进入第 i 轮时 a[0..i-1] 已升序；本轮结束后 a[0..i] 升序。
  · 先暂存 key、再整体后移：a[i] 被 a[i-1] 覆盖之前 key 已经存下来了，
    所以不会丢数据。每轮只做「一次比较 + 一次赋值」，比「比较 + 交换」
    少两次写内存，这是它比冒泡快的原因。
  · 内层条件是 a[j] > key（严格大于）：遇到相等就停下并插在它后面，
    相等元素的次序不变，所以稳定；写成 a[j] >= key 就会变成不稳定。
  · j 可能一路减到 -1，所以必须先判 j >= 0，再去访问 a[j]，顺序反了就
    越界。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：
  1. i = 1，key = 1：a[0] = 3 > 1 后移 → [3, 3, 4, 1, 5]，j 减到 -1，
     落点 0 → [1, 3, 4, 1, 5]。
  2. i = 2，key = 4：a[1] = 3 不大于 4，立即停，落点 2。
  3. i = 3，key = 1：4、3 依次后移 → [1, 3, 3, 4, 5]，j 减到 -1，
     落点 0 → [1, 1, 3, 4, 5]。
  4. i = 4，key = 5：a[3] = 4 不大于 5，立即停。结束。

复杂度分析：
  时间：最好 O(n)（已有序时每轮只比较一次就停），最坏 O(n^2)（逆序时
    每轮都要挪到最前面），平均 O(n^2)（随机数据平均移动 n^2/4 次）。
  空间 O(1)：只有 key、i、j 三个变量，移动都发生在原切片上。
  稳定性：稳定，见关键点第三条。
*/

package main

import (
	"fmt"
	"math/rand"
	"sort"
)

// 仅用于自测：统计比较次数，验证「已有序时每轮只比较一次」。
var cmpCount int

// 仅用于自测：统计元素后移次数，逆序时应当等于 n(n-1)/2。
var moveCount int

func insertionSort(a []int) {
	n := len(a)
	// a[0] 单独一个元素天然有序，所以从下标 1 开始扩张前缀。
	for i := 1; i < n; i++ {
		key := a[i] // 先暂存，后面 a[i] 会被覆盖
		j := i - 1
		// 先判 j >= 0 再取 a[j]，顺序反了就会读到 -1。
		for j >= 0 {
			cmpCount++ // 这次比较真实发生，无论结果如何都计数
			// 小于等于就停：相等元素原地不动，保证稳定。
			if a[j] <= key {
				break
			}
			a[j+1] = a[j] // 整体后移一格，不交换
			moveCount++
			j--
		}
		a[j+1] = key // 此时 a[j] <= key（或 j == -1），key 落在 j+1
	}
}

func show(a []int) string {
	s := "["
	for i, v := range a {
		if i != 0 {
			s += ", "
		}
		s += fmt.Sprint(v)
	}
	return s + "]"
}

func checkSorted(input, expect []int, name string) {
	a := make([]int, len(input))
	copy(a, input)
	insertionSort(a)
	if len(a) != len(expect) {
		panic(fmt.Sprintf("case %s failed: got %s expect %s",
			name, show(a), show(expect)))
	}
	for i := range a {
		if a[i] != expect[i] {
			panic(fmt.Sprintf("case %s failed: got %s expect %s",
				name, show(a), show(expect)))
		}
	}
}

func main() {
	// 1. 正常用例。
	checkSorted([]int{5, 2, 4, 1, 3}, []int{1, 2, 3, 4, 5}, "normal")

	// 2. 边界：空数组、单元素。
	checkSorted([]int{}, []int{}, "empty")
	checkSorted([]int{7}, []int{7}, "single")

	// 3. 已有序：每轮只比较一次就停，总比较次数恰好 n-1，一次移动都没有。
	cmpCount = 0
	moveCount = 0
	checkSorted([]int{1, 2, 3, 4, 5}, []int{1, 2, 3, 4, 5}, "already-sorted")
	if cmpCount != 4 || moveCount != 0 {
		panic(fmt.Sprintf("已有序：比较 %d 次、移动 %d 次，应为 4 和 0",
			cmpCount, moveCount))
	}

	// 4. 逆序：最坏情况，移动次数 = 0+1+...+(n-1) = n(n-1)/2。
	cmpCount = 0
	moveCount = 0
	checkSorted([]int{5, 4, 3, 2, 1}, []int{1, 2, 3, 4, 5}, "reverse")
	if moveCount != 10 {
		panic(fmt.Sprintf("逆序移动 %d 次，应为 10 次", moveCount))
	}
	if cmpCount != 10 { // 每轮都一路比到 j == -1
		panic(fmt.Sprintf("逆序比较 %d 次，应为 10 次", cmpCount))
	}

	// 5. 重复值：全相同 + 交替重复。全相同时每轮只比较一次，
	//    说明相等元素不会触发后移，稳定性靠的正是这一点。
	checkSorted([]int{7, 7, 7, 7}, []int{7, 7, 7, 7}, "all-same")
	cmpCount = 0
	checkSorted([]int{7, 7, 7, 7}, []int{7, 7, 7, 7}, "all-same-cmp")
	if cmpCount != 3 {
		panic(fmt.Sprintf("全相同比较 %d 次，应为 3 次", cmpCount))
	}
	checkSorted([]int{2, 1, 2, 1, 2, 1}, []int{1, 1, 1, 2, 2, 2},
		"duplicates")
	checkSorted([]int{0, 0, -1, -1, 0}, []int{-1, -1, 0, 0, 0},
		"dups-with-negative")

	// 6. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
	rng := rand.New(rand.NewSource(20240501))
	for _, ln := range []int{0, 1, 2, 3, 10, 100} {
		for round := 0; round < 20; round++ {
			a := make([]int, ln)
			for i := range a {
				a[i] = rng.Intn(101) - 50
			}
			expect := make([]int, ln)
			copy(expect, a)
			sort.Ints(expect) // 仅用于校验：实现部分没有使用内置排序
			insertionSort(a)
			for i := range a {
				if a[i] != expect[i] {
					panic(fmt.Sprintf("len=%d round=%d: got %s expect %s",
						ln, round, show(a), show(expect)))
				}
			}
		}
	}

	fmt.Println("all tests passed")
}
