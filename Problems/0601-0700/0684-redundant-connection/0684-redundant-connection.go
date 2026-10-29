/*
============================================================================
684. 冗余连接（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/redundant-connection/
分类：并查集 · 逐边判环

树可以看成是一个连通且 无环 的无向图。

给定一个图，该图从一棵 n 个节点 (节点值 1～n) 的树中添加一条边后获得。
添加的边的两个不同顶点编号在 1 到 n 中间，且这条附加的边不属于树中已有
的边。图的信息记录于长度为 n 的二维数组 edges ，edges[i] = [ai, bi]
表示图中在 ai 和 bi 之间存在一条边。

请找出一条可以删去的边，删除后可使得剩余部分是一个有着 n 个节点的树。
如果有多个答案，则返回数组 edges 中最后出现的那个。

示例 1：
输入: edges = [[1,2], [1,3], [2,3]]
输出: [2,3]

示例 2：
输入: edges = [[1,2], [2,3], [3,4], [1,4], [1,5]]
输出: [1,4]

提示：
- n == edges.length
- 3 <= n <= 1000
- edges[i].length == 2
- 1 <= ai < bi <= edges.length
- ai != bi
- edges 中无重复元素
- 给定的图是连通的
*/
package main

import (
	"fmt"
	"slices"
)

// 并查集：路径压缩 + 按大小合并。
// 路径压缩把查找路径上的点直接挂到根，缩短后续查找；
// 按大小合并把元素少的树挂到元素多的树下，防止树退化成链。
// 两者合起来让单次操作的均摊代价接近 O(α(n))。
type DSU struct {
	parent []int
	size   []int
}

func newDSU(n int) *DSU {
	d := &DSU{parent: make([]int, n), size: make([]int, n)}
	for i := 0; i < n; i++ {
		d.parent[i] = i
		d.size[i] = 1
	}
	return d
}

func (d *DSU) Find(x int) int {
	if d.parent[x] != x {
		d.parent[x] = d.Find(d.parent[x])
	}
	return d.parent[x]
}

// 返回 false 表示 x、y 本就在同一集合，这条边会成环
func (d *DSU) Unite(x, y int) bool {
	rx, ry := d.Find(x), d.Find(y)
	if rx == ry {
		return false
	}
	if d.size[rx] < d.size[ry] {
		rx, ry = ry, rx
	}
	d.parent[ry] = rx
	d.size[rx] += d.size[ry]
	return true
}

/*
解法一：并查集逐边合并，首次“两端已连通”的边即答案【最优解】

为什么慢：最直白的做法是枚举每条边，把它删掉后用一次 DFS 检查剩下的图
是否还连通 —— 每条边 O(n)，共 O(n^2)。而且“返回最后出现的那条”要求
按输入顺序比较，枚举删边并不能自然给出顺序。

换个角度：n 个点的树 + 一条额外边，边数也是 n，图中恰好只有一个环，
所有可以删的边正好是这个环上的边。按输入顺序逐条并入并查集，若某条边的
两端在合并前就已经连通，说明它落在环上，且是环上最后被读入的一条。

关键点：

	· 必须先查后并：`Find(a) == Find(b)` 成立时直接返回这条边，
	  此时环上更早的边都已并入，它就是“最后出现”的答案
	· 顺序不能乱：先 Unite 再判断会永远判成连通
	· 节点编号是 1..n，并查集开 n+1 个位置，0 号位空置
	· 不用真的删边：唯一加不进去的那条边就是答案

手推一遍：以示例 1（edges = [[1,2],[1,3],[2,3]]）为例：

	[1,2]：1、2 不同集合，合并 {1,2}
	[1,3]：3 与 {1,2} 不同集合，合并 {1,2,3}
	[2,3]：2、3 已同集合 → 返回 [2,3] ✔

复杂度分析：

	时间 O(n α(n))：n 条边各一次查找、至多一次合并，路径压缩 + 按大小
	  合并让单次操作均摊接近常数
	空间 O(n)：parent 与 size 各一个长度 n+1 的切片

解法二：按顺序建图，逐边 DFS 判可达【非最优】

为什么慢：判断“两个点是否已连通”这个动作，并查集只花近似 O(1)；换成
显式邻接表就要做一次 O(n) 的 DFS，n 条边累计 O(n^2)。

换个角度：把前 i-1 条边建成图，在加入第 i 条边之前先问 a 能否走到 b。
能走到说明这条边成环，等价于并查集的“查询”，只是查询代价更高。

关键点：

	· 查询必须发生在把当前边加入邻接表之前，否则一定可达
	· 邻接表按无向图双向加边
	· 只需在前 i-1 条边构成的子图里搜索，后面的边还没读入

手推一遍：仍以示例 1 为例：

	加入 [1,2] 前图为空，2 到不了 1，加边
	加入 [1,3] 前 3 到不了 1，加边
	加入 [2,3] 前已有 1-2、1-3，2 能走到 3 → 返回 [2,3] ✔

复杂度分析：

	时间 O(n^2)：每条边一次 DFS，子图规模 O(n)
	空间 O(n)：邻接表、visited 与迭代栈
*/
func findRedundantConnection(edges [][]int) []int {
	n := len(edges)
	dsu := newDSU(n + 1) // 节点编号 1..n
	for _, e := range edges {
		// 先查后并：连不上才说明这条边是多余的
		if !dsu.Unite(e[0], e[1]) {
			return []int{e[0], e[1]}
		}
	}
	return nil
}

// 解法二：按顺序建图，逐边 DFS 判可达
func findRedundantConnectionDFS(edges [][]int) []int {
	n := len(edges)
	graph := make([][]int, n+1)
	for _, e := range edges {
		// 此刻 graph 里只有前面的边，可达即说明当前边成环
		if reachable(graph, e[0], e[1]) {
			return []int{e[0], e[1]}
		}
		graph[e[0]] = append(graph[e[0]], e[1])
		graph[e[1]] = append(graph[e[1]], e[0])
	}
	return nil
}

func reachable(graph [][]int, from, to int) bool {
	seen := make([]bool, len(graph))
	stack := []int{from}
	seen[from] = true
	for len(stack) > 0 {
		x := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if x == to {
			return true
		}
		for _, y := range graph[x] {
			if seen[y] {
				continue
			}
			seen[y] = true
			stack = append(stack, y)
		}
	}
	return false
}

func check(edges [][]int, want []int) {
	if got := findRedundantConnection(edges); !slices.Equal(got, want) {
		panic(fmt.Sprintf("dsu(%v) = %v, want %v", edges, got, want))
	}
	if got := findRedundantConnectionDFS(edges); !slices.Equal(got, want) {
		panic(fmt.Sprintf("dfs(%v) = %v, want %v", edges, got, want))
	}
}

func main() {
	// 示例 1
	check([][]int{{1, 2}, {1, 3}, {2, 3}}, []int{2, 3})

	// 示例 2
	check([][]int{{1, 2}, {2, 3}, {3, 4}, {1, 4}, {1, 5}}, []int{1, 4})

	// 多余边在输入最前面：答案仍是环上最后出现的一条
	check([][]int{{1, 3}, {1, 2}, {2, 3}}, []int{2, 3})

	// 三元环，答案就是最后一条
	check([][]int{{1, 2}, {2, 3}, {3, 1}}, []int{3, 1})

	// 大环：1-2-3-4-1，答案取最后读入的 [1,4]
	check([][]int{{1, 2}, {2, 3}, {3, 4}, {1, 4}}, []int{1, 4})

	fmt.Println("all tests passed")
}
