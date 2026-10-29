/*
============================================================================
889. 根据前序和后序遍历构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
分类：二叉树 · 分治构造（前序 + 后序，答案不唯一）

给定两个整数数组，preorder 和 postorder ，其中 preorder 是一个具有 无重复
值的二叉树的前序遍历，postorder 是同一棵树的后序遍历，重构并返回二叉树。

如果存在多个答案，您可以返回其中 任何 一个。

示例 1：
输入：preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
输出：[1,2,3,4,5,6,7]

示例 2：
输入: preorder = [1], postorder = [1]
输出: [1]

提示：
- 1 <= preorder.length <= 30
- 1 <= preorder[i] <= preorder.length
- preorder 中所有值都 不同
- postorder.length == preorder.length
- 1 <= postorder[i] <= postorder.length
- postorder 中所有值都 不同
- 保证 preorder 和 postorder 是同一棵二叉树的前序遍历和后序遍历
*/
package main

import (
	"fmt"
	"slices"
)

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
解法：递归 + 后序下标哈希表【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每层都必须确定"左子树
占了多长"，而这一步靠查表降到了 O(1)。

为什么慢：前序第一个元素是根、后序最后一个元素也是根，根本身不费事，难在切
分点。左子树的根必然是 `preorder[preL + 1]`，它在后序中的位置决定了左子树的
长度；若每层线性扫描后序区间去找这个位置，链状树上有 O(n) 层，退化成
O(n^2)。

换个角度：`preorder[preL + 1]` 在后序中的下标 p 就是左子树在后序里的终点。
于是左子树节点数 = `p - postL + 1`，一次查表就把前序、后序四个端点全部切出来。

关键点：
  · 前序的根在左端 `preL`，后序的根在右端 `postR`，两侧同时收缩
  · 左子树长度必须用**后序下标**算：`pos[pre[preL + 1]] - postL + 1`，
    用前序下标算会得到同一段长度却对不上边界
  · 递归出口有两个：`preL > preR` 返回空；`preL == preR` 时直接返回叶子，
    否则下一步取 `pre[preL + 1]` 会越界
  · 答案不唯一：某节点只有一个孩子时，前序和后序都无法区分这孩子挂左还是挂
    右，本实现一律当左孩子
  · 正因为不唯一，自测不能断言层序，只能把结果重新前序/后序遍历后与原序列
    逐项比对（两者相等当且仅当它是同一棵树的合法还原）
  · Go 里用闭包递归，map 与两个切片在闭包外捕获，避免层层传参

以示例 1（preorder = [1,2,4,5,3,6,7]，postorder = [4,5,2,6,7,3,1]）为例：
  根 1，左孩子 2 在后序下标 2 → leftSize = 2 - 0 + 1 = 3
  左：前序 [2,4,5]、后序 [4,5,2] → 根 2，左孩子 4 在后序下标 0
      leftSize = 1 → 左 [4]、右 [5]
  右：前序 [3,6,7]、后序 [6,7,3] → 根 3，左孩子 6 在后序下标 3
      leftSize = 1 → 左 [6]、右 [7]
  得到 [1,2,3,4,5,6,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，每个节点只被创建一次
  空间 O(n)：map 存 n 项；递归栈 O(h)，链状时也是 O(n)
*/

// 解法：递归 + 后序下标哈希表（最优解）
func constructFromPrePost(preorder []int, postorder []int) *TreeNode {
	// pos：值 → 后序下标，用来把「左孩子」翻译成「左子树的长度」
	pos := make(map[int]int, len(postorder))
	for i, v := range postorder {
		pos[v] = i
	}

	var build func(preL, preR, postL, postR int) *TreeNode
	build = func(preL, preR, postL, postR int) *TreeNode {
		if preL > preR {
			return nil
		}

		root := &TreeNode{Val: preorder[preL]}
		// 只剩一个节点时没有孩子，继续取 preorder[preL+1] 就越界了
		if preL == preR {
			return root
		}

		leftSize := pos[preorder[preL+1]] - postL + 1

		root.Left = build(preL+1, preL+leftSize, postL, postL+leftSize-1)
		// 右子树的根在后序的 postR - 1 处，那一位是根，不再划给别人
		root.Right = build(preL+leftSize+1, preR, postL+leftSize, postR-1)
		return root
	}

	return build(0, len(preorder)-1, 0, len(postorder)-1)
}

// ===== 测试辅助：前序/后序遍历、节点计数 =====
func preOrder(node *TreeNode, out *[]int) {
	if node == nil {
		return
	}
	*out = append(*out, node.Val)
	preOrder(node.Left, out)
	preOrder(node.Right, out)
}

func postOrder(node *TreeNode, out *[]int) {
	if node == nil {
		return
	}
	postOrder(node.Left, out)
	postOrder(node.Right, out)
	*out = append(*out, node.Val)
}

func countNodes(node *TreeNode) int {
	if node == nil {
		return 0
	}
	return 1 + countNodes(node.Left) + countNodes(node.Right)
}

// 答案不唯一，所以不比对层序：只要求还原出的树能重现原始的两条序列
func check(preorder, postorder []int) {
	t := constructFromPrePost(preorder, postorder)

	gotPre := []int{}
	gotPost := []int{}
	preOrder(t, &gotPre)
	postOrder(t, &gotPost)

	if !slices.Equal(gotPre, preorder) {
		panic(fmt.Sprintf("preorder %v -> %v (%v)", preorder, gotPre, postorder))
	}
	if !slices.Equal(gotPost, postorder) {
		panic(fmt.Sprintf("postorder %v -> %v (%v)", postorder, gotPost, preorder))
	}
	// 节点数相同才说明没有多建或漏建节点
	if countNodes(t) != len(preorder) {
		panic(fmt.Sprintf("node count %d, want %d", countNodes(t), len(preorder)))
	}
}

func main() {
	// 示例 1
	check([]int{1, 2, 4, 5, 3, 6, 7}, []int{4, 5, 2, 6, 7, 3, 1})

	// 示例 2：单节点，验证 preL == preR 的出口
	check([]int{1}, []int{1})

	// 关键点用例：根只有一个左孩子，取 pre[preL + 1] 后必须及时停
	check([]int{1, 2}, []int{2, 1})

	// 反直觉用例：链 [1,2,3] 与 [3,2,1] 同时对应「全左链」和「全右链」，
	// 这里不校验形状，只要能重现两条序列即可
	check([]int{1, 2, 3}, []int{3, 2, 1})

	// 不平衡树：3 只有左孩子 6，左子树长度取到最大
	check([]int{1, 2, 4, 5, 3, 6}, []int{4, 5, 2, 6, 3, 1})

	// 满二叉树 15 个节点，两侧子树长度都不为 0
	check([]int{1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15},
		[]int{8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1})

	// 边界：题面上限 30 个节点的单链
	pre := []int{}
	post := []int{}
	for v := 1; v <= 30; v++ {
		pre = append(pre, v)
	}
	for v := 30; v >= 1; v-- {
		post = append(post, v)
	}
	check(pre, post)

	fmt.Println("all tests passed")
}
