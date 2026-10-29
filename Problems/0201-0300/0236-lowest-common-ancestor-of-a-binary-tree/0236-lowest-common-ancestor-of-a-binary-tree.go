/*
============================================================================
236. 二叉树的最近公共祖先（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/
分类：二叉树 · 后序递归 / 最近公共祖先

给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个节点 p、q，最近公共
祖先表示为一个节点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点
也可以是它自己的祖先）。”

示例 1：
输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
输出：3
解释：节点 5 和节点 1 的最近公共祖先是节点 3 。

示例 2：
输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
输出：5
解释：节点 5 和节点 4 的最近公共祖先是节点 5 。因为根据定义最近公共祖先
节点可以为节点本身。

示例 3：
输入：root = [1,2], p = 1, q = 2
输出：1

提示：
- 树中节点数目在范围 [2, 10^5] 内。
- -10^9 <= Node.val <= 10^9
- 所有 Node.val 互不相同 。
- p != q
- p 和 q 均存在于给定的二叉树中。
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
解法：后序递归（每棵子树向上报告"找到了谁"）【最优解】

为什么慢：最直白的做法是"从根往下逐层检查"：对每个节点分别判断 p、q 是否
在它的子树里，第一个"两侧都含"的节点就是答案。可判断一次子树是否含某节点
本身要 O(n)，套在每个节点上最坏 O(n^2)（斜链）。另一种常见写法是用哈希表
记录每个节点的父节点，从 p 向上爬并标记，再从 q 向上找第一个被标记的祖先：
时间 O(n)，但要额外存父指针表，空间 O(n)。

换个角度：递归函数不必回答"是不是"，直接回答"这棵子树里找到了 p 或 q 中的
哪一个"。后序遍历先拿到左右子树的答案，再在当前节点合并：
  · 左右都非空 → p、q 分居两侧，当前节点就是最近公共祖先；
  · 只有一侧非空 → 把这一侧的结果继续向上传；
  · 两侧都空 → 本子树里一个都没找到。
每个节点只走一次，也不需要额外容器。

关键点：
  · 出口先判 node == p || node == q 并立刻返回 node —— 这一条让"p 是 q
    的祖先"时答案停在 p；删掉它就会继续下沉到 q，返回错误节点
  · 合并必须放在后序位置：先拿到左右结果再判断，先序时还不知道两侧情况
  · 只有左右都非空才返回当前节点；一侧为空时返回另一侧，返回 nil 会把
    已经找到的节点弄丢
  · 返回值是"节点"而不是布尔值 —— 布尔值只说明找到了，无法区分找到的是谁

以示例 2（root = [3,5,1,6,2,0,8,null,null,7,4]，p = 5，q = 4）为例：
  根 3 的左子树：走到 5 命中 p，立即返回 5，不再看它的孩子；
  根 3 的右子树：既没有 5 也没有 4，返回 nil；
  根 3 处左非空、右为空 → 把 5 继续上报，最终答案是 5 ✔
  （若去掉"命中即返回"，左子树会返回 4，答案就错成了 4。）

复杂度分析：
  时间 O(n)：每个节点恰好进入递归一次，合并是常数操作
  空间 O(h)：递归栈只保存当前路径，h 为树高；最坏（链）为 O(n)
*/

// lowestCommonAncestor 是平台入口，签名不能改。
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}
	// 命中 p 或 q 就停：p 是 q 的祖先时，答案正是先遇到的这个节点
	if root == p || root == q {
		return root
	}

	left := lowestCommonAncestor(root.Left, p, q)
	right := lowestCommonAncestor(root.Right, p, q)

	// 两侧都找到了，说明 p、q 从这里分开，当前节点是第一个分叉点
	if left != nil && right != nil {
		return root
	}
	// 只剩一侧有结果：该结果可能是 LCA，也可能还要继续往上比
	if left != nil {
		return left
	}
	return right
}

// ===== 测试辅助：按层序数组建树、按值找节点 =====
const nullNode = 1000000001 // 节点值不超过 10^9

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
	if root == nil {
		return nil
	}
	if root.Val == val {
		return root
	}
	if found := findNode(root.Left, val); found != nil {
		return found
	}
	return findNode(root.Right, val)
}

func check(levelOrder []int, pVal, qVal, want int) {
	root := buildTree(levelOrder)
	p := findNode(root, pVal)
	q := findNode(root, qVal)
	wantNode := findNode(root, want)
	if p == nil || q == nil || wantNode == nil {
		panic(fmt.Sprintf("input %v 里找不到 %d/%d/%d", levelOrder, pVal, qVal, want))
	}

	// 比指针而不是值：题目保证值唯一，指针相等才能证明返回的是那个节点
	if got := lowestCommonAncestor(root, p, q); got != wantNode {
		panic(fmt.Sprintf("lca(%v, %d, %d) = %d, want %d",
			levelOrder, pVal, qVal, got.Val, want))
	}
}

func main() {
	example := []int{3, 5, 1, 6, 2, 0, 8, nullNode, nullNode, 7, 4}

	// 示例 1：p、q 分居根的两侧
	check(example, 5, 1, 3)

	// 示例 2：p 是 q 的祖先，答案必须是 p 本身
	check(example, 5, 4, 5)

	// 示例 3：根就是其中一个节点
	check([]int{1, 2}, 1, 2, 1)

	// 反直觉：两个节点都在根的左子树里，不能因为根同时含二者就返回根
	check(example, 6, 4, 5)
	check(example, 7, 4, 2)

	// 反直觉：答案藏在右子树内部，且 p、q 分别是左右孙子
	check(example, 7, 8, 3)

	// 兄弟节点
	check([]int{1, 2, 3}, 2, 3, 1)

	// 参数顺序无关：把 p、q 反过来，答案不变
	check(example, 4, 5, 5)

	// 边界：只有两个节点的左斜链
	check([]int{1, 2}, 2, 1, 1)

	// 边界：200 层右斜链，祖先关系与深链两种情况
	chain := []int{1}
	for i := 2; i <= 200; i++ {
		chain = append(chain, nullNode, i)
	}
	check(chain, 1, 200, 1)
	check(chain, 150, 200, 150)
	check(chain, 100, 150, 100)

	fmt.Println("all tests passed")
}
