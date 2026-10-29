/*
============================================================================
面试题 17.12. BiNode（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binode-lcci/
分类：二叉搜索树 · 中序遍历 / Morris

二叉树数据结构TreeNode可用来表示单向链表（其中left置空，right为下一个链表
节点）。实现一个方法，把二叉搜索树转换为单向链表，要求依然符合二叉搜索树的
性质，转换操作应是原址的，也就是在原始的二叉搜索树上直接修改。

返回转换后的单向链表的头节点。

注意：本题相对原题稍作改动

示例：
输入： [4,2,5,1,3,null,6,0]
输出： [0,null,1,null,2,null,3,null,4,null,5,null,6]

提示：
- 节点数量不会超过 100000。
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
解法一：中序遍历，递归返回 (头, 尾) 再拼接【非最优】

为什么慢：中序遍历本身 O(n)，慢的地方在"怎么原地拼"。如果只保存一个"上一
个访问的节点"，递归回溯时无法在 O(1) 里把左链表接到根上；如果先收集节点再
重连，又要 O(n) 数组。真正的瓶颈是信息不够：拼接需要同时知道一段链表的头
和尾。

换个角度：让递归函数直接返回展开后的 (头, 尾) 一对。根的中序序列就是
"左链表 + 根 + 右链表"，拿到左尾、右头之后，拼接只是两次指针赋值，返回时
把头尾合并即可。

关键点：
  · 返回值必须是 (头, 尾) 一对：只返回头就得 O(n) 找尾才能拼接
  · 尾的取法：右子树非空取右尾，否则取当前节点；不能一律取当前节点
  · 必须先递归完左右子树再改当前节点的指针，否则会丢失子树
  · 拼接后当前节点的 left 必须置空（题目要求单向链表）
  · 空树返回 (nil, nil)，作为递归出口
  · Go 用多返回值表达一对指针，不必额外定义结构体

以示例（root = [4,2,5,1,3,null,6,0]）为例：
  节点 0：无子树，返回 (0, 0)；
  节点 1：左尾 0 接到 1，返回 (0, 1)；
  节点 2：左链表 (0,1) 的尾 1 接到 2，右链表 (3,3) 接在 2 后，
          返回 (0, 3)；
  依此向上，最终头是最左的 0，右链为 0→1→…→6 ✔

复杂度分析：
  时间 O(n)：每个节点进入递归一次，拼接是常数次赋值
  空间 O(h)：递归栈，h 为树高；最坏（斜树）O(n)，这正是解法二要省的

解法二：Morris 中序遍历，原地调整指针【最优解】

为什么慢：解法一的递归栈是 O(h)。中序需要栈，只是为了"从左子树回来后知道
该访问谁"；这个"谁"其实就是当前节点，可以用左子树最右节点的空 right 指针
直接指回来（线索），不需要真正的栈。

换个角度：中序里，节点 cur 的前驱是"左子树最右节点"。让前驱的空 right 指向
cur，向左走；等左子树遍历完，顺着这条线索回到 cur，此时才访问 cur，并把
"上一个访问节点"的 right 接成 cur，同时清空 cur 的 left。遍历和重连在同一个
循环里完成。

关键点：
  · 有左孩子时不算访问：先找前驱 pred（左子树最右），pred.right = cur 建
    线索，清空 cur.left，再向左走
  · 没有左孩子时才是访问：prev.right = cur；prev 为空时 cur 就是链表头
  · 线索只在回到 cur 后被 prev.right = cur 覆盖，值相同，不必单独拆链
  · 每个被访问节点的 left 都必须为 nil；有左孩子的节点在向左走之前就已清空
  · 最后访问的节点是中序最大节点，它的 right 本来就是 nil，无需额外处理

以示例（root = [4,2,5,1,3,null,6,0]）为例：
  一路向左：4 的前驱 3 指向 4，2 的前驱 1 指向 2，1 的前驱 0 指向 1；
  访问 0（头）、顺线索回 1 并访问、回 2 并访问、访问 3、回 4；
  再依次访问 5、6，右链 0→1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：每条线索（边）最多被走两次，建线索一次、沿线索返回一次
  空间 O(1)：只用 cur、prev、pred 三个指针，不借助栈
*/

// 解法一：中序遍历，递归返回 (头, 尾) 再拼接
func convertBiNode(root *TreeNode) *TreeNode {
	head, _ := convertDfs(root)
	return head
}

// 返回展开后的 (头, 尾)：只返回头就没法 O(1) 接上左链表
func convertDfs(node *TreeNode) (*TreeNode, *TreeNode) {
	if node == nil {
		return nil, nil
	}

	// 先递归完左右子树，再动指针，否则会丢失子树
	leftHead, leftTail := convertDfs(node.Left)
	rightHead, rightTail := convertDfs(node.Right)

	node.Left = nil
	node.Right = rightHead
	if leftTail != nil {
		leftTail.Right = node
	}

	head := node
	if leftHead != nil {
		head = leftHead
	}
	// 尾取右链的尾；没有右子树时当前节点就是尾
	tail := node
	if rightTail != nil {
		tail = rightTail
	}
	return head, tail
}

// 解法二：Morris 中序遍历，原地调整指针
func convertBiNodeMorris(root *TreeNode) *TreeNode {
	cur := root
	var head, prev *TreeNode

	for cur != nil {
		if cur.Left != nil {
			// 前驱 = 左子树最右节点；它此刻还没有右孩子
			pred := cur.Left
			for pred.Right != nil {
				pred = pred.Right
			}
			pred.Right = cur // 线索：左子树走完能回到 cur

			left := cur.Left
			cur.Left = nil
			cur = left
		} else {
			// 访问 cur：把上一个访问节点的 right 接过来
			if prev != nil {
				prev.Right = cur
			} else {
				head = cur // 第一个访问的就是最左节点，即链表头
			}
			prev = cur
			cur = cur.Right // 可能是线索，也可能是原右子树
		}
	}
	return head
}

// ===== 测试辅助：按层序数组（nullNode 表示空）建树、按右链收集 =====
const nullNode = -1 << 40 // 测试数据只用小整数，不会撞上这个哨兵

const maxNodes = 100000 // 题目上限，用来兜住成环的错误

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

// 沿 right 收集：同时断言 left 全为空、右链没有环
func collectRightChain(head *TreeNode) []int {
	var values []int
	cur := head
	for cur != nil {
		if cur.Left != nil {
			panic(fmt.Sprintf("node %d 残留左孩子 %d", cur.Val, cur.Left.Val))
		}
		values = append(values, cur.Val)
		cur = cur.Right
		if len(values) > maxNodes {
			panic("right 链成环")
		}
	}
	return values
}

// BST 的中序就是升序，所以期望值 = 输入里的非空值排序
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

func sameInts(a, b []int) bool {
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

func check(levelOrder []int) {
	want := sortedValues(levelOrder)

	got := collectRightChain(convertBiNode(buildTree(levelOrder)))
	if !sameInts(got, want) {
		panic(fmt.Sprintf("dfs(%v) = %v, want %v", levelOrder, got, want))
	}

	got = collectRightChain(convertBiNodeMorris(buildTree(levelOrder)))
	if !sameInts(got, want) {
		panic(fmt.Sprintf("morris(%v) = %v, want %v", levelOrder, got, want))
	}
}

func main() {
	// 示例：左右都有子树，且最左节点在第三层
	check([]int{4, 2, 5, 1, 3, nullNode, 6, 0})

	// 边界：空树
	check([]int{})

	// 边界：单节点
	check([]int{1})

	// 边界：只有左孩子的斜链，头是最深的那个
	check([]int{3, 2, nullNode, 1})

	// 边界：只有右孩子的斜链，头就是根
	check([]int{1, nullNode, 2, nullNode, 3})

	// 反直觉：右子树里有比根小的节点 —— 结果仍必须是全升序
	check([]int{1, nullNode, 3, 2})

	// 反直觉：根的两个孩子都不是最值，最左/最右藏在深处
	check([]int{10, 5, 15, 2, 7, 12, 20, 1, 3, 6, 8, 11, 13})

	// 满 BST
	check([]int{4, 2, 6, 1, 3, 5, 7})

	// 边界：2000 层左斜链，验证递归与线索都不改变顺序
	chain := []int{2000}
	for i := 1999; i >= 1; i-- {
		chain = append(chain, i, nullNode)
	}
	check(chain)

	fmt.Println("all tests passed")
}
