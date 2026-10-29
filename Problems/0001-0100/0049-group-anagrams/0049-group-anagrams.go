/*
============================================================================
49. 字母异位词分组（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/group-anagrams/
分类：哈希表 · 排序键 / 计数键

给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回
结果列表。

示例 1:
输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
解释：
- 在 strs 中没有字符串可以通过重新排列来形成 "bat"。
- 字符串 "nat" 和 "tan" 是字母异位词，因为它们可以重新排列以形成彼此。
- 字符串 "ate" 、"eat" 和 "tea" 是字母异位词，因为它们可以重新排列
  以形成彼此。

示例 2:
输入: strs = [""]
输出: [[""]]

示例 3:
输入: strs = ["a"]
输出: [["a"]]

提示：
- 1 <= strs.length <= 10^4
- 0 <= strs[i].length <= 100
- strs[i] 仅包含小写字母
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法一：排序后的字符串作为键【非最优】

为什么慢：互为字母异位词的字符串排序后完全相同，所以把排序结果当键
可以正确分组。但每个字符串都要排序，长度 k 的排序是 O(k log k)；
而"是不是同一组"只取决于每个字母出现多少次，与顺序无关，排序做了
多余的工作。

换个角度：用长度为 26 的计数数组描述"每种字母出现几次"，它才是字母
异位词真正的等价类标识。计数一次线性扫描即可，不需要排序。

关键点：
  · 键必须能唯一标识一组异位词：排序串和计数数组都满足
  · 排序键直接复用 string 作为哈希键，写法最短
  · 计数键在 Go 里是 [26]int，数组可直接作为 map 的键
  · 哈希冲突只影响性能，不影响正确性，相等判断仍逐元素比较

以示例 1 中的 "eat"、"tea"、"ate" 为例：
  排序后都是 "aet"，落进同一个桶 ✔
  "tan"、"nat" 排序后都是 "ant"，落进另一个桶 ✔

复杂度分析：
  时间 O(n k log k)：n 个字符串，每个排序 O(k log k)
  空间 O(n k)：哈希表保存所有字符串

解法二：计数数组作为键（最优解）

对每个字符串统计 26 个字母的出现次数，把计数数组整体作为键。
  · 每个字符串 O(k) 统计，总时间 O(n(k+26)) = O(nk)
  · 空间仍是 O(nk)，但时间比排序键少一个 log k
  · 不要用"26 个质数相乘"当键：前 26 个质数的乘积远超 64 位整数，
    会溢出；即使改用大整数，代价也比计数数组高

关键点：
  · 计数数组按字母顺序固定长度 26，空字符串得到全 0 数组，同样能分组
  · Go 的 [26]int 是可比较类型，可以直接作为 map 键，无需自定义哈希
  · 遍历字符串时用下标取 byte，s[i]-'a' 落在 0..25
  · 结果顺序任意，最后只需把 map 的值收集成切片

以示例 1（strs = ["eat","tea","tan","ate","nat","bat"]）为例：
  "eat"/"tea"/"ate" 的计数都是 a=1,e=1,t=1 → 同一组
  "tan"/"nat" 的计数都是 a=1,n=1,t=1 → 同一组
  "bat" 是 a=1,b=1,t=1 → 单独一组 ✔

复杂度分析：
  时间 O(n(k+26)) = O(nk)：每个字符串线性统计
  空间 O(nk)：哈希表保存所有字符串；每个键额外 26 个整数
*/
func groupAnagramsBySort(strs []string) [][]string {
	groups := map[string][]string{}
	for _, s := range strs {
		key := []byte(s)
		sort.Slice(key, func(i, j int) bool { return key[i] < key[j] })
		k := string(key)
		groups[k] = append(groups[k], s)
	}

	ans := make([][]string, 0, len(groups))
	for _, g := range groups {
		ans = append(ans, g)
	}
	return ans
}

// 平台入口：计数数组作键，时间 O(nk)
func groupAnagrams(strs []string) [][]string {
	groups := map[[26]int][]string{}
	for _, s := range strs {
		var counts [26]int
		for i := 0; i < len(s); i++ {
			counts[s[i]-'a']++
		}
		groups[counts] = append(groups[counts], s)
	}

	ans := make([][]string, 0, len(groups))
	for _, g := range groups {
		ans = append(ans, g)
	}
	return ans
}

// 题面允许任意顺序：组内排序、组间排序后再比较
func normalizeGroups(groups [][]string) [][]string {
	for _, g := range groups {
		sort.Strings(g)
	}
	sort.Slice(groups, func(i, j int) bool {
		a, b := groups[i], groups[j]
		for k := 0; k < len(a) && k < len(b); k++ {
			if a[k] != b[k] {
				return a[k] < b[k]
			}
		}
		return len(a) < len(b)
	})
	return groups
}

func sameGroups(a, b [][]string) bool {
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

func checkGroups(strs []string, want [][]string) {
	expected := normalizeGroups(want)

	got1 := normalizeGroups(groupAnagramsBySort(strs))
	if !sameGroups(got1, expected) {
		panic(fmt.Sprintf("sort(%v) = %v, want %v", strs, got1, expected))
	}

	got2 := normalizeGroups(groupAnagrams(strs))
	if !sameGroups(got2, expected) {
		panic(fmt.Sprintf("count(%v) = %v, want %v", strs, got2, expected))
	}
}

func main() {
	// 示例 1
	checkGroups([]string{"eat", "tea", "tan", "ate", "nat", "bat"},
		[][]string{{"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}})

	// 示例 2
	checkGroups([]string{""}, [][]string{{""}})

	// 示例 3
	checkGroups([]string{"a"}, [][]string{{"a"}})

	// 边界：两个空串属于同一组
	checkGroups([]string{"", ""}, [][]string{{"", ""}})

	// 边界：空串与非空串不能混组
	checkGroups([]string{"", "a"}, [][]string{{""}, {"a"}})

	// 反直觉：顺序完全不同，但排序键一致
	checkGroups([]string{"abc", "bca", "cab", "xyz", "zyx", "foo"},
		[][]string{{"abc", "bca", "cab"}, {"xyz", "zyx"}, {"foo"}})

	// 反直觉：两组异位词交错出现
	checkGroups([]string{"aab", "aba", "baa", "abb", "bab", "bba"},
		[][]string{{"aab", "aba", "baa"}, {"abb", "bab", "bba"}})

	// 边界：完全相同的字符串也要全部保留
	checkGroups([]string{"dddddddddddddddddddd", "dddddddddddddddddddd"},
		[][]string{{"dddddddddddddddddddd", "dddddddddddddddddddd"}})

	// 边界：长度不同的异位词（前缀相同也不是同组）
	checkGroups([]string{"ab", "ba", "abc", "cba", "bac"},
		[][]string{{"ab", "ba"}, {"abc", "cba", "bac"}})

	fmt.Println("all tests passed")
}
