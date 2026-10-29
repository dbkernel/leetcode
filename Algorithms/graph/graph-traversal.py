"""
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
"""


class Graph:
    """无向图邻接表：邻居保持升序，遍历结果才可复现。"""

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
        self._insert_sorted(u, v)
        if u != v:
            self._insert_sorted(v, u)  # 自环只存一份即可

    def neighbors(self, u):
        return self.adj[u]


class RingQueue:
    """手写环形队列：head/tail 取模回绕，cnt 区分空与满。"""

    def __init__(self, cap):
        self.buf = [0] * cap
        self.head = 0
        self.tail = 0
        self.cnt = 0

    def empty(self):
        return self.cnt == 0

    def size(self):
        return self.cnt

    def push(self, x):
        assert self.cnt < len(self.buf)  # 调用方保证不会塞满
        self.buf[self.tail] = x
        self.tail = (self.tail + 1) % len(self.buf)
        self.cnt += 1

    def pop(self):
        assert self.cnt > 0
        x = self.buf[self.head]
        self.head = (self.head + 1) % len(self.buf)
        self.cnt -= 1
        return x


def _dfs_visit(g, u, vis, out):
    vis[u] = True
    out.append(u)  # 先序位置：进入时记录
    for v in g.neighbors(u):
        if not vis[v]:
            _dfs_visit(g, v, vis, out)


def dfs_recursive(g, s):
    vis = [False] * g.n
    out = []
    _dfs_visit(g, s, vis, out)
    return out


def dfs_iterative(g, s):
    vis = [False] * g.n
    st = []  # 用 list 当栈，不用 collections.deque
    out = []
    vis[s] = True
    st.append(s)  # 关键：入栈时就标记，不能等出栈
    while st:
        u = st.pop()
        out.append(u)
        # 逆序压栈，弹出时才是邻居下标升序
        for i in range(len(g.neighbors(u)) - 1, -1, -1):
            v = g.neighbors(u)[i]
            if not vis[v]:
                vis[v] = True
                st.append(v)
    return out


def bfs_order(g, s):
    vis = [False] * g.n
    q = RingQueue(g.n + 1)
    out = []
    vis[s] = True
    q.push(s)
    while not q.empty():
        u = q.pop()
        out.append(u)
        for v in g.neighbors(u):
            if not vis[v]:
                vis[v] = True  # 入队即标记，避免同一点重复入队
                q.push(v)
    return out


def bfs_dist(g, s):
    # 边权全为 1 时第一次到达即最短；带权图要换 Dijkstra
    dist = [-1] * g.n
    q = RingQueue(g.n + 1)
    dist[s] = 0
    q.push(s)
    while not q.empty():
        u = q.pop()
        for v in g.neighbors(u):
            if dist[v] == -1:
                dist[v] = dist[u] + 1
                q.push(v)
    return dist


def component_sizes(g):
    # 按最小顶点从小到大的顺序，给出每个连通块的节点数。
    vis = [False] * g.n
    sizes = []
    for s in range(g.n):
        if vis[s]:
            continue
        cnt = 0
        st = [s]
        vis[s] = True
        while st:
            u = st.pop()
            cnt += 1
            for v in g.neighbors(u):
                if not vis[v]:
                    vis[v] = True
                    st.append(v)
        sizes.append(cnt)
    return sizes


def component_count(g):
    return len(component_sizes(g))


def shortest_path(g, s, t):
    # BFS 记录 parent，再从终点回溯，最后整体反转得到 s 到 t 的路径。
    parent = [-1] * g.n
    dist = [-1] * g.n
    q = RingQueue(g.n + 1)
    dist[s] = 0
    q.push(s)
    while not q.empty():
        u = q.pop()
        for v in g.neighbors(u):
            if dist[v] == -1:
                dist[v] = dist[u] + 1
                parent[v] = u
                q.push(v)
    if dist[t] == -1:
        return []  # 不可达返回空
    path = []
    cur = t
    while cur != s:
        path.append(cur)
        cur = parent[cur]
    path.append(s)
    path.reverse()
    return path


def all_unique(a, n):
    seen = [False] * n
    for x in a:
        if seen[x]:
            return False
        seen[x] = True
    return True


def main():
    # 1. 单点图：所有接口都要能处理 n = 1
    g = Graph(1)
    assert dfs_recursive(g, 0) == [0]
    assert dfs_iterative(g, 0) == [0]
    assert bfs_order(g, 0) == [0]
    assert bfs_dist(g, 0) == [0]
    assert component_count(g) == 1
    assert component_sizes(g) == [1]
    assert shortest_path(g, 0, 0) == [0]

    # 2. 链 0-1-2-3-4：三种遍历顺序一致，路径可正反还原
    g = Graph(5)
    for i in range(4):
        g.add_edge(i, i + 1)
    expect = [0, 1, 2, 3, 4]
    assert dfs_recursive(g, 0) == expect
    assert dfs_iterative(g, 0) == expect
    assert bfs_order(g, 0) == expect
    assert bfs_dist(g, 0) == [0, 1, 2, 3, 4]
    assert shortest_path(g, 0, 4) == expect
    assert shortest_path(g, 4, 0) == [4, 3, 2, 1, 0]

    # 3. 星形：0 连 1..5，BFS 第一层顺序必须按下标升序
    g = Graph(6)
    for i in range(1, 6):
        g.add_edge(0, i)
    expect = [0, 1, 2, 3, 4, 5]
    assert dfs_recursive(g, 0) == expect
    assert dfs_iterative(g, 0) == expect
    assert bfs_order(g, 0) == expect
    assert bfs_dist(g, 0) == [0, 1, 1, 1, 1, 1]
    assert shortest_path(g, 2, 5) == [2, 0, 5]

    # 4. 两个连通分量：计数、大小、不可达距离都盯住
    g = Graph(5)
    g.add_edge(0, 1)
    g.add_edge(1, 2)
    g.add_edge(3, 4)
    assert component_count(g) == 2
    assert component_sizes(g) == [3, 2]
    assert bfs_dist(g, 0) == [0, 1, 2, -1, -1]
    assert shortest_path(g, 0, 4) == []
    assert dfs_recursive(g, 3) == [3, 4]

    # 5. 带环图：0-1-2-3-0 再加弦 0-2，BFS 距离不被环绕远
    g = Graph(4)
    g.add_edge(0, 1)
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    g.add_edge(3, 0)
    g.add_edge(0, 2)
    assert bfs_order(g, 0) == [0, 1, 2, 3]
    assert bfs_dist(g, 0) == [0, 1, 1, 1]
    assert shortest_path(g, 0, 3) == [0, 3]
    assert component_count(g) == 1

    # 6. 菱形 0-1、0-2、1-2、1-3、2-3：
    #    若迭代 DFS 出栈才标记，3 会被压两次，顺序长度也会超出 V。
    g = Graph(4)
    g.add_edge(0, 1)
    g.add_edge(0, 2)
    g.add_edge(1, 2)
    g.add_edge(1, 3)
    g.add_edge(2, 3)
    it = dfs_iterative(g, 0)
    assert len(it) == 4
    assert all_unique(it, 4)
    assert it[0] == 0
    # 同一份输入重复跑，结果必须完全一致
    assert dfs_iterative(g, 0) == it
    rec = dfs_recursive(g, 0)
    assert len(rec) == 4
    assert all_unique(rec, 4)
    assert rec == [0, 1, 2, 3]
    assert bfs_dist(g, 0) == [0, 1, 1, 2]

    print("all tests passed")


if __name__ == "__main__":
    main()
