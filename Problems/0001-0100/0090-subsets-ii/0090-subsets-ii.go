/*
============================================================================
90. 子集 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/subsets-ii/
分类：回溯 · 幂集去重（同层跳过 / 按键值分组）

给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的子集
（幂集）。

解集 不能 包含重复的子集。返回的解集中，子集可以按 任意顺序 排列。

示例 1：
输入：nums = [1,2,2]
输出：[[],[1],[1,2],[1,2,2],[2],[2,2]]

示例 2：
输入：nums = [0]
输出：[[],[0]]

提示：
- 1 <= nums.length <= 10
- -10 <= nums[i] <= 10
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法一：排序后回溯 + 同一层跳过相同元素【最优解】

为什么慢：直接套 78 题的回溯会产出 2^n 个子集，可重复元素让其中很多子集
长得一模一样（[1,2,2] 取第一个 2 与取第二个 2 得到同一个集合）。若先全部
生成再用 map/set 去重，就要额外付出 2^n 次排序与哈希的开销，空间也多一份。

换个角度：重复只可能发生在"同一个位置上选了不同的相同值"。把数组排好序，
相同的值就挤在一起；在某一层里，如果 nums[i] 和它左边的 nums[i-1] 相等，
那么"这个位置选 nums[i]"产生的整棵子树，和"选 nums[i-1]"时已经走过的那棵
完全一样，直接跳过即可。

关键点：
  · 跳过条件写成 `i > start && nums[i] == nums[i-1]`，判断的是"本层是否已经
    用同样的值起过一棵子树"。i > start 才是"同一层"的定义
  · 若误写成 `i > 0 && nums[i] == nums[i-1]`，就变成了跨层跳过 —— 上一层
    已经选了 2、这一层还想再选 2 时会被挡掉，[2,2] 这种合法子集反而丢了
  · 必须先排序，否则相同的值不挨着，nums[i-1] 根本代表不了"本层的上一个"
  · 每进入一个节点就收集一次，与 78 题一致，因为每个节点都是一个合法子集

手推一遍：
以示例 1（nums = [1,2,2]，排序后仍是 [1,2,2]）为例：
  根：收集 []
  取 1 → 收集 [1]；下一层从下标 1 起
    取 2（下标 1）→ 收集 [1,2]；下一层只剩下标 2 的 2 → [1,2,2]
    取 2（下标 2）：i > start 且与下标 1 的值相等 → 跳过
  回到根，取 2（下标 1）→ 收集 [2]；下一层下标 2 的 2 → [2,2]
  回到根，取 2（下标 2）：与下标 1 相等 → 跳过
  共 6 个 ✔

复杂度分析：
  时间 O(2^n · n)：最坏情况（元素全不相同）仍是 2^n 个节点，每次拷贝长度
    最多 n；有重复时节点数正好等于不重复子集个数，比 2^n 更少
  空间 O(n)：递归栈与 path
*/

func subsetsWithDupSkip(nums []int) [][]int {
	sorted := make([]int, len(nums))
	copy(sorted, nums)
	sort.Ints(sorted)

	res := [][]int{}
	path := []int{}

	var dfs func(start int)
	dfs = func(start int) {
		// 进入节点即收：当前 path 就是一个合法子集
		one := make([]int, len(path))
		copy(one, path)
		res = append(res, one)

		for i := start; i < len(sorted); i++ {
			// 同一层里重复出现的值，其整棵子树与左边那次完全重合
			if i > start && sorted[i] == sorted[i-1] {
				continue
			}
			path = append(path, sorted[i])
			dfs(i + 1)
			path = path[:len(path)-1]
		}
	}

	dfs(0)
	return res
}

/*
解法二：排序后按每个不同值的出现次数展开【最优解，与解法一并列】

为什么慢：解法一是"边试边跳过"，跳过的判断落在每个分支上；代价是每个候选
下标都要被访问一次、再比较一次。既然重复值是成组的，可以先把组算出来，
让决策直接落在组上。

换个角度：排序后把数组压成 (值, 出现次数) 的列表。对每一组只做一个决定：
这一组在子集里出现 0 次、1 次……直到 cnt 次。组与组之间互不影响，所以任何
一组决策序列唯一确定一个子集，且不同的决策序列必然给出不同的子集 ——
重复在构造阶段就不存在，也就不需要任何跳过逻辑。

关键点：
  · 分组必须基于排序后的数组，同一组的 cnt 才是连续的相同值
  · 收集放在"所有组都决定完"之后（g == len(groups)），因为这个版本不在
    中途收集 —— 中途的 path 只覆盖了前 g 组，仍会被后面的组扩展
  · 每组把 j 从 0 枚举到 cnt，j 个相同值要连续压入 j 份、结束时弹出 j 份，
    压弹次数必须一致，否则 path 会留下残余

手推一遍：
以示例 1（nums = [1,2,2]）为例，分组为 (1,1)、(2,2)：
  第 0 组取 0 个 1、第 1 组取 0 个 2 → []
  第 0 组取 0 个 1、第 1 组取 1 个 2 → [2]
  第 0 组取 0 个 1、第 1 组取 2 个 2 → [2,2]
  第 0 组取 1 个 1、第 1 组取 0 个 2 → [1]
  第 0 组取 1 个 1、第 1 组取 1 个 2 → [1,2]
  第 0 组取 1 个 1、第 1 组取 2 个 2 → [1,2,2]
  共 2 × 3 = 6 个 ✔

复杂度分析：
  时间 O(2^n · n)：设第 i 组有 cnt_i 个相同值，答案共 Π (cnt_i + 1) 个，
    不超过 2^n；每个答案要拷贝 O(n)
  空间 O(n)：递归栈深度等于组数（≤ n），path 长度不超过 n

取舍：两条解法时间同为 O(2^n·n)、空间同为 O(n)，并列最优。解法一少一次
分组、写起来更短，但去重条件必须靠 i > start 保证；解法二把"重复"从源头
消掉，不需要任何跳过条件，代价是先分组、收集只能放在叶子处。
*/

type valueGroup struct {
	val int
	cnt int
}

func subsetsWithDupGroup(nums []int) [][]int {
	sorted := make([]int, len(nums))
	copy(sorted, nums)
	sort.Ints(sorted)

	groups := []valueGroup{}
	for _, v := range sorted {
		if n := len(groups); n > 0 && groups[n-1].val == v {
			groups[n-1].cnt++
		} else {
			groups = append(groups, valueGroup{val: v, cnt: 1})
		}
	}

	res := [][]int{}
	path := []int{}

	var dfs func(g int)
	dfs = func(g int) {
		if g == len(groups) {
			// 所有组都决定完了，path 才是一个完整子集
			one := make([]int, len(path))
			copy(one, path)
			res = append(res, one)
			return
		}

		// 这一组取 0 个
		dfs(g + 1)

		// 这一组取 j 个：j 从 1 到 cnt，每层只决定数量，所以不会重复
		for j := 1; j <= groups[g].cnt; j++ {
			path = append(path, groups[g].val)
			dfs(g + 1)
		}
		// 撤销这一组压入的 j 个元素，个数必须与上面压入的一致
		path = path[:len(path)-groups[g].cnt]
	}

	dfs(0)
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

// 独立参照：枚举 2^n 个掩码，用 map 去掉重复子集
func bruteForce(nums []int) [][]int {
	seen := map[string]bool{}
	res := [][]int{}
	for mask := 0; mask < 1<<len(nums); mask++ {
		one := []int{}
		for i := 0; i < len(nums); i++ {
			if mask&(1<<i) != 0 {
				one = append(one, nums[i])
			}
		}
		sort.Ints(one)
		key := fmt.Sprint(one)
		if !seen[key] {
			seen[key] = true
			res = append(res, one)
		}
	}
	return res
}

// 不重复子集个数应等于各不同值 (出现次数 + 1) 的乘积
func expectCount(nums []int) int {
	sorted := make([]int, len(nums))
	copy(sorted, nums)
	sort.Ints(sorted)

	total := 1
	for i := 0; i < len(sorted); {
		j := i
		for j < len(sorted) && sorted[j] == sorted[i] {
			j++
		}
		total *= j - i + 1
		i = j
	}
	return total
}

func check(nums []int) {
	got1 := subsetsWithDupSkip(nums)
	got2 := subsetsWithDupGroup(nums)

	want := bruteForce(nums)
	if !sameSets(got1, want) {
		panic(fmt.Sprintf("skip(%v) = %v, want %v", nums, got1, want))
	}
	if !sameSets(got2, want) {
		panic(fmt.Sprintf("group(%v) = %v, want %v", nums, got2, want))
	}

	if e := expectCount(nums); len(got1) != e || len(got2) != e {
		panic(fmt.Sprintf("%v: 个数 %d/%d, want %d",
			nums, len(got1), len(got2), e))
	}

	// 同一解法内部也不能有重复：排序后相邻两组不应相等
	sorted1 := normalize(got1)
	for i := 1; i < len(sorted1); i++ {
		if len(sorted1[i]) == len(sorted1[i-1]) {
			same := true
			for j := range sorted1[i] {
				if sorted1[i][j] != sorted1[i-1][j] {
					same = false
					break
				}
			}
			if same {
				panic(fmt.Sprintf("%v: 出现重复子集 %v", nums, sorted1[i]))
			}
		}
	}
}

func main() {
	// 示例 1：nums = [1,2,2]，共 6 个不重复子集
	{
		nums := []int{1, 2, 2}
		want := [][]int{{}, {1}, {1, 2}, {1, 2, 2}, {2}, {2, 2}}
		if !sameSets(subsetsWithDupSkip(nums), want) {
			panic("skip 版与示例 1 不符")
		}
		if !sameSets(subsetsWithDupGroup(nums), want) {
			panic("group 版与示例 1 不符")
		}
	}

	// 示例 2：单元素
	{
		nums := []int{0}
		want := [][]int{{}, {0}}
		if !sameSets(subsetsWithDupSkip(nums), want) {
			panic("skip 版与示例 2 不符")
		}
		if !sameSets(subsetsWithDupGroup(nums), want) {
			panic("group 版与示例 2 不符")
		}
	}

	// 关键点用例：全是同一个值，必须保留 [2,2] 这类"跨层取相同值"的子集，
	// 写成 i > 0 的全局跳过会只剩 [], [2] 两个
	{
		got := normalize(subsetsWithDupSkip([]int{2, 2, 2}))
		want := normalize([][]int{{}, {2}, {2, 2}, {2, 2, 2}})
		if !sameSets(got, want) {
			panic(fmt.Sprintf("skip([2,2,2]) = %v, want %v", got, want))
		}
	}

	// 边界：完全无重复，等价于 78 题，2^3 = 8 个
	check([]int{1, 2, 3})

	// 边界：多组重复交错，验证分组不打乱
	check([]int{4, 4, 4, 4})
	check([]int{1, 1, 2, 2})
	check([]int{-1, -1, 0, 0, 0})
	check([]int{0, 0})

	// 上限规模 n = 10：全相同 → 11 个；两组各 5 个 → 36 个
	check([]int{7, 7, 7, 7, 7, 7, 7, 7, 7, 7})
	check([]int{1, 1, 1, 1, 1, 2, 2, 2, 2, 2})

	fmt.Println("all tests passed")
}
