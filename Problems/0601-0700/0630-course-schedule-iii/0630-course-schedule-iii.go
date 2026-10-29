/*
============================================================================
630. 课程表 III（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/course-schedule-iii/
分类：贪心 · 大根堆（可反悔）

这里有 n 门不同的在线课程，按从 1 到 n 编号。给你一个数组 courses ，其中
courses[i] = [duration_i, lastDay_i] 表示第 i 门课将会 持续 上 duration_i
天课，并且必须在不晚于 lastDay_i 的时候完成。

你的学期从第 1 天开始。且不能同时修读两门及两门以上的课程。

返回你最多可以修读的课程数目。

示例 1：
输入：courses = [[100,200], [200,1300], [1000,1250], [2000,3200]]
输出：3
解释：
这里一共有 4 门课程，但是你最多可以修 3 门：
首先，修第 1 门课，耗费 100 天，在第 100 天完成，在第 101 天开始下门课。
第二，修第 3 门课，耗费 1000 天，在第 1100 天完成，在第 1101 天开始下门课程。
第三，修第 2 门课，耗时 200 天，在第 1300 天完成。
第 4 门课现在不能修，因为将会在第 3300 天完成它，这已经超出了关闭日期。

示例 2：
输入：courses = [[1,2]]
输出：1

示例 3：
输入：courses = [[3,2],[4,3]]
输出：0

提示：
- 1 <= courses.length <= 10^4
- 1 <= duration_i, lastDay_i <= 10^4
*/
package main

import (
	"container/heap"
	"fmt"
	"sort"
)

/*
解法：按截止时间排序 + 大根堆维护已选课程（可反悔贪心）【最优解】

为什么慢：课程不能并行，选课顺序直接决定能否在截止日前收尾。若把「选哪些
课」当成 0/1 背包，按天做 DP 是 O(n·maxDay)；若枚举子集则是 2^n。更常见
的错误是「按截止时间扫一遍，放不下当前课就跳过」—— 这会漏掉「用当前短课
换掉已选的最长课」这种反悔，例如 [[5,5],[2,6],[4,6]] 会得到 1 而不是 2。

换个角度：只盯两个量——「已选了几门」和「已选课程的总时长」。门数相同
时总时长越小，对后面越有利。于是遇到新课程放不下时，唯一值得做的调整
是把已选里最长的那门换成当前这门：门数不变，总时长下降。

关键点：
  · 先按 lastDay 升序排：交换论证，两门课里截止早的排在前面总能满足
    更紧的约束，反过来不一定，所以按截止时间处理不会漏解
  · 放得下（total + t <= d）就无条件收下，不做任何比较
  · 放不下时，只有「堆顶 > 当前时长」才替换：换完总时长下降，
    说明这次反悔有收益；堆顶 <= 当前时长则替换只会变差，直接丢弃当前课
  · 替换前必须先判堆非空：堆空说明一门都没选、total = 0，此时
    total + t > d 只可能是 t > d，这门课自己就修不完，只能丢弃
  · total 与堆内容必须同步：先 total -= 堆顶 - t，再 pop 再 push，
    顺序写反会让 total 与堆短暂不一致，后续判断全部失真
  · Go 里 sort.Slice 的比较函数只读 courses，不做原地交换，
    排序键写成 `courses[i][1] < courses[j][1]` 即可

手推一遍（示例 1）：按 lastDay 排序后为
(100,200), (1000,1250), (200,1300), (2000,3200)：
  (100,200)：0 + 100 <= 200，收下，total = 100
  (1000,1250)：100 + 1000 = 1100 <= 1250，收下，total = 1100
  (200,1300)：1100 + 200 = 1300 <= 1300，收下，total = 1300，已有 3 门
  (2000,3200)：1300 + 2000 = 3300 > 3200，堆顶 1000 < 2000，丢弃
  答案 3 ✔

复杂度分析：
  时间 O(n log n)：排序 O(n log n)，每门课至多一次入堆、一次出堆，
    堆操作 O(log n)
  空间 O(n)：堆里最坏存下全部 n 门课的时长
*/

// maxHeap 存已选课程的时长，堆顶是其中最长的一门
type maxHeap []int

func (h maxHeap) Len() int           { return len(h) }
func (h maxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h maxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *maxHeap) Push(x any) { *h = append(*h, x.(int)) }

func (h *maxHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

// 契约：courses[i] = {duration, lastDay}，返回能修完的最多课程数
func scheduleCourse(courses [][]int) int {
	// 按关闭时间升序：同一门课越早截止，越该占前面的位置
	sort.Slice(courses, func(i, j int) bool {
		return courses[i][1] < courses[j][1]
	})

	chosen := &maxHeap{}
	heap.Init(chosen)
	total := 0

	for _, course := range courses {
		duration, lastDay := course[0], course[1]

		// 放得下就直接收下，无需与已选课程比较
		if total+duration <= lastDay {
			heap.Push(chosen, duration)
			total += duration
			continue
		}

		// 堆空说明一门都没选，此时 total 必为 0，
		// 走到这里只能是 duration > lastDay，这门课自身就无法完成
		if chosen.Len() == 0 {
			continue
		}

		// 只有换掉更长的课才能降低总时长；否则替换只会更差
		if (*chosen)[0] > duration {
			total -= (*chosen)[0] - duration
			heap.Pop(chosen)
			heap.Push(chosen, duration)
		}
	}

	return chosen.Len()
}

// 按值传参：scheduleCourse 会原地排序，避免污染用例
func run630(courses [][]int) int {
	copyCourses := make([][]int, len(courses))
	copy(copyCourses, courses)
	return scheduleCourse(copyCourses)
}

func check630(courses [][]int, want int) {
	if got := run630(courses); got != want {
		panic(fmt.Sprintf("scheduleCourse(%v) = %d, want %d", courses, got, want))
	}
}

func main() {
	// 示例 1：需要在 4 门里放弃最长的一门
	check630([][]int{{100, 200}, {200, 1300}, {1000, 1250}, {2000, 3200}}, 3)

	// 示例 2
	check630([][]int{{1, 2}}, 1)

	// 示例 3：没有一门课能在截止日前修完
	check630([][]int{{3, 2}, {4, 3}}, 0)

	// 边界：空课表（题面保证至少 1 门，这里只验证健壮性）
	check630([][]int{}, 0)

	// 边界：第一门课就超期，堆为空时必须跳过而不是取堆顶
	check630([][]int{{10, 5}, {1, 10}}, 1)

	// 反直觉：放不下当前课时要反悔 —— 用 2 换掉 5 才装得下第 3 门
	check630([][]int{{5, 5}, {2, 6}, {4, 6}}, 2)

	// 反直觉：链式截止时间下只能修 2 门
	check630([][]int{{1, 2}, {2, 3}, {3, 4}, {4, 5}}, 2)

	// 反直觉：三门课截止日相同，只能塞下两门短的
	check630([][]int{{9, 10}, {8, 10}, {1, 10}}, 2)

	// 反直觉：靠反悔挤进 4 门 —— 丢掉 10 才能装下 3 + 5 + 2
	check630([][]int{{7, 17}, {3, 12}, {10, 20}, {5, 22}, {2, 6}}, 4)

	// 边界：n = 10^4 门课，每门 1 天、第 10^4 天截止，全部修完
	big := make([][]int, 10000)
	for i := range big {
		big[i] = []int{1, 10000}
	}
	check630(big, 10000)

	fmt.Println("all tests passed")
}
