/*
============================================================================
Dijkstra 单源最短路（堆优化 + 朴素对照）
分类：手写数据结构 · 图算法
复杂度：时间 O((V + E) log V)；空间 O(V + E)
============================================================================

用途：
  边权非负的单源最短路：给定源点 s，算出 s 到每个点的最短距离，
  并记录前驱，用于还原「一条具体路径」。面试里常要求手写堆，
  或者让你解释「为什么不能有负权边」。

核心思想：
  点分成「已确定」和「未确定」两拨。每次从堆里取出当前距离最小的
  未确定点 u，它的距离就此钉死，再用 u 松弛所有出边。堆里会残留
  同一个点的旧距离，弹出时和 dist[u] 比较，旧的就丢掉。

关键点（不变量）：
  · 只适用于权值 >= 0 的边。反例：0->1 权 2，0->2 权 5，2->1 权 -4。
    Dijkstra 先弹出 1（d=2）并钉死；随后 0->2->1 只有 1，却再也
    改不动 dist[1]。负权让「先弹出 = 已最优」的前提失效。
  · 弹出 (d, u) 后若 d > dist[u]，说明这是历史遗留的旧记录，直接跳过，
    这叫惰性删除：不改堆内部，靠丢弃过期项保证正确性。
  · 松弛成功必须同时写 parent[v] = u，否则还原出来的路径是错的。
  · 堆自己写：push 上浮，pop 用末尾元素补根后下沉；不用现成容器。

手推一遍：
  6 个点、有向边（u->v 权 w）：
    0->1 7，0->2 9，0->5 14
    1->2 10，1->3 15
    2->3 11，2->5 2
    3->4 6，5->4 9
  从 0 出发，按弹出顺序：
  1. 弹出 0（d=0），松弛得到 d1=7、d2=9、d5=14。
  2. 弹出 1（d=7）：0->1->2=17 不如 9，放弃；d3=22。
  3. 弹出 2（d=9）：d3=min(22, 20)=20，d5=min(14, 11)=11。
  4. 弹出 5（d=11）：d4=11+9=20。
  5. 弹出 3（d=20）：3->4 得 26，不如 20，放弃。
  6. 弹出 4（d=20）。
  dist = [0, 7, 9, 20, 20, 11]，到 4 的一条路径是 0 -> 2 -> 5 -> 4，
  长度 9 + 2 + 9 = 20。

复杂度分析：
  堆优化 O((V + E) log V)：每条边最多触发一次入堆，堆操作 O(log E)。
  朴素 O(V^2)：每轮线性扫一遍找最小。V 只有几十时它常常更快，
    因为堆版本多了上浮/下沉和判重的常数；稠密图也偏向朴素版。
  空间 O(V + E)：邻接表存 E 条边，dist / parent / 堆各 O(V)。
  不可达约定：dist 里用 -1 表示不可达，内部计算用 inf = 1e9。
*/
package main

import (
	"fmt"
	"os"
)

// 内部距离上界，只要比任何可能的最短路大即可；输出前统一换成 -1。
const inf = 1000000000

type edge struct {
	to int
	w  int
}

// 邻接表：g[u] 存放 u 的全部出边。
type graph [][]edge

// 仅用于自测：统计堆弹出次数，用来证明「过期记录确实存在且被跳过」。
var pops int

// minHeap 是手写的二叉小根堆，元素是 (dist, vertex) 二元组。
type minHeap struct {
	d []int // 距离
	v []int // 顶点
	n int   // 逻辑大小，容量可以更大
}

func (h *minHeap) empty() bool { return h.n == 0 }

func (h *minHeap) push(dist, vert int) {
	if h.n == len(h.d) {
		h.d = append(h.d, 0)
		h.v = append(h.v, 0)
	}
	i := h.n // 新元素放到末尾，再一路向上浮
	h.n++
	h.d[i] = dist
	h.v[i] = vert
	for i > 0 {
		p := (i - 1) / 2 // 完全二叉树的父下标
		if h.d[p] <= h.d[i] { // 父不比子大，堆性质已满足
			break
		}
		h.d[p], h.d[i] = h.d[i], h.d[p]
		h.v[p], h.v[i] = h.v[i], h.v[p]
		i = p
	}
}

// pop 弹出堆顶；调用方保证堆非空。
func (h *minHeap) pop() (int, int) {
	dist, vert := h.d[0], h.v[0]
	h.n--
	if h.n == 0 {
		return dist, vert
	}
	h.d[0], h.v[0] = h.d[h.n], h.v[h.n] // 末尾元素补到根
	i := 0
	for {
		l, r, m := 2*i+1, 2*i+2, i
		if l < h.n && h.d[l] < h.d[m] {
			m = l
		}
		if r < h.n && h.d[r] < h.d[m] {
			m = r
		}
		if m == i {
			break
		}
		h.d[m], h.d[i] = h.d[i], h.d[m]
		h.v[m], h.v[i] = h.v[i], h.v[m]
		i = m
	}
	return dist, vert
}

// dijkstraHeap 是堆优化版本，O((V + E) log V)。
func dijkstraHeap(g graph, src int) ([]int, []int) {
	n := len(g)
	dist := make([]int, n)
	parent := make([]int, n)
	for i := 0; i < n; i++ {
		dist[i] = inf
		parent[i] = -1
	}
	dist[src] = 0
	var h minHeap
	h.push(0, src)
	for !h.empty() {
		d, u := h.pop()
		pops++
		// 惰性删除：堆里可能躺着 u 的旧距离，旧记录直接丢弃。
		if d > dist[u] {
			continue
		}
		for _, e := range g[u] {
			nd := d + e.w
			// 必须严格小于才更新：相等时不动 parent，避免出现环。
			if nd < dist[e.to] {
				dist[e.to] = nd
				parent[e.to] = u
				h.push(nd, e.to)
			}
		}
	}
	for i := 0; i < n; i++ {
		if dist[i] == inf {
			dist[i] = -1
		}
	}
	return dist, parent
}

// dijkstraNaive 是朴素版本，O(V^2)：每轮线性扫描找最近的未确定点。
// 小图（V 几十）时它常常比堆版本快，省掉了堆的常数。
// 堆优化版本能到 O(E log V)，这里只作为对照。
func dijkstraNaive(g graph, src int) ([]int, []int) {
	n := len(g)
	dist := make([]int, n)
	parent := make([]int, n)
	done := make([]bool, n)
	for i := 0; i < n; i++ {
		dist[i] = inf
		parent[i] = -1
	}
	dist[src] = 0
	for round := 0; round < n; round++ {
		u := -1
		for v := 0; v < n; v++ {
			if done[v] || dist[v] == inf {
				continue
			}
			if u == -1 || dist[v] < dist[u] {
				u = v
			}
		}
		if u == -1 { // 剩下的点全不可达，提前收工
			break
		}
		done[u] = true
		for _, e := range g[u] {
			if dist[u]+e.w < dist[e.to] {
				dist[e.to] = dist[u] + e.w
				parent[e.to] = u
			}
		}
	}
	for i := 0; i < n; i++ {
		if dist[i] == inf {
			dist[i] = -1
		}
	}
	return dist, parent
}

// buildPath 顺着 parent 从 dst 回溯到 src，再整体反转。
// 不可达时返回空切片。
func buildPath(parent []int, src, dst int) []int {
	path := []int{}
	v := dst
	// parent 只会指向距离更小的点，所以循环一定在有限步内结束。
	for v != -1 {
		path = append(path, v)
		if v == src {
			break
		}
		v = parent[v]
	}
	if len(path) == 0 || path[len(path)-1] != src {
		return []int{}
	}
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}
	return path
}

// ---------------------------------------------------------------------------
// 以下均为自测代码。
// ---------------------------------------------------------------------------

// rng 是固定种子的 64 位 LCG：四语言使用同一套参数，失败可复现。
type rng struct{ s uint64 }

func (r *rng) next() uint32 {
	r.s = r.s*6364136223846793005 + 1442695040888963407
	return uint32(r.s >> 33)
}

func (r *rng) in(lo, hi int) int {
	return lo + int(r.next()%uint32(hi-lo+1))
}

func fail(format string, a ...any) {
	fmt.Printf(format+"\n", a...)
	os.Exit(1)
}

func checkVec(name string, got, want []int) {
	if len(got) != len(want) {
		fail("case %s failed: len %d != %d (got %v)",
			name, len(got), len(want), got)
	}
	for i := range got {
		if got[i] != want[i] {
			fail("case %s failed at %d: got %v want %v",
				name, i, got, want)
		}
	}
}

// floyd 是参照实现，仅用于校验：实现部分没有用它。
func floyd(g graph) [][]int {
	n := len(g)
	d := make([][]int, n)
	for i := 0; i < n; i++ {
		d[i] = make([]int, n)
		for j := 0; j < n; j++ {
			d[i][j] = inf
		}
		d[i][i] = 0
	}
	for u := 0; u < n; u++ {
		for _, e := range g[u] {
			if e.w < d[u][e.to] { // 重边取最小
				d[u][e.to] = e.w
			}
		}
	}
	for k := 0; k < n; k++ {
		for i := 0; i < n; i++ {
			if d[i][k] == inf {
				continue
			}
			for j := 0; j < n; j++ {
				if d[k][j] == inf {
					continue
				}
				if d[i][k]+d[k][j] < d[i][j] {
					d[i][j] = d[i][k] + d[k][j]
				}
			}
		}
	}
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if d[i][j] == inf {
				d[i][j] = -1
			}
		}
	}
	return d
}

// minWeight 取 u->v 的最小边权，重边时取最小。
func minWeight(g graph, u, v int) (int, bool) {
	w, found := 0, false
	for _, e := range g[u] {
		if e.to != v {
			continue
		}
		if !found || e.w < w {
			w = e.w
			found = true
		}
	}
	return w, found
}

func main() {
	// 1. 正常用例：手推的 6 节点图。
	g := make(graph, 6)
	add := func(u, v, w int) { g[u] = append(g[u], edge{v, w}) }
	add(0, 1, 7)
	add(0, 2, 9)
	add(0, 5, 14)
	add(1, 2, 10)
	add(1, 3, 15)
	add(2, 3, 11)
	add(2, 5, 2)
	add(3, 4, 6)
	add(5, 4, 9)
	dist, parent := dijkstraHeap(g, 0)
	checkVec("six-node dist", dist, []int{0, 7, 9, 20, 20, 11})
	checkVec("six-node path to 4", buildPath(parent, 0, 4), []int{0, 2, 5, 4})
	checkVec("six-node path to 3", buildPath(parent, 0, 3), []int{0, 2, 3})
	// 朴素版本必须给出同样的距离。
	naiveDist, _ := dijkstraNaive(g, 0)
	checkVec("six-node naive", naiveDist, dist)

	// 2. 边界：单点、单点自环、不可达、零权边。
	one := make(graph, 1)
	d1, p1 := dijkstraHeap(one, 0)
	checkVec("single dist", d1, []int{0})
	checkVec("single parent", p1, []int{-1})
	checkVec("single path", buildPath(p1, 0, 0), []int{0})

	two := make(graph, 2)
	two[0] = append(two[0], edge{1, 5})
	d2, _ := dijkstraHeap(two, 0)
	checkVec("unreachable from 0", d2, []int{0, 5})
	d3, p3 := dijkstraHeap(two, 1)
	checkVec("unreachable from 1", d3, []int{-1, 0})
	if len(buildPath(p3, 1, 0)) != 0 {
		fail("case unreachable path should be empty")
	}

	zero := make(graph, 3)
	zero[0] = append(zero[0], edge{1, 0})
	zero[1] = append(zero[1], edge{2, 0})
	dz, pz := dijkstraHeap(zero, 0)
	checkVec("zero weight", dz, []int{0, 0, 0})
	checkVec("zero weight path", buildPath(pz, 0, 2), []int{0, 1, 2})

	// 自环不参与最短路：0->0 权 1 不应该把 dist[0] 变大。
	selfLoop := make(graph, 2)
	selfLoop[0] = append(selfLoop[0], edge{0, 1})
	selfLoop[0] = append(selfLoop[0], edge{1, 4})
	ds, _ := dijkstraHeap(selfLoop, 0)
	checkVec("self loop", ds, []int{0, 4})

	// 3. 关键点：惰性删除。0->3 初始 10，经 0->1->3 变成 3，
	//    堆里那条 (10, 3) 成为过期记录，弹出时必须跳过。
	dec := make(graph, 5)
	dec[0] = append(dec[0], edge{3, 10})
	dec[0] = append(dec[0], edge{1, 1})
	dec[1] = append(dec[1], edge{3, 2})
	dec[3] = append(dec[3], edge{2, 1})
	dec[2] = append(dec[2], edge{4, 1})
	pops = 0
	dd, pd := dijkstraHeap(dec, 0)
	checkVec("stale dist", dd, []int{0, 1, 4, 3, 5})
	checkVec("stale path to 4", buildPath(pd, 0, 4), []int{0, 1, 3, 2, 4})
	// 只有 5 个点却弹了 6 次，说明过期记录确实存在且被跳过。
	if pops <= 5 {
		fail("stale entries not observed: pops = %d", pops)
	}

	// 4. 随机对拍：200 组小图，堆版本 vs 朴素版本 vs Floyd。
	r := &rng{s: 20240501}
	for round := 0; round < 200; round++ {
		n := r.in(1, 6)
		rg := make(graph, n)
		m := r.in(0, 3*n)
		for i := 0; i < m; i++ {
			u := r.in(0, n-1)
			v := r.in(0, n-1)
			w := r.in(0, 9) // 含 0 权，仍满足非负
			rg[u] = append(rg[u], edge{v, w})
		}
		src := r.in(0, n-1)
		heapDist, heapParent := dijkstraHeap(rg, src)
		naiveRes, _ := dijkstraNaive(rg, src)
		want := floyd(rg)
		checkVec("random heap vs naive", heapDist, naiveRes)
		checkVec("random heap vs floyd", heapDist, want[src])
		// parent 链必须真能还原出一条权重等于 dist 的路径。
		for v := 0; v < n; v++ {
			path := buildPath(heapParent, src, v)
			if heapDist[v] == -1 {
				if len(path) != 0 {
					fail("round %d: unreachable %d has path", round, v)
				}
				continue
			}
			if len(path) == 0 || path[0] != src || path[len(path)-1] != v {
				fail("round %d: bad path to %d: %v", round, v, path)
			}
			sum := 0
			for i := 0; i+1 < len(path); i++ {
				w, ok := minWeight(rg, path[i], path[i+1])
				if !ok {
					fail("round %d: no edge %d->%d", round, path[i], path[i+1])
				}
				sum += w
			}
			if sum != heapDist[v] {
				fail("round %d: path weight %d != dist %d",
					round, sum, heapDist[v])
			}
		}
	}

	fmt.Println("all tests passed")
}
