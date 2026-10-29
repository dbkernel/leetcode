/*
============================================================================
手写二叉堆（Binary Heap）
分类：手写数据结构 · 堆与优先队列
操作复杂度：

	push   O(log n)  新元素放末尾后上浮，最多走一条树高
	pop    O(log n)  末尾元素补到根后下沉，最多走一条树高
	peek   O(1)      根就是最值
	build  O(n)      自底向上 heapify，不是逐个 push 的 O(n log n)

============================================================================

用途：

	需要反复取最值的场景：优先队列、TopK、Dijkstra、合并 K 个有序链表。
	面试常要求手写堆，而不是直接调用 container/heap。

核心思想：

	把完全二叉树按层序塞进切片，父子关系用下标算出来，省掉指针。
	push 只在末尾追加再上浮；pop 把末尾元素搬到根再下沉，两者都只沿着
	一条从根到叶的路径比较，所以是 O(log n)。

关键点（不变量）：

	· 大根堆：每个父节点 >= 它的两个子节点；小根堆方向相反。
	· 0-based 下标：parent = (i-1)/2，children = 2*i+1、2*i+2。
	· 上浮看到父节点已满足堆序就停；下沉看到两个孩子都不更优就停。
	· build 必须从最后一个非叶节点 n/2-1 倒着下沉，才是 O(n)。

手推一遍：

	以小根堆为例，依次 push 3、1、4：
	1. push 3 -> [3]。
	2. push 1 -> [3,1]，1 比父 3 小，交换 -> [1,3]。
	3. push 4 -> [1,3,4]，4 比父 3 大，停。
	接着 pop：根 1 取出，末尾 4 补到根 -> [4,3]，
	4 与唯一的（较小的）孩子 3 交换 -> [3,4]。

复杂度分析：

	时间 O(log n)：push/pop 各沿树高走一条链，树高 log2(n)。
	空间 O(n)：一个切片存 n 个元素，不额外拷贝。
	稳定性：不稳定，等值元素的相对次序由交换决定。
*/
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

type BinaryHeap struct {
	data  []int
	isMax bool
}

func NewBinaryHeap(isMax bool) *BinaryHeap {
	return &BinaryHeap{data: make([]int, 0), isMax: isMax}
}

func (h *BinaryHeap) Size() int   { return len(h.data) }
func (h *BinaryHeap) Empty() bool { return len(h.data) == 0 }
func (h *BinaryHeap) Clear()      { h.data = h.data[:0] }

func (h *BinaryHeap) Push(value int) {
	h.data = append(h.data, value)
	h.siftUp(len(h.data) - 1)
}

// 空堆时返回 false，调用方不会被塞一个假值。
func (h *BinaryHeap) Pop() (int, bool) {
	if len(h.data) == 0 {
		return 0, false
	}
	top := h.data[0]
	last := h.data[len(h.data)-1]
	h.data = h.data[:len(h.data)-1]
	if len(h.data) > 0 {
		h.data[0] = last
		h.siftDown(0)
	}
	return top, true
}

func (h *BinaryHeap) Peek() (int, bool) {
	if len(h.data) == 0 {
		return 0, false
	}
	return h.data[0], true
}

// 自底向上建堆：叶子不用下沉，从最后一个非叶节点往回扫，每个节点
// 下沉的高度之和是 O(n)，比逐个 Push 的 O(n log n) 更快。
func (h *BinaryHeap) Build(values []int) {
	h.data = h.data[:0]
	h.data = append(h.data, values...)
	for i := len(h.data)/2 - 1; i >= 0; i-- {
		h.siftDown(i)
	}
}

// 大根堆里 a 该在上面；小根堆里 b 该在上面。
func (h *BinaryHeap) higher(a, b int) bool {
	if h.isMax {
		return a > b
	}
	return a < b
}

func (h *BinaryHeap) siftUp(i int) {
	for i > 0 {
		parent := (i - 1) / 2
		if !h.higher(h.data[i], h.data[parent]) {
			break
		}
		h.data[i], h.data[parent] = h.data[parent], h.data[i]
		i = parent
	}
}

func (h *BinaryHeap) siftDown(i int) {
	n := len(h.data)
	for {
		best := i
		left := 2*i + 1
		right := 2*i + 2
		if left < n && h.higher(h.data[left], h.data[best]) {
			best = left
		}
		if right < n && h.higher(h.data[right], h.data[best]) {
			best = right
		}
		// best 还是自己，说明两个孩子都不比它更优，再走就破坏堆序。
		if best == i {
			break
		}
		h.data[i], h.data[best] = h.data[best], h.data[i]
		i = best
	}
}

// 孩子比父亲更优先（大根堆更大、小根堆更小）就是违反堆序。
func violates(child, parent int, isMax bool) bool {
	if isMax {
		return child > parent
	}
	return child < parent
}

// 直接扫一遍切片，验证「父节点与两个孩子」的堆序不变量。
func heapOK(a []int, isMax bool) bool {
	n := len(a)
	for i := 0; i < n; i++ {
		left := 2*i + 1
		right := 2*i + 2
		if left < n && violates(a[left], a[i], isMax) {
			return false
		}
		if right < n && violates(a[right], a[i], isMax) {
			return false
		}
	}
	return true
}

func drain(h *BinaryHeap) []int {
	out := make([]int, 0, h.Size())
	for {
		v, ok := h.Pop()
		if !ok {
			return out
		}
		out = append(out, v)
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

func check(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
}

func main() {
	// 1. 正常用例：大根堆 push/pop 序列。
	{
		heap := NewBinaryHeap(true)
		for _, v := range []int{5, 3, 8, 1, 9, 2} {
			heap.Push(v)
		}
		check(heap.Size() == 6, "size after pushes")

		top, ok := heap.Peek()
		check(ok && top == 9, "peek max")
		v, ok := heap.Pop()
		check(ok && v == 9, "pop 9")
		v, ok = heap.Pop()
		check(ok && v == 8, "pop 8")
		check(heap.Size() == 4, "size after pops")

		heap.Push(7)
		got := drain(heap)
		check(equalInts(got, []int{7, 5, 3, 2, 1}), "max drain order")
		check(heap.Empty(), "empty after drain")

		small := NewBinaryHeap(false)
		for _, v := range []int{5, 3, 8, 1, 9, 2} {
			small.Push(v)
		}
		check(equalInts(drain(small), []int{1, 2, 3, 5, 8, 9}),
			"min drain order")
	}

	// 2. 边界用例：空堆 Pop/Peek 必须返回 false。
	{
		heap := NewBinaryHeap(true)
		if v, ok := heap.Pop(); ok {
			panic(fmt.Sprintf("empty pop returned %d", v))
		}
		if v, ok := heap.Peek(); ok {
			panic(fmt.Sprintf("empty peek returned %d", v))
		}
		check(heap.Empty() && heap.Size() == 0, "empty state")

		heap.Push(-7)
		check(heap.Size() == 1, "size 1")
		v, ok := heap.Pop()
		check(ok && v == -7, "single element pop")
		check(heap.Empty(), "empty after single pop")
		if _, ok := heap.Pop(); ok {
			panic("pop after single drain should fail")
		}
	}

	// 3. 关键点用例：Build 结果满足堆序，且重复 Build 不残留旧值。
	{
		heap := NewBinaryHeap(true)
		heap.Build([]int{7, 2, 9, 4, 4, 1, 8})
		check(heap.Size() == 7, "build size")
		check(heapOK(heap.data, true), "build heap property")

		heap.Build([]int{3, 1, 2})
		check(heap.Size() == 3, "rebuild size")
		check(heapOK(heap.data, true), "rebuild heap property")
		check(equalInts(drain(heap), []int{3, 2, 1}), "rebuild drain")

		// 空切片 Build 也不能越界（n/2-1 = -1，循环一次都不进）。
		heap.Build(nil)
		check(heap.Empty() && heapOK(heap.data, true), "empty build")
	}

	// 4. 全相同元素：既不能死循环，也不能把某个副本弄丢。
	{
		small := NewBinaryHeap(false)
		for i := 0; i < 6; i++ {
			small.Push(6)
		}
		got := drain(small)
		check(len(got) == 6, "duplicates count")
		for _, v := range got {
			check(v == 6, "duplicates value")
		}

		many := NewBinaryHeap(true)
		same := make([]int, 16)
		for i := range same {
			same[i] = 4
		}
		many.Build(same)
		check(heapOK(many.data, true), "duplicates heap property")
		check(len(drain(many)) == 16, "duplicates drain count")
	}

	// 5. 随机对拍：固定种子，弹出序列必须与升序/降序一致。
	{
		rng := rand.New(rand.NewSource(20240501))
		for round := 0; round < 5; round++ {
			n := 20 + round*37
			raw := make([]int, 0, n)
			big := NewBinaryHeap(true)
			small := NewBinaryHeap(false)
			for i := 0; i < n; i++ {
				v := rng.Intn(2001) - 1000
				raw = append(raw, v)
				big.Push(v)
				small.Push(v)
			}
			check(big.Size() == n && small.Size() == n, "random size")

			// 仅用于校验：实现部分没有使用内置排序。
			sort.Ints(raw)
			asc := make([]int, n)
			copy(asc, raw)
			desc := make([]int, n)
			for i := 0; i < n; i++ {
				desc[i] = raw[n-1-i]
			}
			check(equalInts(drain(big), desc), "random max order")
			check(equalInts(drain(small), asc), "random min order")
			check(big.Empty() && small.Empty(), "random drained")
		}

		// 交错 push/pop：size 始终等于有效元素个数。
		heap := NewBinaryHeap(true)
		heap.Push(4)
		heap.Push(10)
		v, ok := heap.Pop()
		check(ok && v == 10, "interleaved pop 10")
		heap.Push(1)
		heap.Push(7)
		check(heap.Size() == 3, "interleaved size")
		v, ok = heap.Pop()
		check(ok && v == 7, "interleaved pop 7")
		heap.Clear()
		check(heap.Empty() && heap.Size() == 0, "clear")
	}

	fmt.Println("all tests passed")
}
