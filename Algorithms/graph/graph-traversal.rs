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
#![allow(dead_code)]

// 无向图邻接表：邻居保持升序，遍历结果才可复现。
struct Graph {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            n,
            adj: vec![Vec::new(); n],
        }
    }

    fn insert_sorted(&mut self, u: usize, v: usize) {
        let mut pos = self.adj[u].len();
        while pos > 0 && self.adj[u][pos - 1] > v {
            pos -= 1;
        }
        self.adj[u].insert(pos, v);
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.insert_sorted(u, v);
        if u != v {
            self.insert_sorted(v, u); // 自环只存一份即可
        }
    }

    fn neighbors(&self, u: usize) -> &Vec<usize> {
        &self.adj[u]
    }
}

// 手写环形队列：head/tail 取模回绕，cnt 区分空与满。
struct RingQueue {
    buf: Vec<usize>,
    head: usize,
    tail: usize,
    cnt: usize,
}

impl RingQueue {
    fn new(size: usize) -> Self {
        RingQueue {
            buf: vec![0; size],
            head: 0,
            tail: 0,
            cnt: 0,
        }
    }

    fn empty(&self) -> bool {
        self.cnt == 0
    }

    fn push(&mut self, x: usize) {
        assert!(self.cnt < self.buf.len()); // 调用方保证不会塞满
        self.buf[self.tail] = x;
        self.tail = (self.tail + 1) % self.buf.len();
        self.cnt += 1;
    }

    fn pop(&mut self) -> usize {
        assert!(self.cnt > 0);
        let x = self.buf[self.head];
        self.head = (self.head + 1) % self.buf.len();
        self.cnt -= 1;
        x
    }
}

fn dfs_visit(g: &Graph, u: usize, vis: &mut Vec<bool>, out: &mut Vec<usize>) {
    vis[u] = true;
    out.push(u); // 先序位置：进入时记录
    for &v in g.neighbors(u) {
        if !vis[v] {
            dfs_visit(g, v, vis, out);
        }
    }
}

fn dfs_recursive(g: &Graph, s: usize) -> Vec<usize> {
    let mut vis = vec![false; g.n];
    let mut out = Vec::new();
    dfs_visit(g, s, &mut vis, &mut out);
    out
}

fn dfs_iterative(g: &Graph, s: usize) -> Vec<usize> {
    let mut vis = vec![false; g.n];
    let mut st: Vec<usize> = Vec::new(); // 用 Vec 当栈，不用 VecDeque
    let mut out = Vec::new();
    vis[s] = true;
    st.push(s); // 关键：入栈时就标记，不能等出栈
    while let Some(u) = st.pop() {
        out.push(u);
        // 逆序压栈，弹出时才是邻居下标升序
        let nb = g.neighbors(u);
        for i in (0..nb.len()).rev() {
            let v = nb[i];
            if !vis[v] {
                vis[v] = true;
                st.push(v);
            }
        }
    }
    out
}

fn bfs_order(g: &Graph, s: usize) -> Vec<usize> {
    let mut vis = vec![false; g.n];
    let mut q = RingQueue::new(g.n + 1);
    let mut out = Vec::new();
    vis[s] = true;
    q.push(s);
    while !q.empty() {
        let u = q.pop();
        out.push(u);
        for &v in g.neighbors(u) {
            if !vis[v] {
                vis[v] = true; // 入队即标记，避免同一点重复入队
                q.push(v);
            }
        }
    }
    out
}

// 边权全为 1 时第一次到达即最短；带权图要换 Dijkstra。
fn bfs_dist(g: &Graph, s: usize) -> Vec<i32> {
    let mut dist = vec![-1i32; g.n];
    let mut q = RingQueue::new(g.n + 1);
    dist[s] = 0;
    q.push(s);
    while !q.empty() {
        let u = q.pop();
        for &v in g.neighbors(u) {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                q.push(v);
            }
        }
    }
    dist
}

// 按最小顶点从小到大的顺序，给出每个连通块的节点数。
fn component_sizes(g: &Graph) -> Vec<usize> {
    let mut vis = vec![false; g.n];
    let mut sizes = Vec::new();
    for s in 0..g.n {
        if vis[s] {
            continue;
        }
        let mut cnt = 0;
        let mut st = vec![s];
        vis[s] = true;
        while let Some(u) = st.pop() {
            cnt += 1;
            for &v in g.neighbors(u) {
                if !vis[v] {
                    vis[v] = true;
                    st.push(v);
                }
            }
        }
        sizes.push(cnt);
    }
    sizes
}

fn component_count(g: &Graph) -> usize {
    component_sizes(g).len()
}

// 用 BFS 记录 parent，再从终点回溯并反转得到 s 到 t 的路径。
fn shortest_path(g: &Graph, s: usize, t: usize) -> Vec<usize> {
    let mut parent = vec![-1i32; g.n];
    let mut dist = vec![-1i32; g.n];
    let mut q = RingQueue::new(g.n + 1);
    dist[s] = 0;
    q.push(s);
    while !q.empty() {
        let u = q.pop();
        for &v in g.neighbors(u) {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                parent[v] = u as i32;
                q.push(v);
            }
        }
    }
    let mut path = Vec::new();
    if dist[t] == -1 {
        return path; // 不可达返回空
    }
    let mut cur = t;
    while cur != s {
        path.push(cur);
        cur = parent[cur] as usize;
    }
    path.push(s);
    path.reverse();
    path
}

fn all_unique(a: &[usize], n: usize) -> bool {
    let mut seen = vec![false; n];
    for &x in a {
        if seen[x] {
            return false;
        }
        seen[x] = true;
    }
    true
}

fn main() {
    // 1. 单点图：所有接口都要能处理 n = 1
    {
        let g = Graph::new(1);
        assert_eq!(dfs_recursive(&g, 0), vec![0]);
        assert_eq!(dfs_iterative(&g, 0), vec![0]);
        assert_eq!(bfs_order(&g, 0), vec![0]);
        assert_eq!(bfs_dist(&g, 0), vec![0]);
        assert_eq!(component_count(&g), 1);
        assert_eq!(component_sizes(&g), vec![1]);
        assert_eq!(shortest_path(&g, 0, 0), vec![0]);
    }

    // 2. 链 0-1-2-3-4：三种遍历顺序一致，路径可正反还原
    {
        let mut g = Graph::new(5);
        for i in 0..4 {
            g.add_edge(i, i + 1);
        }
        let expect = vec![0, 1, 2, 3, 4];
        assert_eq!(dfs_recursive(&g, 0), expect);
        assert_eq!(dfs_iterative(&g, 0), expect);
        assert_eq!(bfs_order(&g, 0), expect);
        assert_eq!(bfs_dist(&g, 0), vec![0, 1, 2, 3, 4]);
        assert_eq!(shortest_path(&g, 0, 4), expect);
        assert_eq!(shortest_path(&g, 4, 0), vec![4, 3, 2, 1, 0]);
    }

    // 3. 星形：0 连 1..5，BFS 第一层顺序必须按下标升序
    {
        let mut g = Graph::new(6);
        for i in 1..6 {
            g.add_edge(0, i);
        }
        let expect = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(dfs_recursive(&g, 0), expect);
        assert_eq!(dfs_iterative(&g, 0), expect);
        assert_eq!(bfs_order(&g, 0), expect);
        assert_eq!(bfs_dist(&g, 0), vec![0, 1, 1, 1, 1, 1]);
        assert_eq!(shortest_path(&g, 2, 5), vec![2, 0, 5]);
    }

    // 4. 两个连通分量：计数、大小、不可达距离都盯住
    {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(3, 4);
        assert_eq!(component_count(&g), 2);
        assert_eq!(component_sizes(&g), vec![3, 2]);
        assert_eq!(bfs_dist(&g, 0), vec![0, 1, 2, -1, -1]);
        assert!(shortest_path(&g, 0, 4).is_empty());
        assert_eq!(dfs_recursive(&g, 3), vec![3, 4]);
    }

    // 5. 带环图：0-1-2-3-0 再加弦 0-2，BFS 距离不被环绕远
    {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        g.add_edge(0, 2);
        assert_eq!(bfs_order(&g, 0), vec![0, 1, 2, 3]);
        assert_eq!(bfs_dist(&g, 0), vec![0, 1, 1, 1]);
        assert_eq!(shortest_path(&g, 0, 3), vec![0, 3]);
        assert_eq!(component_count(&g), 1);
    }

    // 6. 菱形 0-1、0-2、1-2、1-3、2-3：
    //    若迭代 DFS 出栈才标记，3 会被压两次，顺序长度也会超出 V。
    {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        let it = dfs_iterative(&g, 0);
        assert_eq!(it.len(), 4);
        assert!(all_unique(&it, 4));
        assert_eq!(it[0], 0);
        // 同一份输入重复跑，结果必须完全一致
        assert_eq!(dfs_iterative(&g, 0), it);
        let rec = dfs_recursive(&g, 0);
        assert_eq!(rec.len(), 4);
        assert!(all_unique(&rec, 4));
        assert_eq!(rec, vec![0, 1, 2, 3]);
        assert_eq!(bfs_dist(&g, 0), vec![0, 1, 1, 2]);
    }

    println!("all tests passed");
}
