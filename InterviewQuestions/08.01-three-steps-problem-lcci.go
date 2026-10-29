/*
============================================================================
面试题 08.01. 三步问题（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/three-steps-problem-lcci/
分类：动态规划 · 递推 / 滚动数组

三步问题。有个小孩正在上楼梯，楼梯有 n 阶台阶，小孩一次可以上 1 阶、
2 阶或 3 阶。实现一种方法，计算小孩有多少种上楼梯的方式。结果可能很大，
你需要对结果模 1000000007。

示例 1：
输入：n = 3
输出：4
说明：有四种走法

示例 2：
输入：n = 5
输出：13

提示：
- n 范围在[1, 1000000]之间
*/
package main

import "fmt"

/*
解法一：自底向上 DP，保存整张表【非最优】

为什么慢：直接按 f(n) = f(n-1) + f(n-2) + f(n-3) 递归，同一层会被
反复展开，n 稍大就退化成指数级；而且递归深度等于 n，n = 10^6 时栈
根本放不下。用数组把每个 f(i) 记下来，可以消掉重复计算。

换个角度：f(i) 只依赖前面三个状态，按 i 从小到大填表，每个状态只算一次。
这正是官方提示里说的「制表法」：先算小规模，再用小规模拼出大规模。

关键点：

	· 边界取 f(0) = 1：不走也是一种走法，这样 f(1)、f(2)、f(3) 都能用
	  同一条递推式算出；题目约束 n >= 1，f(0) 只用于统一公式
	· 三项相加必须用 int64 中间值：Go 的 int 宽度随平台变化，不能假设
	  它是 64 位；三个小于 1e9+7 的数相加可能超过 32 位上限
	· 取模放在写入 f[i] 时做，保证表里每个元素都小于 MOD

以 n = 3 为例：

	f(0) = 1
	f(1) = f(0) = 1
	f(2) = f(1) + f(0) = 2
	f(3) = f(2) + f(1) + f(0) = 4  ✔

复杂度分析：

	时间 O(n)：从 1 填到 n，每个位置做常数次加法
	空间 O(n)：保存 n+1 个状态
*/
const stepMod int64 = 1000000007

func waysToStepTable(n int) int {
	// f[i]：走到第 i 阶的方案数，所有元素都保持在 [0, MOD)
	f := make([]int64, n+1)
	f[0] = 1

	for i := 1; i <= n; i++ {
		// 最后一步可能迈 1、2、3 阶，分别对应三个前驱
		ways := f[i-1]
		if i >= 2 {
			ways += f[i-2]
		}
		if i >= 3 {
			ways += f[i-3]
		}
		f[i] = ways % stepMod
	}

	return int(f[n])
}

/*
解法二：滚动数组【最优解】

为什么慢：解法一保存了 n+1 个状态，但递推只用到最近三个；更早的值
算完就再也用不到，白占了 O(n) 空间。

换个角度：用 a、b、c 三个变量分别代表 f(i-3)、f(i-2)、f(i-1)，每轮
算出 d = (a + b + c) % MOD，然后整体左移。空间从 O(n) 降到 O(1)。

关键点：

	· 必须先算 d 再移动 a、b、c；先移动会把还要用的旧值覆盖掉
	· n = 1、n = 2 要提前返回，否则 c 的初始值 f(2) 会被误当成答案
	· a + b + c 用 int64 计算，避免 32 位平台上溢出

以 n = 4 为例：

	初始 a = f(0) = 1，b = f(1) = 1，c = f(2) = 2
	d = 1 + 1 + 2 = 4 = f(3)，左移后 a = 1，b = 2，c = 4
	d = 1 + 2 + 4 = 7 = f(4)  ✔

复杂度分析：

	时间 O(n)：每个 i 做一次加法和一次取模
	空间 O(1)：只保留最近三个状态
*/
func waysToStep(n int) int {
	if n == 0 {
		return 1
	}
	if n == 1 {
		return 1
	}
	if n == 2 {
		return 2
	}

	// a、b、c 分别对应 f(i-3)、f(i-2)、f(i-1)
	a, b, c := int64(1), int64(1), int64(2)

	for i := 3; i <= n; i++ {
		d := (a + b + c) % stepMod
		a, b, c = b, c, d
	}

	return int(c)
}

// 两个解法必须给出同一个答案，避免某一条递推写错却各自自洽
func checkSteps(n int, want int) {
	if got := waysToStepTable(n); got != want {
		panic(fmt.Sprintf("table(%d) = %d, want %d", n, got, want))
	}
	if got := waysToStep(n); got != want {
		panic(fmt.Sprintf("rolling(%d) = %d, want %d", n, got, want))
	}
}

func main() {
	// 示例 1
	checkSteps(3, 4)

	// 示例 2
	checkSteps(5, 13)

	// 边界：最小台阶数
	checkSteps(1, 1)
	checkSteps(2, 2)

	// 边界：题目约束外，按 f(0) = 1 约定
	checkSteps(0, 1)

	// 反直觉：f(4) = f(3) + f(2) + f(1) = 4 + 2 + 1 = 7
	checkSteps(4, 7)

	// 手算前几项：1, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274
	checkSteps(6, 24)
	checkSteps(10, 274)

	// 边界：最大规模，两个解法必须一致，且答案落在 [0, MOD)
	large1 := waysToStepTable(1000000)
	large2 := waysToStep(1000000)
	if large1 != large2 {
		panic(fmt.Sprintf("large mismatch: %d vs %d", large1, large2))
	}
	if large1 < 0 || large1 >= 1000000007 {
		panic(fmt.Sprintf("large out of range: %d", large1))
	}

	fmt.Println("all tests passed")
}
