/*
============================================================================
动态数组（Dynamic Array）
分类：手写数据结构 · 线性结构

操作复杂度：
  pushBack   均摊 O(1)  满时容量翻倍，n 次 push 累计搬移小于 2n
  popBack    O(1)       缩容只在 size <= capacity/4 时发生
  insert     O(n)       index 及其后的元素整体右移一位
  erase      O(n)       index 之后的元素整体左移一位
  at         O(1)       带越界检查，越界返回 (0, false)
  find       O(n)       线性扫描，返回下标，找不到返回 -1
  clear      O(1)       把 size 置 0，必要时顺带缩容
============================================================================

用途：
  手写一切「长度可变」容器的底座：栈、队列、哈希表的桶都能建在它上面。
  面试常以「实现一个 vector」或「这行为什么退化到 O(n^2)」的形式出现。

核心思想：
  切片 + 逻辑长度两个量。切片的 len 当 capacity 用，另存一个 n 当 size。
  写满就换一块两倍长的切片把元素拷过去；空出来四分之三就换成一半长。

关键点（不变量）：
  · 0 <= n <= len(data)，len(data) 从 4 开始，每次只翻倍或减半，
    所以它永远是 2 的幂；
  · pushBack 必须先扩容再写 data[n]，顺序反了就是越界写；
  · 缩容条件是 n <= len(data)/4 且 len(data) >= 8。若在 n 刚低于
    capacity/2 时就减半，同一个位置反复 push/pop 会来回搬移；
  · insert 从后往前搬，erase 从前往后搬，方向反了会覆盖未读元素。

手推一遍：
  以 capacity=4、空数组开始连续 push 0..4：
  1. push 0..3 一路顺序写，写完 n=4，正好等于 capacity；
  2. push 4 时 n==capacity，先换 8 长的切片并拷走 4 个元素，再写 data[4]；
  3. 累计搬移 4 次，总写入 5 次，摊到每次 push 不到 1 次；
  4. 之后 pop 到 n=2（== 8/4）时减半回 4，此时占用率 1/2，
     再 push 两次才会再扩容，不会在两倍临界点上抖动。

复杂度分析：
  时间：单次 pushBack 最坏 O(n)（扩容搬移），但 n 次连续 push 的总搬移
    是 4+8+16+... < 2n，把这份代价摊到每一次 push 上就是均摊 O(1)。
    insert/erase 要整体挪动，是 O(n)；at 是 O(1)。
  空间：O(n)，capacity 至多是 size 的 2 倍，浪费是常数倍。
  为什么不是 1.5 倍：1.5 倍同样能保证均摊 O(1)（总搬移仍是 n 的常数倍），
    只是常数更大、更省内存；选 2 倍纯粹是加法比乘法好写，不影响正确性。
*/

package main

import "fmt"

const initCap = 4

type DynamicArray struct {
	data []int // 长度当 capacity 用，前 n 个是有效元素
	n    int
}

func newDynamicArray() *DynamicArray {
	return &DynamicArray{data: make([]int, initCap)}
}

func (a *DynamicArray) size() int { return a.n }

func (a *DynamicArray) capacity() int { return len(a.data) }

func (a *DynamicArray) empty() bool { return a.n == 0 }

func (a *DynamicArray) grow() {
	nd := make([]int, len(a.data)*2)
	copy(nd, a.data[:a.n])
	a.data = nd
}

// 一次可能连降两级（比如从 16 掉到 2 再掉到 1），所以用 for 而不是 if。
func (a *DynamicArray) shrink() {
	for len(a.data) >= 8 && a.n <= len(a.data)/4 {
		nd := make([]int, len(a.data)/2)
		copy(nd, a.data[:a.n])
		a.data = nd
	}
}

func (a *DynamicArray) pushBack(v int) {
	if a.n == len(a.data) {
		a.grow() // 先扩容，再写入
	}
	a.data[a.n] = v
	a.n++
}

func (a *DynamicArray) popBack() bool {
	if a.n == 0 {
		return false
	}
	a.n--
	a.shrink()
	return true
}

func (a *DynamicArray) insert(index, v int) bool {
	// index == n 等价于尾插，仍然合法；其余越界一律拒绝。
	if index < 0 || index > a.n {
		return false
	}
	if a.n == len(a.data) {
		a.grow()
	}
	// 从后往前挪一位，给 index 腾位置；反向会覆盖还没搬的元素。
	for i := a.n; i > index; i-- {
		a.data[i] = a.data[i-1]
	}
	a.data[index] = v
	a.n++
	return true
}

func (a *DynamicArray) erase(index int) bool {
	if index < 0 || index >= a.n {
		return false
	}
	// 从前往后挪，覆盖掉 index 这一格；i+1 < n 保证读不越界。
	for i := index; i+1 < a.n; i++ {
		a.data[i] = a.data[i+1]
	}
	a.n--
	a.shrink()
	return true
}

func (a *DynamicArray) at(index int) (int, bool) {
	if index < 0 || index >= a.n {
		return 0, false
	}
	return a.data[index], true
}

func (a *DynamicArray) find(v int) int {
	for i := 0; i < a.n; i++ {
		if a.data[i] == v {
			return i
		}
	}
	return -1
}

func (a *DynamicArray) clear() {
	a.n = 0
	a.shrink() // 清空后通常不会马上塞满，顺手把内存还回去
}

func (a *DynamicArray) print() {
	fmt.Print("[")
	for i := 0; i < a.n; i++ {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(a.data[i])
	}
	fmt.Println("]")
}

func check(cond bool, msg string) {
	if !cond {
		panic("assert failed: " + msg)
	}
}

func expect(a *DynamicArray, want []int) {
	check(a.n == len(want), "size mismatch")
	for i, v := range want {
		check(a.data[i] == v, fmt.Sprintf("index %d mismatch", i))
	}
}

func main() {
	// 用例 1：连续 push 跨多次扩容，容量按 4/8/16/... 翻倍。
	a := newDynamicArray()
	check(a.capacity() == 4 && a.empty(), "empty init")
	for i := 0; i < 100; i++ {
		a.pushBack(i)
	}
	check(a.size() == 100, "size after 100 push")
	check(a.capacity() == 128, "growth sequence") // 4,8,16,32,64,128
	v0, ok0 := a.at(0)
	v99, ok99 := a.at(99)
	v50, ok50 := a.at(50)
	check(ok0 && v0 == 0, "at(0)")
	check(ok99 && v99 == 99, "at(99)")
	check(ok50 && v50 == 50, "at(50)")
	_, bad := a.at(100)
	check(!bad, "at out of range")
	check(a.find(42) == 42 && a.find(-1) == -1, "find")
	a.print()

	// 用例 2：insert 的头、尾、中间与越界。
	b := newDynamicArray()
	b.pushBack(1)
	b.pushBack(2)
	b.pushBack(3)
	check(b.insert(0, 0), "insert head")
	check(b.insert(4, 4), "insert tail")
	check(b.insert(2, 99), "insert middle")
	check(!b.insert(99, 7), "insert out of range")
	check(!b.insert(-1, 7), "insert negative")
	expect(b, []int{0, 1, 99, 2, 3, 4})
	check(b.size() == 6, "size after insert")

	// 用例 3：erase 的头、尾、中间与越界。
	check(b.erase(0), "erase head")
	expect(b, []int{1, 99, 2, 3, 4})
	check(b.erase(4), "erase tail")
	expect(b, []int{1, 99, 2, 3})
	check(b.erase(1), "erase middle")
	expect(b, []int{1, 2, 3})
	check(!b.erase(3), "erase index == size")
	check(!b.erase(-1), "erase negative")
	check(b.size() == 3, "size after erase")

	// 用例 4：缩容后再 push，数据不能丢、容量要能重新长回来。
	c := newDynamicArray()
	for i := 0; i < 16; i++ {
		c.pushBack(i)
	}
	check(c.capacity() == 16, "cap 16")
	for i := 0; i < 12; i++ {
		check(c.popBack(), "pop 12 times")
	}
	check(c.size() == 4, "size 4")
	check(c.capacity() == 8, "shrink to 8") // size==4 命中 4<=16/4
	expect(c, []int{0, 1, 2, 3})
	for i := 0; i < 4; i++ {
		c.pushBack(100 + i)
	}
	check(c.size() == 8 && c.capacity() == 8, "half full, no grow")
	c.pushBack(200) // 第 9 个才扩容
	check(c.size() == 9 && c.capacity() == 16, "grow at 9")
	expect(c, []int{0, 1, 2, 3, 100, 101, 102, 103, 200})

	// 用例 5：空数组的每个出口。
	e := newDynamicArray()
	check(e.empty() && e.size() == 0 && e.capacity() == 4, "empty state")
	check(!e.popBack(), "pop empty")
	check(e.find(1) == -1, "find in empty")
	check(!e.erase(0), "erase empty")
	if _, ok := e.at(0); ok {
		panic("at on empty should fail")
	}
	e.pushBack(7)
	e.clear()
	check(e.size() == 0 && e.empty(), "after clear")
	e.pushBack(8) // 清空之后还能继续用
	expect(e, []int{8})

	fmt.Println("all tests passed")
}
