/*
============================================================================
LeetCode 2. 两数相加（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/add-two-numbers/
分类：链表 · 竖式加法（一次遍历）
============================================================================

给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的
方式存储的，并且每个节点只能存储 一位 数字。

请你将两个数相加，并以相同形式返回一个表示和的链表。

你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例 1：
输入：l1 = [2,4,3], l2 = [5,6,4]
输出：[7,0,8]
解释：342 + 465 = 807.

示例 2：
输入：l1 = [0], l2 = [0]
输出：[0]

示例 3：
输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
输出：[8,9,9,9,0,0,0,1]

提示：
每个链表中的节点数在范围 [1, 100] 内
0 <= Node.val <= 9
题目数据保证列表表示的数字不含前导零
*/
package main

import (
	"fmt"
	"slices"
)

// 平台预置的链表节点定义；本地 go run 时需要补上
type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法：模拟（竖式加法，一次遍历）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：结果的每一位都必须
被写出来，答案链表长度是 O(max(m,n))，因此 O(max(m,n)) 的时间已经到底。

为什么慢：先把两条链表还原成整数、相加、再拆回链表，看起来只扫两遍，
但节点数最多 100 位，远超 64 位整数；要么改用大整数或字符串加法，
要么在转换中反复处理进位，代码和复杂度都不划算。真正需要的信息只有
「同一位置的两位数字 + 来自低位的进位」。

换个角度：链表按逆序存储，表头就是个位，两条链表同一下标天然对齐到
同一位。于是把竖式加法直接搬到链表上：边走边算 n1 + n2 + carry，
当前位留下 sum % 10，进位 sum / 10 留给下一位。

关键点：
  · 进位必须参与当前位求和 —— 漏掉它，5 + 5 会得到 0 而不是 0 进 1
  · 循环条件用 l1 != nil || l2 != nil，短链表走完后按 0 参与
  · 循环结束后 carry > 0 要补一个节点 —— 否则 [9,9] + [1] 会少最高位

以示例 1（l1 = [2,4,3], l2 = [5,6,4]，即 342 + 465）为例：
  个位：3 + 4 + 0 = 7，carry = 0 → 答案第 1 位 7
  十位：4 + 6 + 0 = 10，carry = 1 → 答案第 2 位 0
  百位：2 + 5 + 1 = 8，carry = 0 → 答案第 3 位 8
  结果 7 → 0 → 8，即 807

Go 的两处写法差异：
  · 指针用 nil 判空，取 Val 前必须先判空，不能假设短链表一定更长
  · &ListNode{Val: v} 的 Next 默认是零值 nil，正好作为新表尾

复杂度分析：
  时间 O(max(m,n))：两条链表各走一遍，每位只做常数次运算
  空间 O(1)：除返回值外只用了 head / tail / carry 三个变量
             （返回值本身占 O(max(m,n))，按官方口径不计入）
*/

// 返回一条新链表，表示 l1 与 l2 所代表整数之和，不修改输入链表
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	// head 固定答案表头，tail 始终指向答案末尾，省掉每次追加都从头找
	var head, tail *ListNode
	carry := 0

	// 任一链表还有位就继续：短链表缺的位按 0 处理
	for l1 != nil || l2 != nil {
		n1, n2 := 0, 0
		if l1 != nil {
			n1 = l1.Val
		}
		if l2 != nil {
			n2 = l2.Val
		}

		// 进位来自低位，必须和当前两位一起相加
		sum := n1 + n2 + carry

		node := &ListNode{Val: sum % 10}
		if head == nil {
			head, tail = node, node
		} else {
			tail.Next = node
			tail = node
		}

		// 留给下一位；sum < 20，所以 carry 只可能是 0 或 1
		carry = sum / 10

		// 先判空再前进，否则长度较短的链表会提前解引用 nil
		if l1 != nil {
			l1 = l1.Next
		}
		if l2 != nil {
			l2 = l2.Next
		}
	}

	// 最高位仍可能产生进位，例如 [9,9] + [1] = [0,0,1]
	if carry > 0 {
		tail.Next = &ListNode{Val: carry}
	}

	return head
}

// ---- 以下为自测代码，不属于题解主体 ----

// 由切片构造链表；节点由 Go 的 GC 回收，无需手动释放
func build(digits []int) *ListNode {
	var head, tail *ListNode
	for _, d := range digits {
		node := &ListNode{Val: d}
		if head == nil {
			head, tail = node, node
		} else {
			tail.Next = node
			tail = node
		}
	}
	return head
}

// 按题目约定的顺序展开链表：表头是个位，表尾是最高位
func toSlice(head *ListNode) []int {
	var digits []int
	for p := head; p != nil; p = p.Next {
		digits = append(digits, p.Val)
	}
	return digits
}

// 一个用例 = 两组输入 + 一组期望输出；失败时 panic 并带上用例标识
func check(name string, a, b, want []int) {
	solution := addTwoNumbers(build(a), build(b))
	if got := toSlice(solution); !slices.Equal(got, want) {
		panic(fmt.Sprintf("%s: got %v, want %v", name, got, want))
	}
}

func main() {
	// 示例 1：342 + 465 = 807，逐位无跨位进位
	check("example 1", []int{2, 4, 3}, []int{5, 6, 4}, []int{7, 0, 8})

	// 示例 2：两个 0 相加，答案必须仍是单节点 [0]
	check("example 2", []int{0}, []int{0}, []int{0})

	// 示例 3：答案比两个输入都长，专门验证末尾补进位
	check("example 3",
		[]int{9, 9, 9, 9, 9, 9, 9},
		[]int{9, 9, 9, 9},
		[]int{8, 9, 9, 9, 0, 0, 0, 1})

	// 边界：5 + 5 = 10，最短的「进位导致多一位」
	check("carry grows length", []int{5}, []int{5}, []int{0, 1})

	// 边界：短链表先走完，长链表剩余位按 0 继续加
	check("uneven length", []int{1}, []int{9, 9}, []int{0, 0, 1})

	fmt.Println("all tests passed")
}
