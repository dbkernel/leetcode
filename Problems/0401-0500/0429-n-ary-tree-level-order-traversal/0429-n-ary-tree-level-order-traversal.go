/*
============================================================================
429. N 叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-level-order-traversal/
分类：N 叉树 · 层序遍历（BFS / 队列）

给定一个 N 叉树，返回其节点值的层序遍历。（即从左到右，逐层遍历）。

树的序列化输入是用层序遍历，每组子节点都由 null 值分隔（参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[[1],[3,2,4],[5,6]]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]

提示：
- 树的高度不会超过 1000
- 树的节点总数在 [0, 10^4] 之间
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
解法：BFS + 按层定长出队【最优解】

为什么慢：层序遍历要求「同层从左到右、不同层分开」。如果只用一个普通队列
逐个弹出，虽然节点顺序是对的，但不知道哪几个属于同一层；若给每个节点挂
层号再按层号分组，就要多存 O(n) 个层号。

换个角度：BFS 队列在某一时刻恰好是「当前层剩余节点 + 下一层已入队节点」。
每轮开始先记下 cnt = len(q) - head，再连续弹出 cnt 个节点，这批就是完整的
一层；而这 cnt 个节点入队的孩子，恰好全部是下一层，并且按父节点的顺序排好。

关键点：
  · cnt 必须在内层循环之前取，循环中队列还会增长，不能写成 `len(q)`
  · 每轮新建 level 再 append 到 ans，避免各层共用同一个切片
  · N 叉树没有「先左后右」，只要按 Children 的下标顺序入队，层内自然有序
  · 空树要单独返回：不要入队再靠循环兜底，否则会多出一个空层
  · 用 head 下标出队，避免 `q = q[1:]` 反复移动底层数组；树高 1000 不影响 BFS

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  第 1 轮 cnt=1 → 出 1，入孩子 3、2、4 → [[1]]
  第 2 轮 cnt=3 → 出 3（入 5、6）、2、4 → [[1],[3,2,4]]
  第 3 轮 cnt=2 → 出 5、6 → [[1],[3,2,4],[5,6]]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案外，队列最多同时存两层节点
*/
func levelOrder(root *Node) [][]int {
	ans := [][]int{}
	if root == nil {
		return ans
	}

	q := []*Node{root}
	for head := 0; head < len(q); {
		// 本层节点个数必须在出队前锁定
		cnt := len(q) - head
		level := make([]int, 0, cnt)
		for i := 0; i < cnt; i++ {
			cur := q[head]
			head++
			level = append(level, cur.Val)
			q = append(q, cur.Children...)
		}
		ans = append(ans, level)
	}
	return ans
}

// ===== 测试辅助：按层序数组建 N 叉树（nullNode 表示空） =====
const nullNode = 10001

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

func check(input []int, want [][]int) {
	if got := levelOrder(buildNaryTree(input)); !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("levelOrder(%v) = %v, want %v", input, got, want))
	}
}

func main() {
	// 示例 1
	check([]int{1, nullNode, 3, 2, 4, nullNode, 5, 6},
		[][]int{{1}, {3, 2, 4}, {5, 6}})

	// 示例 2
	check([]int{1, nullNode, 2, 3, 4, 5, nullNode, nullNode, 6, 7, nullNode,
		8, nullNode, 9, 10, nullNode, nullNode, 11, nullNode, 12,
		nullNode, 13, nullNode, nullNode, 14},
		[][]int{{1}, {2, 3, 4, 5}, {6, 7, 8, 9, 10}, {11, 12, 13}, {14}})

	// 边界：空树
	check([]int{}, [][]int{})

	// 边界：单节点
	check([]int{1}, [][]int{{1}})

	// 边界：根节点没有孩子
	check([]int{1, nullNode}, [][]int{{1}})

	// 边界：星形树，第二层顺序就是孩子数组顺序
	check([]int{1, nullNode, 2, 3, 4, 5, 6},
		[][]int{{1}, {2, 3, 4, 5, 6}})

	// 边界：某层节点有的有孩子、有的没有（null 占位）
	check([]int{1, nullNode, 2, 3, nullNode, nullNode, 4},
		[][]int{{1}, {2, 3}, {4}})

	// 边界：深链（每层一个节点），深度 200
	levelOrderInput := []int{}
	want := [][]int{}
	for i := 1; i <= 200; i++ {
		levelOrderInput = append(levelOrderInput, i, nullNode)
		want = append(want, []int{i})
	}
	check(levelOrderInput, want)

	fmt.Println("all tests passed")
}
