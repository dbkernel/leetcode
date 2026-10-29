/*
============================================================================
106. 从中序与后序遍历序列构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
分类：二叉树 · 分治构造（中序 + 后序）

给定两个整数数组 inorder 和 postorder ，其中 inorder 是二叉树的中序遍历，
postorder 是同一棵树的后序遍历，请你构造并返回这颗 二叉树 。

示例 1：
输入：inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
输出：[3,9,20,null,null,15,7]

示例 2：
输入：inorder = [-1], postorder = [-1]
输出：[-1]

提示：
- 1 <= inorder.length <= 3000
- postorder.length == inorder.length
- -3000 <= inorder[i], postorder[i] <= 3000
- inorder 和 postorder 都由 不同 的值组成
- postorder 中每一个值都在 inorder 中
- inorder 保证是树的中序遍历
- postorder 保证是树的后序遍历
*/
package main

import (
	"fmt"
	"math"
	"slices"
)

// 力扣预置的节点定义
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/*
解法一：递归分治 + 中序下标哈希表【最优解】

为什么慢：后序的最后一个元素就是根，这一步 O(1)；开销全在"根在中序的哪个
位置"。若每层递归都线性扫描中序区间找根，每层花 O(区间长)，左斜链与右斜链
都有 O(n) 层，累计退化成 O(n^2)。

换个角度：根的位置由"值 → 中序下标"决定，而题目保证值互不重复。先建一张表，
每次定位 O(1)，整棵树 O(n) 次访问就够。

关键点：
  · 后序的根在区间**右端**（`pr`），先序题在左端，这是两题唯一的切换点
  · 右子树节点数 `rightLen = 中序区间右端 - 中序根下标`；后序里右半段紧贴
    根之前，即 [pl + leftLen, pr - 1]
  · 左半段是 [pl, pl + leftLen - 1]，leftLen 与中序左段长度必须相等
  · 递归出口是区间为空（`pl > pr`），长度 1 时仍要建出叶子
  · 值域含负数，只能用 map 存下标
  · Go 里用闭包递归，map 与两个切片在闭包外捕获，避免层层传参

以示例 1（inorder = [9,3,15,20,7]，postorder = [9,15,7,20,3]）为例：
  根 3（后序末尾），中序下标 1 → leftLen = 1
  左：中序 [9]、后序 [9] → 叶子 9
  右：中序 [15,20,7]、后序 [15,7,20]，根 20（后序末尾），leftLen = 1
      左边中序 [15]、后序 [15] → 15；右边中序 [7]、后序 [7] → 7
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，每个节点恰被创建一次
  空间 O(n)：map 存 n 项；递归栈 O(h)，斜链时也是 O(n)

解法二：迭代栈，从后序末尾倒着走【最优解】

为什么慢：解法一要递归 O(h) 层，再额外占一张 O(n) 的哈希表；树深时递归栈
同样到 O(n)，时间上也不比解法二省。

换个角度：后序是「左 → 右 → 根」，倒着读就是「根 → 右 → 左」，等价于把
先序题的树做左右镜像后照抄。于是栈维护的是当前未完成的**右链**，中序指针
从末尾往前扫：
  · 栈顶与 `inorder[inIdx]` 不等：新节点是栈顶的右孩子，压栈继续往右下走
  · 相等：右链到底，连续弹栈跳过已在中序出现过的祖先，新节点挂在最后弹出的
    节点的**左边**

关键点：
  · inIdx 从 `n - 1` 开始、只减不增，方向与栈的「右链」一致
  · 相等时必须连续弹栈，弹到「不相等或栈空」再挂左孩子
  · 挂左孩子用的是最后弹出的节点，不是弹完后的栈顶
  · 后序末元素先建根、先入栈，循环从 i = n - 2 往下走
  · 与解法一并列最优：时间同为 O(n)，但不再需要哈希表，空间从 O(n) 降到
    O(h)；它是解法一的镜像写法，可用来对照「根在末尾」这一侧的边界

以示例 1（inorder = [9,3,15,20,7]，postorder = [9,15,7,20,3]）为例：
  i=3，值 20：栈顶 3 ≠ in[4]=7 → 3.Right = 20，压栈
  i=2，值 7：栈顶 20 ≠ in[4]=7 → 20.Right = 7，压栈
  i=1，值 15：栈顶 7 = in[4] → 弹 7、弹 20，inIdx 走到 2；15 挂 20 左边
  i=0，值 9：栈顶 15 = in[2] → 弹 15、弹 3，inIdx 走到 0；9 挂 3 左边
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：每个节点进出栈各一次
  空间 O(h)：栈里只保存当前右链，不再需要哈希表
*/

// 解法一：递归分治 + 中序下标哈希表（最优解之一，空间 O(n)）
func buildTree(inorder []int, postorder []int) *TreeNode {
	pos := make(map[int]int, len(inorder))
	for i, v := range inorder {
		pos[v] = i
	}

	var build func(pl, pr, il, ir int) *TreeNode
	build = func(pl, pr, il, ir int) *TreeNode {
		// 空区间才是出口：长度为 1 时仍需建出那个叶子
		if pl > pr {
			return nil
		}
		rootVal := postorder[pr]
		k := pos[rootVal]
		leftLen := k - il

		node := &TreeNode{Val: rootVal}
		// 左半段在前，右半段紧贴根，两段长度由 leftLen 唯一确定
		node.Left = build(pl, pl+leftLen-1, il, k-1)
		node.Right = build(pl+leftLen, pr-1, k+1, ir)
		return node
	}

	n := len(postorder)
	return build(0, n-1, 0, n-1)
}

// 解法二：迭代栈，从后序末尾倒着走
func buildTreeIterative(inorder []int, postorder []int) *TreeNode {
	n := len(postorder)
	root := &TreeNode{Val: postorder[n-1]}
	stk := []*TreeNode{root}
	// 中序指针从末尾往前：与「根 → 右 → 左」的倒序后序同向
	inIdx := n - 1

	for i := n - 2; i >= 0; i-- {
		node := stk[len(stk)-1]
		if node.Val != inorder[inIdx] {
			// 栈顶尚未在中序出现，倒序后序的这一项只能是它的右孩子
			node.Right = &TreeNode{Val: postorder[i]}
			stk = append(stk, node.Right)
		} else {
			// 弹到不相等为止：最后弹出的节点正是新节点的父节点
			for len(stk) > 0 && stk[len(stk)-1].Val == inorder[inIdx] {
				node = stk[len(stk)-1]
				stk = stk[:len(stk)-1]
				inIdx--
			}
			node.Left = &TreeNode{Val: postorder[i]}
			stk = append(stk, node.Left)
		}
	}
	return root
}

// ===== 测试辅助：层序序列化（nullNode 表示空） =====
const nullNode = math.MinInt32 // 节点值在 [-3000, 3000]，不会撞上哨兵

func toLevelOrder(root *TreeNode) []int {
	out := []int{}
	if root == nil {
		return out
	}

	// 用队列逐个展开：空节点也入队一次，保证位置不塌陷
	q := []*TreeNode{root}
	for i := 0; i < len(q); i++ {
		if q[i] == nil {
			out = append(out, nullNode)
			continue
		}
		out = append(out, q[i].Val)
		q = append(q, q[i].Left, q[i].Right)
	}

	// 末尾的空孩子不改变结构，裁掉才能和题面输出逐项比较
	for len(out) > 0 && out[len(out)-1] == nullNode {
		out = out[:len(out)-1]
	}
	return out
}

func check(inorder, postorder, want []int) {
	got := toLevelOrder(buildTree(inorder, postorder))
	if !slices.Equal(got, want) {
		panic(fmt.Sprintf("optimal got %v, want %v", got, want))
	}

	got = toLevelOrder(buildTreeIterative(inorder, postorder))
	if !slices.Equal(got, want) {
		panic(fmt.Sprintf("iterative got %v, want %v", got, want))
	}
}

func main() {
	// 示例 1
	check([]int{9, 3, 15, 20, 7}, []int{9, 15, 7, 20, 3},
		[]int{3, 9, 20, nullNode, nullNode, 15, 7})

	// 示例 2：单节点，两个解法都不该进循环体
	check([]int{-1}, []int{-1}, []int{-1})

	// 边界：左斜链，后序升序
	check([]int{1, 2, 3}, []int{1, 2, 3}, []int{3, 2, nullNode, 1})

	// 边界：右斜链，后序降序
	check([]int{1, 2, 3}, []int{3, 2, 1}, []int{1, nullNode, 2, nullNode, 3})

	// 关键点用例：根在末尾且右子树非空，右半段偏移写错时会挂到左边
	check([]int{4, 2, 3, 1}, []int{4, 3, 2, 1}, []int{1, 2, nullNode, 4, 3})

	// 满二叉树
	check([]int{4, 2, 5, 1, 6, 3, 7}, []int{4, 5, 2, 6, 7, 3, 1},
		[]int{1, 2, 3, 4, 5, 6, 7})

	// 边界：含 0 与负数，验证按值定位
	check([]int{-3000, 0, 3000}, []int{-3000, 3000, 0}, []int{0, -3000, 3000})

	// 边界：1000 层左斜链，中序升序、后序也升序
	in := []int{}
	post := []int{}
	for v := 1; v <= 1000; v++ {
		in = append(in, v)
		post = append(post, v)
	}
	want := []int{1000}
	for v := 999; v >= 1; v-- {
		want = append(want, v)
		if v > 1 {
			want = append(want, nullNode)
		}
	}
	check(in, post, want)

	// 边界：1000 层右斜链，中序升序、后序降序
	in = []int{}
	post = []int{}
	for v := 1; v <= 1000; v++ {
		in = append(in, v)
		post = append(post, 1001-v)
	}
	want = []int{1}
	for v := 2; v <= 1000; v++ {
		want = append(want, nullNode, v)
	}
	check(in, post, want)

	fmt.Println("all tests passed")
}
