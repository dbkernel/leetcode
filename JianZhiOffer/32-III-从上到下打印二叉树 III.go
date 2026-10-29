/*
============================================================================
LCR 151 / 剑指 Offer 32 - III. 从上到下打印二叉树 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/
分类：二叉树 · 层序遍历（BFS / 双端填充）

一棵圣诞树记作根节点为 root 的二叉树，节点值为该位置装饰彩灯的颜色编号。
请按照如下规则记录彩灯装饰结果：

- 第一层按照从左到右的顺序记录
- 除第一层外每一层的记录顺序均与上一层相反。即第一层为从左到右，
  第二层为从右到左。

说明：本题原名「从上到下打印二叉树 III」，leetcode.cn 现已改用
decorateRecord 作为函数名，题面意思不变。

示例 1：
输入：root = [8,17,21,18,null,null,6]
输出：[[8],[21,17],[18,6]]

提示：
- 节点总数 <= 1000
*/
package main

import (
	"fmt"
	"reflect"
)

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
解法一：BFS 按层收集 + 奇数层反转【非最优】

为什么慢：直接的 BFS 只能保证「从左到右」的顺序；要得到之字形，最顺手的
补丁是每层收完再整体翻转一次。代码最短，但奇数层的元素被多写了一遍，
每一层都是一次额外的 O(层宽) 操作，总多出 O(n) 的搬运。

换个角度：与其收完再整体翻转，不如「收的时候就放到正确的一端」。
一层要么全从头填，要么全从尾填，用双端队列（或预分配数组 + 首尾指针）
就能一步到位。

关键点：
  · 判断方向只能看层号，不能看节点值或队列长度
  · 反转只作用于奇数层，层号从 0 开始数
  · 保证本题正确性的核心仍是先左后右入队

复杂度分析：
  时间 O(n)：遍历 O(n)，最坏反转总量也不超过 O(n)
  空间 O(w)：队列与当前层

解法二：BFS + 按方向从两端填充【最优解】

按层处理，每层开始前记录本层节点数 size，然后：
  · 偶数层（0、2、4…）：从数组左端向右填，得到从左到右；
  · 奇数层（1、3、5…）：从数组右端向左填，得到从右到左。
孩子入队的方向（左、右）始终不变 —— 只翻转当前层的输出，不翻转遍历顺序。

关键点：
  · 需要翻转的是「当前层写入的位置」，而不是「孩子入队的顺序」
  · 遍历节点的顺序永远是左孩子在前，否则下一层的父子关系都会乱
  · size 必须在 for 循环之前锁定，循环中队列长度会增长
  · 预分配长度为 size 的数组，两个指针从两端向中间靠，天然 O(1) 写入；
    这等价于使用双端队列，但省掉了 deque 的额外封装

以示例 1（root = [8,17,21,18,null,null,6]）为例：
  第 0 层 size=1，从左往右填 → [8]
  第 1 层 size=2，从右往左填：17 放最后 → [_,17]，21 放它前面 → [21,17]
  第 2 层 size=2，从左往右填：18、6 → [18,6]
  → [[8],[21,17],[18,6]]

复杂度分析：
  时间 O(n)：每个节点入队、出队、写入当前层各一次
  空间 O(w)：BFS 队列与本层数组各占 O(w)
*/

// 解法一：BFS + 奇数层反转
func decorateRecordReverse(root *TreeNode) [][]int {
	res := [][]int{}
	if root == nil {
		return res
	}

	q := []*TreeNode{root}
	leftToRight := true
	for head := 0; head < len(q); {
		size := len(q) - head
		level := make([]int, 0, size)
		for i := 0; i < size; i++ {
			node := q[head]
			head++
			level = append(level, node.Val)
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		// 奇数层整体翻转
		if !leftToRight {
			for i, j := 0, len(level)-1; i < j; i, j = i+1, j-1 {
				level[i], level[j] = level[j], level[i]
			}
		}
		res = append(res, level)
		leftToRight = !leftToRight
	}
	return res
}

// 解法二：BFS + 按方向从两端填充（最优解）
func decorateRecord(root *TreeNode) [][]int {
	res := [][]int{}
	if root == nil {
		return res
	}

	q := []*TreeNode{root}
	leftToRight := true
	for head := 0; head < len(q); {
		// 本层节点个数必须在出队前锁定
		size := len(q) - head
		level := make([]int, size)
		lo, hi := 0, size
		for i := 0; i < size; i++ {
			node := q[head]
			head++

			// 只改变写入当前层的位置，孩子入队顺序始终先左后右
			if leftToRight {
				level[lo] = node.Val
				lo++
			} else {
				hi--
				level[hi] = node.Val
			}

			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		res = append(res, level)
		leftToRight = !leftToRight
	}
	return res
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

func check(input []int, want [][]int) {
	if got := decorateRecordReverse(buildTree(input)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("反转版(%v) = %v, want %v", input, got, want))
	}
	if got := decorateRecord(buildTree(input)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("双端填充(%v) = %v, want %v", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{8, 17, 21, 18, nullNode, nullNode, 6},
		[][]int{{8}, {21, 17}, {18, 6}})

	// 边界：空树
	check([]int{}, [][]int{})

	// 边界：单节点
	check([]int{1}, [][]int{{1}})

	// 边界：只有一层两个节点
	check([]int{1, 2, 3}, [][]int{{1}, {3, 2}})

	// 边界：满二叉树三层
	check([]int{1, 2, 3, 4, 5, 6, 7},
		[][]int{{1}, {3, 2}, {4, 5, 6, 7}})

	// 边界：满二叉树四层（第 4 层从右到左）
	levelOrder := []int{}
	for i := 1; i <= 15; i++ {
		levelOrder = append(levelOrder, i)
	}
	check(levelOrder, [][]int{{1}, {3, 2}, {4, 5, 6, 7},
		{15, 14, 13, 12, 11, 10, 9, 8}})

	// 边界：左斜链 / 右斜链，每层只有一个节点，方向翻转不影响结果
	check([]int{1, 2, nullNode, 3}, [][]int{{1}, {2}, {3}})
	check([]int{1, nullNode, 2, nullNode, 3}, [][]int{{1}, {2}, {3}})

	// 边界：缺左孩子，奇数层只有单个节点，验证不会重复写入
	check([]int{1, 2, 3, nullNode, 4, 5}, [][]int{{1}, {3, 2}, {4, 5}})

	// 边界：负数值
	check([]int{-100, -50, 100, 0}, [][]int{{-100}, {100, -50}, {0}})

	fmt.Println("all tests passed")
}
