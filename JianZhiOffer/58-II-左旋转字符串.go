/*
============================================================================
LCR 182 / 剑指 Offer 58 - II. 动态口令（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/
分类：字符串 · 左旋转（切片 / 遍历拼接 / 三次翻转）

说明：本题原名「左旋转字符串」，leetcode.cn 现已更名为「动态口令」，
函数名改为 dynamicPassword，题面意思不变。
============================================================================

某公司门禁密码使用动态口令技术。初始密码为字符串 password，密码更新均
遵循以下步骤：

设定一个正整数目标值 target
将 password 前 target 个字符按原顺序移动至字符串末尾

请返回更新后的密码字符串。

示例 1：
输入: password = "s3cur1tyC0d3", target = 4
输出: "r1tyC0d3s3cu"

示例 2：
输入: password = "lrloseumgh", target = 6
输出: "umghlrlose"

提示：
1 <= target < password.length <= 10000
*/
package main

import "fmt"

/*
解法一：字符串切片（后半段 + 前半段直接拼接）【非最优】

为什么慢：Go 的切片拼接会新建一份长度 N 的结果串，两段切片本身不额外
拷贝（共享底层数组），但结果仍是 O(N) 空间。思路最短，空间却不是最少。

换个角度：左旋转 target 位，就是把字符串切成 s1（前 target 个）和
s2（其余）再交换顺序 —— 结果就是 s2 + s1。直接拼即可。

关键点：
  · 第二段的右边界省略即到末尾：password[target:]
  · target 由题设为合法值（1 <= target < n），无需额外取模

以示例 1（password = "s3cur1tyC0d3", target = 4）为例：
  s1 = "s3cu"，s2 = "r1tyC0d3"
  拼接 s2 + s1 → "r1tyC0d3s3cu"

复杂度分析：
  时间 O(N)：两段各拷贝一次
  空间 O(N)：结果字符串
*/

// 左旋转 target 位
func dynamicPassword1(password string, target int) string {
	return password[target:] + password[:target]
}

/*
解法二：列表遍历拼接（先写后半段，再写前半段）【非最优】

为什么慢：官方原文的前提是「面试规定不允许使用切片函数」。为了不切片，
就开一个缓冲区按顺序把字符一个个搬过去，额外占 O(N) 空间。

换个角度：切片之所以快，是因为它内部也是按段拷贝。既然不能用切片，那就
手动做同一件事：先把 [target, n) 搬进缓冲区，再把 [0, target) 搬进去。
两段还能合并成一个循环 —— 从 target 开始走 n 步，用 i % n 回绕到开头。

关键点：
  · 两段循环的边界别写反：先 [target, n)，再 [0, target)
  · 用回绕写法时上界是 target + n，不是 n，否则只走了一半
  · 缓冲区预留 n 字节，避免反复扩容

以示例 2（password = "lrloseumgh", target = 6）为例：
  先搬 [6, 11) → "umghl"
  再搬 [0, 6) → "lrlose"
  结果 "umghlrlose"

复杂度分析：
  时间 O(N)：每个字符搬一次
  空间 O(N)：缓冲区
*/

// 左旋转 target 位
func dynamicPassword2(password string, target int) string {
	n := len(password)
	ans := make([]byte, 0, n)

	// 用求余把「后半段 + 前半段」合并成一个循环
	for i := target; i < target+n; i++ {
		ans = append(ans, password[i%n])
	}

	return string(ans)
}

/*
解法三：字符串遍历拼接（不用缓冲区，直接往结果串上追加）【非最优】

为什么慢：官方原文的前提是「规定不能用 join、只能用 String」。Go 的
string 不可变，循环里反复 += 会每次都新建一份，最坏退化成 O(N^2)。

换个角度：与方法二完全相同，只是把「切片缓冲」换成「字符串拼接」。
它存在的意义只在于满足语言特性受限的面试场景，不是更优的算法。

关键点：
  · 两段顺序仍是「后半段在前」；顺序反了就变成右旋转
  · 不要在两段之间插入任何分隔符
  · 在 Go 里要注意：这种写法实际比方法二慢得多，能避免就避免

以示例 2（password = "lrloseumgh", target = 6）为例：
  追加 [6, 11) → "umghl"
  追加 [0, 6) → "umghlrlose"

复杂度分析：
  时间 O(N^2)：每轮 += 都新分配并拷贝一遍（Go 的 string 不可变）
  空间 O(N)：结果字符串
*/

// 左旋转 target 位
func dynamicPassword3(password string, target int) string {
	n := len(password)
	ans := ""

	for i := target; i < n; i++ {
		ans += string(password[i])
	}
	for i := 0; i < target; i++ {
		ans += string(password[i])
	}

	return ans
}

/*
解法四：三次翻转（原地旋转）【最优解】

官方四解中空间最优的一个，也是本题的最优解：时间 O(N) 已是下界，
额外空间 O(1)（把输入转成 []byte 后原地操作）—— 前三种都要另开 O(N)。

为什么慢：三种写法都在「搬运」。可有意思的是，交换两段顺序这件事，用翻转
就能在原地完成，一次额外内存都不用申请。

换个角度：记 s = s1 s2（|s1| = target），目标是 s2 s1。
  · 整体翻转 → rev(s2) rev(s1)
  · 把前 n - target 个（即 rev(s2)）翻转 → s2 rev(s1)
  · 把后 target 个（即 rev(s1)）翻转 → s2 s1
三次翻转，结果正是左旋转。

关键点：
  · 第二次翻转的区间是 [0, n-target)，第三次是 [n-target, n) ——
    分界点是 n - target，不是 target
  · 三次翻转必须按「整体 → 前段 → 后段」的顺序；先翻某一段再翻整体，
    得到的是右旋转
  · 边界写错一格结果就全错，建议按 [left, right] 闭区间手写交换

以示例 1（password = "s3cur1tyC0d3", target = 4）为例：
  整体翻转 → "3d0Cyt1ruc3s"
  前 8 个翻转 → "r1tyC0d3uc3s"
  后 4 个翻转 → "r1tyC0d3s3cu"

复杂度分析：
  时间 O(N)：三次翻转合计扫两遍
  空间 O(1)：只在字节切片上交换（不计输入转 []byte 的那份拷贝，
                若严格要求 O(1) 且输入为 string，则这份拷贝不可避免）
*/

// 反转 b 的 [left, right] 闭区间
func reverseRange(b []byte, left, right int) {
	for left < right {
		b[left], b[right] = b[right], b[left]
		left++
		right--
	}
}

// 左旋转 target 位
func dynamicPassword4(password string, target int) string {
	n := len(password)
	b := []byte(password)
	cut := n - target

	reverseRange(b, 0, n-1)
	reverseRange(b, 0, cut-1)
	reverseRange(b, cut, n-1)

	return string(b)
}

// ---- 以下为自测代码，不属于题解主体 ----

// 一个用例喂给四种解法：四种写法必须给出同一个结果
func check(name, password string, target int, want string) {
	got1 := dynamicPassword1(password, target)
	got2 := dynamicPassword2(password, target)
	got3 := dynamicPassword3(password, target)
	got4 := dynamicPassword4(password, target)

	if got1 != want {
		panic(fmt.Sprintf("%s: slice got %q, want %q", name, got1, want))
	}
	if got2 != want {
		panic(fmt.Sprintf("%s: list got %q, want %q", name, got2, want))
	}
	if got3 != want {
		panic(fmt.Sprintf("%s: string got %q, want %q", name, got3, want))
	}
	if got4 != want {
		panic(fmt.Sprintf("%s: reverse got %q, want %q", name, got4, want))
	}
}

func main() {
	// 示例 1：target 小于一半，验证分界点在 n - target
	check("example 1", "s3cur1tyC0d3", 4, "r1tyC0d3s3cu")

	// 示例 2：target 大于一半，专防「分界点写成 target」
	check("example 2", "lrloseumgh", 6, "umghlrlose")

	// 边界：target == n - 1，等价于把最后一个字符挪到最前
	check("target near end", "abc", 2, "cab")

	// 边界：target == 1，等价于把第一个字符挪到最后
	check("target one", "abc", 1, "bca")

	// 边界：只有两个字符，四种写法的最短形式
	check("two chars", "ab", 1, "ba")

	fmt.Println("all tests passed")
}
