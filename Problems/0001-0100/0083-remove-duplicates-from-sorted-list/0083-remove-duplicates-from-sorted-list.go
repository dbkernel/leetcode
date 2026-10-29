/*
============================================================================
LeetCode 83. 删除排序链表中的重复元素（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-list/
分类：链表 · 一次遍历去重
============================================================================

给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。
返回 已排序的链表 。

示例 1：
输入：head = [1,1,2]
输出：[1,2]

示例 2：
输入：head = [1,1,2,3,3]
输出：[1,2,3]

提示：
链表中节点数目在范围 [0, 300] 内
-100 <= Node.val <= 100
题目数据保证链表已经按升序 排列
*/
package main

import "fmt"

// 平台预置的链表节点定义；本地 go run 时需要补上
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法：一次遍历（当前节点与下一个节点比较）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每个节点访问一次，
时间 O(n) 已是下界，且完全不需要额外空间。

为什么慢：想「收集所有不同值再重建链表」就要 O(n) 额外空间；想「发现重复
就回头找前驱」又得 O(n^2)。而链表本身已经有序，重复元素必然相邻，这个
性质足以让一趟扫描解决问题。

换个角度：站在当前节点上，只需要看它的下一个节点值是否相同：
  · 相同 → 把当前节点的 Next 跨过下一个节点（丢弃重复的那个），当前节点
    本身不动，继续和新的 Next 比
  · 不同 → 当前节点前进一步
因为链表有序，每个值只会连续出现一段，处理完这段就再也不会遇到它。

关键点：
  · 删除时当前节点不动 —— 否则 [1,1,1] 这种连续重复会漏删
  · 只有值不同才前进 —— 这一步是「不重复时才走」，容易写反
  · 不需要哨兵 —— 头节点永远会被保留，这与 203 题不同
  · 循环条件是 cur != nil 且 cur.Next != nil —— 要拿到 Next 才能比

以示例 2（head = [1,1,2,3,3]）为例：
  cur = 1：Next 也是 1 → 跨过 → [1,2,3,3]（cur 仍是第 1 个 1）
  cur = 1：Next 是 2 → 不同，cur 前进到 2
  cur = 2：Next 是 3 → 不同，cur 前进到 3
  cur = 3：Next 也是 3 → 跨过 → [1,2,3]
  cur = 3：Next 为空 → 结束

复杂度分析：
  时间 O(n)：每个节点最多比较一次、被跨越一次
  空间 O(1)：只用一条指针
*/

// 删除排序链表中的重复元素，返回表头
func deleteDuplicates(head *ListNode) *ListNode {
	cur := head

	// 需要拿到 Next 才能比较，所以两个条件都要判
	for cur != nil && cur.Next != nil {
		if cur.Val == cur.Next.Val {
			// 跨过重复节点；cur 不动，可能后面还有相同的值
			cur.Next = cur.Next.Next
		} else {
			cur = cur.Next
		}
	}

	return head
}

// ---- 以下为自测代码，不属于题解主体 ----

// 由切片构造链表，方便写断言
func build(values []int) *ListNode {
	dummy := &ListNode{}
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

// 一个用例 = 输入 + 期望输出
func check(name string, values, want []int) {
	got := toSlice(deleteDuplicates(build(values)))
	if !equal(got, want) {
		panic(fmt.Sprintf("%s: got %v, want %v", name, got, want))
	}
}

func main() {
	// 示例 1：一对重复
	check("example 1", []int{1, 1, 2}, []int{1, 2})

	// 示例 2：两组重复，验证跨过之后 cur 不动
	check("example 2", []int{1, 1, 2, 3, 3}, []int{1, 2, 3})

	// 边界：空链表，不能解引用
	check("empty", []int{}, []int{})

	// 边界：只有一个节点，cur.Next 为空，循环一次都不进
	check("single node", []int{1}, []int{1})

	// 边界：全部相同，连续跨越三次
	check("all equal", []int{1, 1, 1, 1}, []int{1})

	// 边界：没有重复，指针一路前进，链表不应被改动
	check("all distinct", []int{1, 2, 3}, []int{1, 2, 3})

	fmt.Println("all tests passed")
}
