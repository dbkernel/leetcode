#![allow(dead_code)]
/*
============================================================================
AVL 树（AVL Tree）
分类：手写数据结构 · 树
操作复杂度：
  height   O(1)     每个节点缓存子树高度
  find     O(log n)
  insert   O(log n) 回溯更新高度，至多一次旋转（单旋或双旋）
  erase    O(log n) 双孩子先搬后继，再从后继父节点回溯
  rotate   O(1)     单次旋转只改三条指针、修正两个高度
  空间 O(n)；树高恒为 O(log n)，最坏也是，不会退化成链
============================================================================

用途：
  需要「有序 + 稳定 O(log n)」时的首选手写结构；面试里常用来考
  「你知不知道旋转之后要回溯修高度」「删除为什么比重插入难」。

核心思想：
  AVL 在 BST 的有序性之上再加一条：每个节点左右子树高度差不超过 1。
  插入 / 删除后只有从改动点到根这一条路径上的节点可能失衡，于是回溯
  时自底向上修高度，一旦某节点 bf 变成 +2 或 -2，就按失衡形态做
  一次旋转把高度压回去。旋转不改变中序序列，所以有序性自动保持。

关键点（不变量）：
  · 先修自己的高度，再看 bf；旋转返回新的子树根，调用方必须接住；
  · 单旋后两个节点的高度都要重算：先算子节点，再算新根，顺序反了
    就会把旧高度写回去；
  · 删除双孩子节点后，必须从「后继的父节点」开始回溯，不能只修到
    被删节点为止。这一段是 AVL 删除最容易漏的地方：后继在右子树
    深处，它的祖先高度全变了，漏掉就会出现 bf 已经越界却没人旋转；
  · 重复值直接返回、不计入、不旋转，保证树里每个值唯一。

手推一遍：
  依次插入 30 20 10：
  1. 插 30 后 h=1；插 20 挂在左边，30 的 bf=1；
  2. 插 10 仍往左，30 的 bf=2，且左孩子 20 的 bf=1，属于 LL：
     对 30 右旋，20 升为根，30 成为 20 的右孩子，20 原来的右子树
     （空）挂到 30 左边；先修 30 的高度为 1，再修 20 为 2；
  3. 若插入序列是 30 10 20，则 30 的 bf=2 而左孩子 10 的 bf=-1，
     属于 LR：先对 10 左旋得到 30 -> 20 -> 10 的形状，再对 30
     右旋，最终 20 为根，10、30 为两个孩子。

复杂度分析：
  时间 O(log n)：树高被 bf <= 1 卡在 1.44 log2(n+2) 附近；
        insert / erase 各走一条根到叶的路径，旋转 O(1)。
  空间 O(n) 存节点，O(log n) 递归栈。

Rust 实现选择：
  索引数组 + free 列表。旋转要同时改三个节点的 left / right，若用
  Option<Box<Node>> 就得反复 take / 重新装箱，索引下标直接写即可；
  被摘掉的槽位压回 free，等价于 C++ 的 delete，且全程 safe。
*/

const NIL: usize = usize::MAX;

struct Node {
    val: i32,
    height: i32, // 以该节点为根的子树高度，空树记 0，叶子记 1
    left: usize,
    right: usize,
}

struct Avl {
    nodes: Vec<Node>,
    free: Vec<usize>,
    root: usize,
}

impl Avl {
    fn new() -> Self {
        Avl { nodes: Vec::new(), free: Vec::new(), root: NIL }
    }

    fn new_node(&mut self, v: i32) -> usize {
        let n = Node { val: v, height: 1, left: NIL, right: NIL };
        if let Some(id) = self.free.pop() {
            self.nodes[id] = n;
            id
        } else {
            self.nodes.push(n);
            self.nodes.len() - 1
        }
    }

    fn h(&self, t: usize) -> i32 {
        if t == NIL { 0 } else { self.nodes[t].height }
    }

    fn bf(&self, t: usize) -> i32 {
        if t == NIL {
            0
        } else {
            self.h(self.nodes[t].left) - self.h(self.nodes[t].right)
        }
    }

    fn fix(&mut self, t: usize) {
        let l = self.h(self.nodes[t].left);
        let r = self.h(self.nodes[t].right);
        self.nodes[t].height = if l > r { l } else { r } + 1;
    }

    // LL 失衡：左孩子的左子树过高，对 t 右旋
    fn rotate_ll(&mut self, t: usize) -> usize {
        let x = self.nodes[t].left;
        self.nodes[t].left = self.nodes[x].right; // 仍有序
        self.nodes[x].right = t;
        self.fix(t); // 先算子节点，再算新根
        self.fix(x);
        x
    }

    // RR 失衡：右孩子的右子树过高，对 t 左旋
    fn rotate_rr(&mut self, t: usize) -> usize {
        let x = self.nodes[t].right;
        self.nodes[t].right = self.nodes[x].left;
        self.nodes[x].left = t;
        self.fix(t);
        self.fix(x);
        x
    }

    // LR 失衡：先对左孩子左旋，把它变成 LL，再对自己右旋
    fn rotate_lr(&mut self, t: usize) -> usize {
        let l = self.nodes[t].left;
        let nl = self.rotate_rr(l);
        self.nodes[t].left = nl;
        self.rotate_ll(t)
    }

    // RL 失衡：先对右孩子右旋，把它变成 RR，再对自己左旋
    fn rotate_rl(&mut self, t: usize) -> usize {
        let r = self.nodes[t].right;
        let nr = self.rotate_ll(r);
        self.nodes[t].right = nr;
        self.rotate_rr(t)
    }

    fn rebalance(&mut self, t: usize) -> usize {
        self.fix(t);
        let b = self.bf(t);
        if b > 1 {
            // 左孩子 bf >= 0 是 LL；< 0 说明新节点插在左孩子的右边
            if self.bf(self.nodes[t].left) >= 0 {
                return self.rotate_ll(t);
            }
            return self.rotate_lr(t);
        }
        if b < -1 {
            if self.bf(self.nodes[t].right) <= 0 {
                return self.rotate_rr(t);
            }
            return self.rotate_rl(t);
        }
        t
    }

    fn insert_rec(&mut self, t: usize, v: i32, inserted: &mut bool) -> usize {
        if t == NIL {
            *inserted = true;
            return self.new_node(v);
        }
        if v < self.nodes[t].val {
            let l = self.nodes[t].left;
            let nl = self.insert_rec(l, v, inserted);
            self.nodes[t].left = nl;
        } else if v > self.nodes[t].val {
            let r = self.nodes[t].right;
            let nr = self.insert_rec(r, v, inserted);
            self.nodes[t].right = nr;
        } else {
            return t; // 重复值不插入，也不必修高度
        }
        self.rebalance(t)
    }

    fn erase_rec(&mut self, t: usize, v: i32, removed: &mut bool) -> usize {
        if t == NIL {
            return NIL;
        }
        if v < self.nodes[t].val {
            let l = self.nodes[t].left;
            let nl = self.erase_rec(l, v, removed);
            self.nodes[t].left = nl;
        } else if v > self.nodes[t].val {
            let r = self.nodes[t].right;
            let nr = self.erase_rec(r, v, removed);
            self.nodes[t].right = nr;
        } else {
            *removed = true;
            if self.nodes[t].left == NIL || self.nodes[t].right == NIL {
                // 叶子或单孩子：孩子顶上，高度由调用方 rebalance 修
                let c = if self.nodes[t].left != NIL {
                    self.nodes[t].left
                } else {
                    self.nodes[t].right
                };
                self.free.push(t);
                return c;
            }
            // 双孩子：中序后继（右子树最小值）的值搬上来
            let mut s = self.nodes[t].right;
            while self.nodes[s].left != NIL {
                s = self.nodes[s].left;
            }
            let successor = self.nodes[s].val;
            self.nodes[t].val = successor;
            // 关键：真正删的是右子树里的后继，回溯必须从右子树内部
            // 一路 rebalance 上来，否则后继的祖先高度会停在旧值
            let r = self.nodes[t].right;
            let nr = self.erase_rec(r, successor, removed);
            self.nodes[t].right = nr;
        }
        self.rebalance(t)
    }

    fn insert(&mut self, v: i32) -> bool {
        let mut inserted = false;
        self.root = self.insert_rec(self.root, v, &mut inserted);
        inserted
    }

    fn erase(&mut self, v: i32) -> bool {
        let mut removed = false;
        self.root = self.erase_rec(self.root, v, &mut removed);
        removed
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

    fn height(&self) -> i32 {
        self.h(self.root)
    }

    fn root_val(&self) -> i32 {
        self.nodes[self.root].val // 调用前保证非空
    }

    fn size(&self) -> i32 {
        self.size_rec(self.root)
    }

    fn size_rec(&self, t: usize) -> i32 {
        if t == NIL {
            return 0;
        }
        1 + self.size_rec(self.nodes[t].left)
            + self.size_rec(self.nodes[t].right)
    }

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
        out.push(self.nodes[t].val);
        self.inorder_rec(self.nodes[t].right, out);
    }

    // 校验函数：同时验证 BST 严格递增、|bf| <= 1、缓存高度没写错
    fn check_balanced(&self) -> bool {
        self.check_rec(self.root, &mut false, &mut 0)
    }

    fn check_rec(&self, t: usize, has_prev: &mut bool, prev: &mut i32) -> bool {
        if t == NIL {
            return true;
        }
        let (l, r) = (self.nodes[t].left, self.nodes[t].right);
        if !self.check_rec(l, has_prev, prev) {
            return false;
        }
        if *has_prev && !(*prev < self.nodes[t].val) {
            return false; // BST 必须严格递增
        }
        *has_prev = true;
        *prev = self.nodes[t].val;
        if !self.check_rec(r, has_prev, prev) {
            return false;
        }
        let (lh, rh) = (self.h(l), self.h(r));
        if (lh - rh).abs() > 1 {
            return false;
        }
        if self.nodes[t].height != (if lh > rh { lh } else { rh }) + 1 {
            return false; // 缓存高度也不能说谎
        }
        true
    }
}

fn main() {
    // 1. 顺序插入 1..100，高度不能超过理论上界 9
    let mut t = Avl::new();
    for v in 1..=100 {
        assert!(t.insert(v));
        assert!(t.check_balanced());
    }
    assert_eq!(t.size(), 100);
    // 高度为 h 的 AVL 最少节点数 N(h)=N(h-1)+N(h-2)+1，
    // N(9)=88 <= 100 < N(10)=143，所以高度最多 9
    assert!(t.height() <= 9);
    assert!(t.height() >= 7);
    assert_eq!(t.inorder(), (1..=100).collect::<Vec<i32>>());
    println!("1..100 height: {}", t.height());

    // 2. 四种旋转各一个最小用例，最终都应是 20 为根、高度 2
    let mut ll = Avl::new();
    ll.insert(30);
    ll.insert(20);
    ll.insert(10);
    assert_eq!(ll.root_val(), 20);
    assert_eq!(ll.height(), 2);
    assert_eq!(ll.inorder(), vec![10, 20, 30]);
    assert!(ll.check_balanced());

    let mut rr = Avl::new();
    rr.insert(10);
    rr.insert(20);
    rr.insert(30);
    assert_eq!(rr.root_val(), 20);
    assert_eq!(rr.height(), 2);
    assert_eq!(rr.inorder(), vec![10, 20, 30]);
    assert!(rr.check_balanced());

    let mut lr = Avl::new();
    lr.insert(30);
    lr.insert(10);
    lr.insert(20);
    assert_eq!(lr.root_val(), 20);
    assert_eq!(lr.height(), 2);
    assert_eq!(lr.inorder(), vec![10, 20, 30]);
    assert!(lr.check_balanced());

    let mut rl = Avl::new();
    rl.insert(10);
    rl.insert(30);
    rl.insert(20);
    assert_eq!(rl.root_val(), 20);
    assert_eq!(rl.height(), 2);
    assert_eq!(rl.inorder(), vec![10, 20, 30]);
    assert!(rl.check_balanced());

    // 3. 重复值不插入
    let mut t = Avl::new();
    assert!(t.insert(5));
    assert!(!t.insert(5));
    assert!(!t.insert(5));
    assert_eq!(t.size(), 1);
    assert_eq!(t.height(), 1);
    assert_eq!(t.inorder(), vec![5]);
    assert!(t.check_balanced());
    assert!(!t.erase(4)); // 不存在，不算删除
    assert_eq!(t.size(), 1);

    // 4. 删除：叶子 / 单孩子 / 根 / 空树
    let mut t = Avl::new();
    for v in 1..=20 {
        t.insert(v);
    }
    assert!(t.erase(1)); // 叶子
    assert!(!t.find(1));
    assert!(t.check_balanced());
    while t.size() > 1 {
        // 逐个删到只剩根
        let v = t.inorder()[0];
        assert!(t.erase(v));
        assert!(t.check_balanced());
    }
    assert!(t.erase(t.root_val())); // 删根
    assert_eq!(t.size(), 0);
    assert_eq!(t.height(), 0);
    assert!(t.check_balanced());
    assert!(!t.erase(42)); // 空树删除
    assert_eq!(t.size(), 0);

    // 5. 双孩子删除：后继在右子树深处，回溯必须覆盖后继的祖先
    let mut t = Avl::new();
    for v in [50, 25, 75, 10, 30, 60, 90, 5, 15, 27, 35, 55, 65, 80, 95,
        1, 7, 12, 18]
    {
        t.insert(v);
    }
    assert!(t.check_balanced());
    // 50 有双孩子，后继是 55，删完从 55 的父节点一路修回根
    assert!(t.erase(50));
    assert!(!t.find(50));
    assert!(t.find(55));
    assert!(t.check_balanced());
    assert_eq!(t.size(), 18);

    // 6. 随机插入 / 删除对拍：固定种子，与有序数组对照
    let mut t = Avl::new();
    let mut refs: Vec<i32> = Vec::new();
    let mut seed: u32 = 20240601;
    for _ in 0..1600 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let v = ((seed >> 16) % 201) as i32 - 100;
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let do_erase = (seed >> 16) % 2 == 0;
        if do_erase {
            let removed = t.erase(v);
            let at = refs.iter().position(|&x| x == v);
            assert_eq!(removed, at.is_some());
            if let Some(i) = at {
                refs.remove(i); // ref 中的值互不重复
            }
        } else {
            let inserted = t.insert(v);
            let mut at = 0;
            while at < refs.len() && refs[at] < v {
                at += 1;
            }
            let has = at < refs.len() && refs[at] == v;
            assert_eq!(inserted, !has);
            if !has {
                refs.insert(at, v); // 手写插入，保持有序
            }
        }
        // 每一步都必须满足：BST 有序 + |bf| <= 1 + 高度缓存正确
        assert!(t.check_balanced());
        assert_eq!(t.size(), refs.len() as i32);
        assert_eq!(t.inorder(), refs);
    }
    println!("random ops size: {} height: {}", t.size(), t.height());
    // 再把剩下的按升序删空，验证连续的删除路径也能保持平衡
    while !refs.is_empty() {
        let v = refs.remove(0);
        assert!(t.erase(v));
        assert!(t.check_balanced());
        assert_eq!(t.inorder(), refs);
    }
    assert_eq!(t.size(), 0);
    assert_eq!(t.height(), 0);

    println!("all tests passed");
}
