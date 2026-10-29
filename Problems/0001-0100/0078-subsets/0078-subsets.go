/*
============================================================================
78. 子集（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/subsets/
分类：回溯 · 幂集枚举（回溯 / 位运算）

给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集
（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。

示例 1：
输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

示例 2：
输入：nums = [0]
输出：[[],[0]]

提示：
- 1 <= nums.length <= 10
- -10 <= nums[i] <= 10
- nums 中的所有元素 互不相同
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法一：回溯，树的每个节点都收集【最优解】

为什么慢：幂集有 2^n 个子集，答案本身平均每个长度 n/2，光把它们写出去就要
Ω(2^n·n)，所以这里不存在"更快的算法"，只存在"每个子集构造得够不够省"。
朴素写法常见的翻车点是"只在叶子处收集"—— 那样只能拿到一种长度的子集。

换个角度：把"取不取 nums[i]"看成一条按下标递增的决策链。链上的每一个时刻
（包括一个都没取的时候）都对应一个合法子集，所以不必等到走完 n 层才收，
每进入一个节点就收一次。

关键点：
  · 收集动作写在 dfs 的第一行，不是写在循环之后也不是写在叶子判断里 ——
    根节点对应空集，只收叶子会丢掉所有真子集
  · 循环从 start 开始、递归传 i + 1，按下标递增取数，保证每个元素最多出现
    一次，也就保证 2^n 个子集互不相同
  · Go 的 path 是共享底层数组的，收集时必须 make + copy 一份再存，
    否则后续 pop 会把已经存进去的答案改掉

手推一遍：
以示例 1（nums = [1,2,3]）为例：
  进入根节点，收集 []
  取 1 → 收集 [1]；再取 2 → 收集 [1,2]；再取 3 → 收集 [1,2,3]
  回到 [1]，取 3 → 收集 [1,3]
  回到根，取 2 → 收集 [2]；再取 3 → [2,3]
  回到根，取 3 → 收集 [3]
  共 8 个 = 2^3 ✔

复杂度分析：
  时间 O(2^n · n)：2^n 个节点各收集一次，每次拷贝当前 path，长度最多 n
  空间 O(n)：递归栈与 path 都不超过 n 层；答案数组不计入
*/

func subsetsBacktrace(nums []int) [][]int {
	res := [][]int{}
	path := []int{}

	var dfs func(start int)
	dfs = func(start int) {
		// 进入节点就先收：当前 path 本身已经是一个合法子集
		one := make([]int, len(path))
		copy(one, path)
		res = append(res, one)

		for i := start; i < len(nums); i++ {
			path = append(path, nums[i])
			// 传 i + 1：同一层之后的数只能取更大的下标，避免重复与漏解
			dfs(i + 1)
			path = path[:len(path)-1]
		}
	}

	dfs(0)
	return res
}

/*
解法二：位运算枚举掩码 0..2^n-1【最优解，与解法一并列】

为什么慢：解法一靠递归"试探"出每个下标取或不取；而"取或不取"本身就是一位
二进制，n 个下标的状态天然可以用一个 n 位整数编码，递归带来的栈和回溯开销
在这个视角下是多余的。

换个角度：第 i 位为 1 就表示 nums[i] 在子集里。于是 [0, 2^n) 里的每个整数
唯一对应一个子集，直接从小到大枚举整数即可，既不会漏也不会重。

关键点：
  · 掩码取遍 [0, 2^n)，左端 0 对应空集，右端 2^n - 1 对应全集；
    写成 `mask <= 1<<n` 就会多枚举一个越界掩码
  · 判断第 i 位用 `mask&(1<<i)`，命中的下标顺序天然递增，
    与解法一产生的子集完全相同（顺序也一致）
  · n == 0 时 1<<0 = 1，循环只跑 mask = 0 一次，返回一个空集，
    与"幂集至少含空集"一致

手推一遍：
以示例 1（nums = [1,2,3]）为例：
  mask = 0 → []        mask = 4 → [3]
  mask = 1 → [1]       mask = 5 → [1,3]
  mask = 2 → [2]       mask = 6 → [2,3]
  mask = 3 → [1,2]     mask = 7 → [1,2,3]
  共 8 个 = 2^3 ✔

复杂度分析：
  时间 O(2^n · n)：外层 2^n 个掩码，内层扫 n 位决定每一位取不取
  空间 O(n)：只有当前子集这一个缓冲数组，递归栈为空；答案数组不计入

取舍：两条解法时间同为 O(2^n·n)、空间同为 O(n)，并列最优。解法二的 O(n)
只是一个输出缓冲、没有递归栈，深度增长时不会栈溢出；解法一不用想位运算，
但要记牢 start 下标与"进节点就收集"这两处。n <= 10 时两者实测量级一致。
*/

func subsetsByMask(nums []int) [][]int {
	n := len(nums)
	res := make([][]int, 0, 1<<n)

	for mask := 0; mask < 1<<n; mask++ {
		one := []int{}
		// 位为 1 表示取该下标，低位到高位对应 nums 从前到后
		for i := 0; i < n; i++ {
			if mask&(1<<i) != 0 {
				one = append(one, nums[i])
			}
		}
		res = append(res, one)
	}
	return res
}

// 子集集合按题面可以任意顺序返回，内外层都排序后整体比较
func normalize(v [][]int) [][]int {
	out := make([][]int, len(v))
	for i, inner := range v {
		one := make([]int, len(inner))
		copy(one, inner)
		sort.Ints(one)
		out[i] = one
	}
	sort.Slice(out, func(a, b int) bool {
		x, y := out[a], out[b]
		for i := 0; i < len(x) && i < len(y); i++ {
			if x[i] != y[i] {
				return x[i] < y[i]
			}
		}
		return len(x) < len(y)
	})
	return out
}

func sameSets(a, b [][]int) bool {
	x, y := normalize(a), normalize(b)
	if len(x) != len(y) {
		return false
	}
	for i := range x {
		if len(x[i]) != len(y[i]) {
			return false
		}
		for j := range x[i] {
			if x[i][j] != y[i][j] {
				return false
			}
		}
	}
	return true
}

// 校验幂集的三个不变量：个数 2^n、互不相同、元素总数 n·2^(n-1)
func checkInvariants(subs [][]int, n int) {
	if len(subs) != 1<<n {
		panic(fmt.Sprintf("子集个数 %d, want %d", len(subs), 1<<n))
	}

	sorted := normalize(subs)
	for i := 1; i < len(sorted); i++ {
		if len(sorted[i]) == len(sorted[i-1]) {
			same := true
			for j := range sorted[i] {
				if sorted[i][j] != sorted[i-1][j] {
					same = false
					break
				}
			}
			if same {
				panic(fmt.Sprintf("出现重复子集 %v", sorted[i]))
			}
		}
	}

	// 每个元素在各子集中出现的总次数：一半子集含它，即 2^(n-1) 次
	total := 0
	for _, one := range subs {
		total += len(one)
	}
	if n == 0 {
		if total != 0 {
			panic(fmt.Sprintf("元素总数 %d, want 0", total))
		}
	} else if total != n*(1<<(n-1)) {
		panic(fmt.Sprintf("元素总数 %d, want %d", total, n*(1<<(n-1))))
	}
}

func check(nums []int) {
	a := subsetsBacktrace(nums)
	b := subsetsByMask(nums)

	// 两个解法必须给出同一组子集，顺便交叉验证
	if !sameSets(a, b) {
		panic(fmt.Sprintf("nums = %v：回溯 %v 与掩码 %v 不一致", nums, a, b))
	}
	checkInvariants(a, len(nums))
	checkInvariants(b, len(nums))
}

func main() {
	// 示例 1：n = 3，共 8 个子集
	{
		nums := []int{1, 2, 3}
		want := [][]int{{}, {1}, {2}, {1, 2}, {3}, {1, 3}, {2, 3},
			{1, 2, 3}}
		if !sameSets(subsetsBacktrace(nums), want) {
			panic("回溯版与示例 1 不符")
		}
		if !sameSets(subsetsByMask(nums), want) {
			panic("掩码版与示例 1 不符")
		}
	}

	// 示例 2：单元素，只有空集与它自己
	{
		nums := []int{0}
		want := [][]int{{}, {0}}
		if !sameSets(subsetsBacktrace(nums), want) {
			panic("回溯版与示例 2 不符")
		}
		if !sameSets(subsetsByMask(nums), want) {
			panic("掩码版与示例 2 不符")
		}
	}

	// 边界：空数组，幂集只有一个空集（力扣约束 n >= 1，这里验证健壮性）
	{
		want := [][]int{{}}
		if !sameSets(subsetsBacktrace([]int{}), want) {
			panic("空数组：回溯版应只返回一个空集")
		}
		if !sameSets(subsetsByMask([]int{}), want) {
			panic("空数组：掩码版应只返回一个空集")
		}
		checkInvariants(subsetsByMask([]int{}), 0)
	}

	// 元素互不相同但不是 1..n：验证实现不依赖元素具体取值
	check([]int{4, 7})
	check([]int{-10, 0, 10})

	// 上限规模 n = 10：2^10 = 1024 个子集，逐一去重后仍应是 1024 个
	big := []int{}
	for i := 0; i < 10; i++ {
		big = append(big, i-5)
	}
	check(big)

	fmt.Println("all tests passed")
}
