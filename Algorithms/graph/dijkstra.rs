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
  · 堆自己写：push 上浮，pop 用末尾元素补根后下沉；不用 BinaryHeap。

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
  不可达约定：dist 里用 -1 表示不可达，内部计算用 INF = 1e9。
*/
#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};

// 内部距离上界，只要比任何可能的最短路大即可；输出前统一换成 -1。
const INF: i32 = 1_000_000_000;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    w: i32,
}

// 邻接表：g[u] 存放 u 的全部出边。
type Graph = Vec<Vec<Edge>>;

// 仅用于自测：统计堆弹出次数，用来证明「过期记录确实存在且被跳过」。
static POPS: AtomicU64 = AtomicU64::new(0);

// 手写二叉小根堆，元素是 (dist, vertex) 二元组。
struct MinHeap {
    d: Vec<i32>,    // 距离
    v: Vec<usize>,  // 顶点
    n: usize,       // 逻辑大小，容量可以更大
}

impl MinHeap {
    fn new() -> MinHeap {
        MinHeap { d: Vec::new(), v: Vec::new(), n: 0 }
    }

    fn empty(&self) -> bool {
        self.n == 0
    }

    fn push(&mut self, dist: i32, vert: usize) {
        if self.n == self.d.len() {
            self.d.push(0);
            self.v.push(0);
        }
        let mut i = self.n; // 新元素放到末尾，再一路向上浮
        self.n += 1;
        self.d[i] = dist;
        self.v[i] = vert;
        while i > 0 {
            let p = (i - 1) / 2; // 完全二叉树的父下标
            if self.d[p] <= self.d[i] {
                break; // 父不比子大，堆性质已满足
            }
            self.d.swap(p, i);
            self.v.swap(p, i);
            i = p;
        }
    }

    // 弹出堆顶；调用方保证堆非空。
    fn pop(&mut self) -> (i32, usize) {
        let top = (self.d[0], self.v[0]);
        self.n -= 1;
        if self.n == 0 {
            return top;
        }
        self.d[0] = self.d[self.n]; // 末尾元素补到根，再下沉
        self.v[0] = self.v[self.n];
        let mut i = 0;
        loop {
            let l = 2 * i + 1;
            let r = l + 1;
            let mut m = i;
            if l < self.n && self.d[l] < self.d[m] {
                m = l;
            }
            if r < self.n && self.d[r] < self.d[m] {
                m = r;
            }
            if m == i {
                break;
            }
            self.d.swap(m, i);
            self.v.swap(m, i);
            i = m;
        }
        top
    }
}

struct ShortestPath {
    dist: Vec<i32>,   // 不可达为 -1
    parent: Vec<i32>, // 源点与不可达点为 -1
}

// 堆优化 Dijkstra，O((V + E) log V)。
fn dijkstra_heap(g: &Graph, src: usize) -> ShortestPath {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut parent = vec![-1; n];
    dist[src] = 0;
    let mut h = MinHeap::new();
    h.push(0, src);
    while !h.empty() {
        let (d, u) = h.pop();
        POPS.fetch_add(1, Ordering::Relaxed);
        // 惰性删除：堆里可能躺着 u 的旧距离，旧记录直接丢弃。
        if d > dist[u] {
            continue;
        }
        for e in &g[u] {
            let nd = d + e.w;
            // 必须严格小于才更新：相等时不动 parent，避免出现环。
            if nd < dist[e.to] {
                dist[e.to] = nd;
                parent[e.to] = u as i32;
                h.push(nd, e.to);
            }
        }
    }
    for x in dist.iter_mut() {
        if *x == INF {
            *x = -1;
        }
    }
    ShortestPath { dist, parent }
}

// 朴素 Dijkstra，O(V^2)：每轮线性扫描找未确定点里 dist 最小的。
// 小图（V 几十）时它常常比堆版本快，省掉了堆的常数。
// 堆优化版本能到 O(E log V)，这里只作为对照。
fn dijkstra_naive(g: &Graph, src: usize) -> ShortestPath {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut parent = vec![-1; n];
    let mut done = vec![false; n];
    dist[src] = 0;
    for _ in 0..n {
        let mut u: i32 = -1;
        for v in 0..n {
            if done[v] || dist[v] == INF {
                continue;
            }
            if u == -1 || dist[v] < dist[u as usize] {
                u = v as i32;
            }
        }
        if u == -1 {
            break; // 剩下的点全不可达，提前收工
        }
        let u = u as usize;
        done[u] = true;
        for e in &g[u] {
            if dist[u] + e.w < dist[e.to] {
                dist[e.to] = dist[u] + e.w;
                parent[e.to] = u as i32;
            }
        }
    }
    for x in dist.iter_mut() {
        if *x == INF {
            *x = -1;
        }
    }
    ShortestPath { dist, parent }
}

// 顺着 parent 从 dst 回溯到 src，再整体反转；不可达返回空数组。
fn build_path(parent: &[i32], src: i32, dst: i32) -> Vec<i32> {
    let mut path: Vec<i32> = Vec::new();
    let mut v = dst;
    // parent 只会指向距离更小的点，所以循环一定在有限步内结束。
    while v != -1 {
        path.push(v);
        if v == src {
            break;
        }
        v = parent[v as usize];
    }
    if path.is_empty() || *path.last().unwrap() != src {
        return Vec::new();
    }
    let mut i = 0;
    let mut j = path.len() - 1;
    while i < j {
        path.swap(i, j);
        i += 1;
        j -= 1;
    }
    path
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

// 参照实现，仅用于校验：实现部分没有用它。
fn floyd(g: &Graph) -> Vec<Vec<i32>> {
    let n = g.len();
    let mut d = vec![vec![INF; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for u in 0..n {
        for e in &g[u] {
            if e.w < d[u][e.to] {
                d[u][e.to] = e.w; // 重边取最小
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            if d[i][k] == INF {
                continue;
            }
            for j in 0..n {
                if d[k][j] == INF {
                    continue;
                }
                if d[i][k] + d[k][j] < d[i][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }
    for row in d.iter_mut() {
        for x in row.iter_mut() {
            if *x == INF {
                *x = -1;
            }
        }
    }
    d
}

// 取 u->v 的最小边权，重边时取最小。
fn min_weight(g: &Graph, u: usize, v: usize) -> Option<i32> {
    let mut best: Option<i32> = None;
    for e in &g[u] {
        if e.to != v {
            continue;
        }
        if best.is_none() || e.w < best.unwrap() {
            best = Some(e.w);
        }
    }
    best
}

fn add(g: &mut Graph, u: usize, v: usize, w: i32) {
    g[u].push(Edge { to: v, w });
}

fn main() {
    // 1. 正常用例：手推的 6 节点图。
    let mut g: Graph = vec![Vec::new(); 6];
    add(&mut g, 0, 1, 7);
    add(&mut g, 0, 2, 9);
    add(&mut g, 0, 5, 14);
    add(&mut g, 1, 2, 10);
    add(&mut g, 1, 3, 15);
    add(&mut g, 2, 3, 11);
    add(&mut g, 2, 5, 2);
    add(&mut g, 3, 4, 6);
    add(&mut g, 5, 4, 9);
    let sp = dijkstra_heap(&g, 0);
    assert_eq!(sp.dist, vec![0, 7, 9, 20, 20, 11]);
    assert_eq!(build_path(&sp.parent, 0, 4), vec![0, 2, 5, 4]);
    assert_eq!(build_path(&sp.parent, 0, 3), vec![0, 2, 3]);
    // 朴素版本必须给出同样的距离。
    assert_eq!(dijkstra_naive(&g, 0).dist, sp.dist);

    // 2. 边界：单点、不可达、零权边、自环。
    let one: Graph = vec![Vec::new()];
    let s1 = dijkstra_heap(&one, 0);
    assert_eq!(s1.dist, vec![0]);
    assert_eq!(s1.parent, vec![-1]);
    assert_eq!(build_path(&s1.parent, 0, 0), vec![0]);

    let mut two: Graph = vec![Vec::new(); 2];
    add(&mut two, 0, 1, 5);
    assert_eq!(dijkstra_heap(&two, 0).dist, vec![0, 5]);
    let s3 = dijkstra_heap(&two, 1);
    assert_eq!(s3.dist, vec![-1, 0]);
    assert!(build_path(&s3.parent, 1, 0).is_empty());

    let mut zero: Graph = vec![Vec::new(); 3];
    add(&mut zero, 0, 1, 0);
    add(&mut zero, 1, 2, 0);
    let sz = dijkstra_heap(&zero, 0);
    assert_eq!(sz.dist, vec![0, 0, 0]);
    assert_eq!(build_path(&sz.parent, 0, 2), vec![0, 1, 2]);

    // 自环不参与最短路：0->0 权 1 不应该把 dist[0] 变大。
    let mut self_loop: Graph = vec![Vec::new(); 2];
    add(&mut self_loop, 0, 0, 1);
    add(&mut self_loop, 0, 1, 4);
    assert_eq!(dijkstra_heap(&self_loop, 0).dist, vec![0, 4]);

    // 3. 关键点：惰性删除。0->3 初始 10，经 0->1->3 变成 3，
    //    堆里那条 (10, 3) 成为过期记录，弹出时必须跳过。
    let mut dec: Graph = vec![Vec::new(); 5];
    add(&mut dec, 0, 3, 10);
    add(&mut dec, 0, 1, 1);
    add(&mut dec, 1, 3, 2);
    add(&mut dec, 3, 2, 1);
    add(&mut dec, 2, 4, 1);
    POPS.store(0, Ordering::Relaxed);
    let sd = dijkstra_heap(&dec, 0);
    assert_eq!(sd.dist, vec![0, 1, 4, 3, 5]);
    assert_eq!(build_path(&sd.parent, 0, 4), vec![0, 1, 3, 2, 4]);
    // 只有 5 个点却弹了 6 次，说明过期记录确实存在且被跳过。
    assert!(POPS.load(Ordering::Relaxed) > 5, "stale entries not seen");

    // 4. 随机对拍：200 组小图，堆版本 vs 朴素版本 vs Floyd。
    let mut rng = Rng::new(20240501);
    for round in 0..200 {
        let n = rng.in_range(1, 6) as usize;
        let mut rg: Graph = vec![Vec::new(); n];
        let m = rng.in_range(0, 3 * n as i32);
        for _ in 0..m {
            let u = rng.in_range(0, n as i32 - 1) as usize;
            let v = rng.in_range(0, n as i32 - 1) as usize;
            let w = rng.in_range(0, 9); // 含 0 权，仍满足非负
            add(&mut rg, u, v, w);
        }
        let src = rng.in_range(0, n as i32 - 1) as usize;
        let heap_res = dijkstra_heap(&rg, src);
        let naive_res = dijkstra_naive(&rg, src);
        let want = floyd(&rg);
        assert_eq!(heap_res.dist, naive_res.dist, "round {}", round);
        assert_eq!(heap_res.dist, want[src], "round {}", round);
        // parent 链必须真能还原出一条权重等于 dist 的路径。
        for v in 0..n {
            let path = build_path(&heap_res.parent, src as i32, v as i32);
            if heap_res.dist[v] == -1 {
                assert!(path.is_empty(), "round {}: {} has path", round, v);
                continue;
            }
            assert!(!path.is_empty(), "round {}: empty path", round);
            assert_eq!(path[0], src as i32);
            assert_eq!(path[path.len() - 1], v as i32);
            let mut sum = 0;
            for i in 0..path.len() - 1 {
                let w = min_weight(&rg, path[i] as usize, path[i + 1] as usize);
                assert!(w.is_some(), "round {}: missing edge", round);
                sum += w.unwrap();
            }
            assert_eq!(sum, heap_res.dist[v], "round {}", round);
        }
    }

    println!("all tests passed");
}
