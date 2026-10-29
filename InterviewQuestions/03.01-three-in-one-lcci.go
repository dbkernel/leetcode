/*
============================================================================
面试题 03.01. 三合一（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/three-in-one-lcci/
分类：栈 · 设计（一个数组实现三个栈）

三合一。描述如何只用一个数组来实现三个栈。

你应该实现 push(stackNum, value)、pop(stackNum)、isEmpty(stackNum)、
peek(stackNum) 方法。stackNum 表示栈下标，value 表示压入的值。

构造函数会传入一个 stackSize 参数，代表每个栈的大小。

示例 1：
输入：
["TripleInOne", "push", "push", "pop", "pop", "pop", "isEmpty"]
[[1], [0, 1], [0, 2], [0], [0], [0], [0]]
输出：
[null, null, null, 1, -1, -1, true]
说明：当栈为空时 pop, peek 返回 -1，当栈满时 push 不压入元素。

示例 2：
输入：
["TripleInOne", "push", "push", "push", "pop", "pop", "pop", "peek"]
[[2], [0, 1], [0, 2], [0, 3], [0], [0], [0], [0]]
输出：
[null, null, null, null, 2, 1, -1, -1]

提示：
0 <= stackNum <= 2
*/
package main

import (
	"fmt"
	"reflect"
	"strconv"
)

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面按《程序员面试金典》的标准做法给出
两种实现，并把真正满足「只用一个数组」的那一种标为最优解。

解法一：三个独立数组【非最优，不满足题意】

最直白的写法就是开三个切片，每个独立管理自己的栈顶。push / pop / peek /
isEmpty 都是 O(1)，但用了三块数组，与题面「只用一个数组」的要求不符，这里只作为
对照，用来说明为什么需要「分区 + 下标换算」。

复杂度分析：
  时间：四个操作均 O(1)
  空间：总 O(3 * stackSize)，但分散在三个数组里

解法二：一个数组分三段 + 三个栈顶指针【最优解】

既然每个栈的大小固定为 stackSize，那就把长度为 3 * stackSize 的一个数组切成三段：
第 s 个栈独占 [s * stackSize, (s + 1) * stackSize) 这一段。
再维护三个「栈顶下标」top[s]，初值取 s * stackSize - 1 —— 这样偶数段时 top[s] 是
段首前一格，「top[s] >= s * stackSize」就等价于「第 s 个栈非空」。

关键点：
  · 判空用 top[s] >= s * stackSize（等价于 top[s] - s * stackSize >= 0），
    不要写成 top[s] >= 0，否则第 1、2 段会被误判为非空
  · 判满用 top[s] < (s + 1) * stackSize - 1，满了就丢弃新元素（题面要求「当栈满时
    push 不压入元素」），不要扩容
  · stackSize == 0 是合法输入：此时数组为空，push 一律被丢弃，pop / peek 一律返回 -1
  · 三个栈共用一块内存但互不越界，这就是「一个数组实现三个栈」的标准答案

以示例 2（stackSize = 2）为例，数组长 6：
  push(0, 1)：top[0] 由 -1 变 0，data[0] = 1
  push(0, 2)：top[0] 变 1，data[1] = 2（第 0 段 [0, 1] 已满）
  push(0, 3)：top[0] = 1 == (0+1)*2 - 1，已满 → 丢弃
  pop(0) → 返回 data[1] = 2，top[0] 变 0
  pop(0) → 返回 data[0] = 1，top[0] 变 -1（空）
  pop(0) → 空 → 返回 -1
  peek(0) → 空 → 返回 -1

复杂度分析：
  时间：四个操作均 O(1)
  空间 O(3 * stackSize)：一个数组 + 三个下标
*/

// 解法二：一个数组分三段 + 三个栈顶指针
type TripleInOne struct {
	data      []int
	top       [3]int
	stackSize int
}

func Constructor(stackSize int) TripleInOne {
	t := TripleInOne{data: make([]int, 3*stackSize), stackSize: stackSize}
	for s := 0; s < 3; s++ {
		t.top[s] = s*stackSize - 1 // 空栈：栈顶指向段首前一格
	}
	return t
}

func (t *TripleInOne) Push(stackNum int, value int) {
	// 满了就丢弃（题面要求），不扩容
	if t.top[stackNum] < (stackNum+1)*t.stackSize-1 {
		t.top[stackNum]++
		t.data[t.top[stackNum]] = value
	}
}

func (t *TripleInOne) Pop(stackNum int) int {
	if t.IsEmpty(stackNum) {
		return -1
	}
	value := t.data[t.top[stackNum]]
	t.top[stackNum]--
	return value
}

func (t *TripleInOne) Peek(stackNum int) int {
	if t.IsEmpty(stackNum) {
		return -1
	}
	return t.data[t.top[stackNum]]
}

func (t *TripleInOne) IsEmpty(stackNum int) bool {
	return t.top[stackNum] < stackNum*t.stackSize
}

// 解法一：三个独立切片（仅作对照，不满足「只用一个数组」的要求）
type TripleInOneThreeSlices struct {
	data      [3][]int
	stackSize int
}

func NewTripleInOneThreeSlices(stackSize int) *TripleInOneThreeSlices {
	return &TripleInOneThreeSlices{stackSize: stackSize}
}

func (t *TripleInOneThreeSlices) Push(stackNum int, value int) {
	if len(t.data[stackNum]) < t.stackSize {
		t.data[stackNum] = append(t.data[stackNum], value)
	}
}

func (t *TripleInOneThreeSlices) Pop(stackNum int) int {
	if t.IsEmpty(stackNum) {
		return -1
	}
	last := len(t.data[stackNum]) - 1
	value := t.data[stackNum][last]
	t.data[stackNum] = t.data[stackNum][:last]
	return value
}

func (t *TripleInOneThreeSlices) Peek(stackNum int) int {
	if t.IsEmpty(stackNum) {
		return -1
	}
	return t.data[stackNum][len(t.data[stackNum])-1]
}

func (t *TripleInOneThreeSlices) IsEmpty(stackNum int) bool {
	return len(t.data[stackNum]) == 0
}

// 一次操作：name 为 push/pop/peek/isEmpty，stackNum 是栈下标，value 是 push 的值
type op struct {
	name     string
	stackNum int
	value    int
}

// 两套实现共用的接口，便于同一份断言跑两种解法
type triple interface {
	Push(stackNum int, value int)
	Pop(stackNum int) int
	Peek(stackNum int) int
	IsEmpty(stackNum int) bool
}

func run(t triple, ops []op) []string {
	out := []string{}
	for _, o := range ops {
		switch o.name {
		case "push":
			t.Push(o.stackNum, o.value)
			out = append(out, "null")
		case "pop":
			out = append(out, strconv.Itoa(t.Pop(o.stackNum)))
		case "peek":
			out = append(out, strconv.Itoa(t.Peek(o.stackNum)))
		default:
			out = append(out, strconv.FormatBool(t.IsEmpty(o.stackNum)))
		}
	}
	return out
}

func check(t triple, ops []op, want []string) {
	if got := run(t, ops); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("run(%v) = %v, want %v", ops, got, want))
	}
}

// 随机交叉验证：用三个切片当对照栈
func crossCheck(t triple, stackSize int) {
	expect := [3][]int{}
	seed := uint32(20240607 + stackSize)
	for i := 0; i < 3000; i++ {
		seed = seed*1103515245 + 12345
		r := (seed >> 16) % 10
		s := int((seed >> 8) % 3)
		switch {
		case r < 5:
			value := int((seed >> 4) % 100)
			t.Push(s, value)
			// 对照栈同样按「满了丢弃」
			if len(expect[s]) < stackSize {
				expect[s] = append(expect[s], value)
			}
		case r < 8:
			want := -1
			if n := len(expect[s]); n > 0 {
				want = expect[s][n-1]
				expect[s] = expect[s][:n-1]
			}
			if got := t.Pop(s); got != want {
				panic(fmt.Sprintf("Pop(%d) = %d, want %d", s, got, want))
			}
		case r < 9:
			want := -1
			if n := len(expect[s]); n > 0 {
				want = expect[s][n-1]
			}
			if got := t.Peek(s); got != want {
				panic(fmt.Sprintf("Peek(%d) = %d, want %d", s, got, want))
			}
		default:
			if got := t.IsEmpty(s); got != (len(expect[s]) == 0) {
				panic(fmt.Sprintf("IsEmpty(%d) = %v, want %v", s, got, len(expect[s]) == 0))
			}
		}
	}
}

func main() {
	// 示例 1：stackSize = 1，push 第二个元素被丢弃
	ops1 := []op{{"push", 0, 1}, {"push", 0, 2}, {"pop", 0, 0},
		{"pop", 0, 0}, {"pop", 0, 0}, {"isEmpty", 0, 0}}
	want1 := []string{"null", "null", "1", "-1", "-1", "true"}
	t1 := Constructor(1)
	check(&t1, ops1, want1)
	check(NewTripleInOneThreeSlices(1), ops1, want1)

	// 示例 2：stackSize = 2，压三个只留两个
	ops2 := []op{{"push", 0, 1}, {"push", 0, 2}, {"push", 0, 3}, {"pop", 0, 0},
		{"pop", 0, 0}, {"pop", 0, 0}, {"peek", 0, 0}}
	want2 := []string{"null", "null", "null", "2", "1", "-1", "-1"}
	t2 := Constructor(2)
	check(&t2, ops2, want2)
	check(NewTripleInOneThreeSlices(2), ops2, want2)

	// 边界：stackSize = 0，任何 push 都丢弃、任何取数都返回 -1
	ops0 := []op{{"push", 1, 7}, {"pop", 1, 0}, {"peek", 1, 0}, {"isEmpty", 1, 0}}
	want0 := []string{"null", "-1", "-1", "true"}
	t0 := Constructor(0)
	check(&t0, ops0, want0)
	check(NewTripleInOneThreeSlices(0), ops0, want0)

	// 边界：三个栈互不干扰，各自后进先出
	opsMix := []op{{"push", 0, 1}, {"push", 1, 2}, {"push", 2, 3},
		{"push", 1, 4}, {"pop", 1, 0}, {"pop", 0, 0},
		{"pop", 2, 0}, {"pop", 1, 0}, {"pop", 0, 0}}
	wantMix := []string{"null", "null", "null", "null", "4", "1", "3", "2", "-1"}
	tMix := Constructor(2)
	check(&tMix, opsMix, wantMix)
	check(NewTripleInOneThreeSlices(2), opsMix, wantMix)

	// 随机交叉验证
	for _, stackSize := range []int{0, 1, 3, 10} {
		a := Constructor(stackSize)
		crossCheck(&a, stackSize)
		crossCheck(NewTripleInOneThreeSlices(stackSize), stackSize)
	}

	fmt.Println("all tests passed")
}
