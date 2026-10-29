/*
============================================================================
手写优先队列（Priority Queue）
分类：手写数据结构 · 堆与优先队列
操作复杂度：

	push   O(log n)  追加到末尾后上浮
	pop    O(log n)  末尾补根后下沉
	peek   O(1)      根就是当前最优任务
	size/empty O(1)

============================================================================

用途：

	任务调度、事件模拟、带权最短路的取最小边。面试里常接着问一句
	「同优先级怎么保证先来先服务」，这就是下面的稳定化写法。

核心思想：

	和二叉堆一样用切片存完全二叉树，但比较的是二元组 (优先级, 插入序号)：
	优先级大的先出；优先级相同时插入序号小的先出。序号随 push 单调递增，
	于是同优先级天然按入队顺序出队，不需要额外队列。

关键点（不变量）：

	· 比较必须两级：先比 priority，相等再比 seq，且 seq 小的优先。
	  只比 priority 时，等值元素的先后由堆的交换决定，顺序不稳定。
	· seq 由队列自己发号，调用方不能自己传，否则稳定性无从保证。
	· 上浮/下沉只走一条根到叶的路径，停的条件是「父不再更优」。
	· pop 后必须先把末尾元素搬到根再下沉，不能留空洞。

手推一遍：

	任务依次入队：(2,a)、(3,b)、(3,c)、(1,d)，序号分别是 0..3。
	1. a 进 -> 根 a。
	2. b 优先级 3 比 a 的 2 大，b 上浮成根。
	3. c 与 b 同级但序号 2 更大，b 仍在上，c 挂在下面。
	4. d 优先级最小，沉在叶子上。
	出队顺序：b(3,#1)、c(3,#2)、a(2,#0)、d(1,#3)，即 b c a d。

复杂度分析：

	时间 O(log n)：每次 push/pop 只沿一条链走，链长不超过树高 log2(n)。
	空间 O(n)：一个任务切片，元素为 (priority, seq, id) 三个 int。
	稳定性：稳定（靠 seq 次关键字），不是排序算法本身稳定。
*/
package main

import (
	"fmt"
	"strings"
)

type task struct {
	priority int
	seq      int
	id       int
}

type PriorityQueue struct {
	data    []task
	nextSeq int
}

func NewPriorityQueue() *PriorityQueue {
	return &PriorityQueue{data: make([]task, 0)}
}

func (q *PriorityQueue) Size() int   { return len(q.data) }
func (q *PriorityQueue) Empty() bool { return len(q.data) == 0 }

func (q *PriorityQueue) Clear() {
	q.data = q.data[:0]
	q.nextSeq = 0
}

func (q *PriorityQueue) Push(priority, id int) {
	q.data = append(q.data, task{priority: priority, seq: q.nextSeq, id: id})
	q.nextSeq++
	q.siftUp(len(q.data) - 1)
}

// 空队列返回 ok=false，调用方不用猜一个默认优先级。
func (q *PriorityQueue) Pop() (priority, id int, ok bool) {
	if len(q.data) == 0 {
		return 0, 0, false
	}
	priority = q.data[0].priority
	id = q.data[0].id
	last := q.data[len(q.data)-1]
	q.data = q.data[:len(q.data)-1]
	if len(q.data) > 0 {
		q.data[0] = last
		q.siftDown(0)
	}
	return priority, id, true
}

func (q *PriorityQueue) Peek() (priority, id int, ok bool) {
	if len(q.data) == 0 {
		return 0, 0, false
	}
	return q.data[0].priority, q.data[0].id, true
}

// 先比优先级，再比插入序号：这就是「堆 + 稳定化」的全部秘密。
func higher(a, b task) bool {
	if a.priority != b.priority {
		return a.priority > b.priority
	}
	return a.seq < b.seq
}

func (q *PriorityQueue) siftUp(i int) {
	for i > 0 {
		parent := (i - 1) / 2
		if !higher(q.data[i], q.data[parent]) {
			break
		}
		q.data[i], q.data[parent] = q.data[parent], q.data[i]
		i = parent
	}
}

func (q *PriorityQueue) siftDown(i int) {
	n := len(q.data)
	for {
		best := i
		left := 2*i + 1
		right := 2*i + 2
		if left < n && higher(q.data[left], q.data[best]) {
			best = left
		}
		if right < n && higher(q.data[right], q.data[best]) {
			best = right
		}
		// 两个孩子都不更优就停，再往下扫只会破坏堆序。
		if best == i {
			break
		}
		q.data[i], q.data[best] = q.data[best], q.data[i]
		i = best
	}
}

// 小应用：按优先级从高到低输出任务名，同级保持入队顺序。
func schedule(priorities []int, names []string) []string {
	q := NewPriorityQueue()
	for i, p := range priorities {
		q.Push(p, i)
	}
	order := make([]string, 0, len(priorities))
	for {
		_, id, ok := q.Pop()
		if !ok {
			return order
		}
		order = append(order, names[id])
	}
}

func equalInts(a, b []int) bool {
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

func check(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
}

func main() {
	// 1. 正常用例 + 稳定性：同优先级必须按原顺序输出。
	{
		got := schedule(
			[]int{3, 1, 3, 2},
			[]string{"write", "coffee", "review", "test"},
		)
		check(equalStrings(got, []string{"write", "review", "test", "coffee"}),
			"stable schedule order")
		fmt.Println("schedule order: " + strings.Join(got, " "))
	}

	// 2. 边界用例：空队列的 Pop/Peek 返回 false，Clear 后序号重置。
	{
		q := NewPriorityQueue()
		if p, id, ok := q.Pop(); ok {
			panic(fmt.Sprintf("empty pop returned %d/%d", p, id))
		}
		if _, _, ok := q.Peek(); ok {
			panic("empty peek should fail")
		}
		check(q.Empty() && q.Size() == 0, "empty queue state")

		q.Push(5, 100)
		check(q.Size() == 1, "size 1")
		p, id, ok := q.Pop()
		check(ok && p == 5 && id == 100, "single pop")
		check(q.Empty(), "empty after single pop")
		if _, _, ok := q.Pop(); ok {
			panic("pop after single drain should fail")
		}

		// 清空后可继续使用，序号从 0 重新发，顺序依然正确。
		q.Push(1, 0)
		q.Push(2, 1)
		_, id, _ = q.Pop()
		check(id == 1, "pop higher priority")
		q.Clear()
		check(q.Empty() && q.Size() == 0, "clear")
	}

	// 3. 关键点用例：只比优先级会挂——五个同级任务的顺序必须原样。
	{
		q := NewPriorityQueue()
		for i := 0; i < 5; i++ {
			q.Push(7, i)
		}
		check(q.Size() == 5, "five same priority")
		for i := 0; i < 5; i++ {
			p, id, ok := q.Pop()
			check(ok && p == 7, "same priority value")
			check(id == i, "same priority order")
		}
		check(q.Empty(), "drained same priority")

		// 交错 push/pop：序号继续递增，后进的同级任务排在后面。
		q.Push(4, 10)
		q.Push(4, 11)
		_, id, _ := q.Pop()
		check(id == 10, "interleaved first")
		q.Push(4, 12)
		_, id, _ = q.Pop()
		check(id == 11, "interleaved second")
		_, id, _ = q.Pop()
		check(id == 12, "interleaved third")
	}

	// 4. 随机优先级序列：期望序列用两重循环按「优先级降序、原序」构造。
	{
		// 固定种子，保证失败可复现。
		seed := uint32(20240501)
		nextRand := func() uint32 {
			seed ^= seed << 13
			seed ^= seed >> 17
			seed ^= seed << 5
			return seed
		}

		const n = 120
		priorities := make([]int, n)
		names := make([]string, n)
		pool := []string{"task-a", "task-b", "task-c", "task-d"}
		minPriority := 10
		maxPriority := -1
		for i := 0; i < n; i++ {
			v := int(nextRand() % 6)
			priorities[i] = v
			names[i] = pool[i%4]
			if v < minPriority {
				minPriority = v
			}
			if v > maxPriority {
				maxPriority = v
			}
		}

		// 期望值：优先级从高到低，同级按入队顺序扫一遍，不用任何排序。
		expect := make([]int, 0, n)
		for p := maxPriority; p >= minPriority; p-- {
			for i := 0; i < n; i++ {
				if priorities[i] == p {
					expect = append(expect, i)
				}
			}
		}

		q := NewPriorityQueue()
		for i := 0; i < n; i++ {
			q.Push(priorities[i], i)
		}
		check(q.Size() == n, "random queue size")

		got := make([]int, 0, n)
		for {
			_, id, ok := q.Pop()
			if !ok {
				break
			}
			got = append(got, id)
		}
		check(equalInts(got, expect), "random order")

		// 弹出来的优先级必须是单调不增的，这一条独立于期望序列。
		last := 1 << 30
		for _, id := range got {
			p := priorities[id]
			check(p <= last, "priority must be non-increasing")
			last = p
		}
		check(q.Empty(), "random drained")
	}

	fmt.Println("all tests passed")
}
