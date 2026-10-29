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
#![allow(dead_code)]

const INF: i32 = 1_000_000_000;

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: i32,
}

struct MstResult {
    total: i32,       // 选中边的权重和
    edges: Vec<Edge>, // 实际选中的边，条数 = V - components
    components: i32,  // 原图的连通分量数
}

// 手写自底向上归并排序，按 w 升序；稳定、无递归、空间 O(E)。
fn sort_edges(es: &mut [Edge]) {
    let n = es.len();
    if n < 2 {
        return;
    }
    let mut src = es.to_vec();
    let mut dst = vec![Edge { u: 0, v: 0, w: 0 }; n];
    let mut width = 1;
    while width < n {
        let mut lo = 0;
        while lo < n {
            let mid = (lo + width).min(n);
            let hi = (lo + 2 * width).min(n);
            let mut i = lo;
            let mut j = mid;
            let mut k = lo;
            while i < mid && j < hi {
                // 相等时先取左边，保证稳定（虽然 MST 不依赖稳定性）。
                if src[j].w < src[i].w {
                    dst[k] = src[j];
                    j += 1;
                } else {
                    dst[k] = src[i];
                    i += 1;
                }
                k += 1;
            }
            while i < mid {
                dst[k] = src[i];
                i += 1;
                k += 1;
            }
            while j < hi {
                dst[k] = src[j];
                j += 1;
                k += 1;
            }
            lo += 2 * width;
        }
        std::mem::swap(&mut src, &mut dst);
        width *= 2;
    }
    // 每轮写完都交换，所以 src 始终指向刚写满的数组，最后就是结果。
    es.copy_from_slice(&src);
}

// Kruskal 专用并查集：两趟路径压缩 + 按大小合并，缺一不可。
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Dsu {
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }
        Dsu { parent, size: vec![1; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        let mut cur = x;
        while self.parent[cur] != root { // 第二趟：路径上所有点直挂根
            let next = self.parent[cur];
            self.parent[cur] = root;
            cur = next;
        }
        root
    }

    // 返回 false 表示两端本来就在同一个连通块，这条边会成环。
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

// Kruskal：排序 + 并查集，O(E log E)。
fn kruskal(n: usize, edges: &[Edge]) -> MstResult {
    let mut sorted = edges.to_vec();
    sort_edges(&mut sorted);
    let mut dsu = Dsu::new(n);
    let mut res = MstResult {
        total: 0,
        edges: Vec::new(),
        components: n as i32,
    };
    for e in &sorted {
        if !dsu.unite(e.u, e.v) {
            continue; // 成环，跳过
        }
        res.total += e.w;
        res.edges.push(*e);
        res.components -= 1;
    }
    res
}

// Prim 朴素版：从 0 号点出发，每轮线性扫描 O(V) 选最近的外部点，
// 总 O(V^2)。图不连通时外层继续找未访问点，得到最小生成森林。
fn prim_naive(n: usize, edges: &[Edge]) -> MstResult {
    // 邻接矩阵，重边只留最小权重；O(V^2) 空间对朴素版可以接受。
    let mut w = vec![vec![INF; n]; n];
    for e in edges {
        if e.w < w[e.u][e.v] {
            w[e.u][e.v] = e.w;
            w[e.v][e.u] = e.w; // 无向图，两个方向都记
        }
    }
    let mut best = vec![INF; n]; // 树到该点的最近边
    let mut from = vec![-1i32; n]; // 那条边从哪个树内点来
    let mut inside = vec![false; n];
    let mut res = MstResult { total: 0, edges: Vec::new(), components: 0 };
    for s in 0..n {
        if inside[s] {
            continue;
        }
        res.components += 1;
        best[s] = 0; // 起点不需要边连进树
        for _ in 0..n {
            let mut u: i32 = -1;
            for v in 0..n {
                if inside[v] || best[v] == INF {
                    continue;
                }
                if u == -1 || best[v] < best[u as usize] {
                    u = v as i32;
                }
            }
            if u == -1 {
                break; // 本分量吸完了
            }
            let u = u as usize;
            inside[u] = true;
            if from[u] != -1 {
                res.total += best[u];
                res.edges.push(Edge { u: from[u] as usize, v: u, w: best[u] });
            }
            for v in 0..n {
                if !inside[v] && w[u][v] < best[v] {
                    best[v] = w[u][v];
                    from[v] = u as i32;
                }
            }
        }
    }
    res
}

// ---------------------------------------------------------------------------
// 以下均为自测代码。
// ---------------------------------------------------------------------------

// 固定种子的 64 位 LCG：四语言使用同一套参数，失败可复现。
struct Rng {
    s: u64,
}

impl Rng {
    fn new(seed: u64) -> Rng {
        Rng { s: seed }
    }

    fn next(&mut self) -> u32 {
        self.s = self
            .s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.s >> 33) as u32
    }

    fn in_range(&mut self, lo: i32, hi: i32) -> i32 {
        let span = (hi - lo + 1) as u32;
        lo + (self.next() % span) as i32
    }
}

fn same_edge(a: &Edge, b: &Edge) -> bool {
    (a.u == b.u && a.v == b.v) || (a.u == b.v && a.v == b.u)
}

// 独立算一遍连通分量数，作为 components 字段的参照。
fn count_comps(n: usize, edges: &[Edge]) -> i32 {
    let mut dsu = Dsu::new(n);
    for e in edges {
        dsu.unite(e.u, e.v);
    }
    let mut c = 0;
    for i in 0..n {
        if dsu.find(i) == i {
            c += 1;
        }
    }
    c
}

// 校验选中的边：都来自输入、彼此不成环、权重和等于 total。
fn check_forest(n: usize, res: &MstResult, tag: &str, input: &[Edge]) {
    let mut dsu = Dsu::new(n);
    let mut sum = 0;
    for e in &res.edges {
        let found = input.iter().any(|s| s.w == e.w && same_edge(e, s));
        assert!(found, "{} picked phantom edge ({},{},{})", tag, e.u, e.v, e.w);
        assert!(dsu.unite(e.u, e.v), "{} contains a cycle", tag);
        sum += e.w;
    }
    assert_eq!(sum, res.total, "{} sum mismatch", tag);
}

// 两版本总权重相同、分量数相同、边数都是 V - comps。
fn check_consistent(n: usize, es: &[Edge]) {
    let comps = count_comps(n, es);
    let k = kruskal(n, es);
    let p = prim_naive(n, es);
    assert_eq!(k.total, p.total, "kruskal total != prim total");
    assert_eq!(k.components, comps);
    assert_eq!(p.components, comps);
    assert_eq!(k.edges.len(), n - comps as usize);
    assert_eq!(p.edges.len(), n - comps as usize);
    check_forest(n, &k, "kruskal", es);
    check_forest(n, &p, "prim", es);
}

fn check_mst(
    name: &str,
    n: usize,
    es: &[Edge],
    want_total: i32,
    want_comps: i32,
) {
    check_consistent(n, es);
    let k = kruskal(n, es);
    assert_eq!(k.total, want_total, "case {} total", name);
    assert_eq!(k.components, want_comps, "case {} comps", name);
}

fn e(u: usize, v: usize, w: i32) -> Edge {
    Edge { u, v, w }
}

fn main() {
    // 1. 正常用例：三角形。
    check_mst("triangle", 3, &[e(0, 1, 1), e(1, 2, 2), e(0, 2, 3)], 3, 1);

    // 2. 重边：同一条 (0,1) 出现两次，必须选权重小的 2。
    check_mst("multi-edge", 3, &[e(0, 1, 5), e(0, 1, 2), e(1, 2, 4)], 6, 1);

    // 3. 不连通：{0,1} 和 {2,3} 各自成树，4 号点孤立。
    //    这是「边数 = V - comps = 5 - 3 = 2，而不是 V - 1 = 4」的用例。
    check_mst("disconnected", 5, &[e(0, 1, 1), e(2, 3, 2)], 3, 3);

    // 4. 边界：单点、空图。
    check_mst("single", 1, &[], 0, 1);
    check_mst("empty", 0, &[], 0, 0);

    // 5. 链：唯一生成树就是全部边。
    check_mst(
        "chain",
        5,
        &[e(0, 1, 1), e(1, 2, 2), e(2, 3, 3), e(3, 4, 4)],
        10,
        1,
    );

    // 6. 关键点：必须跳过成环边。四边形四条 1 + 一条重对角边 5，
    //    只需 3 条边就够，总权重 3；若忘了判环会多收一条（甚至收下 5）。
    check_mst(
        "square",
        4,
        &[e(0, 1, 1), e(1, 2, 1), e(2, 3, 1), e(0, 3, 1), e(0, 2, 5)],
        3,
        1,
    );

    // 7. 排序自检：手写归并排序必须非递减，且元素一个不少。
    let mut ugly = vec![
        e(0, 1, 5),
        e(1, 2, 1),
        e(2, 3, 5),
        e(3, 4, 2),
        e(4, 0, 3),
    ];
    sort_edges(&mut ugly);
    assert_eq!(ugly.len(), 5);
    for i in 1..ugly.len() {
        assert!(ugly[i - 1].w <= ugly[i].w);
    }
    assert_eq!(ugly[0].w, 1);
    assert_eq!(ugly[4].w, 5);

    // 8. 随机对拍：300 组，两版本总权重必须一致，且满足森林不变量。
    let mut rng = Rng::new(20240502);
    for _ in 0..300 {
        let n = rng.in_range(0, 8) as usize;
        let m = rng.in_range(0, 3 * n as i32);
        let mut es: Vec<Edge> = Vec::new();
        for _ in 0..m {
            let u = rng.in_range(0, n as i32 - 1) as usize;
            let v = rng.in_range(0, n as i32 - 1) as usize;
            let w = rng.in_range(1, 20);
            es.push(Edge { u, v, w });
        }
        check_consistent(n, &es);
    }

    println!("all tests passed");
}
