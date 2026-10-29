/*
============================================================================
337. 打家劫舍 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/house-robber-iii/
分类：动态规划 · 树形 DP

小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为 root 。

除了 root 之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果 两个直接相连的房子在同一天晚上被打劫 ，房屋将自动报警。

给定二叉树的 root 。返回 在不触动警报的情况下 ，小偷能够盗取的最高金额 。

示例 1:
输入: root = [3,2,3,null,3,null,1]
输出: 7 
解释: 小偷一晚能够盗取的最高金额 3 + 3 + 1 = 7

示例 2:
输入: root = [3,4,5,1,3,null,1]
输出: 9
解释: 小偷一晚能够盗取的最高金额 4 + 5 = 9

提示：
- 树的节点数在 [1, 10^4] 范围内
- 0 <= Node.val <= 10^4
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
解法一：树上 DP，后序一次返回 (选, 不选)【最优解】

为什么慢：把数组上的递推照搬到树上，最直白的式子是对每个节点比较
"选它 + 孙子们的最优" 和 "两个孩子的最优"。但"两个孩子的最优"本身又要
递归求孙子，同一个子树会在两条分支里各展开一次，节点被重复进入的次数
随深度指数增长。

换个角度：每个节点只需要向父节点汇报两个数——以它为根的子树里，
"选它"的最大值与"不选它"的最大值。父节点拿到孩子的这两个数就能自己
做决定，完全不需要知道子树内部怎么选的。于是后序一次遍历即可。

关键点：
  · 选 x 时孩子只能"不选"：take(x) = x.Val + skip(左) + skip(右)
  · 不选 x 时孩子选不选都行：skip(x) = max(take(左), skip(左))
    + max(take(右), skip(右))
  · 返回二元组的两个分量顺序必须在整棵树里统一，父子两侧写反是这类题
    最常见的错
  · 空节点返回 (0, 0)：缺失的孩子既不能被选，也不会贡献金额
  · 答案是根节点两个值的 max，不能只看"选根"那一个
  · Go 用两个返回值的函数表达二元组，不需要额外结构体

以示例 1（root = [3,2,3,null,3,null,1]）为例，从叶子往上：
  val=3 的叶子： (3, 0)
  左孩子 2（右孩子是上面那个 3）：take = 2 + 0 + 0 = 2，
    skip = max(3,0) + 0 = 3 → (2, 3)
  右孩子 3（右孩子是 val=1 的叶子）：take = 3 + 0 + 0 = 3，
    skip = 0 + max(1,0) = 1 → (3, 1)
  根 3：take = 3 + skip(左)=3 + skip(右)=1 = 7；
    skip = max(2,3) + max(3,1) = 6 → max(7,6) = 7 ✔

复杂度分析：
  时间 O(n)：每个节点只被后序访问一次，汇报两个整数
  空间 O(h)：递归栈等于树高，斜树最坏 O(n)，平衡树 O(log n)
*/
func robTree(root *TreeNode) int {
	var dfs func(node *TreeNode) (int, int)
	// 返回 (选当前节点的最大值, 不选当前节点的最大值)
	dfs = func(node *TreeNode) (int, int) {
		if node == nil {
			return 0, 0
		}

		lt, ls := dfs(node.Left)
		rt, rs := dfs(node.Right)

		// 选当前节点：两个孩子都只能跳过
		take := node.Val + ls + rs
		// 不选当前节点：每个孩子都可以在"选"与"不选"之间取较优
		skip := maxInt(lt, ls) + maxInt(rt, rs)
		return take, skip
	}

	take, skip := dfs(root)
	return maxInt(take, skip)
}

func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}

/*
解法二：记忆化递归，按节点缓存它的最优值【非最优】

为什么慢：解法一的式子在"不选 x"时要取孩子的 max，"选 x"时要取孙子的值，
如果直接递归而不缓存，同一个节点的最优值会被它的父、祖父、曾祖父反复
求解，深度为 h 的链上退化成指数级。

换个角度：把"以 x 为根的子树的最优值"当成唯一状态，算过就存。于是与解法
一一样每节点只算一次，但缓存用的是节点指针而不是返回值结构。

关键点：
  · 缓存键必须是节点指针，不能用 Val —— 值相同的节点是不同子树
  · 进入函数要立刻查表，查表放在递归调用之后等于没缓存
  · 取孙子时要判空：node.Left 本身可能为 nil，再取 .Left 会解引用空指针

复杂度分析：
  时间 O(n)：每个节点的最优值只计算一次
  空间 O(n)：哈希表给每个节点存一个整数，比解法一多一个数量级；
        递归栈 O(h) 相对次要
*/
func robMemo(root *TreeNode) int {
	memo := make(map[*TreeNode]int)

	var best func(node *TreeNode) int
	best = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		if v, ok := memo[node]; ok {
			return v
		}

		// 不选当前节点
		skip := best(node.Left) + best(node.Right)
		// 选当前节点：只能再从孙子们身上取
		take := node.Val
		if node.Left != nil {
			take += best(node.Left.Left) + best(node.Left.Right)
		}
		if node.Right != nil {
			take += best(node.Right.Left) + best(node.Right.Right)
		}

		ans := maxInt(take, skip)
		memo[node] = ans
		return ans
	}

	return best(root)
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = -1 // 节点值非负，用 -1 表示空位

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
	if got := robTree(buildTree(levelOrder)); got != want {
		panic(fmt.Sprintf("treeDP(%v) = %d, want %d", levelOrder, got, want))
	}
	if got := robMemo(buildTree(levelOrder)); got != want {
		panic(fmt.Sprintf("memo(%v) = %d, want %d", levelOrder, got, want))
	}
}

func main() {
	// 示例 1：3 + 3 + 1，根与右孩子、右孩子的右孩子互不相邻
	check([]int{3, 2, 3, nullNode, 3, nullNode, 1}, 7)

	// 示例 2：4 + 5 优于 3 + 1 + 3 + 1
	check([]int{3, 4, 5, 1, 3, nullNode, 1}, 9)

	// 边界：单节点，必选
	check([]int{1}, 1)

	// 边界：值为 0 的节点，答案是 0 而不是 -1
	check([]int{0}, 0)

	// 关键点用例：隔代能同选，根 5 与两个孙子 4、6 一起拿共 15
	check([]int{5, 1, nullNode, 4, 6}, 15)

	// 反直觉：3 与 4 分属不同分支，中间隔着一层，3+4=7 比 2+4=6 更好
	check([]int{2, 1, 3, nullNode, 4}, 7)

	// 关键点用例：全零树，任何选法都是 0
	check([]int{0, 0, 0, 0, 0}, 0)

	// 边界：右斜链 200 个节点，等价于 198 处理 [1..200]，偶数位之和更大
	levelOrder := []int{}
	for i := 1; i <= 200; i++ {
		levelOrder = append(levelOrder, i, nullNode)
	}
	check(levelOrder, 10100)

	fmt.Println("all tests passed")
}
