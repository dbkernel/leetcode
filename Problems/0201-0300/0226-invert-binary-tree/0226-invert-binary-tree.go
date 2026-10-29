/*
============================================================================
226. 翻转二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/invert-binary-tree/
分类：二叉树 · 递归 / 层序遍历

给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。

示例 1：
输入：root = [4,2,7,1,3,6,9]
输出：[4,7,2,9,6,3,1]

示例 2：
输入：root = [2,1,3]
输出：[2,3,1]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目范围在 [0, 100] 内
- -100 <= Node.val <= 100
*/
package main

import "fmt"

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
解法一：递归（交换左右孩子 + 递归翻转）【最优解】

为什么慢：按"新建一棵镜像树"的思路写也能过，但要额外 new 出一整棵树，
空间翻倍；而且原树被丢弃，白白浪费。层序迭代也能做，但要显式维护队列，
代码并不比递归短。

换个角度：翻转是一棵子树一个动作 —— 对当前节点，先交换它的左右孩子，
再分别翻转这两棵子树即可。由于"翻转左右子树"与"交换左右孩子"可交换顺序，
先序、后序写都成立；中序则会因为交换后访问的子树已变而需要小心，一般不用。

关键点：
  · 每次递归只需做两件事：交换 Left / Right，然后递归两个孩子
  · 空节点直接返回，作为递归出口
  · 交换后递归时的 Left / Right 已是"翻转前"的另一侧，注意别重复交换
  · 递归深度 = 树高，本题节点数 ≤ 100，绝对安全
  · 若题目不允许修改原树，才需要新建节点；本题允许原地翻转

以示例 1（root = [4,2,7,1,3,6,9]）为例：
  节点 1、3、6、9 无孩子，返回自身；
  节点 2 交换左右（1、3）→ 变 (3,1)；节点 7 同理变 (9,6)；
  根 4 交换左右（2、7）→ (7,2)，结果 [4,7,2,9,6,3,1] ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：BFS 层序迭代【非最优】

官方方法二。用队列做层序遍历，每弹出一个节点就交换它的左右孩子，
再把非空孩子入队 —— 因为交换只依赖节点自身，与访问顺序无关。

关键点：
  · 交换发生在"弹出"时，孩子入队顺序无所谓，反正后面还会各自交换
  · 空树直接返回 root（nil），不要先入队
  · 不需要按层分批处理，本题没有"按层"语义，一层一层反而多余
  · 空间 O(w)：宽树时要存下整层节点

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/

// 解法一：递归（最优解）
func invertTree(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}

	root.Left, root.Right = root.Right, root.Left
	invertTree(root.Left)
	invertTree(root.Right)
	return root
}

// 解法二：BFS 层序迭代
func invertTreeBFS(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}

	q := []*TreeNode{root}
	for head := 0; head < len(q); head++ {
		node := q[head]
		node.Left, node.Right = node.Right, node.Left
		if node.Left != nil {
			q = append(q, node.Left)
		}
		if node.Right != nil {
			q = append(q, node.Right)
		}
	}
	return root
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = 1 << 30 // 节点值范围 [-100,100]

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

func sameTree(a, b *TreeNode) bool {
	if a == nil || b == nil {
		return a == b
	}
	return a.Val == b.Val && sameTree(a.Left, b.Left) && sameTree(a.Right, b.Right)
}

func check(input, want []int) {
	if got := invertTree(buildTree(input)); !sameTree(got, buildTree(want)) {
		panic(fmt.Sprintf("DFS(%v) mismatch, want %v", input, want))
	}
	if got := invertTreeBFS(buildTree(input)); !sameTree(got, buildTree(want)) {
		panic(fmt.Sprintf("BFS(%v) mismatch, want %v", input, want))
	}
}

func main() {
	// 示例 1
	check([]int{4, 2, 7, 1, 3, 6, 9}, []int{4, 7, 2, 9, 6, 3, 1})

	// 示例 2
	check([]int{2, 1, 3}, []int{2, 3, 1})

	// 示例 3：空树
	check([]int{}, []int{})

	// 边界：单节点
	check([]int{1}, []int{1})

	// 边界：只有左孩子 → 翻转后变成右孩子
	check([]int{1, 2}, []int{1, nullNode, 2})

	// 边界：只有右孩子
	check([]int{1, nullNode, 2}, []int{1, 2})

	// 边界：叶子层的孩子也要换位
	check([]int{1, 2, 3, 4}, []int{1, 3, 2, nullNode, nullNode, nullNode, 4})

	// 边界：负数不影响结构
	check([]int{-100, -50, 100}, []int{-100, 100, -50})

	// 边界：左斜链 100 层，翻转后变成同深度的右斜链
	chain := []int{1}
	for i := 2; i <= 100; i++ {
		chain = append(chain, i, nullNode)
	}
	want := []int{}
	for i := 1; i <= 100; i++ {
		want = append(want, i, nullNode)
	}
	check(chain, want)

	fmt.Println("all tests passed")
}
