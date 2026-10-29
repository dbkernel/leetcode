/*
============================================================================
循环队列（Circular Queue）
分类：手写数据结构 · 线性结构

操作复杂度：
  enqueue   均摊 O(1)  满时 2 倍扩容并重排下标
  dequeue   O(1)       只动 head，不搬元素
  front     O(1)       队首元素
  rear      O(1)       队尾元素，由 (head+size-1) 算出
  empty     O(1)
  full      O(1)
  size      O(1)
  clear     O(1)
============================================================================

用途：
  BFS 的待访问队列、任务调度、滑动窗口最值、生产者/消费者之间的环形
  缓冲（音频流、日志缓冲都是这个形状）。

核心思想：
  一块环形数组，只维护 head 和 size 两个量：队尾就是
  (head + size) % capacity，不用单独存 tail。出队只把 head 往前走一格，
  元素一个都不搬，于是 dequeue 是 O(1)，而不是数组删除的 O(n)。

关键点（不变量）：
  · 队首逻辑下标是 head，队尾逻辑下标是 (head + size - 1) % capacity，
    一律用公式算，不要另存一个 tail 指针去维护两套状态；
  · 用 size 区分空满，而不是「留一个空槽」：留空槽法最多只能存
    capacity-1 个元素，且每次判满都要绕一圈算 (tail+1)%cap == head；
  · 下标回绕统一写 (i + 1) % capacity 这类形式，往左走必须写成
    (i - 1 + capacity) % capacity，Go 的 % 对负数同样会给负余数；
  · 扩容时必须先按「旧 head、旧 capacity」把元素按逻辑顺序搬到新切片的
    0..size-1，循环结束后才能把 head 置 0、capacity 翻倍；先改 head
    或先改 capacity，取模映射就全错，元素顺序会乱。

手推一遍：
  容量 4，依次 enqueue 1、2、3、4 后 size=4、head=0，队满。
  1. dequeue 两次，取走 1、2，head 走到 2，size=2，逻辑内容 [3, 4]；
  2. 再 enqueue 5：写入下标 (2+2)%4 = 0；enqueue 6：写入 (2+3)%4 = 1；
     物理上是 [5, 6, 3, 4]，逻辑上仍是 [3, 4, 5, 6]，这就是回绕；
  3. 此时再 enqueue 7 就满了，触发扩容：按逻辑顺序搬出 3、4、5、6、7
     到新数组下标 0..4，head 置 0，capacity 变 8。

复杂度分析：
  时间：enqueue 均摊 O(1)（扩容搬移摊到每次入队，n 次入队总搬移小于 2n），
    dequeue/front/rear 都是 O(1)，因为不搬元素只挪下标。
  空间：O(n)，capacity 最多是 size 的 2 倍。
  稳定性：队列是 FIFO，谈不到稳定性。
  与「数组模拟队列」的对比：不做循环就得每次出队整体左移，那是 O(n)；
  循环队列把这份 O(n) 摊成 O(1)，代价只是多一次取模。
*/

package main

import "fmt"

const queueInitCap = 4

type CircularQueue struct {
	data []int
	h    int // 队首所在的物理下标
	n    int // 元素个数，空满全靠它区分
}

func newCircularQueue() *CircularQueue {
	return &CircularQueue{data: make([]int, queueInitCap)}
}

func (q *CircularQueue) size() int { return q.n }

func (q *CircularQueue) capacity() int { return len(q.data) }

// 只为自测观察回绕，不参与逻辑。
func (q *CircularQueue) head() int { return q.h }

func (q *CircularQueue) empty() bool { return q.n == 0 }

func (q *CircularQueue) full() bool { return q.n == len(q.data) }

func (q *CircularQueue) grow() {
	nd := make([]int, len(q.data)*2)
	// 关键：这一步用的是旧 q.h、旧 capacity，所以映射关系一定正确。
	for i := 0; i < q.n; i++ {
		nd[i] = q.data[(q.h+i)%len(q.data)]
	}
	q.data = nd
	// 搬完之后才能改这两个量：先改就再也算不出旧下标了。
	q.h = 0
}

func (q *CircularQueue) enqueue(v int) bool {
	if q.n == len(q.data) {
		q.grow() // 先扩容，再写入
	}
	// 队尾逻辑位置：head 往右数 n 格，取模把尾巴绕回头部
	q.data[(q.h+q.n)%len(q.data)] = v
	q.n++
	return true
}

// 空队列返回 (0, false)。
func (q *CircularQueue) dequeue() (int, bool) {
	if q.n == 0 {
		return 0, false
	}
	v := q.data[q.h]
	q.h = (q.h + 1) % len(q.data) // 只挪下标，不搬元素
	q.n--
	return v, true
}

func (q *CircularQueue) front() (int, bool) {
	if q.n == 0 {
		return 0, false
	}
	return q.data[q.h], true
}

func (q *CircularQueue) rear() (int, bool) {
	if q.n == 0 {
		return 0, false
	}
	// n-1 是队尾的偏移量；n >= 1 在这里已由判空保证
	return q.data[(q.h+q.n-1)%len(q.data)], true
}

func (q *CircularQueue) clear() {
	q.h = 0
	q.n = 0
}

func (q *CircularQueue) toSlice() []int {
	out := []int{}
	for i := 0; i < q.n; i++ {
		out = append(out, q.data[(q.h+i)%len(q.data)])
	}
	return out
}

func (q *CircularQueue) print() {
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

func expect(q *CircularQueue, want []int) {
	got := q.toSlice()
	check(len(got) == len(want), "length mismatch")
	for i := range want {
		check(got[i] == want[i], fmt.Sprintf("index %d mismatch", i))
	}
	check(q.n == len(want), "size mismatch")
}

func main() {
	// 用例 1：出两个进两个，反复 6 轮，让 head 回绕多圈（容量保持 4）。
	a := newCircularQueue()
	m := []int{} // 仅用于校验：当作 FIFO 的参考记录
	next := 1
	for i := 0; i < 4; i++ {
		check(a.enqueue(next), "fill")
		m = append(m, next)
		next++
	}
	check(a.full() && a.size() == 4, "full")
	for round := 0; round < 6; round++ {
		for k := 0; k < 2; k++ {
			v, ok := a.dequeue()
			check(ok, "dequeue")
			check(v == m[0], "fifo order") // 回绕之后顺序依然正确
			m = m[1:]
		}
		for k := 0; k < 2; k++ {
			check(a.enqueue(next), "enqueue")
			m = append(m, next)
			next++
		}
		check(a.full() && a.size() == 4, "full again")
		expect(a, m)
		check(a.head() == ((round+1)*2)%4, "head advance") // head 回绕
	}
	check(a.capacity() == 4, "no growth") // 全程没超过容量，没触发扩容
	a.print()

	// 用例 2：半满时再入队，物理下标会绕回头部。
	b := newCircularQueue()
	for i := 1; i <= 4; i++ {
		b.enqueue(i) // 满，head=0
	}
	v, ok := b.dequeue()
	check(ok && v == 1, "dequeue 1")
	v, ok = b.dequeue()
	check(ok && v == 2, "dequeue 2") // head 走到 2
	check(b.enqueue(5), "enqueue 5") // 写到下标 0
	check(b.enqueue(6), "enqueue 6") // 写到下标 1
	expect(b, []int{3, 4, 5, 6})
	check(b.head() == 2 && b.full(), "wrapped full")
	b.print()

	// 用例 3：满队列再入队触发扩容，head 归零且顺序不变。
	check(b.enqueue(7), "enqueue 7")
	check(b.capacity() == 8, "grew to 8")
	check(b.head() == 0, "head reset")
	expect(b, []int{3, 4, 5, 6, 7})
	fv, ok := b.front()
	check(ok && fv == 3, "front kept") // 扩容后队首没变
	rv, ok := b.rear()
	check(ok && rv == 7, "rear")
	b.print()
	// 扩容后继续绕：出两个再进两个，不应该再扩容。
	v, ok = b.dequeue()
	check(ok && v == 3, "dequeue 3")
	v, ok = b.dequeue()
	check(ok && v == 4, "dequeue 4")
	check(b.enqueue(8) && b.enqueue(9), "enqueue 8,9")
	check(b.capacity() == 8, "still 8")
	expect(b, []int{5, 6, 7, 8, 9})

	// 用例 4：空队列的每个出口。
	e := newCircularQueue()
	check(e.empty() && !e.full() && e.size() == 0, "empty init")
	if _, ok := e.dequeue(); ok {
		panic("dequeue on empty should fail")
	}
	if _, ok := e.front(); ok {
		panic("front on empty should fail")
	}
	if _, ok := e.rear(); ok {
		panic("rear on empty should fail")
	}
	expect(e, []int{})
	check(e.enqueue(1), "enqueue into empty")
	fv, ok = e.front()
	rv, ok2 := e.rear()
	check(ok && ok2 && fv == 1 && rv == 1, "single element ends")
	e.clear()
	check(e.empty() && e.size() == 0 && e.head() == 0, "after clear")
	check(e.enqueue(2), "enqueue after clear") // clear 之后还能继续用
	expect(e, []int{2})

	// 用例 5：长操作序列，size 与出队顺序逐步核对（参考模型是切片）。
	q := newCircularQueue()
	model := []int{}
	pushed := 0
	for step := 0; step < 200; step++ {
		if step%3 != 2 {
			check(q.enqueue(pushed), "seq enqueue")
			model = append(model, pushed)
			pushed++
		} else if len(model) > 0 {
			want := model[0]
			model = model[1:] // 参考模型可以直接 O(n) 删
			got, ok := q.dequeue()
			check(ok && got == want, "seq dequeue")
		}
		check(q.size() == len(model), "seq size")
	}
	expect(q, model)

	fmt.Println("all tests passed")
}
