/*
============================================================================
144. 二叉树的前序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-preorder-traversal/
分类：二叉树 · 遍历（递归 / 栈 / Morris）

给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

示例 1：
输入：root = [1,null,2,3]
输出：[1,2,3]

示例 2：
输入：root = [1,2,3,4,5,null,8,null,null,6,7,9]
输出：[1,2,4,5,6,7,3,8,9]

示例 3：
输入：root = []
输出：[]

示例 4：
输入：root = [1]
输出：[1]

提示：
树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100

进阶：递归算法很简单，你可以通过迭代算法完成吗？
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
解法一：递归【非最优】

官方方法一。前序遍历的定义就是「根 → 左 → 右」，直接按定义写递归：
  · 空节点返回；
  · 先把当前节点值加入答案；
  · 再递归左子树、递归右子树。

关键点：
  · 三个步骤的顺序就是前/中/后序的唯一区别，前序是「访问」放在最前面
  · Go 里用闭包 + 外层返回变量最省事；也可以用带指针的辅助函数
  · 递归深度 = 树高；最坏（链状树）为 O(n)

复杂度分析：
  时间 O(n)：每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高，平均 O(log n)、最坏 O(n)

解法二：迭代（显式栈模拟）【非最优】

官方方法二。把递归里「隐式的栈」显式写出来：
  · 只要当前节点非空，就访问它、把它压栈，然后一路向左走；
  · 走到空就弹出栈顶、转向它的右子树。
这正好对应递归「先处理根、再处理左、最后处理右」的展开顺序。

关键点：
  · 循环条件是 `栈非空 或 当前节点非空`，两个条件缺一不可
  · 入栈的时机是「访问节点时」，出栈后立刻转向它的右子树
  · 与中序遍历的代码只差一行：前序在入栈前就记录答案，中序在出栈时记录
  · 栈的深度最多为树高，同样是 O(h)

复杂度分析：
  时间 O(n)
  空间 O(h)：显式栈

解法三：Morris 遍历【最优解】

官方方法三。前序、中序、后序都能用 Morris 做到 O(1) 额外空间（不计答案数组），
思路是利用叶子节点空着的 Right 指针当「回退线索」：
  · 若当前节点没有左子节点：直接访问它，然后走向右子节点；
  · 若有左子节点：在左子树里找到当前节点在中序下的前驱（左子树的最右节点）。
      - 前驱的 Right 为空 → 说明第一次来到这里：把 Right 指向当前节点（建立线索），
        访问当前节点，然后走向左子节点；
      - 前驱的 Right 指向当前节点 → 说明左子树已经遍历完：把线索拆掉（恢复原树），
        走向右子节点。
每个有左子树的节点会被访问两次，无左子树的节点只访问一次，总代价仍是 O(n)。

关键点：
  · 前序与中序的 Morris 差别：前序在「建立线索」时就访问当前节点，
    中序在「拆除线索」时才访问
  · 拆线索这一步必须做，否则会破坏原树结构（本题虽然不检查，但这是正确性的一部分）
  · 找前驱时循环条件是 `p2.Right != nil && p2.Right != p1`，
    两个条件都要有：既要走到最右，又要在已经建立线索时及时停下
  · 建立线索后要 continue，因为此时 p1 已经变成左子节点，不能再往下走 p1.Right

以 root = [1,null,2,3] 为例（1 只有右子树 2，2 的左孩子是 3）：
  递归/迭代：1 → 右子树 2 → 2 的左孩子 3，得到 [1,2,3]
  Morris：1 没有左孩子 → 访问 1，走向 2；2 有左孩子 3，3 无右孩子 →
          建立线索 3.Right=2，访问 2，走向 3；3 无左孩子 → 访问 3，走向线索 2；
          回到 2 时拆线索，走向 2 的右孩子（空）→ 结束。得到 [1,2,3]

复杂度分析：
  时间 O(n)：每个节点至多被访问两次
  空间 O(1)：只用了两个指针，不改动额外数据结构
*/

// ---------- 解法一：递归 ----------
func preorderTraversalRecursive(root *TreeNode) []int {
	vals := []int{}
	var preorder func(*TreeNode)
	preorder = func(node *TreeNode) {
		if node == nil {
			return
		}
		vals = append(vals, node.Val)
		preorder(node.Left)
		preorder(node.Right)
	}
	preorder(root)
	return vals
}

// ---------- 解法二：迭代（栈） ----------
func preorderTraversalIterative(root *TreeNode) []int {
	vals := []int{}
	stack := []*TreeNode{}
	node := root
	for node != nil || len(stack) > 0 {
		for node != nil {
			vals = append(vals, node.Val)
			stack = append(stack, node)
			node = node.Left
		}
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node = node.Right
	}
	return vals
}

// ---------- 解法三：Morris 遍历（最优解） ----------
func preorderTraversal(root *TreeNode) []int {
	vals := []int{}
	p1 := root
	for p1 != nil {
		p2 := p1.Left
		if p2 != nil {
			// 找左子树中最右节点（当前节点在中序下的前驱）
			for p2.Right != nil && p2.Right != p1 {
				p2 = p2.Right
			}
			if p2.Right == nil {
				// 第一次到达：建立线索，访问当前节点，转向左子树
				vals = append(vals, p1.Val)
				p2.Right = p1
				p1 = p1.Left
				continue
			}
			// 第二次到达：拆除线索，转向右子树
			p2.Right = nil
		} else {
			vals = append(vals, p1.Val)
		}
		p1 = p1.Right
	}
	return vals
}

// ===== 测试辅助：按层序数组建树（nullNode 表示空） =====
const nullNode = 1000 // 节点值范围 [-100,100]

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

func check(levelOrder []int, want []int) {
	// 每跑一种解法都重新建树：Morris 会临时改动指针
	if got := preorderTraversalRecursive(buildTree(levelOrder)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("递归(%v) = %v, want %v", levelOrder, got, want))
	}
	if got := preorderTraversalIterative(buildTree(levelOrder)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("迭代(%v) = %v, want %v", levelOrder, got, want))
	}
	if got := preorderTraversal(buildTree(levelOrder)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("Morris(%v) = %v, want %v", levelOrder, got, want))
	}
}

// 右斜链：只填右孩子
func rightChain(n int) []int {
	a := []int{}
	for i := 1; i <= n; i++ {
		a = append(a, i, nullNode)
	}
	return a
}

func main() {
	// 示例 1
	check([]int{1, nullNode, 2, 3}, []int{1, 2, 3})

	// 示例 2
	check([]int{1, 2, 3, 4, 5, nullNode, 8, nullNode, nullNode, 6, 7, 9},
		[]int{1, 2, 4, 5, 6, 7, 3, 8, 9})

	// 示例 3：空树
	check([]int{}, []int{})

	// 示例 4：单节点
	check([]int{1}, []int{1})

	// 边界：只有右子树链
	check([]int{1, nullNode, 2, nullNode, 3}, []int{1, 2, 3})

	// 边界：完全二叉树
	check([]int{1, 2, 3, 4, 5, 6, 7}, []int{1, 2, 4, 5, 3, 6, 7})

	// 边界：负数值
	check([]int{-100, -50, 100, 0}, []int{-100, -50, 0, 100})

	// 边界：深层右斜链（深度 100）
	want := []int{}
	for i := 1; i <= 100; i++ {
		want = append(want, i)
	}
	check(rightChain(100), want)

	fmt.Println("all tests passed")
}
