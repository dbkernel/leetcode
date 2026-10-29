/*
============================================================================
双端队列（Deque）
分类：手写数据结构 · 线性结构

操作复杂度：
  pushFront   均摊 O(1)  满时 2 倍扩容并重排下标
  pushBack    均摊 O(1)  同循环队列的入队
  popFront    O(1)       只挪 head
  popBack     O(1)       只改 size，队尾由公式算出
  front       O(1)
  back        O(1)
  empty/size  O(1)
  clear       O(1)
============================================================================

用途：
  两端都要进出的场景：滑动窗口最值（单调队列）、0-1 BFS、回文判断、
  撤销栈、以及工作窃取调度器的本地队列。

核心思想：
  循环队列再加一个「往左长」的能力。队尾仍然是 (head + size - 1)，
  队头插就是把 head 往左挪一格再写；挪的时候要 (head - 1 + capacity)
  再取模，否则下标会变成 -1。于是两端增删都只动下标，不搬元素。

关键点（不变量）：
  · 队首物理下标是 head，队尾物理下标是 (head + size - 1) % capacity；
  · pushFront 必须「先左移 head，再写 data[head]」；popFront 必须
    「先读 data[head]，再右移 head」，两步顺序反了就会写错/读错位置；
  · 头部的下标运算一律写成 (head - 1 + capacity) % capacity。Go 的 %
    对负数返回负余数，直接写 (head-1)%capacity 会拿到负下标并 panic；
  · 扩容时按逻辑顺序把 head..head+size-1 搬到新切片 0..size-1，最后才把
    head 置 0；容量翻倍后旧的取模关系全部失效。

手推一遍：
  容量 4，pushBack 1、2、3、4 后 head=0、size=4，队满。
  1. popFront 取走 1，head 变 1，size=3，逻辑内容 [2, 3, 4]；
  2. pushFront(0)：head = (1-1+4)%4 = 0，写 data[0]=0，size 回到 4，
     逻辑内容 [0, 2, 3, 4]，物理上下标 0..3 全占满；
  3. 再 pushBack(5)：size==capacity 先扩容到 8，按逻辑顺序搬成
     [0, 2, 3, 4] 占下标 0..3、head 归 0，再写 data[4]=5。

复杂度分析：
  时间：两端插入/删除都是均摊 O(1)；单次触发扩容是 O(n)，但一次扩容至少
    带来 capacity 次入队，摊下来仍是 O(1)。
  空间：O(n)，capacity 最多是 size 的 2 倍，且总是 2 的幂。
  取舍：相对数组的优势是两端 O(1)（数组头插是 O(n)），代价是随机访问
    变成一次取模。
*/

package main

import "fmt"

const dequeInitCap = 4

type Deque struct {
	data []int
	h    int // 队首物理下标
	n    int // 元素个数
}

func newDeque() *Deque {
	return &Deque{data: make([]int, dequeInitCap)}
}

func (q *Deque) size() int { return q.n }

func (q *Deque) capacity() int { return len(q.data) }

// 只为自测观察回绕，不参与逻辑。
func (q *Deque) head() int { return q.h }

func (q *Deque) empty() bool { return q.n == 0 }

func (q *Deque) full() bool { return q.n == len(q.data) }

func (q *Deque) grow() {
	nd := make([]int, len(q.data)*2)
	// 用旧 q.h、旧 capacity 按逻辑顺序搬；搬完才能改动这两个量。
	for i := 0; i < q.n; i++ {
		nd[i] = q.data[(q.h+i)%len(q.data)]
	}
	q.data = nd
	q.h = 0
}

func (q *Deque) pushFront(v int) {
	if q.n == len(q.data) {
		q.grow()
	}
	// 0-1 会变成 -1，必须先加 capacity 再取模
	q.h = (q.h - 1 + len(q.data)) % len(q.data)
	q.data[q.h] = v
	q.n++
}

func (q *Deque) pushBack(v int) {
	if q.n == len(q.data) {
		q.grow()
	}
	q.data[(q.h+q.n)%len(q.data)] = v
	q.n++
}

// 头删：先读再右移，否则读到的是新 head 上的元素。
func (q *Deque) popFront() (int, bool) {
	if q.n == 0 {
		return 0, false
	}
	v := q.data[q.h]
	q.h = (q.h + 1) % len(q.data)
	q.n--
	return v, true
}

// 尾删：只需 n 减一，把那一格「逻辑上」丢掉。
func (q *Deque) popBack() (int, bool) {
	if q.n == 0 {
		return 0, false
	}
	v := q.data[(q.h+q.n-1)%len(q.data)]
	q.n--
	return v, true
}

func (q *Deque) front() (int, bool) {
	if q.n == 0 {
		return 0, false
	}
	return q.data[q.h], true
}

func (q *Deque) back() (int, bool) {
	if q.n == 0 {
		return 0, false
	}
	return q.data[(q.h+q.n-1)%len(q.data)], true
}

func (q *Deque) clear() {
	q.h = 0
	q.n = 0
}

func (q *Deque) toSlice() []int {
	out := []int{}
	for i := 0; i < q.n; i++ {
		out = append(out, q.data[(q.h+i)%len(q.data)])
	}
	return out
}

func (q *Deque) print() {
	fmt.Print("[")
	for i, v := range q.toSlice() {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(v)
	}
	fmt.Printf("] size=%d cap=%d head=%d\n", q.n, len(q.data), q.h)
}

func check(cond bool, msg string) {
	if !cond {
		panic("assert failed: " + msg)
	}
}

func expect(q *Deque, want []int) {
	got := q.toSlice()
	check(len(got) == len(want), "length mismatch")
	for i := range want {
		check(got[i] == want[i], fmt.Sprintf("index %d mismatch", i))
	}
	check(q.n == len(want), "size mismatch")
}

func main() {
	// 用例 1：两端交替操作，顺序与两端取值都要对。
	a := newDeque()
	a.pushBack(3)
	a.pushFront(2)
	a.pushBack(4)
	a.pushFront(1)
	expect(a, []int{1, 2, 3, 4})
	fv, ok := a.front()
	check(ok && fv == 1, "front")
	bv, ok := a.back()
	check(ok && bv == 4, "back")
	v, ok := a.popFront()
	check(ok && v == 1, "popFront")
	v, ok = a.popBack()
	check(ok && v == 4, "popBack")
	expect(a, []int{2, 3})
	a.pushFront(1)
	a.pushBack(4)
	expect(a, []int{1, 2, 3, 4})
	a.print()

	// 用例 2：头插到底，跨多次扩容，顺序必须是整体倒序。
	b := newDeque()
	for i := 0; i < 100; i++ {
		b.pushFront(i)
	}
	check(b.size() == 100 && b.capacity() == 128, "head-insert growth")
	for i := 0; i < 100; i++ {
		check(b.toSlice()[i] == 99-i, "reverse order")
	}
	fv, ok = b.front()
	check(ok && fv == 99, "front after head inserts")
	bv, ok = b.back()
	check(ok && bv == 0, "back after head inserts")

	// 用例 3：关键点用例——head 不为 0 时扩容，两端数据顺序不能乱。
	c := newDeque()
	for i := 1; i <= 4; i++ {
		c.pushBack(i) // 满，head=0
	}
	v, ok = c.popFront()
	check(ok && v == 1, "pop 1")
	v, ok = c.popFront()
	check(ok && v == 2, "pop 2") // head 走到 2
	c.pushBack(5)                // 写到下标 0
	c.pushBack(6)                // 写到下标 1
	expect(c, []int{3, 4, 5, 6})
	check(c.head() == 2 && c.full(), "wrapped full")
	c.pushBack(7) // 触发扩容
	check(c.capacity() == 8 && c.head() == 0, "grow resets head")
	expect(c, []int{3, 4, 5, 6, 7})
	// 扩容后立刻头插：head 必须回绕到 7，而不是变成 -1。
	c.pushFront(2)
	check(c.head() == 7, "head wrap to 7")
	expect(c, []int{2, 3, 4, 5, 6, 7})
	fv, ok = c.front()
	check(ok && fv == 2, "front 2")
	bv, ok = c.back()
	check(ok && bv == 7, "back 7")
	c.print()

	// 用例 4：空队列的每个出口。
	e := newDeque()
	check(e.empty() && !e.full() && e.size() == 0, "empty init")
	if _, ok := e.popFront(); ok {
		panic("popFront on empty should fail")
	}
	if _, ok := e.popBack(); ok {
		panic("popBack on empty should fail")
	}
	if _, ok := e.front(); ok {
		panic("front on empty should fail")
	}
	if _, ok := e.back(); ok {
		panic("back on empty should fail")
	}
	expect(e, []int{})
	e.pushFront(1)
	fv, ok = e.front()
	bv, ok2 := e.back()
	check(ok && ok2 && fv == 1 && bv == 1, "single element both ends")
	e.clear()
	check(e.empty() && e.size() == 0 && e.head() == 0, "after clear")
	e.pushBack(2) // clear 之后还能继续用
	expect(e, []int{2})

	// 用例 5：长操作序列对拍，两端交替进出，逐步核对 size 与内容。
	q := newDeque()
	model := []int{} // 仅用于校验：当作双端队列的参考记录
	for step := 0; step < 300; step++ {
		switch step % 4 {
		case 0:
			q.pushBack(step)
			model = append(model, step)
		case 1:
			q.pushFront(step)
			model = append([]int{step}, model...)
		case 2:
			if len(model) > 0 {
				want := model[0]
				model = model[1:]
				got, ok := q.popFront()
				check(ok && got == want, "seq popFront")
			}
		default:
			if len(model) > 0 {
				want := model[len(model)-1]
				model = model[:len(model)-1]
				got, ok := q.popBack()
				check(ok && got == want, "seq popBack")
			}
		}
		check(q.size() == len(model), "seq size")
		if step%37 == 0 {
			expect(q, model)
		}
	}
	expect(q, model)

	fmt.Println("all tests passed")
}
