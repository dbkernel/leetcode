/*
============================================================================
LCR 153 / 剑指 Offer 34. 二叉树中和为某一值的路径（中等）
来源：剑指 Offer（第 2 版）
链接：https://leetcode.cn/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/
分类：二叉树 · DFS 回溯（根到叶子路径）

给你二叉树的根节点 root 和一个整数目标和 targetSum ，找出所有 从根节点到
叶子节点 路径总和等于给定目标和的路径。

叶子节点 是指没有子节点的节点。

示例 1：
输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
输出：[[5,4,11,2],[5,8,4,5]]

示例 2：
输入：root = [1,2,3], targetSum = 5
输出：[]

示例 3：
输入：root = [1,2], targetSum = 0
输出：[]

提示：
- 树中节点总数在范围 [0, 5000] 内
- -1000 <= Node.val <= 1000
- -1000 <= targetSum <= 1000
*/
package main

import (
	"fmt"
	"sort"
)

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
解法：DFS 回溯（进入压栈，离开弹栈）【最优解】

为什么慢：要枚举所有根到叶子的路径，路径条数最坏是 O(n)（叶子数），每条
路径长 O(h)。若每到一个叶子都从头重建一次路径，或者每层都把 path 拷贝一份
往下传，重复付出的代价会到 O(n*h)；而答案本身最多也只有 O(n*h) 个元素。

换个角度：所有根到叶子的路径共享前缀。维护一条 path，进入节点时压入、
离开时弹出，它自然就等于"当前根到节点的链"。命中时只拷贝一次，
没命中就原地复用，前缀部分不再重复构造。

关键点：

	· 弹出 path 末尾元素必须做，且放在左右递归之后 —— 漏掉它，兄弟分支
	  会带着上一分支的尾巴，路径越积越长
	· 只在该节点是叶子（左右都空）且剩余和恰为 0 时收集；在非叶子处收集，
	  会把"根到中间节点"误当成一条完整路径
	· 剩余和用 targetSum 沿途递减，避免每次重新求和
	· 值可以为负，remain < 0 不能剪枝，remain == 0 也不能提前返回 ——
	  后面接着负数仍可能把和拉回 0
	· Go 的切片共享底层数组，收集时必须拷贝，否则后续 append 会改写已存答案
	· 用闭包捕获 path 与 ans，避免额外定义结构体

手推一遍：以示例 1（root = [5,4,8,11,null,13,4,7,2,null,null,5,1]，
targetSum = 22）为例：

	5 → 4 → 11 → 7，remain = -5，是叶子但归不了零，弹出 7
	回到 11 → 2，remain = 0，是叶子 → 收集 [5,4,11,2]，再弹出 2、11、4
	5 → 8 → 13，remain = -4，是叶子，弹出
	回到 8 → 4 → 5，remain = 0，是叶子 → 收集 [5,8,4,5]
	回到根，path 清空 → 共两条 ✔

复杂度分析：

	时间 O(n^2)：节点进出各一次是 O(n)，但每次命中要拷贝长度 O(h) 的路径，
	  最坏（斜链且处处命中）为 O(n^2)，输出本身也有这么多元素
	空间 O(h)：path 与递归栈都只保存当前一条根到叶子的链，最坏 O(n)
*/
func pathSum(root *TreeNode, targetSum int) [][]int {
	path := []int{}
	ans := [][]int{}

	var dfs func(node *TreeNode, remain int)
	dfs = func(node *TreeNode, remain int) {
		if node == nil {
			return
		}

		path = append(path, node.Val)
		remain -= node.Val

		// 叶子且差值归零，才是一条符合题意的路径
		if node.Left == nil && node.Right == nil && remain == 0 {
			// 拷贝：path 还会被后续分支改写
			cp := make([]int, len(path))
			copy(cp, path)
			ans = append(ans, cp)
		}

		// 负值存在，两条分支都必须走，不能按 remain 剪枝
		dfs(node.Left, remain)
		dfs(node.Right, remain)

		// 撤销压栈：兄弟分支不能继承这条路径的尾巴
		path = path[:len(path)-1]
	}
	dfs(root, targetSum)
	return ans
}

// ===== 测试辅助：按层序数组建树、路径归一化 =====
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

// 题面没有约定路径顺序，统一排序后再比较
func normalize(paths [][]int) [][]int {
	for _, p := range paths {
		sort.Ints(p)
	}
	sort.Slice(paths, func(i, j int) bool {
		a, b := paths[i], paths[j]
		for k := 0; k < len(a) && k < len(b); k++ {
			if a[k] != b[k] {
				return a[k] < b[k]
			}
		}
		return len(a) < len(b)
	})
	return paths
}

func samePaths(a, b [][]int) bool {
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

func check(levelOrder []int, targetSum int, want [][]int) {
	got := pathSum(buildTree(levelOrder), targetSum)
	if !samePaths(normalize(got), normalize(want)) {
		panic(fmt.Sprintf("pathSum(%v, %d) = %v, want %v",
			levelOrder, targetSum, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{5, 4, 8, 11, nullNode, 13, 4, 7, 2, nullNode, nullNode, 5, 1},
		22, [][]int{{5, 4, 11, 2}, {5, 8, 4, 5}})

	// 示例 2
	check([]int{1, 2, 3}, 5, [][]int{})

	// 示例 3：目标和为 0，但两条路径分别是 1 和 3，都不等于 0
	check([]int{1, 2}, 0, [][]int{})

	// 边界：空树
	check([]int{}, 0, [][]int{})

	// 反直觉：单个 0 既是叶子又等于目标和，应收集一条路径
	check([]int{0}, 0, [][]int{{0}})

	// 反直觉：正负抵消，1 + (-1) = 0，路径跨两层
	check([]int{1, -1}, 0, [][]int{{1, -1}})

	// 反直觉：全负值，目标和也为负
	check([]int{-2, nullNode, -3}, -5, [][]int{{-2, -3}})

	// 关键点：根节点和已等于 1，但它不是叶子，不能收集
	check([]int{1, 2, 2}, 1, [][]int{})

	// 关键点：remain 中途归零后还要继续走（后续节点是负数）
	check([]int{1, -1, -1}, 0, [][]int{{1, -1}, {1, -1}})

	// 关键点：左分支结束后必须弹干净，右分支路径不能带左分支的值
	check([]int{1, 2, 3, 4}, 7, [][]int{{1, 2, 4}})

	// 边界：200 层斜链全为 1，目标和 200 只有一整条路径
	chain := []int{}
	wantPath := []int{}
	for i := 1; i <= 200; i++ {
		chain = append(chain, 1)
		wantPath = append(wantPath, 1)
		if i < 200 {
			chain = append(chain, nullNode)
		}
	}
	check(chain, 200, [][]int{wantPath})

	fmt.Println("all tests passed")
}
