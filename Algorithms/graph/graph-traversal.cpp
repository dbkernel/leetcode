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
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

// 无向图邻接表：邻居保持升序，遍历结果才可复现。
struct Graph {
    int n;
    vector<vector<int>> adj;

    explicit Graph(int n_) : n(n_), adj(n_) {}

    void insert_sorted(int u, int v) {
        int pos = (int)adj[u].size();
        while (pos > 0 && adj[u][pos - 1] > v) --pos;
        adj[u].insert(adj[u].begin() + pos, v);
    }

    void add_edge(int u, int v) {
        assert(0 <= u && u < n && 0 <= v && v < n);
        insert_sorted(u, v);
        if (u != v) insert_sorted(v, u);  // 自环只存一份即可
    }

    const vector<int>& neighbors(int u) const { return adj[u]; }
};

// 手写环形队列：容量固定，head/tail 取模回绕，cnt 区分空与满。
struct RingQueue {
    vector<int> buf;
    int head, tail, cnt;

    explicit RingQueue(int cap)
        : buf(cap), head(0), tail(0), cnt(0) {}

    bool empty() const { return cnt == 0; }
    int size() const { return cnt; }

    void push(int x) {
        assert(cnt < (int)buf.size());  // 调用方保证不会塞满
        buf[tail] = x;
        tail = (tail + 1) % (int)buf.size();
        ++cnt;
    }

    int pop() {
        assert(cnt > 0);
        int x = buf[head];
        head = (head + 1) % (int)buf.size();
        --cnt;
        return x;
    }
};

static void dfs_visit(const Graph& g, int u, vector<char>& vis,
                      vector<int>& out) {
    vis[u] = 1;
    out.push_back(u);  // 先序位置：进入时记录
    for (int v : g.neighbors(u))
        if (!vis[v]) dfs_visit(g, v, vis, out);
}

vector<int> dfs_recursive(const Graph& g, int s) {
    vector<char> vis(g.n, 0);
    vector<int> out;
    dfs_visit(g, s, vis, out);
    return out;
}

vector<int> dfs_iterative(const Graph& g, int s) {
    vector<char> vis(g.n, 0);
    vector<int> st;  // 用动态数组当栈，不用 std::stack
    vector<int> out;
    vis[s] = 1;
    st.push_back(s);  // 关键：入栈时就标记，不能等出栈
    while (!st.empty()) {
        int u = st.back();
        st.pop_back();
        out.push_back(u);
        // 逆序压栈，弹出时才是邻居下标升序
        for (int i = (int)g.neighbors(u).size() - 1; i >= 0; --i) {
            int v = g.neighbors(u)[i];
            if (!vis[v]) {
                vis[v] = 1;
                st.push_back(v);
            }
        }
    }
    return out;
}

vector<int> bfs_order(const Graph& g, int s) {
    vector<char> vis(g.n, 0);
    RingQueue q(g.n + 1);
    vector<int> out;
    vis[s] = 1;
    q.push(s);
    while (!q.empty()) {
        int u = q.pop();
        out.push_back(u);
        for (int v : g.neighbors(u)) {
            if (!vis[v]) {
                vis[v] = 1;  // 入队即标记，避免同一点重复入队
                q.push(v);
            }
        }
    }
    return out;
}

// 边权全为 1 时第一次到达即最短；带权图要换 Dijkstra。
vector<int> bfs_dist(const Graph& g, int s) {
    vector<int> dist(g.n, -1);
    RingQueue q(g.n + 1);
    dist[s] = 0;
    q.push(s);
    while (!q.empty()) {
        int u = q.pop();
        for (int v : g.neighbors(u)) {
            if (dist[v] == -1) {
                dist[v] = dist[u] + 1;
                q.push(v);
            }
        }
    }
    return dist;
}

// 按最小顶点从小到大的顺序，给出每个连通块的节点数。
vector<int> component_sizes(const Graph& g) {
    vector<char> vis(g.n, 0);
    vector<int> sizes;
    for (int s = 0; s < g.n; ++s) {
        if (vis[s]) continue;
        int cnt = 0;
        vector<int> st;
        vis[s] = 1;
        st.push_back(s);
        while (!st.empty()) {
            int u = st.back();
            st.pop_back();
            ++cnt;
            for (int v : g.neighbors(u))
                if (!vis[v]) {
                    vis[v] = 1;
                    st.push_back(v);
                }
        }
        sizes.push_back(cnt);
    }
    return sizes;
}

int component_count(const Graph& g) {
    return (int)component_sizes(g).size();
}

// BFS 记录 parent，再从终点回溯，最后整体反转得到 s 到 t 的路径。
vector<int> shortest_path(const Graph& g, int s, int t) {
    vector<int> parent(g.n, -1);
    vector<int> dist(g.n, -1);
    RingQueue q(g.n + 1);
    dist[s] = 0;
    q.push(s);
    while (!q.empty()) {
        int u = q.pop();
        for (int v : g.neighbors(u)) {
            if (dist[v] == -1) {
                dist[v] = dist[u] + 1;
                parent[v] = u;
                q.push(v);
            }
        }
    }
    vector<int> path;
    if (dist[t] == -1) return path;  // 不可达返回空
    int cur = t;
    while (cur != s) {
        path.push_back(cur);
        cur = parent[cur];
    }
    path.push_back(s);
    for (int i = 0, j = (int)path.size() - 1; i < j; ++i, --j) {
        int tmp = path[i];
        path[i] = path[j];
        path[j] = tmp;
    }
    return path;
}

static bool all_unique(const vector<int>& a, int n) {
    vector<char> seen(n, 0);
    for (int x : a) {
        if (seen[x]) return false;
        seen[x] = 1;
    }
    return true;
}

int main() {
    // 1. 单点图：所有接口都要能处理 n = 1
    {
        Graph g(1);
        assert(dfs_recursive(g, 0) == (vector<int>{0}));
        assert(dfs_iterative(g, 0) == (vector<int>{0}));
        assert(bfs_order(g, 0) == (vector<int>{0}));
        assert(bfs_dist(g, 0) == (vector<int>{0}));
        assert(component_count(g) == 1);
        assert(component_sizes(g) == (vector<int>{1}));
        assert(shortest_path(g, 0, 0) == (vector<int>{0}));
    }

    // 2. 链 0-1-2-3-4：三种遍历顺序一致，路径可正反还原
    {
        Graph g(5);
        for (int i = 0; i + 1 < 5; ++i) g.add_edge(i, i + 1);
        vector<int> expect = {0, 1, 2, 3, 4};
        assert(dfs_recursive(g, 0) == expect);
        assert(dfs_iterative(g, 0) == expect);
        assert(bfs_order(g, 0) == expect);
        assert(bfs_dist(g, 0) == (vector<int>{0, 1, 2, 3, 4}));
        assert(shortest_path(g, 0, 4) == expect);
        assert(shortest_path(g, 4, 0) == (vector<int>{4, 3, 2, 1, 0}));
    }

    // 3. 星形：0 连 1..5，BFS 第一层顺序必须按下标升序
    {
        Graph g(6);
        for (int i = 1; i < 6; ++i) g.add_edge(0, i);
        vector<int> expect = {0, 1, 2, 3, 4, 5};
        assert(dfs_recursive(g, 0) == expect);
        assert(dfs_iterative(g, 0) == expect);
        assert(bfs_order(g, 0) == expect);
        assert(bfs_dist(g, 0) == (vector<int>{0, 1, 1, 1, 1, 1}));
        assert(shortest_path(g, 2, 5) == (vector<int>{2, 0, 5}));
    }

    // 4. 两个连通分量：计数、大小、不可达距离都盯住
    {
        Graph g(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(3, 4);
        assert(component_count(g) == 2);
        assert(component_sizes(g) == (vector<int>{3, 2}));
        assert(bfs_dist(g, 0) == (vector<int>{0, 1, 2, -1, -1}));
        assert(shortest_path(g, 0, 4).empty());
        assert(dfs_recursive(g, 3) == (vector<int>{3, 4}));
    }

    // 5. 带环图：0-1-2-3-0 再加弦 0-2，BFS 距离不被环绕远
    {
        Graph g(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        g.add_edge(0, 2);
        assert(bfs_order(g, 0) == (vector<int>{0, 1, 2, 3}));
        assert(bfs_dist(g, 0) == (vector<int>{0, 1, 1, 1}));
        assert(shortest_path(g, 0, 3) == (vector<int>{0, 3}));
        assert(component_count(g) == 1);
    }

    // 6. 菱形 0-1、0-2、1-2、1-3、2-3：
    //    若迭代 DFS 出栈才标记，3 会被压两次，顺序长度也会超出 V。
    {
        Graph g(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        vector<int> it = dfs_iterative(g, 0);
        assert((int)it.size() == 4);
        assert(all_unique(it, 4));
        assert(it.front() == 0);
        // 同一份输入重复跑，结果必须完全一致
        assert(dfs_iterative(g, 0) == it);
        vector<int> rec = dfs_recursive(g, 0);
        assert((int)rec.size() == 4);
        assert(all_unique(rec, 4));
        assert(rec == (vector<int>{0, 1, 2, 3}));
        assert(bfs_dist(g, 0) == (vector<int>{0, 1, 1, 2}));
    }

    cout << "all tests passed" << endl;
    return 0;
}
