/*
============================================================================
347. 前 K 个高频元素（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/top-k-frequent-elements/
分类：堆 · 频率统计（小根堆 / 快选 / 桶排序）
============================================================================

给你一个整数数组 nums 和一个整数 k ，请你返回其中出现频率前 k 高的元素。
你可以按 任意顺序 返回答案。

示例 1：
输入：nums = [1,1,1,2,2,3], k = 2
输出：[1,2]

示例 2：
输入：nums = [1], k = 1
输出：[1]

提示：
- 1 <= nums.length <= 10^5
- k 的取值范围是 [1, 数组中不相同的元素的个数]
- 题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的

进阶：你所设计算法的时间复杂度 必须 优于 O(n log n) ，其中 n 是数组大小。
*/
package main

import (
	"container/heap"
	"fmt"
	"math/rand"
	"sort"
)

/*
解法一：哈希计数 + 大小为 k 的小根堆【非最优】

为什么慢：最容易想到的是「统计完频率，把所有出现的数按频率从大到小排序，
取前 k 个」。这一步要对 m 个不同的数做比较排序，O(m log m)。但题目只要前
k 名，剩下 m - k 个数的相对次序完全没人关心，排序把信息浪费在了这里。

换个角度：只留 k 个候选。维护一个容量为 k 的小根堆，堆顶是「当前候选里
频率最低」的那个；新元素频率比堆顶高就换掉堆顶。这样堆里始终是前 k 名，
每个元素最多进出堆一次。

关键点：
  · 堆里存 (频率, 数值) 而不是单个数值 —— 比较的对象是频率，不是大小
  · 必须是小根堆：堆顶是候选中最差的那个，才谈得上「被替换」
  · 容量判断用 Len() > k 而不是 == k：先放进去再挤掉，边界只有一处
  · 结果顺序任意，题目不要求堆内元素有序
  · Go 的 container/heap 里由 Less 决定堆顶：Less 返回「i 更优先」，
    所以比较频率用 < 得到的就是小根堆

以示例 1（nums = [1,1,1,2,2,3], k = 2）为例：
  频率：1 → 3，2 → 2，3 → 1
  堆放入 (3,1)、(2,2) 后已满 k = 2
  再来 (1,3)：1 < 堆顶 2，直接丢掉
  堆内剩下 1 和 2 ✔

复杂度分析：
  时间 O(n log k)：统计频率 O(n)，每个不同的数进出堆各 O(log k)
  空间 O(m + k)：频率表要装 m 个不同的数，堆最多 k 个
*/

// freqItem 既用于堆，也用于快选：按频率比较，值只是附带信息
type freqItem struct {
	value int
	count int
}

// 小根堆：频率小的排在堆顶，正好是「最该被替换」的那个
type freqHeap []freqItem

func (h freqHeap) Len() int           { return len(h) }
func (h freqHeap) Less(i, j int) bool { return h[i].count < h[j].count }
func (h freqHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *freqHeap) Push(x any)        { *h = append(*h, x.(freqItem)) }

func (h *freqHeap) Pop() any {
	old := *h
	last := len(old) - 1
	item := old[last]
	*h = old[:last]
	return item
}

func topKFrequentHeap(nums []int, k int) []int {
	count := make(map[int]int, len(nums))
	for _, v := range nums {
		count[v]++
	}

	h := &freqHeap{}
	for value, c := range count {
		heap.Push(h, freqItem{value: value, count: c})
		// 只保留 k 个候选，多出来的从堆顶（最差的）挤掉
		if h.Len() > k {
			heap.Pop(h)
		}
	}

	result := make([]int, 0, k)
	for h.Len() > 0 {
		result = append(result, heap.Pop(h).(freqItem).value)
	}
	return result
}

/*
解法二：哈希计数 + 快选（partition 剪枝）【非最优】

官方第二解就是「基于快速排序」，但它并不需要对全体排序 —— 只在 partition
之后递归包含第 k 名的那一侧，这一步剪枝把复杂度从 O(m log m) 降到期望
O(m)。严格说它不是最优：最坏情况仍是 O(m^2)，而解法三最坏也线性。

为什么慢：解法一每个不同的数都要付一次 log k；当 k 接近 m 时，这等于把
m 个元素排了一遍。

换个角度：题目只要「频次第 k 大」这条分界线。partition 一次就能把数组切成
「频次高于枢轴 / 等于枢轴 / 低于枢轴」三段，分界线落在哪一段，就决定了下一
步只需要继续处理那一段 —— 另一段根本不用再看。

为什么只递归一侧就能 O(m)：
  · 每次 partition 的代价正比于当前段长度，而下一次处理的规模是这次的一半
    左右，于是总代价是 m + m/2 + m/4 + … < 2m，是等比级数而不是 log 层
  · 反过来说，如果两侧都递归，每层的工作量是 m，层数 log m，就退化成快排
  · 这条等比级数的前提是「枢轴把数组切得大致均匀」，所以它只是期望 O(m)

为什么随机选枢轴：
  · 若枢轴固定取区间端点，那么「频次恰好有序」（例如按值升序、值越大频次
    越高）的输入每次只切掉一个元素，退化成 O(m^2)
  · 随机枢轴让坏切分的概率与输入的具体排列无关，期望切分点落在中间

关键点：
  · 必须三路划分：频次有很多重复值，两路划分遇到「整段频次都等于枢轴」
    时会原地打转，要么死循环，要么每轮只前进一格
  · 三路划分后按 k 与三段长度的关系决定去向：k 落在「高于」段就只递归该段；
    落在「等于」段说明前 k 名已经凑齐，直接停；落在「低于」段则把 k 减去
    已确定的高频个数，再递归该段
  · 命中「等于」段要立刻返回，不能继续往低处切 —— 再切就会把本该保留的
    元素也划出去
  · 用 for 循环改写尾递归：每轮只留一侧，栈深度恒为 O(1)
  · k 等于不同元素个数时一次划分都不用做，直接返回全体

以示例 1（nums = [1,1,1,2,2,3], k = 2，items = [(1,3),(2,2),(3,1)]）为例：
  取枢轴 (2,2)，三路划分后 [(1,3)] | [(2,2)] | [(3,1)]
  k = 2 落在「高于」段（长度 1）之外，且在「等于」段的右边界之内 → 命中
  取前 2 个 → [1,2] ✔

复杂度分析：
  时间 期望 O(n + m)：统计频率 O(n)，快选的期望总代价 O(m)；最坏 O(m^2)
  空间 O(m)：频率表与待划分数组；循环改写后没有递归栈
*/

// partitionByCount 三路划分 [l, r]：返回相等段的左右端点
// 返回后 items[l:above] 频次更高，items[above:below+1] 频次相等，
// items[below+1:r+1] 频次更低
func partitionByCount(items []freqItem, l, r int, rng *rand.Rand) (int, int) {
	// 随机枢轴：输入若按频次有序，固定取端点会每轮只切掉一个元素
	pivot := items[l+rng.Intn(r-l+1)].count

	above, i, below := l, l, r
	for i <= below {
		switch {
		case items[i].count > pivot:
			items[i], items[above] = items[above], items[i]
			above++
			i++
		case items[i].count < pivot:
			// 换过来的元素还没检查过，i 不能前进
			items[i], items[below] = items[below], items[i]
			below--
		default:
			i++
		}
	}
	return above, below
}

// quickSelect 把前 k 大的元素挪到 items[:k] 里；调用方保证 1 <= k <= len
func quickSelect(items []freqItem, k int, rng *rand.Rand) {
	l, r := 0, len(items)-1
	for l < r {
		above, below := partitionByCount(items, l, r, rng)
		aboveLen := above - l
		equalLen := below - above + 1

		if k <= aboveLen {
			// 答案全在「高于」段里，另一段整体丢掉
			r = above - 1
		} else if k <= aboveLen+equalLen {
			// 高频段 + 等值段已经够 k 个，前 k 名确定
			return
		} else {
			// 舍弃高于段与等值段，k 相应扣掉，只在「低于」段里继续找
			k -= aboveLen + equalLen
			l = below + 1
		}
	}
}

func topKFrequentQuickSelect(nums []int, k int) []int {
	count := make(map[int]int, len(nums))
	for _, v := range nums {
		count[v]++
	}

	items := make([]freqItem, 0, len(count))
	for value, c := range count {
		items = append(items, freqItem{value: value, count: c})
	}

	// k 等于不同元素个数时全体都是答案，不必划分
	if k < len(items) {
		// 固定种子只为让自测可复现，正式提交可以去掉这个参数
		quickSelect(items, k, rand.New(rand.NewSource(20240607)))
	}

	result := make([]int, 0, k)
	for _, it := range items[:k] {
		result = append(result, it.value)
	}
	return result
}

/*
解法三：哈希计数 + 桶排序【最优解】

为什么慢：解法二虽然把期望降到 O(m)，但最坏情况仍是 O(m^2)，而且「期望」
二字依赖于随机枢轴的运气。解法一则是稳稳的 O(m log k)。两者都在「比较
频率」，而比较排序的下界就是 O(m log m)。

换个角度：这里的键不是任意整数 —— 频率一定落在 [1, n] 之内，是一个小范围
的整数。既然频率的取值只有 n 种，就开 n + 1 个桶，把「频率为 c」的数全部
扔进 buckets[c]。从 c = n 往下一路扫，扫够 k 个数就停。每个数只被写进桶
一次、读出来一次，没有任何比较。

关键点：

	· 桶的下标是频率，桶的数量是 n + 1（频率最大就是 n，不能只开 m 个）
	· 必须从高频往低频扫，扫到收集够 k 个就立刻返回 —— 继续扫会把低频的
	  也算进来
	· 同一个桶里可能有多个数，全部都要收，但收满 k 个后要截断
	· 这是本题的最优解：最坏情况也是 O(n)，瓶颈已经变成「必须读一遍输入」
	· 代价是 O(n) 的桶空间：n 远大于不同元素个数时，比快选多吃内存

以示例 1（nums = [1,1,1,2,2,3], k = 2）为例：

	桶：buckets[3] = {1}，buckets[2] = {2}，buckets[1] = {3}
	从 3 往下扫，先收 1（第 1 个），再收 2（第 2 个），已够 k 个 → [1,2] ✔

复杂度分析：

	时间 O(n)：统计频率 O(n)，建桶与扫桶各 O(n)，桶的规模是 n + 1
	空间 O(n)：频率表 O(m)，桶数组 O(n)，两者都不超过 O(n)
*/
func topKFrequentBucket(nums []int, k int) []int {
	count := make(map[int]int, len(nums))
	for _, v := range nums {
		count[v]++
	}

	// 下标即频率：buckets[c] 装的是出现 c 次的所有数
	buckets := make([][]int, len(nums)+1)
	for value, c := range count {
		buckets[c] = append(buckets[c], value)
	}

	result := make([]int, 0, k)
	// 从高频往低频扫，收满 k 个立即停下
	for c := len(nums); c >= 1 && len(result) < k; c-- {
		for _, v := range buckets[c] {
			result = append(result, v)
			if len(result) == k {
				break
			}
		}
	}
	return result
}

// ===== 测试辅助 =====
func sortedCopy(v []int) []int {
	out := make([]int, len(v))
	copy(out, v)
	sort.Ints(out)
	return out
}

func equalInts(a, b []int) bool {
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

// 题目不要求顺序，统一排序后再比较
func check(nums []int, k int, want []int) {
	expect := sortedCopy(want)

	if got := sortedCopy(topKFrequentHeap(nums, k)); !equalInts(got, expect) {
		panic(fmt.Sprintf("heap(%v, %d) = %v, want %v", nums, k, got, expect))
	}
	quick := sortedCopy(topKFrequentQuickSelect(nums, k))
	if !equalInts(quick, expect) {
		panic(fmt.Sprintf("quickselect(%v, %d) = %v, want %v",
			nums, k, quick, expect))
	}
	if got := sortedCopy(topKFrequentBucket(nums, k)); !equalInts(got, expect) {
		panic(fmt.Sprintf("bucket(%v, %d) = %v, want %v", nums, k, got, expect))
	}
}

// 仅用于「频次两两不同」的输入：此时前 k 名的集合唯一，三个解法必须一致
func checkAgainstBucket(nums []int, k int) {
	expect := sortedCopy(topKFrequentBucket(nums, k))
	if len(expect) != k {
		panic(fmt.Sprintf("bucket size = %d, want %d", len(expect), k))
	}
	if got := sortedCopy(topKFrequentHeap(nums, k)); !equalInts(got, expect) {
		panic(fmt.Sprintf("heap size mismatch: %v vs %v", got, expect))
	}
	quick := sortedCopy(topKFrequentQuickSelect(nums, k))
	if !equalInts(quick, expect) {
		panic(fmt.Sprintf("quickselect: %v vs %v", quick, expect))
	}
}

func main() {
	// 示例 1：三种频率各不相同
	check([]int{1, 1, 1, 2, 2, 3}, 2, []int{1, 2})

	// 示例 2：只有一个元素，k 也只有一个
	check([]int{1}, 1, []int{1})

	// 边界：k 等于不同元素的个数，快选应当一次划分都不做
	check([]int{1, 1, 2, 2, 3}, 3, []int{1, 2, 3})

	// 边界：有两个数频率并列第一，答案集合仍然唯一
	check([]int{1, 1, 2, 2, 3}, 2, []int{1, 2})

	// 边界：所有元素相同，只有一个桶被用到
	check([]int{4, 4, 4, 4}, 1, []int{4})

	// 边界：k = 1 且最高频元素是负数
	check([]int{-1, -1, 2, 2, 2}, 1, []int{2})

	// 边界：频率从 1 到 4 全部不同，验证桶的扫描不会漏掉低频桶
	check([]int{1, 2, 2, 3, 3, 3, 4, 4, 4, 4}, 4, []int{1, 2, 3, 4})

	// 边界：所有元素频次完全相同，三路划分正是为了这种整段等值
	{
		nums := []int{0, 1, 2, 3, 4, 5, 6, 7}
		got := sortedCopy(topKFrequentQuickSelect(nums, 3))
		// 频次全为 1，前 3 名的集合不唯一，只要求规模、去重与取值域
		if len(got) != 3 {
			panic(fmt.Sprintf("all-equal-freq size = %d, want 3", len(got)))
		}
		for i := 1; i < len(got); i++ {
			if got[i] == got[i-1] {
				panic(fmt.Sprintf("all-equal-freq duplicate: %v", got))
			}
		}
		for _, v := range got {
			if v < 0 || v > 7 {
				panic(fmt.Sprintf("all-equal-freq out of range: %v", got))
			}
		}
	}

	// 规模：100 个不同的数，频率分别是 1..100，答案唯一
	{
		nums := []int{}
		want := []int{}
		for v := 0; v < 100; v++ {
			for t := 0; t <= v; t++ {
				nums = append(nums, v)
			}
			if v >= 50 {
				want = append(want, v)
			}
		}
		// 频次随值单调递增：固定端点枢轴会退化成 O(m^2)，这里照跑不退
		check(nums, 50, want)
	}

	// 随机用例：频次两两不同，前 k 名唯一，三解必须给出同一个集合
	{
		rng := rand.New(rand.NewSource(12345))
		for round := 0; round < 20; round++ {
			m := 200
			values := make([]int, m)
			for i := 0; i < m; i++ {
				values[i] = i
			}
			// 随机打乱「值 → 频次」的对应关系，让频次分布与大小顺序无关
			swapValues := func(i, j int) {
				values[i], values[j] = values[j], values[i]
			}
			rng.Shuffle(m, swapValues)

			nums := []int{}
			for i := 0; i < m; i++ {
				// 频次取 i + 1，两两不同，于是前 k 名的集合唯一
				for t := 0; t <= i; t++ {
					nums = append(nums, values[i])
				}
			}
			// 再打乱输入顺序，避免按值升序这种对划分特别友好的排列
			swapNums := func(i, j int) {
				nums[i], nums[j] = nums[j], nums[i]
			}
			rng.Shuffle(len(nums), swapNums)
			checkAgainstBucket(nums, 1+rng.Intn(m))
		}
	}

	fmt.Println("all tests passed")
}
