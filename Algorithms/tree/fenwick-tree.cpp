/*
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
*/
#include <cassert>
#include <iostream>
#include <vector>

struct Fenwick {
    int n;
    std::vector<int> tree;  // 1 基下标，tree[i] 管 (i-lowbit(i), i]

    explicit Fenwick(int n_) : n(n_), tree(n_ + 1, 0) {}

    explicit Fenwick(const std::vector<int>& a)
        : n(static_cast<int>(a.size())), tree(a.size() + 1, 0) {
        // O(n) 建树：i 从小到大，tree[i] 此时已经是自己那段的真和
        for (int i = 1; i <= n; ++i) {
            tree[i] += a[i - 1];
            int j = i + lowbit(i);
            if (j <= n) tree[j] += tree[i];
        }
    }

    static int lowbit(int x) { return x & (-x); }

    // 单点加：idx 是 0 基下标，delta 可正可负
    void update(int idx, int delta) {
        for (int i = idx + 1; i <= n; i += lowbit(i)) tree[i] += delta;
    }

    // 前缀和 [0, idx]
    int query(int idx) const {
        int s = 0;
        for (int i = idx + 1; i > 0; i -= lowbit(i)) s += tree[i];
        return s;
    }

    // 区间和 [l, r]，l > r 时返回 0
    int range_sum(int l, int r) const {
        if (l > r) return 0;
        int s = query(r);
        if (l > 0) s -= query(l - 1);  // l=0 时没有前缀要减
        return s;
    }
};

static int brute_prefix(const std::vector<int>& a, int idx) {
    int s = 0;
    for (int i = 0; i <= idx; ++i) s += a[i];
    return s;
}

static int brute_range(const std::vector<int>& a, int l, int r) {
    int s = 0;
    for (int i = l; i <= r; ++i) s += a[i];
    return s;
}

int main() {
    // 1. 基本用例：前缀和 / 区间和
    {
        std::vector<int> a = {1, 2, 3, 4, 5};
        Fenwick f(a);
        assert(f.query(0) == 1);
        assert(f.query(2) == 6);
        assert(f.query(4) == 15);
        assert(f.range_sum(1, 3) == 9);
        assert(f.range_sum(0, 4) == 15);
        assert(f.range_sum(2, 2) == 3);
        assert(f.range_sum(3, 2) == 0);  // 空区间
        std::cout << "total: " << f.query(4) << "\n";
    }

    // 2. O(n) 建树必须与逐个 update 的结果完全一致
    {
        std::vector<int> a = {4, -3, 7, 0, 2, -8, 5, 1, 9, -6, 3};
        Fenwick fast(a);
        Fenwick slow(static_cast<int>(a.size()));
        for (size_t i = 0; i < a.size(); ++i) {
            slow.update(static_cast<int>(i), a[i]);
        }
        for (int i = 0; i < static_cast<int>(a.size()); ++i) {
            assert(fast.query(i) == slow.query(i));
            assert(fast.query(i) == brute_prefix(a, i));
        }
    }

    // 3. n = 1
    {
        Fenwick f(std::vector<int>{7});
        assert(f.query(0) == 7);
        assert(f.range_sum(0, 0) == 7);
        f.update(0, 3);
        assert(f.query(0) == 10);
        f.update(0, -10);
        assert(f.query(0) == 0);
    }

    // 4. 全 0 与负数
    {
        Fenwick f(std::vector<int>{0, 0, 0, 0});
        assert(f.query(3) == 0);
        assert(f.range_sum(0, 3) == 0);
        f.update(2, 5);
        assert(f.range_sum(0, 3) == 5);

        Fenwick g(std::vector<int>{-1, -2, -3});
        assert(g.query(2) == -6);
        assert(g.range_sum(0, 1) == -3);
        assert(g.range_sum(1, 2) == -5);
        g.update(1, 2);  // -2 变成 0
        assert(g.query(2) == -4);
    }

    // 5. 连续更新后的区间和：加加减减必须对得上
    {
        std::vector<int> a = {10, 20, 30, 40, 50, 60};
        Fenwick f(a);
        f.update(0, -10);
        f.update(5, 5);
        f.update(2, 100);
        f.update(2, -100);
        a[0] -= 10;
        a[5] += 5;
        for (int l = 0; l < 6; ++l) {
            for (int r = l; r < 6; ++r) {
                assert(f.range_sum(l, r) == brute_range(a, l, r));
            }
        }
    }

    // 6. 与暴力对拍：固定种子随机单点加 + 随机前缀 / 区间查询
    {
        const int n = 37;
        std::vector<int> a(n, 0);
        Fenwick f(n);
        unsigned int seed = 24680u;
        for (int step = 0; step < 500; ++step) {
            seed = seed * 1103515245u + 12345u;
            int x = static_cast<int>((seed >> 16) % 1000);
            seed = seed * 1103515245u + 12345u;
            int y = static_cast<int>((seed >> 16) % 1000);
            if (step % 2 == 0) {
                int idx = x % n;
                int delta = y % 200 - 100;
                a[idx] += delta;
                f.update(idx, delta);
            } else {
                if (step % 4 == 1) {
                    int idx = x % n;
                    assert(f.query(idx) == brute_prefix(a, idx));
                } else {
                    int l = x % n;
                    int r = y % n;
                    if (l > r) {
                        int tmp = l;
                        l = r;
                        r = tmp;
                    }
                    assert(f.range_sum(l, r) == brute_range(a, l, r));
                }
            }
        }
        // 最后整体核对每一个前缀
        for (int i = 0; i < n; ++i) {
            assert(f.query(i) == brute_prefix(a, i));
        }
        std::cout << "random total: " << f.query(n - 1) << "\n";
    }

    std::cout << "all tests passed\n";
    return 0;
}
