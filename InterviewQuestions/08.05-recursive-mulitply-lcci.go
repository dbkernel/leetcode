/*
============================================================================
面试题 08.05. 递归乘法（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/recursive-mulitply-lcci/
分类：分治 · 递归 / 位运算

递归乘法。 写一个递归函数，不使用 * 运算符， 实现两个正整数的相乘。
可以使用加号、减号、位移，但要吝啬一些。

示例 1：
输入：A = 1, B = 10
输出：10

示例 2：
输入：A = 3, B = 4
输出：12

提示：
- 保证乘法范围不会溢出
*/
package main

import "fmt"

/*
解法一：逐次累加（朴素递归）【非最优】

为什么慢：把乘法定义成「加 B 次 A」，multiply(A, B) = A + multiply(A, B-1)，
要做 B 次加法和 B 层递归。B 很大时是线性时间，递归深度也会跟着涨。

换个角度：这是最直白的定义，直接对应「不使用 * 运算符」的字面要求，适合
作为后面折半写法的对照基线。

关键点：

	· 递归基 B == 0 返回 0，否则会一直减到负数
	· 每层只加一个 A，不能漏加也不能多加
	· 题目限定正整数；A = 0 时结果自然为 0，不需要特判

以 A = 3，B = 4 为例：

	3 + multiply(3, 3)
	= 3 + 3 + multiply(3, 2)
	= 3 + 3 + 3 + multiply(3, 1)
	= 3 + 3 + 3 + 3 + 0 = 12  ✔

复杂度分析：

	时间 O(B)：每层做一次加法和一次递归
	空间 O(B)：递归栈深度等于 B
*/
func multiplyNaive(A int, B int) int {
	if B == 0 {
		return 0
	}
	return A + multiplyNaive(A, B-1)
}

/*
解法二：折半递归【非最优】

为什么慢：解法一每层只把 B 减 1，B = 10^5 时要做十万次加法。乘法的
结合律允许我们把 B 一次砍半：A * B = A * (B/2) * 2。

换个角度：先递归算出 half = multiply(A, B/2)，再让 half 自己加自己；
如果 B 是奇数，说明 B/2 向下取整少算了一份 A，最后再补一个 A。

关键点：

	· 只能递归一次，把结果存进 half 再翻倍；写成两次递归会退化成指数级
	· 奇数补 A 的条件是 B%2 == 1，不是 B/2 的奇偶
	· 递归基 B == 0 返回 0；B == 1 时 half = 0，再补 A，也正确

以 A = 3，B = 4 为例：

	multiply(3, 2) -> multiply(3, 1) -> multiply(3, 0) = 0
	回到 B = 1：0 + 0 + 3 = 3
	回到 B = 2：3 + 3 = 6
	回到 B = 4：6 + 6 = 12  ✔

复杂度分析：

	时间 O(log B)：每层把 B 减半
	空间 O(log B)：递归栈深度等于 B 的二进制位数
*/
func multiplyRecursive(A int, B int) int {
	if B == 0 {
		return 0
	}

	half := multiplyRecursive(A, B/2)
	doubled := half + half
	// B 为奇数时，B/2 向下取整少算了一份 A
	if B%2 == 0 {
		return doubled
	}
	return doubled + A
}

/*
解法三：二进制拆分（迭代）【最优解】

为什么慢：解法二时间已经是 O(log B)，但递归栈也是 O(log B)。把折半
过程改成从低位到高位的循环，可以省掉栈。

换个角度：把 B 看成二进制。B 的第 k 位为 1 时，结果里就有一份
A * 2^k。从低位扫到高位，A 每轮翻倍，正好对应 A * 2^k。

关键点：

	· 每轮先看 B 的最低位决定是否累加当前 A，再让 A 翻倍、B 右移；
	  顺序反了会错位
	· A 翻倍用 A + A，不要用 *
	· 中间量用 int64：结果本身不溢出，但最后一轮翻倍可能超过 32 位

以 A = 3，B = 4（二进制 100）为例：

	B = 4：最低位 0，A 翻倍为 6，B = 2
	B = 2：最低位 0，A 翻倍为 12，B = 1
	B = 1：最低位 1，ans += 12，A 翻倍，B = 0
	返回 12  ✔

复杂度分析：

	时间 O(log B)：循环次数等于 B 的二进制位数
	空间 O(1)：只用了答案和当前贡献两个变量
*/
func multiply(A int, B int) int {
	var ans int64
	contribute := int64(A)

	for B > 0 {
		// 当前最低位为 1，说明这一份 A * 2^k 要计入结果
		if B&1 == 1 {
			ans += contribute
		}

		B >>= 1
		// 最后一轮之后 B 已经为 0，不必再翻倍，避免无谓的溢出
		if B > 0 {
			contribute += contribute
		}
	}

	return int(ans)
}

// 三个解法都跑一遍，确保折半与位运算没有写反
func checkSmall(A int, B int, want int) {
	if got := multiplyNaive(A, B); got != want {
		panic(fmt.Sprintf("naive(%d, %d) = %d, want %d", A, B, got, want))
	}
	if got := multiplyRecursive(A, B); got != want {
		panic(fmt.Sprintf("recursive(%d, %d) = %d, want %d", A, B, got, want))
	}
	if got := multiply(A, B); got != want {
		panic(fmt.Sprintf("iterative(%d, %d) = %d, want %d", A, B, got, want))
	}
}

// B 很大时不跑朴素递归，避免不必要的深栈
func checkLarge(A int, B int, want int) {
	if got := multiplyRecursive(A, B); got != want {
		panic(fmt.Sprintf("recursive(%d, %d) = %d, want %d", A, B, got, want))
	}
	if got := multiply(A, B); got != want {
		panic(fmt.Sprintf("iterative(%d, %d) = %d, want %d", A, B, got, want))
	}
}

func main() {
	// 示例 1
	checkSmall(1, 10, 10)

	// 示例 2
	checkSmall(3, 4, 12)

	// 边界：乘数为 1
	checkSmall(1, 1, 1)
	checkSmall(12345, 1, 12345)
	checkSmall(1, 999, 999)

	// 边界：零（题目限定正整数，这里只验证健壮性）
	checkSmall(0, 5, 0)
	checkSmall(5, 0, 0)
	checkSmall(0, 0, 0)

	// 反直觉：奇数折半要补一份 A
	checkSmall(7, 9, 63)
	checkSmall(123, 456, 56088)

	// 边界：较大但仍可通过逐次累加验证的一对
	checkSmall(1000, 1000, 1000000)

	// 边界：结果接近 int 上限，验证迭代版中间翻倍不溢出
	checkLarge(50000, 40000, 2000000000)

	fmt.Println("all tests passed")
}
