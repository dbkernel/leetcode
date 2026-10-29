/*
============================================================================
最小生成树（Minimum Spanning Tree: Kruskal + Prim）
分类：手写数据结构 · 图算法
复杂度：时间 Kruskal O(E log E)、Prim 朴素 O(V^2)；空间 O(V^2 + E)
============================================================================

用途：
  在带权无向图里挑一组边，把所有点连通且总权重最小。面试里常考
  Kruskal 的并查集实现、Prim 的「每次吸最近点」不变量，以及
  「图不连通怎么办」。

核心思想：
  Kruskal 按权重从小到大扫边，只要这条边连接两个不同连通块就收下
  （用并查集判断），收够 V-1 条即止。
  Prim 从一个点开始，维护「树到每个外部点的最近边」，每轮把最近的
  外部点吸进树，并更新它邻居的候选距离，像滚雪球。

关键点（不变量）：
  · Kruskal 的并查集必须「路径压缩 + 按秩（按大小）合并」一起用，
    只写一个都可能退化成长链，边一多就明显变慢。
  · 图不连通时得到最小生成森林：结果边数 = V - 连通分量数，必须按
    实际条数返回，不能硬写 V-1；总权重也只统计森林里的边。
  · 排序自己写：这里用自底向上归并排序（稳定、无递归），O(E log E)，
    不允许调用内置排序。
  · Prim 外层要再扫一遍未访问点，否则不连通图只覆盖 0 号点所在分量。
    堆优化 Prim 可到 O(E log V)，本文件不另写，只做朴素版对照。

手推一遍：
  三角形：边 (0,1,1)、(1,2,2)、(0,2,3)。
  1. Kruskal 排好序正好是 1、2、3：先收 (0,1)，两块合并；
     再收 (1,2)，两块合并。此时已有 2 = 3 - 1 条边，收工，总权重 3。
  2. Prim 从 0 出发：最近的是 1（权 1），吸入并把 2 的候选更新为 2；
     再选最近的 2（权 2）吸入。总权重同样是 3，边数 2。
  两者选中的边相同，都满足「边数 = V - 1」。

复杂度分析：
  Kruskal：排序 O(E log E) 主导，并查集 O(E alpha(V)) 被吸收，
    合计 O(E log E)。稀疏图占优。
  Prim 朴素：V 轮，每轮找最近点 O(V)、更新邻居 O(V)，合计 O(V^2)，
    稠密图占优。堆优化版是 O(E log V)，此处不实现。
  空间：Kruskal 是 O(V + E)（边数组 + 并查集）；Prim 朴素用邻接
    矩阵，是 O(V^2)。整个文件按较大者记作 O(V^2 + E)。
  无向图不讨论稳定性，只看总权重最小。
*/
package main

import (
	"fmt"
	"os"
)

const inf = 1000000000

type edge struct {
	u int
	v int
	w int
}

type mstResult struct {
	total      int    // 选中边的权重和
	edges      []edge // 实际选中的边，条数 = V - components
	components int    // 原图的连通分量数
}

// sortEdges 是手写自底向上归并排序，按 w 升序；稳定、无递归、空间 O(E)。
func sortEdges(es []edge) {
	n := len(es)
	if n < 2 {
		return
	}
	buf := make([]edge, n)
	src, dst := es, buf
	passes := 0
	for width := 1; width < n; width *= 2 {
		for lo := 0; lo < n; lo += 2 * width {
			mid := lo + width
			if mid > n {
				mid = n
			}
			hi := lo + 2*width
			if hi > n {
				hi = n
			}
			i, j, k := lo, mid, lo
			for i < mid && j < hi {
				// 相等时先取左边，保证稳定（虽然 MST 不依赖稳定性）。
				if src[j].w < src[i].w {
					dst[k] = src[j]
					j++
				} else {
					dst[k] = src[i]
					i++
				}
				k++
			}
			for i < mid {
				dst[k] = src[i]
				i++
				k++
			}
			for j < hi {
				dst[k] = src[j]
				j++
				k++
			}
		}
		src, dst = dst, src
		passes++
	}
	// 交换了奇数次时，结果还在 buf 里，需要搬回调用方的切片。
	if passes%2 == 1 {
		copy(es, src)
	}
}

// dsu 是 Kruskal 专用并查集：两趟路径压缩 + 按大小合并，缺一不可。
type dsu struct {
	parent []int
	size   []int
}

func newDsu(n int) *dsu {
	d := &dsu{parent: make([]int, n), size: make([]int, n)}
	for i := 0; i < n; i++ {
		d.parent[i] = i
		d.size[i] = 1
	}
	return d
}

func (d *dsu) find(x int) int {
	root := x
	for d.parent[root] != root {
		root = d.parent[root]
	}
	for d.parent[x] != root { // 第二趟：路径上所有点直挂根
		next := d.parent[x]
		d.parent[x] = root
		x = next
	}
	return root
}

// unite 返回 false 表示两端本来就在同一个连通块，这条边会成环。
func (d *dsu) unite(a, b int) bool {
	ra, rb := d.find(a), d.find(b)
	if ra == rb {
		return false
	}
	if d.size[ra] < d.size[rb] {
		ra, rb = rb, ra
	}
	d.parent[rb] = ra
	d.size[ra] += d.size[rb]
	return true
}

// kruskal：排序 + 并查集，O(E log E)。
func kruskal(n int, edges []edge) mstResult {
	sorted := make([]edge, len(edges))
	copy(sorted, edges)
	sortEdges(sorted)
	d := newDsu(n)
	res := mstResult{components: n}
	for _, e := range sorted {
		if !d.unite(e.u, e.v) { // 成环，跳过
			continue
		}
		res.total += e.w
		res.edges = append(res.edges, e)
		res.components--
	}
	return res
}

// primNaive 从 0 号点出发，每轮线性扫描 O(V) 选最近的外部点，总 O(V^2)。
// 图不连通时外层继续找未访问点，于是得到的是最小生成森林。
func primNaive(n int, edges []edge) mstResult {
	// 邻接矩阵，重边只留最小权重；O(V^2) 空间对朴素版可以接受。
	w := make([][]int, n)
	for i := 0; i < n; i++ {
		w[i] = make([]int, n)
		for j := 0; j < n; j++ {
			w[i][j] = inf
		}
	}
	for _, e := range edges {
		if e.w < w[e.u][e.v] {
			w[e.u][e.v] = e.w
			w[e.v][e.u] = e.w // 无向图，两个方向都记
		}
	}
	best := make([]int, n) // 树到该点的最近边
	from := make([]int, n) // 那条边从哪个树内点来
	in := make([]bool, n)  // 是否已进树
	for i := 0; i < n; i++ {
		best[i] = inf
		from[i] = -1
	}
	res := mstResult{}
	for s := 0; s < n; s++ {
		if in[s] {
			continue
		}
		res.components++
		best[s] = 0 // 起点不需要边连进树
		for round := 0; round < n; round++ {
			u := -1
			for v := 0; v < n; v++ {
				if in[v] || best[v] == inf {
					continue
				}
				if u == -1 || best[v] < best[u] {
					u = v
				}
			}
			if u == -1 { // 本分量吸完了
				break
			}
			in[u] = true
			if from[u] != -1 {
				res.total += best[u]
				res.edges = append(res.edges, edge{from[u], u, best[u]})
			}
			for v := 0; v < n; v++ {
				if !in[v] && w[u][v] < best[v] {
					best[v] = w[u][v]
					from[v] = u
				}
			}
		}
	}
	return res
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

func check(cond bool, msg string) {
	if !cond {
		fail("%s", msg)
	}
}

func sameEdge(a, b edge) bool {
	return (a.u == b.u && a.v == b.v) || (a.u == b.v && a.v == b.u)
}

// countComps 独立算一遍连通分量数，作为 components 字段的参照。
func countComps(n int, edges []edge) int {
	d := newDsu(n)
	for _, e := range edges {
		d.unite(e.u, e.v)
	}
	c := 0
	for i := 0; i < n; i++ {
		if d.find(i) == i {
			c++
		}
	}
	return c
}

// checkForest 校验选中的边：都来自输入、彼此不成环、权重和等于 total。
func checkForest(n int, r mstResult, tag string, input []edge) {
	d := newDsu(n)
	sum := 0
	for _, e := range r.edges {
		found := false
		for _, src := range input {
			if src.w == e.w && sameEdge(e, src) {
				found = true
			}
		}
		if !found {
			fail("case %s picked phantom edge (%d,%d,%d)",
				tag, e.u, e.v, e.w)
		}
		if !d.unite(e.u, e.v) {
			fail("case %s contains a cycle", tag)
		}
		sum += e.w
	}
	if sum != r.total {
		fail("case %s: sum %d != total %d", tag, sum, r.total)
	}
}

// checkConsistent 要求两版本总权重相同、分量数相同、边数都是 V - comps。
func checkConsistent(n int, es []edge) {
	comps := countComps(n, es)
	k := kruskal(n, es)
	p := primNaive(n, es)
	if k.total != p.total {
		fail("kruskal total %d != prim total %d", k.total, p.total)
	}
	if k.components != comps || p.components != comps {
		fail("components %d/%d != %d", k.components, p.components, comps)
	}
	if len(k.edges) != n-comps || len(p.edges) != n-comps {
		fail("edge counts %d/%d != %d", len(k.edges), len(p.edges), n-comps)
	}
	checkForest(n, k, "kruskal", es)
	checkForest(n, p, "prim", es)
}

func checkMst(name string, n int, es []edge, wantTotal, wantComps int) {
	checkConsistent(n, es)
	k := kruskal(n, es)
	if k.total != wantTotal || k.components != wantComps {
		fail("case %s: total %d (want %d), comps %d (want %d)",
			name, k.total, wantTotal, k.components, wantComps)
	}
}

func main() {
	// 1. 正常用例：三角形。
	checkMst("triangle", 3, []edge{{0, 1, 1}, {1, 2, 2}, {0, 2, 3}}, 3, 1)

	// 2. 重边：同一条 (0,1) 出现两次，必须选权重小的 2。
	checkMst("multi-edge", 3, []edge{{0, 1, 5}, {0, 1, 2}, {1, 2, 4}}, 6, 1)

	// 3. 不连通：{0,1} 和 {2,3} 各自成树，4 号点孤立。
	//    这是「边数 = V - comps = 5 - 3 = 2，而不是 V - 1 = 4」的用例。
	checkMst("disconnected", 5, []edge{{0, 1, 1}, {2, 3, 2}}, 3, 3)

	// 4. 边界：单点、空图。
	checkMst("single", 1, []edge{}, 0, 1)
	checkMst("empty", 0, []edge{}, 0, 0)

	// 5. 链：唯一生成树就是全部边。
	checkMst("chain", 5, []edge{{0, 1, 1}, {1, 2, 2}, {2, 3, 3}, {3, 4, 4}},
		10, 1)

	// 6. 关键点：必须跳过成环边。四边形四条 1 + 一条重对角边 5，
	//    只需 3 条边就够，总权重 3；若忘了判环会多收一条（甚至收下 5）。
	checkMst("square", 4,
		[]edge{{0, 1, 1}, {1, 2, 1}, {2, 3, 1}, {0, 3, 1}, {0, 2, 5}}, 3, 1)

	// 7. 排序自检：手写归并排序必须非递减，且元素一个不少。
	ugly := []edge{{0, 1, 5}, {1, 2, 1}, {2, 3, 5}, {3, 4, 2}, {4, 0, 3}}
	sortEdges(ugly)
	check(len(ugly) == 5, "sort keeps length")
	for i := 1; i < len(ugly); i++ {
		check(ugly[i-1].w <= ugly[i].w, "sort is non-decreasing")
	}
	check(ugly[0].w == 1 && ugly[4].w == 5, "sort bounds")

	// 8. 随机对拍：300 组，两版本总权重必须一致，且满足森林不变量。
	r := &rng{s: 20240502}
	for round := 0; round < 300; round++ {
		n := r.in(0, 8)
		m := r.in(0, 3*n)
		es := make([]edge, 0, m)
		for i := 0; i < m; i++ {
			u := r.in(0, n-1)
			v := r.in(0, n-1)
			w := r.in(1, 20)
			es = append(es, edge{u, v, w})
		}
		checkConsistent(n, es)
	}

	fmt.Println("all tests passed")
}
