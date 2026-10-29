/*
============================================================================
234. 回文链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/palindrome-linked-list/
分类：链表 · 回文判断（复制到数组 / 递归 / 快慢指针 + 反转后半）

给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回
true ；否则，返回 false 。

示例 1：
输入：head = [1,2,2,1]
输出：true

示例 2：
输入：head = [1,2]
输出：false

提示：
链表中节点数目在范围 [1, 10^5] 内
0 <= Node.val <= 9

进阶：你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
*/
package main

import "fmt"

// 平台预置的链表节点定义；本地 go run 时需要补上
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法一：把值复制到数组，再用双指针夹逼【非最优】

为什么慢：链表只能单向访问，没法像数组那样从尾部回看，所以先把值抄进数组，
问题立刻退化成「数组是否回文」。代价是 O(n) 的额外空间，不满足题目「进阶」
要求的 O(1) 空间。

换个角度：判断回文需要「同时从两端向中间走」。数组给了随机访问，天然满足；
链表没这个能力，所以要么先把它「数组化」，要么把后半段物理翻转。

关键点：
  · 复制的是 Val，不是节点 —— 比较节点地址永远不相等
  · 双指针用 i < j 而不是 i <= j，奇数长度时中间的节点不需要比较
  · 空链表或单节点直接为真（循环一次都不执行）

以示例 1（[1,2,2,1]）为例：
  数组为 [1,2,2,1]，i=0/j=3 相等，i=1/j=2 相等 → 返回 true

复杂度分析：
  时间 O(n)：复制一遍 + 比较半遍
  空间 O(n)：值切片
*/

// 判断链表是否为回文
func isPalindrome1(head *ListNode) bool {
	vals := make([]int, 0, 16)
	for p := head; p != nil; p = p.Next {
		vals = append(vals, p.Val)
	}

	for i, j := 0, len(vals)-1; i < j; i, j = i+1, j-1 {
		if vals[i] != vals[j] {
			return false
		}
	}

	return true
}

/*
解法二：递归（借调用栈实现「反向遍历」）【非最优】

为什么慢：递归确实能让我们「先走到尾节点，再在回溯时从后往前比较」，看起来
很省事；但这份便利是用调用栈换来的 —— 空间仍是 O(n)，链长上限 10^5 时
还有爆栈风险。

换个角度：回文比较本质是「第 i 个与第 n-1-i 个相等」。递归正好提供了
「反向取节点」的能力：递归函数先处理 Next（向尾部前进），回溯时再拿当前节点
和外部的前向指针 front 比对，然后让 front 前进一步。

关键点：
  · front 必须放在递归函数外面（闭包或包级变量），否则每次回溯拿不到同一份
    进度；Go 里用闭包捕获最自然
  · 比较要在递归调用「之后」做 —— 放在之前就变成正序和自己比了
  · 一旦子问题返回 false 要立刻短路，别继续推进 front

以示例 2（[1,2]）为例：
  递归到 2 的回溯，front 指向 1，2 != 1 → 返回 false

复杂度分析：
  时间 O(n)：每个节点参与一次比较
  空间 O(n)：递归栈深度等于链长
*/

// 判断链表是否为回文
func isPalindrome2(head *ListNode) bool {
	front := head

	// 递归到尾部再回溯，回溯时依次与 front 比对
	var check func(cur *ListNode) bool
	check = func(cur *ListNode) bool {
		if cur == nil {
			return true
		}
		if !check(cur.Next) {
			return false
		}
		if cur.Val != front.Val {
			return false
		}
		front = front.Next
		return true
	}

	return check(head)
}

/*
解法三：快慢指针找中点 + 反转后半段【最优解】

满足题目「进阶」的 O(n) 时间、O(1) 空间，是三个官方解中最优的一个。

为什么慢：解法一把值抄进数组，空间 O(n)；解法二用调用栈反向遍历，空间也是
O(n)。想在 O(1) 里完成比较，就得让「从后往前」这件事不再依赖额外容器。

换个角度：回文的后半段读起来正是前半段的镜像。如果把后半段就地翻转，那么
从「头」和「中点之后」同时向后走，逐个比较就能判断回文 —— 空间只用了几个
指针。翻转完成后比较，比较完再把后半段翻回去，输入结构保持不变。

关键点：
  · 找前半段尾节点时，快指针循环条件是 fast.Next != nil 且
    fast.Next.Next != nil，这样偶数长度停在左中点、奇数长度停在正中间
  · 反转的是 slow.Next 开始的半段，slow 本身留在前半段里
  · 比较循环用 p2 是否走完来控制次数 —— 后半段长度 <= 前半段，
    奇数长度时前半段会多一个正中间的节点，它不需要比较
  · 比较结束后把后半段再反转一次接回去，避免修改输入（官方同样复原）
  · 空链表与单节点要提前返回，否则 endOfFirstHalf 会解引用空 Next

以示例 1（1 → 2 → 2 → 1）为例：
  快慢指针停下时 firstHalfEnd 是第 2 个节点
  反转后半段 2 → 1 得到 1 → 2
  p1 走 1、2；p2 走 1、2，全程相等 → true
  再把后半段翻回 2 → 1 并接回，链表恢复原样

复杂度分析：
  时间 O(n)：找中点 n/2、反转 n/2、比较 n/2、复原 n/2，仍是线性
  空间 O(1)：只用几个指针
*/

// 判断链表是否为回文
func isPalindrome3(head *ListNode) bool {
	if head == nil || head.Next == nil {
		return true
	}

	// 1. 快慢指针找到前半部分的尾节点
	firstHalfEnd := endOfFirstHalf(head)
	// 2. 就地反转后半部分
	secondHalfStart := reverseList234(firstHalfEnd.Next)

	// 3. 从两端向中间比较，p2 走完即结束
	p1, p2 := head, secondHalfStart
	result := true
	for result && p2 != nil {
		if p1.Val != p2.Val {
			result = false
		}
		p1 = p1.Next
		p2 = p2.Next
	}

	// 4. 复原后半部分，保持输入结构不变
	firstHalfEnd.Next = reverseList234(secondHalfStart)

	return result
}

// 停在左中点（偶数）或正中间（奇数）
func endOfFirstHalf(head *ListNode) *ListNode {
	fast, slow := head, head
	for fast.Next != nil && fast.Next.Next != nil {
		fast = fast.Next.Next
		slow = slow.Next
	}
	return slow
}

func reverseList234(head *ListNode) *ListNode {
	var prev *ListNode
	for curr := head; curr != nil; {
		next := curr.Next
		curr.Next = prev
		prev = curr
		curr = next
	}
	return prev
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

// 按链表顺序展开，用于核对「复原」是否真的把链表还原了
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

// 一个用例喂给三种解法
func check(name string, values []int, want bool) {
	if got := isPalindrome1(build(values)); got != want {
		panic(fmt.Sprintf("%s: byArray got %v, want %v", name, got, want))
	}
	if got := isPalindrome2(build(values)); got != want {
		panic(fmt.Sprintf("%s: byRecursion got %v, want %v", name, got, want))
	}

	head3 := build(values)
	if got := isPalindrome3(head3); got != want {
		panic(fmt.Sprintf("%s: twoPointers got %v, want %v", name, got, want))
	}
	// 解法三会翻转再复原，验证链表确实回到原样
	if !equal(toSlice(head3), values) {
		panic(fmt.Sprintf("%s: list was not restored", name))
	}
}

func main() {
	// 示例 1：偶数长度回文
	check("example 1", []int{1, 2, 2, 1}, true)

	// 示例 2：两个不同值，必须在第一对就返回 false
	check("example 2", []int{1, 2}, false)

	// 边界：单节点，天然回文（解法三需提前返回）
	check("single node", []int{7}, true)

	// 边界：奇数长度回文，正中间的节点不参与比较
	check("odd palindrome", []int{1, 2, 1}, true)

	// 边界：奇数长度非回文
	check("odd non-palindrome", []int{1, 2, 3}, false)

	// 边界：全部相同
	check("all same", []int{1, 1, 1, 1}, true)

	// 边界：只有最外层一对不同，验证比较从首尾开始而不是从中间开始
	check("outer pair differs", []int{9, 2, 3, 3, 2, 1}, false)

	// 边界：前半段相同而后半段相反，专防「只比了前半段」
	check("latter half differs", []int{1, 2, 3, 2, 2}, false)

	fmt.Println("all tests passed")
}
