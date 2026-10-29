/*
============================================================================
399. 除法求值（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/evaluate-division/
分类：并查集 · 带权合并

给你一个变量对数组 equations 和一个实数值数组 values 作为已知条件，其中
equations[i] = [Ai, Bi] 和 values[i] 共同表示等式 Ai / Bi = values[i] 。
每个 Ai 或 Bi 是一个表示单个变量的字符串。

另有一些以数组 queries 表示的问题，其中 queries[j] = [Cj, Dj] 表示第 j
个问题，请你根据已知条件找出 Cj / Dj = ? 的结果作为答案。

返回 所有问题的答案 。如果存在某个无法确定的答案，则用 -1.0 替代这个
答案。如果问题中出现了给定的已知条件中没有出现的字符串，也需要用 -1.0
替代这个答案。

注意：输入总是有效的。你可以假设除法运算中不会出现除数为 0 的情况，且
不存在任何矛盾的结果。

注意：未在等式列表中出现的变量是未定义的，因此无法确定它们的答案。

示例 1：
输入：equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
输出：[6.00000,0.50000,-1.00000,1.00000,-1.00000]
解释：
条件：a / b = 2.0, b / c = 3.0
问题：a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
结果：[6.0, 0.5, -1.0, 1.0, -1.0 ]
注意：x 是未定义的 => -1.0

示例 2：
输入：equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
输出：[3.75000,0.40000,5.00000,0.20000]

示例 3：
输入：equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
输出：[0.50000,2.00000,-1.00000,-1.00000]

提示：
- 1 <= equations.length <= 20
- equations[i].length == 2
- 1 <= Ai.length, Bi.length <= 5
- values.length == equations.length
- 0.0 < values[i] <= 20.0
- 1 <= queries.length <= 20
- queries[i].length == 2
- 1 <= Cj.length, Dj.length <= 5
- Ai, Bi, Cj, Dj 由小写英文字母与数字组成
*/
package main

import "fmt"

// 带权并查集：路径压缩 + 按大小合并。
// weight[x] = value(x) / value(parent[x])，根节点的 weight 为 1。
// 路径压缩时把“到旧父亲”和“旧父亲到根”两个比值相乘，
// 压缩后 weight[x] 直接等于 value(x) / value(root)。
// 按大小合并控制树高，两者合起来让查找均摊接近 O(α(n))。
type WeightedDSU struct {
	parent []int
	size   []int
	weight []float64
}

func newWeightedDSU(n int) *WeightedDSU {
	d := &WeightedDSU{
		parent: make([]int, n),
		size:   make([]int, n),
		weight: make([]float64, n),
	}
	for i := 0; i < n; i++ {
		d.parent[i] = i
		d.size[i] = 1
		d.weight[i] = 1.0
	}
	return d
}

func (d *WeightedDSU) Find(x int) int {
	if d.parent[x] != x {
		p := d.parent[x]
		root := d.Find(p)
		// Find(p) 已把 weight[p] 更新为 value(p)/value(root)
		d.weight[x] *= d.weight[p]
		d.parent[x] = root
	}
	return d.parent[x]
}

// 已知 value(a) / value(b) = val
func (d *WeightedDSU) Unite(a, b int, val float64) {
	ra, rb := d.Find(a), d.Find(b)
	if ra == rb {
		return
	}
	if d.size[ra] < d.size[rb] {
		// 把 ra 挂到 rb 下：weight[ra] = value(ra)/value(rb)
		// 由 weight[a]*V(ra) / (weight[b]*V(rb)) = val 反解得到
		d.weight[ra] = d.weight[b] * val / d.weight[a]
		d.parent[ra] = rb
		d.size[rb] += d.size[ra]
	} else {
		// 把 rb 挂到 ra 下：weight[rb] = value(rb)/value(ra)
		d.weight[rb] = d.weight[a] / (d.weight[b] * val)
		d.parent[rb] = ra
		d.size[ra] += d.size[rb]
	}
}

// 返回 value(a) / value(b)；根不同说明推不出来
func (d *WeightedDSU) Ratio(a, b int) float64 {
	if d.Find(a) != d.Find(b) {
		return -1.0
	}
	return d.weight[a] / d.weight[b]
}

/*
解法一：带权并查集【最优解】

为什么慢：最直接的做法是对每个查询重新建一次图再搜一条路径，Q 次查询
就要搜 Q 遍；只要有一条新等式，之前的搜索全部作废。

换个角度：把变量看成点，等式 a / b = v 看成一条带权的无向边。所有能
互相推导的变量属于同一个集合；只要维护“每个点到根的比值”，任意两个
变量的比值就是两次到根比值相除，一次查找即可回答。

关键点：

	· 权值定义要统一：weight[x] = value(x) / value(parent[x])，根为 1
	· 路径压缩必须同步乘：Find(x) 递归拿到根后执行
	  weight[x] *= weight[旧父亲]，压缩完 weight[x] 就是 value(x)/value(root)
	· 合并 a/b = val 时按 size 挂树：挂 rb 到 ra 下取
	  weight[rb] = weight[a] / (weight[b] * val)，挂反方向要取倒数
	· 查询先 Find 两端：根不同返回 -1.0；根相同返回 weight[a]/weight[b]
	· 变量先用哈希表编号，查询里没出现过的变量直接 -1.0

手推一遍：以 a/b = 2、b/c = 3 为例：

	Unite(a,b,2)：b 挂到 a 下，weight[b] = 1/(1*2) = 0.5，即 b = a/2
	Unite(b,c,3)：b 的根是 a，c 挂到 a 下，weight[c] = 0.5/(1*3) = 1/6
	查 a/c：weight[a]/weight[c] = 1/(1/6) = 6 ✔
	查 b/a：weight[b]/weight[a] = 0.5/1 = 0.5 ✔

复杂度分析：

	时间 O((N+Q) α(N))：N 个变量、Q 次查询，每次合并与查找均摊接近常数
	空间 O(N)：parent、size、weight 三个切片加上变量编号表

解法二：建图 + DFS 求路径乘积【非最优】

为什么慢：每次查询都要在图上重新走一条路径，Q 次查询就是 O(QN)；
带权并查集把每个变量直接挂到根上，查找摊还接近常数。

换个角度：把每条等式建成两条有向边：a→b 权 values[i]，b→a 权倒数。
查询 c/d 就是从 c 出发搜到 d，把路径上的权值相乘；搜不到就是 -1.0。

关键点：

	· 必须双向建边且反向权值取倒数，否则 b/a 这类查询查不出来
	· 查询两端相同时：在图中返回 1.0，不在图中返回 -1.0
	· DFS 带 visited，避免在环上绕圈；路径乘积沿递归累乘
	· 变量不在邻接表中直接 -1.0，不要为查询临时建节点
	· 取舍：图 DFS 更好写，还能直接给出路径；但每问一次都要重走一条
    路径，渐近上劣于并查集，此处只作写法对照

手推一遍：仍以 a/b = 2、b/c = 3 为例：

	建边：a→b:2、b→a:0.5、b→c:3、c→b:1/3
	查 a/c：a→b 权 2，b→c 权 3，乘积 2*3 = 6 ✔
	查 b/a：直接走 b→a，权 0.5 ✔

复杂度分析：

	时间 O(N + Q·(V+E))：建图 O(N)，每次查询一次 DFS，V+E = O(N)
	空间 O(N)：邻接表、visited 与递归栈
*/
func calcEquation(equations [][]string, values []float64,
	queries [][]string) []float64 {
	id := make(map[string]int)
	getID := func(s string) int {
		if v, ok := id[s]; ok {
			return v
		}
		nxt := len(id)
		id[s] = nxt
		return nxt
	}
	// 先给所有变量编号，并查集的容量才能一次定好
	for _, e := range equations {
		getID(e[0])
		getID(e[1])
	}

	dsu := newWeightedDSU(len(id))
	for i, e := range equations {
		dsu.Unite(getID(e[0]), getID(e[1]), values[i])
	}

	ans := make([]float64, 0, len(queries))
	for _, q := range queries {
		ia, okA := id[q[0]]
		ib, okB := id[q[1]]
		// 未出现过的变量无法确定答案
		if !okA || !okB {
			ans = append(ans, -1.0)
			continue
		}
		ans = append(ans, dsu.Ratio(ia, ib))
	}
	return ans
}

// 解法二：建图 + DFS 求路径乘积
type edge struct {
	to string
	w  float64
}

func calcEquationDFS(equations [][]string, values []float64,
	queries [][]string) []float64 {
	graph := make(map[string][]edge)
	for i, e := range equations {
		a, b := e[0], e[1]
		graph[a] = append(graph[a], edge{b, values[i]})
		// 反向边取倒数，b/a 才能顺着走回去
		graph[b] = append(graph[b], edge{a, 1.0 / values[i]})
	}

	ans := make([]float64, 0, len(queries))
	for _, q := range queries {
		if _, ok := graph[q[0]]; !ok {
			ans = append(ans, -1.0)
			continue
		}
		if _, ok := graph[q[1]]; !ok {
			ans = append(ans, -1.0)
			continue
		}
		if q[0] == q[1] {
			ans = append(ans, 1.0)
			continue
		}
		seen := make(map[string]bool)
		ans = append(ans, dfs399(graph, q[0], q[1], 1.0, seen))
	}
	return ans
}

// 所有 values 为正，累乘结果必为正，-1.0 可安全当失败标记
func dfs399(graph map[string][]edge, cur, target string, acc float64,
	seen map[string]bool) float64 {
	if cur == target {
		return acc
	}
	seen[cur] = true
	for _, e := range graph[cur] {
		if seen[e.to] {
			continue
		}
		if got := dfs399(graph, e.to, target, acc*e.w, seen); got != -1.0 {
			return got
		}
	}
	return -1.0
}

func closeEnough399(a, b float64) bool {
	d := a - b
	if d < 0 {
		d = -d
	}
	return d < 1e-5
}

func check(equations [][]string, values []float64,
	queries [][]string, want []float64) {
	got1 := calcEquation(equations, values, queries)
	got2 := calcEquationDFS(equations, values, queries)
	if len(got1) != len(want) || len(got2) != len(want) {
		panic(fmt.Sprintf("size mismatch: %v %v want %v", got1, got2, want))
	}
	for i := range want {
		if !closeEnough399(got1[i], want[i]) {
			panic(fmt.Sprintf("dsu[%d] = %v, want %v", i, got1[i], want[i]))
		}
		if !closeEnough399(got2[i], want[i]) {
			panic(fmt.Sprintf("dfs[%d] = %v, want %v", i, got2[i], want[i]))
		}
	}
}

func main() {
	// 示例 1：含未知变量 x、相同变量 a/a
	check([][]string{{"a", "b"}, {"b", "c"}}, []float64{2.0, 3.0},
		[][]string{{"a", "c"}, {"b", "a"}, {"a", "e"}, {"a", "a"}, {"x", "x"}},
		[]float64{6.0, 0.5, -1.0, 1.0, -1.0})

	// 示例 2：两条互不相连的链
	check([][]string{{"a", "b"}, {"b", "c"}, {"bc", "cd"}},
		[]float64{1.5, 2.5, 5.0},
		[][]string{{"a", "c"}, {"c", "b"}, {"bc", "cd"}, {"cd", "bc"}},
		[]float64{3.75, 0.4, 5.0, 0.2})

	// 示例 3：单条等式，正反查询互为倒数
	check([][]string{{"a", "b"}}, []float64{0.5},
		[][]string{{"a", "b"}, {"b", "a"}, {"a", "c"}, {"x", "y"}},
		[]float64{0.5, 2.0, -1.0, -1.0})

	// 不可达：b 与 c 之间没有任何等式
	check([][]string{{"a", "b"}}, []float64{2.0},
		[][]string{{"b", "c"}, {"c", "a"}}, []float64{-1.0, -1.0})

	// a/b 之后查 b/a，必须正好是倒数
	check([][]string{{"a", "b"}}, []float64{2.0},
		[][]string{{"a", "b"}, {"b", "a"}}, []float64{2.0, 0.5})

	// 长链：a/b=2、b/c=3、c/d=4，a/d 应是 24，d/a 是 1/24
	check([][]string{{"a", "b"}, {"b", "c"}, {"c", "d"}},
		[]float64{2.0, 3.0, 4.0},
		[][]string{{"a", "d"}, {"d", "a"}, {"b", "d"}},
		[]float64{24.0, 1.0 / 24.0, 12.0})

	fmt.Println("all tests passed")
}
