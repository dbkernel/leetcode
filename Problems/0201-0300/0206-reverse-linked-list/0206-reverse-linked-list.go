/*
============================================================================
206. 反转链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-linked-list/
分类：链表 · 指针反转（迭代 / 递归）

给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。

示例 1：
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]

示例 2：
输入：head = [1,2]
输出：[2,1]

示例 3：
输入：head = []
输出：[]

提示：
链表中节点的数目范围是 [0, 5000]
-5000 <= Node.val <= 5000

进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
*/
package main

import "fmt"

// 平台预置的链表节点定义；本地 go run 时需要补上
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法一：迭代（逐个把 Next 指回前驱）【最优解】

官方两解中空间最优的一个，因此是本题的最优解：时间 O(n) 已是下界
（每个节点的 Next 都要改一次），空间 O(1)。

为什么慢：最直接的想法是「把节点值倒着放进数组再写回去」——时间虽然也是
O(n)，但要多开 O(n) 空间，而且题目要求的是改链而不是改值（节点身份会变）。

换个角度：反转的本质就是「让每个节点的 Next 指向它的前驱」。难点在于：改掉
Next 之后就找不到后继了，所以必须先保存它，再动指针。

关键点：
  · 必须先把 curr.Next 存到 next，再改 curr.Next，否则后半个链就丢了
  · prev 零值就是 nil —— 反转头节点的 Next 要指向空
  · 循环结束后返回 prev（不是 curr，此时 curr 已经是 nil）
  · 空链表不进循环，直接返回 nil，行为正确无需特判

以示例 1（1 → 2 → 3 → 4 → 5）为例：
  prev=nil，curr=1：存 next=2，1.Next=nil，prev=1，curr=2
  prev=1，curr=2：存 next=3，2.Next=1，prev=2，curr=3
  依此推进到 5.Next=4，最后 prev=5，curr=nil → 返回 5

复杂度分析：
  时间 O(n)：每个节点处理一次
  空间 O(1)：只用 prev / curr / next 三个指针
*/

// 反转链表，返回新的头节点
func reverseList1(head *ListNode) *ListNode {
	var prev *ListNode
	curr := head

	for curr != nil {
		// 先存后继，再改指针；顺序反了会丢掉后半条链
		next := curr.Next
		curr.Next = prev
		prev = curr
		curr = next
	}

	// 结束时 curr 为 nil，新的头是 prev
	return prev
}

/*
解法二：递归（先反转后面，再把自己接到尾巴上）【非最优】

为什么慢：反转次数一样，时间 O(n)；慢在空间 —— 递归深度等于链长，最坏要
n 层栈空间。链长上限 5000，虽然平台能过，但不如迭代稳妥。

换个角度：把「反转整条链」拆成「反转 head 之后的部分」，再把 head 接到反转
结果的末尾。假设 head.Next 及其后的部分已经反转好，此时 head.Next 恰好
就是反转后子链的尾节点，于是只要令 head.Next.Next = head 即可。

关键点：
  · 终止条件是 head 为空或只剩一个节点
  · 必须先递归拿到 newHead，再改两条指针；先改会破坏子链结构
  · head.Next 必须置 nil —— 否则原本的头节点会成为环的一部分
  · 返回值是 newHead 而不是 head（head 已经变成尾节点）

以示例 2（1 → 2）为例：
  递归到 2 时只剩一个节点，返回 2
  回到 1：1.Next.Next = 1 即 2.Next = 1；1.Next = nil
  返回 2 → 得到 2 → 1

复杂度分析：
  时间 O(n)：每个节点的 Next 改一次
  空间 O(n)：递归栈深度等于链长
*/

// 反转链表，返回新的头节点
func reverseList2(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	// 先反转后面，拿到新的头（即原链的尾节点）
	newHead := reverseList2(head.Next)

	// 反转后 head.Next 是子链的尾节点，让它指回 head
	head.Next.Next = head
	// 原头节点成为尾节点，必须断开，否则会成环
	head.Next = nil

	return newHead
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

// 一个用例喂给两种解法：迭代与递归必须得到同一条链表
func check(name string, values, want []int) {
	// 两种解法都会改动节点，各用一份新建的链表
	got1 := toSlice(reverseList1(build(values)))
	got2 := toSlice(reverseList2(build(values)))

	if !equal(got1, want) {
		panic(fmt.Sprintf("%s: iterative got %v, want %v", name, got1, want))
	}
	if !equal(got2, want) {
		panic(fmt.Sprintf("%s: recursive got %v, want %v", name, got2, want))
	}
}

func main() {
	// 示例 1：五个节点
	check("example 1", []int{1, 2, 3, 4, 5}, []int{5, 4, 3, 2, 1})

	// 示例 2：两个节点，验证尾节点的 Next 被正确置空
	check("example 2", []int{1, 2}, []int{2, 1})

	// 示例 3：空链表，必须返回 nil 而不是崩溃
	check("example 3", []int{}, []int{})

	// 边界：单节点，递归走终止分支
	check("single node", []int{7}, []int{7})

	// 边界：含重复值的三段链，验证只改指针不改值
	check("three nodes", []int{-1, 0, -1}, []int{-1, 0, -1})

	// 边界：严格递增的三节点，防止出现环（反序列化时会死循环）
	check("no cycle", []int{1, 2, 3}, []int{3, 2, 1})

	fmt.Println("all tests passed")
}
