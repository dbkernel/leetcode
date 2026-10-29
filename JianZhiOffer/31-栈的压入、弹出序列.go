/*
============================================================================
LCR 148 / 剑指 Offer 31. 验证图书取出顺序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zhan-de-ya-ru-dan-chu-xu-lie-lcof/
分类：栈 · 模拟（贪心验证出栈序列）

现在图书馆有一堆图书需要放入书架，并且图书馆的书架是一种特殊的数据结构，只能按照
一定 的顺序 放入 和 拿取 书籍。

给定一个表示图书放入顺序的整数序列 putIn，请判断序列 takeOut 是否为按照正确的顺序
拿取书籍的操作序列。你可以假设放入书架的所有书籍编号都不相同。

示例 1：
输入：putIn = [6,7,8,9,10,11], takeOut = [9,11,10,8,7,6]
输出：true
解释：我们可以按以下操作放入并拿取书籍：
push(6), push(7), push(8), push(9), pop() -> 9,
push(10), push(11),pop() -> 11,pop() -> 10, pop() -> 8, pop() -> 7, pop() -> 6

示例 2：
输入：putIn = [6,7,8,9,10,11], takeOut = [11,9,8,10,6,7]
输出：false
解释：6 不能在 7 之前取出。

提示：
0 <= putIn.length == takeOut.length <= 1000
0 <= putIn[i], takeOut < 1000
putIn 是 takeOut 的排列。

注意：本题与主站 946 题相同：https://leetcode.cn/problems/validate-stack-sequences/
*/
package main

import "fmt"

/*
解法：辅助栈模拟【最优解】

本题在 leetcode.cn 上没有官方题解文章（它与主站 946「验证栈序列」是同一道题），
下面给出标准做法：时间 O(n)、空间 O(n)，且不修改输入，是本题的通用最优解。

为什么慢：如果去枚举「所有可能的入栈/出栈操作序列」，数量是卡特兰数级别，指数爆炸。
而直接对 takeOut 的每个元素去 putIn 里找位置也不行 —— 只看出栈序列本身，很难判断
某个元素到底是「还没入栈」还是「已经压在栈里」。

换个角度：既然题目保证 putIn 是 takeOut 的排列、且编号互不相同，那就不必枚举，直接
照着 takeOut 的要求去模拟：
  按 putIn 的顺序逐个入栈，每压入一个元素后，只要栈顶正好等于「下一个该取出的元素」
  就立刻弹出，能弹多少弹多少。全程模拟完后，若栈为空，说明这个取书顺序合法。

关键点：
  · 用 j 指向 takeOut 中「下一个待取出的元素」，弹出条件就是栈顶 == takeOut[j]
  · 内层循环必须「弹到不能再弹」为止：一次入栈可能连续满足多次取书
  · 顺序不能颠倒 —— 必须先压入再判断能否弹出（takeOut 的第一个元素也要能取到）
  · 末尾只需判断栈是否为空 / j 是否走到 n，两者等价
  · 正确性直觉：这是贪心 —— 只要栈顶是想要的，现在弹出绝不会让结果变差，因为再压
    入的元素只会把它埋得更深

以示例 1（putIn = [6..11]，takeOut = [9,11,10,8,7,6]）为例：
  压 6，栈顶 6 ≠ 9；压 7；压 8；压 9 → 栈顶 9 == takeOut[0] → 弹出 9，j = 1
  压 10；压 11 → 栈顶 11 == takeOut[1] → 弹出 11，j = 2 → 栈顶 10 == takeOut[2] → 弹出 10
  → 栈顶 8 == takeOut[3] → 弹出 8 → 栈顶 7 → 弹出 → 栈顶 6 → 弹出
  栈空 → 返回 true

以示例 2（takeOut = [11,9,8,10,6,7]）为例：
  压入 6..11 后逐个弹出 11、9、8、10，此时栈里只剩 [6, 7] 且栈顶 7；
  但 takeOut 接下来要 6，而 6 被 7 压住了 → 栈顶 7 ≠ 6，无法弹出
  最终栈非空 → 返回 false

复杂度分析：
  时间 O(n)：每个元素最多入栈一次、出栈一次
  空间 O(n)：辅助栈最坏存下全部元素
*/
func validateBookSequences(putIn []int, takeOut []int) bool {
	st := []int{}
	j := 0 // 指向 takeOut 中下一个待取出的元素
	n := len(putIn)

	for i := 0; i < n; i++ {
		// 先按放入顺序压栈
		st = append(st, putIn[i])
		// 只要栈顶正好是下一个要取出的书，就连续取出
		for len(st) > 0 && j < n && st[len(st)-1] == takeOut[j] {
			st = st[:len(st)-1]
			j++
		}
	}

	// 全部模拟完毕，栈应为空
	return len(st) == 0
}

func check(putIn []int, takeOut []int, want bool) {
	if got := validateBookSequences(putIn, takeOut); got != want {
		panic(fmt.Sprintf("validateBookSequences(%v, %v) = %v, want %v", putIn, takeOut, got, want))
	}
}

func main() {
	// 示例 1：合法的取书顺序
	check([]int{6, 7, 8, 9, 10, 11}, []int{9, 11, 10, 8, 7, 6}, true)

	// 示例 2：6 被 7 压住，无法先取出
	check([]int{6, 7, 8, 9, 10, 11}, []int{11, 9, 8, 10, 6, 7}, false)

	// 边界：空序列（长度可以为 0）
	check([]int{}, []int{}, true)

	// 边界：单元素，怎么取都合法
	check([]int{1}, []int{1}, true)

	// 边界：全部压完再逆序取出，合法
	check([]int{1, 2, 3, 4, 5}, []int{5, 4, 3, 2, 1}, true)

	// 边界：压一个取一个，正序取出也合法
	check([]int{1, 2, 3, 4, 5}, []int{1, 2, 3, 4, 5}, true)

	// 边界：交叉的合法 / 非法序列
	check([]int{1, 2, 3, 4, 5}, []int{4, 5, 3, 2, 1}, true)
	check([]int{1, 2, 3, 4, 5}, []int{4, 3, 5, 1, 2}, false)

	// 边界：图示里常见的 3 元素用例
	check([]int{1, 2, 3}, []int{3, 1, 2}, false)

	fmt.Println("all tests passed")
}
