/*
============================================================================
LCR 183 / 剑指 Offer 59 - I. 望远镜中最高的海拔（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/hua-dong-chuang-kou-de-zui-da-zhi-lcof/
分类：单调队列 · 滑动窗口最值

科技馆内有一台虚拟观景望远镜，它可以用来观测特定纬度地区的地形情况。该纬
度的海拔数据记于数组 heights ，其中 heights[i] 表示对应位置的海拔高度。请
找出并返回望远镜视野范围 limit 内，可以观测到的最高海拔值。

示例 1：
输入：heights = [14,2,27,-5,28,13,39], limit = 3
输出：[27,27,28,28,39]
解释：
滑动窗口的位置                最大值
---------------               -----
[14 2 27] -5 28 13 39          27
14 [2 27 -5] 28 13 39          27
14 2 [27 -5 28] 13 39          28
14 2 27 [-5 28 13] 39          28
14 2 27 -5 [28 13 39]          39

提示：
你可以假设输入总是有效的，在输入数组不为空的情况下：
- 1 <= limit <= heights.length
- -10000 <= heights[i] <= 10000

注意：本题与主站 239 题相同：
https://leetcode.cn/problems/sliding-window-maximum/

说明：本题在 leetcode.cn 上已改号为 LCR 183，题名由「滑动窗口的最大值」改
为「望远镜中最高的海拔」，平台函数名也从 maxSlidingWindow 变成
maxAltitude(heights, limit)，下面按平台给出的签名实现。
*/
package main

import (
	"container/heap"
	"fmt"
	"slices"
)

/*
解法一：逐个窗口重新扫描（暴力）【非最优】

为什么慢：窗口右移一格，只有最左的被移出、最右的被移入，中间 limit-1 个
元素完全没变；暴力却把整段重扫一遍，把没变的信息又算了一次。n-limit+1 个
窗口各扫 limit 个元素，总时间 O(n·limit)。

换个角度：要让比较的次数降下来，就必须把"上一窗口的结论"带过来 —— 即用
一个结构保存仍然"有资格成为最大值"的候选，窗口移动时只做增删。

关键点：
  · 窗口 i 对应区间 [i, i+limit-1]，i 取 0..n-limit，共 n-limit+1 个
  · 内层循环的右端是 i+limit-1，写成开区间即 i+limit
  · limit == 1 时答案就是原数组，可以顺带验证循环边界

以示例 1（heights = [14,2,27,-5,28,13,39]，limit = 3）为例：
  窗口 0：[14,2,27] → 27
  窗口 1：[2,27,-5] → 27
  窗口 2：[27,-5,28] → 28 ✔

复杂度分析：
  时间 O((n-limit+1)·limit)：最坏（limit ≈ n/2）为 O(n^2)
  空间 O(1) 额外：只有扫描用的游标，答案切片不计

解法二：优先队列（大根堆）【非最优】

为什么慢：解法一的问题在于"重复比较"，而比较的目的只是取最大值。若把元素
丢进大根堆，取最大值就是 O(1) 看堆顶，插入 O(log n)。

换个角度：堆里存 (海拔, 下标)。堆顶一定是堆中最大值；如果它的下标已经滑出
当前窗口左边界，那么它以后永远不会再进入窗口，可以永久弹出。反复弹到堆顶
落在窗口内，堆顶就是答案。

关键点：
  · 先入堆再清理过期：新元素可能比堆顶更大，先清理等于白弹一次
  · 过期判定是 堆顶下标 <= i-limit：当前窗口是 [i-limit+1, i]，恰好等于
    i-limit 的下标已经在窗口外
  · 堆里存下标而不是只存值：值无法判断是否过期
  · Go 的 container/heap 需要自己实现 Len / Less / Swap / Push / Pop

以示例 1 的窗口 2（[27,-5,28]）为例：
  入堆 28 后堆顶是 (28,4)；下标 4 在 [2,4] 内，直接取 28 ✔
  （此时 (27,2) 已出窗口但不必立刻清理，等它冒到堆顶时再弹）

复杂度分析：
  时间 O(n log n)：每个元素入堆一次、出堆至多一次
  空间 O(n)：单调递增的输入会让堆里堆积几乎全部元素

解法三：单调队列（双端队列维护递减候选）【最优解】

为什么慢：堆的 O(log n) 花在"维持全序"上，但本题只要最大值。观察：若
i < j 且 heights[i] <= heights[j]，那么只要 j 还在窗口里，i 就一定也在
窗口里（i 更靠左，先出窗口的也是 i），于是 i 永远压不过 j，可以立刻丢弃。
丢弃之后剩下的下标必然"下标递增、值严格递减"，最大值就是队首。

换个角度：队列从队尾进、从队首出。入队前把队尾所有不大于新元素的下标弹掉；
窗口右移后把队首已过期（<= i-limit）的下标弹掉。队首恒为窗口最大值。

关键点：
  · 队尾弹出条件用 heights[q 尾] <= heights[i]（相等也弹）：相等时保留
    更靠右的那个，它在窗口里停留更久
  · 先弹队尾、再入队、最后弹队首过期下标：队首的过期判定依赖当前 i
  · 过期判定用 <= i-limit，与窗口 [i-limit+1, i] 对齐
  · 第一个窗口要先整体填满再开始滑动，否则会漏掉前 limit-1 个元素

以示例 1（heights = [14,2,27,-5,28,13,39]，limit = 3）为例：
  前三个 14、2、27：27 入队时弹掉 14、2，队列存 [27]
  i=3（-5）：入队 [27,-5]，窗口 [2..3] 的队首 27 未过期 → 27
  i=4（28）：弹掉 -5、27，队列 [28] → 28
  i=6（39）：弹掉 28、13，队列 [39] → 39
  答案 [27,27,28,28,39] ✔

复杂度分析：
  时间 O(n)：每个下标入队一次、出队至多一次，均摊 O(1)
  空间 O(limit)：队列里最多 limit+1 个下标（过期元素在窗口右移时立即弹出）

解法四：分块 + 前缀 / 后缀最大值预处理【非最优】

为什么慢：解法三用队列维护"随窗口滑动而变化"的最大值；换个思路，把每个
窗口拆成静态的区间最值查询，就能用预处理把查询变成 O(1)。

换个角度：把数组按 limit 个一组切块。一个长度为 limit 的窗口要么恰好等于
一整块（起点是 limit 的倍数），要么跨两块 —— 左块的**后缀**加右块的**前缀**。
于是预处理出每块内的前缀最大值 prefixMax 和后缀最大值 suffixMax，答案就是
max(suffixMax[i], prefixMax[i+limit-1])。

关键点：
  · prefixMax 在每个块的第一格（i % limit == 0）重置为 heights[i]
  · suffixMax 在每个块的最后一格（(i+1) % limit == 0 或 i == n-1）重置
  · 窗口起点是 limit 的倍数时，suffixMax[i] 与 prefixMax[i+limit-1] 都等于
    整块最大值，公式仍然成立，不必特判
  · 时间与解法三同为 O(n)；但解法三空间 O(limit)，这里要 O(n)，空间更差

以示例 1（heights = [14,2,27,-5,28,13,39]，limit = 3）为例：
  分块 [14,2,27] [−5,28,13] [39]
  prefixMax = [14,14,27, -5,28,28, 39]
  suffixMax = [27,27,27, 28,28,13, 39]
  窗口 2（起点 2）：max(suffixMax[2]=27, prefixMax[4]=28) = 28 ✔

复杂度分析：
  时间 O(n)：两遍预处理 + 一遍查询
  空间 O(n)：prefixMax、suffixMax 各 n 个元素
*/

// 解法一：逐个窗口重新扫描（暴力）
func maxAltitudeBrute(heights []int, limit int) []int {
	n := len(heights)
	if n == 0 || limit <= 0 {
		return []int{}
	}
	ans := make([]int, 0, n-limit+1)
	for i := 0; i+limit <= n; i++ {
		best := heights[i]
		// 窗口是 [i, i+limit-1]，右端写成开区间就是 i+limit
		for j := i + 1; j < i+limit; j++ {
			if heights[j] > best {
				best = heights[j]
			}
		}
		ans = append(ans, best)
	}
	return ans
}

// 堆元素：海拔 + 下标，下标用于判断是否滑出窗口
type heapItem struct {
	value int
	index int
}

// 大根堆：值大的在前；值相同按下标大的在前
type maxHeap []heapItem

func (h maxHeap) Len() int { return len(h) }
func (h maxHeap) Less(i, j int) bool {
	if h[i].value != h[j].value {
		return h[i].value > h[j].value
	}
	return h[i].index > h[j].index
}
func (h maxHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x any)   { *h = append(*h, x.(heapItem)) }
func (h *maxHeap) Pop() any {
	old := *h
	n := len(old)
	item := old[n-1]
	*h = old[:n-1]
	return item
}

// 解法二：优先队列（大根堆）
func maxAltitudeHeap(heights []int, limit int) []int {
	n := len(heights)
	if n == 0 || limit <= 0 {
		return []int{}
	}
	h := &maxHeap{}
	for i := 0; i < limit; i++ {
		*h = append(*h, heapItem{heights[i], i})
	}
	heap.Init(h)

	ans := make([]int, 1, n-limit+1)
	ans[0] = (*h)[0].value
	for i := limit; i < n; i++ {
		// 先入堆：新元素可能就是新的最大值
		heap.Push(h, heapItem{heights[i], i})
		// 当前窗口是 [i-limit+1, i]，下标 <= i-limit 的已永不可能回来
		for (*h)[0].index <= i-limit {
			heap.Pop(h)
		}
		ans = append(ans, (*h)[0].value)
	}
	return ans
}

// 解法三：单调队列（平台入口签名）
func maxAltitude(heights []int, limit int) []int {
	n := len(heights)
	if n == 0 || limit <= 0 {
		return []int{}
	}
	q := make([]int, 0, limit+1)
	// 先把第一个窗口填满，后面才开始滑动
	for i := 0; i < limit; i++ {
		// 相等也弹：保留更靠右的那个，它在窗口里活得更久
		for len(q) > 0 && heights[q[len(q)-1]] <= heights[i] {
			q = q[:len(q)-1]
		}
		q = append(q, i)
	}

	ans := make([]int, 1, n-limit+1)
	ans[0] = heights[q[0]]
	for i := limit; i < n; i++ {
		for len(q) > 0 && heights[q[len(q)-1]] <= heights[i] {
			q = q[:len(q)-1]
		}
		q = append(q, i)
		// 窗口是 [i-limit+1, i]，队首滑出左边界就永久丢弃
		for q[0] <= i-limit {
			q = q[1:]
		}
		ans = append(ans, heights[q[0]])
	}
	return ans
}

// 解法四：分块 + 前缀 / 后缀最大值预处理
func maxAltitudeBlock(heights []int, limit int) []int {
	n := len(heights)
	if n == 0 || limit <= 0 {
		return []int{}
	}
	prefixMax := make([]int, n)
	suffixMax := make([]int, n)
	for i := 0; i < n; i++ {
		// 每块的第一格是前缀递推的起点
		if i%limit == 0 {
			prefixMax[i] = heights[i]
		} else if prefixMax[i-1] > heights[i] {
			prefixMax[i] = prefixMax[i-1]
		} else {
			prefixMax[i] = heights[i]
		}
	}
	for i := n - 1; i >= 0; i-- {
		// 每块的最后一格（或数组末尾）是后缀递推的起点
		if i == n-1 || (i+1)%limit == 0 {
			suffixMax[i] = heights[i]
		} else if suffixMax[i+1] > heights[i] {
			suffixMax[i] = suffixMax[i+1]
		} else {
			suffixMax[i] = heights[i]
		}
	}

	ans := make([]int, 0, n-limit+1)
	for i := 0; i+limit <= n; i++ {
		// 左块后缀与右块前缀的较大者就是整窗最大值
		left := suffixMax[i]
		right := prefixMax[i+limit-1]
		if left > right {
			ans = append(ans, left)
		} else {
			ans = append(ans, right)
		}
	}
	return ans
}

// 四个解法都要过同一组用例；期望值由题面定义独立给出
func checkAltitude(heights []int, limit int, want []int) {
	cases := []struct {
		name string
		got  []int
	}{
		{"brute", maxAltitudeBrute(heights, limit)},
		{"heap", maxAltitudeHeap(heights, limit)},
		{"mono", maxAltitude(heights, limit)},
		{"block", maxAltitudeBlock(heights, limit)},
	}
	for _, c := range cases {
		if !slices.Equal(c.got, want) {
			panic(fmt.Sprintf("%s(%v, %d) = %v, want %v",
				c.name, heights, limit, c.got, want))
		}
	}
}

// 随机交叉验证：海拔取负值，专门覆盖"负数被当成空位"这类错误
func crossCheckAltitude() {
	seed := uint32(20240607)
	for round := 0; round < 200; round++ {
		seed = seed*1103515245 + 12345
		n := int((seed>>16)%40) + 1
		heights := make([]int, n)
		for i := 0; i < n; i++ {
			seed = seed*1103515245 + 12345
			heights[i] = int((seed>>16)%21) - 10
		}
		seed = seed*1103515245 + 12345
		limit := int((seed>>16)%uint32(n)) + 1

		checkAltitude(heights, limit, maxAltitudeBrute(heights, limit))
	}
}

func main() {
	// 示例 1：官方给出的五个窗口
	checkAltitude([]int{14, 2, 27, -5, 28, 13, 39}, 3,
		[]int{27, 27, 28, 28, 39})

	// 边界：limit == 1，答案就是原数组
	checkAltitude([]int{14, 2, 27, -5}, 1, []int{14, 2, 27, -5})

	// 边界：limit == n，窗口只有一个
	checkAltitude([]int{14, 2, 27, -5}, 4, []int{27})

	// 边界：单元素数组
	checkAltitude([]int{7}, 1, []int{7})

	// 反直觉：全负数时"最大"是 -3，不能拿 0 当初值
	checkAltitude([]int{-7, -8, -3, -4}, 2, []int{-7, -3, -3})

	// 反直觉：全等值，队尾弹出用 <= 还是 < 都不影响答案，但不能漏窗口
	checkAltitude([]int{1, 1, 1}, 2, []int{1, 1})

	// 边界：严格递减，最大值一路滑出窗口
	checkAltitude([]int{9, 8, 7, 6}, 2, []int{9, 8, 7})

	// 边界：严格递增，最大值每次都是新进来的元素
	checkAltitude([]int{1, 2, 3, 4}, 2, []int{2, 3, 4})

	// 边界：取海拔上下限
	checkAltitude([]int{-10000, 10000, 0}, 2, []int{10000, 10000})

	// 反直觉：窗口里有多个并列最大值的候选，滑出后必须回落到次大
	checkAltitude([]int{5, 4, 3, 2}, 3, []int{5, 4})

	// 边界：limit == n 且全等
	checkAltitude([]int{3, 3, 3, 3}, 4, []int{3})

	// 随机交叉验证
	crossCheckAltitude()

	// 大规模用例：n = 10^5、limit = 5×10^4，只有下标 777 处是 10^4。
	// 期望值按"窗口是否覆盖 777"直接推出，不必用暴力扫一遍。
	{
		const n = 100000
		const limit = 50000
		const peak = 777
		heights := make([]int, n)
		for i := range heights {
			heights[i] = 5
		}
		heights[peak] = 10000

		want := make([]int, 0, n-limit+1)
		for i := 0; i+limit <= n; i++ {
			if i <= peak && peak <= i+limit-1 {
				want = append(want, 10000)
			} else {
				want = append(want, 5)
			}
		}

		for _, got := range [][]int{
			maxAltitudeHeap(heights, limit),
			maxAltitude(heights, limit),
			maxAltitudeBlock(heights, limit),
		} {
			if !slices.Equal(got, want) {
				panic("large case mismatch")
			}
		}
	}

	fmt.Println("all tests passed")
}
