/*
============================================================================
拓扑排序（Topological Sort）
分类：手写算法 · 图
复杂度：时间 O(V+E)；空间 O(V)
============================================================================

用途：
  把有依赖关系的任务排成线性序，例如课程先修、构建依赖、
  任务调度；面试常问「能否修完所有课程」并要求给出一个方案。

核心思想：
  Kahn：不断取入度为 0 的点，删掉它的出边并让邻居入度减一，
  取不出点时剩下的点就在环上。
  DFS：三色标记做后序遍历，完成顺序反过来就是拓扑序，
  遇到仍在递归栈上的点说明有环。

关键点（不变量）：
  · 结果合法当且仅当每条边 u->v 都满足 pos[u] < pos[v]；
  · Kahn 输出长度小于 V 等价于图中存在环；
  · DFS 中颜色 1 表示在栈上，指向颜色 1 的边就是回边；
  · 同一 DAG 的拓扑序不唯一，测试只校验合法性不比对序列。

手推一遍：
  以 0->1、0->2、1->3、2->3 为例：
  1. Kahn 队列先放入度 0 的 0，弹出后 1、2 入度变 0；
  2. 依次弹出 1、2，最后 3 入度归零，得到 0,1,2,3；
  3. DFS 后序为 3,1,2,0，反转得到 0,2,1,3，同样合法。

复杂度分析：
  时间 O(V+E)：每个点入队一次，每条边被检查一次。
  空间 O(V)：入度、颜色、队列与结果数组。
  注意：只适用于有向无环图，有环时两个版本都要显式报失败。
*/
package main

import "fmt"

// Graph 有向图邻接表：邻居保持升序，输出的拓扑序才可复现。
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
	g.insertSorted(u, v) // 有向图只加单向边
}

func (g *Graph) Neighbors(u int) []int {
	return g.adj[u]
}

// ringQueue 手写环形队列：Kahn 里每个点最多入队一次，容量 V+1 足够。
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

// kahn 入度归零才入队；有环时会有顶点永远进不了队列。
func kahn(g *Graph) (bool, []int) {
	indeg := make([]int, g.n)
	for u := 0; u < g.n; u++ {
		for _, v := range g.Neighbors(u) {
			indeg[v]++ // 自环也计入
		}
	}
	q := newRingQueue(g.n + 1)
	for u := 0; u < g.n; u++ {
		if indeg[u] == 0 {
			q.push(u)
		}
	}
	order := []int{}
	for !q.empty() {
		u := q.pop()
		order = append(order, u)
		for _, v := range g.Neighbors(u) {
			indeg[v]--
			if indeg[v] == 0 {
				q.push(v)
			}
		}
	}
	// 输出不足 V 个点，说明剩下的点互相牵制，必有环
	return len(order) == g.n, order
}

func dfsVisit(g *Graph, u int, color []int, order *[]int) bool {
	color[u] = 1 // 1 = 在递归栈上
	for _, v := range g.Neighbors(u) {
		if color[v] == 1 {
			return false // 回边，成环
		}
		if color[v] == 0 && !dfsVisit(g, v, color, order) {
			return false
		}
	}
	color[u] = 2 // 2 = 完成，后序加入
	*order = append(*order, u)
	return true
}

// topoDfs 后序是「出度为 0 先完成」，整体反转才是拓扑序。
func topoDfs(g *Graph) (bool, []int) {
	color := make([]int, g.n) // 0 未访问 / 1 在栈上 / 2 完成
	order := []int{}
	for u := 0; u < g.n; u++ {
		if color[u] == 0 && !dfsVisit(g, u, color, &order) {
			return false, order
		}
	}
	for i, j := 0, len(order)-1; i < j; i, j = i+1, j-1 {
		order[i], order[j] = order[j], order[i]
	}
	return true, order
}

// isValidOrder 合法性校验：每个点的所有前驱都必须排在它前面。
func isValidOrder(g *Graph, order []int) bool {
	if len(order) != g.n {
		return false
	}
	pos := make([]int, g.n)
	for i := range pos {
		pos[i] = -1
	}
	for i, u := range order {
		if u < 0 || u >= g.n || pos[u] != -1 {
			return false
		}
		pos[u] = i
	}
	for u := 0; u < g.n; u++ {
		for _, v := range g.Neighbors(u) {
			if pos[u] >= pos[v] {
				return false // 边 u->v 被排反
			}
		}
	}
	return true
}

// Dep 一门课的先修关系：before 必须排在 after 之前。
type Dep struct {
	before int
	after  int
}

func courseSchedule(n int, deps []Dep) (bool, []int) {
	g := newGraph(n)
	for _, d := range deps {
		g.AddEdge(d.before, d.after)
	}
	ok, order := kahn(g)
	return ok && isValidOrder(g, order), order
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
	// 1. 单点：两个版本都必须给出 [0]
	g := newGraph(1)
	ok, a := kahn(g)
	ok2, b := topoDfs(g)
	if !ok || !ok2 {
		panic("单点应成功")
	}
	if !equalInts(a, []int{0}) || !equalInts(b, []int{0}) {
		panic("单点拓扑序应为 [0]")
	}
	if !isValidOrder(g, a) || !isValidOrder(g, b) {
		panic("单点序不合法")
	}

	// 2. 链 0->1->2->3->4：拓扑序唯一，两个版本应完全一致
	g = newGraph(5)
	for i := 0; i+1 < 5; i++ {
		g.AddEdge(i, i+1)
	}
	ok, a = kahn(g)
	ok2, b = topoDfs(g)
	if !ok || !ok2 {
		panic("链应成功")
	}
	expect := []int{0, 1, 2, 3, 4}
	if !equalInts(a, expect) || !equalInts(b, expect) {
		panic("链拓扑序应唯一")
	}

	// 3. 多个合法序：0->2、1->2、2->3，只校验合法性不比对序列
	g = newGraph(4)
	g.AddEdge(0, 2)
	g.AddEdge(1, 2)
	g.AddEdge(2, 3)
	ok, a = kahn(g)
	ok2, b = topoDfs(g)
	if !ok || !ok2 {
		panic("DAG 应成功")
	}
	if equalInts(a, b) {
		panic("本例两个算法应给出不同合法序")
	}
	if !isValidOrder(g, a) || !isValidOrder(g, b) {
		panic("合法序校验失败")
	}
	if len(a) != 4 || len(b) != 4 {
		panic("拓扑序应覆盖全部顶点")
	}

	// 4. 有环图：两个版本都要返回失败
	g = newGraph(3)
	g.AddEdge(0, 1)
	g.AddEdge(1, 2)
	g.AddEdge(2, 0)
	ok, a = kahn(g)
	ok2, _ = topoDfs(g)
	if ok || ok2 {
		panic("有环图不应成功")
	}
	if len(a) >= 3 {
		panic("Kahn 遇到环输出应不足 V")
	}

	// 5. 环不在 0 号点：1<->2，0 是挂在旁边的无环入口
	g = newGraph(3)
	g.AddEdge(0, 1)
	g.AddEdge(1, 2)
	g.AddEdge(2, 1)
	ok, _ = kahn(g)
	ok2, _ = topoDfs(g)
	if ok || ok2 {
		panic("局部环未被发现")
	}

	// 6. 有向自环：自己指向自己也是环，两个版本都不能放过
	g = newGraph(2)
	g.AddEdge(0, 0)
	g.AddEdge(1, 0)
	ok, _ = kahn(g)
	ok2, _ = topoDfs(g)
	if ok || ok2 {
		panic("自环未被发现")
	}

	// 7. 孤立点 + 一条边：全部顶点都要出现，顺序仍要合法
	g = newGraph(4)
	g.AddEdge(1, 2)
	ok, a = kahn(g)
	ok2, b = topoDfs(g)
	if !ok || !ok2 {
		panic("含孤立点的 DAG 应成功")
	}
	if len(a) != 4 || len(b) != 4 {
		panic("孤立点应出现在拓扑序中")
	}
	if !isValidOrder(g, a) || !isValidOrder(g, b) {
		panic("含孤立点的序不合法")
	}

	// 8. 应用：6 门课的先修关系，输出必须满足每一条依赖
	deps := []Dep{{0, 1}, {0, 2}, {1, 3}, {2, 3}, {3, 4}, {3, 5}}
	ok, order := courseSchedule(6, deps)
	if !ok {
		panic("课程安排应可行")
	}
	g = newGraph(6)
	for _, d := range deps {
		g.AddEdge(d.before, d.after)
	}
	if !isValidOrder(g, order) {
		panic("课程安排序不合法")
	}
	// 再逐条核对依赖，避免校验函数本身写错
	pos := make([]int, 6)
	for i, u := range order {
		pos[u] = i
	}
	for _, d := range deps {
		if pos[d.before] >= pos[d.after] {
			panic("先修课被排到了后面")
		}
	}

	// 9. 应用：课程之间成环，必须报告无法修完
	ok, _ = courseSchedule(3, []Dep{{0, 1}, {1, 2}, {2, 0}})
	if ok {
		panic("课程成环应返回 false")
	}

	fmt.Println("all tests passed")
}
