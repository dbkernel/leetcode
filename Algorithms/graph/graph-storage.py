"""
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
  1. 矩阵置 m[0][1] = m[1][0] = True，m[1][2] = m[2][1] = True；
  2. 邻接表 adj[0] = [1]，adj[1] = [0,2]，adj[2] = [1]；
  3. 度数和 1 + 2 + 1 = 4 = 2E，两边集都是 {(0,1),(1,2)}。

复杂度分析：
  时间：建图 O(V^2) 矩阵 / O(V+E) 邻接表，查询见上方分操作表。
  空间：O(V^2) 矩阵，O(V+E) 邻接表。
  适用：V 小或边密集用矩阵，V 大且边稀疏用邻接表。
"""


class GraphMatrix:
    """邻接矩阵：m[u][v] 为 True 表示有边 u->v。"""

    def __init__(self, n, directed):
        self.n = n
        self.directed = directed
        self.m = [[False] * n for _ in range(n)]
        self.edges = 0

    def has_edge(self, u, v):
        return self.m[u][v]

    def add_edge(self, u, v):
        if self.m[u][v]:
            return  # 重边按集合语义忽略
        self.m[u][v] = True
        self.edges += 1
        # 无向图对称置位；自环 u == v 时同一格只置一次
        if not self.directed and u != v:
            self.m[v][u] = True

    def neighbors(self, u):
        res = []
        for v in range(self.n):
            if self.m[u][v]:
                res.append(v)
        return res

    def degree(self, u):
        # 无向图的度：自环只占一个格，但图论上算 2 度
        assert not self.directed
        d = 0
        for v in range(self.n):
            if self.m[u][v]:
                d += 2 if u == v else 1
        return d

    def out_degree(self, u):
        assert self.directed
        d = 0
        for v in range(self.n):
            if self.m[u][v]:
                d += 1
        return d

    def in_degree(self, u):
        assert self.directed
        d = 0
        for v in range(self.n):
            if self.m[v][u]:
                d += 1
        return d

    def edge_count(self):
        return self.edges

    def edge_list(self):
        # 规范边集：无向图只取上三角，自环只留一次；有向图全扫
        res = []
        for u in range(self.n):
            start = 0 if self.directed else u
            for v in range(start, self.n):
                if self.m[u][v]:
                    res.append((u, v))
        return res


class GraphList:
    """邻接表：adj[u] 存出边终点，保持升序，遍历顺序可复现。"""

    def __init__(self, n, directed):
        self.n = n
        self.directed = directed
        self.adj = [[] for _ in range(n)]
        self.indeg = [0] * n  # 有向图入度，加边时维护
        self.edges = 0

    def has_edge(self, u, v):
        for x in self.adj[u]:
            if x == v:
                return True
        return False

    def _insert_sorted(self, u, v):
        # 从尾部找插入点再整体右移，O(degree)，不用库排序
        pos = len(self.adj[u])
        while pos > 0 and self.adj[u][pos - 1] > v:
            pos -= 1
        self.adj[u].insert(pos, v)

    def add_edge(self, u, v):
        if self.has_edge(u, v):
            return  # 重边按集合语义忽略
        self._insert_sorted(u, v)
        if self.directed:
            self.indeg[v] += 1
        elif u != v:
            self._insert_sorted(v, u)
        else:
            # 无向自环存两份，使 degree(u) 自然等于 2
            self._insert_sorted(u, u)
        self.edges += 1

    def neighbors(self, u):
        return list(self.adj[u])

    def degree(self, u):
        assert not self.directed
        return len(self.adj[u])

    def out_degree(self, u):
        assert self.directed
        return len(self.adj[u])

    def in_degree(self, u):
        assert self.directed
        return self.indeg[u]

    def edge_count(self):
        return self.edges

    def edge_list(self):
        res = []
        for u in range(self.n):
            for v in self.adj[u]:
                if self.directed:
                    res.append((u, v))
                elif u < v:
                    res.append((u, v))
                elif u == v:
                    # 自环存了两份，只保留第一条，否则边集多算一次
                    if not res or res[-1] != (u, v):
                        res.append((u, v))
        return res


def check_same_graph(gm, gl):
    """同一张图分别用两种存储建一遍，逐项对齐。"""
    assert gm.n == gl.n
    assert gm.directed == gl.directed
    assert gm.edge_count() == gl.edge_count()
    assert gm.edge_list() == gl.edge_list()
    for u in range(gm.n):
        for v in range(gm.n):
            assert gm.has_edge(u, v) == gl.has_edge(u, v)
    if gm.directed:
        out_sum = 0
        in_sum = 0
        for u in range(gm.n):
            assert gm.out_degree(u) == gl.out_degree(u)
            assert gm.in_degree(u) == gl.in_degree(u)
            out_sum += gm.out_degree(u)
            in_sum += gm.in_degree(u)
        # 有向图不变量：出度之和 = 入度之和 = E
        assert out_sum == gm.edge_count()
        assert in_sum == gm.edge_count()
    else:
        deg_sum = 0
        for u in range(gm.n):
            assert gm.degree(u) == gl.degree(u)
            deg_sum += gm.degree(u)
        # 无向图不变量：度数和 = 2E；自环算 2 度正是为了这条成立
        assert deg_sum == 2 * gm.edge_count()


def main():
    # 1. 空图：任何查询都不能越界，边数必须是 0
    gm = GraphMatrix(4, False)
    gl = GraphList(4, False)
    check_same_graph(gm, gl)
    assert gm.edge_count() == 0
    for u in range(4):
        assert gm.degree(u) == 0
        assert gm.neighbors(u) == []
        assert gl.neighbors(u) == []
    assert not gm.has_edge(0, 1)

    # 2. 无向链 0-1-2，并重复加 (0,1) 验证重边不改变结构
    gm = GraphMatrix(3, False)
    gl = GraphList(3, False)
    for u, v in [(0, 1), (1, 2)]:
        gm.add_edge(u, v)
        gl.add_edge(u, v)
    gm.add_edge(0, 1)
    gl.add_edge(0, 1)
    check_same_graph(gm, gl)
    assert gm.edge_count() == 2
    assert gm.degree(1) == 2
    assert gm.neighbors(1) == [0, 2]
    assert gl.neighbors(1) == [0, 2]

    # 3. 无向自环：矩阵只有一格，邻接表存两份，度都必须是 2
    gm = GraphMatrix(3, False)
    gl = GraphList(3, False)
    gm.add_edge(0, 0)
    gl.add_edge(0, 0)
    gm.add_edge(0, 0)  # 再次加同一条自环仍是重边
    gl.add_edge(0, 0)
    check_same_graph(gm, gl)
    assert gm.edge_count() == 1
    assert gm.degree(0) == 2
    assert gl.degree(0) == 2
    assert gm.has_edge(0, 0)
    assert gm.neighbors(0) == [0]
    assert gl.neighbors(0) == [0, 0]
    assert sum(gm.degree(u) for u in range(3)) == 2 * gm.edge_count()

    # 4. 有向图含自环：入度、出度分别核对，并保持两个和不变
    gm = GraphMatrix(5, True)
    gl = GraphList(5, True)
    for u, v in [(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 3)]:
        gm.add_edge(u, v)
        gl.add_edge(u, v)
    check_same_graph(gm, gl)
    assert gm.edge_count() == 6
    assert gm.out_degree(3) == 2
    assert gm.in_degree(3) == 3  # 来自 1、2 以及 3 的自环
    assert gm.in_degree(4) == 1
    assert gm.has_edge(3, 3)
    assert gm.neighbors(3) == [3, 4]
    assert gl.neighbors(3) == [3, 4]

    # 5. 稀疏图：10 点 7 边，两条存储的边集完全一致
    gm = GraphMatrix(10, False)
    gl = GraphList(10, False)
    i = 0
    while i + 3 < 10:
        gm.add_edge(i, i + 3)
        gl.add_edge(i, i + 3)
        i += 1
    check_same_graph(gm, gl)
    assert gm.edge_count() == 7
    assert gm.degree(0) == 1

    # 6. 稠密图：7 点完全图，E = 21，每个点度数为 6
    gm = GraphMatrix(7, False)
    gl = GraphList(7, False)
    for u in range(7):
        for v in range(u + 1, 7):
            gm.add_edge(u, v)
            gl.add_edge(u, v)
    check_same_graph(gm, gl)
    assert gm.edge_count() == 21
    for u in range(7):
        assert gm.degree(u) == 6

    print("all tests passed")


if __name__ == "__main__":
    main()
