/*
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
  · unite 返回「是否真的合并了」：两端已同根时返回 false，且 count 不能
    减一。判环就靠这个返回值。

手推一遍：
  5 个元素 0..4，依次 union(0,1)、union(2,3)、union(0,2)、union(0,1)：
  1. union(0,1)：两块都只有一个点，把 1 挂到 0 下，count 5 -> 4。
  2. union(2,3)：同理把 3 挂到 2 下，count 4 -> 3。
  3. union(0,2)：find(0)=0，find(2)=2，size 都是 2 不交换，把 2 挂到
     0 下；1 和 3 仍在各自父节点下，count 3 -> 2。
  4. union(0,1)：find(0) == find(1) == 0，返回 false，count 仍是 2。
  此时分量是 {0,1,2,3} 与 {4}，connected(3, 1) 为 true。

复杂度分析：
  只做路径压缩：均摊 O(log n)，不是 O(1)；单次仍可能走到较深的节点。
  只做按秩合并：O(log n)，因为树高被限制在 log n 以内。
  两者同用：均摊 O(alpha(n))，alpha 是反 Ackermann 函数，增长极慢，
    但确实不是常数，所以只能说「近似 O(1)」，不能说严格 O(1)。
  空间 O(n)：parent 加一个辅助数组（size 或 rank，二选一）。
*/
#![allow(dead_code)]

// 合并方法统一叫 unite（C++ 里 union 是关键字），四语言保持同名。
// 按大小合并 + 两趟路径压缩。size 只有根节点上有意义。
struct DsuBySize {
    parent: Vec<usize>,
    size: Vec<usize>,
    comps: usize, // 当前连通分量数
}

impl DsuBySize {
    fn new(n: usize) -> DsuBySize {
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }
        DsuBySize { parent, size: vec![1; n], comps: n }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root { // 第一趟：找根
            root = self.parent[root];
        }
        let mut cur = x;
        while self.parent[cur] != root { // 第二趟：路径上每个点直挂根
            let next = self.parent[cur];
            self.parent[cur] = root;
            cur = next;
        }
        root
    }

    // 返回 false 表示两端本来就在同一个集合，合并没有发生。
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] { // 小树挂大树
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.comps -= 1;
        true
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn count(&self) -> usize {
        self.comps
    }
}

// 按秩合并 + 路径减半。rank 是树高的上界，只有根节点有意义。
struct DsuByRank {
    parent: Vec<usize>,
    rank: Vec<usize>,
    comps: usize,
}

impl DsuByRank {
    fn new(n: usize) -> DsuByRank {
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }
        DsuByRank { parent, rank: vec![0; n], comps: n }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut cur = x;
        while self.parent[cur] != cur {
            // 挂到祖父上，路径长度减半。
            self.parent[cur] = self.parent[self.parent[cur]];
            cur = self.parent[cur];
        }
        cur
    }

    fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.rank[ra] < self.rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        // 只有两棵树一样高时，新树才会比原来高一层。
        if self.rank[ra] == self.rank[rb] {
            self.rank[ra] += 1;
        }
        self.comps -= 1;
        true
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn count(&self) -> usize {
        self.comps
    }
}

// 按顺序加无向边，返回第一条会成环的边下标，没有则返回 -1。
// 注意「边数 > V - 1 必有环」只是必要条件：边数等于 V - 1 时也可能
// 有环（别处断开），准确判断只能靠每次 unite 的返回值。
fn first_cycle_edge(n: usize, edges: &[(usize, usize)]) -> i32 {
    let mut dsu = DsuBySize::new(n);
    for (i, &(u, v)) in edges.iter().enumerate() {
        if !dsu.unite(u, v) {
            return i as i32;
        }
    }
    -1
}

// ---------------------------------------------------------------------------
// 以下均为自测代码。
// ---------------------------------------------------------------------------

// 固定种子的 64 位 LCG：四语言使用同一套参数，失败可复现。
struct Rng {
    s: u64,
}

impl Rng {
    fn new(seed: u64) -> Rng {
        Rng { s: seed }
    }

    fn next(&mut self) -> u32 {
        self.s = self
            .s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.s >> 33) as u32
    }

    fn in_range(&mut self, lo: i32, hi: i32) -> i32 {
        let span = (hi - lo + 1) as u32;
        lo + (self.next() % span) as i32
    }
}

// 从 x 走到根经过的边数，用来量树高有没有退化。
fn depth_of(parent: &[usize], x: usize) -> usize {
    let mut d = 0;
    let mut cur = x;
    while parent[cur] != cur {
        cur = parent[cur];
        d += 1;
    }
    d
}

fn main() {
    // 1. 逐步合并：连通性、count 的变化。
    let mut d = DsuBySize::new(5);
    assert_eq!(d.count(), 5);
    assert!(d.unite(0, 1));
    assert_eq!(d.count(), 4);
    assert!(d.connected(0, 1));
    assert!(!d.connected(1, 2));
    assert!(d.unite(2, 3));
    assert_eq!(d.count(), 3);
    assert!(d.unite(0, 2));
    assert!(d.connected(3, 1));
    assert_eq!(d.count(), 2);

    // 2. 重复 merge：返回 false，count 不能变（这正是判环的依据）。
    assert!(!d.unite(0, 1));
    assert_eq!(d.count(), 2);
    assert!(!d.unite(3, 0));
    assert_eq!(d.count(), 2);

    // 按秩版本跑同样的序列，结果必须一致。
    let mut r = DsuByRank::new(5);
    assert!(r.unite(0, 1));
    assert!(r.unite(2, 3));
    assert!(r.unite(0, 2));
    assert!(r.connected(3, 1));
    assert_eq!(r.count(), 2);
    assert!(!r.unite(0, 1));
    assert_eq!(r.count(), 2);

    // 3. 关键点一：两趟路径压缩把链压平。手工造 0->1->2->3->4。
    let mut chain = DsuBySize::new(5);
    for i in 0..4 {
        chain.parent[i] = i + 1;
    }
    assert_eq!(chain.find(0), 4);
    assert_eq!(chain.parent[0], 4); // 第二趟让 0 直挂根
    assert_eq!(chain.parent[1], 4);
    assert_eq!(depth_of(&chain.parent, 0), 1);

    // 关键点二：路径减半只走一趟，隔一个挂一次，高度约减半。
    let mut half = DsuByRank::new(5);
    for i in 0..4 {
        half.parent[i] = i + 1;
    }
    assert_eq!(half.find(0), 4);
    assert_eq!(half.parent[0], 2);
    assert_eq!(half.parent[2], 4);
    assert_eq!(depth_of(&half.parent, 0), 2);

    // 4. 应用：判环。
    assert_eq!(first_cycle_edge(3, &[(0, 1), (1, 2), (2, 0)]), 2);
    assert_eq!(first_cycle_edge(3, &[(0, 1), (1, 2)]), -1);
    assert_eq!(first_cycle_edge(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]), 3);
    // 边数正好等于 V - 1 = 3，却仍有环：3 号点孤立，别处三角形。
    assert_eq!(first_cycle_edge(4, &[(0, 1), (1, 2), (2, 0)]), 2);

    // 5. 顺序合并 1e5 次：不能退化成链，树高必须是对数级。
    let n = 100000;
    let mut big = DsuBySize::new(n);
    for i in 0..n - 1 {
        assert!(big.unite(i, i + 1));
    }
    assert_eq!(big.count(), 1);
    assert!(big.connected(0, n - 1));
    let mut max_depth = 0;
    for i in 0..n {
        let dp = depth_of(&big.parent, i);
        if dp > max_depth {
            max_depth = dp;
        }
    }
    assert!(max_depth <= 40, "depth by size: {}", max_depth); // 约 34

    let mut big_rank = DsuByRank::new(n);
    for i in 0..n - 1 {
        assert!(big_rank.unite(i, i + 1));
    }
    assert_eq!(big_rank.count(), 1);
    max_depth = 0;
    for i in 0..n {
        let dp = depth_of(&big_rank.parent, i);
        if dp > max_depth {
            max_depth = dp;
        }
    }
    assert!(max_depth <= 40, "depth by rank: {}", max_depth);

    // 6. 随机操作序列：count 必须始终等于 n - 成功合并次数。
    let mut gen = Rng::new(20240503);
    for _ in 0..200 {
        let size = gen.in_range(1, 50) as usize;
        let mut ops = DsuBySize::new(size);
        let mut merged = 0;
        for _ in 0..100 {
            let a = gen.in_range(0, size as i32 - 1) as usize;
            let b = gen.in_range(0, size as i32 - 1) as usize;
            if ops.unite(a, b) {
                merged += 1;
            }
            assert_eq!(ops.count(), size - merged);
        }
        // count == 1 时全图连通，任取两点必须 connected。
        if ops.count() == 1 {
            assert!(ops.connected(0, size - 1));
        }
    }

    // 7. 边界：空集合、单元素（自己和自己同根，重复合并返回 false）。
    assert_eq!(DsuBySize::new(0).count(), 0);
    assert_eq!(DsuByRank::new(0).count(), 0);
    let mut one = DsuBySize::new(1);
    assert_eq!(one.count(), 1);
    assert!(one.connected(0, 0));
    assert!(!one.unite(0, 0));
    assert_eq!(one.count(), 1);
    let mut one_rank = DsuByRank::new(1);
    assert!(one_rank.connected(0, 0));
    assert!(!one_rank.unite(0, 0));
    assert_eq!(one_rank.count(), 1);

    println!("all tests passed");
}
