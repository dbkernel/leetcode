/*
============================================================================
328. 奇偶链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/odd-even-linked-list/
分类：链表 · 按位置分组（分离后拼接）

给定单链表的头节点 head ，将所有索引为奇数的节点和索引为偶数的节点分别
分组，保持它们原有的相对顺序，然后把偶数索引节点分组连接到奇数索引节点分组
之后，返回重新排序的链表。

第一个节点的索引被认为是 奇数 ， 第二个节点的索引为 偶数 ，以此类推。

请注意，偶数组和奇数组内部的相对顺序应该与输入时保持一致。

你必须在 O(1) 的额外空间复杂度和 O(n) 的时间复杂度下解决这个问题。

示例 1:
输入: head = [1,2,3,4,5]
输出: [1,3,5,2,4]

示例 2:
输入: head = [2,1,3,5,6,4,7]
输出: [2,3,6,7,1,5,4]

提示:
n == 链表中的节点数
0 <= n <= 10^4
-10^6 <= Node.val <= 10^6
*/
package main

import "fmt"

// 平台预置的链表节点定义；本地 go run 时需要补上
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法：分离节点后合并（奇链表 + 偶链表一次成型）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：题目直接要求 O(n) 时间
与 O(1) 额外空间，本解一次遍历、只用几个指针，正好满足。

为什么慢：最直观的想法是「把值按奇偶位置抄成两个切片，再依次写回链表」——
时间 O(n)，但要多开 O(n) 空间，不满足题目要求；而且要改值而不是改链，
节点身份也随之改变。

换个角度：奇偶位置的节点本来就在一条链上交替出现。只要在遍历时把 Next
指针重新接线，让「奇数位节点」连成一条链、「偶数位节点」连成另一条链，
最后把偶链接到奇链尾部即可 —— 一条链一次遍历，不需要任何额外容器。

关键点：
  · 头节点是奇链的头，head.Next 是偶链的头；evenHead 必须提前存下来，
    否则遍历到后面就找不回偶链的起点了
  · 循环条件是 even != nil && even.Next != nil ——
    偶节点一定比奇节点少，偶链走完就说明分离完毕
  · 每轮先接奇、再接偶：odd.Next = even.Next 之后，odd 前进一格；
    此时 odd 正是 even 的后继，所以 even.Next = odd.Next 才是「隔一个」
  · 循环结束时 odd 指向奇链尾，最后一步 odd.Next = evenHead 把两链接上
  · 空链表要在最开始就返回，否则 even = head.Next 会解引用空指针

以示例 1（1 → 2 → 3 → 4 → 5）为例：
  evenHead = 2，odd = 1，even = 2
  第一轮：odd.Next = 3，odd = 3；even.Next = 4，even = 4
  第二轮：odd.Next = 5，odd = 5；even.Next = nil，even = nil
  even 为 nil，循环结束，odd.Next = 2 → 1 → 3 → 5 → 2 → 4

以示例 2（2 → 1 → 3 → 5 → 6 → 4 → 7）为例：
  奇链为 2 → 3 → 6 → 7，偶链为 1 → 5 → 4
  拼接后 → 2 → 3 → 6 → 7 → 1 → 5 → 4

复杂度分析：
  时间 O(n)：每个节点只被访问一次
  空间 O(1)：只用 odd / even / evenHead 三个指针
*/

// 把奇数位节点与偶数位节点分组后拼接
func oddEvenList(head *ListNode) *ListNode {
	// 空链表必须先返回，否则 even = head.Next 会解引用空指针
	if head == nil {
		return head
	}

	// 偶链的头要提前保存，遍历完就找不回来了
	evenHead := head.Next
	odd, even := head, evenHead

	// 偶节点比奇节点少，偶链走完即分离完毕
	for even != nil && even.Next != nil {
		// 奇节点隔一个指向下一个奇节点
		odd.Next = even.Next
		odd = odd.Next

		// 此时 odd 正是 even 的后继，再隔一个就是下一个偶节点
		even.Next = odd.Next
		even = even.Next
	}

	// 偶链整体接到奇链尾部
	odd.Next = evenHead
	return head
}

// ---- 以下为自测代码，不属于题解主体 ----

// 由切片构造链表，方便写断言
func build(values []int) *ListNode {
	dummy := new(ListNode)
	tail := dummy
	for _, v := range values {
		tail.Next = &ListNode{Val: v}
		tail = tail.Next
	}
	return dummy.Next
}

// 按链表顺序展开，用于和期望序列比较
func toSlice(head *ListNode) []int {
	values := make([]int, 0, 16)
	for p := head; p != nil; p = p.Next {
		values = append(values, p.Val)
	}
	return values
}

func equal(a, b []int) bool {
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

// 一个用例 = 输入序列 + 期望序列
func check(name string, values, want []int) {
	got := toSlice(oddEvenList(build(values)))
	if !equal(got, want) {
		panic(fmt.Sprintf("%s: got %v, want %v", name, got, want))
	}
}

func main() {
	// 示例 1：奇数长度，最后一个节点落在奇链上
	check("example 1", []int{1, 2, 3, 4, 5}, []int{1, 3, 5, 2, 4})

	// 示例 2：奇数长度且值无规律，专防「按值排序」的误写
	check("example 2", []int{2, 1, 3, 5, 6, 4, 7}, []int{2, 3, 6, 7, 1, 5, 4})

	// 边界：空链表，必须直接返回 nil
	check("empty", []int{}, []int{})

	// 边界：单节点，even 为 nil，循环一次都不进
	check("single node", []int{1}, []int{1})

	// 边界：两个节点，偶链只有一个节点
	check("two nodes", []int{1, 2}, []int{1, 2})

	// 边界：偶数长度，最后一个节点落在偶链上
	check("even length", []int{1, 2, 3, 4}, []int{1, 3, 2, 4})

	// 边界：三节点，验证最后一步拼接
	check("three nodes", []int{1, 2, 3}, []int{1, 3, 2})

	fmt.Println("all tests passed")
}
