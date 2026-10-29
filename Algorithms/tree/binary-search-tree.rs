#![allow(dead_code)]
/*
============================================================================
二叉搜索树（Binary Search Tree）
分类：手写数据结构 · 树
操作复杂度：
  insert    O(h)  沿一条路径下降；重复值只把 cnt 加一
  find      O(h)
  erase     O(h)  双孩子要沿右子树最左一路找后继
  min/max   O(h)
  height    O(n)  递归量一遍（缓存子树高度可做到 O(1)）
  size      O(n)  递归把 cnt 累加（维护计数器可做到 O(1)）
  h 为树高，随机形态期望 O(log n)，退化成链时 O(n)
============================================================================

用途：
  「有序 + 动态插删」的最小可用结构，也是 AVL / 红黑树的原型。
  面试常考：插入、查找、删除节点（尤其双孩子）、验证 BST、第 k 小。

核心思想：
  对每个节点，左子树所有值都比它小，右子树所有值都比它大，于是查找
  每次都能砍掉一半子树。插入和删除都只沿一条从根到叶的路径走，改动
  只发生在路径上。删除双孩子节点时，把中序后继（右子树最小值）搬到
  当前位置，问题就退化成「删一个最多只有一个孩子的节点」。

关键点（不变量）：
  · 重复值策略选「计数」：相等时只把 cnt 加一，不新建节点，因此
    每个不同的值在树里恰好占一个节点，中序遍历按 cnt 重复输出；
  · 双孩子删除必须用中序后继：它是右子树最小值，天然大于左子树
    全部、小于右子树其余全部；且它没有左孩子，物理摘除退化成
    「至多一个孩子」的最简情况。随便挑一个节点会破坏 BST 有序性；
  · cnt > 1 时删除只减计数、绝不动结构，否则会把同值的其它出现
    一起删掉；
  · erase 找不到值时原树必须原样返回，不能顺手改指针。

手推一遍：
  依次插入 50 30 70 20 40 60 80，得到：

            50
          /    \
        30      70
       /  \    /  \
      20  40  60  80

  1. 插入 40 时走 50 -> 30 -> 40，40 比 30 大且 30 的右孩子为空，
     挂成 30 的右孩子；
  2. 删除 50：它有两个孩子，进右子树一路向左找到 60，把 60 搬到
     根，70 的左孩子置空；中序仍是 20 30 40 60 70 80，高度 3；
  3. 再插入一个 40：走到已有节点 40，只把它的 cnt 变成 2，节点数
     不变，中序变成 20 30 40 40 60 70 80。

复杂度分析：
  时间 O(h)：insert / find / erase / min / max 都只走一条路径；
        顺序插入会退化成链，h = n，这就是要 AVL 的原因。
  空间 O(h)：递归栈；节点本身 O(n)。

Rust 实现选择：
  用索引数组而不是 Option<Box<Node>>：删除要同时改父子的 left /
  right，索引下标能避开借用检查的连环冲突；被摘掉的槽位压进 free
  复用，等价于 C++ 的 delete，又不用 unsafe。
*/

const NIL: usize = usize::MAX;

struct Node {
    val: i32,
    cnt: i32, // 该值出现次数，重复值不新建节点
    left: usize,
    right: usize,
}

struct Bst {
    nodes: Vec<Node>,
    free: Vec<usize>,
    root: usize,
}

impl Bst {
    fn new() -> Self {
        Bst { nodes: Vec::new(), free: Vec::new(), root: NIL }
    }

    fn new_node(&mut self, v: i32) -> usize {
        if let Some(id) = self.free.pop() {
            self.nodes[id] = Node { val: v, cnt: 1, left: NIL, right: NIL };
            id
        } else {
            self.nodes.push(Node { val: v, cnt: 1, left: NIL, right: NIL });
            self.nodes.len() - 1
        }
    }

    // 返回 true 表示新建了节点；false 表示只是 cnt 加一
    fn insert(&mut self, v: i32) -> bool {
        let mut created = false;
        self.root = self.insert_rec(self.root, v, &mut created);
        created
    }

    fn insert_rec(&mut self, t: usize, v: i32, created: &mut bool) -> usize {
        if t == NIL {
            *created = true;
            return self.new_node(v);
        }
        if v < self.nodes[t].val {
            let l = self.nodes[t].left;
            self.nodes[t].left = self.insert_rec(l, v, created);
        } else if v > self.nodes[t].val {
            let r = self.nodes[t].right;
            self.nodes[t].right = self.insert_rec(r, v, created);
        } else {
            self.nodes[t].cnt += 1; // 重复值只计数，保持「一值一节点」
        }
        t
    }

    fn find(&self, v: i32) -> bool {
        let mut t = self.root;
        while t != NIL {
            if v < self.nodes[t].val {
                t = self.nodes[t].left;
            } else if v > self.nodes[t].val {
                t = self.nodes[t].right;
            } else {
                return true;
            }
        }
        false
    }

    // 返回该值出现次数，不存在返回 0
    fn count(&self, v: i32) -> i32 {
        let mut t = self.root;
        while t != NIL {
            if v < self.nodes[t].val {
                t = self.nodes[t].left;
            } else if v > self.nodes[t].val {
                t = self.nodes[t].right;
            } else {
                return self.nodes[t].cnt;
            }
        }
        0
    }

    fn min_val(&self) -> Option<i32> {
        if self.root == NIL {
            return None;
        }
        let mut t = self.root;
        while self.nodes[t].left != NIL {
            t = self.nodes[t].left;
        }
        Some(self.nodes[t].val)
    }

    fn max_val(&self) -> Option<i32> {
        if self.root == NIL {
            return None;
        }
        let mut t = self.root;
        while self.nodes[t].right != NIL {
            t = self.nodes[t].right;
        }
        Some(self.nodes[t].val)
    }

    // 摘掉子树里最小的节点，返回 (新子树根, 被摘下的节点)
    fn pop_min(&mut self, t: usize) -> (usize, usize) {
        if self.nodes[t].left == NIL {
            return (self.nodes[t].right, t);
        }
        let l = self.nodes[t].left;
        let (nl, s) = self.pop_min(l);
        self.nodes[t].left = nl;
        (t, s)
    }

    // 返回 true 表示确实删掉了一次出现
    fn erase(&mut self, v: i32) -> bool {
        let mut removed = false;
        self.root = self.erase_rec(self.root, v, &mut removed);
        removed
    }

    fn erase_rec(&mut self, t: usize, v: i32, removed: &mut bool) -> usize {
        if t == NIL {
            return NIL; // 空树删除：原样返回，不算删除成功
        }
        if v < self.nodes[t].val {
            let l = self.nodes[t].left;
            self.nodes[t].left = self.erase_rec(l, v, removed);
            return t;
        }
        if v > self.nodes[t].val {
            let r = self.nodes[t].right;
            self.nodes[t].right = self.erase_rec(r, v, removed);
            return t;
        }
        *removed = true;
        if self.nodes[t].cnt > 1 {
            self.nodes[t].cnt -= 1; // 还有同值的其它出现，只减计数
            return t;
        }
        if self.nodes[t].left == NIL {
            let r = self.nodes[t].right;
            self.free.push(t);
            return r;
        }
        if self.nodes[t].right == NIL {
            let l = self.nodes[t].left;
            self.free.push(t);
            return l;
        }
        // 双孩子：中序后继没有左孩子，摘除它只需接上它的右孩子
        let r = self.nodes[t].right;
        let (nr, s) = self.pop_min(r);
        self.nodes[t].right = nr;
        self.nodes[t].val = self.nodes[s].val;
        self.nodes[t].cnt = self.nodes[s].cnt; // 计数整份搬上来
        self.free.push(s);
        t
    }

    fn height(&self) -> i32 {
        self.height_rec(self.root)
    }

    fn height_rec(&self, t: usize) -> i32 {
        if t == NIL {
            return 0;
        }
        let l = self.height_rec(self.nodes[t].left);
        let r = self.height_rec(self.nodes[t].right);
        if l > r { l + 1 } else { r + 1 }
    }

    fn size(&self) -> i32 {
        self.size_rec(self.root)
    }

    fn size_rec(&self, t: usize) -> i32 {
        if t == NIL {
            return 0;
        }
        self.nodes[t].cnt + self.size_rec(self.nodes[t].left)
            + self.size_rec(self.nodes[t].right)
    }

    fn node_count(&self) -> i32 {
        self.node_count_rec(self.root)
    }

    fn node_count_rec(&self, t: usize) -> i32 {
        if t == NIL {
            return 0;
        }
        1 + self.node_count_rec(self.nodes[t].left)
            + self.node_count_rec(self.nodes[t].right)
    }

    // 中序：每个值按 cnt 重复输出，所以整体非递减
    fn inorder(&self) -> Vec<i32> {
        let mut out = Vec::new();
        self.inorder_rec(self.root, &mut out);
        out
    }

    fn inorder_rec(&self, t: usize, out: &mut Vec<i32>) {
        if t == NIL {
            return;
        }
        self.inorder_rec(self.nodes[t].left, out);
        for _ in 0..self.nodes[t].cnt {
            out.push(self.nodes[t].val);
        }
        self.inorder_rec(self.nodes[t].right, out);
    }

    // 校验 BST 不变量：严格区间夹逼 + cnt >= 1
    fn check(&self) -> bool {
        self.check_rec(self.root, i64::MIN, i64::MAX)
    }

    fn check_rec(&self, t: usize, lo: i64, hi: i64) -> bool {
        if t == NIL {
            return true;
        }
        if self.nodes[t].cnt < 1 {
            return false;
        }
        let v = self.nodes[t].val as i64;
        if !(lo < v && v < hi) {
            return false;
        }
        self.check_rec(self.nodes[t].left, lo, v)
            && self.check_rec(self.nodes[t].right, v, hi)
    }
}

fn print_vec(v: &[i32]) {
    print!("inorder: [");
    for (i, x) in v.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", x);
    }
    println!("]");
}

fn main() {
    // 1. 正常用例：插入、查找、极值、高度、中序
    let mut t = Bst::new();
    for v in [50, 30, 70, 20, 40, 60, 80] {
        assert!(t.insert(v));
    }
    assert_eq!(t.size(), 7);
    assert_eq!(t.node_count(), 7);
    assert_eq!(t.height(), 3);
    assert!(t.find(40));
    assert!(!t.find(41));
    assert_eq!(t.min_val(), Some(20));
    assert_eq!(t.max_val(), Some(80));
    assert!(t.check());
    assert_eq!(t.inorder(), vec![20, 30, 40, 50, 60, 70, 80]);
    print_vec(&t.inorder());

    // 2. 重复值：只加计数，不新建节点
    let mut t = Bst::new();
    t.insert(40);
    assert!(!t.insert(40));
    assert!(!t.insert(40));
    assert_eq!(t.count(40), 3);
    assert_eq!(t.node_count(), 1);
    assert_eq!(t.size(), 3);
    assert!(t.erase(40));
    assert_eq!(t.count(40), 2);
    assert!(t.find(40));
    assert!(t.erase(40));
    assert!(t.erase(40));
    assert!(!t.find(40));
    assert!(!t.erase(40)); // 已经没了，删除失败
    assert_eq!(t.node_count(), 0);
    assert_eq!(t.height(), 0);

    // 3. 删除叶子 / 单孩子 / 根 / 双孩子
    let mut t = Bst::new();
    for v in [50, 30, 70, 20, 40, 60, 80] {
        t.insert(v);
    }
    assert!(t.erase(20)); // 叶子
    assert!(!t.find(20));
    assert!(t.check());
    assert!(t.erase(60));
    assert!(t.erase(70)); // 70 此时只有右孩子 80
    assert!(t.find(80));
    assert!(t.check());
    assert!(t.erase(50)); // 双孩子，后继 80 顶上
    assert!(t.find(80));
    assert!(t.check());
    assert_eq!(t.inorder(), vec![30, 40, 80]);
    // 80 上位后左边还挂着 30 -> 40，所以高度是 3
    assert_eq!(t.height(), 3);

    // 4. 单孩子删根：根被唯一孩子顶替
    let mut t = Bst::new();
    t.insert(10);
    t.insert(5);
    t.insert(3);
    assert!(t.erase(10));
    assert_eq!(t.inorder(), vec![3, 5]);
    assert!(t.check());
    assert_eq!(t.height(), 2);
    assert_eq!(t.min_val(), Some(3));
    assert_eq!(t.max_val(), Some(5));

    // 5. 空树删除：返回 false，树保持为空
    let mut t = Bst::new();
    assert!(!t.erase(1));
    assert!(t.inorder().is_empty());
    assert_eq!(t.height(), 0);
    assert_eq!(t.min_val(), None);
    assert_eq!(t.max_val(), None);
    assert_eq!(t.size(), 0);
    assert!(t.check());

    // 6. 双孩子 + 后继自身带重复计数：计数必须整份搬上来
    let mut t = Bst::new();
    t.insert(10);
    t.insert(5);
    t.insert(20);
    t.insert(20);
    assert_eq!(t.count(20), 2);
    assert!(t.erase(10)); // 后继是 20(cnt=2)
    assert_eq!(t.inorder(), vec![5, 20, 20]);
    assert_eq!(t.count(20), 2);
    assert_eq!(t.node_count(), 2);
    assert!(t.check());

    // 7. 中序严格递增（无重复时）
    let mut t = Bst::new();
    for i in 0..64 {
        t.insert(i * 3 - 50);
    }
    let seq = t.inorder();
    for i in 1..seq.len() {
        assert!(seq[i - 1] < seq[i]);
    }
    assert!(t.check());

    // 8. 操作序列对拍：固定种子随机插删，与有序数组对照
    let mut t = Bst::new();
    let mut refs: Vec<i32> = Vec::new();
    let mut seed: u32 = 12345;
    for _ in 0..400 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let v = ((seed >> 16) % 201) as i32 - 100;
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let do_erase = (seed >> 16) % 2 == 0;
        if do_erase {
            let removed = t.erase(v);
            let at = refs.iter().position(|&x| x == v);
            assert_eq!(removed, at.is_some());
            if let Some(i) = at {
                refs.remove(i);
            }
        } else {
            t.insert(v);
            let mut at = 0;
            while at < refs.len() && refs[at] < v {
                at += 1;
            }
            refs.insert(at, v); // 手写插入，保持有序
        }
        assert!(t.check());
        assert_eq!(t.size(), refs.len() as i32);
        assert_eq!(t.inorder(), refs);
    }
    println!("random ops size: {}", t.size());

    println!("all tests passed");
}
