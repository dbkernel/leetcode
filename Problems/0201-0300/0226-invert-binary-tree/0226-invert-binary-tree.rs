/*
============================================================================
226. 翻转二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/invert-binary-tree/
分类：二叉树 · 递归 / 层序遍历

给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。

示例 1：
输入：root = [4,2,7,1,3,6,9]
输出：[4,7,2,9,6,3,1]

示例 2：
输入：root = [2,1,3]
输出：[2,3,1]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目范围在 [0, 100] 内
- -100 <= Node.val <= 100
*/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// 力扣预置的节点定义
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/*
解法一：递归（交换左右孩子 + 递归翻转）【最优解】

为什么慢：按"新建一棵镜像树"的思路写也能过，但要额外构造一整棵树，
空间翻倍；而且原树被丢弃，白白浪费。层序迭代也能做，但要显式维护队列，
代码并不比递归短。

换个角度：翻转是一棵子树一个动作 —— 对当前节点，先交换它的左右孩子，
再分别翻转这两棵子树即可。由于"翻转左右子树"与"交换左右孩子"可交换顺序，
先序、后序写都成立；中序则会因为交换后访问的子树已变而需要小心，一般不用。

关键点：
  · 每次递归只需做两件事：交换 left / right，然后递归两个孩子
  · 空节点直接返回，作为递归出口
  · 交换后递归时的 left / right 已是"翻转前"的另一侧，注意别重复交换
  · 递归深度 = 树高，本题节点数 ≤ 100，绝对安全
  · 若题目不允许修改原树，才需要新建节点；本题允许原地翻转
  · Rust 中先取出孩子再借用修改，避免同一节点双重 borrow

以示例 1（root = [4,2,7,1,3,6,9]）为例：
  节点 1、3、6、9 无孩子，返回自身；
  节点 2 交换左右（1、3）→ 变 (3,1)；节点 7 同理变 (9,6)；
  根 4 交换左右（2、7）→ (7,2)，结果 [4,7,2,9,6,3,1] ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：BFS 层序迭代【非最优】

官方方法二。用队列做层序遍历，每弹出一个节点就交换它的左右孩子，
再把非空孩子入队 —— 因为交换只依赖节点自身，与访问顺序无关。

关键点：
  · 交换发生在"弹出"时，孩子入队顺序无所谓，反正后面还会各自交换
  · 空树直接返回 root（None），不要先入队
  · 不需要按层分批处理，本题没有"按层"语义，一层一层反而多余
  · 空间 O(w)：宽树时要存下整层节点

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/

// ---------- 解法一：递归（最优解） ----------
pub struct Solution1;

impl Solution1 {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(node) = root else {
            return None;
        };
        let (left, right) = {
            let n = node.borrow();
            (n.left.clone(), n.right.clone())
        };
        {
            let mut n = node.borrow_mut();
            n.left = right;
            n.right = left;
        }
        let new_left = Solution1::invert_tree(node.borrow().left.clone());
        let new_right = Solution1::invert_tree(node.borrow().right.clone());
        {
            let mut n = node.borrow_mut();
            n.left = new_left;
            n.right = new_right;
        }
        Some(node)
    }
}

// ---------- 解法二：BFS 层序迭代 ----------
pub struct Solution2;

impl Solution2 {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(Rc::clone(&root));
        while let Some(node) = q.pop_front() {
            let (left, right) = {
                let mut n = node.borrow_mut();
                let l = n.left.take();
                let r = n.right.take();
                n.left = r;
                n.right = l;
                (n.left.clone(), n.right.clone())
            };
            if let Some(l) = left {
                q.push_back(l);
            }
            if let Some(r) = right {
                q.push_back(r);
            }
        }
        Some(root)
    }
}

// ===== 测试辅助：按层序数组（None 表示空）建树 =====
fn build_tree(level_order: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let first = *level_order.first()?;
    let root = Rc::new(RefCell::new(TreeNode::new(first?)));

    let mut nodes = vec![Rc::clone(&root)];
    let mut idx = 1;
    let mut i = 0;
    while i < nodes.len() && idx < level_order.len() {
        if let Some(v) = level_order[idx] {
            let left = Rc::new(RefCell::new(TreeNode::new(v)));
            nodes[i].borrow_mut().left = Some(Rc::clone(&left));
            nodes.push(left);
        }
        idx += 1;
        if idx >= level_order.len() {
            break;
        }
        if let Some(v) = level_order[idx] {
            let right = Rc::new(RefCell::new(TreeNode::new(v)));
            nodes[i].borrow_mut().right = Some(Rc::clone(&right));
            nodes.push(right);
        }
        idx += 1;
        i += 1;
    }
    Some(root)
}

fn same_tree(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            let (xv, xl, xr) = {
                let n = x.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            let (yv, yl, yr) = {
                let n = y.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            xv == yv && same_tree(&xl, &yl) && same_tree(&xr, &yr)
        }
        _ => false,
    }
}

fn check(input: &[Option<i32>], want: &[Option<i32>]) {
    assert!(same_tree(
        &Solution1::invert_tree(build_tree(input)),
        &build_tree(want)
    ));
    assert!(same_tree(
        &Solution2::invert_tree(build_tree(input)),
        &build_tree(want)
    ));
}

fn main() {
    // 示例 1
    check(
        &[
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ],
        &[
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ],
    );

    // 示例 2
    check(
        &[Some(2), Some(1), Some(3)],
        &[Some(2), Some(3), Some(1)],
    );

    // 示例 3：空树
    check(&[], &[]);

    // 边界：单节点
    check(&[Some(1)], &[Some(1)]);

    // 边界：只有左孩子 → 翻转后变成右孩子
    check(&[Some(1), Some(2)], &[Some(1), None, Some(2)]);

    // 边界：只有右孩子
    check(&[Some(1), None, Some(2)], &[Some(1), Some(2)]);

    // 边界：叶子层的孩子也要换位
    check(
        &[Some(1), Some(2), Some(3), Some(4)],
        &[Some(1), Some(3), Some(2), None, None, None, Some(4)],
    );

    // 边界：负数不影响结构
    check(
        &[Some(-100), Some(-50), Some(100)],
        &[Some(-100), Some(100), Some(-50)],
    );

    // 边界：左斜链 100 层，翻转后变成同深度的右斜链
    let mut chain = vec![Some(1)];
    for i in 2..=100 {
        chain.push(Some(i));
        chain.push(None);
    }
    let mut want = Vec::new();
    for i in 1..=100 {
        want.push(Some(i));
        want.push(None);
    }
    check(&chain, &want);

    println!("all tests passed");
}
