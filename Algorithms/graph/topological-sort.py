"""
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
"""


class Graph:
    """有向图邻接表：邻居保持升序，输出的拓扑序才可复现。"""

    def __init__(self, n):
        self.n = n
        self.adj = [[] for _ in range(n)]

    def _insert_sorted(self, u, v):
        lst = self.adj[u]
        pos = len(lst)
        while pos > 0 and lst[pos - 1] > v:
            pos -= 1
        lst.insert(pos, v)

    def add_edge(self, u, v):
        self._insert_sorted(u, v)  # 有向图只加单向边

    def neighbors(self, u):
        return self.adj[u]


class RingQueue:
    """手写环形队列：Kahn 里每个点最多入队一次，容量 V+1 足够。"""

    def __init__(self, cap):
        self.buf = [0] * cap
        self.head = 0
        self.tail = 0
        self.cnt = 0

    def empty(self):
        return self.cnt == 0

    def push(self, x):
        assert self.cnt < len(self.buf)
        self.buf[self.tail] = x
        self.tail = (self.tail + 1) % len(self.buf)
        self.cnt += 1

    def pop(self):
        assert self.cnt > 0
        x = self.buf[self.head]
        self.head = (self.head + 1) % len(self.buf)
        self.cnt -= 1
        return x


def kahn(g):
    """入度归零才入队；有环时会有顶点永远进不了队列。"""
    indeg = [0] * g.n
    for u in range(g.n):
        for v in g.neighbors(u):
            indeg[v] += 1  # 自环也计入
    q = RingQueue(g.n + 1)
    for u in range(g.n):
        if indeg[u] == 0:
            q.push(u)
    order = []
    while not q.empty():
        u = q.pop()
        order.append(u)
        for v in g.neighbors(u):
            indeg[v] -= 1
            if indeg[v] == 0:
                q.push(v)
    # 输出不足 V 个点，说明剩下的点互相牵制，必有环
    return len(order) == g.n, order


def _dfs_visit(g, u, color, order):
    color[u] = 1  # 1 = 在递归栈上
    for v in g.neighbors(u):
        if color[v] == 1:
            return False  # 回边，成环
        if color[v] == 0 and not _dfs_visit(g, v, color, order):
            return False
    color[u] = 2  # 2 = 完成，后序加入
    order.append(u)
    return True


def topo_dfs(g):
    """后序是「出度为 0 先完成」，整体反转才是拓扑序。"""
    color = [0] * g.n  # 0 未访问 / 1 在栈上 / 2 完成
    order = []
    for u in range(g.n):
        if color[u] == 0 and not _dfs_visit(g, u, color, order):
            return False, order
    order.reverse()
    return True, order


def is_valid_order(g, order):
    """合法性校验：每个点的所有前驱都必须排在它前面。"""
    if len(order) != g.n:
        return False
    pos = [-1] * g.n
    for i, u in enumerate(order):
        if u < 0 or u >= g.n or pos[u] != -1:
            return False
        pos[u] = i
    for u in range(g.n):
        for v in g.neighbors(u):
            if pos[u] >= pos[v]:
                return False  # 边 u->v 被排反
    return True


def course_schedule(n, deps):
    """deps 中 (before, after) 表示 before 必须在 after 之前修。"""
    g = Graph(n)
    for before, after in deps:
        g.add_edge(before, after)
    ok, order = kahn(g)
    return ok and is_valid_order(g, order), order


def main():
    # 1. 单点：两个版本都必须给出 [0]
    g = Graph(1)
    ok, a = kahn(g)
    ok2, b = topo_dfs(g)
    assert ok and ok2
    assert a == [0]
    assert b == [0]
    assert is_valid_order(g, a) and is_valid_order(g, b)

    # 2. 链 0->1->2->3->4：拓扑序唯一，两个版本应完全一致
    g = Graph(5)
    for i in range(4):
        g.add_edge(i, i + 1)
    ok, a = kahn(g)
    ok2, b = topo_dfs(g)
    assert ok and ok2
    expect = [0, 1, 2, 3, 4]
    assert a == expect
    assert b == expect
    assert is_valid_order(g, a) and is_valid_order(g, b)

    # 3. 多个合法序：0->2、1->2、2->3，只校验合法性不比对序列
    g = Graph(4)
    g.add_edge(0, 2)
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    ok, a = kahn(g)
    ok2, b = topo_dfs(g)
    assert ok and ok2
    assert a != b  # 两个算法确实给出了不同的合法序
    assert is_valid_order(g, a)
    assert is_valid_order(g, b)
    assert len(a) == 4 and len(b) == 4

    # 4. 有环图：两个版本都要返回失败
    g = Graph(3)
    g.add_edge(0, 1)
    g.add_edge(1, 2)
    g.add_edge(2, 0)
    ok, a = kahn(g)
    ok2, b = topo_dfs(g)
    assert not ok and not ok2
    assert len(a) < 3  # Kahn 用长度暴露环

    # 5. 环不在 0 号点：1<->2，0 是挂在旁边的无环入口
    g = Graph(3)
    g.add_edge(0, 1)
    g.add_edge(1, 2)
    g.add_edge(2, 1)
    ok, _ = kahn(g)
    ok2, _ = topo_dfs(g)
    assert not ok and not ok2

    # 6. 有向自环：自己指向自己也是环，两个版本都不能放过
    g = Graph(2)
    g.add_edge(0, 0)
    g.add_edge(1, 0)
    ok, _ = kahn(g)
    ok2, _ = topo_dfs(g)
    assert not ok and not ok2

    # 7. 孤立点 + 一条边：全部顶点都要出现，顺序仍要合法
    g = Graph(4)
    g.add_edge(1, 2)
    ok, a = kahn(g)
    ok2, b = topo_dfs(g)
    assert ok and ok2
    assert len(a) == 4
    assert len(b) == 4
    assert is_valid_order(g, a)
    assert is_valid_order(g, b)

    # 8. 应用：6 门课的先修关系，输出必须满足每一条依赖
    deps = [(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5)]
    ok, order = course_schedule(6, deps)
    assert ok
    g = Graph(6)
    for before, after in deps:
        g.add_edge(before, after)
    assert is_valid_order(g, order)
    # 再逐条核对依赖，避免校验函数本身写错
    pos = [-1] * 6
    for i, u in enumerate(order):
        pos[u] = i
    for before, after in deps:
        assert pos[before] < pos[after]

    # 9. 应用：课程之间成环，必须报告无法修完
    ok, _ = course_schedule(3, [(0, 1), (1, 2), (2, 0)])
    assert not ok

    print("all tests passed")


if __name__ == "__main__":
    main()
