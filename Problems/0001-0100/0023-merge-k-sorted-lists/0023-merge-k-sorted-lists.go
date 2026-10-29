/*
============================================================================
23. 合并 K 个升序链表（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/merge-k-sorted-lists/
分类：堆 · K 路归并（分治 / 优先队列）
============================================================================

给你一个链表数组，每个链表都已经按升序排列。

请你将所有链表合并到一个升序链表中，返回合并后的链表。

示例 1：
输入：lists = [[1,4,5],[1,3,4],[2,6]]
输出：[1,1,2,3,4,4,5,6]
解释：链表数组如下：
[

	1->4->5,
	1->3->4,
	2->6

]
将它们合并到一个有序链表中得到。
1->1->2->3->4->4->5->6

示例 2：
输入：lists = []
输出：[]

示例 3：
输入：lists = [[]]
输出：[]

提示：
- k == lists.length
- 0 <= k <= 10^4
- 0 <= lists[i].length <= 500
- -10^4 <= lists[i][j] <= 10^4
- lists[i] 按 升序 排列
- lists[i].length 的总和不超过 10^4
*/
package main

import (
	"container/heap"
	"fmt"
	"sort"
)

// 力扣预置的节点定义
type ListNode struct {
	Val  int
	Next *ListNode
}

// 两条升序链表合并成一条；三个解法都要用它，抽出来避免三份拷贝
func mergeTwo(a, b *ListNode) *ListNode {
	// 哨兵节点让「结果头」和「后续节点」走同一套赋值逻辑
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

	// 剩下那段整体有序，直接整段接上，不必逐节点搬运
	if a != nil {
		tail.Next = a
	} else {
		tail.Next = b
	}
	return dummy.Next
}

/*
解法一：顺序合并（拿已合并的长链去接下一个）【非最优】

为什么慢：先把第 1、2 条合并成结果，再拿这个结果去和第 3 条合并，依此类推。
第 i 次合并要把当时的结果链完整走一遍，长度约为 i * n，于是总代价是
n(1 + 2 + ... + k) = O(k^2 n)。慢的地方很具体：已经合并好的长链被反复重扫。

换个角度：合并两条链的代价正比于「两条链长度之和」，所以要减少每条链参与
合并的次数。把 k 条链两两配对，每轮合并的总长度仍然是 n，轮数却是 log k。

关键点：

	· 合并两条链必须用哨兵节点，否则「结果头」要单独判一次空
	· 尾部接剩余段时整段接上，不要再逐节点搬运
	· 这里保留顺序合并，是为了给另外两个解法的复杂度对比一个基准

以示例 1（lists = [[1,4,5],[1,3,4],[2,6]]）为例：

	merge([1,4,5], [1,3,4]) → [1,1,3,4,4,5]
	merge([1,1,3,4,4,5], [2,6]) → [1,1,2,3,4,4,5,6]
	第 2 次合并把长度为 6 的结果链又完整扫了一遍

复杂度分析：

	时间 O(k^2 n)：第 i 次合并要走 i * n 个节点
	空间 O(1)：只改指针，不新建节点
*/
func mergeKListsSequential(lists []*ListNode) *ListNode {
	var merged *ListNode
	for _, list := range lists {
		merged = mergeTwo(merged, list)
	}
	return merged
}

/*
解法二：分治两两合并【最优解】

为什么慢：解法一的浪费不在「合并」这个动作，而在合并的顺序 —— 第 1 条链
被扫了 k 次。只要让每条链参与的合并次数降到 log k 次，总代价就降到
O(n log k)，而这个量级已经是最优的：每个节点至少要读一次，k 条链归并的
比较次数下界就是 Ω(n log k)。

换个角度：把 lists 看成待归并的序列，对半切开、左右各自合并成一条，再合并
这两条。每一层里所有链加起来恰好是 n 个节点，层数是 log k。

关键点：

	· 区间写成左闭右开 [l, r)，这样 r - l == 1 就是最小切口，只需返回
	  lists[l]，不会因为 l == r 无限递归
	· 中点用 l + (r-l)/2，避免 l + r 在 k 很大时溢出
	· 先判 lists 为空：此时区间是 [0, 0)，任何最小切口都不命中
	· 合并只在递归返回时做，不能先建结果再递归
	· Go 里把区间做成闭包参数，不引入额外结构体

以示例 1（lists = [[1,4,5],[1,3,4],[2,6]]）为例：

	区间 [0,3) 切成 [0,1) 与 [1,3)
	[0,1) 直接返回 [1,4,5]
	[1,3) 再切成 [1,2) = [1,3,4] 与 [2,3) = [2,6]，合并成 [1,2,3,4,6]
	最后 merge([1,4,5], [1,2,3,4,6]) → [1,1,2,3,4,4,5,6] ✔

复杂度分析：

	时间 O(n log k)：每层合并的总长度是 n，共 log k 层
	空间 O(log k)：递归栈深度；结果链表复用原节点，不额外分配
*/
func mergeKListsDivide(lists []*ListNode) *ListNode {
	if len(lists) == 0 {
		return nil
	}

	// 合并左闭右开区间 [l, r) 内的链表，区间至少有一个元素
	var mergeRange func(l, r int) *ListNode
	mergeRange = func(l, r int) *ListNode {
		// 只剩一条链时直接返回，这是分治的唯一出口
		if r-l == 1 {
			return lists[l]
		}
		mid := l + (r-l)/2
		return mergeTwo(mergeRange(l, mid), mergeRange(mid, r))
	}

	return mergeRange(0, len(lists))
}

/*
解法三：优先队列（K 路归并）【最优解，与解法二并列】

为什么慢：若要「每次取出全局最小」，最朴素的写法是每轮扫一遍 k 个头，
O(k) 一次，总共 O(nk)。慢在「k 个头里挑最小」被重复做了 n 次却没有任何
信息被复用。

换个角度：维护一个只装「每条链当前头节点」的小根堆，堆顶就是全局最小。
取走堆顶后，把它的后继补进堆 —— 堆里始终只有 k 个候选，取最小只需 O(log k)。

它与解法二时间同为 O(n log k)，是并列最优：分治不额外占空间、实现更短；
堆只需要 k 个额外指针，且天然支持「链表一条条到达」的流式输入。

关键点：
  · 堆里放的是「每条链尚未接上的头」，不是整条链 —— 否则堆会膨胀到 n
  · 出堆之后才把 node.Next 入堆：入堆的一定还没被接过
  · 走完的链不要再入堆，堆为空的时刻就是合并完成的时刻
  · Go 的 container/heap 需要自己实现 heap.Interface，Less 写成 `<`
    才是小根堆（Less 决定的是「谁排前面」，和 C++ 的反向比较器不同）
  · 相等时谁先出堆不影响结果，题目只要求整体升序

以示例 1（lists = [[1,4,5],[1,3,4],[2,6]]）为例：
  初始堆 = {1(链1), 1(链2), 2(链3)}
  依次弹出 1、1、2，每次弹出后把该链的下一个节点入堆
  接出的顺序是 1,1,2,3,4,4,5,6 ✔

复杂度分析：
  时间 O(n log k)：每个节点进出堆各一次，每次 O(log k)
  空间 O(k)：堆里最多同时存在 k 个候选节点
*/

// 小根堆：Less 用 `<`，于是 Val 最小的节点排在堆顶
type listHeap []*ListNode

func (h listHeap) Len() int           { return len(h) }
func (h listHeap) Less(i, j int) bool { return h[i].Val < h[j].Val }
func (h listHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *listHeap) Push(x any) { *h = append(*h, x.(*ListNode)) }

func (h *listHeap) Pop() any {
	old := *h
	last := len(old) - 1
	node := old[last]
	// 先把尾元素摘下来，再把切片截短，避免底层数组继续持有它
	*h = old[:last]
	return node
}

func mergeKListsHeap(lists []*ListNode) *ListNode {
	h := &listHeap{}
	// 只放每条链的头，堆的大小因此被 k 卡住
	for _, head := range lists {
		if head != nil {
			*h = append(*h, head)
		}
	}
	heap.Init(h)

	dummy := &ListNode{}
	tail := dummy

	for h.Len() > 0 {
		node := heap.Pop(h).(*ListNode)
		tail.Next = node
		tail = node

		// 出堆之后才补后继：还在堆里的节点都必然没被接上
		if node.Next != nil {
			heap.Push(h, node.Next)
		}
	}

	return dummy.Next
}

// ===== 测试辅助：由切片建链表、展开 =====
func buildList(values []int) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for _, v := range values {
		tail.Next = &ListNode{Val: v}
		tail = tail.Next
	}
	return dummy.Next
}

func buildLists(lists [][]int) []*ListNode {
	heads := make([]*ListNode, 0, len(lists))
	for _, values := range lists {
		heads = append(heads, buildList(values))
	}
	return heads
}

func toSlice(head *ListNode) []int {
	values := []int{}
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

// 一个用例喂给三个解法；输入节点会被接进结果，所以每个解法各建一份
func check(lists [][]int, want []int) {
	if got := toSlice(mergeKListsSequential(buildLists(lists))); !equal(got, want) {
		panic(fmt.Sprintf("sequential(%v) = %v, want %v", lists, got, want))
	}
	if got := toSlice(mergeKListsDivide(buildLists(lists))); !equal(got, want) {
		panic(fmt.Sprintf("divide(%v) = %v, want %v", lists, got, want))
	}
	if got := toSlice(mergeKListsHeap(buildLists(lists))); !equal(got, want) {
		panic(fmt.Sprintf("heap(%v) = %v, want %v", lists, got, want))
	}
}

func main() {
	// 示例 1：三条链交错，验证每步都取到全局最小
	check([][]int{{1, 4, 5}, {1, 3, 4}, {2, 6}}, []int{1, 1, 2, 3, 4, 4, 5, 6})

	// 示例 2：一条链都没有
	check([][]int{}, []int{})

	// 示例 3：只有一个空链表，验证「空输入」与「空元素」两条不同的分支
	check([][]int{{}}, []int{})

	// 边界：多条空链表与有值链表混在一起
	check([][]int{{}, {1}, {}, {2, 3}, {}}, []int{1, 2, 3})

	// 边界：单条链，不需要任何合并
	check([][]int{{-3, -1, 7}}, []int{-3, -1, 7})

	// 边界：负数与相同的值，验证比较用 <= 不会漏节点
	check([][]int{{-5, -1}, {-3, 0}, {-5, 0}}, []int{-5, -5, -3, -1, 0, 0})

	// 边界：每条链都只有一个节点，退化成对 k 个数的排序
	check([][]int{{3}, {1}, {2}, {5}, {4}}, []int{1, 2, 3, 4, 5})

	// 规模：20 条链各 20 个节点，k 较大时三条链的分治与堆都不能出错
	{
		lists := [][]int{}
		want := []int{}
		for i := 0; i < 20; i++ {
			cur := []int{}
			for j := 0; j < 20; j++ {
				cur = append(cur, i+j*20)
				want = append(want, i+j*20)
			}
			lists = append(lists, cur)
		}
		sort.Ints(want)
		check(lists, want)
	}

	fmt.Println("all tests passed")
}
