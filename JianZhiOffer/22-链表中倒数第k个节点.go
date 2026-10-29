/*
============================================================================
LCR 140 / 剑指 Offer 22. 训练计划 II（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/
分类：链表 · 倒数第 k 个节点（两次遍历 / 快慢双指针）

说明：本题原名「链表中倒数第 k 个节点」，leetcode.cn 现已改写成场景题
「训练计划 II」，函数名为 trainingPlan，参数名改为 cnt。
============================================================================

给定一个头节点为 head 的链表用于记录一系列核心肌群训练项目编号，请查找并
返回倒数第 cnt 个训练项目编号对应的节点。

示例 1：
输入：head = [2,4,7,8], cnt = 1
输出：8

提示：
1 <= head.length <= 100
0 <= head[i] <= 100
1 <= cnt <= head.length
*/
package main

import "fmt"

// 平台预置的链表节点定义；本地 go run 时需要补上
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法一：顺序查找（先数长度，再走到第 n - cnt 个）【非最优】

为什么慢：要走两遍。第一遍只为了得到长度 n，第二遍才真正定位；对链表这种
只能单向遍历的结构，等于把前 n - cnt 个节点白扫了一次。时间仍是 O(n)，
但常数上明显多一倍。

换个角度：倒数第 cnt 个，就是正数第 n - cnt 个（从 0 开始计数）。只要知道
n，一条循环走过去即可。

关键点：
  · 第 n - cnt 个是 0 基下标，循环条件写成 n > cnt 时每轮减一
  · cnt 是 1 基的（倒数第 1 个是尾节点），不要多走一步
  · 题目保证 1 <= cnt <= n，无需处理越界

以示例 1（head = [2,4,7,8], cnt = 1）为例：
  第一遍数出 n = 4
  从 head 走 n - cnt = 3 步 → 停在 8

复杂度分析：
  时间 O(n)：两遍遍历
  空间 O(1)：只用两个指针
*/

// 返回倒数第 cnt 个节点
func trainingPlan1(head *ListNode, cnt int) *ListNode {
	n := 0
	for node := head; node != nil; node = node.Next {
		n++
	}

	// 倒数第 cnt 个 = 正数第 n - cnt 个（0 基）
	node := head
	for ; n > cnt; n-- {
		node = node.Next
	}

	return node
}

/*
解法二：快慢双指针（只走一遍）【最优解】

官方两解中只需一次遍历的一个，因此是本题的最优解：时间 O(n)（每个节点只被
访问一次），空间 O(1)。

为什么慢：解法一的第二遍是从头重新走。其实「倒数第 cnt 个」这个位置信息，
可以用两指针之间的固定间距表达出来，不必先知道 n。

换个角度：让 fast 先走 cnt 步，此时 fast 与 slow 之间正好隔着 cnt 个节点。
之后两者同步前进，当 fast 越过尾节点变成空指针时，slow 与 fast 相隔 cnt 步，
而 fast 已经走完了整条链表 —— slow 停下的位置正是倒数第 cnt 个。

关键点：
  · fast 先走 cnt 步，不是 cnt - 1 步；走完之后 fast 指向正数第 cnt + 1 个
  · 同步循环的条件是 fast != nil（而非 fast.Next != nil），
    因为 fast 要走到「尾节点的下一个」
  · 第一个循环要带 cnt > 0，避免 cnt 超过链长时把 fast 走穿
  · cnt == n 时 fast 先走成空，同步循环一次都不进，slow 就是 head

以示例 1（head = [2,4,7,8], cnt = 1）为例：
  fast 走 1 步 → 指向 4（此时与 slow 相隔 1 个节点）
  两者同步前进：fast 4→7→8→空，slow 2→4→7→8
  fast 变空时 slow 停在 8 —— 倒数第 1 个

复杂度分析：
  时间 O(n)：fast 走 n 步，slow 走 n - cnt 步，合计不到 2n
  空间 O(1)：只用两个指针
*/

// 返回倒数第 cnt 个节点
func trainingPlan2(head *ListNode, cnt int) *ListNode {
	fast, slow := head, head

	// 先让 fast 与 slow 之间拉开 cnt 个节点的间距
	for fast != nil && cnt > 0 {
		fast = fast.Next
		cnt--
	}

	// fast 越过尾节点时，slow 正好停在倒数第 cnt 个
	for fast != nil {
		fast = fast.Next
		slow = slow.Next
	}

	return slow
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

// 从某个节点开始展开，用于核对返回的节点及其后续
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

// 一个用例喂给两种解法：两次遍历与双指针必须返回同一个节点
func check(name string, values []int, cnt int, want []int) {
	got1 := toSlice(trainingPlan1(build(values), cnt))
	got2 := toSlice(trainingPlan2(build(values), cnt))

	if !equal(got1, want) {
		panic(fmt.Sprintf("%s: byLength got %v, want %v", name, got1, want))
	}
	if !equal(got2, want) {
		panic(fmt.Sprintf("%s: twoPointers got %v, want %v", name, got2, want))
	}
}

func main() {
	// 示例 1：cnt = 1，返回尾节点
	check("example 1", []int{2, 4, 7, 8}, 1, []int{8})

	// 边界：cnt == n，返回头节点，验证 fast 先走成空时 slow 不动
	check("cnt equals n", []int{2, 4, 7, 8}, 4, []int{2, 4, 7, 8})

	// 边界：单节点链表，cnt 只能是 1
	check("single node", []int{7}, 1, []int{7})

	// 一般情况：返回中间节点，后续部分要一并正确
	check("middle node", []int{1, 2, 3, 4, 5}, 2, []int{4, 5})

	// 边界：cnt == n - 1，验证间距算的是 cnt 而不是 cnt - 1
	check("cnt near end", []int{1, 2, 3}, 2, []int{2, 3})

	fmt.Println("all tests passed")
}
