/*
============================================================================
617. 合并二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/merge-two-binary-trees/
分类：二叉树 · 递归 / BFS

给你两棵二叉树： root1 和 root2 。

想象一下，当你将其中一棵覆盖到另一棵之上时，两棵树上的一些节点将会重叠
（而另一些不会）。你需要将这两棵树合并成一棵新二叉树。合并的规则是：
如果两个节点重叠，那么将这两个节点的值相加作为合并后节点的新值；
否则，不为 null 的节点将直接作为新二叉树的节点。

返回合并后的二叉树。

注意: 合并过程必须从两个树的根节点开始。

示例 1：
输入：root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
输出：[3,4,5,5,4,null,7]

示例 2：
输入：root1 = [1], root2 = [1,2]
输出：[2,2]

提示：
- 两棵树中的节点数目在范围 [0, 2000] 内
- -10^4 <= Node.val <= 10^4
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
解法一：DFS 递归【最优解】

为什么慢：BFS 也能合并，但要显式维护两个队列，代码量和常数都更大；
而且合并本身是"同位置一对一"的结构化操作，用递归直接对应结构，最自然。

换个角度：把"合并"看成对两个根同时做一次前序/后序都无所谓的遍历：
  · 若 root1 为空，合并结果就是 root2；
  · 若 root2 为空，合并结果就是 root1；
  · 都不空时，新节点值 = 两者之和，左右孩子分别递归合并。
  "某一边为空就把另一边接回来"这一条，一次性处理了"不重叠节点直接保留"
  和"两边都空返回空"两种情况，不需要额外判断。

关键点：
  · 先把两个为空的出口写完，后面就不必再判空
  · 直接复用 root1 作为结果节点，能省一次构造；若不想改动输入，
    则新建节点再递归（下面复用写法，题目允许修改）
  · 递归顺序不影响结果，因为每个位置只合并一次
  · 递归深度 = 树高，节点数 ≤ 2000，不会爆栈
  · Rust 中先把两边的孩子 clone 出来再借用修改，避免 RefCell 重复借用 panic

以示例 1 为例（root1 = [1,3,2,5]，root2 = [2,1,3,null,4,null,7]）：
  根 1 + 2 = 3；
  左：3 + 1 = 4，其左 5 + 空 = 5，其右 空 + 4 = 4；
  右：2 + 3 = 5，其左 空，其右 空 + 7 = 7；
  结果 [3,4,5,5,4,null,7] ✔

复杂度分析：
  时间 O(m + n)：每个节点最多访问一次
  空间 O(h)：递归栈，h 为合并后树高；最坏（斜链）O(m + n)

解法二：BFS（队列成对出队）【非最优】

官方方法二。用一个队列存"待合并的节点对 (a, b)"，初始为 (root1, root2)：
  · 弹出 (a, b)，两者都不空；
  · 累加 a.val += b.val；
  · 左孩子：a 空 → 接 b 的左；b 空 → 保持 a 的左；都不空 → 成对入队；
  · 右孩子同理。
  这样任何时刻队列里都是已经存在的节点对，不会出现 None。

关键点：
  · 空树要先处理：root1 空直接返回 root2，反之亦然，否则后面无法成对
  · 只有当两个孩子都不空时才入队；一方为空时直接"嫁接"，不要入队
  · 队列里存的是 Rc 节点对，注意别把已经合并过的节点重复入队
  · 空间 O(w)：宽树时队列可能存下整层节点对

复杂度分析：
  时间 O(m + n)
  空间 O(w)：w 为合并后树的最大层宽
*/

// ---------- 解法一：DFS 递归（最优解） ----------
pub struct Solution1;

impl Solution1 {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => {
                let (a_left, b_left, a_right, b_right) = {
                    let an = a.borrow();
                    let bn = b.borrow();
                    (
                        an.left.clone(),
                        bn.left.clone(),
                        an.right.clone(),
                        bn.right.clone(),
                    )
                };
                a.borrow_mut().val += b.borrow().val;
                let left = Self::merge_trees(a_left, b_left);
                let right = Self::merge_trees(a_right, b_right);
                {
                    let mut an = a.borrow_mut();
                    an.left = left;
                    an.right = right;
                }
                Some(a)
            }
        }
    }
}

// ---------- 解法二：BFS 成对出队 ----------
pub struct Solution2;

impl Solution2 {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, b) => return b,
            (a, None) => return a,
            (Some(a), Some(b)) => {
                let mut q: VecDeque<(Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>)> =
                    VecDeque::new();
                q.push_back((Rc::clone(&a), Rc::clone(&b)));

                while let Some((x, y)) = q.pop_front() {
                    let (x_left, y_left, x_right, y_right) = {
                        let xn = x.borrow();
                        let yn = y.borrow();
                        (
                            xn.left.clone(),
                            yn.left.clone(),
                            xn.right.clone(),
                            yn.right.clone(),
                        )
                    };
                    x.borrow_mut().val += y.borrow().val;

                    // 左孩子：都不空才入队，否则把非空的那一边嫁接过来
                    match (x_left, y_left) {
                        (Some(xl), Some(yl)) => q.push_back((xl, yl)),
                        (None, yl) => x.borrow_mut().left = yl,
                        _ => {}
                    }

                    // 右孩子同理
                    match (x_right, y_right) {
                        (Some(xr), Some(yr)) => q.push_back((xr, yr)),
                        (None, yr) => x.borrow_mut().right = yr,
                        _ => {}
                    }
                }
                Some(a)
            }
        }
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

fn check(lhs: &[Option<i32>], rhs: &[Option<i32>], want: &[Option<i32>]) {
    assert!(same_tree(
        &Solution1::merge_trees(build_tree(lhs), build_tree(rhs)),
        &build_tree(want)
    ));
    assert!(same_tree(
        &Solution2::merge_trees(build_tree(lhs), build_tree(rhs)),
        &build_tree(want)
    ));
}

fn main() {
    // 示例 1
    check(
        &[Some(1), Some(3), Some(2), Some(5)],
        &[Some(2), Some(1), Some(3), None, Some(4), None, Some(7)],
        &[Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)],
    );

    // 示例 2
    check(&[Some(1)], &[Some(1), Some(2)], &[Some(2), Some(2)]);

    // 边界：两棵空树
    check(&[], &[], &[]);

    // 边界：只有一棵为空 —— 另一棵原样返回
    check(
        &[Some(1), Some(2), Some(3)],
        &[],
        &[Some(1), Some(2), Some(3)],
    );
    check(&[], &[Some(1), None, Some(2)], &[Some(1), None, Some(2)]);

    // 边界：root2 在内部位置为空，root1 的节点要保留
    check(
        &[Some(1), None, Some(2)],
        &[Some(1), Some(3)],
        &[Some(2), Some(3), Some(2)],
    );

    // 边界：root1 在内部位置为空，root2 的节点要接过来
    check(
        &[Some(1), Some(3)],
        &[Some(1), None, Some(2)],
        &[Some(2), Some(3), Some(2)],
    );

    // 边界：负数相加
    check(&[Some(-1)], &[Some(-2)], &[Some(-3)]);

    // 边界：一边深一边浅
    check(
        &[Some(1), Some(2), Some(3), Some(4)],
        &[Some(1), Some(2)],
        &[Some(2), Some(4), Some(3), Some(4)],
    );

    // 边界：两棵 200 层斜链，值逐层翻倍
    let mut chain = Vec::new();
    let mut want = Vec::new();
    for i in 1..=200 {
        chain.push(Some(i));
        chain.push(None);
        want.push(Some(2 * i));
        want.push(None);
    }
    check(&chain, &chain, &want);

    println!("all tests passed");
}
