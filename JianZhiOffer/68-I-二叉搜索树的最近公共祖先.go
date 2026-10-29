/*
============================================================================
LCR 193 / 剑指 Offer 68 - I. 二叉搜索树的最近公共祖先（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-zui-jin-gong-gong-zu-xian-lcof/
分类：二叉搜索树 · 一次遍历 / 路径比较

给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个结点 p、q，最近公共
祖先表示为一个结点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点
也可以是它自己的祖先）。”

例如，给定如下二叉搜索树:  root = [6,2,8,0,4,7,9,null,null,3,5]

示例 1：
输入：root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
输出：6
解释：节点 2 和节点 8 的最近公共祖先是 6。

示例 2：
输入：root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
输出：2
解释：节点 2 和节点 4 的最近公共祖先是 2, 因为根据定义最近公共祖先节点可以
为节点本身。

说明：
- 所有节点的值都是唯一的。
- p、q 为不同节点且均存在于给定的二叉搜索树中。

注意：本题与主站 235 题相同：
https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/
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
解法一：两次遍历求出根到 p、q 的路径，取最后一个公共前缀【非最优】

为什么慢：时间上两趟下降各 O(h)，并不算慢；代价在空间和多余信息：要各存
一条长度最多 h 的路径（斜树上 O(n)），可题目只关心"两条路径从哪里分开"，
中间那些公共前缀节点存下来之后只用得到最后一个。

换个角度：从根走到某个节点的路径，在 BST 上唯一。p、q 的最近公共祖先就是
两条路径的最后一个相同节点 —— 分叉发生的位置。既然路径本身是单调下降的，
就可以边比较边丢掉前缀，只保留"当前还相同"的那个节点。

关键点：
  · 每走一步先把当前节点压入路径，再与 target 比较 —— 先比较再压入会漏掉
    target 自己，p 是 q 的祖先时会退回上一层
  · 路径必须从根开始，否则公共前缀不完整
  · 两条路径从头比，遇到第一个不同的节点就停，答案取最后一个相同的节点
  · 题目保证节点存在，走空只可能出现在非法输入上

以示例 2（root = [6,2,8,0,4,7,9,null,null,3,5]，p = 2，q = 4）为例：
  到 2 的路径：[6, 2]（2 就是目标，压入后立刻停）
  到 4 的路径：[6, 2, 4]
  公共前缀 6、2，最后一个相同节点是 2 ✔
  （若压入时不包含 target，路径变成 [6] 和 [6,2]，答案会错成 6。）

复杂度分析：
  时间 O(h)：两条路径各下降一次，h 为树高（最坏 n）
  空间 O(h)：两个切片保存路径，最坏（斜树）O(n)

解法二：一次遍历，利用 BST 的比较性质直接找分叉点【最优解】

为什么慢：解法一存了两条完整路径，而 BST 上根本不需要知道路径长什么样：
当前节点的值就能告诉我们 p、q 在哪一侧。

换个角度：从根往下走，p、q 的走向只有三种可能：
  · 两个都小于当前值 → 二者都在左子树，LCA 在左边，向左走；
  · 两个都大于当前值 → 二者都在右子树，LCA 在右边，向右走；
  · 一大一小，或其中之一等于当前值 → 二者从这里分到两侧（或当前节点
    就是其中一个），当前节点就是 LCA，不能再往下走。

关键点：
  · 两个判断都要写成"严格小于 / 严格大于当前值"：只要有一个等于当前节点，
    当前节点就是答案，把等号放进下沉条件会多走一层，返回更深的节点
  · 必须拿 p、q 分别与当前节点比较，不能先比较 p 与 q 的大小再定方向 ——
    "p < q" 说明不了它们相对当前节点在左边还是右边
  · p、q 的先后顺序不可假设，p 完全可以比 q 大，两个条件都要同时成立
  · 下沉到 nil 说明输入不合法；题目保证 p、q 存在，所以正常循环内必返回

以示例 2（root = [6,2,8,0,4,7,9,null,null,3,5]，p = 2，q = 4）为例：
  root = 6：2 < 6 且 4 < 6，向左走到 2；
  root = 2：p 等于当前值，不满足"两个都严格小于/大于"，返回 2 ✔

复杂度分析：
  时间 O(h)：每步只下降一层，h 为树高；平衡时 O(log n)，斜树 O(n)
  空间 O(1)：只用了一个游标指针，没有栈也没有切片
*/

// lowestCommonAncestor 是平台入口，采用官方签名的节点指针版本。
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	pathP := collectPath(root, p)
	pathQ := collectPath(root, q)

	var ans *TreeNode
	n := len(pathP)
	if len(pathQ) < n {
		n = len(pathQ)
	}
	// 公共前缀一路相同，最后一个相同节点就是分叉前的位置
	for i := 0; i < n && pathP[i] == pathQ[i]; i++ {
		ans = pathP[i]
	}
	return ans
}

// collectPath 的路径包含 target 本身：
// 否则"某节点是另一个的祖先"会被漏掉
func collectPath(node, target *TreeNode) []*TreeNode {
	path := []*TreeNode{}
	for node != nil {
		path = append(path, node)
		if node == target {
			return path
		}
		if target.Val < node.Val {
			node = node.Left
		} else {
			node = node.Right
		}
	}
	return path
}

// lowestCommonAncestorOnePass 是解法二：一次遍历，O(1) 额外空间。
func lowestCommonAncestorOnePass(root, p, q *TreeNode) *TreeNode {
	cur := root
	for cur != nil {
		// 两者都严格小于当前值，说明都在左子树，分叉点不可能在这
		if p.Val < cur.Val && q.Val < cur.Val {
			cur = cur.Left
		} else if p.Val > cur.Val && q.Val > cur.Val {
			// 两者都严格大于当前值，说明都在右子树
			cur = cur.Right
		} else {
			// 分居两侧，或其中之一正是当前节点
			return cur
		}
	}
	// 题目保证 p、q 都在树上，只有非法输入才会走到这里
	return nil
}

// ===== 测试辅助：按层序数组建树、按值找节点 =====
const nullNode = -(1 << 62) // 节点值为 int，不会等于这个哨兵

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

func findNode(root *TreeNode, val int) *TreeNode {
	if root == nil {
		return nil
	}
	if root.Val == val {
		return root
	}
	if found := findNode(root.Left, val); found != nil {
		return found
	}
	return findNode(root.Right, val)
}

func check(levelOrder []int, pVal, qVal, want int) {
	for _, isOnePass := range []bool{false, true} {
		root := buildTree(levelOrder)
		p := findNode(root, pVal)
		q := findNode(root, qVal)
		wantNode := findNode(root, want)
		if p == nil || q == nil || wantNode == nil {
			panic(fmt.Sprintf("input %v 里找不到节点", levelOrder))
		}

		var got *TreeNode
		if isOnePass {
			got = lowestCommonAncestorOnePass(root, p, q)
		} else {
			got = lowestCommonAncestor(root, p, q)
		}
		// 比指针而不是值：指针相等才证明返回的是那个节点
		if got != wantNode {
			panic(fmt.Sprintf("onePass=%v lca(%v, %d, %d) = %v, want %d",
				isOnePass, levelOrder, pVal, qVal, got, want))
		}
	}
}

func main() {
	sample := []int{6, 2, 8, 0, 4, 7, 9, nullNode, nullNode, 3, 5}

	// 示例 1：p、q 分居根的两侧
	check(sample, 2, 8, 6)

	// 示例 2：p 是 q 的祖先，答案必须是 p 自己（路径必须含 target）
	check(sample, 2, 4, 2)

	// 参数顺序无关：p 比 q 大
	check(sample, 8, 2, 6)
	check(sample, 5, 3, 4)

	// 边界：根就是其中一个节点
	check(sample, 6, 0, 6)

	// 反直觉：分叉点不在根，而在左边的 2 上
	check(sample, 0, 5, 2)

	// 反直觉：两个都在 2 的右子树里，分叉点是 4
	check(sample, 3, 5, 4)

	// 边界：右子树内部
	check(sample, 7, 9, 8)

	// 边界：最小规模，p、q 是根的两个孩子
	check([]int{2, 1, 3}, 1, 3, 2)

	// 边界：最小规模，p 是根
	check([]int{2, 1, 3}, 2, 1, 2)

	// 边界：200 层右斜链，值 1..200
	chain := []int{1}
	for i := 2; i <= 200; i++ {
		chain = append(chain, nullNode, i)
	}
	check(chain, 1, 200, 1)
	check(chain, 150, 200, 150)
	check(chain, 120, 130, 120)

	fmt.Println("all tests passed")
}
