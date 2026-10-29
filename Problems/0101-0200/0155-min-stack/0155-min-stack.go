/*
============================================================================
155. 最小栈（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/min-stack/
分类：栈 · 设计（辅助栈同步记录最小值）

设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

实现 MinStack 类:
  · MinStack() 初始化堆栈对象。
  · void push(int value) 将元素 value 推入堆栈。
  · void pop() 删除堆栈顶部的元素。
  · int top() 获取堆栈顶部的元素。
  · int getMin() 获取堆栈中的最小元素。

示例 1:
输入：
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

输出：
[null,null,null,null,-3,null,0,-2]

解释：
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.

提示：
-2^31 <= val <= 2^31 - 1
pop、top 和 getMin 操作总是在 非空栈 上调用
push, pop, top, and getMin最多被调用 3 * 10^4 次
*/
package main

import (
	"fmt"
	"math"
	"reflect"
	"strconv"
)

/*
解法：辅助栈【最优解】

官方题解只给出这一种方法，也就是本题的最优解：四个操作全部 O(1)，空间 O(n)。

为什么慢：如果每次 GetMin 都遍历一遍栈，单次就是 O(n)；若用一个变量只记「当前
最小值」，那么当这个最小值被 pop 掉之后，就再也找不回「第二小」，除非重新扫一遍。

换个角度：注意栈的先进后出性质 —— 元素 a 入栈时栈里已有 b、c、d，那么只要 a 还在栈中，
b、c、d 就一定还在。换句话说，「a 在栈中的这段期间，栈内最小值是固定的」。
于是可以让每个元素随身携带「它入栈那一刻的栈内最小值」，再把这份信息按相同的顺序
存进一个辅助栈。

关键点：
  · 辅助栈与数据栈严格同步：Push 一起 Push，Pop 一起 Pop，时刻保持一一对应
  · Push 时压入 min(当前最小值, 新元素)；新元素不比当前最小值更小，就沿用旧的最小值
  · 辅助栈预置一个 math.MaxInt32 作为哨兵，省掉第一次 Push 的判空特判
  · GetMin 直接返回辅助栈栈顶，无需任何遍历
  · 题面保证 Pop / Top / GetMin 都在非空栈上调用，所以不必额外判空

以示例（Push -2、Push 0、Push -3、GetMin、Pop、Top、GetMin）为例：
  Push(-2)：数据栈 [-2]，辅助栈 [MaxInt32, -2]
  Push(0) ：数据栈 [-2, 0]，辅助栈 [..., -2, min(-2, 0) = -2]
  Push(-3)：数据栈 [-2, 0, -3]，辅助栈 [..., -2, min(-2, -3) = -3]
  GetMin → 辅助栈栈顶 -3
  Pop    ：两栈同时弹出 → 数据栈 [-2, 0]，辅助栈 [..., -2]
  Top    → 数据栈栈顶 0
  GetMin → 辅助栈栈顶 -2

复杂度分析：
  时间：Push / Pop / Top / GetMin 均为 O(1)
  空间 O(n)：辅助栈与数据栈等长
*/
type MinStack struct {
	dataStack []int
	minStack  []int
}

func Constructor() MinStack {
	// 哨兵：让第一次 Push 不必特判空栈
	return MinStack{minStack: []int{math.MaxInt32}}
}

func (s *MinStack) Push(value int) {
	s.dataStack = append(s.dataStack, value)
	// 每个元素随身携带「入栈这一刻的栈内最小值」
	min := s.minStack[len(s.minStack)-1]
	if value < min {
		min = value
	}
	s.minStack = append(s.minStack, min)
}

func (s *MinStack) Pop() {
	s.dataStack = s.dataStack[:len(s.dataStack)-1]
	// 两个栈必须同步弹出，才能保持一一对应
	s.minStack = s.minStack[:len(s.minStack)-1]
}

func (s *MinStack) Top() int {
	return s.dataStack[len(s.dataStack)-1]
}

func (s *MinStack) GetMin() int {
	return s.minStack[len(s.minStack)-1]
}

/**
 * Your MinStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(val);
 * obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.GetMin();
 */
type op struct {
	name  string
	value int
}

func run(ops []op) []string {
	obj := Constructor()
	out := []string{}
	for _, o := range ops {
		switch o.name {
		case "push":
			obj.Push(o.value)
			out = append(out, "null")
		case "pop":
			obj.Pop()
			out = append(out, "null")
		case "top":
			out = append(out, strconv.Itoa(obj.Top()))
		default:
			out = append(out, strconv.Itoa(obj.GetMin()))
		}
	}
	return out
}

func check(ops []op, want []string) {
	if got := run(ops); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("run(%v) = %v, want %v", ops, got, want))
	}
}

// 随机交叉验证：用切片当对照，GetMin 每次线性求最小
func crossCheck() {
	minStack := Constructor()
	expect := []int{}
	seed := uint32(20240607)
	for i := 0; i < 3000; i++ {
		seed = seed*1103515245 + 12345
		r := (seed >> 16) % 10
		switch {
		case r < 5 || len(expect) == 0:
			value := int((seed>>8)%200) - 100
			minStack.Push(value)
			expect = append(expect, value)
		case r < 7:
			minStack.Pop()
			expect = expect[:len(expect)-1]
		case r < 9:
			if got, want := minStack.Top(), expect[len(expect)-1]; got != want {
				panic(fmt.Sprintf("Top() = %d, want %d", got, want))
			}
		default:
			want := expect[0]
			for _, v := range expect {
				if v < want {
					want = v
				}
			}
			if got := minStack.GetMin(); got != want {
				panic(fmt.Sprintf("GetMin() = %d, want %d", got, want))
			}
		}
	}
}

func main() {
	// 示例 1
	check([]op{{"push", -2}, {"push", 0}, {"push", -3}, {"getMin", 0},
		{"pop", 0}, {"top", 0}, {"getMin", 0}},
		[]string{"null", "null", "null", "-3", "null", "0", "-2"})

	// 边界：单元素，Top 与 GetMin 相同
	check([]op{{"push", 7}, {"top", 0}, {"getMin", 0}}, []string{"null", "7", "7"})

	// 边界：重复最小值，Pop 一个后最小值仍应不变
	check([]op{{"push", 1}, {"push", 1}, {"getMin", 0}, {"pop", 0}, {"getMin", 0}},
		[]string{"null", "null", "1", "null", "1"})

	// 边界：最小值被弹出后要能恢复到第二小
	check([]op{{"push", 3}, {"push", 1}, {"push", 2}, {"getMin", 0},
		{"pop", 0}, {"pop", 0}, {"getMin", 0}},
		[]string{"null", "null", "null", "1", "null", "null", "3"})

	// 边界：极端值
	check([]op{{"push", math.MaxInt32}, {"push", math.MinInt32}, {"getMin", 0},
		{"pop", 0}, {"getMin", 0}},
		[]string{"null", "null", strconv.Itoa(math.MinInt32), "null", strconv.Itoa(math.MaxInt32)})

	// 随机交叉验证
	crossCheck()

	fmt.Println("all tests passed")
}
