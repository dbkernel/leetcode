/*
============================================================================
面试题 16.01. 交换数字（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/swap-numbers-lcci/
分类：位运算 · 原地交换 / 异或与加减

编写一个函数，不用临时变量，直接交换 numbers = [a, b] 中 a 与 b 的值。

示例：
输入: numbers = [1,2]
输出: [2,1]

提示：
- numbers.length == 2
- -2147483647 <= numbers[i] <= 2147483647
*/
package main

import "fmt"

/*
解法一：异或交换【最优解】

为什么慢：用临时变量交换当然最快，但题目禁止；而任何「先把一个值搬到别处
再搬回来」的写法（临时数组、辅助函数传值）都只是把临时变量藏起来，并没有
真正省下空间。

换个角度：异或自带可逆性 —— 同一个值异或两次等于没异或。用第一个槽位当
「草稿纸」，先让它记住 a^b，再用它反推出 a 和 b 分别写回去，就不需要额外
的变量。

关键点：
  · 三步顺序固定：a ^= b; b ^= a; a ^= b
  · 第二步读到的 a 已经是 a^b，所以 b 拿到 (a^b)^b = a，这是交换成立的关键
  · 必须作用在两个不同的槽位：若用下标 i、j 且 i == j，第一步就把该槽清零，
    后两步全是 0，原值彻底丢失
  · 两个槽位存相同的值没关系，异或照样交换（结果还是那个值）
  · 异或逐位进行，不受数值范围限制，没有溢出问题

以示例（numbers = [1,2]）为例：
  a = 1^2 = 3
  b = 3^2 = 1
  a = 3^1 = 2
  结果 [2,1] ✔

复杂度分析：
  时间 O(1)：三次异或
  空间 O(1)：没有申请任何变量，只用两个槽位本身

解法二：加减交换【最优解，与解法一并列】

为什么慢：思路与异或同源，成本也一样是常数次运算；真正的问题不在时间，
而在值域 —— 提示允许 |numbers[i]| 达到 2147483647，两数相加最大可达
2^32-2，32 位有符号整数放不下。

换个角度：加法与减法互为逆运算，先用 a 记住 a+b，再从中减掉 b 得到 a，
从和中减掉 a 得到 b，同样只需要槽位本身，不需要额外变量。

关键点：
  · 三步顺序固定：a = a + b; b = a - b; a = a - b
  · 第二步用的 a 是更新后的和，b = (a+b) - b = a 才对
  · 第三步的 numbers[0] 仍是原来的 a（还没被覆盖），减出来才是 b
  · 中间和必须用更宽的类型承接，不能让 32 位整数溢出
  · 加减法对「两个下标相同」是安全的（a = 2a 再减回来仍是 a），这一点与
    异或版相反
  · 乘法/除法版不能用：除以 0 会崩，乘法同样溢出

以示例（numbers = [1,2]）为例：
  sum = 3
  numbers[1] = 3 - 2 = 1
  numbers[0] = 3 - 1 = 2
  结果 [2,1] ✔

复杂度分析：
  时间 O(1)：两次减法加一次加法
  空间 O(1)：一个累加器，与输入规模无关

取舍：两者都是 O(1) 时间、O(1) 空间，都算最优解。异或版不受数值范围影响，
任何 32 位值都能安全交换；加减版更直观，但需要更宽的类型承接中间和。工程
上优先异或，除非可读性要求更高且已确认不会溢出。
*/

// 解法一：异或交换
func swapNumbers(numbers []int) []int {
	// 三步都作用在两个不同下标上，i == j 时会把该槽清零
	numbers[0] ^= numbers[1]
	numbers[1] ^= numbers[0]
	numbers[0] ^= numbers[1]
	return numbers
}

// 解法二：加减交换
func swapNumbersArith(numbers []int) []int {
	// 和用 64 位承接：两个 2^31-1 相加超出 32 位有符号整数的表示范围
	sum := int64(numbers[0]) + int64(numbers[1])

	numbers[1] = int(sum - int64(numbers[1])) // 原 numbers[0]
	numbers[0] = int(sum - int64(numbers[0])) // 原 numbers[1]
	return numbers
}

// 两个解法各拿一份原始输入，避免相互污染
func check(numbers []int, wantA, wantB int) {
	a := append([]int(nil), numbers...)
	b := append([]int(nil), numbers...)

	gotA := swapNumbers(a)
	if len(gotA) != 2 || gotA[0] != wantA || gotA[1] != wantB {
		panic(fmt.Sprintf("xor(%v) = %v, want [%d %d]", numbers, gotA, wantA, wantB))
	}

	gotB := swapNumbersArith(b)
	if len(gotB) != 2 || gotB[0] != wantA || gotB[1] != wantB {
		panic(fmt.Sprintf("arith(%v) = %v, want [%d %d]", numbers, gotB, wantA, wantB))
	}
}

func main() {
	// 示例
	check([]int{1, 2}, 2, 1)

	// 指定用例：两个值相差较大
	check([]int{1, 10}, 10, 1)

	// 边界：两个值相同，异或版交换后必须还是原值
	check([]int{5, 5}, 5, 5)

	// 边界：都取提示允许的最大正数，加减版的和会顶到 2^32 量级
	check([]int{2147483647, 2147483647}, 2147483647, 2147483647)

	// 边界：一正一负的极值，中间和与差都越过 32 位范围
	check([]int{-2147483647, 2147483647}, 2147483647, -2147483647)

	// 边界：含 0 与负数
	check([]int{0, -2147483647}, -2147483647, 0)
	check([]int{-1, 0}, 0, -1)

	// 边界：负数之间交换
	check([]int{-3, -8}, -8, -3)

	fmt.Println("all tests passed")
}
