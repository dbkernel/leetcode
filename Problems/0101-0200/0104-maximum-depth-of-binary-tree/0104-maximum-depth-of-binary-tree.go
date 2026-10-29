/*
============================================================================
104. 二叉树的最大深度（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-depth-of-binary-tree/
分类：二叉树 · 深度（DFS / BFS）

给定一个二叉树 root ，返回其最大深度。

二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：3

示例 2：
输入：root = [1,null,2]
输出：2

提示：
- 树中节点的数量在 [0, 10^4] 区间内。
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
解法一：DFS（后序递归取较大值）【最优解】

为什么慢：如果按层去数，必须真的把每一层都走完、并在每层结束时计数；
虽然也是 O(n)，但要多维护一个队列，空间随层宽增长。

换个角度：最大深度满足递归关系 —— 空树深度 0，非空树深度是
`1 + max(左子树深度, 右子树深度)`。把问题拆成两个子问题再合并，
一次后序遍历即可。

关键点：
  · 空节点返回 0，这是递归的边界，也是叶子返回 1 的来源
  · 必须取 max 而不是固定走某一侧；写成 `left + right` 就变成求直径了
  · 后序位置才相加：先拿到左右子树的深度，再 +1 计当前节点
  · 递归深度 = 结果高度，最坏（斜链）O(n)

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  叶子 9、15、7 返回 1；节点 20 返回 1 + max(1,1) = 2；
  根 3 返回 1 + max(1,2) = 3

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈；最坏（斜链）O(n)

解法二：BFS 按层计数【非最优】

官方方法二。层序遍历时每处理完一层就把层数加一，循环结束时层数就是最大深度。
  · 用队列，每轮先记下本层节点数 size；
  · 弹出 size 个节点并把孩子入队，然后 depth++。

关键点：
  · size 必须在 for 循环之前锁定，循环中队列还会增长
  · 空树返回 0：不要入队再靠循环兜底
  · 每层结束才 depth++，不能每弹出一个节点就加
  · 空间 O(w)：宽树时队列可能存下整层，比 DFS 更费

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/

// 解法一：DFS（最优解）
func maxDepthDFS(root *TreeNode) int {
	if root == nil {
		return 0
	}
	left := maxDepthDFS(root.Left)
	right := maxDepthDFS(root.Right)
	if left > right {
		return left + 1
	}
	return right + 1
}

// 解法二：BFS 按层计数
func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	depth := 0
	q := []*TreeNode{root}
	for head := 0; head < len(q); {
		size := len(q) - head
		for i := 0; i < size; i++ {
			node := q[head]
			head++
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		depth++
	}
	return depth
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = 1001

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

func check(input []int, want int) {
	if got := maxDepthDFS(buildTree(input)); got != want {
		panic(fmt.Sprintf("DFS(%v) = %d, want %d", input, got, want))
	}
	if got := maxDepth(buildTree(input)); got != want {
		panic(fmt.Sprintf("BFS(%v) = %d, want %d", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{3, 9, 20, nullNode, nullNode, 15, 7}, 3)

	// 示例 2：只有右孩子
	check([]int{1, nullNode, 2}, 2)

	// 边界：空树
	check([]int{}, 0)

	// 边界：单节点
	check([]int{1}, 1)

	// 边界：左斜链 / 右斜链，深度等于节点数
	check([]int{1, 2, nullNode, 3, nullNode, 4}, 4)
	check([]int{1, nullNode, 2, nullNode, 3}, 3)

	// 边界：完全二叉树，深度 3
	check([]int{1, 2, 3, 4, 5, 6, 7}, 3)

	// 边界：左子树深、右子树浅（不能只看右边）
	check([]int{1, 2, 3, 4, nullNode, nullNode, nullNode, 5}, 4)

	// 边界：负数值不影响结构
	check([]int{-100, -50, 100, 0}, 3)

	// 边界：深链 200 层
	levelOrder := []int{}
	for i := 1; i <= 200; i++ {
		levelOrder = append(levelOrder, i, nullNode)
	}
	check(levelOrder, 200)

	fmt.Println("all tests passed")
}
