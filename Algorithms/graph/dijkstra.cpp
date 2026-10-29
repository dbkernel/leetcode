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
  · 堆自己写：push 上浮，pop 用末尾元素补根后下沉；不用现成容器。

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
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <vector>

// 内部距离上界，只要比任何可能的最短路大即可；输出前统一换成 -1。
const int INF = 1000000000;

struct Edge {
    int to;
    int w;
};

// 邻接表：g[u] 存放 u 的全部出边。
using Graph = std::vector<std::vector<Edge>>;

// 仅用于自测：统计堆弹出次数，用来证明「过期记录确实存在且被跳过」。
static long long g_pops = 0;

// 手写二叉小根堆，元素是 (dist, vertex) 二元组。
struct MinHeap {
    std::vector<int> d;  // 距离
    std::vector<int> v;  // 顶点
    int n = 0;           // 逻辑大小，容量可以更大

    bool empty() const { return n == 0; }

    void push(int dist, int vert) {
        if (n == static_cast<int>(d.size())) {
            d.push_back(0);
            v.push_back(0);
        }
        int i = n++;  // 新元素放到末尾，再一路向上浮
        d[i] = dist;
        v[i] = vert;
        while (i > 0) {
            int p = (i - 1) / 2;  // 完全二叉树的父下标
            if (d[p] <= d[i]) break;  // 父不比子大，堆性质已满足
            std::swap(d[p], d[i]);
            std::swap(v[p], v[i]);
            i = p;
        }
    }

    // 弹出堆顶；调用方保证堆非空。
    void pop(int &outDist, int &outVert) {
        outDist = d[0];
        outVert = v[0];
        --n;
        if (n == 0) return;
        d[0] = d[n];  // 末尾元素补到根，再下沉到合适位置
        v[0] = v[n];
        int i = 0;
        while (true) {
            int l = 2 * i + 1;
            int r = l + 1;
            int m = i;
            if (l < n && d[l] < d[m]) m = l;
            if (r < n && d[r] < d[m]) m = r;
            if (m == i) break;
            std::swap(d[m], d[i]);
            std::swap(v[m], v[i]);
            i = m;
        }
    }
};

struct ShortestPath {
    std::vector<int> dist;    // 不可达为 -1
    std::vector<int> parent;  // 源点与不可达点为 -1
};

// 堆优化 Dijkstra，O((V + E) log V)。
ShortestPath dijkstraHeap(const Graph &g, int src) {
    int n = static_cast<int>(g.size());
    std::vector<int> dist(static_cast<size_t>(n), INF);
    std::vector<int> parent(static_cast<size_t>(n), -1);
    dist[src] = 0;
    MinHeap h;
    h.push(0, src);
    while (!h.empty()) {
        int d, u;
        h.pop(d, u);
        ++g_pops;
        // 惰性删除：堆里可能躺着 u 的旧距离，旧记录直接丢弃。
        if (d > dist[u]) continue;
        for (const Edge &e : g[u]) {
            int nd = d + e.w;
            // 必须严格小于才更新：相等时不动 parent，避免出现环。
            if (nd < dist[e.to]) {
                dist[e.to] = nd;
                parent[e.to] = u;
                h.push(nd, e.to);
            }
        }
    }
    for (int i = 0; i < n; ++i) {
        if (dist[i] == INF) dist[i] = -1;
    }
    return {dist, parent};
}

// 朴素 Dijkstra，O(V^2)：每轮线性扫描找未确定点里 dist 最小的。
// 小图（V 几十）时它常常比堆版本快，省掉了堆的常数。
// 堆优化版本能到 O(E log V)，但这里只作为对照。
ShortestPath dijkstraNaive(const Graph &g, int src) {
    int n = static_cast<int>(g.size());
    std::vector<int> dist(static_cast<size_t>(n), INF);
    std::vector<int> parent(static_cast<size_t>(n), -1);
    std::vector<char> done(static_cast<size_t>(n), 0);
    dist[src] = 0;
    for (int round = 0; round < n; ++round) {
        int u = -1;
        for (int v = 0; v < n; ++v) {
            if (done[v] || dist[v] == INF) continue;
            if (u == -1 || dist[v] < dist[u]) u = v;
        }
        if (u == -1) break;  // 剩下的点全不可达，提前收工
        done[u] = 1;
        for (const Edge &e : g[u]) {
            if (dist[u] + e.w < dist[e.to]) {
                dist[e.to] = dist[u] + e.w;
                parent[e.to] = u;
            }
        }
    }
    for (int i = 0; i < n; ++i) {
        if (dist[i] == INF) dist[i] = -1;
    }
    return {dist, parent};
}

// 顺着 parent 从 dst 回溯到 src，再整体反转。不可达时返回空数组。
std::vector<int> buildPath(const std::vector<int> &parent, int src, int dst) {
    std::vector<int> path;
    int v = dst;
    // parent 只会指向距离更小的点，所以循环一定在有限步内结束。
    while (v != -1) {
        path.push_back(v);
        if (v == src) break;
        v = parent[v];
    }
    if (path.empty() || path.back() != src) return {};
    for (size_t i = 0, j = path.size() - 1; i < j; ++i, --j) {
        std::swap(path[i], path[j]);
    }
    return path;
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

static void addEdge(Graph &g, int u, int v, int w) {
    g[u].push_back(Edge{v, w});
}

static void printVec(const std::vector<int> &a) {
    std::cout << "[";
    for (size_t i = 0; i < a.size(); ++i) {
        if (i != 0) std::cout << ", ";
        std::cout << a[i];
    }
    std::cout << "]";
}

static void checkVec(const char *name, const std::vector<int> &got,
                     const std::vector<int> &expect) {
    if (got == expect) return;
    std::cout << "case " << name << " failed\n  got    ";
    printVec(got);
    std::cout << "\n  expect ";
    printVec(expect);
    std::cout << std::endl;
    std::abort();
}

// 仅用于校验：Floyd 作为参照，实现部分没有用它。
static std::vector<std::vector<int>> floyd(const Graph &g) {
    int n = static_cast<int>(g.size());
    std::vector<std::vector<int>> d(
        n, std::vector<int>(static_cast<size_t>(n), INF));
    for (int i = 0; i < n; ++i) d[i][i] = 0;
    for (int u = 0; u < n; ++u) {
        for (const Edge &e : g[u]) {
            if (e.w < d[u][e.to]) d[u][e.to] = e.w;  // 重边取最小
        }
    }
    for (int k = 0; k < n; ++k) {
        for (int i = 0; i < n; ++i) {
            if (d[i][k] == INF) continue;
            for (int j = 0; j < n; ++j) {
                if (d[k][j] == INF) continue;
                if (d[i][k] + d[k][j] < d[i][j]) {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            if (d[i][j] == INF) d[i][j] = -1;
        }
    }
    return d;
}

// 取 u->v 的最小边权，重边时取最小。找不到返回 false。
static bool minWeight(const Graph &g, int u, int v, int *out) {
    bool found = false;
    for (const Edge &e : g[u]) {
        if (e.to != v) continue;
        if (!found || e.w < *out) {
            *out = e.w;
            found = true;
        }
    }
    return found;
}

int main() {
    // 1. 正常用例：手推的 6 节点图。
    Graph g(6);
    addEdge(g, 0, 1, 7);
    addEdge(g, 0, 2, 9);
    addEdge(g, 0, 5, 14);
    addEdge(g, 1, 2, 10);
    addEdge(g, 1, 3, 15);
    addEdge(g, 2, 3, 11);
    addEdge(g, 2, 5, 2);
    addEdge(g, 3, 4, 6);
    addEdge(g, 5, 4, 9);
    ShortestPath sp = dijkstraHeap(g, 0);
    checkVec("six-node dist", sp.dist, {0, 7, 9, 20, 20, 11});
    checkVec("six-node path to 4", buildPath(sp.parent, 0, 4), {0, 2, 5, 4});
    checkVec("six-node path to 3", buildPath(sp.parent, 0, 3), {0, 2, 3});
    // 朴素版本必须给出同样的距离。
    checkVec("six-node naive", dijkstraNaive(g, 0).dist, sp.dist);

    // 2. 边界：单点、单点自环、不可达、零权边。
    Graph one(1);
    ShortestPath s1 = dijkstraHeap(one, 0);
    checkVec("single dist", s1.dist, {0});
    checkVec("single parent", s1.parent, {-1});
    checkVec("single path", buildPath(s1.parent, 0, 0), {0});

    Graph two(2);
    addEdge(two, 0, 1, 5);
    checkVec("unreachable from 0", dijkstraHeap(two, 0).dist, {0, 5});
    ShortestPath s3 = dijkstraHeap(two, 1);
    checkVec("unreachable from 1", s3.dist, {-1, 0});
    assert(buildPath(s3.parent, 1, 0).empty());

    Graph zero(3);
    addEdge(zero, 0, 1, 0);
    addEdge(zero, 1, 2, 0);
    checkVec("zero weight", dijkstraHeap(zero, 0).dist, {0, 0, 0});
    checkVec("zero weight path", buildPath(dijkstraHeap(zero, 0).parent, 0, 2),
             {0, 1, 2});

    // 自环不参与最短路：0->0 权 1 不应该把 dist[0] 变大。
    Graph selfLoop(2);
    addEdge(selfLoop, 0, 0, 1);
    addEdge(selfLoop, 0, 1, 4);
    checkVec("self loop", dijkstraHeap(selfLoop, 0).dist, {0, 4});

    // 3. 关键点：惰性删除。0->3 初始 10，经 0->1->3 变成 3，
    //    堆里那条 (10, 3) 成为过期记录，弹出时必须跳过。
    Graph dec(5);
    addEdge(dec, 0, 3, 10);
    addEdge(dec, 0, 1, 1);
    addEdge(dec, 1, 3, 2);
    addEdge(dec, 3, 2, 1);
    addEdge(dec, 2, 4, 1);
    g_pops = 0;
    ShortestPath sd = dijkstraHeap(dec, 0);
    checkVec("stale dist", sd.dist, {0, 1, 4, 3, 5});
    checkVec("stale path to 4", buildPath(sd.parent, 0, 4), {0, 1, 3, 2, 4});
    // 只有 5 个点却弹了 6 次，说明过期记录确实存在且被跳过。
    assert(g_pops > 5);

    // 4. 随机对拍：200 组小图，堆版本 vs 朴素版本 vs Floyd。
    Rng rng(20240501ULL);
    for (int round = 0; round < 200; ++round) {
        int n = rng.in(1, 6);
        Graph rg(static_cast<size_t>(n));
        int m = rng.in(0, 3 * n);
        for (int i = 0; i < m; ++i) {
            int u = rng.in(0, n - 1);
            int v = rng.in(0, n - 1);
            int w = rng.in(0, 9);  // 含 0 权，仍满足非负
            addEdge(rg, u, v, w);
        }
        int src = rng.in(0, n - 1);
        ShortestPath heapRes = dijkstraHeap(rg, src);
        ShortestPath naiveRes = dijkstraNaive(rg, src);
        std::vector<std::vector<int>> want = floyd(rg);
        checkVec("random heap vs naive", heapRes.dist, naiveRes.dist);
        checkVec("random heap vs floyd", heapRes.dist, want[src]);
        // parent 链必须真能还原出一条权重等于 dist 的路径。
        for (int v = 0; v < n; ++v) {
            std::vector<int> path = buildPath(heapRes.parent, src, v);
            if (heapRes.dist[v] == -1) {
                assert(path.empty());
                continue;
            }
            assert(!path.empty());
            assert(path.front() == src && path.back() == v);
            int sum = 0;
            for (size_t i = 0; i + 1 < path.size(); ++i) {
                int w = 0;
                assert(minWeight(rg, path[i], path[i + 1], &w));
                sum += w;
            }
            assert(sum == heapRes.dist[v]);
        }
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
