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
#include <cassert>
#include <iostream>
#include <utility>
#include <vector>
using namespace std;

// 邻接矩阵：m[u * n + v] 非 0 表示有边 u->v。
// 用一维数组模拟二维，避免 vector 嵌套多一次指针跳转。
struct GraphMatrix {
    int n;
    bool directed;
    vector<char> m;
    int m_edges;

    GraphMatrix(int n_, bool directed_)
        : n(n_), directed(directed_), m(n_ * n_, 0), m_edges(0) {}

    bool has_edge(int u, int v) const {
        assert(0 <= u && u < n && 0 <= v && v < n);
        return m[u * n + v] != 0;
    }

    void add_edge(int u, int v) {
        assert(0 <= u && u < n && 0 <= v && v < n);
        if (has_edge(u, v)) return;  // 重边按集合语义忽略
        m[u * n + v] = 1;
        ++m_edges;
        // 无向图要对称置位；自环时 u == v，同一格只置一次
        if (!directed && u != v) m[v * n + u] = 1;
    }

    vector<int> neighbors(int u) const {
        assert(0 <= u && u < n);
        vector<int> res;
        for (int v = 0; v < n; ++v)
            if (m[u * n + v]) res.push_back(v);
        return res;
    }

    // 无向图的度：自环在同一行只出现一次，但图论上算 2 度
    int degree(int u) const {
        assert(!directed && 0 <= u && u < n);
        int d = 0;
        for (int v = 0; v < n; ++v)
            if (m[u * n + v]) d += (u == v ? 2 : 1);
        return d;
    }

    int out_degree(int u) const {
        assert(directed && 0 <= u && u < n);
        int d = 0;
        for (int v = 0; v < n; ++v)
            if (m[u * n + v]) ++d;
        return d;
    }

    int in_degree(int u) const {
        assert(directed && 0 <= u && u < n);
        int d = 0;
        for (int v = 0; v < n; ++v)
            if (m[v * n + u]) ++d;
        return d;
    }

    int edge_count() const { return m_edges; }

    // 规范边集：无向图只取上三角，自环只留一次；有向图全扫
    vector<pair<int, int>> edge_list() const {
        vector<pair<int, int>> res;
        for (int u = 0; u < n; ++u)
            for (int v = (directed ? 0 : u); v < n; ++v)
                if (m[u * n + v]) res.push_back({u, v});
        return res;
    }
};

// 邻接表：adj[u] 存 u 的出边终点，保持升序，遍历顺序可复现。
struct GraphList {
    int n;
    bool directed;
    vector<vector<int>> adj;
    vector<int> indeg;  // 有向图入度，加边时维护，查询 O(1)
    int m_edges;

    GraphList(int n_, bool directed_)
        : n(n_), directed(directed_), adj(n_), indeg(n_, 0), m_edges(0) {}

    bool has_edge(int u, int v) const {
        assert(0 <= u && u < n && 0 <= v && v < n);
        for (int x : adj[u])
            if (x == v) return true;
        return false;
    }

    // 从尾部找插入点再整体右移，复杂度 O(degree)，不用库排序
    void insert_sorted(int u, int v) {
        int pos = (int)adj[u].size();
        while (pos > 0 && adj[u][pos - 1] > v) --pos;
        adj[u].insert(adj[u].begin() + pos, v);
    }

    void add_edge(int u, int v) {
        assert(0 <= u && u < n && 0 <= v && v < n);
        if (has_edge(u, v)) return;  // 重边按集合语义忽略
        insert_sorted(u, v);
        if (directed) {
            ++indeg[v];
        } else if (u != v) {
            insert_sorted(v, u);
        } else {
            // 无向自环存两份，使 degree(u) 自然等于 2
            insert_sorted(u, u);
        }
        ++m_edges;
    }

    vector<int> neighbors(int u) const {
        assert(0 <= u && u < n);
        return adj[u];
    }

    int degree(int u) const {
        assert(!directed && 0 <= u && u < n);
        return (int)adj[u].size();
    }

    int out_degree(int u) const {
        assert(directed && 0 <= u && u < n);
        return (int)adj[u].size();
    }

    int in_degree(int u) const {
        assert(directed && 0 <= u && u < n);
        return indeg[u];
    }

    int edge_count() const { return m_edges; }

    vector<pair<int, int>> edge_list() const {
        vector<pair<int, int>> res;
        for (int u = 0; u < n; ++u) {
            for (int v : adj[u]) {
                if (directed) {
                    res.push_back({u, v});
                } else if (u < v) {
                    res.push_back({u, v});
                } else if (u == v) {
                    // 自环存了两份，只保留第一条，否则边集多算一次
                    if (res.empty() || res.back() != make_pair(u, v))
                        res.push_back({u, v});
                }
            }
        }
        return res;
    }
};

// 同一张图分别用两种存储建一遍，逐项对齐。
static void check_same_graph(const GraphMatrix& gm, const GraphList& gl) {
    assert(gm.n == gl.n);
    assert(gm.directed == gl.directed);
    assert(gm.edge_count() == gl.edge_count());
    assert(gm.edge_list() == gl.edge_list());
    for (int u = 0; u < gm.n; ++u)
        for (int v = 0; v < gm.n; ++v)
            assert(gm.has_edge(u, v) == gl.has_edge(u, v));
    if (gm.directed) {
        int out_sum = 0, in_sum = 0;
        for (int u = 0; u < gm.n; ++u) {
            assert(gm.out_degree(u) == gl.out_degree(u));
            assert(gm.in_degree(u) == gl.in_degree(u));
            out_sum += gm.out_degree(u);
            in_sum += gm.in_degree(u);
        }
        // 有向图不变量：出度之和 = 入度之和 = E
        assert(out_sum == gm.edge_count());
        assert(in_sum == gm.edge_count());
    } else {
        int deg_sum = 0;
        for (int u = 0; u < gm.n; ++u) {
            assert(gm.degree(u) == gl.degree(u));
            deg_sum += gm.degree(u);
        }
        // 无向图不变量：度数和 = 2E；自环算 2 度正是为了这条成立
        assert(deg_sum == 2 * gm.edge_count());
    }
}

int main() {
    // 1. 空图：任何查询都不能越界，边数必须是 0
    {
        GraphMatrix gm(4, false);
        GraphList gl(4, false);
        check_same_graph(gm, gl);
        assert(gm.edge_count() == 0);
        for (int u = 0; u < 4; ++u) {
            assert(gm.degree(u) == 0);
            assert(gm.neighbors(u).empty());
            assert(gl.neighbors(u).empty());
        }
        assert(!gm.has_edge(0, 1));
    }

    // 2. 无向链 0-1-2，并重复加 (0,1) 验证重边不改变结构
    {
        GraphMatrix gm(3, false);
        GraphList gl(3, false);
        int es[2][2] = {{0, 1}, {1, 2}};
        for (auto& e : es) {
            gm.add_edge(e[0], e[1]);
            gl.add_edge(e[0], e[1]);
        }
        gm.add_edge(0, 1);
        gl.add_edge(0, 1);
        check_same_graph(gm, gl);
        assert(gm.edge_count() == 2);
        assert(gm.degree(1) == 2);
        assert(gm.neighbors(1) == (vector<int>{0, 2}));
        assert(gl.neighbors(1) == (vector<int>{0, 2}));
    }

    // 3. 无向自环：矩阵只有一格，邻接表存两份，度都必须是 2
    {
        GraphMatrix gm(3, false);
        GraphList gl(3, false);
        gm.add_edge(0, 0);
        gl.add_edge(0, 0);
        gm.add_edge(0, 0);  // 再次加同一条自环仍是重边
        gl.add_edge(0, 0);
        check_same_graph(gm, gl);
        assert(gm.edge_count() == 1);
        assert(gm.degree(0) == 2);
        assert(gl.degree(0) == 2);
        assert(gm.has_edge(0, 0));
        assert(gm.neighbors(0) == (vector<int>{0}));
        assert(gl.neighbors(0) == (vector<int>{0, 0}));
        int sum = 0;
        for (int u = 0; u < 3; ++u) sum += gm.degree(u);
        assert(sum == 2 * gm.edge_count());
    }

    // 4. 有向图含自环：入度、出度分别核对，并保持两个和不变
    {
        GraphMatrix gm(5, true);
        GraphList gl(5, true);
        int es[6][2] = {{0, 1}, {0, 2}, {1, 3}, {2, 3}, {3, 4}, {3, 3}};
        for (auto& e : es) {
            gm.add_edge(e[0], e[1]);
            gl.add_edge(e[0], e[1]);
        }
        check_same_graph(gm, gl);
        assert(gm.edge_count() == 6);
        assert(gm.out_degree(3) == 2);
        assert(gm.in_degree(3) == 3);  // 来自 1、2 以及 3 的自环
        assert(gm.in_degree(4) == 1);
        assert(gm.has_edge(3, 3));
        assert(gm.neighbors(3) == (vector<int>{3, 4}));
        assert(gl.neighbors(3) == (vector<int>{3, 4}));
    }

    // 5. 稀疏图：10 点 7 边，两条存储的边集完全一致
    {
        GraphMatrix gm(10, false);
        GraphList gl(10, false);
        for (int i = 0; i + 3 < 10; ++i) {
            gm.add_edge(i, i + 3);
            gl.add_edge(i, i + 3);
        }
        check_same_graph(gm, gl);
        assert(gm.edge_count() == 7);
        assert(gm.degree(0) == 1);
    }

    // 6. 稠密图：7 点完全图，E = 21，每个点度数为 6
    {
        GraphMatrix gm(7, false);
        GraphList gl(7, false);
        for (int u = 0; u < 7; ++u)
            for (int v = u + 1; v < 7; ++v) {
                gm.add_edge(u, v);
                gl.add_edge(u, v);
            }
        check_same_graph(gm, gl);
        assert(gm.edge_count() == 21);
        for (int u = 0; u < 7; ++u) assert(gm.degree(u) == 6);
    }

    cout << "all tests passed" << endl;
    return 0;
}
