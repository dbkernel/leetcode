/*
============================================================================
LCR 149 / 剑指 Offer 32 - I. 从上到下打印二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/
分类：二叉树 · 层序遍历（BFS / 队列）

一棵圣诞树记作根节点为 root 的二叉树，节点值为该位置装饰彩灯的颜色编号。
请按照从 左 到 右 的顺序返回每一层彩灯编号。

说明：本题原名「从上到下打印二叉树」，leetcode.cn 现已改用
decorateRecord 作为函数名，题面意思不变。

示例 1：
输入：root = [8,17,21,18,null,null,6]
输出：[8,17,21,18,6]

提示：
- 节点总数 <= 1000
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
解法：BFS + 队列【最优解】

为什么慢：如果按「深度优先」去搜，虽然每个节点只访问一次，但得到的顺序是
一条路走到黑（根-左子树整条-右子树整条），要还原成层序还得额外记层号再排序，
反而把简单问题复杂化。

换个角度：题目要的正是「先来先访问」的顺序 —— 先遇到第 1 层，再遇到第 2 层。
这正是先进先出队列的语义：把根入队，每次从队头取出一个节点输出，并把它
非空的左右孩子依次接到队尾。

关键点：
  · 入队顺序必须是「先左后右」，否则同一层内的顺序会反
  · 孩子为空时不要入队，否则会把 nil 带进队列，后续取值崩溃
  · 根为空要单独处理：直接返回空切片，不要入队
  · 队列里同时最多存两层的节点（当前层剩余 + 下一层），空间 O(w)，
    w 为最大层宽；本题不要求按层分组，所以不需要记录层大小
  · 用切片模拟队列，配合 head 下标出队，避免 `st = st[1:]` 反复移动底层数组

以示例 1（root = [8,17,21,18,null,null,6]）为例：
  队列 [8] → 输出 8，入队 17、21
  输出 17，入队 18 → 输出 21，入队 6
  输出 18、6 → [8,17,21,18,6]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为树的最大层宽，最坏（完全二叉树最后一层）O(n)
*/
func decorateRecord(root *TreeNode) []int {
	ans := []int{}
	if root == nil {
		return ans
	}

	q := []*TreeNode{root}
	for head := 0; head < len(q); head++ {
		node := q[head]
		ans = append(ans, node.Val)

		// 先左后右，保证同层内从左到右
		if node.Left != nil {
			q = append(q, node.Left)
		}
		if node.Right != nil {
			q = append(q, node.Right)
		}
	}
	return ans
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

func check(levelOrder []int, want []int) {
	if got := decorateRecord(buildTree(levelOrder)); !equal(got, want) {
		panic(fmt.Sprintf("decorateRecord(%v) = %v, want %v", levelOrder, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{8, 17, 21, 18, nullNode, nullNode, 6}, []int{8, 17, 21, 18, 6})

	// 边界：空树
	check([]int{}, []int{})

	// 边界：单节点
	check([]int{1}, []int{1})

	// 边界：完全二叉树，验证同层从左到右
	check([]int{1, 2, 3, 4, 5, 6, 7}, []int{1, 2, 3, 4, 5, 6, 7})

	// 边界：左斜链 / 右斜链
	check([]int{1, 2, nullNode, 3}, []int{1, 2, 3})
	check([]int{1, nullNode, 2, nullNode, 3}, []int{1, 2, 3})

	// 边界：右孩子缺失、左孩子存在（不能因为右空就提前结束本层）
	check([]int{1, 2, 3, 4, nullNode, nullNode, 5}, []int{1, 2, 3, 4, 5})

	// 边界：负数值
	check([]int{-100, -50, 100, 0}, []int{-100, -50, 100, 0})

	// 边界：满二叉树三层，层序就是 1..15
	levelOrder := []int{}
	for i := 1; i <= 15; i++ {
		levelOrder = append(levelOrder, i)
	}
	check(levelOrder, levelOrder)

	fmt.Println("all tests passed")
}
