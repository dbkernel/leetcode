/*
============================================================================
单链表（Singly Linked List）
分类：手写数据结构 · 线性结构

操作复杂度：
  pushFront    O(1)   插到哑节点后面，不用判断空表
  pushBack     O(1)   靠 tail 指针直接接在尾部
  insertAt     O(n)   先走到 index-1 再改指针
  removeAt     O(n)   同样要先走到前驱
  removeValue  O(n)   遍历找第一个值相等的节点
  find         O(n)   返回下标，找不到返回 -1
  reverse      O(n)   就地三指针，额外空间 O(1)
  toSlice      O(n)
  size/empty   O(1)
============================================================================

用途：
  面试最高频的手写结构。所有「改指针」的题都建在它上面：反转、找中点、
  判环、合并两个有序链表、删除倒数第 k 个。

核心思想：
  每个节点只记 next。头插和删头本来要改外部的 head 指针、到处是 if，
  于是在最前面放一个不存数据的哑节点（sentinel），所有位置就统一成
  「改前一个节点的 next」，头尾一视同仁；再用 tail 指针把尾插降到 O(1)。

关键点（不变量）：
  · head 永远是哑节点，真实元素从 head.next 开始，size 只数真实节点；
  · 空链表时 tail == head，凡是可能改动尾巴的地方都要维护这条；
  · Go 没有手动释放，但改指针前仍要先存 next，否则链子就断了；
  · 删掉的正好是 tail 时要让 tail 退回 prev，否则下次 pushBack 会把
    新节点挂到链表之外，长度和内容立刻对不上。

手推一遍：
  链表 1 -> 2 -> 3（哑节点 d 在最前），执行 removeValue(1)：
  1. prev=d 时 d.next=1 命中，cur=节点 1，先存 nxt = 节点 2；
  2. d.next = 节点 2；cur 不是 tail（tail 还是 3），可以安全丢弃 cur；
  3. 得到 2 -> 3，size 从 3 变 2。
  再执行 reverse：prev=nil、cur=2，每次先把 next 存下来再反向箭头，
  三轮之后链表变 3 -> 2，tail 指向原来的头节点 2。

复杂度分析：
  时间：头尾插入与删除 O(1)，按下标/按值操作 O(n)，反转 O(n)。
  空间：每个元素一个节点，多一个指针字段，总计 O(n)；reverse 只用
    三个指针，额外 O(1)。
  取舍：没有随机访问，按下标访问是 O(n)，换来头尾插入删除 O(1)。
*/

package main

import "fmt"

type node struct {
	val  int
	next *node
}

type SinglyLinkedList struct {
	head *node // 哑节点，不存数据
	tail *node // 指向最后一个真实节点，空表时等于 head
	n    int   // 真实节点个数
}

func newSinglyLinkedList() *SinglyLinkedList {
	dummy := &node{}
	return &SinglyLinkedList{head: dummy, tail: dummy}
}

func (l *SinglyLinkedList) size() int { return l.n }

func (l *SinglyLinkedList) empty() bool { return l.n == 0 }

func (l *SinglyLinkedList) pushFront(v int) {
	n := &node{val: v}
	n.next = l.head.next
	l.head.next = n
	if l.tail == l.head {
		l.tail = n // 原本空表，新节点同时也是尾
	}
	l.n++
}

func (l *SinglyLinkedList) pushBack(v int) {
	n := &node{val: v}
	l.tail.next = n // 空表时 tail 就是哑节点，这一句照样成立
	l.tail = n
	l.n++
}

func (l *SinglyLinkedList) insertAt(index, v int) bool {
	// index == size 等价于尾插；越界直接拒绝。
	if index < 0 || index > l.n {
		return false
	}
	prev := l.head
	for i := 0; i < index; i++ {
		prev = prev.next
	}
	n := &node{val: v}
	n.next = prev.next
	prev.next = n
	if n.next == nil {
		l.tail = n // 接在了最后，尾指针跟着走
	}
	l.n++
	return true
}

func (l *SinglyLinkedList) removeAt(index int) bool {
	if index < 0 || index >= l.n {
		return false
	}
	prev := l.head
	for i := 0; i < index; i++ {
		prev = prev.next
	}
	cur := prev.next
	prev.next = cur.next
	if cur == l.tail {
		l.tail = prev // 删的是尾巴，尾指针回退一格
	}
	l.n--
	return true
}

func (l *SinglyLinkedList) removeValue(v int) bool {
	prev := l.head
	// 停在「下一个就是目标」的位置，这样拿得到前驱
	for prev.next != nil && prev.next.val != v {
		prev = prev.next
	}
	if prev.next == nil {
		return false // 值不存在
	}
	cur := prev.next
	prev.next = cur.next
	if cur == l.tail {
		l.tail = prev
	}
	l.n--
	return true
}

func (l *SinglyLinkedList) find(v int) int {
	i := 0
	for p := l.head.next; p != nil; p = p.next {
		if p.val == v {
			return i
		}
		i++
	}
	return -1
}

func (l *SinglyLinkedList) reverse() {
	var prev *node
	cur := l.head.next
	if cur != nil {
		l.tail = cur // 反转后原来的头变成尾
	}
	for cur != nil {
		nxt := cur.next // 三指针：先存后改，才不会断链
		cur.next = prev
		prev = cur
		cur = nxt
	}
	l.head.next = prev // 哑节点重新接上新的头
}

func (l *SinglyLinkedList) toSlice() []int {
	out := []int{}
	for p := l.head.next; p != nil; p = p.next {
		out = append(out, p.val)
	}
	return out
}

func (l *SinglyLinkedList) clear() {
	l.head.next = nil
	l.tail = l.head
	l.n = 0
}

func (l *SinglyLinkedList) print() {
	fmt.Print("[")
	for i, v := range l.toSlice() {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(v)
	}
	fmt.Println("]")
}

func check(cond bool, msg string) {
	if !cond {
		panic("assert failed: " + msg)
	}
}

func expect(l *SinglyLinkedList, want []int) {
	got := l.toSlice()
	check(len(got) == len(want), "length mismatch")
	for i := range want {
		check(got[i] == want[i], fmt.Sprintf("index %d mismatch", i))
	}
	check(l.n == len(want), "size mismatch")
}

func main() {
	// 用例 1：头插尾插混用，顺序与 size 都要对。
	a := newSinglyLinkedList()
	check(a.empty() && a.size() == 0, "empty init")
	a.pushBack(2)
	a.pushFront(1)
	a.pushBack(3)
	expect(a, []int{1, 2, 3})
	check(a.find(1) == 0 && a.find(3) == 2 && a.find(9) == -1, "find")
	a.print()

	// 用例 2：insertAt 的头、尾、中间与越界。
	check(a.insertAt(0, 0), "insert head")  // 头
	check(a.insertAt(4, 4), "insert tail")  // 尾，index == size
	check(a.insertAt(2, 99), "insert mid")  // 中间
	check(!a.insertAt(99, 7), "insert oor") // 越界
	check(!a.insertAt(-1, 7), "insert neg")
	expect(a, []int{0, 1, 99, 2, 3, 4})
	check(a.size() == 6, "size after insert")

	// 用例 3：删头、删尾、删中间，size 一并核对。
	check(a.removeAt(0), "remove head")
	expect(a, []int{1, 99, 2, 3, 4})
	check(a.removeAt(4), "remove tail") // 删的是尾节点
	expect(a, []int{1, 99, 2, 3})
	check(a.removeAt(1), "remove mid")
	expect(a, []int{1, 2, 3})
	check(!a.removeAt(3), "remove oor")
	check(!a.removeAt(-1), "remove neg")

	// 用例 3b：关键点用例——删掉尾节点后 pushBack 必须落在新的尾上。
	a.pushBack(4)
	expect(a, []int{1, 2, 3, 4})

	// 用例 4：重复值只删第一个匹配。
	b := newSinglyLinkedList()
	b.pushBack(1)
	b.pushBack(2)
	b.pushBack(1)
	b.pushBack(3)
	check(b.removeValue(1), "remove first 1") // 只删最前面那个 1
	expect(b, []int{2, 1, 3})
	check(b.removeValue(1), "remove second 1")
	expect(b, []int{2, 3})
	check(!b.removeValue(1), "remove missing")
	check(b.size() == 2, "size after removeValue")

	// 用例 5：反转，反转后还能继续头尾插入（tail 必须同步）。
	c := newSinglyLinkedList()
	for i := 1; i <= 4; i++ {
		c.pushBack(i)
	}
	c.reverse()
	expect(c, []int{4, 3, 2, 1})
	c.pushBack(9) // 反转后 tail 若是旧的，这里就会挂错位置
	expect(c, []int{4, 3, 2, 1, 9})
	c.reverse()
	expect(c, []int{9, 1, 2, 3, 4})

	// 用例 6：空链表与单元素链表。
	e := newSinglyLinkedList()
	check(!e.removeValue(1), "remove from empty")
	check(!e.removeAt(0), "removeAt empty")
	e.reverse() // 空表反转不能崩
	check(e.empty() && e.size() == 0, "still empty")
	e.pushBack(5)
	e.reverse()
	expect(e, []int{5})
	check(e.find(5) == 0, "find single")
	e.clear()
	check(e.empty() && len(e.toSlice()) == 0, "after clear")
	e.pushBack(6) // clear 之后还能用
	expect(e, []int{6})

	fmt.Println("all tests passed")
}
