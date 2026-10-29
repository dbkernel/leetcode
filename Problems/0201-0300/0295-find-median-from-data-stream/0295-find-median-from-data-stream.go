/*
============================================================================
295. 数据流的中位数（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-median-from-data-stream/
分类：堆 · 对顶堆（数据流中位数）
============================================================================

中位数是有序整数列表中的中间值。如果列表的大小是偶数，则没有中间值，
中位数是两个中间值的平均值。

例如 arr = [2,3,4] 的中位数是 3 。
例如 arr = [2,3] 的中位数是 (2 + 3) / 2 = 2.5 。

实现 MedianFinder 类:
  - MedianFinder() 初始化 MedianFinder 对象。
  - void addNum(int num) 将数据流中的整数 num 添加到数据结构中。
  - double findMedian() 返回到目前为止所有元素的中位数。与实际答案相差
    10^-5 以内的答案将被接受。

示例 1：
输入：
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
输出：
[null, null, null, 1.5, null, 2.0]

解释：
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // 返回 1.5 ((1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0

提示：
-10^5 <= num <= 10^5
在调用 findMedian 之前，数据结构中至少有一个元素
最多 5 * 10^4 次调用 addNum 和 findMedian
*/
package main

import (
	"container/heap"
	"fmt"
	"math"
	"sort"
)

/*
解法一：有序数组 + 二分插入【非最优】

为什么慢：这个解法把「让数组保持有序」当成不变式，于是 addNum 每次都要
把新元素放到正确位置。二分查找只能把「找位置」这一步压到 O(log n)，找到
之后仍要把后面所有元素整体后移一格，那一步是 O(n)。调用 5 * 10^4 次
addNum，最坏就是 5 * 10^4 * 5 * 10^4 次移动。

换个角度：findMedian 只看「中间那一两个元素」，并不需要全体有序。把数据
按大小劈成两半，各自内部只要能让「最靠中间的那个」浮到顶，就够了。

关键点：

	· 用 sort.SearchInts 找第一个 >= num 的位置；等值元素排它后面也合法，
	  重复值的相对次序不影响中位数
	· 切片插入要「先 append 扩容、再从后往前搬」，否则会覆盖还没读到的值
	· 偶数个元素时取 len/2 与 len/2-1 两处的平均，奇数个时取 len/2
	· findMedian 是 O(1)：下标算出来即可，不需要再排序
	· 这是「有序数组」解法的本地命名，平台上的类名是 MedianFinder

以示例的调用序列为例：

	addNum(1) → [1]，findMedian 时 len = 1，取 [0] → 1.0
	addNum(2) → [1,2]，len = 2，取 (1 + 2) / 2 → 1.5
	addNum(3) → [1,2,3]，len = 3，取 [1] → 2.0 ✔

复杂度分析：

	时间 addNum O(n)（插入时的整体后移），findMedian O(1)
	空间 O(n)：装下所有元素
*/
type MedianFinderSorted struct {
	data []int
}

// NewMedianFinderSorted 对应平台上的构造函数，换个名字免得和最优解重名
func NewMedianFinderSorted() *MedianFinderSorted {
	return &MedianFinderSorted{data: []int{}}
}

func (m *MedianFinderSorted) AddNum(num int) {
	// i 之前的元素都 < num，i 之后的元素都 >= num
	i := sort.SearchInts(m.data, num)
	m.data = append(m.data, 0)
	copy(m.data[i+1:], m.data[i:])
	m.data[i] = num
}

func (m *MedianFinderSorted) FindMedian() float64 {
	n := len(m.data)
	if n%2 == 1 {
		return float64(m.data[n/2])
	}
	// 先转 float64 再相加，避免两个大整数相加后精度被截断
	return (float64(m.data[n/2-1]) + float64(m.data[n/2])) / 2
}

/*
解法二：对顶双堆（大根堆放小的一半，小根堆放大的一个半）【最优解】

契约：low 是大根堆，装较小的一半；high 是小根堆，装较大的一半。不变式是
len(low) == len(high) 或者 len(low) == len(high) + 1 —— 中位数只由两个
堆顶决定。

为什么慢：解法一的瓶颈在「维持全体有序」。可中位数并不关心 low 内部、
high 内部的次序，只关心两条边界：low 的最大值与 high 的最小值。把这两条
边界做成堆顶，其余元素爱怎么排就怎么排。

换个角度：新元素只可能落在 low 的最大值和 high 的最小值之间。所以先比较
一次决定它进哪个堆，再用一次「搬运」把两个堆的大小差拉回 0 或 1。

关键点：
  · 先决定入哪个堆：num <= low 的堆顶进 low，否则进 high；low 为空时直接进 low
  · 平衡只朝一个方向修：low 比 high 多 2 个就搬一个到 high；high 比 low 多
    就搬回 low。两步都是「先 push 对方堆顶、再 pop」，顺序反了会白搬一个元素
  · 两个堆的大小差永远不超过 1，所以 findMedian 只需读两个堆顶
  · Go 没有 double，float64 就是平台要求的返回类型；除以 2 时两个操作数
    必须是浮点，否则会做整数除法截掉 0.5
  · 平台签名是 type MedianFinder struct + Constructor() + AddNum/FindMedian，
    这里照录；解法一的类型只能换个名字

以示例的调用序列为例：
  AddNum(1)：low = [1]，high = []
  AddNum(2)：2 > 堆顶 1，进 high → low = [1]，high = [2]
    FindMedian：两堆等大 → (1 + 2) / 2 = 1.5 ✔
  AddNum(3)：3 > 堆顶 2，进 high → high 比 low 多，把 2 搬回 low
    low = [2,1]（堆顶 2），high = [3]；low 多一个 → 取堆顶 2.0 ✔

复杂度分析：
  时间 AddNum O(log n)：最多两次堆操作；FindMedian O(1)
  空间 O(n)：两个堆合起来正好装下所有元素
*/

// 大根堆：Less 用 `>`，于是最大值在堆顶，用来读「较小一半」的上界
type maxHeap []int

func (h maxHeap) Len() int           { return len(h) }
func (h maxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h maxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *maxHeap) Pop() any {
	old := *h
	last := len(old) - 1
	v := old[last]
	*h = old[:last]
	return v
}

// 小根堆：Less 用 `<`，于是最小值在堆顶，用来读「较大一半」的下界
type minHeap []int

func (h minHeap) Len() int           { return len(h) }
func (h minHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h minHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *minHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *minHeap) Pop() any {
	old := *h
	last := len(old) - 1
	v := old[last]
	*h = old[:last]
	return v
}

type MedianFinder struct {
	low  maxHeap
	high minHeap
}

// Constructor 是平台约定的构造函数名
func Constructor() MedianFinder {
	return MedianFinder{}
}

func (m *MedianFinder) AddNum(num int) {
	// 只跟 low 的堆顶比：low 的最大值就是「较小那一半」的上界
	if m.low.Len() == 0 || num <= m.low[0] {
		heap.Push(&m.low, num)
	} else {
		heap.Push(&m.high, num)
	}
	m.rebalance()
}

func (m *MedianFinder) FindMedian() float64 {
	if m.low.Len() > m.high.Len() {
		return float64(m.low[0])
	}
	return (float64(m.low[0]) + float64(m.high[0])) / 2
}

func (m *MedianFinder) rebalance() {
	// 只可能是 low 多出 2 个，或者 high 多出 1 个
	if m.low.Len() > m.high.Len()+1 {
		heap.Push(&m.high, heap.Pop(&m.low))
	} else if m.high.Len() > m.low.Len() {
		heap.Push(&m.low, heap.Pop(&m.high))
	}
}

// ===== 测试辅助 =====
func bruteMedian(prefix []int) float64 {
	v := make([]int, len(prefix))
	copy(v, prefix)
	sort.Ints(v)
	n := len(v)
	if n%2 == 1 {
		return float64(v[n/2])
	}
	return (float64(v[n/2-1]) + float64(v[n/2])) / 2
}

func nearlyEqual(a, b float64) bool {
	return math.Abs(a-b) < 1e-6
}

// 依次 AddNum，每到一个检查点就比较两种实现的 FindMedian
// probes 里的数字是「已经喂进去多少个元素」，必须递增
func check(nums []int, probes []int) {
	sortedImpl := NewMedianFinderSorted()
	heapImpl := Constructor()

	fed := 0
	for _, c := range probes {
		for fed < c {
			sortedImpl.AddNum(nums[fed])
			heapImpl.AddNum(nums[fed])
			fed++
		}

		want := bruteMedian(nums[:c])
		if got := sortedImpl.FindMedian(); !nearlyEqual(got, want) {
			panic(fmt.Sprintf("sorted(%v, %d) = %v, want %v", nums, c, got, want))
		}
		if got := heapImpl.FindMedian(); !nearlyEqual(got, want) {
			panic(fmt.Sprintf("heap(%v, %d) = %v, want %v", nums, c, got, want))
		}
	}
}

func main() {
	// 示例 1：按题目给的调用序列回放，1 个、2 个、3 个元素各查一次
	check([]int{1, 2, 3}, []int{1, 2, 3})

	// 边界：只有一个元素，奇偶分支走的是奇数那一支
	check([]int{5}, []int{1})

	// 边界：全部是负数，验证大根堆的比较不会因为符号出错
	check([]int{-1, -2, -3, -4, -5}, []int{1, 2, 3, 4, 5})

	// 边界：全部相同，两个堆的大小差永远是 0 或 1
	check([]int{1, 1, 1, 1}, []int{1, 2, 3, 4})

	// 边界：先大后小地插入，正是「不预先排序」最容易错的地方
	check([]int{100, 90, 80, 70, 60}, []int{1, 2, 3, 4, 5})

	// 边界：范围的上下界，平均值必须用浮点算
	check([]int{-100000, 100000}, []int{1, 2})

	// 边界：相邻两个元素，验证除以 2 不被截断
	check([]int{1, 2}, []int{2})

	// 规模：1000 个元素，每加 100 个查一次，和暴力排序对照
	{
		nums := []int{}
		probes := []int{}
		for i := 0; i < 1000; i++ {
			// 固定公式生成，保证每次运行结果一致
			nums = append(nums, (i*37)%201-100)
			if (i+1)%100 == 0 {
				probes = append(probes, i+1)
			}
		}
		check(nums, probes)
	}

	fmt.Println("all tests passed")
}
