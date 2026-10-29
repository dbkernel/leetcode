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
  1. 矩阵置 m[0][1] = m[1][0] = 1，m[1][2] = m[2][1] = 1；
  2. 邻接表 adj[0] = [1]，adj[1] = [0,2]，adj[2] = [1]；
  3. 度数和 1 + 2 + 1 = 4 = 2E，两边集都是 {(0,1),(1,2)}。

复杂度分析：
  时间：建图 O(V^2) 矩阵 / O(V+E) 邻接表，查询见上方分操作表。
  空间：O(V^2) 矩阵，O(V+E) 邻接表。
  适用：V 小或边密集用矩阵，V 大且边稀疏用邻接表。
*/
#![allow(dead_code)]

// 邻接矩阵：m[u * n + v] 非 0 表示有边 u->v。
struct GraphMatrix {
    n: usize,
    directed: bool,
    m: Vec<u8>,
    edges: usize,
}

impl GraphMatrix {
    fn new(n: usize, directed: bool) -> Self {
        GraphMatrix {
            n,
            directed,
            m: vec![0; n * n],
            edges: 0,
        }
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.m[u * self.n + v] != 0
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        if self.has_edge(u, v) {
            return; // 重边按集合语义忽略
        }
        self.m[u * self.n + v] = 1;
        self.edges += 1;
        // 无向图对称置位；自环 u == v 时同一格只置一次
        if !self.directed && u != v {
            self.m[v * self.n + u] = 1;
        }
    }

    fn neighbors(&self, u: usize) -> Vec<usize> {
        let mut res = Vec::new();
        for v in 0..self.n {
            if self.m[u * self.n + v] != 0 {
                res.push(v);
            }
        }
        res
    }

    // 无向图的度：自环只占一个格，但图论上算 2 度。
    fn degree(&self, u: usize) -> usize {
        let mut d = 0;
        for v in 0..self.n {
            if self.m[u * self.n + v] != 0 {
                d += if u == v { 2 } else { 1 };
            }
        }
        d
    }

    fn out_degree(&self, u: usize) -> usize {
        let mut d = 0;
        for v in 0..self.n {
            if self.m[u * self.n + v] != 0 {
                d += 1;
            }
        }
        d
    }

    fn in_degree(&self, u: usize) -> usize {
        let mut d = 0;
        for v in 0..self.n {
            if self.m[v * self.n + u] != 0 {
                d += 1;
            }
        }
        d
    }

    fn edge_count(&self) -> usize {
        self.edges
    }

    // 规范边集：无向图只取上三角，自环只留一次；有向图全扫。
    fn edge_list(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for u in 0..self.n {
            let start = if self.directed { 0 } else { u };
            for v in start..self.n {
                if self.m[u * self.n + v] != 0 {
                    res.push((u, v));
                }
            }
        }
        res
    }
}

// 邻接表：adj[u] 存出边终点，保持升序，遍历顺序可复现。
struct GraphList {
    n: usize,
    directed: bool,
    adj: Vec<Vec<usize>>,
    indeg: Vec<usize>, // 有向图入度，加边时维护，查询 O(1)
    edges: usize,
}

impl GraphList {
    fn new(n: usize, directed: bool) -> Self {
        GraphList {
            n,
            directed,
            adj: vec![Vec::new(); n],
            indeg: vec![0; n],
            edges: 0,
        }
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        for &x in &self.adj[u] {
            if x == v {
                return true;
            }
        }
        false
    }

    // 从尾部找插入点再整体右移，O(degree)，不用库排序。
    fn insert_sorted(&mut self, u: usize, v: usize) {
        let mut pos = self.adj[u].len();
        while pos > 0 && self.adj[u][pos - 1] > v {
            pos -= 1;
        }
        self.adj[u].insert(pos, v);
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        if self.has_edge(u, v) {
            return; // 重边按集合语义忽略
        }
        self.insert_sorted(u, v);
        if self.directed {
            self.indeg[v] += 1;
        } else if u != v {
            self.insert_sorted(v, u);
        } else {
            // 无向自环存两份，使 degree(u) 自然等于 2
            self.insert_sorted(u, u);
        }
        self.edges += 1;
    }

    fn neighbors(&self, u: usize) -> Vec<usize> {
        self.adj[u].clone()
    }

    fn degree(&self, u: usize) -> usize {
        self.adj[u].len()
    }

    fn out_degree(&self, u: usize) -> usize {
        self.adj[u].len()
    }

    fn in_degree(&self, u: usize) -> usize {
        self.indeg[u]
    }

    fn edge_count(&self) -> usize {
        self.edges
    }

    fn edge_list(&self) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        for u in 0..self.n {
            for &v in &self.adj[u] {
                if self.directed {
                    res.push((u, v));
                } else if u < v {
                    res.push((u, v));
                } else if u == v {
                    // 自环存了两份，只保留第一条，否则边集多算一次
                    if res.last() != Some(&(u, v)) {
                        res.push((u, v));
                    }
                }
            }
        }
        res
    }
}

// 同一张图分别用两种存储建一遍，逐项对齐。
fn check_same_graph(gm: &GraphMatrix, gl: &GraphList) {
    assert_eq!(gm.n, gl.n);
    assert_eq!(gm.directed, gl.directed);
    assert_eq!(gm.edge_count(), gl.edge_count());
    assert_eq!(gm.edge_list(), gl.edge_list());
    for u in 0..gm.n {
        for v in 0..gm.n {
            assert_eq!(gm.has_edge(u, v), gl.has_edge(u, v));
        }
    }
    if gm.directed {
        let mut out_sum = 0;
        let mut in_sum = 0;
        for u in 0..gm.n {
            assert_eq!(gm.out_degree(u), gl.out_degree(u));
            assert_eq!(gm.in_degree(u), gl.in_degree(u));
            out_sum += gm.out_degree(u);
            in_sum += gm.in_degree(u);
        }
        // 有向图不变量：出度之和 = 入度之和 = E
        assert_eq!(out_sum, gm.edge_count());
        assert_eq!(in_sum, gm.edge_count());
    } else {
        let mut deg_sum = 0;
        for u in 0..gm.n {
            assert_eq!(gm.degree(u), gl.degree(u));
            deg_sum += gm.degree(u);
        }
        // 无向图不变量：度数和 = 2E；自环算 2 度正是为了这条成立
        assert_eq!(deg_sum, 2 * gm.edge_count());
    }
}

fn main() {
    // 1. 空图：任何查询都不能越界，边数必须是 0
    {
        let gm = GraphMatrix::new(4, false);
        let gl = GraphList::new(4, false);
        check_same_graph(&gm, &gl);
        assert_eq!(gm.edge_count(), 0);
        for u in 0..4 {
            assert_eq!(gm.degree(u), 0);
            assert!(gm.neighbors(u).is_empty());
            assert!(gl.neighbors(u).is_empty());
        }
        assert!(!gm.has_edge(0, 1));
    }

    // 2. 无向链 0-1-2，并重复加 (0,1) 验证重边不改变结构
    {
        let mut gm = GraphMatrix::new(3, false);
        let mut gl = GraphList::new(3, false);
        let es = [(0, 1), (1, 2)];
        for &(u, v) in es.iter() {
            gm.add_edge(u, v);
            gl.add_edge(u, v);
        }
        gm.add_edge(0, 1);
        gl.add_edge(0, 1);
        check_same_graph(&gm, &gl);
        assert_eq!(gm.edge_count(), 2);
        assert_eq!(gm.degree(1), 2);
        assert_eq!(gm.neighbors(1), vec![0, 2]);
        assert_eq!(gl.neighbors(1), vec![0, 2]);
    }

    // 3. 无向自环：矩阵只有一格，邻接表存两份，度都必须是 2
    {
        let mut gm = GraphMatrix::new(3, false);
        let mut gl = GraphList::new(3, false);
        gm.add_edge(0, 0);
        gl.add_edge(0, 0);
        gm.add_edge(0, 0); // 再次加同一条自环仍是重边
        gl.add_edge(0, 0);
        check_same_graph(&gm, &gl);
        assert_eq!(gm.edge_count(), 1);
        assert_eq!(gm.degree(0), 2);
        assert_eq!(gl.degree(0), 2);
        assert!(gm.has_edge(0, 0));
        assert_eq!(gm.neighbors(0), vec![0]);
        assert_eq!(gl.neighbors(0), vec![0, 0]);
        let mut sum = 0;
        for u in 0..3 {
            sum += gm.degree(u);
        }
        assert_eq!(sum, 2 * gm.edge_count());
    }

    // 4. 有向图含自环：入度、出度分别核对，并保持两个和不变
    {
        let mut gm = GraphMatrix::new(5, true);
        let mut gl = GraphList::new(5, true);
        let es = [(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 3)];
        for &(u, v) in es.iter() {
            gm.add_edge(u, v);
            gl.add_edge(u, v);
        }
        check_same_graph(&gm, &gl);
        assert_eq!(gm.edge_count(), 6);
        assert_eq!(gm.out_degree(3), 2);
        assert_eq!(gm.in_degree(3), 3); // 来自 1、2 以及 3 的自环
        assert_eq!(gm.in_degree(4), 1);
        assert!(gm.has_edge(3, 3));
        assert_eq!(gm.neighbors(3), vec![3, 4]);
        assert_eq!(gl.neighbors(3), vec![3, 4]);
    }

    // 5. 稀疏图：10 点 7 边，两条存储的边集完全一致
    {
        let mut gm = GraphMatrix::new(10, false);
        let mut gl = GraphList::new(10, false);
        let mut i = 0;
        while i + 3 < 10 {
            gm.add_edge(i, i + 3);
            gl.add_edge(i, i + 3);
            i += 1;
        }
        check_same_graph(&gm, &gl);
        assert_eq!(gm.edge_count(), 7);
        assert_eq!(gm.degree(0), 1);
    }

    // 6. 稠密图：7 点完全图，E = 21，每个点度数为 6
    {
        let mut gm = GraphMatrix::new(7, false);
        let mut gl = GraphList::new(7, false);
        for u in 0..7 {
            for v in (u + 1)..7 {
                gm.add_edge(u, v);
                gl.add_edge(u, v);
            }
        }
        check_same_graph(&gm, &gl);
        assert_eq!(gm.edge_count(), 21);
        for u in 0..7 {
            assert_eq!(gm.degree(u), 6);
        }
    }

    println!("all tests passed");
}
