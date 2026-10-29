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
#![allow(dead_code)]

// 有向图邻接表：邻居保持升序，输出的拓扑序才可复现。
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
        self.insert_sorted(u, v); // 有向图只加单向边
    }

    fn neighbors(&self, u: usize) -> &Vec<usize> {
        &self.adj[u]
    }
}

// 手写环形队列：Kahn 里每个点最多入队一次，容量 V+1 足够。
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
        assert!(self.cnt < self.buf.len());
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

// 入度归零才入队；有环时会有顶点永远进不了队列。
fn kahn(g: &Graph) -> (bool, Vec<usize>) {
    let mut indeg = vec![0usize; g.n];
    for u in 0..g.n {
        for &v in g.neighbors(u) {
            indeg[v] += 1; // 自环也计入
        }
    }
    let mut q = RingQueue::new(g.n + 1);
    for u in 0..g.n {
        if indeg[u] == 0 {
            q.push(u);
        }
    }
    let mut order = Vec::new();
    while !q.empty() {
        let u = q.pop();
        order.push(u);
        for &v in g.neighbors(u) {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push(v);
            }
        }
    }
    // 输出不足 V 个点，说明剩下的点互相牵制，必有环
    (order.len() == g.n, order)
}

fn dfs_visit(g: &Graph, u: usize, color: &mut Vec<u8>,
             order: &mut Vec<usize>) -> bool {
    color[u] = 1; // 1 = 在递归栈上
    for &v in g.neighbors(u) {
        if color[v] == 1 {
            return false; // 回边，成环
        }
        if color[v] == 0 && !dfs_visit(g, v, color, order) {
            return false;
        }
    }
    color[u] = 2; // 2 = 完成，后序加入
    order.push(u);
    true
}

// 后序是「出度为 0 先完成」，整体反转才是拓扑序。
fn topo_dfs(g: &Graph) -> (bool, Vec<usize>) {
    let mut color = vec![0u8; g.n]; // 0 未访问 / 1 在栈上 / 2 完成
    let mut order = Vec::new();
    for u in 0..g.n {
        if color[u] == 0 && !dfs_visit(g, u, &mut color, &mut order) {
            return (false, order);
        }
    }
    order.reverse();
    (true, order)
}

// 合法性校验：每个点的所有前驱都必须排在它前面。
fn is_valid_order(g: &Graph, order: &[usize]) -> bool {
    if order.len() != g.n {
        return false;
    }
    let mut pos = vec![-1i32; g.n];
    for (i, &u) in order.iter().enumerate() {
        if u >= g.n || pos[u] != -1 {
            return false;
        }
        pos[u] = i as i32;
    }
    for u in 0..g.n {
        for &v in g.neighbors(u) {
            if pos[u] >= pos[v] {
                return false; // 边 u->v 被排反
            }
        }
    }
    true
}

// 一门课的先修关系：before 必须排在 after 之前。
struct Dep {
    before: usize,
    after: usize,
}

fn course_schedule(n: usize, deps: &[Dep]) -> (bool, Vec<usize>) {
    let mut g = Graph::new(n);
    for d in deps {
        g.add_edge(d.before, d.after);
    }
    let (ok, order) = kahn(&g);
    (ok && is_valid_order(&g, &order), order)
}

fn main() {
    // 1. 单点：两个版本都必须给出 [0]
    {
        let g = Graph::new(1);
        let (ok, a) = kahn(&g);
        let (ok2, b) = topo_dfs(&g);
        assert!(ok && ok2);
        assert_eq!(a, vec![0]);
        assert_eq!(b, vec![0]);
        assert!(is_valid_order(&g, &a));
        assert!(is_valid_order(&g, &b));
    }

    // 2. 链 0->1->2->3->4：拓扑序唯一，两个版本应完全一致
    {
        let mut g = Graph::new(5);
        for i in 0..4 {
            g.add_edge(i, i + 1);
        }
        let (ok, a) = kahn(&g);
        let (ok2, b) = topo_dfs(&g);
        assert!(ok && ok2);
        let expect = vec![0, 1, 2, 3, 4];
        assert_eq!(a, expect);
        assert_eq!(b, expect);
    }

    // 3. 多个合法序：0->2、1->2、2->3，只校验合法性不比对序列
    {
        let mut g = Graph::new(4);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        let (ok, a) = kahn(&g);
        let (ok2, b) = topo_dfs(&g);
        assert!(ok && ok2);
        assert_ne!(a, b); // 两个算法确实给出了不同的合法序
        assert!(is_valid_order(&g, &a));
        assert!(is_valid_order(&g, &b));
        assert_eq!(a.len(), 4);
        assert_eq!(b.len(), 4);
    }

    // 4. 有环图：两个版本都要返回失败
    {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        let (ok, a) = kahn(&g);
        let (ok2, _) = topo_dfs(&g);
        assert!(!ok && !ok2);
        assert!(a.len() < 3); // Kahn 用长度暴露环
    }

    // 5. 环不在 0 号点：1<->2，0 是挂在旁边的无环入口
    {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 1);
        let (ok, _) = kahn(&g);
        let (ok2, _) = topo_dfs(&g);
        assert!(!ok && !ok2);
    }

    // 6. 有向自环：自己指向自己也是环，两个版本都不能放过
    {
        let mut g = Graph::new(2);
        g.add_edge(0, 0);
        g.add_edge(1, 0);
        let (ok, _) = kahn(&g);
        let (ok2, _) = topo_dfs(&g);
        assert!(!ok && !ok2);
    }

    // 7. 孤立点 + 一条边：全部顶点都要出现，顺序仍要合法
    {
        let mut g = Graph::new(4);
        g.add_edge(1, 2);
        let (ok, a) = kahn(&g);
        let (ok2, b) = topo_dfs(&g);
        assert!(ok && ok2);
        assert_eq!(a.len(), 4);
        assert_eq!(b.len(), 4);
        assert!(is_valid_order(&g, &a));
        assert!(is_valid_order(&g, &b));
    }

    // 8. 应用：6 门课的先修关系，输出必须满足每一条依赖
    {
        let deps = vec![
            Dep { before: 0, after: 1 },
            Dep { before: 0, after: 2 },
            Dep { before: 1, after: 3 },
            Dep { before: 2, after: 3 },
            Dep { before: 3, after: 4 },
            Dep { before: 3, after: 5 },
        ];
        let (ok, order) = course_schedule(6, &deps);
        assert!(ok);
        let mut g = Graph::new(6);
        for d in &deps {
            g.add_edge(d.before, d.after);
        }
        assert!(is_valid_order(&g, &order));
        // 再逐条核对依赖，避免校验函数本身写错
        let mut pos = vec![0usize; 6];
        for (i, &u) in order.iter().enumerate() {
            pos[u] = i;
        }
        for d in &deps {
            assert!(pos[d.before] < pos[d.after]);
        }
    }

    // 9. 应用：课程之间成环，必须报告无法修完
    {
        let deps = vec![
            Dep { before: 0, after: 1 },
            Dep { before: 1, after: 2 },
            Dep { before: 2, after: 0 },
        ];
        let (ok, _) = course_schedule(3, &deps);
        assert!(!ok);
    }

    println!("all tests passed");
}
