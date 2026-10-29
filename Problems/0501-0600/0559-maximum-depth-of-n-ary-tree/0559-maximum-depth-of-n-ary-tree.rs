/*
============================================================================
559. N 叉树的最大深度（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-depth-of-n-ary-tree/
分类：N 叉树 · 深度（DFS / BFS）

给定一个 N 叉树，找到其最大深度。

最大深度是指从根节点到最远叶子节点的最长路径上的节点总数。

N 叉树输入按层序遍历序列化表示，每组子节点由空值分隔（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：3

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：5

提示：
- 树的深度不会超过 1000 。
- 树的节点数目位于 [0, 10^4] 之间。
*/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// 力扣预置的节点定义（多叉树，注意孩子是一个数组）
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            children: Vec::new(),
        }
    }
}

/*
解法一：DFS 后序递归【最优解】

为什么慢：BFS 按层数数也能出答案，但要维护一个队列，空间随最宽的那一层
增长；对 N 叉树来说，一个「星形」根节点就能把队列塞成 O(m)。
而且按层数数还要额外写「每层弹固定个数」的循环，容易写成每弹一个加一次。

换个角度：最大深度天然满足递归结构 —— 空树深度 0；非空树的深度是
`1 + max(每个孩子的深度)`。把「兄弟之间取最大」交给一次遍历即可，
递归返回时顺手加一。

关键点：
  · 空节点返回 0，是递归边界，也是叶子返回 1 的来源
  · N 叉树没有「左右」之分，必须在 children 数组里遍历取 max，
    不能只取第一个孩子（那是求某条特定路径，不是最大深度）
  · 孩子在数组中的顺序不影响答案，只影响遍历顺序
  · 递归深度 = 树高 h，题目保证 h ≤ 1000，不会炸栈
  · Rust 里 `Rc<RefCell<Node>>` 需要在递归前 borrow，注意别持有借用跨递归

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  根 1 的孩子是 3、2、4；
  3 的孩子 5、6 都返回 1 → 3 返回 1 + max(1,1) = 2；
  2、4 无孩子，返回 1；
  根 1 返回 1 + max(2,1,1) = 3 ✔

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：BFS 按层计数【非最优】

官方方法二。层序遍历：每轮先锁定本层节点数 size，弹出 size 个节点并把
它们的孩子入队，然后 depth += 1；队列空时 depth 就是最大深度。

关键点：
  · size 必须在 for 之前取，循环中队列还会增长
  · 空树直接返回 0
  · 每层结束才 depth += 1，不能每弹一个节点就加
  · 空间 O(w)：w 为最大层宽，星形树时约为 m，比 DFS 差

复杂度分析：
  时间 O(m)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/

// ---------- 解法一：DFS 后序递归（最优解） ----------
pub struct Solution1;

impl Solution1 {
    pub fn max_depth(root: Option<Rc<RefCell<Node>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<Node>>>) -> i32 {
            let Some(n) = node else {
                return 0;
            };
            let children: Vec<Rc<RefCell<Node>>> = n.borrow().children.clone();
            let mut best = 0;
            for child in children {
                best = best.max(dfs(&Some(child)));
            }
            best + 1
        }
        dfs(&root)
    }
}

// ---------- 解法二：BFS 按层计数 ----------
pub struct Solution2;

impl Solution2 {
    pub fn max_depth(root: Option<Rc<RefCell<Node>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let mut depth = 0;
        let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                let children = node.borrow().children.clone();
                for child in children {
                    q.push_back(child);
                }
            }
            depth += 1;
        }
        depth
    }
}

// ===== 测试辅助：按层序数组建 N 叉树（None 表示空） =====
fn build_nary_tree(level_order: &[Option<i32>]) -> Option<Rc<RefCell<Node>>> {
    let first = *level_order.first()?;
    let root = Rc::new(RefCell::new(Node::new(first?)));

    let mut nodes = vec![Rc::clone(&root)];
    let mut idx = 2; // 跳过根与它后面的 None
    let mut p = 0;
    while idx < level_order.len() && p < nodes.len() {
        let parent = Rc::clone(&nodes[p]);
        p += 1;
        while idx < level_order.len() && level_order[idx].is_some() {
            let child = Rc::new(RefCell::new(Node::new(level_order[idx].unwrap())));
            parent.borrow_mut().children.push(Rc::clone(&child));
            nodes.push(child);
            idx += 1;
        }
        idx += 1; // 跳过孩子列表的 None 结束符
    }
    Some(root)
}

fn check(level_order: &[Option<i32>], want: i32) {
    assert_eq!(Solution1::max_depth(build_nary_tree(level_order)), want);
    assert_eq!(Solution2::max_depth(build_nary_tree(level_order)), want);
}

fn main() {
    // 示例 1
    check(
        &[Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)],
        3,
    );

    // 示例 2
    check(
        &[
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            None,
            Some(6),
            Some(7),
            None,
            Some(8),
            None,
            Some(9),
            Some(10),
            None,
            None,
            Some(11),
            None,
            Some(12),
            None,
            Some(13),
            None,
            None,
            Some(14),
        ],
        5,
    );

    // 边界：空树
    check(&[], 0);

    // 边界：单节点
    check(&[Some(1)], 1);

    // 边界：根没有孩子
    check(&[Some(1), None], 1);

    // 边界：星形树，深度 2
    check(
        &[Some(1), None, Some(2), Some(3), Some(4), Some(5), Some(6)],
        2,
    );

    // 边界：不能只取第一个孩子 —— 深度藏在第二个孩子下面
    check(
        &[
            Some(1),
            None,
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            None,
            Some(5),
            None,
        ],
        4,
    );

    // 边界：孩子顺序不影响答案
    check(
        &[Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)],
        3,
    );

    // 边界：深链（每个节点只有一个孩子），深度 200
    let mut level_order = Vec::new();
    for i in 1..=200 {
        level_order.push(Some(i));
        level_order.push(None);
    }
    check(&level_order, 200);

    println!("all tests passed");
}
