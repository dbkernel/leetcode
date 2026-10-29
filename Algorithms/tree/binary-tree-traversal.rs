#![allow(dead_code)]
/*
============================================================================
二叉树遍历（Binary Tree Traversal）
分类：手写数据结构 · 树
复杂度：时间 O(n)：四种遍历都恰好访问每个节点一次；
        空间 O(h)：递归栈或显式栈，h 为树高；Morris 中序 O(1)
============================================================================

用途：
  树的入门动作，也是序列化 / 反序列化、树形 DP 的底座。面试常问
  「不用递归写后序」「不用栈和递归遍历，空间 O(1)」。

核心思想：
  三种深度优先序的差别只在「输出根」放在两次递归的哪一侧：前序在
  进左子树之前，中序在左子树回来之后，后序在右子树回来之后。把递归
  栈换成一个自己维护的数组就是迭代版，层序把栈换成先进先出的循环
  队列。Morris 中序借叶子结点空闲的右指针指向「中序后继」，走完左
  子树后顺着这根线回到祖先，用完立刻拆掉。

关键点（不变量）：
  · 迭代前序：先压右孩子再压左孩子，栈顶才是左孩子；
  · 迭代后序：last 记录上一个真正输出的节点，只有右孩子为空或
    右孩子 == last 时才允许输出栈顶，否则先把右子树走完；
  · Morris：prev.right == cur 表示第二次到达 cur，必须把
    prev.right 还原成 NIL，否则树里留下环；
  · 层序队列用 count 记录元素个数：回绕后 head == tail 无法区分
    空与满，只看 head / tail 会误判。

手推一遍：
  以层序数组 {1,2,3,4,5,-1,6,-1,-1,7} 建出的树：

            1
          /   \
         2     3
        / \     \
       4   5     6
          /
         7

  1. 前序：输出 1，进左 2，进左 4 输出 4，回 2 进右 5，进左 7
     输出 7，回到 5 输出 5，回到 2 输出 2，回根进右 3，进右 6，
     得 1 2 4 5 7 3 6；
  2. 中序：一路向左到 4 输出，回 2 输出，进 5 先到 7 输出，
     得 4 2 7 5 1 3 6；
  3. 后序得 4 7 5 2 6 3 1；层序按出队顺序得 1 2 3 4 5 6 7。

复杂度分析：
  时间 O(n)：每节点进栈、出栈各一次；Morris 每条边最多走两遍。
  空间 O(h)：递归栈 / 显式栈；层序 O(w)，w 为最大层宽；
        Morris 只用两个指针，O(1)。

Rust 实现选择：
  用索引数组（Vec<Node> + usize 下标）而不是 Option<Box<Node>>：
  Morris 穿线要反复修改同一节点的 right，旋转和删除也要同时看父子
  关系，索引不会和借用检查打架，节点还能按需留空复用。代价是空位
  用 NIL 常量表示，越界检查靠断言保证。
*/

const NIL: usize = usize::MAX;

struct Node {
    val: i32,
    left: usize,
    right: usize,
}

// 自己写的循环队列：count 区分空与满，容量不够就翻倍。
struct RingQueue {
    buf: Vec<usize>,
    head: usize,
    tail: usize,
    count: usize,
}

impl RingQueue {
    fn new() -> Self {
        RingQueue { buf: vec![NIL; 8], head: 0, tail: 0, count: 0 }
    }

    fn empty(&self) -> bool {
        self.count == 0
    }

    fn push(&mut self, x: usize) {
        if self.count == self.buf.len() {
            // 扩容：按 head 起的逻辑顺序搬移，搬完 head 回到 0
            let cap = self.buf.len();
            let mut nb = vec![NIL; cap * 2];
            for i in 0..self.count {
                nb[i] = self.buf[(self.head + i) % cap];
            }
            self.buf = nb;
            self.head = 0;
            self.tail = self.count;
        }
        self.buf[self.tail] = x;
        self.tail = (self.tail + 1) % self.buf.len();
        self.count += 1;
    }

    fn pop(&mut self) -> usize {
        let x = self.buf[self.head];
        self.head = (self.head + 1) % self.buf.len();
        self.count -= 1;
        x
    }
}

struct BinTree {
    nodes: Vec<Node>,
    root: usize,
}

impl BinTree {
    fn new() -> Self {
        BinTree { nodes: Vec::new(), root: NIL }
    }

    fn new_node(&mut self, v: i32) -> usize {
        self.nodes.push(Node { val: v, left: NIL, right: NIL });
        self.nodes.len() - 1
    }

    // 层序数组建树，-1 表示空位。仅用于构造。
    fn build(&mut self, a: &[i32]) {
        self.nodes.clear();
        self.root = NIL;
        if a.is_empty() || a[0] == -1 {
            return;
        }
        self.root = self.new_node(a[0]);
        let mut q = RingQueue::new();
        q.push(self.root);
        let mut i = 1;
        while !q.empty() && i < a.len() {
            let cur = q.pop();
            if a[i] != -1 {
                let id = self.new_node(a[i]);
                self.nodes[cur].left = id;
                q.push(id);
            }
            i += 1;
            if i < a.len() {
                if a[i] != -1 {
                    let id = self.new_node(a[i]);
                    self.nodes[cur].right = id;
                    q.push(id);
                }
                i += 1;
            }
        }
    }

    // 只有左孩子的链，值从根往下是 n, n-1, ..., 1
    fn build_left_chain(&mut self, n: i32) {
        self.nodes.clear();
        self.root = NIL;
        for v in 1..=n {
            let id = self.new_node(v);
            self.nodes[id].left = self.root;
            self.root = id;
        }
    }

    // 只有右孩子的链，值从根往下同样是 n, n-1, ..., 1
    fn build_right_chain(&mut self, n: i32) {
        self.nodes.clear();
        self.root = NIL;
        for v in 1..=n {
            let id = self.new_node(v);
            self.nodes[id].right = self.root;
            self.root = id;
        }
    }

    // 层序序列化，空位写 -1 并去掉尾部空位，用来确认遍历没改坏树
    fn to_level_array(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut q = RingQueue::new();
        q.push(self.root);
        while !q.empty() {
            let t = q.pop();
            if t == NIL {
                out.push(-1);
                continue;
            }
            out.push(self.nodes[t].val);
            q.push(self.nodes[t].left);
            q.push(self.nodes[t].right);
        }
        while out.last() == Some(&-1) {
            out.pop();
        }
        out
    }

    // ---------- 递归版 ----------
    fn pre_rec(nodes: &[Node], t: usize, out: &mut Vec<i32>) {
        if t == NIL {
            return;
        }
        out.push(nodes[t].val);
        Self::pre_rec(nodes, nodes[t].left, out);
        Self::pre_rec(nodes, nodes[t].right, out);
    }

    fn in_rec(nodes: &[Node], t: usize, out: &mut Vec<i32>) {
        if t == NIL {
            return;
        }
        Self::in_rec(nodes, nodes[t].left, out);
        out.push(nodes[t].val);
        Self::in_rec(nodes, nodes[t].right, out);
    }

    fn post_rec(nodes: &[Node], t: usize, out: &mut Vec<i32>) {
        if t == NIL {
            return;
        }
        Self::post_rec(nodes, nodes[t].left, out);
        Self::post_rec(nodes, nodes[t].right, out);
        out.push(nodes[t].val);
    }

    fn preorder(&self) -> Vec<i32> {
        let mut out = Vec::new();
        Self::pre_rec(&self.nodes, self.root, &mut out);
        out
    }

    fn inorder(&self) -> Vec<i32> {
        let mut out = Vec::new();
        Self::in_rec(&self.nodes, self.root, &mut out);
        out
    }

    fn postorder(&self) -> Vec<i32> {
        let mut out = Vec::new();
        Self::post_rec(&self.nodes, self.root, &mut out);
        out
    }

    // ---------- 显式栈迭代版 ----------
    fn preorder_iter(&self) -> Vec<i32> {
        let mut out = Vec::new();
        if self.root == NIL {
            return out;
        }
        let mut st = vec![self.root];
        while let Some(t) = st.pop() {
            out.push(self.nodes[t].val);
            // 先压右再压左：后进先出，左孩子先被处理
            if self.nodes[t].right != NIL {
                st.push(self.nodes[t].right);
            }
            if self.nodes[t].left != NIL {
                st.push(self.nodes[t].left);
            }
        }
        out
    }

    fn inorder_iter(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut st: Vec<usize> = Vec::new();
        let mut cur = self.root;
        while cur != NIL || !st.is_empty() {
            while cur != NIL {
                st.push(cur);
                cur = self.nodes[cur].left;
            }
            cur = st.pop().unwrap();
            out.push(self.nodes[cur].val);
            cur = self.nodes[cur].right;
        }
        out
    }

    fn postorder_iter(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut st: Vec<usize> = Vec::new();
        let mut cur = self.root;
        let mut last = NIL; // 上一个真正输出的节点
        while cur != NIL || !st.is_empty() {
            while cur != NIL {
                st.push(cur);
                cur = self.nodes[cur].left;
            }
            let top = *st.last().unwrap();
            // 右子树存在且还没走完，先转过去，top 留在栈上
            if self.nodes[top].right != NIL && self.nodes[top].right != last {
                cur = self.nodes[top].right;
            } else {
                out.push(self.nodes[top].val);
                last = top;
                st.pop();
            }
        }
        out
    }

    // ---------- 层序：自己写的循环队列 ----------
    fn level_order(&self) -> Vec<i32> {
        let mut out = Vec::new();
        if self.root == NIL {
            return out;
        }
        let mut q = RingQueue::new();
        q.push(self.root);
        while !q.empty() {
            let t = q.pop();
            out.push(self.nodes[t].val);
            if self.nodes[t].left != NIL {
                q.push(self.nodes[t].left);
            }
            if self.nodes[t].right != NIL {
                q.push(self.nodes[t].right);
            }
        }
        out
    }

    // ---------- Morris 中序：O(1) 额外空间 ----------
    fn morris_inorder(&mut self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut cur = self.root;
        while cur != NIL {
            if self.nodes[cur].left == NIL {
                out.push(self.nodes[cur].val);
                cur = self.nodes[cur].right;
                continue;
            }
            let mut prev = self.nodes[cur].left;
            while self.nodes[prev].right != NIL
                && self.nodes[prev].right != cur
            {
                prev = self.nodes[prev].right;
            }
            if self.nodes[prev].right == NIL {
                self.nodes[prev].right = cur; // 穿线
                cur = self.nodes[cur].left;
            } else {
                self.nodes[prev].right = NIL; // 拆线，树恢复原状
                out.push(self.nodes[cur].val);
                cur = self.nodes[cur].right;
            }
        }
        out
    }
}

fn print_vec(v: &[i32]) {
    print!("[");
    for (i, x) in v.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", x);
    }
    println!("]");
}

fn main() {
    // 1. 固定形状的树，验证四种遍历序列
    let mut t = BinTree::new();
    t.build(&[1, 2, 3, 4, 5, -1, 6, -1, -1, 7]);
    let pre = vec![1, 2, 4, 5, 7, 3, 6];
    let ino = vec![4, 2, 7, 5, 1, 3, 6];
    let post = vec![4, 7, 5, 2, 6, 3, 1];
    let lvl = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(t.preorder(), pre);
    assert_eq!(t.inorder(), ino);
    assert_eq!(t.postorder(), post);
    assert_eq!(t.preorder_iter(), pre);
    assert_eq!(t.inorder_iter(), ino);
    assert_eq!(t.postorder_iter(), post);
    assert_eq!(t.level_order(), lvl);
    // Morris 必须与递归中序一致
    assert_eq!(t.morris_inorder(), ino);
    // 遍历（含 Morris 穿线拆线）之后树结构不能变
    assert_eq!(t.to_level_array(), vec![1, 2, 3, 4, 5, -1, 6, -1, -1, 7]);
    assert_eq!(t.preorder(), pre);
    print!("fixed tree preorder: ");
    print_vec(&t.preorder());

    // 2. 单节点
    let mut t = BinTree::new();
    t.build(&[5]);
    assert_eq!(t.preorder(), vec![5]);
    assert_eq!(t.inorder(), vec![5]);
    assert_eq!(t.postorder(), vec![5]);
    assert_eq!(t.preorder_iter(), vec![5]);
    assert_eq!(t.inorder_iter(), vec![5]);
    assert_eq!(t.postorder_iter(), vec![5]);
    assert_eq!(t.level_order(), vec![5]);
    assert_eq!(t.morris_inorder(), vec![5]);
    assert_eq!(t.to_level_array(), vec![5]);

    // 3. 空树
    let mut t = BinTree::new();
    assert!(t.preorder().is_empty());
    assert!(t.postorder().is_empty());
    assert!(t.preorder_iter().is_empty());
    assert!(t.inorder_iter().is_empty());
    assert!(t.postorder_iter().is_empty());
    assert!(t.level_order().is_empty());
    assert!(t.morris_inorder().is_empty());
    assert!(t.to_level_array().is_empty());
    // 空树里 -1 只表示空位，不产生节点
    t.build(&[-1, 1, 2]);
    assert!(t.to_level_array().is_empty());

    // 4. 只有左 / 右孩子的长链：验证显式栈不塌
    let n: i32 = 500;
    let asc: Vec<i32> = (1..=n).collect();
    let desc: Vec<i32> = (1..=n).rev().collect();
    let mut lt = BinTree::new();
    lt.build_left_chain(n);
    assert_eq!(lt.preorder_iter(), desc);
    assert_eq!(lt.inorder_iter(), asc);
    assert_eq!(lt.postorder_iter(), asc);
    assert_eq!(lt.level_order(), desc);
    assert_eq!(lt.morris_inorder(), asc);

    let mut rt = BinTree::new();
    rt.build_right_chain(n);
    assert_eq!(rt.preorder_iter(), desc);
    assert_eq!(rt.inorder_iter(), desc);
    assert_eq!(rt.postorder_iter(), asc);
    assert_eq!(rt.level_order(), desc);
    assert_eq!(rt.morris_inorder(), desc);

    println!("all tests passed");
}
