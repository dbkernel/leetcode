"""
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
  · unite 返回「是否真的合并了」：两端已同根时返回 False，count 不能
    减一。判环就靠这个返回值。

手推一遍：
  5 个元素 0..4，依次 union(0,1)、union(2,3)、union(0,2)、union(0,1)：
  1. union(0,1)：两块都只有一个点，把 1 挂到 0 下，count 5 -> 4。
  2. union(2,3)：同理把 3 挂到 2 下，count 4 -> 3。
  3. union(0,2)：find(0)=0，find(2)=2，size 都是 2 不交换，把 2 挂到
     0 下；1 和 3 仍在各自父节点下，count 3 -> 2。
  4. union(0,1)：find(0) == find(1) == 0，返回 False，count 仍是 2。
  此时分量是 {0,1,2,3} 与 {4}，connected(3, 1) 为 True。

复杂度分析：
  只做路径压缩：均摊 O(log n)，不是 O(1)；单次仍可能走到较深的节点。
  只做按秩合并：O(log n)，因为树高被限制在 log n 以内。
  两者同用：均摊 O(alpha(n))，alpha 是反 Ackermann 函数，增长极慢，
    但确实不是常数，所以只能说「近似 O(1)」，不能说严格 O(1)。
  空间 O(n)：parent 加一个辅助数组（size 或 rank，二选一）。
"""


# 合并方法统一叫 unite（C++ 里 union 是关键字），四语言保持同名。
class DsuBySize:
    """按大小合并 + 两趟路径压缩。size 只有根节点上有意义。"""

    def __init__(self, n):
        self.parent = list(range(n))
        self.size = [1] * n
        self.comps = n  # 当前连通分量数

    def find(self, x):
        root = x
        while self.parent[root] != root:  # 第一趟：找根
            root = self.parent[root]
        while self.parent[x] != root:  # 第二趟：路径上每个点直挂根
            nxt = self.parent[x]
            self.parent[x] = root
            x = nxt
        return root

    def unite(self, a, b):
        """返回 False 表示两端本来就在同一个集合，合并没有发生。"""
        ra, rb = self.find(a), self.find(b)
        if ra == rb:
            return False
        if self.size[ra] < self.size[rb]:  # 小树挂大树
            ra, rb = rb, ra
        self.parent[rb] = ra
        self.size[ra] += self.size[rb]
        self.comps -= 1
        return True

    def connected(self, a, b):
        return self.find(a) == self.find(b)

    def count(self):
        return self.comps


class DsuByRank:
    """按秩合并 + 路径减半。rank 是树高的上界，只有根节点有意义。"""

    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [0] * n
        self.comps = n

    def find(self, x):
        while self.parent[x] != x:
            self.parent[x] = self.parent[self.parent[x]]  # 挂到祖父，减半
            x = self.parent[x]
        return x

    def unite(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra == rb:
            return False
        if self.rank[ra] < self.rank[rb]:
            ra, rb = rb, ra
        self.parent[rb] = ra
        # 只有两棵树一样高时，新树才会比原来高一层。
        if self.rank[ra] == self.rank[rb]:
            self.rank[ra] += 1
        self.comps -= 1
        return True

    def connected(self, a, b):
        return self.find(a) == self.find(b)

    def count(self):
        return self.comps


def first_cycle_edge(n, edges):
    """按顺序加无向边，返回第一条会成环的边下标，没有则返回 -1。

    注意「边数 > V - 1 必有环」只是必要条件：边数等于 V - 1 时也可能
    有环（别处断开），准确判断只能靠每次 unite 的返回值。
    """
    dsu = DsuBySize(n)
    for i, (u, v) in enumerate(edges):
        if not dsu.unite(u, v):
            return i
    return -1


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


def depth_of(parent, x):
    """从 x 走到根经过的边数，用来量树高有没有退化。"""
    d = 0
    while parent[x] != x:
        x = parent[x]
        d += 1
    return d


def main():
    # 1. 逐步合并：连通性、count 的变化。
    d = DsuBySize(5)
    assert d.count() == 5
    assert d.unite(0, 1)
    assert d.count() == 4
    assert d.connected(0, 1)
    assert not d.connected(1, 2)
    assert d.unite(2, 3)
    assert d.count() == 3
    assert d.unite(0, 2)
    assert d.connected(3, 1)
    assert d.count() == 2

    # 2. 重复 merge：返回 False，count 不能变（这正是判环的依据）。
    assert not d.unite(0, 1)
    assert d.count() == 2
    assert not d.unite(3, 0)
    assert d.count() == 2

    # 按秩版本跑同样的序列，结果必须一致。
    r = DsuByRank(5)
    assert r.unite(0, 1)
    assert r.unite(2, 3)
    assert r.unite(0, 2)
    assert r.connected(3, 1)
    assert r.count() == 2
    assert not r.unite(0, 1)
    assert r.count() == 2

    # 3. 关键点一：两趟路径压缩把链压平。手工造 0->1->2->3->4。
    chain = DsuBySize(5)
    for i in range(4):
        chain.parent[i] = i + 1
    assert chain.find(0) == 4
    assert chain.parent[0] == 4  # 第二趟让 0 直挂根
    assert chain.parent[1] == 4
    assert depth_of(chain.parent, 0) == 1

    # 关键点二：路径减半只走一趟，隔一个挂一次，高度约减半。
    half = DsuByRank(5)
    for i in range(4):
        half.parent[i] = i + 1
    assert half.find(0) == 4
    assert half.parent[0] == 2
    assert half.parent[2] == 4
    assert depth_of(half.parent, 0) == 2

    # 4. 应用：判环。
    assert first_cycle_edge(3, [(0, 1), (1, 2), (2, 0)]) == 2
    assert first_cycle_edge(3, [(0, 1), (1, 2)]) == -1
    assert first_cycle_edge(4, [(0, 1), (1, 2), (2, 3), (3, 0)]) == 3
    # 边数正好等于 V - 1 = 3，却仍有环：3 号点孤立，别处三角形。
    assert first_cycle_edge(4, [(0, 1), (1, 2), (2, 0)]) == 2

    # 5. 顺序合并 1e5 次：不能退化成链，树高必须是对数级。
    n = 100000
    big = DsuBySize(n)
    for i in range(n - 1):
        assert big.unite(i, i + 1)
    assert big.count() == 1
    assert big.connected(0, n - 1)
    max_depth = 0
    for i in range(n):
        dp = depth_of(big.parent, i)
        if dp > max_depth:
            max_depth = dp
    assert max_depth <= 40, max_depth  # 2 * log2(1e5) 约 34

    big_rank = DsuByRank(n)
    for i in range(n - 1):
        assert big_rank.unite(i, i + 1)
    assert big_rank.count() == 1
    max_depth = 0
    for i in range(n):
        dp = depth_of(big_rank.parent, i)
        if dp > max_depth:
            max_depth = dp
    assert max_depth <= 40, max_depth

    # 6. 随机操作序列：count 必须始终等于 n - 成功合并次数。
    gen = Rng(20240503)
    for _round in range(200):
        size = gen.in_range(1, 50)
        ops = DsuBySize(size)
        merged = 0
        for _k in range(100):
            a = gen.in_range(0, size - 1)
            b = gen.in_range(0, size - 1)
            if ops.unite(a, b):
                merged += 1
            assert ops.count() == size - merged
        # count == 1 时全图连通，任取两点必须 connected。
        if ops.count() == 1:
            assert ops.connected(0, size - 1)

    # 7. 边界：空集合、单元素（自己和自己同根，重复合并返回 False）。
    assert DsuBySize(0).count() == 0
    assert DsuByRank(0).count() == 0
    one = DsuBySize(1)
    assert one.count() == 1
    assert one.connected(0, 0)
    assert not one.unite(0, 0)
    assert one.count() == 1
    one_rank = DsuByRank(1)
    assert one_rank.connected(0, 0)
    assert not one_rank.unite(0, 0)
    assert one_rank.count() == 1

    print("all tests passed")


if __name__ == "__main__":
    main()
