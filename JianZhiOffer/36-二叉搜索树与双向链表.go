/*
============================================================================
剑指 Offer 36. 二叉搜索树与双向链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/
分类：二叉搜索树 · 中序遍历 / 双向循环链表

将一个 二叉搜索树 就地转化为一个 已排序的双向循环链表 。

对于双向循环列表，你可以将左右孩子指针作为双向循环链表的前驱和后继指针，
第一个节点的前驱是最后一个节点，最后一个节点的后继是第一个节点。

特别地，我们希望可以 就地 完成转换操作。当转化完成以后，树中节点的左指针
需要指向前驱，树中节点的右指针需要指向后继。还需要返回链表中最小元素的
指针。

示例 1：
输入：root = [4,2,5,1,3]
输出：[1,2,3,4,5]
解释：下图显示了转化后的二叉搜索树，实线表示后继关系，虚线表示前驱关系。

示例 2：
输入：root = [2,1,3]
输出：[1,2,3]

示例 3：
输入：root = []
输出：[]
解释：输入是空树，所以输出也是空链表。

示例 4：
输入：root = [1]
输出：[1]

提示：
- -1000 <= Node.val <= 1000
- Node.left.val < Node.val < Node.right.val
- Node.val 的所有值都是独一无二的
- 0 <= Number of Nodes <= 2000

注意：本题与主站 426 题相同：
https://leetcode.cn/problems/convert-binary-search-tree-to-sorted-doubly-linked-list/
*/
package main

import (
	"fmt"
	"sort"
)

// 力扣预置的节点定义
type Node struct {
	Val   int
	Left  *Node
	Right *Node
}

/*
解法一：中序遍历 + 全局前驱指针【最优解】

为什么慢：BST 的中序序列就是升序，转换的难点不在"排序"，而在"改指针时不能
丢节点"。最直白的做法是先中序收集所有节点到数组，再按数组重连：正确但要多
一个 O(n) 的数组，而题目明确要求就地转换。

换个角度：中序遍历每次访问节点时，恰好知道它的前一个节点是谁。只要在访问
当前节点 node 时保存着 prev，就能当场把 `prev.Right = node`、
`node.Left = prev` 接好，指针改完再递归右子树，右子树根本没被碰过。

关键点：
  · 先递归左子树、再改指针、最后递归右子树 —— 顺序固定，提前改指针会丢掉
    还没访问的右子树或左子树
  · `prev` 为空时才赋值 head：中序第一个访问的节点就是最小值，即链表头
  · 递归结束后 prev 停在中序最后一个节点（最大值），此时才补首尾互指：
    `head.Left = prev`、`prev.Right = head`，少这一步得到的是单向链表
  · 空树必须提前返回 nil，否则 head 一直为空，补互指时会解引用空指针
  · 返回的是 head（最小元素），不是 root —— 根在中序中间，位置会变
  · Go 没有类成员，用闭包捕获 head、prev 两个状态

以示例 1（root = [4,2,5,1,3]）为例：
  中序访问顺序 1、2、3、4、5；
  访问 1 时 prev 为空 → head = 1；访问 2 时 1.Right = 2、2.Left = 1；
  依此接到 5，结束时 prev = 5，补 5.Right = 1、1.Left = 5 ✔
  （若漏掉补互指，5 的 Right 仍是原来的空指针，链表不成环。）

复杂度分析：
  时间 O(n)：每个节点访问一次，接线是常数次赋值
  空间 O(h)：递归栈深度等于树高，最坏（斜树）O(n)

解法二：显式栈中序【非最优】

为什么慢：思路与解法一完全相同，只是把系统递归栈换成手写栈。时间、空间
同为 O(n)/O(h)，与解法一同为最优；标为非最优只是避免重复计最优。它的实际
好处是递归深度不再受语言默认栈限制，2000 层的斜树也不会栈溢出。

换个角度：中序的"下一个访问节点"就是栈顶。一路向左压栈到底，弹栈即访问，
访问完把游标移到右孩子再重复压栈动作。

关键点：
  · 压栈用 while 走完整条左链，不能只压一层 —— 弹出时才保证是当前最小
  · 弹栈后立刻接 `prev.Right` 和 `cur.Left`，此时 cur 的右孩子还没被动过，
    所以 `cur = cur.Right` 拿到的仍是原右子树
  · head 同样只在 prev 为空时赋值，即中序第一个节点
  · 循环条件是 `cur != nil || len(stk) > 0`：游标空了还要继续弹栈，
    写成 `cur != nil` 会漏掉整条右链
  · 结束后与解法一一样补首尾互指

以示例 2（root = [2,1,3]）为例：
  压 2、1 → 弹 1（head = 1）→ cur 为 1 的右孩子（空）；
  弹 2，接 1.Right = 2、2.Left = 1 → cur 为 3；
  压 3、弹 3，接 2.Right = 3、3.Left = 2；
  补 3.Right = 1、1.Left = 3 ✔

复杂度分析：
  时间 O(n)：每个节点入栈、出栈、接线各一次
  空间 O(h)：栈最多保存一条根到叶的路径，最坏（斜树）O(n)
*/

// 解法一：中序遍历 + 全局前驱指针
func treeToDoublyList(root *Node) *Node {
	if root == nil {
		return nil
	}

	var head, prev *Node

	var inorder func(node *Node)
	inorder = func(node *Node) {
		if node == nil {
			return
		}

		inorder(node.Left)

		// 中序第一个节点没有前驱，它的位置就是链表头
		if prev != nil {
			prev.Right = node
			node.Left = prev
		} else {
			head = node
		}
		prev = node

		// 右子树尚未被改动，递归进去时指针还是原样
		inorder(node.Right)
	}

	inorder(root)

	// 首尾互指：prev 停在中序最后一个节点（最大值）
	head.Left = prev
	prev.Right = head
	return head
}

// 解法二：显式栈中序
func treeToDoublyListStack(root *Node) *Node {
	if root == nil {
		return nil
	}

	var head, prev *Node
	stk := []*Node{}
	cur := root

	for cur != nil || len(stk) > 0 {
		// 一路向左到底，栈顶就是当前最小的未访问节点
		for cur != nil {
			stk = append(stk, cur)
			cur = cur.Left
		}

		cur = stk[len(stk)-1]
		stk = stk[:len(stk)-1]

		if prev != nil {
			prev.Right = cur
			cur.Left = prev
		} else {
			head = cur
		}
		prev = cur

		// cur 的右孩子没被改过，这里拿到的是原右子树
		cur = cur.Right
	}

	head.Left = prev
	prev.Right = head
	return head
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树、校验双向循环链表 =====
const nullNode = -1 << 40 // 测试数据只用小整数，不会撞上这个哨兵

// BST 的中序就是升序：期望值 = 输入里的非空值排序
func sortedValues(levelOrder []int) []int {
	values := []int{}
	for _, v := range levelOrder {
		if v != nullNode {
			values = append(values, v)
		}
	}
	sort.Ints(values)
	return values
}

func buildTree(levelOrder []int) *Node {
	if len(levelOrder) == 0 || levelOrder[0] == nullNode {
		return nil
	}
	root := &Node{Val: levelOrder[0]}
	nodes := []*Node{root}
	idx := 1
	for i := 0; i < len(nodes) && idx < len(levelOrder); i++ {
		if levelOrder[idx] != nullNode {
			nodes[i].Left = &Node{Val: levelOrder[idx]}
			nodes = append(nodes, nodes[i].Left)
		}
		idx++
		if idx >= len(levelOrder) {
			break
		}
		if levelOrder[idx] != nullNode {
			nodes[i].Right = &Node{Val: levelOrder[idx]}
			nodes = append(nodes, nodes[i].Right)
		}
		idx++
	}
	return root
}

// 从 head 顺 Right 走 n 步，校验升序、双向性与首尾互指
func verify(head *Node, want []int) {
	if len(want) == 0 {
		if head != nil {
			panic("空树必须返回 nil")
		}
		return
	}

	n := len(want)
	nodes := make([]*Node, 0, n)
	cur := head
	for i := 0; i < n; i++ {
		// 走到空说明链表长度不足，接错指针时会在这里暴露
		if cur == nil {
			panic(fmt.Sprintf("链表在第 %d 步断开，want %v", i, want))
		}
		nodes = append(nodes, cur)
		cur = cur.Right
	}
	// 走满 n 步必须回到 head，多出节点或断链都不满足循环链表定义
	if cur != head {
		panic("走满 n 步没有回到 head")
	}

	for i := 0; i < n; i++ {
		if nodes[i].Val != want[i] {
			panic(fmt.Sprintf("第 %d 个节点是 %d, want %d", i, nodes[i].Val, want[i]))
		}
		// 头的前驱是尾、尾的后继是头，正好由下标取模表达
		if nodes[i].Left != nodes[(i+n-1)%n] {
			panic(fmt.Sprintf("节点 %d 的前驱指针不对", nodes[i].Val))
		}
		if nodes[i].Right != nodes[(i+1)%n] {
			panic(fmt.Sprintf("节点 %d 的后继指针不对", nodes[i].Val))
		}
	}
}

func check(levelOrder []int) {
	want := sortedValues(levelOrder)
	verify(treeToDoublyList(buildTree(levelOrder)), want)
	verify(treeToDoublyListStack(buildTree(levelOrder)), want)
}

func main() {
	// 示例 1：左右子树都有节点，最小值在最左端
	check([]int{4, 2, 5, 1, 3})

	// 示例 2：只有三个节点
	check([]int{2, 1, 3})

	// 示例 3：空树，必须返回 nil 而不是崩溃
	check([]int{})

	// 示例 4：单节点，首尾都是它自己
	check([]int{1})

	// 反直觉：最小值不在根的左孩子上，而在左孩子的左孩子上
	check([]int{10, 5, 15, 2, 7, 12, 20, 1, 3, 6, 8, 11, 13})

	// 边界：只有左孩子的斜链，头是最深的那个、尾是根
	check([]int{3, 2, nullNode, 1})

	// 边界：只有右孩子的斜链，头是根、尾是最深的那个
	check([]int{1, nullNode, 2, nullNode, 3})

	// 边界：2000 层左斜链，压一压递归与栈的深度
	leftChain := []int{2000}
	for i := 1999; i >= 1; i-- {
		leftChain = append(leftChain, i, nullNode)
	}
	check(leftChain)

	// 边界：2000 层右斜链
	rightChain := []int{1}
	for i := 2; i <= 2000; i++ {
		rightChain = append(rightChain, nullNode, i)
	}
	check(rightChain)

	fmt.Println("all tests passed")
}
