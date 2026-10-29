"""
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
"""

INF = 10 ** 9


class Dsu:
    """Kruskal 专用并查集：两趟路径压缩 + 按大小合并，缺一不可。"""

    def __init__(self, n):
        self.parent = list(range(n))
        self.size = [1] * n

    def find(self, x):
        root = x
        while self.parent[root] != root:
            root = self.parent[root]
        while self.parent[x] != root:  # 第二趟：路径上所有点直挂根
            nxt = self.parent[x]
            self.parent[x] = root
            x = nxt
        return root

    def unite(self, a, b):
        """返回 False 表示两端本来就在同一个连通块，这条边会成环。"""
        ra, rb = self.find(a), self.find(b)
        if ra == rb:
            return False
        if self.size[ra] < self.size[rb]:
            ra, rb = rb, ra
        self.parent[rb] = ra
        self.size[ra] += self.size[rb]
        return True


class MstResult:
    def __init__(self, total, edges, components):
        self.total = total          # 选中边的权重和
        self.edges = edges          # 实际选中的边，条数 = V - components
        self.components = components  # 原图的连通分量数


def sort_edges(es):
    """手写自底向上归并排序，按 w 升序；稳定、无递归、空间 O(E)。

    边用三元组 (u, v, w) 表示。返回新列表，不改动入参。
    """
    n = len(es)
    if n < 2:
        return list(es)
    src = list(es)
    dst = [None] * n
    width = 1
    while width < n:
        lo = 0
        while lo < n:
            mid = min(lo + width, n)
            hi = min(lo + 2 * width, n)
            i, j, k = lo, mid, lo
            while i < mid and j < hi:
                # 相等时先取左边，保证稳定（虽然 MST 不依赖稳定性）。
                if src[j][2] < src[i][2]:
                    dst[k] = src[j]
                    j += 1
                else:
                    dst[k] = src[i]
                    i += 1
                k += 1
            while i < mid:
                dst[k] = src[i]
                i += 1
                k += 1
            while j < hi:
                dst[k] = src[j]
                j += 1
                k += 1
            lo += 2 * width
        src, dst = dst, src
        width *= 2
    return src


def kruskal(n, edges):
    """排序 + 并查集，O(E log E)。"""
    dsu = Dsu(n)
    total = 0
    chosen = []
    comps = n
    for u, v, w in sort_edges(edges):
        if not dsu.unite(u, v):  # 成环，跳过
            continue
        total += w
        chosen.append((u, v, w))
        comps -= 1
    return MstResult(total, chosen, comps)


def prim_naive(n, edges):
    """从 0 号点出发，每轮线性扫描 O(V) 选最近的外部点，总 O(V^2)。

    图不连通时外层继续找未访问点，于是得到的是最小生成森林。
    """
    # 邻接矩阵，重边只留最小权重；O(V^2) 空间对朴素版可以接受。
    w = [[INF] * n for _ in range(n)]
    for u, v, weight in edges:
        if weight < w[u][v]:
            w[u][v] = weight
            w[v][u] = weight  # 无向图，两个方向都记
    best = [INF] * n  # 树到该点的最近边
    frm = [-1] * n    # 那条边从哪个树内点来
    inside = [False] * n
    total = 0
    chosen = []
    comps = 0
    for s in range(n):
        if inside[s]:
            continue
        comps += 1
        best[s] = 0  # 起点不需要边连进树
        for _ in range(n):
            u = -1
            for v in range(n):
                if inside[v] or best[v] == INF:
                    continue
                if u == -1 or best[v] < best[u]:
                    u = v
            if u == -1:  # 本分量吸完了
                break
            inside[u] = True
            if frm[u] != -1:
                total += best[u]
                chosen.append((frm[u], u, best[u]))
            for v in range(n):
                if not inside[v] and w[u][v] < best[v]:
                    best[v] = w[u][v]
                    frm[v] = u
    return MstResult(total, chosen, comps)


# ---------------------------------------------------------------------------
# 以下均为自测代码。
# ---------------------------------------------------------------------------


class Rng:
    """固定种子的 64 位 LCG：四语言同一套参数，失败可复现。"""

    MASK = (1 << 64) - 1

    def __init__(self, seed):
        self.s = seed & self.MASK

    def next(self):
        self.s = self.s * 6364136223846793005 + 1442695040888963407
        self.s &= self.MASK
        return self.s >> 33

    def in_range(self, lo, hi):
        return lo + self.next() % (hi - lo + 1)


def same_edge(a, b):
    return (a[0] == b[0] and a[1] == b[1]) or \
        (a[0] == b[1] and a[1] == b[0])


def count_comps(n, edges):
    """独立算一遍连通分量数，作为 components 字段的参照。"""
    dsu = Dsu(n)
    for u, v, _ in edges:
        dsu.unite(u, v)
    c = 0
    for i in range(n):
        if dsu.find(i) == i:
            c += 1
    return c


def check_forest(n, res, tag, inputs):
    """校验选中的边：都来自输入、彼此不成环、权重和等于 total。"""
    dsu = Dsu(n)
    total = 0
    for e in res.edges:
        found = False
        for src in inputs:
            if src[2] == e[2] and same_edge(e, src):
                found = True
        assert found, (tag, e)
        assert dsu.unite(e[0], e[1]), (tag, e)  # 不能成环
        total += e[2]
    assert total == res.total, (tag, total, res.total)


def check_consistent(n, es):
    """两版本总权重相同、分量数相同、边数都是 V - comps。"""
    comps = count_comps(n, es)
    k = kruskal(n, es)
    p = prim_naive(n, es)
    assert k.total == p.total, (k.total, p.total, es)
    assert k.components == comps, (k.components, comps)
    assert p.components == comps, (p.components, comps)
    assert len(k.edges) == n - comps, (len(k.edges), n - comps)
    assert len(p.edges) == n - comps, (len(p.edges), n - comps)
    check_forest(n, k, "kruskal", es)
    check_forest(n, p, "prim", es)


def check_mst(name, n, es, want_total, want_comps):
    check_consistent(n, es)
    k = kruskal(n, es)
    assert k.total == want_total, (name, k.total, want_total)
    assert k.components == want_comps, (name, k.components, want_comps)


def main():
    # 1. 正常用例：三角形。
    check_mst("triangle", 3, [(0, 1, 1), (1, 2, 2), (0, 2, 3)], 3, 1)

    # 2. 重边：同一条 (0,1) 出现两次，必须选权重小的 2。
    check_mst("multi-edge", 3, [(0, 1, 5), (0, 1, 2), (1, 2, 4)], 6, 1)

    # 3. 不连通：{0,1} 和 {2,3} 各自成树，4 号点孤立。
    #    这是「边数 = V - comps = 5 - 3 = 2，而不是 V - 1 = 4」的用例。
    check_mst("disconnected", 5, [(0, 1, 1), (2, 3, 2)], 3, 3)

    # 4. 边界：单点、空图。
    check_mst("single", 1, [], 0, 1)
    check_mst("empty", 0, [], 0, 0)

    # 5. 链：唯一生成树就是全部边。
    check_mst("chain", 5,
              [(0, 1, 1), (1, 2, 2), (2, 3, 3), (3, 4, 4)], 10, 1)

    # 6. 关键点：必须跳过成环边。四边形四条 1 + 一条重对角边 5，
    #    只需 3 条边就够，总权重 3；若忘了判环会多收一条（甚至收下 5）。
    check_mst("square", 4,
              [(0, 1, 1), (1, 2, 1), (2, 3, 1), (0, 3, 1), (0, 2, 5)], 3, 1)

    # 7. 排序自检：手写归并排序必须非递减，且元素一个不少。
    ugly = [(0, 1, 5), (1, 2, 1), (2, 3, 5), (3, 4, 2), (4, 0, 3)]
    ordered = sort_edges(ugly)
    assert len(ordered) == 5
    for i in range(1, len(ordered)):
        assert ordered[i - 1][2] <= ordered[i][2]
    assert ordered[0][2] == 1 and ordered[4][2] == 5
    assert ugly[0][2] == 5  # 入参不能被改动

    # 8. 随机对拍：300 组，两版本总权重必须一致，且满足森林不变量。
    rng = Rng(20240502)
    for _round in range(300):
        n = rng.in_range(0, 8)
        es = []
        for _i in range(rng.in_range(0, 3 * n)):
            u = rng.in_range(0, n - 1)
            v = rng.in_range(0, n - 1)
            w = rng.in_range(1, 20)
            es.append((u, v, w))
        check_consistent(n, es)

    print("all tests passed")


if __name__ == "__main__":
    main()
