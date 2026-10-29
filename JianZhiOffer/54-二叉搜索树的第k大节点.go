/*
============================================================================
LCR 174 / 剑指 Offer 54. 寻找二叉搜索树中的目标节点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/
分类：二叉搜索树 · 反序中序遍历

某公司组织架构以二叉搜索树形式记录，节点值为处于该职位的员工编号。
请返回第 cnt 大的员工编号。

示例 1：
输入：root = [7, 3, 9, 1, 5], cnt = 2
       7
      / \
     3   9
    / \
   1   5
输出：7

示例 2：
输入: root = [10, 5, 15, 2, 7, null, 20, 1, null, 6, 8], cnt = 4
       10
      /  \
     5    15
    / \     \
   2   7     20
  /   / \
 1   6   8
输出：8

提示：
- 1 ≤ cnt ≤ 二叉搜索树元素个数
*/
package main

import (
	"fmt"
	"math"
)

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
解法一：反序中序遍历（右 → 根 → 左）【最优解】

为什么慢：最容易想到的是"中序遍历把整棵树存成升序数组，再取倒数第 cnt 个"，
时间是 O(n) 但要多开 O(n) 的数组；如果先求"总节点数 n，再找第 n-cnt+1 小"，
还得额外遍历一次。

换个角度：BST 的中序是升序，"右 → 根 → 左"的遍历就是降序。按降序数到
第 cnt 个就是答案，数到就可以停，不必访问剩余节点。
  · 递归时先走右子树，再处理当前节点，最后走左子树；
  · 每处理一个节点就 `cnt--`，减到 0 时记下答案并停止。

关键点：
  · 顺序是"右、根、左"，写成"左、根、右"就变成第 cnt 小了
  · 计数必须在访问当前节点时做，不能在进入函数时做
  · 找到答案后要能及时收手：`cnt == 0` 时直接 return，
    否则会继续遍历把 cnt 减成负数
  · 用闭包捕获 ans / cnt 两个变量，函数签名只返回 int
  · 题目保证 1 ≤ cnt ≤ 节点数，因此一定能找到答案

以示例 1（root = [7,3,9,1,5]，cnt = 2）为例：
  右子树 9 → 访问 9，cnt 变 1；
  回到根 7 → 访问 7，cnt 变 0 → 答案是 7 ✔

复杂度分析：
  时间 O(n)：最坏要走到最小节点；平均只需访问前 cnt 大的一小部分
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：迭代中序（右根左）【非最优】

把解法一改成显式栈：先一路向右压栈，弹出时计数，再转向左子树。
  · 这样能避免递归，思路与解法一完全一致；
  · 代码稍长，好处是不受递归深度限制（虽然本题数据量也用不上）。

关键点：
  · 压栈方向是"向右"，弹栈时才是"访问"，别把方向写反
  · 计数到 0 即刻 break / return，栈里剩下的节点不用再管
  · 与解法一相同，顺序必须是右 → 根 → 左

复杂度分析：
  时间 O(n)
  空间 O(h)：栈保存当前路径
*/

// 解法一：反序中序遍历（最优解）
func findTargetNode(root *TreeNode, cnt int) int {
	ans := 0
	remain := cnt

	var dfs func(node *TreeNode)
	dfs = func(node *TreeNode) {
		if node == nil || remain == 0 {
			return
		}

		dfs(node.Right)

		remain--
		if remain == 0 {
			ans = node.Val
			return
		}

		dfs(node.Left)
	}

	dfs(root)
	return ans
}

// 解法二：迭代中序（右根左）
func findTargetNodeIterative(root *TreeNode, cnt int) int {
	stk := []*TreeNode{}
	node := root

	for node != nil || len(stk) > 0 {
		for node != nil {
			stk = append(stk, node)
			node = node.Right
		}
		node = stk[len(stk)-1]
		stk = stk[:len(stk)-1]

		cnt--
		if cnt == 0 {
			return node.Val
		}

		node = node.Left
	}
	return -1 // 题目保证不会走到这里
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = math.MaxInt32 // 节点值均非负且远小于哨兵

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

func check(input []int, cnt, want int) {
	if got := findTargetNode(buildTree(input), cnt); got != want {
		panic(fmt.Sprintf("recursive(%v, %d) = %d, want %d", input, cnt, got, want))
	}
	if got := findTargetNodeIterative(buildTree(input), cnt); got != want {
		panic(fmt.Sprintf("iterative(%v, %d) = %d, want %d", input, cnt, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{7, 3, 9, 1, 5}, 2, 7)

	// 示例 2
	check([]int{10, 5, 15, 2, 7, nullNode, 20, 1, nullNode, 6, 8}, 4, 8)

	// 边界：单节点
	check([]int{1}, 1, 1)

	// 剑指 Offer 原示例 1：root = [3,1,4,null,2], k = 1
	check([]int{3, 1, 4, nullNode, 2}, 1, 4)

	// 剑指 Offer 原示例 2：root = [5,3,6,2,4,null,null,1], k = 3
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 3, 4)

	// 边界：取最小值（cnt = 节点总数）
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 6, 1)

	// 边界：cnt = 1 就是最大值
	check([]int{5, 3, 6, 2, 4, nullNode, nullNode, 1}, 1, 6)

	// 边界：只有右孩子的 BST，降序就是 3、2、1
	check([]int{1, nullNode, 2, nullNode, 3}, 2, 2)

	// 边界：只有左孩子的 BST，降序就是 3、2、1
	check([]int{3, 2, nullNode, 1}, 3, 1)

	// 边界：100 层右斜链，值 1..100
	chain := []int{1}
	for i := 2; i <= 100; i++ {
		chain = append(chain, nullNode, i)
	}
	check(chain, 1, 100)
	check(chain, 100, 1)
	check(chain, 50, 51)

	fmt.Println("all tests passed")
}
