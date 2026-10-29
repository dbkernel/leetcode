/*
============================================================================
367. 有效的完全平方数（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/valid-perfect-square/
分类：二分查找 · 整数开方 / 牛顿迭代

给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，
否则返回 false 。

完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成
某个整数和自身的乘积。

不能使用任何内置的库函数，如  sqrt 。

示例 1：
输入：num = 16
输出：true
解释：返回 true ，因为 4 * 4 = 16 且 4 是一个整数。

示例 2：
输入：num = 14
输出：false
解释：返回 false ，因为 3.74 * 3.74 = 14 且 3.74 不是一个整数。

提示：
- 1 <= num <= 2^31 - 1
*/
package main

import (
	"fmt"
	"math"
)

/*
解法一：二分答案【最优解】

为什么慢：从小到大枚举 k，判断 k * k 是否等于 num，最坏要试到
sqrt(num)。num 取 2^31-1 时约 4.6 万次，虽然不大，但完全没有利用
「平方随 k 单调递增」这个结构。

换个角度：问题等价于在单调函数 f(k) = k * k 上查找 num。f 单调递增，
于是可以二分 k 的取值区间，每次比较排除一半候选。

关键点：
  · k 的区间取闭区间 [1, num]：num >= 1 时 floor(sqrt(num)) <= num，
    区间一定覆盖答案
  · 乘法用 int64：mid 最大到 2^31-1，mid * mid 在 32 位平台上会溢出
  · sq == num 立刻返回 true；sq < num 往右，sq > num 往左
  · 循环结束仍未命中，说明 num 夹在相邻两个平方数之间，返回 false
  · num = 1 时区间 [1,1]，mid = 1，1 * 1 == 1 直接命中

以 num = 16 为例：
  [1,16] mid=8，64 > 16 → right=7
  [1,7]  mid=4，16 == 16 → true ✔

以 num = 14 为例：
  [1,14] mid=7，49 > 14 → right=6
  [1,6]  mid=3，9 < 14 → left=4
  [4,6]  mid=5，25 > 14 → right=4
  [4,4]  mid=4，16 > 14 → right=3，区间为空 → false ✔

复杂度分析：
  时间 O(log num)：每轮区间减半
  空间 O(1)：只用了常数个变量
*/
func isPerfectSquareBinary(num int) bool {
	left, right := int64(1), int64(num)

	for left <= right {
		mid := left + (right-left)/2
		// mid * mid 可能超过 32 位，统一用 int64 比较
		sq := mid * mid

		if sq == int64(num) {
			return true
		}
		if sq < int64(num) {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	// 没命中说明 num 落在两个相邻平方数之间
	return false
}

/*
解法二：牛顿迭代（与解法一并列最优）

为什么慢：二分每轮只把区间折半，num 接近 2^31 时要 31 轮；而牛顿法每轮
让误差近似平方级缩小，迭代次数远少于此。

换个角度：求平方根等价于求 f(r) = r^2 - num 的非负零点。牛顿迭代
r' = r - f(r) / f'(r) = (r + num / r) / 2，从 r = num 出发序列单调
下降，一旦 r^2 <= num 就停在 floor(sqrt(num))。

关键点：
  · 除法用整数 `num / x`，全程整数运算，没有浮点误差
  · 初值取 num 而不是 num / 2 + 1，num = 1 时不会跳过循环
  · 循环条件 x * x > num，退出时 x 就是答案候选，不必再判断 x + 1
  · x * x 用 int64：初值 x = num 时平方最大约 4.6e18，仍在 64 位内
  · 序列单调下降且每轮至少减一，必然终止

以 num = 16 为例：
  x=16 → (16+1)/2=8 → (8+2)/2=5 → (5+3)/2=4
  4*4 = 16 不大于 16，退出 → 16 == 16 → true ✔

以 num = 14 为例：
  x=14 → 7 → 4 → 3，3*3 = 9 <= 14 退出 → 9 != 14 → false ✔

复杂度分析：
  时间 O(log num)：每轮有效位数翻倍，实际迭代次数远小于二分
  空间 O(1)：只用了常数个变量
*/
func isPerfectSquareNewton(num int) bool {
	x := int64(num)

	// x 单调下降，退出时 x == floor(sqrt(num))
	for x*x > int64(num) {
		x = (x + int64(num)/x) / 2
	}

	return x*x == int64(num)
}

/*
解法三：内置 sqrt【非最优】

为什么慢：sqrt 是 O(1) 的库调用，单看时间并不慢；但题目明确禁止使用
内置库函数，所以它不是可提交的解法。此外它返回浮点数，num 接近 2^31 时
float64 的尾数仍可能让开方结果差一。

换个角度：只把 sqrt 当作「猜一个初值」，再用整数乘方把候选夹回
[k, k+1) 的正确区间，这样至少不会因浮点误差给出错误答案。

关键点：
  · 截断成整数后，向上修正 `(r+1)^2 <= num` 与向下修正 `r^2 > num`
    都要写，误差可能是正也可能是负
  · 修正循环最多跑一两次，不影响复杂度
  · 最后用整数乘法判等，不要拿浮点数比较

复杂度分析：
  时间 O(1)：sqrt 加常数次整数校正
  空间 O(1)：只用了常数个变量
*/
func isPerfectSquareSqrt(num int) bool {
	r := int64(math.Sqrt(float64(num)))

	// 浮点开方可能差一，用整数乘方把 r 夹回正确区间
	for (r+1)*(r+1) <= int64(num) {
		r++
	}
	for r*r > int64(num) {
		r--
	}

	return r*r == int64(num)
}

func check(num int, want bool) {
	if got := isPerfectSquareBinary(num); got != want {
		panic(fmt.Sprintf("binary(%d) = %v, want %v", num, got, want))
	}
	if got := isPerfectSquareNewton(num); got != want {
		panic(fmt.Sprintf("newton(%d) = %v, want %v", num, got, want))
	}
	if got := isPerfectSquareSqrt(num); got != want {
		panic(fmt.Sprintf("sqrt(%d) = %v, want %v", num, got, want))
	}
}

func main() {
	// 示例 1
	check(16, true)

	// 示例 2
	check(14, false)

	// 边界：最小输入
	check(1, true)

	// 边界：小平方数
	check(4, true)
	check(9, true)
	check(25, true)
	check(100, true)

	// 边界：夹在平方数之间
	check(2, false)
	check(3, false)
	check(15, false)
	check(24, false)

	// 边界：46340^2，int 范围内最大的完全平方数
	check(2147395600, true)

	// 边界：2^31 - 1，不是完全平方数
	check(2147483647, false)

	// 边界：最大平方数加一
	check(2147395601, false)

	// 边界：1000^2
	check(1000000, true)

	fmt.Println("all tests passed")
}
