/*
============================================================================
559. N 叉树的最大深度（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-depth-of-n-ary-tree/
分类：N 叉树 · 深度（DFS / BFS）

给定一个 N 叉树，找到其最大深度。

最大深度是指从根节点到最远叶子节点的最长路径上的节点总数。

N 叉树输入按层序遍历序列化表示，每组子节点由空值分隔（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：3

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：5

提示：
- 树的深度不会超过 1000 。
- 树的节点数目位于 [0, 10^4] 之间。
*/
package main

import "fmt"

// 力扣预置的节点定义（多叉树，注意孩子是一个数组）
type Node struct {
	Val      int
	Children []*Node
}

/*
解法一：DFS 后序递归【最优解】

为什么慢：BFS 按层数数也能出答案，但要维护一个队列，空间随最宽的那一层
增长；对 N 叉树来说，一个「星形」根节点就能把队列塞成 O(m)。
而且按层数数还要额外写「每层弹固定个数」的循环，容易写成每弹一个加一次。

换个角度：最大深度天然满足递归结构 —— 空树深度 0；非空树的深度是
`1 + max(每个孩子的深度)`。把「兄弟之间取最大」交给一次循环即可，
递归返回时顺手加一。

关键点：
  · 空节点返回 0，是递归边界，也是叶子返回 1 的来源
  · N 叉树没有「左右」之分，必须在 Children 数组里遍历取 max，
    不能只取第一个孩子（那是求某条特定路径，不是最大深度）
  · 孩子在数组中的顺序不影响答案，只影响遍历顺序
  · 递归深度 = 树高 h，题目保证 h ≤ 1000，不会炸栈

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  根 1 的孩子是 3、2、4；
  3 的孩子 5、6 都返回 1 → 3 返回 1 + max(1,1) = 2；
  2、4 无孩子，返回 1；
  根 1 返回 1 + max(2,1,1) = 3 ✔

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：BFS 按层计数【非最优】

官方方法二。层序遍历：每轮先锁定本层节点数 size，弹出 size 个节点并把
它们的孩子入队，然后 depth++；队列空时 depth 就是最大深度。

关键点：
  · size 必须在 for 之前取，循环中队列还会增长
  · 空树直接返回 0
  · 每层结束才 depth++，不能每弹一个节点就加
  · 空间 O(w)：w 为最大层宽，星形树时约为 m，比 DFS 差

复杂度分析：
  时间 O(m)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/

// 解法一：DFS 后序递归（最优解）
func maxDepth(root *Node) int {
	if root == nil {
		return 0
	}

	best := 0
	for _, child := range root.Children {
		if d := maxDepth(child); d > best {
			best = d
		}
	}
	return best + 1
}

// 解法二：BFS 按层计数
func maxDepthBFS(root *Node) int {
	if root == nil {
		return 0
	}

	depth := 0
	q := []*Node{root}
	for head := 0; head < len(q); {
		size := len(q) - head
		for i := 0; i < size; i++ {
			node := q[head]
			head++
			q = append(q, node.Children...)
		}
		depth++
	}
	return depth
}

// ===== 测试辅助：按层序数组建 N 叉树（nullNode 表示空） =====
const nullNode = 10001

// 层序序列的规则：根之后先来一个 null，然后是该节点的全部孩子；
// 每个孩子按 BFS 顺序继续展开，孩子列表以 null 结尾。
func buildNaryTree(levelOrder []int) *Node {
	if len(levelOrder) == 0 || levelOrder[0] == nullNode {
		return nil
	}

	root := &Node{Val: levelOrder[0]}
	nodes := []*Node{root}
	idx := 2 // 跳过根与它后面的 null
	p := 0
	for idx < len(levelOrder) && p < len(nodes) {
		parent := nodes[p]
		p++
		for idx < len(levelOrder) && levelOrder[idx] != nullNode {
			child := &Node{Val: levelOrder[idx]}
			parent.Children = append(parent.Children, child)
			nodes = append(nodes, child)
			idx++
		}
		idx++ // 跳过孩子列表的 null 结束符
	}
	return root
}

func check(input []int, want int) {
	if got := maxDepth(buildNaryTree(input)); got != want {
		panic(fmt.Sprintf("DFS(%v) = %d, want %d", input, got, want))
	}
	if got := maxDepthBFS(buildNaryTree(input)); got != want {
		panic(fmt.Sprintf("BFS(%v) = %d, want %d", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{1, nullNode, 3, 2, 4, nullNode, 5, 6}, 3)

	// 示例 2
	check([]int{1, nullNode, 2, 3, 4, 5, nullNode, nullNode, 6, 7, nullNode,
		8, nullNode, 9, 10, nullNode, nullNode, 11, nullNode, 12,
		nullNode, 13, nullNode, nullNode, 14},
		5)

	// 边界：空树
	check([]int{}, 0)

	// 边界：单节点
	check([]int{1}, 1)

	// 边界：根没有孩子
	check([]int{1, nullNode}, 1)

	// 边界：星形树，深度 2
	check([]int{1, nullNode, 2, 3, 4, 5, 6}, 2)

	// 边界：不能只取第一个孩子 —— 深度藏在第二个孩子下面
	check([]int{1, nullNode, 2, 3, nullNode, nullNode, 4, nullNode, 5, nullNode}, 4)

	// 边界：孩子顺序不影响答案
	check([]int{1, nullNode, 3, 2, 4, nullNode, 5, 6}, 3)

	// 边界：深链（每个节点只有一个孩子），深度 200
	levelOrder := []int{}
	for i := 1; i <= 200; i++ {
		levelOrder = append(levelOrder, i, nullNode)
	}
	check(levelOrder, 200)

	fmt.Println("all tests passed")
}
