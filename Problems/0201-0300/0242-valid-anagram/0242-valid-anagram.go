/*
============================================================================
242. 有效的字母异位词（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/valid-anagram/
分类：哈希表 · 计数（频次数组）

给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。

示例 1:
输入: s = "anagram", t = "nagaram"
输出: true

示例 2:
输入: s = "rat", t = "car"
输出: false

提示：
1 <= s.length, t.length <= 5 * 10^4
s 和 t 仅包含小写字母

进阶: 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？
*/
package main

import (
	"fmt"
	"sort"
	"strings"
)

/*
解法一：排序后比较【非最优】

官方方法一。t 是 s 的异位词，等价于「两个字符串排序后完全相同」。
所以先比长度（不同直接 false），再各自排序，逐位比较。

关键点：
  · 长度不同时一定是 false，先判断能省掉排序
  · 字符串在 Go 中不可变，要先把两串各自转成字节切片再排序
  · 只适合字符集不大、字符串长度可控的情况

复杂度分析：
  时间 O(n log n)：主要开销在两次排序
  空间 O(n)：两份额外的字节切片

解法二：哈希表 / 频次数组【最优解】

官方方法二。换个角度：异位词等价于「每种字符出现的次数都相同」。
因为输入只含小写字母，直接开一个长度 26 的计数数组即可，不必动用真正的哈希表。

关键点：
  · 先比长度；不同直接 false（这一步同时也保证了下面的计数不会出现「长度相等但字符
    集合不同」以外的异常）
  · 先遍历 s 累加计数，再遍历 t 逐个减；一旦某个计数减成负数，说明 t 里多出了
    s 里没有的字符，立刻返回 false
  · 长度相等 + 每个计数都不为负 ⇒ 所有计数恰好为 0，所以不必再扫一遍数组
  · 进阶（含 unicode）：把长度 26 的数组换成 map[rune]int 即可，思路完全不变

以 s = "anagram"、t = "nagaram" 为例：
  计数 a:3, n:1, g:1, r:1, m:1
  遍历 t 逐个减：n、a、g、a、r、a、m —— 每次减完都不为负
  返回 true

以 s = "rat"、t = "car" 为例：
  计数 r:1, a:1, t:1
  遍历 t = "car"：c 的计数由 0 减成 -1 → 立即返回 false

复杂度分析：
  时间 O(n)：两趟线性扫描
  空间 O(1)：固定 26 个计数（用哈希表则为 O(∣Σ∣)）
*/

// 解法一：排序后比较
func isAnagramSort(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	a := strings.Split(s, "")
	b := strings.Split(t, "")
	sort.Strings(a)
	sort.Strings(b)
	return strings.Join(a, "") == strings.Join(b, "")
}

// 解法二：频次数组（最优解）
func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	var table [26]int
	for i := 0; i < len(s); i++ {
		table[s[i]-'a']++
	}
	for i := 0; i < len(t); i++ {
		table[t[i]-'a']--
		// 减成负数说明 t 里有 s 中不存在的字符
		if table[t[i]-'a'] < 0 {
			return false
		}
	}
	// 长度相等且无负数 ⇒ 所有计数恰好归零
	return true
}

func check(s string, t string, want bool) {
	if got := isAnagram(s, t); got != want {
		panic(fmt.Sprintf("isAnagram(%q, %q) = %v, want %v", s, t, got, want))
	}
	if got := isAnagramSort(s, t); got != want {
		panic(fmt.Sprintf("isAnagramSort(%q, %q) = %v, want %v", s, t, got, want))
	}
}

func main() {
	// 示例 1：真正的异位词
	check("anagram", "nagaram", true)

	// 示例 2：字符集合不同
	check("rat", "car", false)

	// 边界：单字符相同 / 不同
	check("a", "a", true)
	check("a", "b", false)

	// 边界：长度不同直接 false
	check("ab", "abc", false)

	// 边界：字母相同但次数不同
	check("aacc", "ccac", false)

	// 边界：重复字符的异位词
	check("aabb", "bbaa", true)

	// 边界：较长字符串
	check(strings.Repeat("a", 50000), strings.Repeat("a", 50000), true)
	check(strings.Repeat("a", 49999)+"b", strings.Repeat("a", 49999)+"c", false)

	fmt.Println("all tests passed")
}
