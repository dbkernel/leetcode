/*
============================================================================
22. 括号生成（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/generate-parentheses/
分类：回溯 · 括号匹配与 Catalan 数

数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且
有效的 括号组合。

示例 1：
输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]

示例 2：
输入：n = 1
输出：["()"]

提示：
- 1 <= n <= 8
*/
package main

import (
	"fmt"
	"sort"
)

/*
解法一：枚举全部 2^(2n) 个括号串再校验【非最优】

为什么慢：长度为 2n 的括号串一共 2^(2n) = 4^n 个，全部生成出来再逐个用
"前缀和不能为负、总和为 0"校验，每个串又要 O(n)。其中合法串只有第 n 个
Catalan 数 C_n ≈ 4^n / (n^1.5 · √π)，也就是绝大多数枚举都是废的。

换个角度：废弃率这么高，说明"先乱生成再筛选"的方向就不对。要让剪枝发生在
生成过程中，就必须知道当前前缀是否还有救。

关键点：
  · 校验用余额：'(' 加一、')' 减一，中途为负或结尾不为 0 都不合法
  · 递归只负责决定每一位填 '(' 还是 ')'，不做任何提前判断
  · cur 用定长切片复用，避免每层复制字符串

复杂度分析：
  时间 O(n · 4^n)：4^n 个候选串，每个校验 O(n)
  空间 O(n)：递归栈深度 2n；结果集不计入
*/
func generateParenthesisBrute(n int) []string {
	res := []string{}
	cur := make([]byte, 2*n)

	var enumerate func(pos int)
	enumerate = func(pos int) {
		if pos == len(cur) {
			if validParentheses(string(cur)) {
				res = append(res, string(cur))
			}
			return
		}
		cur[pos] = '('
		enumerate(pos + 1)
		cur[pos] = ')'
		enumerate(pos + 1)
	}
	enumerate(0)
	return res
}

func validParentheses(s string) bool {
	balance := 0
	for i := 0; i < len(s); i++ {
		if s[i] == '(' {
			balance++
		} else {
			balance--
		}
		// 前缀已经为负，后面怎么补都救不回来
		if balance < 0 {
			return false
		}
	}
	return balance == 0
}

/*
解法二：回溯，只放当前还合法的括号【最优解】

为什么慢：解法一的每一层都无脑分成两支，直到最后才判断合法性。而一个括号
串只要某一刻")"比"("多，这个前缀就永远不可能补成合法串。

换个角度：把合法性条件前置到"放之前"判断，只保留还能救的分支，搜索树就
从 4^n 个叶子直接收缩到 Catalan 数个叶子，多余的节点根本不会被生成。

关键点：
  · 左括号没用满（open < n）才能放 '('
  · 右括号数量严格少于左括号（close < open）才能放 ')'
  · 这两个条件同时保证了任意前缀余额非负、且结尾恰好为 0，无需再写校验
  · 长度到 2n 即得到一个解；此时 open == close == n 必然成立

以 n = 3 为例：
  ((( → 只能补 ) → ((() → 只能补 ) → ((()) → 补 ) → ((())) ✔
  回溯到 (( 改放 ) 得 (()( → 继续得到 (()()) 与 (())() ✔

复杂度分析：
  时间 O(n · C_n)：每个节点对应一个合法前缀，叶子数就是 Catalan 数，
    每个叶子拷贝一次长度 2n 的串，即 O(n · 4^n / n^1.5)
  空间 O(n)：递归栈与当前串；结果集按题目要求不计入
*/
func generateParenthesis(n int) []string {
	res := []string{}
	cur := make([]byte, 0, 2*n)

	var dfs func(open, close int)
	dfs = func(open, close int) {
		if len(cur) == 2*n {
			res = append(res, string(cur))
			return
		}

		// 左括号没用满就还能放，放完记得撤销
		if open < n {
			cur = append(cur, '(')
			dfs(open+1, close)
			cur = cur[:len(cur)-1]
		}

		// 右括号必须严格少于左括号，这一条就是"余额非负"的等价形式
		if close < open {
			cur = append(cur, ')')
			dfs(open, close+1)
			cur = cur[:len(cur)-1]
		}
	}
	dfs(0, 0)
	return res
}

/*
解法三：按长度递归，用 Catalan 递推拼接【最优解，与解法二并列】

为什么慢：前两解都是"逐位生成"，需要显式维护左右括号的计数。若换个角度看，
任何合法串都能唯一地写成 `(A)B`，其中 A 是 i 对括号的合法串、B 是 n-1-i 对
的合法串 —— 这直接把问题拆成两个规模更小的同类子问题。

换个角度：记忆化地求出所有子规模的解，再按上面的形式拼接。第 i 项与第
n-1-i 项的所有组合乘起来，总数正好是 Catalan 数，不会多也不会漏。

关键点：
  · 枚举外层括号里包含的对数 i 从 0 到 n-1，B 的规模自动是 n-1-i
  · 必须记忆化：同一个子规模会被不同父问题反复请求，不缓存就退化成指数级
  · memo[0] 必须是 {""} 而不是空切片，否则拼接时乘不出任何结果
  · 用 nil 判断"未计算"，空切片会被误认成已缓存

复杂度分析：
  时间 O(n · C_n)：每个解的每个字符只被拼接常数次，与解法二同阶
  空间 O(n · C_n)：记忆化缓存了全部子规模的结果；sum(C_i) 与 C_n 同阶，
    因此不计结果集时比解法二多出一份同阶缓存

  取舍：两解时间同阶、空间同阶（缓存总量与输出同阶），都算最优。解法二只用
  O(n) 栈空间、可按字典序流式产出；解法三更贴近 Catalan 递推，但多一份缓存
  且字符串拼接常数更大。这里按渐近复杂度判为并列最优。
*/
func generateParenthesisByLength(n int) []string {
	memo := make([][]string, n+1)
	memo[0] = []string{""}

	var build func(k int) []string
	build = func(k int) []string {
		if memo[k] != nil {
			return memo[k]
		}

		res := []string{}
		for i := 0; i < k; i++ {
			for _, l := range build(i) {
				for _, r := range build(k - 1 - i) {
					res = append(res, "("+l+")"+r)
				}
			}
		}
		memo[k] = res
		return res
	}
	return build(n)
}

// ===== 测试辅助 =====

func sortedCopy(v []string) []string {
	out := append([]string(nil), v...)
	sort.Strings(out)
	return out
}

func equalStrings(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

// 数量、合法性、去重、三解一致，一次查完
func checkParentheses(n, want int) {
	a := sortedCopy(generateParenthesisBrute(n))
	b := sortedCopy(generateParenthesis(n))
	c := sortedCopy(generateParenthesisByLength(n))

	if len(a) != want || len(b) != want || len(c) != want {
		panic(fmt.Sprintf("n=%d 数量不对: %d/%d/%d，期望 %d",
			n, len(a), len(b), len(c), want))
	}

	for _, v := range [][]string{a, b, c} {
		for _, s := range v {
			if len(s) != 2*n || !validParentheses(s) {
				panic(fmt.Sprintf("n=%d 生成了非法串 %q", n, s))
			}
		}
	}

	if !equalStrings(a, b) || !equalStrings(b, c) {
		panic(fmt.Sprintf("n=%d 三个解法的结果集不一致", n))
	}

	// 去重：排序后相邻元素不能相等
	for i := 1; i < len(a); i++ {
		if a[i] == a[i-1] {
			panic(fmt.Sprintf("n=%d 出现了重复方案 %q", n, a[i]))
		}
	}
}

func main() {
	// 示例 1：n = 3 的 5 个解
	checkParentheses(3, 5)

	// 示例 2
	checkParentheses(1, 1)

	// 计数序列 1, 2, 5, 14, 42, 132 逐一核对
	checkParentheses(2, 2)
	checkParentheses(4, 14)
	checkParentheses(5, 42)
	checkParentheses(6, 132)

	// 约束上界：n = 8 时 Catalan 数为 1430
	checkParentheses(8, 1430)

	// 反直觉用例：n = 3 的方案必须包含"嵌套"和"并列"两种形态
	got := sortedCopy(generateParenthesis(3))
	want := []string{"((()))", "(()())", "(())()", "()(())", "()()()"}
	sort.Strings(want)
	if !equalStrings(got, want) {
		panic(fmt.Sprintf("n=3 的方案集合不对: %v", got))
	}

	fmt.Println("all tests passed")
}
