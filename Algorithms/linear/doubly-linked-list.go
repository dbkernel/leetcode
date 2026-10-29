/*
============================================================================
双向链表（Doubly Linked List）
分类：手写数据结构 · 线性结构

操作复杂度：
  pushFront        O(1)  插在头哨兵之后
  pushBack         O(1)  插在尾哨兵之前
  removeNode       O(1)  拿到节点指针就能直接摘，不用找前驱
  removeAt         O(n)  按下标要先走
  removeValue      O(n)  按值要先找
  insertAt         O(n)  按下标要先走
  toSlice          O(n)  正向遍历
  toSliceReverse   O(n)  反向遍历
  size/empty       O(1)
============================================================================

用途：
  LRU 缓存那类「要 O(1) 摘掉任意一个已知节点」的场景；浏览器前进后退、
  编辑器的 undo/redo、以及需要双向遍历的任何链表题。

核心思想：
  每个节点同时记 prev 和 next，再在两端各放一个哨兵。有了 prev，删除
  一个已知节点只要改两个邻居的指针，一步到位；再加尾哨兵，删尾、插尾
  都变成普通操作，不必特判。

关键点（不变量）：
  · head 与 tail 都是哨兵，互相指向时链表为空，size 只数真实节点；
  · 插入新节点时先接好新节点的 prev/next，再改两个邻居的指针，
    顺序反了会把邻居之间的链子提前断掉；
  · 摘除节点要成对改线：node.prev.next = node.next 与
    node.next.prev = node.prev，少改一根，反向遍历立刻错；
  · 有尾哨兵后「删最后一个节点」不是特例，tail 永远不用变。

手推一遍：
  链表 head <-> 1 <-> 2 <-> 3 <-> tail，执行 removeNode(节点 2)：
  1. node.prev 是 1、node.next 是 3；
  2. 1.next = 3、3.prev = 1；
  3. 得到 head <-> 1 <-> 3 <-> tail，size 3 -> 2，两个方向都还自洽。
  若换成删尾节点 3：3.next 是 tail 哨兵，于是 2.next = tail 且
  tail.prev = 2，一句话覆盖了单链表里需要特判 tail 的分支。

复杂度分析：
  时间：头尾插入 O(1)，删除「给定的节点」O(1)，这是它相对单链表的唯一
    核心优势（单链表删一个节点得先花 O(n) 找到前驱）。按下标/按值操作
    仍然要遍历，是 O(n)。
  空间：每个元素两个指针，比单链表多一个指针字段，总计仍是 O(n)。
  取舍：多花指针换删除的便捷，随机访问依旧 O(n)。
*/

package main

import "fmt"

type dnode struct {
	val        int
	prev, next *dnode
}

type DoublyLinkedList struct {
	head *dnode // 头哨兵
	tail *dnode // 尾哨兵
	n    int
}

func newDoublyLinkedList() *DoublyLinkedList {
	h := &dnode{}
	t := &dnode{}
	h.next = t // 空表：两个哨兵直接互指
	t.prev = h
	return &DoublyLinkedList{head: h, tail: t}
}

func (l *DoublyLinkedList) size() int { return l.n }

func (l *DoublyLinkedList) empty() bool { return l.n == 0 }

// 把 nd 插到 prev 之后。先接新节点的两根线，再改邻居，避免提前断链。
func (l *DoublyLinkedList) linkAfter(prev, nd *dnode) {
	nd.prev = prev
	nd.next = prev.next
	prev.next.prev = nd // 顺序要紧：这一步必须在改 prev.next 之前
	prev.next = nd
	l.n++
}

// 摘链：成对改两个邻居的指针，nd 本身交给 GC。
func (l *DoublyLinkedList) unlink(nd *dnode) {
	nd.prev.next = nd.next
	nd.next.prev = nd.prev
	l.n--
}

func (l *DoublyLinkedList) pushFront(v int) {
	l.linkAfter(l.head, &dnode{val: v})
}

func (l *DoublyLinkedList) pushBack(v int) {
	l.linkAfter(l.tail.prev, &dnode{val: v})
}

func (l *DoublyLinkedList) insertAt(index, v int) bool {
	// index == n 等价于尾插。
	if index < 0 || index > l.n {
		return false
	}
	prev := l.head
	for i := 0; i < index; i++ {
		prev = prev.next
	}
	l.linkAfter(prev, &dnode{val: v})
	return true
}

// 删除给定节点：O(1)，这正是双链表存在的理由。
func (l *DoublyLinkedList) removeNode(nd *dnode) bool {
	if nd == nil || nd == l.head || nd == l.tail {
		return false
	}
	l.unlink(nd)
	return true
}

func (l *DoublyLinkedList) removeAt(index int) bool {
	if index < 0 || index >= l.n {
		return false
	}
	cur := l.head.next
	for i := 0; i < index; i++ {
		cur = cur.next
	}
	l.unlink(cur)
	return true
}

func (l *DoublyLinkedList) removeValue(v int) bool {
	cur := l.findNode(v)
	if cur == nil {
		return false
	}
	l.unlink(cur)
	return true
}

// 返回节点句柄，找不到返回 nil；配合 removeNode 才是 O(1) 删除。
func (l *DoublyLinkedList) findNode(v int) *dnode {
	for p := l.head.next; p != l.tail; p = p.next {
		if p.val == v {
			return p
		}
	}
	return nil
}

func (l *DoublyLinkedList) toSlice() []int {
	out := []int{}
	for p := l.head.next; p != l.tail; p = p.next {
		out = append(out, p.val)
	}
	return out
}

func (l *DoublyLinkedList) toSliceReverse() []int {
	out := []int{}
	for p := l.tail.prev; p != l.head; p = p.prev {
		out = append(out, p.val)
	}
	return out
}

func (l *DoublyLinkedList) clear() {
	l.head.next = l.tail
	l.tail.prev = l.head
	l.n = 0
}

func (l *DoublyLinkedList) print() {
	fmt.Print("[")
	for i, v := range l.toSlice() {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(v)
	}
	fmt.Print("]")
}

func check(cond bool, msg string) {
	if !cond {
		panic("assert failed: " + msg)
	}
}

// 正反两个方向必须互为逆序，任何一根指针写错都会在这里暴露。
func expect(l *DoublyLinkedList, want []int) {
	got := l.toSlice()
	check(len(got) == len(want), "length mismatch")
	for i := range want {
		check(got[i] == want[i], fmt.Sprintf("index %d mismatch", i))
	}
	rev := l.toSliceReverse()
	check(len(rev) == len(want), "reverse length mismatch")
	for i := range want {
		check(rev[i] == want[len(want)-1-i], fmt.Sprintf("rev %d mismatch", i))
	}
	check(l.n == len(want), "size mismatch")
}

func main() {
	// 用例 1：头尾交替插入，正向与反向都要对。
	a := newDoublyLinkedList()
	check(a.empty() && a.n == 0, "empty init")
	a.pushBack(3)
	a.pushFront(2)
	a.pushBack(4)
	a.pushFront(1)
	expect(a, []int{1, 2, 3, 4})
	a.print()
	fmt.Println()

	// 用例 2：删除中间节点（O(1) 删除给定节点）。
	mid := a.findNode(2)
	check(mid != nil, "find mid")
	check(a.removeNode(mid), "remove mid")
	expect(a, []int{1, 3, 4})
	check(a.findNode(2) == nil, "mid gone")

	// 用例 3：删除尾节点——有尾哨兵，不该有任何特判分支出错。
	last := a.findNode(4)
	check(last != nil, "find last")
	check(a.removeNode(last), "remove last")
	expect(a, []int{1, 3})
	a.pushBack(5) // 删尾之后再 pushBack 必须接在 3 后面
	expect(a, []int{1, 3, 5})

	// 用例 4：insertAt 的头、尾、中间，以及它与 pushFront/pushBack 的等价。
	b := newDoublyLinkedList()
	check(b.insertAt(0, 10), "insert into empty") // 空表上插，等价 pushBack
	expect(b, []int{10})
	check(b.insertAt(0, 9), "insert head")  // 等价 pushFront
	check(b.insertAt(2, 11), "insert tail") // 等价 pushBack
	check(b.insertAt(2, 99), "insert mid")  // 中间
	check(!b.insertAt(9, 7), "insert oor")
	check(!b.insertAt(-1, 7), "insert neg")
	expect(b, []int{9, 10, 99, 11})

	// 用例 5：removeAt / removeValue 的头、尾、中间。
	check(b.removeAt(0), "remove head")
	expect(b, []int{10, 99, 11})
	check(b.removeAt(2), "remove tail")
	expect(b, []int{10, 99})
	check(b.removeValue(99), "remove value")
	expect(b, []int{10})
	check(!b.removeValue(99), "remove missing")
	check(!b.removeAt(1), "remove oor")
	check(b.n == 1, "size one")

	// 用例 6：空表的每个出口，以及 clear 之后能继续用。
	e := newDoublyLinkedList()
	check(!e.removeNode(nil), "remove nil")
	check(!e.removeAt(0), "removeAt empty")
	check(!e.removeValue(1), "removeValue empty")
	check(e.findNode(1) == nil, "find in empty")
	check(len(e.toSlice()) == 0 && len(e.toSliceReverse()) == 0, "empty view")
	e.pushFront(7)
	e.pushBack(8)
	expect(e, []int{7, 8})
	e.clear()
	check(e.empty() && len(e.toSlice()) == 0, "after clear")
	check(len(e.toSliceReverse()) == 0, "after clear rev")
	e.pushBack(9) // clear 之后还能用
	expect(e, []int{9})
	// 关键点用例：把节点一个个删空，再重新插入，两个哨兵必须重新互指。
	for !e.empty() {
		check(e.removeAt(0), "drain")
	}
	expect(e, []int{})
	e.pushFront(1)
	e.pushFront(2)
	expect(e, []int{2, 1})

	fmt.Println("all tests passed")
}
