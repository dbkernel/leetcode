/*
============================================================================
面试题 04.03. 特定深度节点链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/list-of-depth-lcci/
分类：二叉树 · 层序遍历（BFS）

给定一棵二叉树，设计一个算法，创建含有某一深度上所有节点的链表（比如，若
一棵树的深度为 D，则会创建出 D 个链表）。返回一个包含所有深度的链表的数组。

示例：
输入：[1,2,3,4,5,null,7,8]

        1
       /  \
      2    3
     / \    \
    4   5    7
   /
  8

输出：[[1],[2,3],[4,5,7],[8]]
*/
package main

import "fmt"

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type ListNode struct {
	Val  int
	Next *ListNode
}

/*
解法：BFS 逐层建链表【最优解】

为什么慢：要找"每一层"，最直接的想法是 DFS 记录深度，把节点塞进 depth 对应
的数组，最后每层再建链表。但 DFS 先访问的节点不一定来自同一层，必须额外维护
depth 数组；它把"分层"这件事交给调用方，函数参数里到处是深度。本质上缺的是
"同一层节点连续处理"的顺序。

换个角度：层序遍历本身就把同一层节点排在一起。BFS 队列里当前这批节点正好是
一整层：先记下队列长度 size，只弹出 size 个节点建一条链表，同时把它们的左右
孩子入队，下一轮队列里就是下一层。每轮产出一条链表，自然对应一个深度。

关键点：
  · 每轮开始先固定 size，循环中用 i < size 而不是 len(queue)，
    否则本层入队的孩子会把下一层节点卷进来
  · 用哨兵 dummy 建链表：dummy.Next 就是本层头节点，tail 负责尾插，
    省掉"第一个节点特殊处理"
  · 孩子入队前判空，空孩子不入队，否则会凭空多出一层
  · 空树直接返回空切片，不能进循环，否则会把 nil 入队
  · 返回的是每层链表头节点组成的切片，顺序从浅到深
  · Go 用 queue = queue[1:] 出队，底层数组会随入队自动扩容

以示例（[1,2,3,4,5,null,7,8]）为例：
  第一轮 size = 1：弹出 1，建 [1]，2、3 入队；
  第二轮 size = 2：弹出 2、3，建 [2,3]，4、5、7 入队；
  第三轮 size = 3：弹出 4、5、7，建 [4,5,7]，只有 4 带左孩子 8；
  第四轮 size = 1：弹出 8，建 [8]，无孩子，队列空；
  返回 [[1],[2,3],[4,5,7],[8]] ✔

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次，每个节点新建一个链表节点
  空间 O(n)：队列最宽一层最多 n/2 个节点，输出本身也占 O(n)
*/

func listOfDepth(tree *TreeNode) []*ListNode {
	levels := []*ListNode{}
	if tree == nil {
		return levels
	}

	queue := []*TreeNode{tree}
	for len(queue) > 0 {
		// 先固定本层大小：本层孩子入队后 len(queue) 会变大
		size := len(queue)
		dummy := &ListNode{}
		tail := dummy

		for i := 0; i < size; i++ {
			node := queue[0]
			queue = queue[1:]

			tail.Next = &ListNode{Val: node.Val}
			tail = tail.Next

			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}

		// dummy.Next 是本层链表头，空层不会出现（size >= 1）
		levels = append(levels, dummy.Next)
	}
	return levels
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树、收集每层值 =====
const nullNode = -1 << 40 // 测试数据只用小整数，不会撞上这个哨兵

func buildTree(levelOrder []int) *TreeNode {
	if len(levelOrder) == 0 || levelOrder[0] == nullNode {
		return nil
	}
	root := &TreeNode{Val: levelOrder[0]}
	nodes := []*TreeNode{root}
	idx := 1
	for i := 0; i < len(nodes) && idx < len(levelOrder); i++ {
		if levelOrder[idx] != nullNode {
			nodes[i].Left = &TreeNode{Val: levelOrder[idx]}
			nodes = append(nodes, nodes[i].Left)
		}
		idx++
		if idx >= len(levelOrder) {
			break
		}
		if levelOrder[idx] != nullNode {
			nodes[i].Right = &TreeNode{Val: levelOrder[idx]}
			nodes = append(nodes, nodes[i].Right)
		}
		idx++
	}
	return root
}

// 逐层收集链表的值，层数或层内顺序不对都会在这里暴露
func collectLevels(levels []*ListNode) [][]int {
	got := make([][]int, 0, len(levels))
	for _, head := range levels {
		values := []int{}
		for cur := head; cur != nil; cur = cur.Next {
			values = append(values, cur.Val)
		}
		got = append(got, values)
	}
	return got
}

func sameLevels(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if len(a[i]) != len(b[i]) {
			return false
		}
		for j := range a[i] {
			if a[i][j] != b[i][j] {
				return false
			}
		}
	}
	return true
}

func check(levelOrder []int, want [][]int) {
	got := collectLevels(listOfDepth(buildTree(levelOrder)))
	if !sameLevels(got, want) {
		panic(fmt.Sprintf("listOfDepth(%v) = %v, want %v", levelOrder, got, want))
	}
}

func main() {
	// 示例：第四层只有一个节点，最容易漏层
	check([]int{1, 2, 3, 4, 5, nullNode, 7, 8},
		[][]int{{1}, {2, 3}, {4, 5, 7}, {8}})

	// 边界：空树，返回空切片而不是含一个空链表的切片
	check([]int{}, [][]int{})

	// 边界：单节点，只有一层
	check([]int{1}, [][]int{{1}})

	// 满二叉树，最宽一层有 4 个节点
	check([]int{1, 2, 3, 4, 5, 6, 7},
		[][]int{{1}, {2, 3}, {4, 5, 6, 7}})

	// 边界：只有左孩子的斜链，每层一个节点
	check([]int{1, 2, nullNode, 3, nullNode, 4},
		[][]int{{1}, {2}, {3}, {4}})

	// 边界：只有右孩子的斜链，每层一个节点
	check([]int{1, nullNode, 2, nullNode, 3},
		[][]int{{1}, {2}, {3}})

	// 反直觉：中间层缺孩子，但下一层仍有节点
	check([]int{1, 2, 3, nullNode, 4, nullNode, 5},
		[][]int{{1}, {2, 3}, {4, 5}})

	// 反直觉：左右孩子交叉分布，层内顺序必须从左到右
	check([]int{1, 2, 3, 4, nullNode, nullNode, 5},
		[][]int{{1}, {2, 3}, {4, 5}})

	// 边界：负数
	check([]int{-1, -2, -3}, [][]int{{-1}, {-2, -3}})

	fmt.Println("all tests passed")
}
