/*
============================================================================
148. 排序链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-list/
分类：链表 · 归并排序 / 堆排序

给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。

示例 1：
输入：head = [4,2,1,3]
输出：[1,2,3,4]

示例 2：
输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]

示例 3：
输入：head = []
输出：[]

提示：
- 链表中节点的数目在范围 [0, 5 * 10^4] 内
- -10^5 <= Node.val <= 10^5

进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序
吗？
*/
package main

import (
	"fmt"
	"sort"
)

// 力扣预置的节点定义
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法一：自顶向下归并排序（官方方法一）【非最优】

为什么慢：插入排序在链表上定位插入点要 O(n)，总共 O(n^2)。要达到
O(n log n)，适合链表的是归并排序：合并只改指针，不需要随机访问。

换个角度：快慢指针找中点，把链表切成两半，递归排好左右两半，再合并两个
有序链表。递归深度是 O(log n)，这部分栈空间就是它比解法二多出来的开销。

关键点：
  · fast 从 head.Next 起步，偶数长度时 slow 落在左半段末尾，两半长度差
    不超过 1
  · 切分时必须令 slow.Next = nil，否则递归不终止
  · 递归终止条件是空表或单节点
  · 合并用 dummy 承接，比较用 <= 保持稳定

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归调用栈；节点本身复用，没有新建链表

解法二：自底向上归并排序（官方方法二）【最优解】

为什么慢：解法一时间已经最优，但递归栈还要 O(log n)。进阶要求常数级空间，
就差在这层栈上。

换个角度：把「递归切半」换成「按固定步长切段」。subLength 从 1 开始，每次
把链表切成若干长度不超过 subLength 的段，两两合并，再让 subLength 翻倍，
直到超过链表长度。整个过程只用了几个游标指针。

关键点：
  · 先把链表长度数出来，否则不知道 subLength 何时停止
  · 每轮用 prev 记住「已合并部分的尾节点」，合并完接回去
  · 切出 head2 后必须把 head1 段尾的 Next 置空，否则 merge 会越过段边界
  · 右半段可能不足 subLength，甚至为空；所有推进都要判空
  · 与解法一同为 O(n log n)，但辅助空间从 O(log n) 降到 O(1)，因此最优

复杂度分析：
  时间 O(n log n)：共 O(log n) 轮，每轮合并 O(n)
  空间 O(1)：只用游标指针，节点原地重连

解法三：堆排序【非最优】

为什么慢：堆排序时间 O(n log n)，与两种归并同阶，本身不慢；慢在空间。
  · 数组上的堆排序可以做到 O(1) 辅助空间，因为下标能直接定位父子节点
  · 链表没有随机访问，必须先遍历一次把节点值收集到切片，才能建堆和下沉；
    这一步就是 O(n) 额外空间
  · 排完后还要把值写回原节点，节点身份不变，只有 Val 被改写

换个角度：把问题降维成「对一个长为 n 的切片做堆排序」，再写回链表。

关键点：
  · 建堆从最后一个非叶节点 n/2 - 1 开始向前下沉，才是 O(n) 建堆
  · 每轮把堆顶换到末尾，堆大小减一后再从根下沉
  · 下沉时先比较左右孩子取较大者，再和父节点比较；相等不交换
  · 与解法二同为 O(n log n) 时间，但多出 O(n) 切片，故不是最优解

复杂度分析：
  时间 O(n log n)：建堆 O(n)，n 次取堆顶并下沉共 O(n log n)
  空间 O(n)：values 切片保存全部节点值；链表本身没有额外分配
*/
func sortListTopDown(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	// fast 从 head.Next 起步，偶数长度时 slow 落在左半段末尾
	slow, fast := head, head.Next
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}

	mid := slow.Next
	slow.Next = nil

	return mergeTwo(sortListTopDown(head), sortListTopDown(mid))
}

func sortListBottomUp(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	length := 0
	for p := head; p != nil; p = p.Next {
		length++
	}

	dummy := &ListNode{}
	dummy.Next = head
	for subLength := 1; subLength < length; subLength <<= 1 {
		prev, curr := dummy, dummy.Next
		for curr != nil {
			// 切出长度不超过 subLength 的左半段
			head1 := curr
			for i := 1; i < subLength && curr.Next != nil; i++ {
				curr = curr.Next
			}

			head2 := curr.Next
			curr.Next = nil
			curr = head2

			// 再切出长度不超过 subLength 的右半段
			for i := 1; i < subLength && curr != nil && curr.Next != nil; i++ {
				curr = curr.Next
			}

			var next *ListNode
			if curr != nil {
				next = curr.Next
				curr.Next = nil
			}

			prev.Next = mergeTwo(head1, head2)
			for prev.Next != nil {
				prev = prev.Next
			}
			curr = next
		}
	}
	return dummy.Next
}

func sortListHeap(head *ListNode) *ListNode {
	values := []int{}
	for p := head; p != nil; p = p.Next {
		values = append(values, p.Val)
	}

	n := len(values)
	// 从最后一个非叶节点开始下沉，建大根堆是 O(n)
	for i := n/2 - 1; i >= 0; i-- {
		siftDown(values, i, n)
	}

	// 每轮把堆顶换到末尾，缩小堆后重新下沉
	for end := n - 1; end > 0; end-- {
		values[0], values[end] = values[end], values[0]
		siftDown(values, 0, end)
	}

	// 节点顺序不变，只把排好的值写回
	p := head
	for _, v := range values {
		p.Val = v
		p = p.Next
	}
	return head
}

func mergeTwo(a, b *ListNode) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for a != nil && b != nil {
		if a.Val <= b.Val {
			tail.Next = a
			a = a.Next
		} else {
			tail.Next = b
			b = b.Next
		}
		tail = tail.Next
	}
	if a != nil {
		tail.Next = a
	} else {
		tail.Next = b
	}
	return dummy.Next
}

func siftDown(a []int, i, n int) {
	for {
		left := 2*i + 1
		right := left + 1
		largest := i
		if left < n && a[left] > a[largest] {
			largest = left
		}
		if right < n && a[right] > a[largest] {
			largest = right
		}
		if largest == i {
			break
		}
		a[i], a[largest] = a[largest], a[i]
		i = largest
	}
}

// ===== 测试辅助：切片建表、链表转切片 =====
func buildList(values []int) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for _, v := range values {
		tail.Next = &ListNode{Val: v}
		tail = tail.Next
	}
	return dummy.Next
}

func toSlice(head *ListNode) []int {
	values := []int{}
	for p := head; p != nil; p = p.Next {
		values = append(values, p.Val)
	}
	return values
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

func check(input, want []int) {
	if got := toSlice(sortListTopDown(buildList(input))); !equalInts(got, want) {
		panic(fmt.Sprintf("top-down(%v) = %v, want %v", input, got, want))
	}
	if got := toSlice(sortListBottomUp(buildList(input))); !equalInts(got, want) {
		panic(fmt.Sprintf("bottom-up(%v) = %v, want %v", input, got, want))
	}
	if got := toSlice(sortListHeap(buildList(input))); !equalInts(got, want) {
		panic(fmt.Sprintf("heap(%v) = %v, want %v", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{4, 2, 1, 3}, []int{1, 2, 3, 4})

	// 示例 2
	check([]int{-1, 5, 3, 4, 0}, []int{-1, 0, 3, 4, 5})

	// 示例 3：空链表
	check([]int{}, []int{})

	// 边界：单节点
	check([]int{7}, []int{7})

	// 边界：已经有序
	check([]int{1, 2, 3, 4, 5}, []int{1, 2, 3, 4, 5})

	// 边界：完全逆序
	check([]int{5, 4, 3, 2, 1}, []int{1, 2, 3, 4, 5})

	// 边界：全部相同，验证合并不会丢掉重复节点
	check([]int{3, 3, 3, 3}, []int{3, 3, 3, 3})

	// 边界：重复值夹杂在乱序里
	check([]int{2, 1, 2, 1, 2}, []int{1, 1, 2, 2, 2})

	// 边界：两个节点，验证自底向上时右半段可能为空
	check([]int{2, 1}, []int{1, 2})

	// 边界：负数与正数混合
	check([]int{0, -100000, 100000, -1, 1}, []int{-100000, -1, 0, 1, 100000})

	// 边界：奇数长度，验证按长度切分的最后一段不足 subLength
	check([]int{5, 1, 4, 2, 3}, []int{1, 2, 3, 4, 5})

	// 规模用例：1000 个乱序节点，与标准库排序结果对照
	input := make([]int, 1000)
	for i := range input {
		input[i] = (i*37)%1000 - 500
	}
	want := append([]int(nil), input...)
	sort.Ints(want)
	check(input, want)

	fmt.Println("all tests passed")
}
