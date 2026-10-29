/*
============================================================================
589. N 叉树的前序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-preorder-traversal/
分类：N 叉树 · 遍历（递归 / 栈）

给定一个 n 叉树的根节点 root ，返回 其节点值的 前序遍历 。

n 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔
（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[1,3,5,6,2,4]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[1,2,3,6,7,11,14,4,8,12,5,9,13,10]

提示：
- 节点总数在范围 [0, 10^4]内
- 0 <= Node.val <= 10^4
- n 叉树的高度小于或等于 1000

进阶：递归法很简单，你可以使用迭代法完成此题吗?
*/
package main

import (
	"fmt"
	"reflect"
)

// 力扣预置的节点定义（多叉树，注意孩子是一个切片）
type Node struct {
	Val      int
	Children []*Node
}

/*
解法一：递归【最优解】

官方方法一。N 叉树的前序遍历与二叉树完全一致，只是「左右两个孩子」换成了
「children 数组」：
  · 先访问当前节点；
  · 再按从左到右的顺序依次递归每个孩子。

为什么它是最优解：
  · 时间 O(m)，每个节点只访问一次，不可能更快；
  · 辅助空间只有递归栈 O(h)，不需要额外哈希表或「待处理兄弟」缓存，
    因此在空间上优于下面两种迭代写法。

关键点：
  · 访问语句必须在 for 循环之前，否则就变成后序了
  · Children 为空时 for 循环自然跳过，不需要单独判空
  · Go 里没有尾递归优化，递归深度 = 树高 h，本题 h ≤ 1000 不会爆栈

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  访问 1 → 依次进孩子 3、2、4：
  访问 3 → 孩子 5、6：访问 5，访问 6；回到 2：访问 2；回到 4：访问 4
  得到 [1,3,5,6,2,4]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：迭代（显式栈 + 哈希表记录孩子下标）【非最优】

官方方法二。直接模拟递归：递归时「返回到父节点后该处理第几个孩子」这件事
由调用栈自动记住，手写栈就必须自己记，于是用 map 存每个节点已处理到的下标。
  · 一路沿着第一个孩子向下，边下边访问、边把节点压栈，并在表中记 0；
  · 走到空后看栈顶 p：取它下一个未访问孩子的下标 index；
      - index 有效 → 更新表中下标，转到该孩子继续向下；
      - index 无效 → 说明 p 的孩子都处理完了，弹出 p、删掉表项，回到 p 的父亲。

关键点：
  · 「先访问再往下」决定了这是前序；如果改成出栈时才访问就变成后序
  · 表项要在节点弹出时删除，否则同一节点再次入栈会读到旧下标
  · 叶子的表项不存在，用 -1 兜底：加一后为 0，恰好判定为「已无孩子」
  · 额外的 map 让常数和空间都变差，所以不是最优

复杂度分析：
  时间 O(m)
  空间 O(h)：栈与 map 都只保存当前路径上的节点

解法三：迭代（逆序压栈，最简写法）【非最优】

官方方法三。前序要求「根 → 孩子从左到右」，而栈是后进先出：
  · 先弹出并访问当前节点；
  · 再把孩子「从右到左」压栈，这样左孩子最后入栈、最先出栈。

关键点：
  · 压栈必须逆序，正序压栈会让最右孩子先被访问，顺序就反了
  · 不需要 map，也不需要「当前节点」指针，循环体只有三步
  · 空间最坏是 O(m)：星形树（根有 m-1 个孩子）时栈里同时存下所有兄弟节点
  · 与解法二同为 O(m) 时间，但写法短得多，是迭代法里的首选

复杂度分析：
  时间 O(m)
  空间 O(m)：最坏为星形树，栈同时保存 O(m) 个兄弟节点

-------- 三种写法对比 --------
  递归：时间 O(m)、空间 O(h)，最省空间，标记为【最优解】
  解法二：时间 O(m)、空间 O(h) + map，常数最大，仅作为「模拟递归」的示范
  解法三：时间 O(m)、空间 O(m)，代码最短，迭代时优先选它
  本题 N 叉树没有 Morris 那样的 O(1) 空间遍历写法。
*/

// 解法一：递归（最优解）
func preorderHelper(node *Node, ans *[]int) {
	if node == nil {
		return
	}
	*ans = append(*ans, node.Val)
	for _, child := range node.Children {
		preorderHelper(child, ans)
	}
}

func preorder(root *Node) []int {
	ans := []int{}
	preorderHelper(root, &ans)
	return ans
}

// 解法二：迭代 + map 记录孩子下标
func preorderIterativeWithIndex(root *Node) []int {
	ans := []int{}
	if root == nil {
		return ans
	}

	nextIndex := map[*Node]int{}
	stk := []*Node{}
	node := root
	for len(stk) > 0 || node != nil {
		// 沿着第一个孩子一路向下，同时访问沿途节点
		for node != nil {
			ans = append(ans, node.Val)
			stk = append(stk, node)
			if len(node.Children) > 0 {
				nextIndex[node] = 0
				node = node.Children[0]
			} else {
				node = nil
			}
		}
		node = stk[len(stk)-1]
		// 叶子没有表项，-1 + 1 = 0，恰好越界
		index := -1
		if v, ok := nextIndex[node]; ok {
			index = v
		}
		index++
		if index < len(node.Children) {
			nextIndex[node] = index
			node = node.Children[index]
		} else {
			stk = stk[:len(stk)-1]
			delete(nextIndex, node)
			node = nil
		}
	}
	return ans
}

// 解法三：迭代 + 逆序压栈
func preorderIterativeReverse(root *Node) []int {
	ans := []int{}
	if root == nil {
		return ans
	}

	stk := []*Node{root}
	for len(stk) > 0 {
		node := stk[len(stk)-1]
		stk = stk[:len(stk)-1]
		ans = append(ans, node.Val)
		// 逆序压栈：左孩子最后入栈，从而最先出栈
		for i := len(node.Children) - 1; i >= 0; i-- {
			stk = append(stk, node.Children[i])
		}
	}
	return ans
}

// ===== 测试辅助：按层序数组建 N 叉树（nullNode 表示空） =====
const nullNode = 10001 // 节点值范围 [0, 10^4]

// 层序序列规则：根之后先来一个 null，然后是该节点的全部孩子；
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

func check(levelOrder []int, want []int) {
	if got := preorder(buildNaryTree(levelOrder)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("递归(%v) = %v, want %v", levelOrder, got, want))
	}
	if got := preorderIterativeWithIndex(buildNaryTree(levelOrder)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("迭代+下标(%v) = %v, want %v", levelOrder, got, want))
	}
	if got := preorderIterativeReverse(buildNaryTree(levelOrder)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("迭代+逆序(%v) = %v, want %v", levelOrder, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{1, nullNode, 3, 2, 4, nullNode, 5, 6}, []int{1, 3, 5, 6, 2, 4})

	// 示例 2
	check([]int{1, nullNode, 2, 3, 4, 5, nullNode, nullNode, 6, 7, nullNode,
		8, nullNode, 9, 10, nullNode, nullNode, 11, nullNode, 12,
		nullNode, 13, nullNode, nullNode, 14},
		[]int{1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10})

	// 边界：空树
	check([]int{}, []int{})

	// 边界：单节点
	check([]int{1}, []int{1})

	// 边界：根节点没有孩子
	check([]int{1, nullNode}, []int{1})

	// 边界：叶子节点顺序 —— 根的 3 个孩子都没有孩子
	check([]int{1, nullNode, 2, 3, 4}, []int{1, 2, 3, 4})

	// 边界：星形树，验证逆序压栈有没有把兄弟顺序写反
	check([]int{1, nullNode, 2, 3, 4, 5, 6}, []int{1, 2, 3, 4, 5, 6})

	// 边界：孩子为空要用 null 占位（2 没有孩子，3 有孩子 4）
	check([]int{1, nullNode, 2, 3, nullNode, nullNode, 4}, []int{1, 2, 3, 4})

	// 边界：深链（每个节点只有一个孩子），深度 200
	levelOrder := []int{}
	want := []int{}
	for i := 1; i <= 200; i++ {
		levelOrder = append(levelOrder, i, nullNode)
		want = append(want, i)
	}
	check(levelOrder, want)

	fmt.Println("all tests passed")
}
