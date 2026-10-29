r"""
============================================================================
树状数组（Fenwick Tree / Binary Indexed Tree）
分类：手写数据结构 · 树
操作复杂度：
  lowbit     O(1)     x & (-x)，取出最低位的 1
  build      O(n)     线性建树，不是 n 次 update 的 O(n log n)
  update     O(log n) 单点加，沿 i += lowbit(i) 往上跳
  query      O(log n) 前缀和，沿 i -= lowbit(i) 往下跳
  range_sum  O(log n) 两个前缀和相减
  空间 O(n)：tree 下标从 1 开始
============================================================================

用途：
  单点修改 + 前缀和 / 区间和的高频结构，比线段树短得多。
  面试常问：lowbit 什么意思、为什么 O(n) 建树、能不能求区间最值。

核心思想：
  把下标 i 的管辖范围定义成 (i - lowbit(i), i] 这一段，tree[i] 存
  这段的和。比如 i=6 (110)，lowbit=2，管 5..6；i=8 (1000) 管
  1..8。查询前缀和时每次去掉最低位的 1，最多走 log n 步；单点更新
  时每次加上最低位的 1，把改动推给所有管到它的节点。

关键点（不变量）：
  · tree[i] 的区间是左开右闭 (i - lowbit(i), i]，所以树状数组必须
    1 基下标；用 0 基会 lowbit(0) = 0 死循环；
  · update 用 i += lowbit(i) 往上跳，query 用 i -= lowbit(i) 往下
    跳，两个方向写反就是死循环或漏算；
  · O(n) 建树必须按 i 从小到大：走到 i 时，所有 j + lowbit(j) = i
    的孩子都已经把值推给了 tree[i]，此时 tree[i] 才是这段的真和；
    逆序时父节点先被加了一次「半成品」，孩子再来加就重复且不准；
  · 区间和是 query(r) - query(l-1)，l=0 时不能查 -1，要单独处理。

手推一遍：
  以 a = {1,2,3,4,5} 为例，线性建树：
  1. i=1: tree[1] += 1，j=2，tree[2] += tree[1] 得 1；
  2. i=2: tree[2] += 2 得 3，j=4，tree[4] += 3 得 3；
  3. i=3: tree[3] += 3 得 3，j=4，tree[4] += 3 得 6；
  4. i=4: tree[4] += 4 得 10，j=8 越界；
  5. i=5: tree[5] += 5 得 5，j=6 越界；
  最终 tree = [_,1,3,3,10,5]；
  6. query(4) 求 0..4：i=5 取 5，i=4 取 10，合计 15，正确。

复杂度分析：
  时间：update / query / range_sum 都是 O(log n)，因为每次至少
        去掉或加上一个二进制位，最多 log2(n) 步；build O(n)。
  空间：O(n)，只比原数组多一个 n+1 的数组。
"""


def lowbit(x):
    return x & (-x)


class Fenwick:
    def __init__(self, a):
        self.n = len(a)
        self.tree = [0] * (len(a) + 1)  # 1 基下标
        # O(n) 建树：i 从小到大，tree[i] 此时已经是自己那段的真和
        for i in range(1, self.n + 1):
            self.tree[i] += a[i - 1]
            j = i + lowbit(i)
            if j <= self.n:
                self.tree[j] += self.tree[i]

    @classmethod
    def zeros(cls, n):
        """全 0 数组，之后用 update 逐个填"""
        return cls([0] * n)

    def update(self, idx, delta):
        """单点加：idx 是 0 基下标，delta 可正可负"""
        i = idx + 1
        while i <= self.n:
            self.tree[i] += delta
            i += lowbit(i)

    def query(self, idx):
        """前缀和 [0, idx]"""
        s = 0
        i = idx + 1
        while i > 0:
            s += self.tree[i]
            i -= lowbit(i)
        return s

    def range_sum(self, l, r):
        """区间和 [l, r]，l > r 时返回 0"""
        if l > r:
            return 0
        s = self.query(r)
        if l > 0:
            s -= self.query(l - 1)  # l=0 时没有前缀要减
        return s


def brute_prefix(a, idx):
    s = 0
    for i in range(idx + 1):
        s += a[i]
    return s


def brute_range(a, l, r):
    s = 0
    for i in range(l, r + 1):
        s += a[i]
    return s


def main():
    # 1. 基本用例：前缀和 / 区间和
    f = Fenwick([1, 2, 3, 4, 5])
    assert f.query(0) == 1
    assert f.query(2) == 6
    assert f.query(4) == 15
    assert f.range_sum(1, 3) == 9
    assert f.range_sum(0, 4) == 15
    assert f.range_sum(2, 2) == 3
    assert f.range_sum(3, 2) == 0  # 空区间
    print("total:", f.query(4))

    # 2. O(n) 建树必须与逐个 update 的结果完全一致
    a = [4, -3, 7, 0, 2, -8, 5, 1, 9, -6, 3]
    fast = Fenwick(a)
    slow = Fenwick.zeros(len(a))
    for i, v in enumerate(a):
        slow.update(i, v)
    for i in range(len(a)):
        assert fast.query(i) == slow.query(i)
        assert fast.query(i) == brute_prefix(a, i)

    # 3. n = 1
    f = Fenwick([7])
    assert f.query(0) == 7
    assert f.range_sum(0, 0) == 7
    f.update(0, 3)
    assert f.query(0) == 10
    f.update(0, -10)
    assert f.query(0) == 0

    # 4. 全 0 与负数
    f = Fenwick([0, 0, 0, 0])
    assert f.query(3) == 0
    assert f.range_sum(0, 3) == 0
    f.update(2, 5)
    assert f.range_sum(0, 3) == 5

    g = Fenwick([-1, -2, -3])
    assert g.query(2) == -6
    assert g.range_sum(0, 1) == -3
    assert g.range_sum(1, 2) == -5
    g.update(1, 2)  # -2 变成 0
    assert g.query(2) == -4

    # 5. 连续更新后的区间和：加加减减必须对得上
    a = [10, 20, 30, 40, 50, 60]
    f = Fenwick(a)
    f.update(0, -10)
    f.update(5, 5)
    f.update(2, 100)
    f.update(2, -100)
    a[0] -= 10
    a[5] += 5
    for l in range(6):
        for r in range(l, 6):
            assert f.range_sum(l, r) == brute_range(a, l, r)

    # 6. 与暴力对拍：固定种子随机单点加 + 随机前缀 / 区间查询
    n = 37
    a = [0] * n
    f = Fenwick.zeros(n)
    seed = 24680
    for step in range(500):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        x = (seed >> 16) % 1000
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        y = (seed >> 16) % 1000
        if step % 2 == 0:
            idx = x % n
            delta = y % 200 - 100
            a[idx] += delta
            f.update(idx, delta)
        elif step % 4 == 1:
            idx = x % n
            assert f.query(idx) == brute_prefix(a, idx)
        else:
            l, r = x % n, y % n
            if l > r:
                l, r = r, l
            assert f.range_sum(l, r) == brute_range(a, l, r)
    # 最后整体核对每一个前缀
    for i in range(n):
        assert f.query(i) == brute_prefix(a, i)
    print("random total:", f.query(n - 1))

    print("all tests passed")


if __name__ == "__main__":
    main()
