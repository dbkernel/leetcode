/*
============================================================================
并查集（Disjoint Set Union）
分类：手写数据结构 · 并查集
============================================================================
操作复杂度（n 个元素、m 次操作）：
  find       O(alpha(n)) 均摊  路径压缩 + 按秩（按大小）合并同用
  unite      O(alpha(n)) 均摊  两次 find 加一次挂接
  connected  O(alpha(n)) 均摊  一次 find 比较根
  count      O(1)              维护分量计数器，合并成功才减一
============================================================================

用途：
  维护一堆互不相交的集合，支持「合并两个集合」和「查两个元素是否同
  集合」。面试里常用于判环、Kruskal、统计连通块、账户合并。

核心思想：
  每个集合是一棵有根树：parent[x] 指向父节点，树根指向自己。合并就是
  把一棵树的根挂到另一棵的根下，查询就是沿 parent 一路走到根。
  树越矮越快，于是用「按秩/按大小合并」压住高度，「路径压缩」压扁路径。

关键点（不变量）：
  · parent[x] == x 当且仅当 x 是根；所有操作入口都是 find，别绕开它。
  · 合并策略只选一种：按大小（size）或按秩（rank），不要两套同时维护，
    否则判断依据不一致，还白搭一个数组。
  · 路径压缩必须迭代：递归在 1e6 长的链上会爆栈。两趟写法先找根再
    把路径上所有点直挂根；路径减半写法边走边把点挂到祖父上，一趟完事。
  · unite 返回「是否真的合并了」：两端已同根时返回 false，且 count 不能
    减一。判环就靠这个返回值。

手推一遍：
  5 个元素 0..4，依次 union(0,1)、union(2,3)、union(0,2)、union(0,1)：
  1. union(0,1)：两块都只有一个点，把 1 挂到 0 下，count 5 -> 4。
  2. union(2,3)：同理把 3 挂到 2 下，count 4 -> 3。
  3. union(0,2)：find(0)=0，find(2)=2，size 都是 2 不交换，把 2 挂到
     0 下；1 和 3 仍在各自父节点下，count 3 -> 2。
  4. union(0,1)：find(0) == find(1) == 0，返回 false，count 仍是 2。
  此时分量是 {0,1,2,3} 与 {4}，connected(3, 1) 为 true。

复杂度分析：
  只做路径压缩：均摊 O(log n)，不是 O(1)；单次仍可能走到较深的节点。
  只做按秩合并：O(log n)，因为树高被限制在 log n 以内。
  两者同用：均摊 O(alpha(n))，alpha 是反 Ackermann 函数，增长极慢，
    但确实不是常数，所以只能说「近似 O(1)」，不能说严格 O(1)。
  空间 O(n)：parent 加一个辅助数组（size 或 rank，二选一）。
*/
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <vector>

// 合并方法统一叫 unite：union 在 C++ 里是关键字，四语言保持同名。
// 按大小合并 + 两趟路径压缩。size 只有根节点上有意义。
struct DsuBySize {
    std::vector<int> parent;
    std::vector<int> size;
    int comps;  // 当前连通分量数

    explicit DsuBySize(int n)
        : parent(static_cast<size_t>(n)),
          size(static_cast<size_t>(n), 1),
          comps(n) {
        for (int i = 0; i < n; ++i) parent[i] = i;
    }

    int find(int x) {
        int root = x;
        while (parent[root] != root) root = parent[root];  // 第一趟：找根
        while (parent[x] != root) {  // 第二趟：路径上每个点直挂根
            int next = parent[x];
            parent[x] = root;
            x = next;
        }
        return root;
    }

    // 返回 false 表示两端本来就在同一个集合，合并没有发生。
    bool unite(int a, int b) {
        int ra = find(a);
        int rb = find(b);
        if (ra == rb) return false;
        if (size[ra] < size[rb]) std::swap(ra, rb);  // 小树挂大树
        parent[rb] = ra;
        size[ra] += size[rb];
        --comps;
        return true;
    }

    bool connected(int a, int b) { return find(a) == find(b); }

    int count() const { return comps; }
};

// 按秩合并 + 路径减半。rank 是树高的上界，只有根节点上有意义。
struct DsuByRank {
    std::vector<int> parent;
    std::vector<int> rank;
    int comps;

    explicit DsuByRank(int n)
        : parent(static_cast<size_t>(n)),
          rank(static_cast<size_t>(n), 0),
          comps(n) {
        for (int i = 0; i < n; ++i) parent[i] = i;
    }

    int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];  // 挂到祖父上，路径长度减半
            x = parent[x];
        }
        return x;
    }

    bool unite(int a, int b) {
        int ra = find(a);
        int rb = find(b);
        if (ra == rb) return false;
        if (rank[ra] < rank[rb]) std::swap(ra, rb);
        parent[rb] = ra;
        // 只有两棵树一样高时，新树才会比原来高一层。
        if (rank[ra] == rank[rb]) ++rank[ra];
        --comps;
        return true;
    }

    bool connected(int a, int b) { return find(a) == find(b); }

    int count() const { return comps; }
};

struct Edge {
    int u;
    int v;
};

// 应用：按顺序加无向边，返回第一条会成环的边下标，没有则返回 -1。
// 注意「边数 > V - 1 必有环」只是必要条件：边数等于 V - 1 时也可能
// 有环（别处断开），准确判断只能靠每次 unite 的返回值。
int firstCycleEdge(int n, const std::vector<Edge> &edges) {
    DsuBySize dsu(n);
    for (size_t i = 0; i < edges.size(); ++i) {
        if (!dsu.unite(edges[i].u, edges[i].v)) {
            return static_cast<int>(i);
        }
    }
    return -1;
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

// 从 x 走到根经过的边数，用来量树高有没有退化。
static int depthOf(const std::vector<int> &parent, int x) {
    int d = 0;
    while (parent[x] != x) {
        x = parent[x];
        ++d;
    }
    return d;
}

int main() {
    // 1. 逐步合并：连通性、count 的变化。
    DsuBySize d(5);
    assert(d.count() == 5);
    assert(d.unite(0, 1));
    assert(d.count() == 4);
    assert(d.connected(0, 1));
    assert(!d.connected(1, 2));
    assert(d.unite(2, 3));
    assert(d.count() == 3);
    assert(d.unite(0, 2));
    assert(d.connected(3, 1));
    assert(d.count() == 2);

    // 2. 重复 merge：返回 false，count 不能变（这正是判环的依据）。
    assert(!d.unite(0, 1));
    assert(d.count() == 2);
    assert(!d.unite(3, 0));
    assert(d.count() == 2);

    // 按秩版本跑同样的序列，结果必须一致。
    DsuByRank r(5);
    assert(r.unite(0, 1));
    assert(r.unite(2, 3));
    assert(r.unite(0, 2));
    assert(r.connected(3, 1));
    assert(r.count() == 2);
    assert(!r.unite(0, 1));
    assert(r.count() == 2);

    // 3. 关键点一：两趟路径压缩把链压平。手工造 0->1->2->3->4。
    DsuBySize chain(5);
    for (int i = 0; i < 4; ++i) chain.parent[i] = i + 1;
    assert(chain.find(0) == 4);
    assert(chain.parent[0] == 4);  // 第二趟让 0 直挂根
    assert(chain.parent[1] == 4);
    assert(depthOf(chain.parent, 0) == 1);

    // 关键点二：路径减半只走一趟，隔一个挂一次，高度约减半。
    DsuByRank half(5);
    for (int i = 0; i < 4; ++i) half.parent[i] = i + 1;
    assert(half.find(0) == 4);
    assert(half.parent[0] == 2);
    assert(half.parent[2] == 4);
    assert(depthOf(half.parent, 0) == 2);

    // 4. 应用：判环。
    assert(firstCycleEdge(3, {{0, 1}, {1, 2}, {2, 0}}) == 2);
    assert(firstCycleEdge(3, {{0, 1}, {1, 2}}) == -1);
    assert(firstCycleEdge(4, {{0, 1}, {1, 2}, {2, 3}, {3, 0}}) == 3);
    // 边数正好等于 V - 1 = 3，却仍有环：3 号点孤立，别处三角形。
    assert(firstCycleEdge(4, {{0, 1}, {1, 2}, {2, 0}}) == 2);

    // 5. 顺序合并 1e5 次：不能退化成链，树高必须是对数级。
    const int N = 100000;
    DsuBySize big(N);
    for (int i = 0; i + 1 < N; ++i) assert(big.unite(i, i + 1));
    assert(big.count() == 1);
    assert(big.connected(0, N - 1));
    int maxDepth = 0;
    for (int i = 0; i < N; ++i) {
        int dpt = depthOf(big.parent, i);
        if (dpt > maxDepth) maxDepth = dpt;
    }
    assert(maxDepth <= 40);  // 2 * log2(1e5) 约 34

    DsuByRank bigRank(N);
    for (int i = 0; i + 1 < N; ++i) assert(bigRank.unite(i, i + 1));
    assert(bigRank.count() == 1);
    maxDepth = 0;
    for (int i = 0; i < N; ++i) {
        int dpt = depthOf(bigRank.parent, i);
        if (dpt > maxDepth) maxDepth = dpt;
    }
    assert(maxDepth <= 40);

    // 6. 随机操作序列：count 必须始终等于 n - 成功合并次数。
    Rng rng(20240503ULL);
    for (int round = 0; round < 200; ++round) {
        int n = rng.in(1, 50);
        DsuBySize ops(n);
        int merged = 0;
        for (int k = 0; k < 100; ++k) {
            int a = rng.in(0, n - 1);
            int b = rng.in(0, n - 1);
            if (ops.unite(a, b)) ++merged;
            assert(ops.count() == n - merged);
        }
        // count == 1 时全图连通，任取两点必须 connected。
        if (ops.count() == 1) assert(ops.connected(0, n - 1));
    }

    // 7. 边界：空集合、单元素（自己和自己同根，重复合并返回 false）。
    assert(DsuBySize(0).count() == 0);
    assert(DsuByRank(0).count() == 0);
    DsuBySize one(1);
    assert(one.count() == 1);
    assert(one.connected(0, 0));
    assert(!one.unite(0, 0));
    assert(one.count() == 1);
    DsuByRank oneRank(1);
    assert(oneRank.connected(0, 0));
    assert(!oneRank.unite(0, 0));
    assert(oneRank.count() == 1);

    std::cout << "all tests passed" << std::endl;
    return 0;
}
