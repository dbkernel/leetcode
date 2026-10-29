/*
============================================================================
顺序栈（Stack）
分类：手写数据结构 · 线性结构

操作复杂度：
  push    均摊 O(1)  底层动态数组满时 2 倍扩容，总搬移小于 2n
  pop     O(1)
  peek    O(1)       只看栈顶，不弹出
  empty   O(1)
  size    O(1)
  clear   O(1)       只把 size 置 0
============================================================================

用途：
  后进先出的最简容器，也是很多算法的零件：括号匹配、表达式求值、
  单调栈（下一个更大元素）、深搜/回溯、把递归改写成迭代。

核心思想：
  只用一端进出：push 写在 n 位置，pop 读 data[n-1] 再让 n 减一。
  「栈顶」就是 n-1，没有任何额外状态；底层换成切片顺序存储，
  再套上动态数组的翻倍扩容，就得到均摊 O(1) 的 push。

关键点（不变量）：
  · 0 <= n <= len(data)，栈内元素是 data[0..n-1]，栈顶是 data[n-1]；
  · push 必须先扩容（n == len(data) 时）再写 data[n]；
  · pop 先判空、再取 data[n-1]、最后才 n--，顺序反了空栈就下溢；
  · 空栈取元素绝不能崩：C++ 用 bool + 出参、Go 用 (int, bool)、
    Python 抛 IndexError、Rust 返回 Option<i32>，四种写法是同一个语义
    「没有元素可取」，调用方必须自己处理这条失败路径。

手推一遍：
  依次 push 1、2、3：n 从 0 涨到 3，栈内容自底向上是 [1, 2, 3]。
  再 pop：取 data[2] 得到 3，n 变 2，栈内容 [1, 2]；再 pop 得 2。
  括号匹配里输入 "([)]"：读到 ')' 时栈顶是 '['，与 ')' 不配对，
  立刻返回 false，而不是等扫完再看栈空——这一步省掉了后半段的无用功。

复杂度分析：
  时间：push 均摊 O(1)（扩容代价摊到每次 push），pop/peek/empty/size 都
    是 O(1)；括号匹配扫一遍字符串 O(n)，每个字符最多进出栈一次。
  空间：O(n)，capacity 最多是 size 的 2 倍。
  稳定性：栈是 LIFO，谈不到稳定性。
*/

package main

import "fmt"

const stackInitCap = 4

type Stack struct {
	data []int
	n    int
}

func newStack() *Stack {
	return &Stack{data: make([]int, stackInitCap)}
}

func (s *Stack) size() int { return s.n }

func (s *Stack) capacity() int { return len(s.data) }

func (s *Stack) empty() bool { return s.n == 0 }

func (s *Stack) grow() {
	nd := make([]int, len(s.data)*2)
	copy(nd, s.data[:s.n])
	s.data = nd
}

func (s *Stack) push(v int) {
	if s.n == len(s.data) {
		s.grow() // 先扩容，再写入
	}
	s.data[s.n] = v
	s.n++
}

// 空栈返回 (0, false)，调用方必须看第二个返回值。
func (s *Stack) pop() (int, bool) {
	if s.n == 0 {
		return 0, false
	}
	v := s.data[s.n-1]
	s.n--
	return v, true
}

// 与 pop 的差别只有最后一句 n--。
func (s *Stack) peek() (int, bool) {
	if s.n == 0 {
		return 0, false
	}
	return s.data[s.n-1], true
}

func (s *Stack) clear() { s.n = 0 }

// 应用：括号匹配。栈里存左括号的字符编码，用 int 统一类型。
func isBalanced(s string) bool {
	st := newStack()
	for i := 0; i < len(s); i++ {
		c := s[i]
		if c == '(' || c == '[' || c == '{' {
			st.push(int(c))
		} else if c == ')' || c == ']' || c == '}' {
			top, ok := st.peek()
			if !ok {
				return false // 右括号多了
			}
			open := byte(top)
			match := (open == '(' && c == ')') ||
				(open == '[' && c == ']') ||
				(open == '{' && c == '}')
			if !match {
				return false // 类型不配对，提前失败
			}
			st.pop()
		}
	}
	return st.empty() // 左括号多了也会在这里失败
}

func check(cond bool, msg string) {
	if !cond {
		panic("assert failed: " + msg)
	}
}

func main() {
	// 用例 1：空栈的 pop / peek 返回失败而不是崩溃。
	empty := newStack()
	check(empty.empty() && empty.size() == 0, "empty init")
	if _, ok := empty.pop(); ok {
		panic("pop on empty should fail")
	}
	if _, ok := empty.peek(); ok {
		panic("peek on empty should fail")
	}

	// 用例 2：连续 push 跨多次扩容，再一路 pop 回来。
	a := newStack()
	for i := 0; i < 100; i++ {
		a.push(i)
	}
	check(a.size() == 100, "size after 100 push")
	check(a.capacity() == 128, "growth sequence") // 4,8,16,32,64,128
	check(!a.empty(), "not empty")
	top, ok := a.peek()
	check(ok && top == 99, "peek top")
	check(a.size() == 100, "peek keeps size")
	// 关键点用例：push 与 pop 交错，size 与内容必须始终一致。
	a.push(100)
	v, ok := a.pop()
	check(ok && v == 100, "pop 100")
	v, ok = a.pop()
	check(ok && v == 99, "pop 99")
	check(a.size() == 99, "size after pops")
	top, ok = a.peek()
	check(ok && top == 98, "peek 98")
	for i := 98; i >= 0; i-- {
		v, ok = a.pop()
		check(ok && v == i, "reverse order pop") // 必须是逆序弹出
	}
	check(a.empty() && a.size() == 0, "drained")
	if _, ok := a.pop(); ok {
		panic("pop on drained should fail")
	}

	// 用例 3：clear 之后还能继续用。
	a.push(1)
	a.push(2)
	a.clear()
	check(a.empty() && a.size() == 0, "after clear")
	a.push(7)
	v, ok = a.pop()
	check(ok && v == 7, "push after clear")

	// 用例 4：括号匹配的正例。
	positives := []string{"", "()", "()[]{}", "{[()]}", "((()))", "([]{}())"}
	for _, s := range positives {
		check(isBalanced(s), "should balance: "+s)
	}
	// 用例 5：括号匹配的反例——右括号过多、类型不匹配、左括号过多。
	for _, s := range []string{"(", ")", "(]", "([)]", "(){[]}("} {
		check(!isBalanced(s), "should not balance: "+s)
	}

	fmt.Println("all tests passed")
}
