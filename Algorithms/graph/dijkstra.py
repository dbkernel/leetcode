"""
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
  · 堆自己写：push 上浮，pop 用末尾元素补根后下沉；不用 heapq。

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
"""

INF = 10 ** 9

# 仅用于自测：统计堆弹出次数，用来证明「过期记录确实存在且被跳过」。
POPS = 0


class MinHeap:
    """手写二叉小根堆，元素是 (dist, vertex) 二元组。"""

    def __init__(self):
        self.d = []  # 距离
        self.v = []  # 顶点

    def empty(self):
        return not self.d

    def push(self, dist, vert):
        self.d.append(dist)
        self.v.append(vert)
        i = len(self.d) - 1
        while i > 0:
            p = (i - 1) // 2  # 完全二叉树的父下标
            if self.d[p] <= self.d[i]:  # 父不比子大，堆性质已满足
                break
            self.d[p], self.d[i] = self.d[i], self.d[p]
            self.v[p], self.v[i] = self.v[i], self.v[p]
            i = p

    def pop(self):
        dist, vert = self.d[0], self.v[0]
        last_d = self.d.pop()
        last_v = self.v.pop()
        if self.d:
            self.d[0] = last_d  # 末尾元素补到根，再下沉
            self.v[0] = last_v
            i, n = 0, len(self.d)
            while True:
                left = 2 * i + 1
                right = left + 1
                m = i
                if left < n and self.d[left] < self.d[m]:
                    m = left
                if right < n and self.d[right] < self.d[m]:
                    m = right
                if m == i:
                    break
                self.d[m], self.d[i] = self.d[i], self.d[m]
                self.v[m], self.v[i] = self.v[i], self.v[m]
                i = m
        return dist, vert


def dijkstra_heap(g, src):
    """堆优化版本，O((V + E) log V)。返回 (dist, parent)。"""
    global POPS
    n = len(g)
    dist = [INF] * n
    parent = [-1] * n
    dist[src] = 0
    heap = MinHeap()
    heap.push(0, src)
    while not heap.empty():
        d, u = heap.pop()
        POPS += 1
        # 惰性删除：堆里可能躺着 u 的旧距离，旧记录直接丢弃。
        if d > dist[u]:
            continue
        for to, w in g[u]:
            nd = d + w
            # 必须严格小于才更新：相等时不动 parent，避免出现环。
            if nd < dist[to]:
                dist[to] = nd
                parent[to] = u
                heap.push(nd, to)
    for i in range(n):
        if dist[i] == INF:
            dist[i] = -1
    return dist, parent


def dijkstra_naive(g, src):
    """朴素版本 O(V^2)：每轮线性扫描找最近的未确定点。

    小图（V 几十）时它常常比堆版本快，省掉了堆的常数。
    堆优化版本能到 O(E log V)，这里只作为对照。
    """
    n = len(g)
    dist = [INF] * n
    parent = [-1] * n
    done = [False] * n
    dist[src] = 0
    for _ in range(n):
        u = -1
        for v in range(n):
            if done[v] or dist[v] == INF:
                continue
            if u == -1 or dist[v] < dist[u]:
                u = v
        if u == -1:  # 剩下的点全不可达，提前收工
            break
        done[u] = True
        for to, w in g[u]:
            if dist[u] + w < dist[to]:
                dist[to] = dist[u] + w
                parent[to] = u
    for i in range(n):
        if dist[i] == INF:
            dist[i] = -1
    return dist, parent


def build_path(parent, src, dst):
    """顺着 parent 从 dst 回溯到 src，再整体反转；不可达返回空列表。"""
    path = []
    v = dst
    # parent 只会指向距离更小的点，所以循环一定在有限步内结束。
    while v != -1:
        path.append(v)
        if v == src:
            break
        v = parent[v]
    if not path or path[-1] != src:
        return []
    i, j = 0, len(path) - 1
    while i < j:
        path[i], path[j] = path[j], path[i]
        i += 1
        j -= 1
    return path


# ---------------------------------------------------------------------------
# 以下均为自测代码。
# ---------------------------------------------------------------------------


class Rng:
    """固定种子的 64 位 LCG：四语言同一套参数，失败可复现。"""

    MASK = (1 << 64) - 1

    def __init__(self, seed):
        self.s = seed & self.MASK

    def next(self):
        self.s = (self.s * 6364136223846793005 + 1442695040888963407)
        self.s &= self.MASK
        return self.s >> 33

    def in_range(self, lo, hi):
        return lo + self.next() % (hi - lo + 1)


def floyd(g):
    """参照实现，仅用于校验：实现部分没有用它。"""
    n = len(g)
    d = [[INF] * n for _ in range(n)]
    for i in range(n):
        d[i][i] = 0
    for u in range(n):
        for to, w in g[u]:
            if w < d[u][to]:  # 重边取最小
                d[u][to] = w
    for k in range(n):
        for i in range(n):
            if d[i][k] == INF:
                continue
            for j in range(n):
                if d[k][j] == INF:
                    continue
                if d[i][k] + d[k][j] < d[i][j]:
                    d[i][j] = d[i][k] + d[k][j]
    for i in range(n):
        for j in range(n):
            if d[i][j] == INF:
                d[i][j] = -1
    return d


def min_weight(g, u, v):
    """取 u->v 的最小边权，重边时取最小；没有这条边返回 None。"""
    w, found = 0, False
    for to, weight in g[u]:
        if to != v:
            continue
        if not found or weight < w:
            w = weight
            found = True
    return w if found else None


def main():
    # 1. 正常用例：手推的 6 节点图。
    g = [[] for _ in range(6)]
    edges = [
        (0, 1, 7), (0, 2, 9), (0, 5, 14),
        (1, 2, 10), (1, 3, 15),
        (2, 3, 11), (2, 5, 2),
        (3, 4, 6), (5, 4, 9),
    ]
    for u, v, w in edges:
        g[u].append((v, w))
    dist, parent = dijkstra_heap(g, 0)
    assert dist == [0, 7, 9, 20, 20, 11], dist
    assert build_path(parent, 0, 4) == [0, 2, 5, 4]
    assert build_path(parent, 0, 3) == [0, 2, 3]
    # 朴素版本必须给出同样的距离。
    assert dijkstra_naive(g, 0)[0] == dist

    # 2. 边界：单点、不可达、零权边、自环。
    one = [[] for _ in range(1)]
    d1, p1 = dijkstra_heap(one, 0)
    assert d1 == [0]
    assert p1 == [-1]
    assert build_path(p1, 0, 0) == [0]

    two = [[(1, 5)], []]
    assert dijkstra_heap(two, 0)[0] == [0, 5]
    d3, p3 = dijkstra_heap(two, 1)
    assert d3 == [-1, 0], d3
    assert build_path(p3, 1, 0) == []

    zero = [[(1, 0)], [(2, 0)], []]
    dz, pz = dijkstra_heap(zero, 0)
    assert dz == [0, 0, 0]
    assert build_path(pz, 0, 2) == [0, 1, 2]

    # 自环不参与最短路：0->0 权 1 不应该把 dist[0] 变大。
    self_loop = [[(0, 1), (1, 4)], []]
    assert dijkstra_heap(self_loop, 0)[0] == [0, 4]

    # 3. 关键点：惰性删除。0->3 初始 10，经 0->1->3 变成 3，
    #    堆里那条 (10, 3) 成为过期记录，弹出时必须跳过。
    dec = [[] for _ in range(5)]
    for u, v, w in [(0, 3, 10), (0, 1, 1), (1, 3, 2), (3, 2, 1), (2, 4, 1)]:
        dec[u].append((v, w))
    global POPS
    POPS = 0
    dd, pd = dijkstra_heap(dec, 0)
    assert dd == [0, 1, 4, 3, 5], dd
    assert build_path(pd, 0, 4) == [0, 1, 3, 2, 4]
    # 只有 5 个点却弹了 6 次，说明过期记录确实存在且被跳过。
    assert POPS > 5, POPS

    # 4. 随机对拍：200 组小图，堆版本 vs 朴素版本 vs Floyd。
    rng = Rng(20240501)
    for _ in range(200):
        n = rng.in_range(1, 6)
        rg = [[] for _ in range(n)]
        for _ in range(rng.in_range(0, 3 * n)):
            u = rng.in_range(0, n - 1)
            v = rng.in_range(0, n - 1)
            w = rng.in_range(0, 9)  # 含 0 权，仍满足非负
            rg[u].append((v, w))
        src = rng.in_range(0, n - 1)
        heap_dist, heap_parent = dijkstra_heap(rg, src)
        assert heap_dist == dijkstra_naive(rg, src)[0]
        assert heap_dist == floyd(rg)[src]
        # parent 链必须真能还原出一条权重等于 dist 的路径。
        for v in range(n):
            path = build_path(heap_parent, src, v)
            if heap_dist[v] == -1:
                assert path == [], (path, v)
                continue
            assert path and path[0] == src and path[-1] == v
            total = 0
            for i in range(len(path) - 1):
                w = min_weight(rg, path[i], path[i + 1])
                assert w is not None, (path, i)
                total += w
            assert total == heap_dist[v], (total, heap_dist[v])

    print("all tests passed")


if __name__ == "__main__":
    main()
