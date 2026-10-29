/*
============================================================================
513. 找树左下角的值（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-bottom-left-tree-value/
分类：二叉树 · 层序/深度遍历（DFS / BFS）

给定一个二叉树的 根节点 root。

请找出该二叉树的 最底层 最左边 节点的值。

示例 1:
输入: root = [2,1,3]
输出: 1
解释: 最后一行是 [1, 3]，因此最左边的值是 1。

示例 2:
输入: [1,2,3,4,null,5,6,null,null,7]
输出: 7
解释: 最后一层只有节点 7。

提示:
- 二叉树的节点个数的范围是 [1, 10^4]
- -2^31 <= Node.val <= 2^31 - 1
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
解法一：DFS（先左后右，记录最大深度）【最优解】

为什么慢：直接按层来做当然可以，但「最底层」意味着必须把每一层都走完才知道
哪层最深；而且同一层还有「最左」的要求。如果每层都额外存下来再比较，
空间就白花了。

换个角度：只用一个变量记住「目前见过的最深深度」和「那个深度最左的值」。
按「先左后右」做 DFS，并且在回溯时判断当前深度是否严格大于已记录深度：
一旦某个节点刷新了最大深度，它一定就是该层最先被访问到的节点，也就是
最左边的节点。

关键点：
  · 必须先递归左、再递归右，否则同一层刷新最大深度的就不是最左节点
  · 判断用严格大于 `height > curHeight`：同一层的后续节点深度相同，不能覆盖
  · 判断写在两个递归之后（后序位置），此时 height 已经是当前节点的深度
  · 根节点存在（题目保证非空），返回值可以用指针一路带出来
  · Go 没有尾递归优化，递归深度 = 树高，本题最多 10^4，需注意栈增长（Go 栈会
    自动扩容，因此不会像 C 那样直接爆栈）

以示例 2（root = [1,2,3,4,null,5,6,null,null,7]）为例：
  先一路向左到 4（深度 3），刷新 curHeight=3、curVal=4
  回溯到 2，右空；回到 1，进右子树 3：先 5，再到 7（深度 4）刷新 curHeight=4
  再回溯 6（深度 3，不刷新）
  → 最底层最左是 7

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈；最坏（斜链）O(n)，平均 O(log n)

解法二：BFS（右孩子先入队，最后出队即答案）【非最优】

官方方法二。如果按「右 → 左」的顺序做层序遍历，那么每一层都是从右往左，
整体出队序列的最后一个节点，就是最底层、且是该层最靠左的节点。
  · 队列初始化放根；
  · 每次出队一个节点，先把右孩子入队、再入左孩子；
  · 用变量记录「最近一次出队的值」，循环结束时它就是答案。

关键点：
  · 入队顺序必须是「先右后左」：这样每层的最左节点会最后出队
  · 不需要记录层号，也不需要按层分组，最后一个出队的节点自然满足两个条件
  · 空孩子不要入队；题目保证根非空，所以 ret 一定会被赋值
  · 队列最坏要存一层节点，空间 O(w)，宽树时比 DFS 更费

以示例 2 为例：
  第 1 层出 1（入 3、2），第 2 层出 3（入 6、5）、2（入 4），
  第 3 层出 6、5（入 7）、4，第 4 层出 7 → 最后一个出队的是 7

复杂度分析：
  时间 O(n)
  空间 O(w)：w 为最大层宽
*/

// 解法一：DFS（最优解）
func dfsBottomLeft(node *TreeNode, height int, curVal, curHeight *int) {
	if node == nil {
		return
	}
	height++
	dfsBottomLeft(node.Left, height, curVal, curHeight)
	dfsBottomLeft(node.Right, height, curVal, curHeight)
	// 同一层只会被最先到达的节点刷新，因此一定是这一层最左的
	if height > *curHeight {
		*curHeight = height
		*curVal = node.Val
	}
}

func findBottomLeftValueDFS(root *TreeNode) int {
	curVal, curHeight := 0, 0
	dfsBottomLeft(root, 0, &curVal, &curHeight)
	return curVal
}

// 解法二：BFS，先右后左，最后出队即答案
func findBottomLeftValue(root *TreeNode) int {
	ret := 0
	q := []*TreeNode{root}
	for head := 0; head < len(q); head++ {
		node := q[head]
		// 先右后左：每层最左的节点最后出队
		if node.Right != nil {
			q = append(q, node.Right)
		}
		if node.Left != nil {
			q = append(q, node.Left)
		}
		ret = node.Val
	}
	return ret
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = 1 << 40 // 节点值范围到 2^31-1

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
	if got := findBottomLeftValueDFS(buildTree(input)); got != want {
		panic(fmt.Sprintf("DFS(%v) = %d, want %d", input, got, want))
	}
	if got := findBottomLeftValue(buildTree(input)); got != want {
		panic(fmt.Sprintf("BFS(%v) = %d, want %d", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{2, 1, 3}, 1)

	// 示例 2
	check([]int{1, 2, 3, 4, nullNode, 5, 6, nullNode, nullNode, 7}, 7)

	// 边界：只有一个节点
	check([]int{0}, 0)

	// 边界：只有左链，最底层最左是链尾
	check([]int{1, 2, nullNode, 3}, 3)

	// 边界：只有右链，最底层最左同样是链尾（每层只有一个节点）
	check([]int{1, nullNode, 2, nullNode, 3}, 3)

	// 边界：最底层最左节点在左子树上，而不是最靠右的叶子
	check([]int{1, 2, 3, 4, 5, 6, 7}, 4)

	// 边界：最底层只有一个节点，且是左孩子
	check([]int{1, 2, 3, 4}, 4)

	// 边界：最后一层的最左节点在右子树深处
	check([]int{1, 2, 3, nullNode, nullNode, 4, 5, 6}, 6)

	// 边界：负数值
	check([]int{-1, -2, -3}, -2)

	// 边界：深链（深度 200），最底层最左是最后一个节点
	levelOrder := []int{}
	for i := 1; i <= 200; i++ {
		levelOrder = append(levelOrder, i, nullNode)
	}
	check(levelOrder, 200)

	fmt.Println("all tests passed")
}
