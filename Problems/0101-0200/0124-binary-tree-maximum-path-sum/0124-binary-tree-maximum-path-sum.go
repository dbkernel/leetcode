/*
============================================================================
124. 二叉树中的最大路径和（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-maximum-path-sum/
分类：二叉树 · 后序递归（单边贡献）

二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。
同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定
经过根节点。

路径和 是路径中各节点值的总和。

给你一个二叉树的根节点 root ，返回其 最大路径和 。

示例 1：
输入：root = [1,2,3]
输出：6
解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6

示例 2：
输入：root = [-10,9,20,null,null,15,7]
输出：42
解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42

提示：
- 树中节点数目范围是 [1, 3 * 10^4]
- -1000 <= Node.val <= 1000
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
解法：后序递归，返回单边最大贡献【最优解】

为什么慢：路径可以在任意节点拐弯，最直白的做法是枚举每个节点当拐点，
再分别求左右子树的最大下行和。若求下行和时每次都重新往下扫一遍，
同一条链会被它的每个祖先各扫一次，斜链上退化成 O(n^2)，而树本身只有 n 个点。

换个角度：把问题拆成"贡献值"——以某节点为起点、只朝一个方向往下走能取得的
最大和。它天然可以后序返回：左、右贡献各自与 0 取 max，挑较大的一边接上
自己。而"以当前节点为拐点"的完整路径和是 左 + 当前 + 右，它只用来更新全局
答案，不参与返回。这样每个节点只算一次。

关键点：

	· 返回给父节点的只能是一条腿：路径一旦向上回到父节点，就不能再分叉，
	  同时带两条腿会和父节点组成环
	· 单边贡献与 0 取 max：负数贡献宁可不要，取 0 等价于"从当前节点开始"
	· 全局答案用 左 + 右 + 当前值 更新，并且必须在后序位置（拿到左右结果后）
	· 答案初值取尽可能小的数，不能取 0：全负树必须返回最大的那个负数
	· 更新答案和返回值是两个不同的量，不要用返回值去更新答案
	· Go 没有 int 最小值常量，用 -1<<60 作哨兵，结果仍落在 int 范围内

手推一遍：以示例 2（root = [-10,9,20,null,null,15,7]）为例：

	叶子 15、7：更新 15、7，各自返回 15、7
	叶子 9：更新 9，返回 9
	节点 20：15 + 20 + 7 = 42 更新答案；返回 20 + max(15,7) = 35
	节点 -10：9 + 35 - 10 = 34，不及 42；返回 -10 + 35 = 25
	答案 42 ✔

复杂度分析：

	时间 O(n)：每个节点只进出一次，贡献值一次算清，没有重复扫描
	空间 O(h)：递归栈深度等于树高，最坏（斜链）O(n)
*/
func maxPathSum(root *TreeNode) int {
	best := -1 << 60

	// 返回以 node 为起点、只朝一个方向向下的最大路径和
	var gain func(node *TreeNode) int
	gain = func(node *TreeNode) int {
		if node == nil {
			return 0
		}

		left := gain(node.Left)
		if left < 0 {
			left = 0
		}
		right := gain(node.Right)
		if right < 0 {
			right = 0
		}

		// 拐点在当前节点：两条腿可以同时要，但只用于更新答案
		if v := node.Val + left + right; v > best {
			best = v
		}

		// 交给父节点只能带一条腿，否则路径在父节点处分叉成环
		if left > right {
			return node.Val + left
		}
		return node.Val + right
	}

	gain(root)
	return best
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = 1001 // 节点值范围 [-1000,1000]

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

func check(levelOrder []int, want int) {
	if got := maxPathSum(buildTree(levelOrder)); got != want {
		panic(fmt.Sprintf("maxPathSum(%v) = %d, want %d",
			levelOrder, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{1, 2, 3}, 6)

	// 示例 2
	check([]int{-10, 9, 20, nullNode, nullNode, 15, 7}, 42)

	// 边界：单节点，答案就是节点值本身
	check([]int{-3}, -3)

	// 反直觉：全负树不能返回 0，应取最大的单个节点
	check([]int{-5, -2, -3}, -2)

	// 反直觉：负孩子贡献为负，必须丢掉，答案仍是根
	check([]int{2, -1}, 2)

	// 反直觉：根为负，最优路径完全在右子树
	check([]int{-1, 2}, 2)

	// 关键点：路径可以只走一侧，不能强求左右都取
	check([]int{1, -2, 3}, 4)

	// 关键点：最长路径不经过根，藏在左子树内部
	check([]int{1, 2, 3, 4, 5}, 11)

	// 边界：全 0，答案为 0
	check([]int{0, 0, 0}, 0)

	// 边界：负根 + 长负链，答案在链中间取到
	check([]int{-10, -20, -30, nullNode, 5}, 5)

	// 边界：1000 层右斜链全为 1，答案 1000，用来压递归深度
	chain := []int{1}
	for i := 2; i <= 1000; i++ {
		chain = append(chain, nullNode, 1)
	}
	check(chain, 1000)

	fmt.Println("all tests passed")
}
