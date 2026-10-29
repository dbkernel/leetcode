/*
============================================================================
69. x 的平方根（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sqrtx/
分类：二分查找 · 整数开方 / 牛顿迭代

给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。

示例 1：
输入：x = 4
输出：2

示例 2：
输入：x = 8
输出：2
解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将
被舍去。

提示：
- 0 <= x <= 2^31 - 1
*/
package main

import (
	"fmt"
	"math"
)

/*
解法一：二分答案【非最优】

为什么慢：从 0 开始逐个试 k，判断 k * k <= x，需要 O(sqrt(x)) 次。
x 取 2^31-1 时约 4.6 万次，能过但不利用「平方随 k 单调递增」的结构。

换个角度：答案 k 满足 k^2 <= x < (k+1)^2，也就是单调函数 f(k) = k^2
上最后一个不超过 x 的位置。有序就能二分，把定位压到 O(log x)。

关键点：
  · 二分区间取闭区间 [0, x]：x >= 1 时 floor(sqrt(x)) <= x，一定覆盖答案
  · mid * mid 用 int64：x 接近 2^31-1 时 mid 可达 2^31-1，
    32 位 int 乘法会溢出
  · 判断写成 `mid * mid <= x` 才记录 ans = mid 并继续向右找，
    不能命中就返回，因为答案要的是「最大的」那个
  · 每次收缩都是 mid ± 1，区间严格变小，循环必然结束
  · x = 0 时区间 [0,0]，循环一次，ans 保持 0

以 x = 8 为例：
  [0,8] mid=4，16 > 8 → right=3
  [0,3] mid=1，1 <= 8 → ans=1, left=2
  [2,3] mid=2，4 <= 8 → ans=2, left=3
  [3,3] mid=3，9 > 8 → right=2 → 结束，返回 2 ✔

复杂度分析：
  时间 O(log x)：每轮区间减半
  空间 O(1)：只用了常数个变量
*/
func mySqrtBinary(x int) int {
	left, right, ans := 0, x, 0

	for left <= right {
		mid := left + (right-left)/2
		// mid * mid 在 32 位 int 上会溢出，先转 int64
		if int64(mid)*int64(mid) <= int64(x) {
			ans = mid
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return ans
}

/*
解法二：牛顿迭代【最优解】

为什么慢：二分每轮只把区间折半，x 接近 2^31 时要 31 轮；牛顿法利用
导数信息，每轮把误差从 e 压到 e^2 / (2 * sqrt(x)) 量级，有效位数翻倍，
通常五六轮就收敛。

换个角度：求 floor(sqrt(x)) 等价于求 f(r) = r^2 - x 的非负零点。
牛顿迭代 r' = r - f(r) / f'(r) = (r + x / r) / 2，从 r = x 出发
序列单调下降，一旦 r^2 <= x 就正好停在 floor(sqrt(x))。

关键点：
  · x = 0 必须单独返回，否则会出现 x / r 的除零
  · 除法用整数 `x / r`，全程整数运算，没有浮点误差
  · 初值取 x 而不是 x / 2 + 1，x = 1 时也能正常判断
  · 循环条件 `r * r > x`，退出时 r 即答案，不要再判断 r + 1
  · r * r 用 int64：初值 r = x 时平方最大约 4.6e18，仍在 64 位内

以 x = 8 为例：
  r=8 → (8+1)/2=4 → (4+2)/2=3 → (3+2)/2=2
  2*2 = 4 <= 8，退出 → 返回 2 ✔

以 x = 1 为例：
  r=1，1 <= 1，不进循环 → 返回 1 ✔

复杂度分析：
  时间 O(log x)：每轮误差平方级缩小，实际远快于二分
  空间 O(1)：只用了常数个变量
*/
func mySqrtNewton(x int) int {
	// r = 0 会除零，同时 0 的平方根就是 0
	if x == 0 {
		return 0
	}

	r := int64(x)

	// r 单调下降，退出时 r == floor(sqrt(x))
	for r*r > int64(x) {
		r = (r + int64(x)/r) / 2
	}

	return int(r)
}

/*
解法三：内置 exp-log【非最优】

为什么慢：log 和 exp 都是 O(1) 的库函数，时间上并不慢；但题目明确禁止
使用内置指数/对数函数，所以它不是可提交的解法。并且浮点结果在 x 接近
2^31-1 时可能差一，必须再用整数运算校正。

换个角度：sqrt(x) = exp(0.5 * log(x))。只把它当作初值，再用整数乘方把
答案夹到 [k, k+1) 的正确区间。

关键点：
  · x = 0 时 log(0) 无定义，必须先返回
  · 初值截断成整数后，向上修正 `(ans+1)^2 <= x`、向下修正 `ans^2 > x`
    都要做，误差可能为正也可能为负
  · 修正用 int64 乘方，避免 32 位溢出
  · 浮点只负责猜，最终正确性由整数比较保证

复杂度分析：
  时间 O(1)：两次库调用加常数次整数校正
  空间 O(1)：只用了常数个变量
*/
func mySqrtExpLog(x int) int {
	if x == 0 {
		return 0
	}

	ans := int(math.Exp(0.5 * math.Log(float64(x))))

	// 浮点结果可能差一，靠整数比较把 ans 夹回正确区间
	for int64(ans+1)*int64(ans+1) <= int64(x) {
		ans++
	}
	for int64(ans)*int64(ans) > int64(x) {
		ans--
	}

	return ans
}

func check(x, want int) {
	if got := mySqrtBinary(x); got != want {
		panic(fmt.Sprintf("binary(%d) = %d, want %d", x, got, want))
	}
	if got := mySqrtNewton(x); got != want {
		panic(fmt.Sprintf("newton(%d) = %d, want %d", x, got, want))
	}
	if got := mySqrtExpLog(x); got != want {
		panic(fmt.Sprintf("exp-log(%d) = %d, want %d", x, got, want))
	}
}

func main() {
	// 示例 1
	check(4, 2)

	// 示例 2
	check(8, 2)

	// 边界：最小输入
	check(0, 0)
	check(1, 1)

	// 边界：小整数
	check(2, 1)
	check(3, 1)
	check(9, 3)
	check(15, 3)
	check(16, 4)
	check(100, 10)

	// 边界：46340^2 与它前一个数，验证恰好落在平方数边界
	check(2147395600, 46340)
	check(2147395599, 46339)

	// 边界：46340^2 + 1，仍然只能取 46340
	check(2147395601, 46340)

	// 边界：2^31 - 1
	check(2147483647, 46340)

	fmt.Println("all tests passed")
}
