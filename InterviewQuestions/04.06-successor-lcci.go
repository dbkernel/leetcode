/*
============================================================================
面试题 04.06. 后继者（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/successor-lcci/
分类：二叉搜索树 · 中序后继

设计一个算法，找出二叉搜索树中指定节点的“下一个”节点（也即中序后继）。

如果指定节点没有对应的“下一个”节点，则返回 null。

示例 1:
输入: root = [2,1,3], p = 1
输出: 2

示例 2:
输入: root = [5,3,6,2,4,null,null,1], p = 6
输出: null
*/
package main

import (
	"fmt"
	"math"
)

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
解法一：中序遍历收集全部节点【非最优】

为什么慢：中序序列天然升序，找到 p 之后的下一个就是答案，思路零门槛；
但要把整棵树存进数组才能取"下一个"，时间 O(n)、额外空间 O(n)，
而且明明只需要一条从根到 p 的路径信息。

换个角度：真正需要的只是"p 之后第一个被访问的节点"。如果能让遍历在
访问到 p 时记下状态，下一个被访问的节点就是答案，就无需存整棵序列。

关键点：
  · 中序遍历的访问顺序是左 → 根 → 右，p 的后继就是 p 之后第一个访问到的节点
  · 用一个 found 标记记录"是否已经访问过 p"
  · 找到后继后要能提前收手，否则会被后续节点覆盖答案
  · 递归写法最直观，但额外 O(n) 的数组是它的主要缺点

以示例 1（root = [2,1,3]，p = 1）为例：
  中序为 1、2、3，p = 1 之后是 2 ✔

复杂度分析：
  时间 O(n)：完整中序遍历
  空间 O(n)：保存中序序列（若改成"边遍历边判断"可以降到 O(h)）

解法二：利用 BST 性质（不遍历全树）【最优解】

官方方法二。分两种情况，都不需要访问整棵树：
  1. p 有右子树：p 的中序后继一定是右子树里的**最小**节点，
     也就是右子树一路向左走到底的那个节点；
  2. p 没有右子树：后继在"祖先"里 —— 从根往下找，每当遇到比 p 大的节点，
     它就有可能是答案（先记下来），然后向左子树继续找更近的；
     遇到比 p 小或相等的节点则向右走。走到底记下的最后一个"更大的祖先"
     就是答案。
情况 2 的本质是：后继是"大于 p 的最小祖先"。

关键点：
  · 情况 1 别忘了先判断 p.Right 是否为空
  · 情况 2 里 successor 要在"往左走"之前更新，往右走时不更新
  · 比较用值即可（题目保证值互不相同）；若要更严谨可以比较节点地址
  · 都没找到说明 p 是整棵树的最大值，返回 nil
  · 只沿一条路径走，时间 O(h)、空间 O(1)，明显优于解法一

以示例 2（root = [5,3,6,2,4,null,null,1]，p = 6）为例：
  6 没有右子树 → 从根找"大于 6 的最小祖先"：
  5 < 6 往右到 6；6 不大于 6 往右到空 → 没有更大的祖先 → 返回 nil ✔
以 p = 4 为例：
  4 没有右子树 → 5 > 4，successor = 5，往左到 3；3 < 4 往右到 4；
  4 不大于 4 → 往右到空，返回 successor = 5 ✔

复杂度分析：
  时间 O(h)：h 为树高，最坏（斜链）O(n)
  空间 O(1)：只用几个指针，连递归栈都不需要
*/

// 解法一：中序遍历收集全部节点
func inorderSuccessorInorder(root, p *TreeNode) *TreeNode {
	var order []*TreeNode

	var dfs func(node *TreeNode)
	dfs = func(node *TreeNode) {
		if node == nil {
			return
		}
		dfs(node.Left)
		order = append(order, node)
		dfs(node.Right)
	}

	dfs(root)
	for i := 0; i+1 < len(order); i++ {
		if order[i] == p {
			return order[i+1]
		}
	}
	return nil
}

// 解法二：利用 BST 性质（最优解）
func inorderSuccessor(root, p *TreeNode) *TreeNode {
	// 情况 1：有右子树，后继是右子树中的最小节点
	if p.Right != nil {
		node := p.Right
		for node.Left != nil {
			node = node.Left
		}
		return node
	}

	// 情况 2：没有右子树，后继是"大于 p 的最小祖先"
	var successor *TreeNode
	node := root
	for node != nil {
		if node.Val > p.Val {
			successor = node // 先记下这个候选
			node = node.Left
		} else {
			node = node.Right
		}
	}
	return successor
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = math.MaxInt32 // 节点值均远小于哨兵

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

func findNode(root *TreeNode, val int) *TreeNode {
	for root != nil && root.Val != val {
		if val < root.Val {
			root = root.Left
		} else {
			root = root.Right
		}
	}
	return root
}

// want < 0 表示期望 nil
func check(input []int, pVal, want int) {
	t1 := buildTree(input)
	got1 := inorderSuccessorInorder(t1, findNode(t1, pVal))
	if got1 == nil {
		if want != -1 {
			panic(fmt.Sprintf("inorder(%v, %d) = nil, want %d", input, pVal, want))
		}
	} else if got1.Val != want {
		panic(fmt.Sprintf("inorder(%v, %d) = %d, want %d", input, pVal, got1.Val, want))
	}

	t2 := buildTree(input)
	got2 := inorderSuccessor(t2, findNode(t2, pVal))
	if got2 == nil {
		if want != -1 {
			panic(fmt.Sprintf("bst(%v, %d) = nil, want %d", input, pVal, want))
		}
	} else if got2.Val != want {
		panic(fmt.Sprintf("bst(%v, %d) = %d, want %d", input, pVal, got2.Val, want))
	}
}

func main() {
	// 示例 1
	check([]int{2, 1, 3}, 1, 2)

	// 示例 2
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 6, -1)

	// 同一个树上的其它后继
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 1, 2)
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 2, 3)
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 3, 4)
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 4, 5)
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 5, 6)

	// 边界：单节点，没有后继
	check([]int{1}, 1, -1)

	// 边界：有右子树时，后继是右子树的最左节点
	check([]int{2, 1, 3}, 2, 3)

	// 边界：左斜链 [3,2,null,1]，降序中序 1,2,3
	check([]int{3, 2, nullNode, 1}, 1, 2)
	check([]int{3, 2, nullNode, 1}, 2, 3)
	check([]int{3, 2, nullNode, 1}, 3, -1)

	// 边界：负数值
	check([]int{0, -1, 1}, -1, 0)
	check([]int{0, -1, 1}, 1, -1)

	// 边界：100 层右斜链 1..100
	chain := []int{1}
	for i := 2; i <= 100; i++ {
		chain = append(chain, nullNode, i)
	}
	check(chain, 1, 2)
	check(chain, 99, 100)
	check(chain, 100, -1)

	fmt.Println("all tests passed")
}
