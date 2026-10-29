/*
============================================================================
102. 二叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-level-order-traversal/
分类：二叉树 · 层序遍历（BFS / 队列）

给你二叉树的根节点 root ，返回其节点值的 层序遍历 。
（即逐层地，从左到右访问所有节点）。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：[[3],[9,20],[15,7]]

示例 2：
输入：root = [1]
输出：[[1]]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目在范围 [0, 2000] 内
- -1000 <= Node.val <= 1000
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
解法：BFS + 按层定长出队【最优解】

为什么慢：最朴素的 BFS 只知道「下一个是谁」，并不知道「这一层到哪儿结束」。
如果给每个节点额外挂一个层号，再用哈希表按层号分组，就要多存 O(n) 个层号；
最后还要按下标顺序遍历哈希表，空间和实现都变重。

换个角度：BFS 的队列有一个几乎白送的性质 —— 某一时刻队列里的元素，恰好就是
「当前这一层剩下的节点 + 下一层已经入队的节点」。因此只要在每轮开始时先记下
`s = len(q)`，就锁定了本层的边界：接下来连续出队 s 个，得到的就是完整的一层，
而这 s 个节点新入队的恰好全是下一层。

关键点：
  · `s` 必须在内层循环之前取，循环中队列长度会增长，不能写成 `i < len(q)`
  · 每轮先 `append(res, []int{})` 占好一层的位置，再往这一层里追加
  · 入队仍要「先左后右」，并且跳过空孩子，才能保证同层从左到右
  · 根为空直接返回空切片：不要入队再靠循环兜底
  · 空树不会进入循环，所以不会多出一个空层

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  第 1 轮 s=1 → 出 3，入 9、20 → [[3]]
  第 2 轮 s=2 → 出 9，入空；出 20，入 15、7 → [[3],[9,20]]
  第 3 轮 s=2 → 出 15、7 → [[3],[9,20],[15,7]]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案数组外，队列最多同时存两层节点
*/
func levelOrder(root *TreeNode) [][]int {
	res := [][]int{}
	if root == nil {
		return res
	}

	q := []*TreeNode{root}
	for head := 0; head < len(q); {
		// 本层节点个数必须在出队前锁定，循环中队列还会增长
		levelSize := len(q) - head
		level := make([]int, 0, levelSize)
		for i := 0; i < levelSize; i++ {
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
		res = append(res, level)
	}
	return res
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

func check(input []int, want [][]int) {
	if got := levelOrder(buildTree(input)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("levelOrder(%v) = %v, want %v", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{3, 9, 20, nullNode, nullNode, 15, 7},
		[][]int{{3}, {9, 20}, {15, 7}})

	// 示例 2：单节点
	check([]int{1}, [][]int{{1}})

	// 示例 3：空树
	check([]int{}, [][]int{})

	// 边界：完全二叉树，层宽递增
	check([]int{1, 2, 3, 4, 5, 6, 7}, [][]int{{1}, {2, 3}, {4, 5, 6, 7}})

	// 边界：左斜链 / 右斜链，每层只有一个节点
	check([]int{1, 2, nullNode, 3}, [][]int{{1}, {2}, {3}})
	check([]int{1, nullNode, 2, nullNode, 3}, [][]int{{1}, {2}, {3}})

	// 边界：缺左孩子，层内顺序不能乱
	check([]int{1, 2, 3, nullNode, 4, 5}, [][]int{{1}, {2, 3}, {4, 5}})

	// 边界：负数值
	check([]int{-1000, -500, 1000, 0},
		[][]int{{-1000}, {-500, 1000}, {0}})

	// 边界：满二叉树四层
	levelOrderInput := []int{}
	for i := 1; i <= 15; i++ {
		levelOrderInput = append(levelOrderInput, i)
	}
	want := [][]int{}
	start := 1
	for width := 1; width <= 8; width <<= 1 {
		level := []int{}
		for j := 0; j < width; j++ {
			level = append(level, start+j)
		}
		want = append(want, level)
		start += width
	}
	check(levelOrderInput, want)

	fmt.Println("all tests passed")
}
