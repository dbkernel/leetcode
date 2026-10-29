/*
============================================================================
LeetCode 344. 反转字符串（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-string/
分类：双指针 · 原地交换
============================================================================

编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 s 的
形式给出。

不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1) 的额外
空间解决这一问题。

示例 1：
输入：s = ["h","e","l","l","o"]
输出：["o","l","l","e","h"]

示例 2：
输入：s = ["H","a","n","n","a","h"]
输出：["h","a","n","n","a","H"]

提示：
1 <= s.length <= 10^5
s[i] 都是 ASCII 码表中的可打印字符
*/
package main

import (
	"fmt"
	"slices"
)

/*
解法：双指针（原地交换）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每个字符都必须被搬到
对侧，O(n) 已是下界，O(1) 额外空间也已是下界。

为什么慢：新建一个切片再从尾部倒着填，虽然也是 O(n)，但要多占 O(n) 空间，
直接违反题目「原地修改 + O(1) 额外空间」的约束。

换个角度：反转只改变「谁在哪个位置」，而位置是成对互换的 —— 反转后
s[i] 一定落在下标 n - 1 - i 上。于是让左右两个指针相向而走，每一步交换
一对字符，走到中间就换完了。

关键点：
  · 循环条件是 left < right，不是 left <= right —— 奇数长度时中间那个
    字符本来就在正确位置，交换自己既浪费又会写坏区间
  · 每轮必须同时 left++ 和 right-- —— 只动一个方向会让区间收敛不了一半

Go 的一处写法细节：
  · 切片是引用语义，函数内修改 s 会直接反映到调用方，无需返回值
  · 多重赋值 s[left], s[right] = s[right], s[left] 会先算完右侧再赋值，
    不需要额外的临时变量

以示例 1（s = ["h","e","l","l","o"]）为例：
  left = 0, right = 4：交换 h 与 o → ["o","e","l","l","h"]
  left = 1, right = 3：交换 e 与 l → ["o","l","l","e","h"]
  left = 2, right = 2：left < right 不成立，结束

复杂度分析：
  时间 O(n)：共交换 n/2 次，每次 O(1)
  空间 O(1)：只用两个下标
*/

// 原地反转，函数无返回值：调用方通过 s 本身看到结果
func reverseString(s []byte) {
	left, right := 0, len(s)-1

	// 相遇即止：偶数长度时 left 会越过 right，奇数长度时两者相等
	for left < right {
		s[left], s[right] = s[right], s[left]
		left++
		right--
	}
}

// ---- 以下为自测代码，不属于题解主体 ----

// 一个用例 = 输入 + 期望输出；slices.Clone 让每个用例互不干扰
func check(name, input, want string) {
	s := []byte(input)
	reverseString(s)
	if !slices.Equal(s, []byte(want)) {
		panic(fmt.Sprintf("%s: got %q, want %q", name, string(s), want))
	}
}

func main() {
	// 示例 1：奇数长度，验证中间字符不会被多交换一次
	check("example 1", "hello", "olleh")

	// 示例 2：大小写混合，反转不改变字符本身
	check("example 2", "Hannah", "hannaH")

	// 边界：单字符，left == right，一次交换都不能发生
	check("single char", "a", "a")

	// 边界：偶数长度，左右指针会恰好交错
	check("two chars", "ab", "ba")

	fmt.Println("all tests passed")
}
