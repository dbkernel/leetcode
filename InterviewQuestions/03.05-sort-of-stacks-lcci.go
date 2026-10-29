/*
============================================================================
面试题 03.05. 栈排序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-of-stacks-lcci/
分类：栈 · 设计（单调栈 / 一个辅助栈排序）

栈排序。 编写程序，对栈进行排序使最小元素位于栈顶。最多只能使用一个其他的临时栈
存放数据，但不得将元素复制到别的数据结构（如数组）中。该栈支持如下操作：push、
pop、peek 和 isEmpty。当栈为空时，peek 返回 -1。

示例 1：
输入：
["SortedStack", "push", "push", "peek", "pop", "peek"]
[[], [1], [2], [], [], []]
输出：
[null,null,null,1,null,2]

示例 2：
输入：
["SortedStack", "pop", "pop", "push", "pop", "isEmpty"]
[[], [], [], [1], [], []]
输出：
[null,null,null,null,null,true]

提示：
栈中的元素数目在[0, 5000]范围内。
*/
package main

import (
	"fmt"
	"reflect"
	"sort"
	"strconv"
)

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出两种标准实现（都只用一个临时栈，
不借助数组等其他数据结构），并把实践中更常用的「push 时维护有序」标为最优解。

解法一：push 时用辅助栈插入排序【最优解】

保持主栈 s1 从栈顶到栈底单调递增 —— 即栈顶永远是最小元素。于是 Pop / Peek /
IsEmpty 全都可以直接 O(1) 完成，代价压在 Push 上：
  1. 把 s1 中所有「比 val 小」的元素挪到辅助栈 s2（它们会被 val 压得更深）；
  2. 把 val 压入 s1；
  3. 再把 s2 里的元素全部搬回 s1。
搬完后 s1 依然「顶小底大」，不变量得以维持。

关键点：
  · 比较用 s1 栈顶 < val（严格小于）：相等时停下即可，相等元素谁在上都不破坏有序
  · 第 3 步必须把 s2 全部搬回，否则 s2 里还留着数据，下次 Push 会乱序
  · 主栈为空时 Peek 返回 -1，Pop 什么也不做（题面要求）
  · 全程只用两个栈，没有借助数组，符合题面限制

以 Push(1)、Push(2) 为例：
  Push(1)：s1 空 → s1 = [1]
  Push(2)：1 < 2，把 1 挪到 s2；s1 空 → 压入 2，s1 = [2]；搬回 1 → s1 = [2, 1]
           （栈顶 1 是最小值）
  Peek → 1；Pop → 弹出 1；Peek → 2

复杂度分析：
  时间：Push 最坏 O(n)（要挪动并搬回 n 个元素），Pop / Peek / IsEmpty 均 O(1)
  空间 O(n)：两个栈合计存放 n 个元素

解法二：Pop / Peek 时才整理（惰性排序）【非最优】

不变量不变（栈顶最小），但把排序推迟到真正需要读数据的时候：Push 只入栈并打上
「脏」标记；Pop / Peek 前若为脏，就用辅助栈 s2 做一次插入排序，把 s1 整理有序。

关键点：
  · 整理时先把 s1 全部倒进 s2（顺序被反转），再逐个从 s2 取出，边取边插入到 s1 的
    正确位置：比当前元素小的一律先挪回 s2，插入后再搬回来
  · 整理完 s1 有序、s2 必为空，脏标记清除
  · 排序后连续 Pop 不需要重排 —— 从有序栈顶弹元素不会破坏有序性

复杂度分析：
  时间：Push O(1)，Pop / Peek 最坏 O(n^2)（一次整理要插入 n 个元素）
  空间 O(n)
  由于 Pop / Peek 往往比 Push 调得更频繁，这种「把代价推到读操作」的做法整体更差，
  因此解法一才是最常用的写法。
*/

// 解法一：Push 时维护有序（栈顶最小）
type SortedStack struct {
	s1 []int // 主栈，栈顶是切片末尾，恒为最小
	s2 []int // 临时栈
}

func Constructor() SortedStack {
	return SortedStack{}
}

func (s *SortedStack) Push(val int) {
	// 比 val 小的元素要被压得更深，先挪到辅助栈
	for len(s.s1) > 0 && s.s1[len(s.s1)-1] < val {
		s.s2 = append(s.s2, s.s1[len(s.s1)-1])
		s.s1 = s.s1[:len(s.s1)-1]
	}
	s.s1 = append(s.s1, val)
	// 辅助栈里的元素全部搬回，维持「顶小底大」
	for len(s.s2) > 0 {
		s.s1 = append(s.s1, s.s2[len(s.s2)-1])
		s.s2 = s.s2[:len(s.s2)-1]
	}
}

func (s *SortedStack) Pop() {
	if len(s.s1) > 0 {
		s.s1 = s.s1[:len(s.s1)-1]
	}
}

func (s *SortedStack) Peek() int {
	if len(s.s1) == 0 {
		return -1
	}
	return s.s1[len(s.s1)-1]
}

func (s *SortedStack) IsEmpty() bool {
	return len(s.s1) == 0
}

// 解法二：Pop / Peek 时才整理（惰性排序）
type SortedStackLazy struct {
	s1    []int
	s2    []int
	dirty bool
}

func (s *SortedStackLazy) Push(val int) {
	s.s1 = append(s.s1, val)
	s.dirty = true
}

func (s *SortedStackLazy) Pop() {
	if len(s.s1) == 0 {
		return
	}
	s.sortIfNeeded()
	s.s1 = s.s1[:len(s.s1)-1]
}

func (s *SortedStackLazy) Peek() int {
	if len(s.s1) == 0 {
		return -1
	}
	s.sortIfNeeded()
	return s.s1[len(s.s1)-1]
}

func (s *SortedStackLazy) IsEmpty() bool {
	return len(s.s1) == 0
}

func (s *SortedStackLazy) sortIfNeeded() {
	if !s.dirty {
		return
	}
	// 先把 s1 全部倒进 s2（顺序反转）
	for len(s.s1) > 0 {
		s.s2 = append(s.s2, s.s1[len(s.s1)-1])
		s.s1 = s.s1[:len(s.s1)-1]
	}
	// 再逐个插入到 s1 的正确位置：比 t 小的元素先挪回 s2
	for len(s.s2) > 0 {
		t := s.s2[len(s.s2)-1]
		s.s2 = s.s2[:len(s.s2)-1]
		for len(s.s1) > 0 && s.s1[len(s.s1)-1] < t {
			s.s2 = append(s.s2, s.s1[len(s.s1)-1])
			s.s1 = s.s1[:len(s.s1)-1]
		}
		s.s1 = append(s.s1, t)
	}
	s.dirty = false
}

// 两套实现共用的接口
type sortedStack interface {
	Push(val int)
	Pop()
	Peek() int
	IsEmpty() bool
}

type op struct {
	name  string
	value int
}

func run(stack sortedStack, ops []op) []string {
	out := []string{}
	for _, o := range ops {
		switch o.name {
		case "push":
			stack.Push(o.value)
			out = append(out, "null")
		case "pop":
			stack.Pop()
			out = append(out, "null")
		case "peek":
			out = append(out, strconv.Itoa(stack.Peek()))
		default:
			out = append(out, strconv.FormatBool(stack.IsEmpty()))
		}
	}
	return out
}

func check(stack sortedStack, ops []op, want []string) {
	if got := run(stack, ops); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("run(%v) = %v, want %v", ops, got, want))
	}
}

// 随机交叉验证：每次 Peek / Pop 都和「排序后的切片」比对
func crossCheck(stack sortedStack) {
	expect := []int{} // 降序排列，末尾是最小值（即栈顶）
	seed := uint32(20240607)
	for i := 0; i < 3000; i++ {
		seed = seed*1103515245 + 12345
		r := (seed >> 16) % 10
		switch {
		case r < 5 || len(expect) == 0:
			value := int(seed>>8) % 20
			stack.Push(value)
			expect = append(expect, value)
			sort.Sort(sort.Reverse(sort.IntSlice(expect)))
		case r < 8:
			stack.Pop()
			expect = expect[:len(expect)-1]
		case r < 9:
			if got, want := stack.Peek(), expect[len(expect)-1]; got != want {
				panic(fmt.Sprintf("Peek() = %d, want %d", got, want))
			}
		default:
			if got := stack.IsEmpty(); got != (len(expect) == 0) {
				panic("IsEmpty() mismatch")
			}
		}
	}
}

func main() {
	// 示例 1
	ops1 := []op{{"push", 1}, {"push", 2}, {"peek", 0}, {"pop", 0}, {"peek", 0}}
	want1 := []string{"null", "null", "1", "null", "2"}
	s1 := Constructor()
	check(&s1, ops1, want1)
	check(&SortedStackLazy{}, ops1, want1)

	// 示例 2：空栈 pop 不报错，最终为空
	ops2 := []op{{"pop", 0}, {"pop", 0}, {"push", 1}, {"pop", 0}, {"isEmpty", 0}}
	want2 := []string{"null", "null", "null", "null", "true"}
	s2 := Constructor()
	check(&s2, ops2, want2)
	check(&SortedStackLazy{}, ops2, want2)

	// 边界：空栈 peek 返回 -1
	s3 := Constructor()
	check(&s3, []op{{"peek", 0}}, []string{"-1"})
	check(&SortedStackLazy{}, []op{{"peek", 0}}, []string{"-1"})

	// 边界：含重复元素
	opsDup := []op{{"push", 3}, {"push", 1}, {"push", 3}, {"push", 1},
		{"peek", 0}, {"pop", 0}, {"peek", 0}, {"pop", 0},
		{"peek", 0}, {"pop", 0}, {"peek", 0}}
	wantDup := []string{"null", "null", "null", "null", "1", "null",
		"1", "null", "3", "null", "3"}
	s4 := Constructor()
	check(&s4, opsDup, wantDup)
	check(&SortedStackLazy{}, opsDup, wantDup)

	// 边界：连续入栈的逆序数据，每次 peek 都是当前最小值
	opsDesc := []op{{"push", 5}, {"push", 4}, {"push", 3}, {"push", 2},
		{"push", 1}, {"peek", 0}, {"pop", 0}, {"peek", 0},
		{"pop", 0}, {"peek", 0}}
	wantDesc := []string{"null", "null", "null", "null", "null", "1",
		"null", "2", "null", "3"}
	s5 := Constructor()
	check(&s5, opsDesc, wantDesc)
	check(&SortedStackLazy{}, opsDesc, wantDesc)

	// 随机交叉验证
	s6 := Constructor()
	crossCheck(&s6)
	crossCheck(&SortedStackLazy{})

	fmt.Println("all tests passed")
}
