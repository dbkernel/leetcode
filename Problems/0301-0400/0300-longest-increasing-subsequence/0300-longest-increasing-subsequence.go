/*
============================================================================
300. 最长递增子序列（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/longest-increasing-subsequence/
分类：动态规划 · 序列 DP / 二分 / 树状数组

给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。

子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余
元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。

示例 1：
输入：nums = [10,9,2,5,3,7,101,18]
输出：4
解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。

示例 2：
输入：nums = [0,1,0,3,2,3]
输出：4

示例 3：
输入：nums = [7,7,7,7,7,7,7]
输出：1

提示：
- 1 <= nums.length <= 2500
- -10^4 <= nums[i] <= 10^4

进阶：
- 你能将算法的时间复杂度降低到 O(n log(n)) 吗?
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法一：钉住结尾的区间 DP【非最优】

为什么慢：直接枚举所有子序列是 2^n 量级；即便改成"逐个元素判断能不能
接上去"的搜索，同一个前缀也会被不同分支反复展开，重复计算无法复用。

换个角度：递增与否只由"上一个被选中的值"决定。把状态钉在结尾上——
f[i] 表示以 nums[i] 收尾的最长严格递增子序列长度，那么 f[i] 只依赖那些
下标更小、值也更小的 f[j]。

关键点：
  · 转移要同时满足 nums[j] < nums[i]（严格递增）与 j < i（顺序不变）
  · f 全部初始化成 1：任何单个元素自己就是长度 1 的子序列
  · 答案是 max(f)，不是 f[n-1]——最长的那条未必以末尾元素收尾
  · 只挑一个最优前驱累加，不能把所有可行的 j 都加起来

手推一遍：以示例 1（nums = [10,9,2,5,3,7,101,18]）为例：
  10 → 前面没有更小的，f = 1；9 → 同理 f = 1
  2 → f = 1；5 → 可接 2，f = 2；3 → 可接 2，f = 2
  7 → 可接 2/5/3，f = 3；101 → f = 4（2,5,7,101）
  18 → 可接 2/5/3/7，f = 4 → 答案 4 ✔

复杂度分析：
  时间 O(n^2)：每个 i 都要回看全部 j < i
  空间 O(n)：f 与输入等长
*/
func lengthOfLISDp(nums []int) int {
	n := len(nums)
	// f[i]：以 nums[i] 收尾的最长严格递增子序列长度
	f := make([]int, n)
	ans := 1

	for i := 0; i < n; i++ {
		f[i] = 1
		// 只有更小且更靠前的元素才能成为 nums[i] 的前驱
		for j := 0; j < i; j++ {
			if nums[j] < nums[i] && f[j]+1 > f[i] {
				f[i] = f[j] + 1
			}
		}
		if f[i] > ans {
			ans = f[i]
		}
	}
	return ans
}

/*
解法二：贪心 + 二分维护 tails【最优解】

为什么慢：解法一的瓶颈是"为每个 nums[i] 回看所有前驱"，而回看想知道的
其实只有一件事：在所有更短的长度里，能不能找到一个足够小的结尾接上去。

换个角度：同长度的所有递增子序列中，只有结尾最小的那条最有用——结尾越小，
后面越容易被接上。于是维护 tails[k] = 长度 k+1 的递增子序列能取到的最小
结尾。tails 天然严格递增，问题就变成在 tails 里二分定位 x。

关键点：
  · 严格递增用 lower_bound（第一个 >= x 的位置），非严格才用 upper_bound
  · 命中位置直接覆盖成 x：把该长度的尾巴压到最小
  · 只有 x 比所有尾巴都大时才追加，长度才真正 +1
  · tails 不是某条真实子序列，只能读长度，不能拿它的和或内容做别的判断
  · 返回 len(tails)；n >= 1，空数组不会被构造出来

手推一遍：以示例 1（nums = [10,9,2,5,3,7,101,18]）为例：
  10 → [10]；9 → 覆盖 10，[9]；2 → 覆盖 9，[2]
  5 → 追加，[2,5]；3 → 覆盖 5，[2,3]
  7 → 追加，[2,3,7]；101 → 追加，[2,3,7,101]
  18 → 覆盖 101，[2,3,7,18] → 长度 4 ✔

复杂度分析：
  时间 O(n log n)：每个元素一次二分，覆盖或追加是 O(1)
  空间 O(n)：tails 最长等于 LIS 长度，最坏（原数组递增）装满 n 个
*/
func lengthOfLIS(nums []int) int {
	// tails[k]：长度 k+1 的递增子序列中，能取到的最小结尾值
	tails := []int{}

	for _, x := range nums {
		// 严格递增：相等的值不能接在后面，所以取第一个 >= x 的位置
		pos := sort.SearchInts(tails, x)
		if pos == len(tails) {
			tails = append(tails, x)
		} else {
			tails[pos] = x
		}
	}
	return len(tails)
}

/*
解法三：树状数组按值域求前缀最大值【最优解，与解法二并列】

为什么慢：解法二已到 O(n log n)，但它只保留了"每个长度的最小结尾"，
丢掉了"以某个位置收尾的 LIS 长度"。若还想拿到每个 f[i]，二分法给不出来，
只能退回解法一的 O(n^2)。

换个角度：解法一的转移 f[i] = 1 + max{f[j] : j < i, nums[j] < nums[i]}
本质是一次"前缀最大值"查询。把值域离散化成排名，用树状数组维护"以某个
排名收尾的最大 f"，查询区间就是 [1, rank-1]。

关键点：
  · 必须先离散化：值域 2*10^4 与 n 无关，树状数组按排名开而不是按原值开
  · 查询区间是 rank-1，不含 rank 自己——相等值不能构成严格递增
  · 先查询后更新：同一轮里提前写入自己的 f，会把自己当成前驱
  · 树状数组节点存 max，更新写 max(tree[i], v) 而不是累加
  · 答案取所有 f[i] 的最大值，而不是 tree 的末元素

手推一遍：以 nums = [2,5,3] 为例（排名 1,3,2）：
  2 → 查询 [1,0] 得 0，f = 1，更新 tree[1] = 1
  5 → 查询 [1,2] 得 1，f = 2，更新 tree[3] = 2
  3 → 查询 [1,1] 得 1，f = 2，更新 tree[2] = 2
  → 答案 2 ✔

复杂度分析：
  时间 O(n log n)：排序去重 O(n log n)，n 次查询与 n 次更新各 O(log n)
  空间 O(n)：排名表与树状数组都以 n 为界
*/
func lengthOfLISFenwick(nums []int) int {
	// 离散化：tree 的下标是值域排名（从 1 开始），不是原始数值
	sorted := make([]int, len(nums))
	copy(sorted, nums)
	sort.Ints(sorted)
	k := 1
	for i := 1; i < len(sorted); i++ {
		if sorted[i] != sorted[k-1] {
			sorted[k] = sorted[i]
			k++
		}
	}
	sorted = sorted[:k]

	tree := make([]int, k+1)
	ans := 0

	for _, x := range nums {
		// 严格小于 x 的排名上界是 rank-1
		rank := sort.SearchInts(sorted, x) + 1

		best := 0
		for i := rank - 1; i > 0; i -= i & -i {
			if tree[i] > best {
				best = tree[i]
			}
		}

		f := best + 1
		if f > ans {
			ans = f
		}

		// 查询结束后再写入，保证不会把自己当成前驱
		for i := rank; i <= k; i += i & -i {
			if f > tree[i] {
				tree[i] = f
			}
		}
	}
	return ans
}

// 暴力参考解：枚举全部子序列，逐个检查是否严格递增
func bruteLIS(nums []int) int {
	n := len(nums)
	best := 0

	for mask := 1; mask < 1<<n; mask++ {
		last := 0
		hasLast := false
		length := 0
		ok := true

		for i := 0; i < n && ok; i++ {
			if mask>>i&1 == 0 {
				continue
			}
			// 严格递增：相等也算违规
			if hasLast && nums[i] <= last {
				ok = false
				break
			}
			last = nums[i]
			hasLast = true
			length++
		}
		if ok && length > best {
			best = length
		}
	}
	return best
}

// 三种解法与暴力解必须给出同一个答案
func check(nums []int, want int) {
	got := []int{
		lengthOfLISDp(nums),
		lengthOfLIS(nums),
		lengthOfLISFenwick(nums),
		bruteLIS(nums),
	}
	for _, g := range got {
		if g != want {
			panic(fmt.Sprintf("nums=%v got=%d want=%d", nums, g, want))
		}
	}
}

func main() {
	// 示例 1
	check([]int{10, 9, 2, 5, 3, 7, 101, 18}, 4)

	// 示例 2：递增链被 0 打断，答案靠 0,1,2,3 撑起来
	check([]int{0, 1, 0, 3, 2, 3}, 4)

	// 示例 3：全部相等，严格递增只能取一个
	check([]int{7, 7, 7, 7, 7, 7, 7}, 1)

	// 边界：完全降序，任何元素都接不上去
	check([]int{9, 8, 7, 6, 5, 4, 3, 2, 1}, 1)

	// 边界：单元素
	check([]int{4}, 1)

	// 边界：含重复值的递增，相等处必须断开
	check([]int{1, 3, 3, 4}, 3)

	// 边界：只差一个相等对，二分法若误用 upper_bound 会得 3
	check([]int{2, 2, 3}, 2)

	// 边界：完全升序，tails 一路追加
	check([]int{1, 2, 3, 4, 5, 6}, 6)

	// 边界：LIS 不以末尾元素收尾，暴露出"答案写成 f[n-1]"的错误
	check([]int{1, 2, 3, 0}, 3)

	// 随机对拍：长度 1..11、取值 -2..4，覆盖负数与重复
	seed := uint32(20260808)
	next := func(mod uint32) int {
		seed = seed*1103515245 + 12345
		return int((seed >> 16) % mod)
	}
	for t := 0; t < 200; t++ {
		n := 1 + next(11)
		nums := make([]int, n)
		for i := 0; i < n; i++ {
			nums[i] = next(7) - 2
		}
		check(nums, bruteLIS(nums))
	}

	fmt.Println("all tests passed")
}
