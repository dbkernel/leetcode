r"""
============================================================================
线段树（Segment Tree）
分类：手写数据结构 · 树
操作复杂度：
  build      O(n)      每个节点只算一次
  update     O(log n)  单点改值，回溯 log n 个祖先
  query      O(log n)  区间和，最多拆成 O(log n) 段
  range_add  O(log n)  懒标记版本：整段覆盖时只打标记，不下钻
  range_sum  O(log n)  懒标记版本
  空间 O(4n)：下标从 1 开始，2i / 2i+1 是孩子
============================================================================

用途：
  区间查询 + 单点 / 区间修改的标准结构。面试里常以「区间和」「区间
  最值」「区间加后求和」出现，考的是分治边界与懒标记下推时机。

核心思想：
  把数组切成二叉树的区间：根管 [0, n-1]，每个节点管一段，孩子各管
  一半，叶子管一个元素。节点存这段区间的和。查询时把目标区间拆成
  若干个「被节点完整覆盖」的段相加，所以只需 O(log n) 段。单点更新
  改叶子后沿父亲一路重算。

  懒标记版本多存一个 add：整段被覆盖时只把 tree[node] 加上
  val * 区间长度、lazy[node] 加上 val，不往下走；等以后真的需要
  进入孩子时再把标记推给两个孩子。

关键点（不变量）：
  · 完全覆盖 (ql <= l && r <= qr) 立刻返回，绝不再下钻，这是
    O(log n) 的来源；只按「相交」判断会退化成 O(n)；
  · push_down 只能在「部分覆盖、必须进孩子」之前调用；整段覆盖时
    调用会把标记提前散掉，白干一趟；
  · add 是叠加：lazy[孩子] += lazy[父亲]，tree[孩子] +=
    lazy[父亲] * 孩子区间长度；写成赋值会丢掉之前的加法；
  · 下推后必须 lazy[node] = 0，否则下次进孩子会重复加一遍；
  · 孩子区间长度分别是 mid-l+1 和 r-mid，不能都乘总长度。

手推一遍：
  以 {1,2,3,4,5} 建树：
  1. 根 [0,4] 存 15，左 [0,2] 存 6，右 [3,4] 存 9，
     [0,1] 存 3，叶子分别是 1 2 3 4 5；
  2. query(1,3)：根不被完全覆盖，左 [0,2] 只取 [1,2] 得 5，
     右 [3,4] 只取 3 得 4，合计 9；
  3. update(2,10)：叶子 3 -> 10，回溯 [0,2] 变 13、根变 22。

  懒标记版本以 {0,0,0,0} 为例，先 range_add(0,2,5)，再
  range_add(1,3,2)：
  1. 第一次：左 [0,1] 被整段覆盖，tree 加 5*2=10、lazy 加 5；
     右 [2,3] 只覆盖叶子 2，tree 加 5；
  2. 第二次：[0,1] 部分覆盖，先把 lazy=5 推给两个叶子（各 +5、
     清零），再给叶子 1 加 2 得 7；右半边同理，叶子 2 得 7、
     叶子 3 得 2；
  3. 四个叶子 5 7 7 2，区间和 21，正好等于 5*3 + 2*3。

复杂度分析：
  时间：build O(n)；update / query / range_add / range_sum
        都是 O(log n)，因为每层最多访问常数个节点。
  空间：tree 开 4n，lazy 再开 4n，共 O(n)。
"""


class SegmentTree:
    """单点改值 + 区间和"""

    def __init__(self, a):
        assert a  # 空数组没有区间可查，直接拒绝
        self.n = len(a)
        self.tree = [0] * (4 * len(a) + 4)  # 4n 够用
        self._build(1, 0, self.n - 1, a)

    def _build(self, node, l, r, a):
        if l == r:
            self.tree[node] = a[l]
            return
        mid = l + (r - l) // 2  # 防溢出写法
        self._build(node * 2, l, mid, a)
        self._build(node * 2 + 1, mid + 1, r, a)
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1]

    def update(self, idx, value):
        """单点赋值：把下标 idx 的值改成 value"""
        self._update(1, 0, self.n - 1, idx, value)

    def _update(self, node, l, r, idx, value):
        if l == r:
            self.tree[node] = value
            return
        mid = l + (r - l) // 2
        if idx <= mid:
            self._update(node * 2, l, mid, idx, value)
        else:
            self._update(node * 2 + 1, mid + 1, r, idx, value)
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1]

    def query(self, ql, qr):
        """区间和 [ql, qr]；ql > qr 时返回 0"""
        if ql > qr:
            return 0
        return self._query(1, 0, self.n - 1, ql, qr)

    def _query(self, node, l, r, ql, qr):
        if ql <= l and r <= qr:
            return self.tree[node]  # 完全覆盖，直接返回
        mid = l + (r - l) // 2
        s = 0
        if ql <= mid:
            s += self._query(node * 2, l, mid, ql, qr)
        if qr > mid:
            s += self._query(node * 2 + 1, mid + 1, r, ql, qr)
        return s


class LazySegmentTree:
    """区间加 + 区间和（懒标记）"""

    def __init__(self, a):
        assert a
        self.n = len(a)
        self.tree = [0] * (4 * len(a) + 4)
        self.lazy = [0] * (4 * len(a) + 4)  # 待下推到孩子的加法
        self._build(1, 0, self.n - 1, a)

    def _build(self, node, l, r, a):
        self.lazy[node] = 0
        if l == r:
            self.tree[node] = a[l]
            return
        mid = l + (r - l) // 2
        self._build(node * 2, l, mid, a)
        self._build(node * 2 + 1, mid + 1, r, a)
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1]

    def _push_down(self, node, l, r):
        """把 node 的标记推给两个孩子，再清空自己"""
        if self.lazy[node] == 0 or l == r:
            return  # 叶子没有孩子可推
        mid = l + (r - l) // 2
        lc = node * 2
        rc = node * 2 + 1
        # 叠加：新加法叠在旧标记上面
        self.lazy[lc] += self.lazy[node]
        self.tree[lc] += self.lazy[node] * (mid - l + 1)
        self.lazy[rc] += self.lazy[node]
        self.tree[rc] += self.lazy[node] * (r - mid)
        self.lazy[node] = 0  # 不清零会在下次下推时重复加

    def range_add(self, ql, qr, val):
        if ql > qr:
            return
        self._add(1, 0, self.n - 1, ql, qr, val)

    def _add(self, node, l, r, ql, qr, val):
        if ql <= l and r <= qr:
            # 整段覆盖：只改本节点的和与标记，绝不往下走
            self.tree[node] += val * (r - l + 1)
            self.lazy[node] += val
            return
        self._push_down(node, l, r)  # 只有需要进孩子时才下推
        mid = l + (r - l) // 2
        if ql <= mid:
            self._add(node * 2, l, mid, ql, qr, val)
        if qr > mid:
            self._add(node * 2 + 1, mid + 1, r, ql, qr, val)
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1]

    def range_sum(self, ql, qr):
        if ql > qr:
            return 0
        return self._sum(1, 0, self.n - 1, ql, qr)

    def _sum(self, node, l, r, ql, qr):
        if ql <= l and r <= qr:
            return self.tree[node]
        # 查询也要下推：孩子的 tree 还没包含父节点挂着的标记
        self._push_down(node, l, r)
        mid = l + (r - l) // 2
        s = 0
        if ql <= mid:
            s += self._sum(node * 2, l, mid, ql, qr)
        if qr > mid:
            s += self._sum(node * 2 + 1, mid + 1, r, ql, qr)
        return s


def brute_sum(a, l, r):
    s = 0
    for i in range(l, r + 1):
        s += a[i]
    return s


def main():
    # 1. 小数组的手算用例：全区间 / 子区间 / 单点
    a = [1, 2, 3, 4, 5]
    st = SegmentTree(a)
    assert st.query(0, 4) == 15
    assert st.query(1, 3) == 9
    assert st.query(2, 2) == 3
    assert st.query(0, 0) == 1
    assert st.query(4, 4) == 5
    assert st.query(3, 2) == 0  # 空区间
    st.update(2, 10)
    assert st.query(0, 4) == 22
    assert st.query(1, 3) == 16
    assert st.query(2, 2) == 10
    print("after update total:", st.query(0, 4))

    # 2. n = 1：区间就是那一个点
    st = SegmentTree([7])
    assert st.query(0, 0) == 7
    st.update(0, 3)
    assert st.query(0, 0) == 3
    assert st.query(0, -1) == 0

    # 3. 负数与全 0
    st = SegmentTree([0, 0, 0, 0])
    assert st.query(0, 3) == 0
    st.update(1, -5)
    st.update(3, -7)
    assert st.query(0, 3) == -12
    assert st.query(1, 1) == -5
    assert st.query(1, 2) == -5

    # 4. 与暴力对拍：固定种子随机单点修改 + 随机区间查询
    n = 40
    a = []
    seed = 987654321
    for _ in range(n):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        a.append(((seed >> 16) % 201) - 100)
    st = SegmentTree(a)
    assert st.query(0, n - 1) == brute_sum(a, 0, n - 1)
    for step in range(300):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        x = (seed >> 16) % 1000
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        y = (seed >> 16) % 1000
        if step % 3 == 0:
            idx = x % n
            value = y % 401 - 200
            a[idx] = value  # 影子数组，只用于暴力对照
            st.update(idx, value)
        else:
            l, r = x % n, y % n
            if l > r:
                l, r = r, l
            assert st.query(l, r) == brute_sum(a, l, r)
    print("random point-update total:", st.query(0, n - 1))

    # 5. 懒标记版本：区间加 + 区间和，与暴力对拍
    n = 30
    a = []
    seed = 13579
    for _ in range(n):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        a.append(((seed >> 16) % 41) - 20)
    st = LazySegmentTree(a)
    b = a[:]  # 暴力的影子数组
    assert st.range_sum(0, n - 1) == brute_sum(b, 0, n - 1)
    for step in range(300):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        x = (seed >> 16) % 1000
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        y = (seed >> 16) % 1000
        l, r = x % n, y % n
        if l > r:
            l, r = r, l
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        val = ((seed >> 16) % 21) - 10
        if step % 2 == 0:
            st.range_add(l, r, val)
            for i in range(l, r + 1):
                b[i] += val
        else:
            assert st.range_sum(l, r) == brute_sum(b, l, r)
    # 最后整体核对一次，确保懒标记没有漏推
    assert st.range_sum(0, n - 1) == brute_sum(b, 0, n - 1)
    print("lazy total:", st.range_sum(0, n - 1))

    # 6. 懒标记边界：n = 1、整段加、负值、l == r
    st = LazySegmentTree([5])
    st.range_add(0, 0, -3)
    assert st.range_sum(0, 0) == 2
    st.range_add(0, 0, 10)
    assert st.range_sum(0, 0) == 12

    t2 = LazySegmentTree([1, 2, 3, 4])
    t2.range_add(0, 3, 5)  # 整段覆盖根，只打标记
    assert t2.range_sum(0, 3) == 30
    t2.range_add(1, 2, -4)  # 此时必须先把根的标记推下去
    assert t2.range_sum(0, 3) == 22
    assert t2.range_sum(1, 1) == 3
    assert t2.range_sum(2, 2) == 4
    assert t2.range_sum(0, 0) == 6
    assert t2.range_sum(3, 3) == 9

    print("all tests passed")


if __name__ == "__main__":
    main()
