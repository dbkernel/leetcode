/*
============================================================================
LeetCode 203. 移除链表元素（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-linked-list-elements/
分类：链表 · 删除指定值（递归 / 迭代）
============================================================================

给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足
Node.val == val 的节点，并返回 新的头节点 。

示例 1：
输入：head = [1,2,6,3,4,5,6], val = 6
输出：[1,2,3,4,5]

示例 2：
输入：head = [], val = 1
输出：[]

示例 3：
输入：head = [7,7,7,7], val = 7
输出：[]

提示：
列表中的节点数目在范围 [0, 10^4] 内
1 <= Node.val <= 50
0 <= val <= 50
*/
package main

import "fmt"

// 平台预置的链表节点定义；本地 go run 时需要补上
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法一：递归（先处理后面的，再决定自己留不留）【非最优】

为什么慢：思路不慢，时间仍是 O(n)；慢在空间 —— 递归深度等于链表长度，
最坏 10^4 层。Go 的 goroutine 栈会按需增长，虽然不像 C++ 那样直接爆栈，
但这仍然是 O(n) 的额外空间。

换个角度：整条链「删掉 val 之后长什么样」，等于「我先看后面删完的结果，
再判断我自己要不要留下」。问题结构相同、规模小一，可以直接递归。

关键点：
  · 递归出口是空节点 —— 空链表删完还是空
  · 先递归再判断自己 —— 顺序反了会丢掉后续的处理结果
  · 自己等于 val 时直接返回子问题的结果（等于把自己摘掉）

以示例 1（head = [1,2,6,3,4,5,6], val = 6）为例：
  递归到最深处返回空，回溯时 6 被丢弃、5 保留、4 保留 …… 依此类推
  最终得到 [1,2,3,4,5]

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(n)：递归深度等于链表长度
*/

// 删除所有值为 val 的节点，返回新的头节点
func removeElements1(head *ListNode, val int) *ListNode {
	// 空链表删完还是空
	if head == nil {
		return nil
	}

	// 先让后面的部分删干净
	head.Next = removeElements1(head.Next, val)

	// 再决定自己留不留：等于 val 就把自己摘掉
	if head.Val == val {
		return head.Next
	}
	return head
}

/*
解法二：迭代（哨兵节点 + 一个前驱指针）【最优解】

官方两解中空间最优的一个，也是本题的最优解：时间 O(n) 已是下界，
空间 O(1)，不占用调用栈，长链表也不会爆栈。

为什么慢：解法一的额外开销全在调用栈上。改成循环就能把这份空间省掉。

换个角度：遍历时始终握着「前一个节点」，看到下一个节点该删，就把它跨过去。
麻烦的只是头节点没有前驱 —— 加一个哨兵就统一了。

关键点：
  · 必须用哨兵 —— 否则「头节点等于 val」要单独写一段 for 处理，
    连续多个头节点都要删时容易漏
  · 只有「没删」时才移动 prev —— 删完之后 prev 还站在原处，
    下一个节点可能也要删（如 [7,7,7]）
  · 循环条件是 cur != nil，不是 cur.Next != nil —— 必须检查到最后一个节点本身

以示例 1（head = [1,2,6,3,4,5,6], val = 6）为例：
  哨兵指向 1，prev 停在哨兵
  1 不等于 6 → prev 移到 1；2 不等于 6 → prev 移到 2
  6 等于 6 → 把 2 的 Next 指向 3，prev 仍停在 2
  继续扫过 3、4、5，最后 5 的 Next 跳过末尾的 6 → [1,2,3,4,5]

复杂度分析：
  时间 O(n)：每个节点最多被检查一次
  空间 O(1)：只用哨兵与两条指针
*/

// 删除所有值为 val 的节点，返回新的头节点
func removeElements2(head *ListNode, val int) *ListNode {
	// 哨兵让「删除头节点」和「删除中间节点」走同一套逻辑
	dummy := &ListNode{Next: head}
	prev, cur := dummy, head

	for cur != nil {
		if cur.Val == val {
			// 跨过待删节点；prev 不动，下一个节点可能也要删
			prev.Next = cur.Next
		} else {
			prev = cur
		}
		cur = cur.Next
	}

	return dummy.Next
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

// 一个用例喂给两种解法：递归与迭代必须得到同一条链表
func check(name string, values []int, val int, want []int) {
	// 解法会复用输入节点，各用一份新建的链表
	got1 := toSlice(removeElements1(build(values), val))
	got2 := toSlice(removeElements2(build(values), val))

	if !equal(got1, want) {
		panic(fmt.Sprintf("%s: recursive got %v, want %v", name, got1, want))
	}
	if !equal(got2, want) {
		panic(fmt.Sprintf("%s: iterative got %v, want %v", name, got2, want))
	}
}

func main() {
	// 示例 1：中间与末尾都有待删节点
	check("example 1", []int{1, 2, 6, 3, 4, 5, 6}, 6, []int{1, 2, 3, 4, 5})

	// 示例 2：空链表，不能解引用空指针
	check("example 2", []int{}, 1, []int{})

	// 示例 3：全部待删，结果必须为空
	check("example 3", []int{7, 7, 7, 7}, 7, []int{})

	// 边界：头部连续多个待删节点，专防「只删一个头」的写法
	check("leading duplicates", []int{6, 6, 1, 6}, 6, []int{1})

	// 边界：没有节点需要删除，链表应原样返回
	check("nothing to remove", []int{1, 2, 3}, 4, []int{1, 2, 3})

	fmt.Println("all tests passed")
}
