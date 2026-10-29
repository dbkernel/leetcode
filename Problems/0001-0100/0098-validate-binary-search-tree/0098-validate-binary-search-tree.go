/*
============================================================================
98. 验证二叉搜索树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/validate-binary-search-tree/
分类：二叉搜索树 · 递归 / 中序遍历 / Morris

给你一个二叉树的根节点 root ，判断其是否是一个有效的二叉搜索树。

有效 二叉搜索树定义如下：
- 节点的左子树只包含 严格小于 当前节点的数。
- 节点的右子树只包含 严格大于 当前节点的数。
- 所有左子树和右子树自身必须也是二叉搜索树。

示例 1：
输入：root = [2,1,3]
输出：true

示例 2：
输入：root = [5,1,4,null,null,3,6]
输出：false
解释：根节点的值是 5 ，但是右子节点的值是 4 。

提示：
- 树中节点数目范围在 [1, 10^4] 内
- -2^31 <= Node.val <= 2^31 - 1
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
解法一：递归 + 上下界【非最优】

为什么慢：很多人第一反应是"只比较父子"—— 判断 left.Val < root.Val 且
root.Val < right.Val。但 BST 约束的是**整棵子树**的取值范围，不只是直接
父子。比如 [5,1,4,null,null,3,6] 里 4 是 5 的右孩子，比 5 小，直接父子比较
能发现；但在 [10,5,15,null,null,6,20] 里，6 是 15 的左孩子、比 15 小，
父子比较会放过它，然而 6 < 10 违反根约束。仅靠父子比较是不完备的。

换个角度：给每个节点带上"它必须落在的开区间 (lower, upper)"：
  · 根的范围是 (-∞, +∞)；
  · 进入左子树时，上界收紧为当前值；
  · 进入右子树时，下界收紧为当前值；
  · 任何节点不满足 `lower < val < upper` 即失败。
  这等价于"中序遍历必须严格递增"，但不需要真正遍历序列。

关键点：
  · 边界必须用 64 位：节点值可以取到 ±2^31，用 32 位会溢出；
    Go 的 int 在 64 位平台是 64 位，这里显式用 int64 更稳妥
  · 判断用严格不等号：BST 不允许重复值
  · 只递归一层不够，必须把收紧后的界传下去
  · 空节点返回 true，作为递归出口

以示例 2（root = [5,1,4,null,null,3,6]）为例：
  5 的区间 (-∞, +∞)；
  4 作为右孩子，区间 (5, +∞)，4 不满足 → false ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：中序遍历（显式栈）【非最优】

官方方法二。BST 的中序遍历结果必须严格递增。于是：
  · 用栈做迭代中序：一路向左压栈，弹出时比较当前值与前一个值；
  · 若 `cur <= prev` 则返回 false；否则更新 prev，转向右子树。
  不需要真的保存整个序列，只留前一个值。

关键点：
  · prev 用 int64，并额外用 bool 标记"是否已有前驱"，
    避免把第一个节点和 MinInt32 比较出错
  · 比较必须是 `>`，等于也不合法
  · 迭代顺序：压左 → 弹栈处理 → 转右，弹栈时才是"访问"
  · 可以在发现逆序时提前返回，省掉剩余遍历

复杂度分析：
  时间 O(n)：每个节点入栈、出栈各一次
  空间 O(h)：栈只保存当前路径

解法三：Morris 中序遍历【最优解】

官方方法三。中序遍历之所以需要栈，是因为要记住"从某个节点回来后该去右子树"。
Morris 用叶子节点的空右指针把这条"返回路径"直接存进树里（线索）：
  · 若当前节点没有左孩子，直接访问它，转向右孩子；
  · 否则找左子树的最右节点 p（中序前驱）：
      - p.Right 为空 → 指向当前节点（建立线索），当前节点向左走；
      - p.Right 就是当前节点 → 说明左子树已遍历完，断开线索（还原树），
        访问当前节点，转向右孩子。
  访问节点时比较它与前驱的值是否严格递增即可。

关键点：
  · 线索的建立与断开必须成对出现，否则会破坏输入树
  · 找最右节点时 `p.Right != node` 是循环条件，
    防止在已有线索的树上绕圈
  · 判断是否逆序发生在"访问"位置，与普通中序完全一致
  · 发现逆序时先用 bool 记下来、继续走完全程再返回：
    提前 return 会把"已建立但未断开"的线索留在树里，树就被改坏了
  · 全程只用几个指针，辅助空间 O(1)，因此是最优解

以示例 1（root = [2,1,3]）为例：
  node=2 有左孩子，最右节点是 1，1.Right 为空 → 建线索 1.Right=2，向左走到 1；
  node=1 没有左孩子 → 访问 1（prev 为空），转向右，回到 2；
  node=2 的最右节点 1 的 Right 正好指向 2 → 断开线索，访问 2（1 < 2 成立），转右；
  node=3 无左孩子 → 访问 3（2 < 3 成立）→ 中序 1,2,3 严格递增 → true ✔

复杂度分析：
  时间 O(n)：每条边最多走两遍（建线索、断线索）
  空间 O(1)：只用了几个指针，不借助栈

-------- 三种写法对比 --------
  递归上下界：O(n) 时间、O(h) 空间，最直观，适合面试第一版
  中序栈：O(n) 时间、O(h) 空间，把"BST ⇔ 中序递增"直接用出来
  Morris：O(n) 时间、O(1) 空间，标记为【最优解】，但会临时改动树结构
*/

// 解法一：递归 + 上下界
func isValidBST(root *TreeNode) bool {
	var check func(node *TreeNode, lower, upper int64) bool
	check = func(node *TreeNode, lower, upper int64) bool {
		if node == nil {
			return true
		}
		v := int64(node.Val)
		if v <= lower || v >= upper {
			return false
		}
		return check(node.Left, lower, v) && check(node.Right, v, upper)
	}
	return check(root, math.MinInt64, math.MaxInt64)
}

// 解法二：中序遍历（显式栈）
func isValidBSTInorder(root *TreeNode) bool {
	stk := []*TreeNode{}
	node := root
	var prev int64
	hasPrev := false

	for node != nil || len(stk) > 0 {
		for node != nil {
			stk = append(stk, node)
			node = node.Left
		}
		node = stk[len(stk)-1]
		stk = stk[:len(stk)-1]

		v := int64(node.Val)
		if hasPrev && v <= prev {
			return false
		}
		prev = v
		hasPrev = true

		node = node.Right
	}
	return true
}

// 解法三：Morris 中序遍历
func isValidBSTMorris(root *TreeNode) bool {
	node := root
	var prev *TreeNode
	valid := true

	for node != nil {
		if node.Left == nil {
			// 访问 node
			if prev != nil && prev.Val >= node.Val {
				valid = false
			}
			prev = node
			node = node.Right
		} else {
			// 找左子树的最右节点（中序前驱）
			p := node.Left
			for p.Right != nil && p.Right != node {
				p = p.Right
			}

			if p.Right == nil {
				// 建立线索，向左
				p.Right = node
				node = node.Left
			} else {
				// 左子树走完，断开线索，访问 node，转右
				p.Right = nil
				if prev != nil && prev.Val >= node.Val {
					valid = false
				}
				prev = node
				node = node.Right
			}
		}
	}
	// 注意：即使发现逆序也走完全程，把所有线索还原，保持输入树完整
	return valid
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树 =====
const nullNode = math.MaxInt32 + 1 // int 为 64 位，可用超出 int32 的哨兵

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

func check(input []int, want bool) {
	if got := isValidBST(buildTree(input)); got != want {
		panic(fmt.Sprintf("recursive(%v) = %v, want %v", input, got, want))
	}
	if got := isValidBSTInorder(buildTree(input)); got != want {
		panic(fmt.Sprintf("inorder(%v) = %v, want %v", input, got, want))
	}
	if got := isValidBSTMorris(buildTree(input)); got != want {
		panic(fmt.Sprintf("morris(%v) = %v, want %v", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{2, 1, 3}, true)

	// 示例 2
	check([]int{5, 1, 4, nullNode, nullNode, 3, 6}, false)

	// 边界：单节点
	check([]int{1}, true)

	// 边界：只有左孩子，且小于根
	check([]int{2, 1}, true)

	// 边界：只有右孩子，但小于根
	check([]int{2, nullNode, 1}, false)

	// 边界：重复值不合法（BST 要求严格）
	check([]int{2, 2, 2}, false)
	check([]int{1, 1}, false)

	// 边界：根满足、子树内部违反 —— 只比较父子会漏掉
	check([]int{10, 5, 15, nullNode, nullNode, 6, 20}, false)

	// 边界：右子树里的节点小于根
	check([]int{5, 4, 6, nullNode, nullNode, 3, 7}, false)

	// 边界：合法满 BST
	check([]int{3, 1, 5, 0, 2, 4, 6}, true)

	// 边界：满 BST 中右子树放了个小值
	check([]int{3, 1, 5, 0, 2, 6, 4}, false)

	// 边界：int 极值，验证上下界是否溢出
	check([]int{math.MinInt32}, true)
	check([]int{math.MaxInt32}, true)
	check([]int{math.MinInt32, nullNode, math.MaxInt32}, true)
	check([]int{math.MinInt32, math.MinInt32}, false)

	// 边界：100 层右斜链，递增合法
	chain := []int{1}
	for i := 2; i <= 100; i++ {
		chain = append(chain, nullNode, i)
	}
	check(chain, true)

	fmt.Println("all tests passed")
}
