#![allow(dead_code)]
/*
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

Rust 实现选择：
  这里是纯数组结构（tree / lazy 用 0 基下标存 1 基节点），没有链式
  节点，所以不需要 Option<Box<Node>>，Vec 当裸数组即可。
*/

// ---------- 单点改值 + 区间和 ----------
struct SegmentTree {
    n: usize,
    tree: Vec<i32>, // 4n 够用：区间二分的节点数上界
}

impl SegmentTree {
    fn new(a: &[i32]) -> Self {
        assert!(!a.is_empty()); // 空数组没有区间可查，直接拒绝
        let n = a.len();
        let mut st = SegmentTree { n, tree: vec![0; 4 * n + 4] };
        st.build(1, 0, n - 1, a);
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize, a: &[i32]) {
        if l == r {
            self.tree[node] = a[l];
            return;
        }
        let mid = l + (r - l) / 2; // 防溢出写法
        self.build(node * 2, l, mid, a);
        self.build(node * 2 + 1, mid + 1, r, a);
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    // 单点赋值：把下标 idx 的值改成 value
    fn update(&mut self, idx: usize, value: i32) {
        self.update_rec(1, 0, self.n - 1, idx, value);
    }

    fn update_rec(&mut self, node: usize, l: usize, r: usize, idx: usize,
                  value: i32) {
        if l == r {
            self.tree[node] = value;
            return;
        }
        let mid = l + (r - l) / 2;
        if idx <= mid {
            self.update_rec(node * 2, l, mid, idx, value);
        } else {
            self.update_rec(node * 2 + 1, mid + 1, r, idx, value);
        }
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    // 区间和 [ql, qr]；ql > qr 时返回 0
    fn query(&self, ql: usize, qr: usize) -> i32 {
        if ql > qr {
            return 0;
        }
        self.query_rec(1, 0, self.n - 1, ql, qr)
    }

    fn query_rec(&self, node: usize, l: usize, r: usize, ql: usize,
                 qr: usize) -> i32 {
        if ql <= l && r <= qr {
            return self.tree[node]; // 完全覆盖，直接返回
        }
        let mid = l + (r - l) / 2;
        let mut s = 0;
        if ql <= mid {
            s += self.query_rec(node * 2, l, mid, ql, qr);
        }
        if qr > mid {
            s += self.query_rec(node * 2 + 1, mid + 1, r, ql, qr);
        }
        s
    }
}

// ---------- 区间加 + 区间和（懒标记） ----------
struct LazySegmentTree {
    n: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>, // 待下推到孩子的加法
}

impl LazySegmentTree {
    fn new(a: &[i32]) -> Self {
        assert!(!a.is_empty());
        let n = a.len();
        let mut st = LazySegmentTree {
            n,
            tree: vec![0; 4 * n + 4],
            lazy: vec![0; 4 * n + 4],
        };
        st.build(1, 0, n - 1, a);
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize, a: &[i32]) {
        self.lazy[node] = 0;
        if l == r {
            self.tree[node] = a[l];
            return;
        }
        let mid = l + (r - l) / 2;
        self.build(node * 2, l, mid, a);
        self.build(node * 2 + 1, mid + 1, r, a);
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    // 把 node 的标记推给两个孩子，再清空自己
    fn push_down(&mut self, node: usize, l: usize, r: usize) {
        if self.lazy[node] == 0 || l == r {
            return; // 叶子没有孩子可推
        }
        let mid = l + (r - l) / 2;
        let add = self.lazy[node];
        self.lazy[node * 2] += add; // 叠加：新加法叠在旧标记上面
        self.tree[node * 2] += add * (mid - l + 1) as i32;
        self.lazy[node * 2 + 1] += add;
        self.tree[node * 2 + 1] += add * (r - mid) as i32;
        self.lazy[node] = 0; // 不清零会在下次下推时重复加
    }

    fn range_add(&mut self, ql: usize, qr: usize, val: i32) {
        if ql > qr {
            return;
        }
        self.add_rec(1, 0, self.n - 1, ql, qr, val);
    }

    fn add_rec(&mut self, node: usize, l: usize, r: usize, ql: usize,
               qr: usize, val: i32) {
        if ql <= l && r <= qr {
            // 整段覆盖：只改本节点的和与标记，绝不往下走
            self.tree[node] += val * (r - l + 1) as i32;
            self.lazy[node] += val;
            return;
        }
        self.push_down(node, l, r); // 只有需要进孩子时才下推
        let mid = l + (r - l) / 2;
        if ql <= mid {
            self.add_rec(node * 2, l, mid, ql, qr, val);
        }
        if qr > mid {
            self.add_rec(node * 2 + 1, mid + 1, r, ql, qr, val);
        }
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    fn range_sum(&mut self, ql: usize, qr: usize) -> i32 {
        if ql > qr {
            return 0;
        }
        self.sum_rec(1, 0, self.n - 1, ql, qr)
    }

    fn sum_rec(&mut self, node: usize, l: usize, r: usize, ql: usize,
               qr: usize) -> i32 {
        if ql <= l && r <= qr {
            return self.tree[node];
        }
        // 查询也要下推：孩子的 tree 还没包含父节点挂着的标记
        self.push_down(node, l, r);
        let mid = l + (r - l) / 2;
        let mut s = 0;
        if ql <= mid {
            s += self.sum_rec(node * 2, l, mid, ql, qr);
        }
        if qr > mid {
            s += self.sum_rec(node * 2 + 1, mid + 1, r, ql, qr);
        }
        s
    }
}

fn brute_sum(a: &[i32], l: usize, r: usize) -> i32 {
    let mut s = 0;
    for i in l..=r {
        s += a[i];
    }
    s
}

fn main() {
    // 1. 小数组的手算用例：全区间 / 子区间 / 单点
    {
        let a = [1, 2, 3, 4, 5];
        let mut st = SegmentTree::new(&a);
        assert_eq!(st.query(0, 4), 15);
        assert_eq!(st.query(1, 3), 9);
        assert_eq!(st.query(2, 2), 3);
        assert_eq!(st.query(0, 0), 1);
        assert_eq!(st.query(4, 4), 5);
        assert_eq!(st.query(3, 2), 0); // 空区间
        st.update(2, 10);
        assert_eq!(st.query(0, 4), 22);
        assert_eq!(st.query(1, 3), 16);
        assert_eq!(st.query(2, 2), 10);
        println!("after update total: {}", st.query(0, 4));
    }

    // 2. n = 1：区间就是那一个点
    {
        let mut st = SegmentTree::new(&[7]);
        assert_eq!(st.query(0, 0), 7);
        st.update(0, 3);
        assert_eq!(st.query(0, 0), 3);
    }

    // 3. 负数与全 0
    {
        let mut st = SegmentTree::new(&[0, 0, 0, 0]);
        assert_eq!(st.query(0, 3), 0);
        st.update(1, -5);
        st.update(3, -7);
        assert_eq!(st.query(0, 3), -12);
        assert_eq!(st.query(1, 1), -5);
        assert_eq!(st.query(1, 2), -5);
    }

    // 4. 与暴力对拍：固定种子随机单点修改 + 随机区间查询
    {
        let n = 40;
        let mut a = vec![0; n];
        let mut seed: u32 = 987654321;
        for x in a.iter_mut() {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            *x = ((seed >> 16) % 201) as i32 - 100;
        }
        let mut st = SegmentTree::new(&a);
        assert_eq!(st.query(0, n - 1), brute_sum(&a, 0, n - 1));
        for step in 0..300 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let x = ((seed >> 16) % 1000) as usize;
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let y = ((seed >> 16) % 1000) as usize;
            if step % 3 == 0 {
                let idx = x % n;
                let value = (y % 401) as i32 - 200;
                a[idx] = value; // 影子数组，只用于暴力对照
                st.update(idx, value);
            } else {
                let (l, r) = if x % n <= y % n {
                    (x % n, y % n)
                } else {
                    (y % n, x % n)
                };
                assert_eq!(st.query(l, r), brute_sum(&a, l, r));
            }
        }
        println!("random point-update total: {}", st.query(0, n - 1));
    }

    // 5. 懒标记版本：区间加 + 区间和，与暴力对拍
    {
        let n = 30;
        let mut a = vec![0; n];
        let mut seed: u32 = 13579;
        for x in a.iter_mut() {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            *x = ((seed >> 16) % 41) as i32 - 20;
        }
        let mut st = LazySegmentTree::new(&a);
        let mut b = a.clone(); // 暴力的影子数组
        assert_eq!(st.range_sum(0, n - 1), brute_sum(&b, 0, n - 1));
        for step in 0..300 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let x = ((seed >> 16) % 1000) as usize;
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let y = ((seed >> 16) % 1000) as usize;
            let (l, r) = if x % n <= y % n {
                (x % n, y % n)
            } else {
                (y % n, x % n)
            };
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let val = ((seed >> 16) % 21) as i32 - 10;
            if step % 2 == 0 {
                st.range_add(l, r, val);
                for i in l..=r {
                    b[i] += val;
                }
            } else {
                assert_eq!(st.range_sum(l, r), brute_sum(&b, l, r));
            }
        }
        // 最后整体核对一次，确保懒标记没有漏推
        assert_eq!(st.range_sum(0, n - 1), brute_sum(&b, 0, n - 1));
        println!("lazy total: {}", st.range_sum(0, n - 1));
    }

    // 6. 懒标记边界：n = 1、整段加、负值、l == r
    {
        let mut st = LazySegmentTree::new(&[5]);
        st.range_add(0, 0, -3);
        assert_eq!(st.range_sum(0, 0), 2);
        st.range_add(0, 0, 10);
        assert_eq!(st.range_sum(0, 0), 12);

        let mut t2 = LazySegmentTree::new(&[1, 2, 3, 4]);
        t2.range_add(0, 3, 5); // 整段覆盖根，只打标记
        assert_eq!(t2.range_sum(0, 3), 30);
        t2.range_add(1, 2, -4); // 此时必须先把根的标记推下去
        assert_eq!(t2.range_sum(0, 3), 22);
        assert_eq!(t2.range_sum(1, 1), 3);
        assert_eq!(t2.range_sum(2, 2), 4);
        assert_eq!(t2.range_sum(0, 0), 6);
        assert_eq!(t2.range_sum(3, 3), 9);
    }

    println!("all tests passed");
}
