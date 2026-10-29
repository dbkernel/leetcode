/*
============================================================================
47. 全排列 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/permutations-ii/
分类：回溯 · 排列树（同层去重）

给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。

示例 1：
输入：nums = [1,1,2]
输出：
[[1,1,2],

	[1,2,1],
	[2,1,1]]

示例 2：
输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

提示：
- 1 <= nums.length <= 8
- -10 <= nums[i] <= 10
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法一：排序 + used 标记 + 同层去重【最优解】

为什么慢：直接沿用 46 的 used 回溯，[1,1,2] 会输出 6 个排列，而不同的只有
3 个。重复来自「两个相同的 1 交换身份」—— 它们在数组里是两个不同的下标，
在任一条路径上互换两者，得到的排列完全相同。最后用集合去重能通过，但重复
的整棵子树已经被枚举过了，丢掉的是实打实的计算量。

换个角度：先排序，让相同的值挨在一起；再规定「同一层里，相同值只允许第一
个被选中」。这样每个值在每个位置只开出一条分支，重复子树在进入之前就被剪掉。

关键点（`!used[i-1]` 与 `used[i-1]` 的含义完全不同）：

	· 本题采用 `i > 0 && nums[i] == nums[i-1] && !used[i-1]`：
	  used[i-1] 为 false 说明前一个相同值「不在当前路径上」，它属于同一层，
	  在本层循环里已经被选过又回溯了；再选 nums[i] 开出的分支与它完全重复
	· 若把条件改成 `used[i-1]`（为 true），判的是「前一个相同值仍在当前路径
	  上」，即同一树枝上的重复。它同样能得到正确答案，但规则变成「相同值只
	  允许按下标从大到小的顺序进入路径」；例如 [1,2] 之后剩下的那个 1 会先被
	  放过，绕到另一条分支才补回来，多出一些走到叶子前才被剪掉的死枝。
	  实测递归节点数：[1,1,2] 是 9 对 12，[1,1,1] 是 4 对 9。两者同阶，
	  但 `!used[i-1]` 在选元素的当下就剪掉，不留死枝
	· 去重必须配合排序：不排序时相同值不相邻，`nums[i] == nums[i-1]` 判不出
	  任何东西，重复照样产生
	· 这里写 `i > 0` 而不是 `i > start`：本解法每层都从 0 扫、靠 used 过滤，
	  不存在 start，i = 0 时由短路求值跳过前一项比较
	· used 的置位与回溯顺序不能反：先置 true 再递归，回来先截断 path 再置 false

手推一遍：

	以示例 1（排序后 nums = [1,1,2]）为例：
	第 0 层选 1a → 第 1 层选 1b（used[0] 为 true，去重条件不触发）
	  → 第 2 层选 2，得到 [1,1,2]
	第 0 层选 1a → 第 1 层选 2 → 第 2 层选 1b，得到 [1,2,1]
	第 0 层 i = 1：used[0] 为 false 且两值相同 → 跳过，重复分支不进入
	第 0 层选 2 → 第 1 层选 1a → 第 2 层选 1b，得到 [2,1,1]
	第 1 层再试 i = 1 时，同层去重同样跳过，不会产出第二个 [2,1,1]
	最终恰好 3 个 ✔

复杂度分析：

	时间 O(n·n!)：最坏（元素互不相同）与 46 同阶；有重复时叶子数降到
	  n! / ∏(c_i!)，每个叶子复制长度 n 的路径
	空间 O(n)：递归深度 n，used 与 path 各 O(n)；返回结果不计入
*/
func permuteUnique(nums []int) [][]int {
	// 相同的值相邻，后面的同层去重才判得出来
	sort.Ints(nums)
	n := len(nums)
	used := make([]bool, n)
	path := make([]int, 0, n)
	ans := make([][]int, 0)

	var dfs func()
	dfs = func() {
		if len(path) == n {
			snapshot := make([]int, n)
			copy(snapshot, path)
			ans = append(ans, snapshot)
			return
		}
		for i := 0; i < n; i++ {
			if used[i] {
				continue
			}
			// 前一个相同值不在路径上 = 它在同一层已经试过并回溯，跳过
			if i > 0 && nums[i] == nums[i-1] && !used[i-1] {
				continue
			}
			used[i] = true
			path = append(path, nums[i])
			dfs()
			path = path[:len(path)-1]
			used[i] = false
		}
	}
	dfs()
	return ans
}

/*
解法二：交换法 + 集合去重【非最优】

为什么慢：46 的交换法靠「元素互不相同，所以不同 i 落位的值不同」保证分支
互斥。有重复值后这个前提没了：交换两个相同的 1，数组没有变化，同一条排列
会被不同分支重复生成。最省事的补救是把结果放进集合，但这只是「先生成再丢
弃」，重复枚举的代价一点没少。

换个角度：保留交换法的框架（第一段代码不动），把结果容器换成集合，让集合
按整个数组的值判重。正确性直接，但完全没有利用「排序后相同值相邻」的结构。

关键点：

	· 集合按元素值判重，与生成顺序无关，所以这里排不排序都不影响正确性；
	  代码里仍排一次，让两条解法的输出顺序一致，便于对照
	· 交换后递归、递归后必须换回，否则后续分支会基于被改乱的数组继续枚举
	· Go 的 map 不能直接用 []int 做键，这里用 fmt.Sprint 把切片编码成字符串；
	  元素之间带空格、两端带方括号，不同序列不会撞键
	· 非最优的根因在空间：集合要持有所有 |ans| 个排列，量级 O(n·n!)；解法一
	  只有 O(n) 的递归栈与路径

手推一遍：

	以示例 1（nums = [1,1,2]）为例：
	first = 0：i = 0 不换，[1,1,2] 产出 112、121
	           i = 1 交换两个 1，数组仍是 [1,1,2]，再产出 112、121（重复）
	           i = 2 换成 [2,1,1]，产出 211、211（重复）
	共生成 6 个候选，集合去重后剩 3 个 ✔

复杂度分析：

	时间 O(n·n!·log(n!))：枚举与拷贝 O(n·n!)，另有 n! 次集合插入，
	  每次比较 O(n)、树高 O(log(n!))
	空间 O(n·n!)：集合保存全部不同排列；递归栈 O(n) 相比之下可忽略
*/
func permuteUniqueSwap(nums []int) [][]int {
	sort.Ints(nums)
	seen := make(map[string][]int)
	ans := make([][]int, 0)

	var dfs func(first int)
	dfs = func(first int) {
		if first == len(nums) {
			key := fmt.Sprint(nums)
			if _, ok := seen[key]; !ok {
				snapshot := make([]int, len(nums))
				copy(snapshot, nums)
				seen[key] = snapshot
			}
			return
		}
		for i := first; i < len(nums); i++ {
			nums[first], nums[i] = nums[i], nums[first]
			dfs(first + 1)
			// 换回原状，父层的下一个 i 才能从同一份数组出发
			nums[first], nums[i] = nums[i], nums[first]
		}
	}
	dfs(0)
	for _, p := range seen {
		ans = append(ans, p)
	}
	return ans
}

// ===== 测试辅助 =====
func lessInts(a, b []int) bool {
	for k := 0; k < len(a) && k < len(b); k++ {
		if a[k] != b[k] {
			return a[k] < b[k]
		}
	}
	return len(a) < len(b)
}

func sortOuter(v [][]int) {
	sort.Slice(v, func(i, j int) bool { return lessInts(v[i], v[j]) })
}

func sameList(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if len(a[i]) != len(b[i]) {
			return false
		}
		for j := range a[i] {
			if a[i][j] != b[i][j] {
				return false
			}
		}
	}
	return true
}

func isPermutationOf(p, nums []int) bool {
	if len(p) != len(nums) {
		return false
	}
	a := append([]int(nil), p...)
	b := append([]int(nil), nums...)
	sort.Ints(a)
	sort.Ints(b)
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func verifyUnique(got [][]int, nums []int, wantSize int) [][]int {
	if len(got) != wantSize {
		panic(fmt.Sprintf("got %d permutations, want %d", len(got), wantSize))
	}
	for _, p := range got {
		if !isPermutationOf(p, nums) {
			panic(fmt.Sprintf("%v is not a permutation of %v", p, nums))
		}
	}
	sortOuter(got)
	for i := 1; i < len(got); i++ {
		// 相等说明同一排列被生成了两次
		if !lessInts(got[i-1], got[i]) {
			panic(fmt.Sprintf("duplicate at %d: %v", i, got[i]))
		}
	}
	return got
}

func checkUnique(nums []int, want [][]int) {
	expected := make([][]int, len(want))
	for i := range want {
		expected[i] = append([]int(nil), want[i]...)
	}
	sortOuter(expected)

	input := append([]int(nil), nums...)
	got := verifyUnique(permuteUnique(input), nums, len(want))
	if !sameList(got, expected) {
		panic(fmt.Sprintf("dedupe(%v) = %v, want %v", nums, got, expected))
	}

	got = verifyUnique(
		permuteUniqueSwap(append([]int(nil), nums...)),
		nums, len(want),
	)
	if !sameList(got, expected) {
		panic(fmt.Sprintf("swap(%v) = %v, want %v", nums, got, expected))
	}
}

func main() {
	// 示例 1：两个 1 只能换身份，不能多出排列
	checkUnique([]int{1, 1, 2}, [][]int{{1, 1, 2}, {1, 2, 1}, {2, 1, 1}})

	// 示例 2：没有重复值，退化成 46，6 个排列
	checkUnique([]int{1, 2, 3}, [][]int{
		{1, 2, 3}, {1, 3, 2}, {2, 1, 3},
		{2, 3, 1}, {3, 1, 2}, {3, 2, 1},
	})

	// 边界：全部相同，只有 1 个排列（专门验证去重不会把唯一解也剪掉）
	checkUnique([]int{1, 1, 1}, [][]int{{1, 1, 1}})

	// 边界：单元素
	checkUnique([]int{1}, [][]int{{1}})

	// 边界：负数与 0，排序后 0 夹在中间
	checkUnique([]int{0, -1, -1, 0}, [][]int{
		{-1, -1, 0, 0}, {-1, 0, -1, 0}, {-1, 0, 0, -1},
		{0, -1, -1, 0}, {0, -1, 0, -1}, {0, 0, -1, -1},
	})

	// 边界：两组重复值，排列数 4! / (2!·2!) = 6
	checkUnique([]int{2, 2, 1, 1}, [][]int{
		{1, 1, 2, 2}, {1, 2, 1, 2}, {1, 2, 2, 1},
		{2, 1, 1, 2}, {2, 1, 2, 1}, {2, 2, 1, 1},
	})

	// 边界：题目上限 n = 8，全相同，仍只有 1 个
	checkUnique([]int{-10, -10, -10, -10, -10, -10, -10, -10},
		[][]int{{-10, -10, -10, -10, -10, -10, -10, -10}})

	fmt.Println("all tests passed")
}
