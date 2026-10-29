/*
============================================================================
图的存储（Graph Storage）
分类：手写数据结构 · 图
操作复杂度：
  add_edge   O(1) 矩阵；O(degree) 邻接表（查重 + 有序插入）
  has_edge   O(1) 矩阵；O(degree) 邻接表
  neighbors  O(V) 矩阵；O(degree) 邻接表
  degree     O(V) 矩阵；O(1) 邻接表（返回表长）
  edge_count O(1) 加边时维护计数
============================================================================

用途：
  面试常要求不用库手写图的存法，并说明稠密图与稀疏图该选哪种；
  后续的遍历、最短路、拓扑排序都建立在这一层之上。

核心思想：
  邻接矩阵用 V×V 的表记录任意两点是否相邻，用空间换 O(1) 判断；
  邻接表只为每条真实存在的边开一个表项，遍历邻居只需 O(degree)，
  稀疏图上不会浪费 O(V^2) 的空位。

关键点（不变量）：
  · 无向图度数和 = 2E，有向图出度之和 = 入度之和 = E；
  · 重边按集合语义忽略，第二次 add_edge 不改变结构；
  · 无向自环对一个顶点贡献 2 度，矩阵要把对角元算两次；
  · 矩阵与邻接表对同一输入必须得到完全相同的边集。

手推一遍：
  以 n = 3 的无向图，边 (0,1)、(1,2) 为例：
  1. 矩阵置 m[0][1] = m[1][0] = true，m[1][2] = m[2][1] = true；
  2. 邻接表 adj[0] = [1]，adj[1] = [0,2]，adj[2] = [1]；
  3. 度数和 1 + 2 + 1 = 4 = 2E，两边集都是 {(0,1),(1,2)}。

复杂度分析：
  时间：建图 O(V^2) 矩阵 / O(V+E) 邻接表，查询见上方分操作表。
  空间：O(V^2) 矩阵，O(V+E) 邻接表。
  适用：V 小或边密集用矩阵，V 大且边稀疏用邻接表。
*/
package main

import "fmt"

// GraphMatrix 邻接矩阵：m[u][v] 为 true 表示有边 u->v。
type GraphMatrix struct {
	n        int
	directed bool
	m        [][]bool
	edges    int
}

func newGraphMatrix(n int, directed bool) *GraphMatrix {
	m := make([][]bool, n)
	for i := range m {
		m[i] = make([]bool, n)
	}
	return &GraphMatrix{n: n, directed: directed, m: m}
}

func (g *GraphMatrix) HasEdge(u, v int) bool {
	return g.m[u][v]
}

func (g *GraphMatrix) AddEdge(u, v int) {
	if g.m[u][v] {
		return // 重边按集合语义忽略
	}
	g.m[u][v] = true
	g.edges++
	// 无向图对称置位；自环 u == v 时同一格只置一次
	if !g.directed && u != v {
		g.m[v][u] = true
	}
}

func (g *GraphMatrix) Neighbors(u int) []int {
	res := []int{}
	for v := 0; v < g.n; v++ {
		if g.m[u][v] {
			res = append(res, v)
		}
	}
	return res
}

// Degree 无向图的度：自环只占一个格，但图论上算 2 度。
func (g *GraphMatrix) Degree(u int) int {
	d := 0
	for v := 0; v < g.n; v++ {
		if g.m[u][v] {
			if u == v {
				d += 2
			} else {
				d++
			}
		}
	}
	return d
}

func (g *GraphMatrix) OutDegree(u int) int {
	d := 0
	for v := 0; v < g.n; v++ {
		if g.m[u][v] {
			d++
		}
	}
	return d
}

func (g *GraphMatrix) InDegree(u int) int {
	d := 0
	for v := 0; v < g.n; v++ {
		if g.m[v][u] {
			d++
		}
	}
	return d
}

func (g *GraphMatrix) EdgeCount() int {
	return g.edges
}

// EdgeList 规范边集：无向图只取上三角，自环只留一次；有向图全扫。
func (g *GraphMatrix) EdgeList() [][2]int {
	res := [][2]int{}
	for u := 0; u < g.n; u++ {
		start := 0
		if !g.directed {
			start = u
		}
		for v := start; v < g.n; v++ {
			if g.m[u][v] {
				res = append(res, [2]int{u, v})
			}
		}
	}
	return res
}

// GraphList 邻接表：adj[u] 存出边终点，保持升序，遍历可复现。
type GraphList struct {
	n        int
	directed bool
	adj      [][]int
	indeg    []int // 有向图入度，加边时维护，查询 O(1)
	edges    int
}

func newGraphList(n int, directed bool) *GraphList {
	adj := make([][]int, n)
	for i := range adj {
		adj[i] = []int{}
	}
	return &GraphList{
		n:        n,
		directed: directed,
		adj:      adj,
		indeg:    make([]int, n),
	}
}

func (g *GraphList) HasEdge(u, v int) bool {
	for _, x := range g.adj[u] {
		if x == v {
			return true
		}
	}
	return false
}

// insertSorted 从尾部找插入点再整体右移，O(degree)，不用库排序。
func (g *GraphList) insertSorted(u, v int) {
	pos := len(g.adj[u])
	for pos > 0 && g.adj[u][pos-1] > v {
		pos--
	}
	g.adj[u] = append(g.adj[u], 0)
	copy(g.adj[u][pos+1:], g.adj[u][pos:])
	g.adj[u][pos] = v
}

func (g *GraphList) AddEdge(u, v int) {
	if g.HasEdge(u, v) {
		return // 重边按集合语义忽略
	}
	g.insertSorted(u, v)
	if g.directed {
		g.indeg[v]++
	} else if u != v {
		g.insertSorted(v, u)
	} else {
		// 无向自环存两份，使 Degree(u) 自然等于 2
		g.insertSorted(u, u)
	}
	g.edges++
}

func (g *GraphList) Neighbors(u int) []int {
	res := make([]int, len(g.adj[u]))
	copy(res, g.adj[u])
	return res
}

func (g *GraphList) Degree(u int) int {
	return len(g.adj[u])
}

func (g *GraphList) OutDegree(u int) int {
	return len(g.adj[u])
}

func (g *GraphList) InDegree(u int) int {
	return g.indeg[u]
}

func (g *GraphList) EdgeCount() int {
	return g.edges
}

func (g *GraphList) EdgeList() [][2]int {
	res := [][2]int{}
	for u := 0; u < g.n; u++ {
		for _, v := range g.adj[u] {
			if g.directed {
				res = append(res, [2]int{u, v})
			} else if u < v {
				res = append(res, [2]int{u, v})
			} else if u == v {
				// 自环存了两份，只保留第一条，否则边集多算一次
				if len(res) == 0 || res[len(res)-1] != [2]int{u, v} {
					res = append(res, [2]int{u, v})
				}
			}
		}
	}
	return res
}

// checkSameGraph 同一张图分别用两种存储建一遍，逐项对齐。
func checkSameGraph(gm *GraphMatrix, gl *GraphList) {
	if gm.n != gl.n || gm.directed != gl.directed {
		panic("图的规模或方向不一致")
	}
	if gm.EdgeCount() != gl.EdgeCount() {
		panic("边数不一致")
	}
	em := gm.EdgeList()
	el := gl.EdgeList()
	if len(em) != len(el) {
		panic("边集长度不一致")
	}
	for i := range em {
		if em[i] != el[i] {
			panic("边集内容不一致")
		}
	}
	for u := 0; u < gm.n; u++ {
		for v := 0; v < gm.n; v++ {
			if gm.HasEdge(u, v) != gl.HasEdge(u, v) {
				panic("has_edge 不一致")
			}
		}
	}
	if gm.directed {
		outSum, inSum := 0, 0
		for u := 0; u < gm.n; u++ {
			if gm.OutDegree(u) != gl.OutDegree(u) {
				panic("出度不一致")
			}
			if gm.InDegree(u) != gl.InDegree(u) {
				panic("入度不一致")
			}
			outSum += gm.OutDegree(u)
			inSum += gm.InDegree(u)
		}
		// 有向图不变量：出度之和 = 入度之和 = E
		if outSum != gm.EdgeCount() || inSum != gm.EdgeCount() {
			panic("有向图出入度和不等于边数")
		}
	} else {
		degSum := 0
		for u := 0; u < gm.n; u++ {
			if gm.Degree(u) != gl.Degree(u) {
				panic("度不一致")
			}
			degSum += gm.Degree(u)
		}
		// 无向图不变量：度数和 = 2E；自环算 2 度正是为此
		if degSum != 2*gm.EdgeCount() {
			panic("无向图度数和不是 2E")
		}
	}
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

func main() {
	// 1. 空图：任何查询都不能越界，边数必须是 0
	gm := newGraphMatrix(4, false)
	gl := newGraphList(4, false)
	checkSameGraph(gm, gl)
	if gm.EdgeCount() != 0 {
		panic("空图边数不为 0")
	}
	for u := 0; u < 4; u++ {
		if gm.Degree(u) != 0 || len(gm.Neighbors(u)) != 0 {
			panic("空图度数或邻居不为空")
		}
		if len(gl.Neighbors(u)) != 0 {
			panic("空图邻接表不为空")
		}
	}
	if gm.HasEdge(0, 1) {
		panic("空图不应有边")
	}

	// 2. 无向链 0-1-2，并重复加 (0,1) 验证重边不改变结构
	gm = newGraphMatrix(3, false)
	gl = newGraphList(3, false)
	es := [][2]int{{0, 1}, {1, 2}}
	for _, e := range es {
		gm.AddEdge(e[0], e[1])
		gl.AddEdge(e[0], e[1])
	}
	gm.AddEdge(0, 1)
	gl.AddEdge(0, 1)
	checkSameGraph(gm, gl)
	if gm.EdgeCount() != 2 {
		panic("重边被重复计数")
	}
	if gm.Degree(1) != 2 {
		panic("链中间点度数应为 2")
	}
	if !equalInts(gm.Neighbors(1), []int{0, 2}) {
		panic("矩阵邻居未升序")
	}
	if !equalInts(gl.Neighbors(1), []int{0, 2}) {
		panic("邻接表邻居未升序")
	}

	// 3. 无向自环：矩阵只有一格，邻接表存两份，度都必须是 2
	gm = newGraphMatrix(3, false)
	gl = newGraphList(3, false)
	gm.AddEdge(0, 0)
	gl.AddEdge(0, 0)
	gm.AddEdge(0, 0) // 再次加同一条自环仍是重边
	gl.AddEdge(0, 0)
	checkSameGraph(gm, gl)
	if gm.EdgeCount() != 1 {
		panic("自环边数应为 1")
	}
	if gm.Degree(0) != 2 || gl.Degree(0) != 2 {
		panic("自环应贡献 2 度")
	}
	if !gm.HasEdge(0, 0) {
		panic("自环 has_edge 应为 true")
	}
	if !equalInts(gm.Neighbors(0), []int{0}) {
		panic("矩阵自环应只出现一次")
	}
	if !equalInts(gl.Neighbors(0), []int{0, 0}) {
		panic("邻接表自环应存两份")
	}
	sum := 0
	for u := 0; u < 3; u++ {
		sum += gm.Degree(u)
	}
	if sum != 2*gm.EdgeCount() {
		panic("自环破坏度数和 = 2E")
	}

	// 4. 有向图含自环：入度、出度分别核对，并保持两个和不变
	gm = newGraphMatrix(5, true)
	gl = newGraphList(5, true)
	es = [][2]int{{0, 1}, {0, 2}, {1, 3}, {2, 3}, {3, 4}, {3, 3}}
	for _, e := range es {
		gm.AddEdge(e[0], e[1])
		gl.AddEdge(e[0], e[1])
	}
	checkSameGraph(gm, gl)
	if gm.EdgeCount() != 6 {
		panic("有向图边数不对")
	}
	if gm.OutDegree(3) != 2 {
		panic("3 的出度应为 2")
	}
	if gm.InDegree(3) != 3 { // 来自 1、2 以及 3 的自环
		panic("3 的入度应为 3")
	}
	if gm.InDegree(4) != 1 {
		panic("4 的入度应为 1")
	}
	if !gm.HasEdge(3, 3) {
		panic("有向自环应存在")
	}
	if !equalInts(gm.Neighbors(3), []int{3, 4}) {
		panic("有向邻居未升序")
	}
	if !equalInts(gl.Neighbors(3), []int{3, 4}) {
		panic("有向邻接表邻居未升序")
	}

	// 5. 稀疏图：10 点 7 边，两条存储的边集完全一致
	gm = newGraphMatrix(10, false)
	gl = newGraphList(10, false)
	for i := 0; i+3 < 10; i++ {
		gm.AddEdge(i, i+3)
		gl.AddEdge(i, i+3)
	}
	checkSameGraph(gm, gl)
	if gm.EdgeCount() != 7 {
		panic("稀疏图边数应为 7")
	}
	if gm.Degree(0) != 1 {
		panic("0 的度应为 1")
	}

	// 6. 稠密图：7 点完全图，E = 21，每个点度数为 6
	gm = newGraphMatrix(7, false)
	gl = newGraphList(7, false)
	for u := 0; u < 7; u++ {
		for v := u + 1; v < 7; v++ {
			gm.AddEdge(u, v)
			gl.AddEdge(u, v)
		}
	}
	checkSameGraph(gm, gl)
	if gm.EdgeCount() != 21 {
		panic("完全图边数应为 21")
	}
	for u := 0; u < 7; u++ {
		if gm.Degree(u) != 6 {
			panic("完全图每个点度数应为 6")
		}
	}

	fmt.Println("all tests passed")
}
