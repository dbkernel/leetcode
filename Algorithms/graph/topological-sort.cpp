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
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

// 有向图邻接表：邻居保持升序，输出的拓扑序才可复现。
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
        insert_sorted(u, v);  // 有向图只加单向边
    }

    const vector<int>& neighbors(int u) const { return adj[u]; }
};

// 手写环形队列：Kahn 里每个点最多入队一次，容量 V+1 足够。
struct RingQueue {
    vector<int> buf;
    int head, tail, cnt;

    explicit RingQueue(int cap)
        : buf(cap), head(0), tail(0), cnt(0) {}

    bool empty() const { return cnt == 0; }

    void push(int x) {
        assert(cnt < (int)buf.size());
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

// Kahn：入度归零才入队；有环时会有顶点永远进不了队列。
bool kahn(const Graph& g, vector<int>& order) {
    vector<int> indeg(g.n, 0);
    for (int u = 0; u < g.n; ++u)
        for (int v : g.neighbors(u)) ++indeg[v];  // 自环也计入
    RingQueue q(g.n + 1);
    for (int u = 0; u < g.n; ++u)
        if (indeg[u] == 0) q.push(u);
    order.clear();
    while (!q.empty()) {
        int u = q.pop();
        order.push_back(u);
        for (int v : g.neighbors(u)) {
            if (--indeg[v] == 0) q.push(v);
        }
    }
    // 输出不足 V 个点，说明剩下的点互相牵制，必有环
    return (int)order.size() == g.n;
}

static bool dfs_visit(const Graph& g, int u, vector<int>& color,
                      vector<int>& order) {
    color[u] = 1;  // 1 = 在递归栈上
    for (int v : g.neighbors(u)) {
        if (color[v] == 1) return false;  // 回边，成环
        if (color[v] == 0 && !dfs_visit(g, v, color, order)) return false;
    }
    color[u] = 2;  // 2 = 完成，后序加入
    order.push_back(u);
    return true;
}

// DFS：后序是「出度为 0 先完成」，整体反转才是拓扑序。
bool topo_dfs(const Graph& g, vector<int>& order) {
    vector<int> color(g.n, 0);  // 0 未访问 / 1 在栈上 / 2 完成
    order.clear();
    for (int u = 0; u < g.n; ++u) {
        if (color[u] == 0 && !dfs_visit(g, u, color, order))
            return false;
    }
    for (int i = 0, j = (int)order.size() - 1; i < j; ++i, --j) {
        int tmp = order[i];
        order[i] = order[j];
        order[j] = tmp;
    }
    return true;
}

// 合法性校验：每个点的所有前驱都必须排在它前面。
bool is_valid_order(const Graph& g, const vector<int>& order) {
    if ((int)order.size() != g.n) return false;
    vector<int> pos(g.n, -1);
    for (int i = 0; i < (int)order.size(); ++i) {
        int u = order[i];
        if (u < 0 || u >= g.n || pos[u] != -1) return false;
        pos[u] = i;
    }
    for (int u = 0; u < g.n; ++u)
        for (int v : g.neighbors(u))
            if (pos[u] >= pos[v]) return false;  // 边 u->v 被排反
    return true;
}

// 应用：一门课的所有先修课都要求排在它前面。
struct Dep {
    int before;
    int after;
};

bool course_schedule(int n, const vector<Dep>& deps, vector<int>& order) {
    Graph g(n);
    for (const Dep& d : deps) g.add_edge(d.before, d.after);
    return kahn(g, order) && is_valid_order(g, order);
}

int main() {
    // 1. 单点：两个版本都必须给出 [0]
    {
        Graph g(1);
        vector<int> a, b;
        assert(kahn(g, a));
        assert(topo_dfs(g, b));
        assert(a == (vector<int>{0}));
        assert(b == (vector<int>{0}));
        assert(is_valid_order(g, a) && is_valid_order(g, b));
    }

    // 2. 链 0->1->2->3->4：拓扑序唯一，两个版本应完全一致
    {
        Graph g(5);
        for (int i = 0; i + 1 < 5; ++i) g.add_edge(i, i + 1);
        vector<int> a, b;
        assert(kahn(g, a));
        assert(topo_dfs(g, b));
        vector<int> expect = {0, 1, 2, 3, 4};
        assert(a == expect);
        assert(b == expect);
        assert(is_valid_order(g, a) && is_valid_order(g, b));
    }

    // 3. 多个合法序：0->2、1->2、2->3，只校验合法性不比对序列
    {
        Graph g(4);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        vector<int> a, b;
        assert(kahn(g, a));
        assert(topo_dfs(g, b));
        assert(a != b);  // 两个算法确实给出了不同的合法序
        assert(is_valid_order(g, a));
        assert(is_valid_order(g, b));
        assert((int)a.size() == 4 && (int)b.size() == 4);
    }

    // 4. 有环图：两个版本都要返回失败
    {
        Graph g(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        vector<int> a, b;
        assert(!kahn(g, a));
        assert(!topo_dfs(g, b));
        assert((int)a.size() < 3);  // Kahn 用长度暴露环
    }

    // 5. 环不在 0 号点：1<->2，0 是挂在旁边的无环入口
    {
        Graph g(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 1);
        vector<int> a, b;
        assert(!kahn(g, a));
        assert(!topo_dfs(g, b));
    }

    // 6. 有向自环：自己指向自己也是环，两个版本都不能放过
    {
        Graph g(2);
        g.add_edge(0, 0);
        g.add_edge(1, 0);
        vector<int> a, b;
        assert(!kahn(g, a));
        assert(!topo_dfs(g, b));
    }

    // 7. 孤立点 + 一条边：全部顶点都要出现，顺序仍要合法
    {
        Graph g(4);
        g.add_edge(1, 2);
        vector<int> a, b;
        assert(kahn(g, a));
        assert(topo_dfs(g, b));
        assert((int)a.size() == 4);
        assert((int)b.size() == 4);
        assert(is_valid_order(g, a));
        assert(is_valid_order(g, b));
    }

    // 8. 应用：6 门课的先修关系，输出必须满足每一条依赖
    {
        vector<Dep> deps = {{0, 1}, {0, 2}, {1, 3}, {2, 3}, {3, 4}, {3, 5}};
        vector<int> order;
        assert(course_schedule(6, deps, order));
        Graph g(6);
        for (const Dep& d : deps) g.add_edge(d.before, d.after);
        assert(is_valid_order(g, order));
        // 再逐条核对依赖，避免校验函数本身写错
        vector<int> pos(6, -1);
        for (int i = 0; i < 6; ++i) pos[order[i]] = i;
        for (const Dep& d : deps) assert(pos[d.before] < pos[d.after]);
    }

    // 9. 应用：课程之间成环，必须报告无法修完
    {
        vector<Dep> deps = {{0, 1}, {1, 2}, {2, 0}};
        vector<int> order;
        assert(!course_schedule(3, deps, order));
    }

    cout << "all tests passed" << endl;
    return 0;
}
