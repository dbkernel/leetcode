/*
============================================================================
并查集（Disjoint Set Union）
分类：手写数据结构 · 并查集
============================================================================
操作复杂度（n 个元素、m 次操作）：
  find       O(alpha(n)) 均摊  路径压缩 + 按秩（按大小）合并同用
  unite      O(alpha(n)) 均摊  两次 find 加一次挂接
  connected  O(alpha(n)) 均摊  一次 find 比较根
  count      O(1)              维护分量计数器，合并成功才减一
============================================================================

用途：
  维护一堆互不相交的集合，支持「合并两个集合」和「查两个元素是否同
  集合」。面试里常用于判环、Kruskal、统计连通块、账户合并。

核心思想：
  每个集合是一棵有根树：parent[x] 指向父节点，树根指向自己。合并就是
  把一棵树的根挂到另一棵的根下，查询就是沿 parent 一路走到根。
  树越矮越快，于是用「按秩/按大小合并」压住高度，「路径压缩」压扁路径。

关键点（不变量）：
  · parent[x] == x 当且仅当 x 是根；所有操作入口都是 find，别绕开它。
  · 合并策略只选一种：按大小（size）或按秩（rank），不要两套同时维护，
    否则判断依据不一致，还白搭一个数组。
  · 路径压缩必须迭代：递归在 1e6 长的链上会爆栈。两趟写法先找根再
    把路径上所有点直挂根；路径减半写法边走边把点挂到祖父上，一趟完事。
  · unite 返回「是否真的合并了」：两端已同根时返回 false，且 count 不能
    减一。判环就靠这个返回值。

手推一遍：
  5 个元素 0..4，依次 union(0,1)、union(2,3)、union(0,2)、union(0,1)：
  1. union(0,1)：两块都只有一个点，把 1 挂到 0 下，count 5 -> 4。
  2. union(2,3)：同理把 3 挂到 2 下，count 4 -> 3。
  3. union(0,2)：find(0)=0，find(2)=2，size 都是 2 不交换，把 2 挂到
     0 下；1 和 3 仍在各自父节点下，count 3 -> 2。
  4. union(0,1)：find(0) == find(1) == 0，返回 false，count 仍是 2。
  此时分量是 {0,1,2,3} 与 {4}，connected(3, 1) 为 true。

复杂度分析：
  只做路径压缩：均摊 O(log n)，不是 O(1)；单次仍可能走到较深的节点。
  只做按秩合并：O(log n)，因为树高被限制在 log n 以内。
  两者同用：均摊 O(alpha(n))，alpha 是反 Ackermann 函数，增长极慢，
    但确实不是常数，所以只能说「近似 O(1)」，不能说严格 O(1)。
  空间 O(n)：parent 加一个辅助数组（size 或 rank，二选一）。
*/
package main

import (
	"fmt"
	"os"
)

// 合并方法统一叫 unite（C++ 里 union 是关键字），四语言保持同名。
// dsuBySize 按大小合并 + 两趟路径压缩。size 只有根节点上有意义。
type dsuBySize struct {
	parent []int
	size   []int
	comps  int // 当前连通分量数
}

func newDsuBySize(n int) *dsuBySize {
	d := &dsuBySize{parent: make([]int, n), size: make([]int, n), comps: n}
	for i := 0; i < n; i++ {
		d.parent[i] = i
		d.size[i] = 1
	}
	return d
}

func (d *dsuBySize) find(x int) int {
	root := x
	for d.parent[root] != root { // 第一趟：找根
		root = d.parent[root]
	}
	for d.parent[x] != root { // 第二趟：路径上每个点直挂根
		next := d.parent[x]
		d.parent[x] = root
		x = next
	}
	return root
}

// unite 返回 false 表示两端本来就在同一个集合，合并没有发生。
func (d *dsuBySize) unite(a, b int) bool {
	ra, rb := d.find(a), d.find(b)
	if ra == rb {
		return false
	}
	if d.size[ra] < d.size[rb] { // 小树挂大树
		ra, rb = rb, ra
	}
	d.parent[rb] = ra
	d.size[ra] += d.size[rb]
	d.comps--
	return true
}

func (d *dsuBySize) connected(a, b int) bool { return d.find(a) == d.find(b) }

func (d *dsuBySize) count() int { return d.comps }

// dsuByRank 按秩合并 + 路径减半。rank 是树高的上界，只有根节点有意义。
type dsuByRank struct {
	parent []int
	rank   []int
	comps  int
}

func newDsuByRank(n int) *dsuByRank {
	d := &dsuByRank{parent: make([]int, n), rank: make([]int, n), comps: n}
	for i := 0; i < n; i++ {
		d.parent[i] = i
	}
	return d
}

func (d *dsuByRank) find(x int) int {
	for d.parent[x] != x {
		d.parent[x] = d.parent[d.parent[x]] // 挂到祖父上，路径减半
		x = d.parent[x]
	}
	return x
}

func (d *dsuByRank) unite(a, b int) bool {
	ra, rb := d.find(a), d.find(b)
	if ra == rb {
		return false
	}
	if d.rank[ra] < d.rank[rb] {
		ra, rb = rb, ra
	}
	d.parent[rb] = ra
	// 只有两棵树一样高时，新树才会比原来高一层。
	if d.rank[ra] == d.rank[rb] {
		d.rank[ra]++
	}
	d.comps--
	return true
}

func (d *dsuByRank) connected(a, b int) bool { return d.find(a) == d.find(b) }

func (d *dsuByRank) count() int { return d.comps }

type edge struct {
	u int
	v int
}

// firstCycleEdge 按顺序加无向边，返回第一条会成环的边下标，没有则 -1。
// 注意「边数 > V - 1 必有环」只是必要条件：边数等于 V - 1 时也可能
// 有环（别处断开），准确判断只能靠每次 unite 的返回值。
func firstCycleEdge(n int, edges []edge) int {
	d := newDsuBySize(n)
	for i, e := range edges {
		if !d.unite(e.u, e.v) {
			return i
		}
	}
	return -1
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

// depthOf 从 x 走到根经过的边数，用来量树高有没有退化。
func depthOf(parent []int, x int) int {
	d := 0
	for parent[x] != x {
		x = parent[x]
		d++
	}
	return d
}

func main() {
	// 1. 逐步合并：连通性、count 的变化。
	d := newDsuBySize(5)
	check(d.count() == 5, "count starts at n")
	check(d.unite(0, 1), "unite 0-1")
	check(d.count() == 4, "count after 0-1")
	check(d.connected(0, 1), "0 and 1 connected")
	check(!d.connected(1, 2), "1 and 2 separate")
	check(d.unite(2, 3), "unite 2-3")
	check(d.count() == 3, "count after 2-3")
	check(d.unite(0, 2), "unite 0-2")
	check(d.connected(3, 1), "3 and 1 connected")
	check(d.count() == 2, "count after 0-2")

	// 2. 重复 merge：返回 false，count 不能变（这正是判环的依据）。
	check(!d.unite(0, 1), "repeat 0-1")
	check(d.count() == 2, "count unchanged")
	check(!d.unite(3, 0), "reverse repeat 3-0")
	check(d.count() == 2, "count still unchanged")

	// 按秩版本跑同样的序列，结果必须一致。
	r := newDsuByRank(5)
	check(r.unite(0, 1), "rank unite 0-1")
	check(r.unite(2, 3), "rank unite 2-3")
	check(r.unite(0, 2), "rank unite 0-2")
	check(r.connected(3, 1), "rank 3 and 1")
	check(r.count() == 2, "rank count 2")
	check(!r.unite(0, 1), "rank repeat")
	check(r.count() == 2, "rank count unchanged")

	// 3. 关键点一：两趟路径压缩把链压平。手工造 0->1->2->3->4。
	chain := newDsuBySize(5)
	for i := 0; i < 4; i++ {
		chain.parent[i] = i + 1
	}
	check(chain.find(0) == 4, "chain root")
	check(chain.parent[0] == 4, "0 hangs on root") // 第二趟的效果
	check(chain.parent[1] == 4, "1 hangs on root")
	check(depthOf(chain.parent, 0) == 1, "chain flattened")

	// 关键点二：路径减半只走一趟，隔一个挂一次，高度约减半。
	half := newDsuByRank(5)
	for i := 0; i < 4; i++ {
		half.parent[i] = i + 1
	}
	check(half.find(0) == 4, "half root")
	check(half.parent[0] == 2, "0 jumps to 2")
	check(half.parent[2] == 4, "2 jumps to 4")
	check(depthOf(half.parent, 0) == 2, "half depth")

	// 4. 应用：判环。
	check(firstCycleEdge(3, []edge{{0, 1}, {1, 2}, {2, 0}}) == 2, "triangle")
	check(firstCycleEdge(3, []edge{{0, 1}, {1, 2}}) == -1, "tree")
	check(firstCycleEdge(4,
		[]edge{{0, 1}, {1, 2}, {2, 3}, {3, 0}}) == 3, "square")
	// 边数正好等于 V - 1 = 3，却仍有环：3 号点孤立，别处三角形。
	check(firstCycleEdge(4, []edge{{0, 1}, {1, 2}, {2, 0}}) == 2,
		"v-1 edges but cyclic")

	// 5. 顺序合并 1e5 次：不能退化成链，树高必须是对数级。
	const n = 100000
	big := newDsuBySize(n)
	for i := 0; i+1 < n; i++ {
		check(big.unite(i, i+1), "sequential unite")
	}
	check(big.count() == 1, "all merged")
	check(big.connected(0, n-1), "0 and last connected")
	maxDepth := 0
	for i := 0; i < n; i++ {
		if dp := depthOf(big.parent, i); dp > maxDepth {
			maxDepth = dp
		}
	}
	check(maxDepth <= 40, "depth by size") // 2 * log2(1e5) 约 34

	bigRank := newDsuByRank(n)
	for i := 0; i+1 < n; i++ {
		check(bigRank.unite(i, i+1), "sequential unite rank")
	}
	check(bigRank.count() == 1, "rank all merged")
	maxDepth = 0
	for i := 0; i < n; i++ {
		if dp := depthOf(bigRank.parent, i); dp > maxDepth {
			maxDepth = dp
		}
	}
	check(maxDepth <= 40, "depth by rank")

	// 6. 随机操作序列：count 必须始终等于 n - 成功合并次数。
	gen := &rng{s: 20240503}
	for round := 0; round < 200; round++ {
		size := gen.in(1, 50)
		ops := newDsuBySize(size)
		merged := 0
		for k := 0; k < 100; k++ {
			a := gen.in(0, size-1)
			b := gen.in(0, size-1)
			if ops.unite(a, b) {
				merged++
			}
			check(ops.count() == size-merged, "count invariant")
		}
		// count == 1 时全图连通，任取两点必须 connected。
		if ops.count() == 1 {
			check(ops.connected(0, size-1), "fully connected")
		}
	}

	// 7. 边界：空集合、单元素（自己和自己同根，重复合并返回 false）。
	check(newDsuBySize(0).count() == 0, "empty size")
	check(newDsuByRank(0).count() == 0, "empty rank")
	one := newDsuBySize(1)
	check(one.count() == 1, "single size")
	check(one.connected(0, 0), "single connected")
	check(!one.unite(0, 0), "single unite self")
	check(one.count() == 1, "single count unchanged")
	oneRank := newDsuByRank(1)
	check(oneRank.connected(0, 0), "rank single connected")
	check(!oneRank.unite(0, 0), "rank single unite self")
	check(oneRank.count() == 1, "rank single count")

	fmt.Println("all tests passed")
}
