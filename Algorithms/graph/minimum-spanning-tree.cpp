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
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <vector>

const int INF = 1000000000;

struct Edge {
    int u;
    int v;
    int w;
};

struct MstResult {
    int total;                // 选中边的权重和
    std::vector<Edge> edges;  // 实际选中的边，条数 = V - components
    int components;           // 原图的连通分量数
};

// 手写自底向上归并排序，按 w 升序；稳定、无递归、空间 O(E)。
static void sortEdges(std::vector<Edge> &es) {
    int n = static_cast<int>(es.size());
    if (n < 2) return;
    std::vector<Edge> buf(static_cast<size_t>(n));
    std::vector<Edge> *src = &es;
    std::vector<Edge> *dst = &buf;
    for (int width = 1; width < n; width *= 2) {
        for (int lo = 0; lo < n; lo += 2 * width) {
            int mid = lo + width;
            int hi = lo + 2 * width;
            if (mid > n) mid = n;
            if (hi > n) hi = n;
            int i = lo;
            int j = mid;
            int k = lo;
            while (i < mid && j < hi) {
                // 相等时先取左边，保证稳定（虽然 MST 不依赖稳定性）。
                if ((*src)[j].w < (*src)[i].w) {
                    (*dst)[k++] = (*src)[j++];
                } else {
                    (*dst)[k++] = (*src)[i++];
                }
            }
            while (i < mid) (*dst)[k++] = (*src)[i++];
            while (j < hi) (*dst)[k++] = (*src)[j++];
        }
        std::swap(src, dst);
    }
    // 经过奇数次交换后结果落在 buf 里，搬回调用方的数组。
    if (src != &es) es = *src;
}

// Kruskal 专用并查集：两趟路径压缩 + 按大小合并，两者缺一不可。
struct Dsu {
    std::vector<int> parent;
    std::vector<int> size;

    explicit Dsu(int n) : parent(static_cast<size_t>(n)),
                          size(static_cast<size_t>(n), 1) {
        for (int i = 0; i < n; ++i) parent[i] = i;
    }

    int find(int x) {
        int root = x;
        while (parent[root] != root) root = parent[root];
        while (parent[x] != root) {  // 第二趟：路径上所有点直挂根
            int next = parent[x];
            parent[x] = root;
            x = next;
        }
        return root;
    }

    // 返回 false 表示两端本来就在同一个连通块，这条边会成环。
    bool unite(int a, int b) {
        int ra = find(a);
        int rb = find(b);
        if (ra == rb) return false;
        if (size[ra] < size[rb]) std::swap(ra, rb);
        parent[rb] = ra;
        size[ra] += size[rb];
        return true;
    }
};

// Kruskal：排序 + 并查集，O(E log E)。
MstResult kruskal(int n, std::vector<Edge> edges) {
    sortEdges(edges);
    Dsu dsu(n);
    MstResult res{0, {}, n};
    for (const Edge &e : edges) {
        if (!dsu.unite(e.u, e.v)) continue;  // 成环，跳过
        res.total += e.w;
        res.edges.push_back(e);
        --res.components;
    }
    return res;
}

// Prim 朴素版：从 0 号点出发，每轮线性扫描 O(V) 选最近的外部点，总 O(V^2)。
// 图不连通时外层继续找未访问点，于是得到的是最小生成森林。
MstResult primNaive(int n, const std::vector<Edge> &edges) {
    // 邻接矩阵，重边只留最小权重；O(V^2) 空间对朴素版可以接受。
    std::vector<std::vector<int>> w(
        static_cast<size_t>(n),
        std::vector<int>(static_cast<size_t>(n), INF));
    for (const Edge &e : edges) {
        if (e.w < w[e.u][e.v]) {
            w[e.u][e.v] = e.w;
            w[e.v][e.u] = e.w;  // 无向图，两个方向都记
        }
    }
    std::vector<int> best(static_cast<size_t>(n), INF);  // 树到该点的最近边
    std::vector<int> from(static_cast<size_t>(n), -1);   // 那条边从哪个树内点来
    std::vector<char> in(static_cast<size_t>(n), 0);     // 是否已进树
    MstResult res{0, {}, 0};
    for (int s = 0; s < n; ++s) {
        if (in[s]) continue;
        ++res.components;
        best[s] = 0;  // 起点不需要边连进树
        for (int round = 0; round < n; ++round) {
            int u = -1;
            for (int v = 0; v < n; ++v) {
                if (in[v] || best[v] == INF) continue;
                if (u == -1 || best[v] < best[u]) u = v;
            }
            if (u == -1) break;  // 本分量吸完了
            in[u] = 1;
            if (from[u] != -1) {
                res.total += best[u];
                res.edges.push_back(Edge{from[u], u, best[u]});
            }
            for (int v = 0; v < n; ++v) {
                if (!in[v] && w[u][v] < best[v]) {
                    best[v] = w[u][v];
                    from[v] = u;
                }
            }
        }
    }
    return res;
}

// ---------------------------------------------------------------------------
// 以下均为自测代码。
// ---------------------------------------------------------------------------

// 固定种子的 64 位 LCG：四语言使用同一套参数，失败可复现。
struct Rng {
    unsigned long long s;
    explicit Rng(unsigned long long seed) : s(seed) {}
    unsigned next() {
        s = s * 6364136223846793005ULL + 1442695040888963407ULL;
        return static_cast<unsigned>(s >> 33);
    }
    int in(int lo, int hi) {
        unsigned span = static_cast<unsigned>(hi - lo + 1);
        return lo + static_cast<int>(next() % span);
    }
};

static bool sameEdge(const Edge &a, const Edge &b) {
    return (a.u == b.u && a.v == b.v) || (a.u == b.v && a.v == b.u);
}

static void printEdges(const std::vector<Edge> &es) {
    std::cout << "[";
    for (size_t i = 0; i < es.size(); ++i) {
        if (i != 0) std::cout << ", ";
        std::cout << "(" << es[i].u << "," << es[i].v << "," << es[i].w << ")";
    }
    std::cout << "]";
}

// 独立算一遍连通分量数，作为 components 字段的参照。
static int countComps(int n, const std::vector<Edge> &edges) {
    Dsu dsu(n);
    for (const Edge &e : edges) dsu.unite(e.u, e.v);
    int c = 0;
    for (int i = 0; i < n; ++i) {
        if (dsu.find(i) == i) ++c;
    }
    return c;
}

// 校验选中的边：都来自输入、彼此不成环、权重和等于 total。
static void checkForest(int n, const MstResult &r, const char *tag,
                        const std::vector<Edge> &input) {
    Dsu dsu(n);
    int sum = 0;
    for (const Edge &e : r.edges) {
        bool found = false;
        for (const Edge &src : input) {
            if (src.w == e.w && sameEdge(e, src)) found = true;
        }
        if (!found) {
            std::cout << "case " << tag << " picked phantom edge ";
            printEdges({e});
            std::cout << std::endl;
            std::abort();
        }
        if (!dsu.unite(e.u, e.v)) {
            std::cout << "case " << tag << " contains a cycle" << std::endl;
            std::abort();
        }
        sum += e.w;
    }
    assert(sum == r.total);
}

// 两个版本必须总权重相同、分量数相同、边数都等于 V - comps。
static void checkConsistent(int n, const std::vector<Edge> &es) {
    int comps = countComps(n, es);
    MstResult k = kruskal(n, es);
    MstResult p = primNaive(n, es);
    if (k.total != p.total) {
        std::cout << "kruskal total " << k.total << " != prim total "
                  << p.total << std::endl;
        std::cout << "  kruskal edges ";
        printEdges(k.edges);
        std::cout << "\n  prim edges    ";
        printEdges(p.edges);
        std::cout << std::endl;
        std::abort();
    }
    assert(k.components == comps);
    assert(p.components == comps);
    assert(static_cast<int>(k.edges.size()) == n - comps);
    assert(static_cast<int>(p.edges.size()) == n - comps);
    checkForest(n, k, "kruskal", es);
    checkForest(n, p, "prim", es);
}

static void checkMst(const char *name, int n, const std::vector<Edge> &es,
                     int wantTotal, int wantComps) {
    checkConsistent(n, es);
    MstResult k = kruskal(n, es);
    if (k.total != wantTotal || k.components != wantComps) {
        std::cout << "case " << name << " failed: total " << k.total
                  << " (want " << wantTotal << "), comps " << k.components
                  << " (want " << wantComps << ")" << std::endl;
        std::abort();
    }
}

int main() {
    // 1. 正常用例：三角形。
    checkMst("triangle", 3, {{0, 1, 1}, {1, 2, 2}, {0, 2, 3}}, 3, 1);

    // 2. 重边：同一条 (0,1) 出现两次，必须选权重小的 2。
    checkMst("multi-edge", 3, {{0, 1, 5}, {0, 1, 2}, {1, 2, 4}}, 6, 1);

    // 3. 不连通：{0,1} 和 {2,3} 各自成树，4 号点孤立。
    //    这是「边数 = V - comps = 5 - 3 = 2，而不是 V - 1 = 4」的用例。
    checkMst("disconnected", 5, {{0, 1, 1}, {2, 3, 2}}, 3, 3);

    // 4. 边界：单点、空图。
    checkMst("single", 1, {}, 0, 1);
    checkMst("empty", 0, {}, 0, 0);

    // 5. 链：唯一生成树就是全部边。
    checkMst("chain", 5, {{0, 1, 1}, {1, 2, 2}, {2, 3, 3}, {3, 4, 4}}, 10, 1);

    // 6. 关键点：必须跳过成环边。四边形四条 1 + 一条重对角边 5，
    //    只需 3 条边就够，总权重 3；若忘了判环会多收一条（甚至收下 5）。
    checkMst("square", 4,
             {{0, 1, 1}, {1, 2, 1}, {2, 3, 1}, {0, 3, 1}, {0, 2, 5}}, 3, 1);

    // 7. 排序自检：手写归并排序必须非递减，且元素一个不少。
    std::vector<Edge> ugly = {
        {0, 1, 5}, {1, 2, 1}, {2, 3, 5}, {3, 4, 2}, {4, 0, 3}};
    sortEdges(ugly);
    assert(ugly.size() == 5);
    for (size_t i = 1; i < ugly.size(); ++i) {
        assert(ugly[i - 1].w <= ugly[i].w);
    }
    assert(ugly[0].w == 1 && ugly[4].w == 5);

    // 8. 随机对拍：300 组，两版本总权重必须一致，且满足森林不变量。
    Rng rng(20240502ULL);
    for (int round = 0; round < 300; ++round) {
        int n = rng.in(0, 8);
        int m = rng.in(0, 3 * n);
        std::vector<Edge> es;
        for (int i = 0; i < m; ++i) {
            int u = rng.in(0, n - 1);
            int v = rng.in(0, n - 1);
            int w = rng.in(1, 20);
            es.push_back(Edge{u, v, w});
        }
        checkConsistent(n, es);
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
