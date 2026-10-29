/*
============================================================================
147. 对链表进行插入排序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/insertion-sort-list/
分类：链表 · 插入排序 / 归并排序

给定单个链表的头 head ，使用 插入排序 对链表进行排序，并返回 排序后链表
的头 。

插入排序 算法的步骤:

1. 插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的
   输出列表。
2. 每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列
   中适当的位置，并将其插入。
3. 重复直到所有输入数据插入完为止。

下面是插入排序算法的一个图形示例。部分排序的列表(黑色)最初只包含列表中的
第一个元素。每次迭代时，从输入数据中删除一个元素(红色)，并就地插入已排序
的列表中。

对链表进行插入排序。

示例 1：
输入: head = [4,2,1,3]
输出: [1,2,3,4]

示例 2：
输入: head = [-1,5,3,4,0]
输出: [-1,0,3,4,5]

提示：
- 列表中的节点数在 [1, 5000] 范围内
- -5000 <= Node.val <= 5000
*/
package main

import "fmt"

// 力扣预置的节点定义
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法一：链表插入排序（官方方法，题面指定算法）【最优解】

为什么慢：数组插入排序的代价在「搬移元素」，链表插入只需改指针，插入本身
是 O(1)；但单链表没有前驱指针，每次都要从表头走一遍才能找到插入点，单次
定位 O(n)，n 次插入累计 O(n^2)。输入已经有序时，lastSorted 一次比较就能
跳过，退化成 O(n)，但逆序输入每次都要从头扫。

换个角度：把链表分成「已排序段」和「待插入段」。已排序段用 lastSorted 标记
尾节点，待插入节点是 lastSorted.Next。如果它不小于尾节点，直接接到尾部；
否则从 dummy 开始找第一个严格大于它的位置插进去。

关键点：
  · 先比较 lastSorted.Val <= curr.Val：相等时留在后面，插入排序才是稳定的
  · 插入前必须先把 lastSorted.Next 接到 curr.Next，否则 curr 原来的后继会丢
  · curr 插到 lastSorted 前面时，lastSorted 不动；所以循环末尾统一用
    curr = lastSorted.Next 取下一个待排节点，而不是 curr = curr.Next
  · dummy 节点让 curr 可以插到头节点之前，返回 dummy.Next

以示例 1（head = [4,2,1,3]）为例：
  已排序段 [4]，curr = 2：2 < 4，插到 4 前面 → [2,4]
  curr = 1：1 < 4，插到最前 → [1,2,4]
  curr = 3：3 < 4，插到 4 前面 → [1,2,3,4]

复杂度分析：
  时间 O(n^2)：每插入一个节点最多从头扫描 O(n)
  空间 O(1)：只用了 dummy、lastSorted、curr、prev 几个指针

解法二：自顶向下归并排序【非最优】

为什么慢：解法一的瓶颈是「每次插入都要重新定位」。链表不能随机访问，插入
排序无法用二分把定位降到 O(log n)。

换个角度：既然最终只要求升序，可以放弃插入排序，改用分治：快慢指针把链表
切成两半，递归排好左右两半，再合并两个有序链表。合并只改指针，不需要额外
节点。

关键点：
  · fast 从 head.Next 起步，偶数长度时 slow 停在左半段的最后一个节点，
    两半长度差不超过 1
  · 切分时必须令 slow.Next = nil，否则递归不会终止
  · 合并时用 dummy 节点承接，比较用 <= 保持稳定
  · 渐近上比插入排序更优，但题面明确要求「使用插入排序」，所以它不算本题
    的解；此处保留只为对照，本题以解法一为准；
    若严格按题意提交，应使用解法一

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归栈深度为 log n；节点本身复用，没有新建链表
*/
func insertionSortList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	dummy := &ListNode{}
	dummy.Next = head
	lastSorted := head
	curr := head.Next

	for curr != nil {
		if lastSorted.Val <= curr.Val {
			// 已排序段整体不大于 curr，直接把它并入尾部
			lastSorted = lastSorted.Next
		} else {
			// 从 dummy 起找第一个严格大于 curr 的节点，插在它前面
			prev := dummy
			for prev.Next.Val <= curr.Val {
				prev = prev.Next
			}

			// 先摘下 curr，再接到 prev 后面，顺序反了会丢后继
			lastSorted.Next = curr.Next
			curr.Next = prev.Next
			prev.Next = curr
		}
		curr = lastSorted.Next
	}
	return dummy.Next
}

// 解法二：自顶向下归并排序
func sortListMerge(head *ListNode) *ListNode {
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

	left := sortListMerge(head)
	right := sortListMerge(mid)
	return mergeLists(left, right)
}

// 合并两个有序链表，返回新表头；节点全部复用
func mergeLists(a, b *ListNode) *ListNode {
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

// ===== 测试辅助：数组建表、链表转数组 =====
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
	if got := toSlice(insertionSortList(buildList(input))); !equalInts(got, want) {
		panic(fmt.Sprintf("insertion(%v) = %v, want %v", input, got, want))
	}
	if got := toSlice(sortListMerge(buildList(input))); !equalInts(got, want) {
		panic(fmt.Sprintf("merge(%v) = %v, want %v", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{4, 2, 1, 3}, []int{1, 2, 3, 4})

	// 示例 2
	check([]int{-1, 5, 3, 4, 0}, []int{-1, 0, 3, 4, 5})

	// 边界：空链表（题面保证至少 1 个节点，这里验证健壮性）
	check([]int{}, []int{})

	// 边界：单节点
	check([]int{7}, []int{7})

	// 边界：已经有序，插入排序应只比较尾节点
	check([]int{1, 2, 3, 4, 5}, []int{1, 2, 3, 4, 5})

	// 边界：完全逆序，每次都要插到最前面
	check([]int{5, 4, 3, 2, 1}, []int{1, 2, 3, 4, 5})

	// 边界：全部相同，验证稳定性与不会死循环
	check([]int{3, 3, 3, 3}, []int{3, 3, 3, 3})

	// 边界：重复值夹杂在乱序里
	check([]int{2, 1, 2, 1, 2}, []int{1, 1, 2, 2, 2})

	// 边界：负数与正数混合
	check([]int{0, -5000, 5000, -1, 1}, []int{-5000, -1, 0, 1, 5000})

	fmt.Println("all tests passed")
}
