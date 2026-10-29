/*
============================================================================
图的遍历（Graph Traversal）
分类：手写算法 · 图
复杂度：时间 O(V+E)；空间 O(V)（visited / 队列 / 栈）
============================================================================

用途：
  连通性判断、连通分量统计、无权图最短路与路径还原，
  面试常以「岛屿数量」「迷宫最短步数」等形式出现。

核心思想：
  DFS 沿一条路走到黑，用递归或显式栈保存回退点；
  BFS 按层扩展，用队列保证先访问距离更近的点。
  两者都靠 visited 保证每个点只被处理一次。

关键点（不变量）：
  · BFS 只在无权图或边权全相等的图上保证最短路；
  · DFS 迭代版必须入栈时就标记 visited，否则点会重复入栈；
  · 邻居按下标升序扩展，遍历序列才可复现；
  · 每个顶点至多入队/入栈一次，故总代价是 O(V+E)。

手推一遍：
  以链 0-1-2、从 0 出发的 BFS 为例：
  1. 队列 [0]，弹出 0，把未访问的 1 标记后入队；
  2. 弹出 1，把未访问的 2 标记后入队，再弹出 2；
  3. 距离 [0,1,2]，顺序 0,1,2；链上 BFS 与 DFS 恰好一致。

复杂度分析：
  时间 O(V+E)：无向图每条邻接表表项最多被看两次。
  空间 O(V)：visited、距离、parent 数组与队列/栈。
  注意：递归 DFS 的栈深最坏 O(V)，链状图会退化成深递归。
*/
package main

import "fmt"

// Graph 无向图邻接表：邻居保持升序，遍历结果才可复现。
type Graph struct {
	n   int
	adj [][]int
}

func newGraph(n int) *Graph {
	adj := make([][]int, n)
	for i := range adj {
		adj[i] = []int{}
	}
	return &Graph{n: n, adj: adj}
}

func (g *Graph) insertSorted(u, v int) {
	pos := len(g.adj[u])
	for pos > 0 && g.adj[u][pos-1] > v {
		pos--
	}
	g.adj[u] = append(g.adj[u], 0)
	copy(g.adj[u][pos+1:], g.adj[u][pos:])
	g.adj[u][pos] = v
}

func (g *Graph) AddEdge(u, v int) {
	g.insertSorted(u, v)
	if u != v {
		g.insertSorted(v, u) // 自环只存一份即可
	}
}

func (g *Graph) Neighbors(u int) []int {
	return g.adj[u]
}

// ringQueue 手写环形队列：head/tail 取模回绕，cnt 区分空与满。
type ringQueue struct {
	buf  []int
	head int
	tail int
	cnt  int
}

func newRingQueue(size int) *ringQueue {
	return &ringQueue{buf: make([]int, size)}
}

func (q *ringQueue) empty() bool {
	return q.cnt == 0
}

func (q *ringQueue) push(x int) {
	if q.cnt == len(q.buf) {
		panic("环形队列已满")
	}
	q.buf[q.tail] = x
	q.tail = (q.tail + 1) % len(q.buf)
	q.cnt++
}

func (q *ringQueue) pop() int {
	if q.cnt == 0 {
		panic("环形队列为空")
	}
	x := q.buf[q.head]
	q.head = (q.head + 1) % len(q.buf)
	q.cnt--
	return x
}

func dfsVisit(g *Graph, u int, vis []bool, out *[]int) {
	vis[u] = true
	*out = append(*out, u) // 先序位置：进入时记录
	for _, v := range g.Neighbors(u) {
		if !vis[v] {
			dfsVisit(g, v, vis, out)
		}
	}
}

func dfsRecursive(g *Graph, s int) []int {
	vis := make([]bool, g.n)
	out := []int{}
	dfsVisit(g, s, vis, &out)
	return out
}

func dfsIterative(g *Graph, s int) []int {
	vis := make([]bool, g.n)
	st := []int{} // 用切片当栈，不用 container/list
	out := []int{}
	vis[s] = true
	st = append(st, s) // 关键：入栈时就标记，不能等出栈
	for len(st) > 0 {
		u := st[len(st)-1]
		st = st[:len(st)-1]
		out = append(out, u)
		// 逆序压栈，弹出时才是邻居下标升序
		nb := g.Neighbors(u)
		for i := len(nb) - 1; i >= 0; i-- {
			v := nb[i]
			if !vis[v] {
				vis[v] = true
				st = append(st, v)
			}
		}
	}
	return out
}

func bfsOrder(g *Graph, s int) []int {
	vis := make([]bool, g.n)
	q := newRingQueue(g.n + 1)
	out := []int{}
	vis[s] = true
	q.push(s)
	for !q.empty() {
		u := q.pop()
		out = append(out, u)
		for _, v := range g.Neighbors(u) {
			if !vis[v] {
				vis[v] = true // 入队即标记，避免同一点重复入队
				q.push(v)
			}
		}
	}
	return out
}

// bfsDist 边权全为 1 时第一次到达即最短；带权图要换 Dijkstra。
func bfsDist(g *Graph, s int) []int {
	dist := make([]int, g.n)
	for i := range dist {
		dist[i] = -1
	}
	q := newRingQueue(g.n + 1)
	dist[s] = 0
	q.push(s)
	for !q.empty() {
		u := q.pop()
		for _, v := range g.Neighbors(u) {
			if dist[v] == -1 {
				dist[v] = dist[u] + 1
				q.push(v)
			}
		}
	}
	return dist
}

// componentSizes 按最小顶点从小到大的顺序，给出每个连通块的节点数。
func componentSizes(g *Graph) []int {
	vis := make([]bool, g.n)
	sizes := []int{}
	for s := 0; s < g.n; s++ {
		if vis[s] {
			continue
		}
		cnt := 0
		st := []int{s}
		vis[s] = true
		for len(st) > 0 {
			u := st[len(st)-1]
			st = st[:len(st)-1]
			cnt++
			for _, v := range g.Neighbors(u) {
				if !vis[v] {
					vis[v] = true
					st = append(st, v)
				}
			}
		}
		sizes = append(sizes, cnt)
	}
	return sizes
}

func componentCount(g *Graph) int {
	return len(componentSizes(g))
}

// shortestPath 用 BFS 记录 parent，再从终点回溯并反转得到路径。
func shortestPath(g *Graph, s, t int) []int {
	parent := make([]int, g.n)
	dist := make([]int, g.n)
	for i := 0; i < g.n; i++ {
		parent[i] = -1
		dist[i] = -1
	}
	q := newRingQueue(g.n + 1)
	dist[s] = 0
	q.push(s)
	for !q.empty() {
		u := q.pop()
		for _, v := range g.Neighbors(u) {
			if dist[v] == -1 {
				dist[v] = dist[u] + 1
				parent[v] = u
				q.push(v)
			}
		}
	}
	path := []int{}
	if dist[t] == -1 {
		return path // 不可达返回空
	}
	cur := t
	for cur != s {
		path = append(path, cur)
		cur = parent[cur]
	}
	path = append(path, s)
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}
	return path
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

func allUnique(a []int, n int) bool {
	seen := make([]bool, n)
	for _, x := range a {
		if seen[x] {
			return false
		}
		seen[x] = true
	}
	return true
}

func main() {
	// 1. 单点图：所有接口都要能处理 n = 1
	g := newGraph(1)
	if !equalInts(dfsRecursive(g, 0), []int{0}) {
		panic("单点 dfs 递归")
	}
	if !equalInts(dfsIterative(g, 0), []int{0}) {
		panic("单点 dfs 迭代")
	}
	if !equalInts(bfsOrder(g, 0), []int{0}) {
		panic("单点 bfs")
	}
	if !equalInts(bfsDist(g, 0), []int{0}) {
		panic("单点距离")
	}
	if componentCount(g) != 1 {
		panic("单点连通分量")
	}
	if !equalInts(componentSizes(g), []int{1}) {
		panic("单点分量大小")
	}
	if !equalInts(shortestPath(g, 0, 0), []int{0}) {
		panic("单点路径")
	}

	// 2. 链 0-1-2-3-4：三种遍历顺序一致，路径可正反还原
	g = newGraph(5)
	for i := 0; i+1 < 5; i++ {
		g.AddEdge(i, i+1)
	}
	expect := []int{0, 1, 2, 3, 4}
	if !equalInts(dfsRecursive(g, 0), expect) {
		panic("链 dfs 递归")
	}
	if !equalInts(dfsIterative(g, 0), expect) {
		panic("链 dfs 迭代")
	}
	if !equalInts(bfsOrder(g, 0), expect) {
		panic("链 bfs")
	}
	if !equalInts(bfsDist(g, 0), []int{0, 1, 2, 3, 4}) {
		panic("链距离")
	}
	if !equalInts(shortestPath(g, 0, 4), expect) {
		panic("链正向路径")
	}
	if !equalInts(shortestPath(g, 4, 0), []int{4, 3, 2, 1, 0}) {
		panic("链反向路径")
	}

	// 3. 星形：0 连 1..5，BFS 第一层顺序必须按下标升序
	g = newGraph(6)
	for i := 1; i < 6; i++ {
		g.AddEdge(0, i)
	}
	expect = []int{0, 1, 2, 3, 4, 5}
	if !equalInts(dfsRecursive(g, 0), expect) {
		panic("星形 dfs 递归")
	}
	if !equalInts(dfsIterative(g, 0), expect) {
		panic("星形 dfs 迭代")
	}
	if !equalInts(bfsOrder(g, 0), expect) {
		panic("星形 bfs")
	}
	if !equalInts(bfsDist(g, 0), []int{0, 1, 1, 1, 1, 1}) {
		panic("星形距离")
	}
	if !equalInts(shortestPath(g, 2, 5), []int{2, 0, 5}) {
		panic("星形路径")
	}

	// 4. 两个连通分量：计数、大小、不可达距离都盯住
	g = newGraph(5)
	g.AddEdge(0, 1)
	g.AddEdge(1, 2)
	g.AddEdge(3, 4)
	if componentCount(g) != 2 {
		panic("两个分量计数")
	}
	if !equalInts(componentSizes(g), []int{3, 2}) {
		panic("两个分量大小")
	}
	if !equalInts(bfsDist(g, 0), []int{0, 1, 2, -1, -1}) {
		panic("不可达距离应为 -1")
	}
	if len(shortestPath(g, 0, 4)) != 0 {
		panic("不可达路径应为空")
	}
	if !equalInts(dfsRecursive(g, 3), []int{3, 4}) {
		panic("第二个分量遍历")
	}

	// 5. 带环图：0-1-2-3-0 再加弦 0-2，BFS 距离不被环绕远
	g = newGraph(4)
	g.AddEdge(0, 1)
	g.AddEdge(1, 2)
	g.AddEdge(2, 3)
	g.AddEdge(3, 0)
	g.AddEdge(0, 2)
	if !equalInts(bfsOrder(g, 0), []int{0, 1, 2, 3}) {
		panic("带环 bfs 顺序")
	}
	if !equalInts(bfsDist(g, 0), []int{0, 1, 1, 1}) {
		panic("带环最短路")
	}
	if !equalInts(shortestPath(g, 0, 3), []int{0, 3}) {
		panic("带环路径应走弦边")
	}
	if componentCount(g) != 1 {
		panic("带环图应连通")
	}

	// 6. 菱形 0-1、0-2、1-2、1-3、2-3：
	//    若迭代 DFS 出栈才标记，3 会被压两次，顺序长度也会超出 V。
	g = newGraph(4)
	g.AddEdge(0, 1)
	g.AddEdge(0, 2)
	g.AddEdge(1, 2)
	g.AddEdge(1, 3)
	g.AddEdge(2, 3)
	it := dfsIterative(g, 0)
	if len(it) != 4 || !allUnique(it, 4) || it[0] != 0 {
		panic("迭代 dfs 出现重复访问")
	}
	// 同一份输入重复跑，结果必须完全一致
	if !equalInts(dfsIterative(g, 0), it) {
		panic("迭代 dfs 结果不可复现")
	}
	rec := dfsRecursive(g, 0)
	if len(rec) != 4 || !allUnique(rec, 4) {
		panic("递归 dfs 访问数量不对")
	}
	if !equalInts(rec, []int{0, 1, 2, 3}) {
		panic("递归 dfs 顺序不对")
	}
	if !equalInts(bfsDist(g, 0), []int{0, 1, 1, 2}) {
		panic("菱形 bfs 距离")
	}

	fmt.Println("all tests passed")
}
