/*
============================================================================
114. 二叉树展开为链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/
分类：二叉树 · 前序遍历 / 原地指针重连

给你二叉树的根结点 root ，请你将它展开为一个单链表：

  - 展开后的单链表应该同样使用 TreeNode ，其中 right 子指针指向链表中下一个
    结点，而左子指针始终为 null 。
  - 展开后的单链表应该与二叉树 先序遍历 顺序相同。

示例 1：
输入：root = [1,2,5,3,4,null,6]
输出：[1,null,2,null,3,null,4,null,5,null,6]

示例 2：
输入：root = []
输出：[]

示例 3：
输入：root = [0]
输出：[0]

提示：
- 树中结点数在范围 [0, 2000] 内
- -100 <= Node.val <= 100

进阶：你可以使用原地算法（O(1) 额外空间）展开这棵树吗？
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
解法一：前序遍历把节点存进切片，再按切片顺序重连【非最优】

为什么慢：遍历本身是 O(n)，额外代价是 O(n) 的节点切片，而且重连时又要把
切片扫一遍。题目进阶要求 O(1) 额外空间，这个切片正是要被去掉的东西。

换个角度：展开后的链表顺序就是前序序列。先把序列完整拿到手，再统一把每个
节点的 Left 清空、Right 指向序列里的下一个节点，指针就不会在遍历途中被
改坏。

关键点：
  · 必须先把整棵树的节点收集完再动指针：边遍历边改会丢掉还没访问的子树
  · 切片里存的是节点地址而不是值 —— 值会重复，地址才是要重连的对象
  · 每个节点的 Left 都要清空，包括链表最后一个节点
  · 最后一个节点的 Right 必须置空，否则会把输入树里的旧指针带进链表

以示例 1（root = [1,2,5,3,4,null,6]）为例：
  前序序列：1、2、3、4、5、6；
  依次令 1.Right=2、2.Right=3 …… 6.Right=nil，且所有 Left 置空；
  结果 1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：前序访问一次，重连扫一次切片
  空间 O(n)：保存全部节点地址，正是进阶要消除的部分

解法二：前序 + 显式栈，边遍历边接链【非最优】

为什么慢：省掉了切片，但栈的深度仍可能到 O(n)：左斜链上路径上的节点全在
栈里。空间是 O(h)，最坏仍是 O(n)，依旧不是 O(1)。

换个角度：前序的"下一个访问节点"可以由栈给出。只要保证左孩子先出栈，弹栈
顺序就与递归前序一致，于是可以一边弹一边把上一个访问节点接到当前节点上。

关键点：
  · 先压右孩子、再压左孩子 —— 栈是后进先出，反过来弹栈顺序就变成"根→右→左"
  · 孩子必须在清空 Left 之前入栈，否则子树丢失
  · 接链发生在出栈时：prev.Right = cur，并把 cur.Left 置空
  · 空树直接返回，不要往栈里压空指针

以示例 1（root = [1,2,5,3,4,null,6]）为例：
  弹 1（栈：5、2），前驱为空；把 5 的右 6、2 的右 4 与左 3 入栈；
  弹 2（栈：5、6、4、3），1.Right=2；再弹 3、4、5、6，
  依次得到 1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：每个节点入栈、出栈各一次
  空间 O(h)：栈只保存尚未访问的右子树，最坏（左斜链）O(n)

解法三：寻找前驱节点（原地展开，O(1) 额外空间）【最优解】

为什么慢：前两种方法的辅助容器都源于同一个需求 —— 前序访问完左子树后要
记得回来走右子树。容器就是在替我们记这件事。

换个角度：把"回来"的路径直接写进树里。对当前节点 cur：
  · 若 cur 没有左孩子，它已经在正确位置，走向右孩子即可；
  · 否则，展开后 cur 的下一个节点是左子树的第一个节点，而原右子树应该接在
    "左子树最右节点（前序前驱）"的后面。于是：
      pred = 左子树最右节点；pred.Right = cur.Right；
      cur.Right = cur.Left；cur.Left = nil。
  左子树被整体挪到右边后，cur 的右孩子就是下一个待处理节点，继续往右走。

关键点：
  · 顺序不能反：必须先找 pred、先把 cur.Right 挂到 pred 上，最后才覆盖
    cur.Right；反了右子树就丢了
  · 找 pred 只在左子树的右链上走，条件就是 `p.Right != nil`
  · 每处理一个节点都要把 Left 置空，题目要求最终左指针全为 null
  · 处理完当前节点后只往右走，不要回头再进左子树 —— 它已经被挪到右边，
    整棵树变成一条右链，沿右指针走一遍即可
  · 每个节点只被当作 cur 处理一次，找 pred 走过的边之后不会再走，均摊 O(n)

以示例 1（root = [1,2,5,3,4,null,6]）为例：
  cur=1：pred 是左子树最右的 4，4.Right=5，1.Right=2，1.Left=nil；
  cur=2：pred 是 3，3.Right=4，2.Right=3，2.Left=nil；
  cur=3：无左孩子，走到 4；4 无左孩子，走到 5；5 无左孩子，走到 6；
  右链 1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：每个节点作为 cur 一次；找 pred 的指针移动总量也是 O(n)
  空间 O(1)：只用 cur、pred 两个指针，没有栈、没有切片
*/

// flatten 是解法一，同时也是平台入口，签名不能改。
func flatten(root *TreeNode) {
	nodes := []*TreeNode{}
	var preorder func(node *TreeNode)
	// 先收完再改：遍历途中改 Right 会丢掉尚未访问的子树
	preorder = func(node *TreeNode) {
		if node == nil {
			return
		}
		nodes = append(nodes, node)
		preorder(node.Left)
		preorder(node.Right)
	}
	preorder(root)

	for i, node := range nodes {
		// Left 一律清空；Right 指向序列里的下一个，末尾为 nil
		node.Left = nil
		if i+1 < len(nodes) {
			node.Right = nodes[i+1]
		} else {
			node.Right = nil
		}
	}
}

// flattenWithStack 是解法二：前序遍历 + 显式栈。
func flattenWithStack(root *TreeNode) {
	if root == nil {
		return
	}

	stk := []*TreeNode{root}
	var prev *TreeNode

	for len(stk) > 0 {
		cur := stk[len(stk)-1]
		stk = stk[:len(stk)-1]

		// 先压右再压左，左孩子才会先出栈，弹栈顺序才是前序
		if cur.Right != nil {
			stk = append(stk, cur.Right)
		}
		if cur.Left != nil {
			stk = append(stk, cur.Left)
		}

		// 孩子已入栈，此时清空 Left 不会丢子树
		if prev != nil {
			prev.Right = cur
		}
		cur.Left = nil
		prev = cur
	}
}

// flattenMorris 是解法三：寻找前驱节点，O(1) 额外空间。
func flattenMorris(root *TreeNode) {
	cur := root
	for cur != nil {
		if cur.Left != nil {
			// 前驱 = 左子树最右节点，即前序中 cur 的前一个节点
			pred := cur.Left
			for pred.Right != nil {
				pred = pred.Right
			}

			// 先用 pred 收留原右子树，再覆盖 cur.Right，顺序反了会丢树
			pred.Right = cur.Right
			cur.Right = cur.Left
			cur.Left = nil
		}
		// 左子树已挪到右边，沿右链继续即可
		cur = cur.Right
	}
}

// ===== 测试辅助：按层序数组建树、按右链收集 =====
const nullNode = -(1 << 62) // 节点值为 int，不会等于这个哨兵
const maxNodes = 2000       // 题目上限，用来兜住成环的错误

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

// 沿 Right 收集：同时检查 Left 全为空、右链没有环
func collectRightChain(root *TreeNode) []int {
	values := []int{}
	cur := root
	for cur != nil {
		if cur.Left != nil {
			panic(fmt.Sprintf("left 未清空: val=%d", cur.Val))
		}
		values = append(values, cur.Val)
		cur = cur.Right
		// 指针接错可能成环，限制步数让测试报错而不是死循环
		if len(values) > maxNodes {
			panic("右链成环")
		}
	}
	return values
}

func equalInts(a, b []int) bool {
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
	flatteners := []struct {
		name string
		fn   func(*TreeNode)
	}{
		{"preorder", flatten},
		{"stack", flattenWithStack},
		{"morris", flattenMorris},
	}

	for _, f := range flatteners {
		root := buildTree(levelOrder)
		f.fn(root)
		got := collectRightChain(root)
		if !equalInts(got, want) {
			panic(fmt.Sprintf("%s(%v) = %v, want %v",
				f.name, levelOrder, got, want))
		}
	}
}

func main() {
	// 示例 1：左子树内部还有左右孩子，右子树非空
	check([]int{1, 2, 5, 3, 4, nullNode, 6}, []int{1, 2, 3, 4, 5, 6})

	// 示例 2：空树
	check([]int{}, []int{})

	// 示例 3：单节点
	check([]int{0}, []int{0})

	// 边界：只有左孩子的斜链
	check([]int{1, 2, nullNode, 3}, []int{1, 2, 3})

	// 边界：只有右孩子的斜链
	check([]int{1, nullNode, 2, nullNode, 3}, []int{1, 2, 3})

	// 反直觉：根有左子树，同时右子树非空 —— 右子树必须挂到前驱后面
	check([]int{2, 1, 4, nullNode, nullNode, 3}, []int{2, 1, 4, 3})

	// 反直觉：根右子树非空，左子树有右链，前驱不是左孩子本身
	check([]int{1, 2, 5, nullNode, 4, nullNode, 6}, []int{1, 2, 4, 5, 6})

	// 满二叉树：前序是 4,2,1,3,5,6
	check([]int{4, 2, 5, 1, 3, nullNode, 6}, []int{4, 2, 1, 3, 5, 6})

	// 左斜链 200 层
	{
		levelOrder := []int{1, 2}
		want := []int{}
		for i := 1; i <= 200; i++ {
			want = append(want, i)
		}
		// 每层只有一个左孩子：成对写入 (null, 下一个值)
		for i := 3; i <= 200; i++ {
			levelOrder = append(levelOrder, nullNode, i)
		}
		check(levelOrder, want)
	}

	// 右斜链 200 层
	{
		levelOrder := []int{1}
		want := []int{1}
		for i := 2; i <= 200; i++ {
			levelOrder = append(levelOrder, nullNode, i)
			want = append(want, i)
		}
		check(levelOrder, want)
	}

	fmt.Println("all tests passed")
}
