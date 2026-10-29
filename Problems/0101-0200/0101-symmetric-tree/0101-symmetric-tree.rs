/*
============================================================================
101. 对称二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/symmetric-tree/
分类：二叉树 · 递归 / 队列

给你一个二叉树的根节点 root ， 检查它是否轴对称。

示例 1：
输入：root = [1,2,2,3,4,4,3]
输出：true

示例 2：
输入：root = [1,2,2,null,3,null,3]
输出：false

提示：
- 树中节点数目在范围 [1, 1000] 内
- -100 <= Node.val <= 100

进阶：你可以运用递归和迭代两种方法解决这个问题吗？
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
解法一：递归（对左右子树做镜像比较）【最优解】

为什么慢：直接的想法是"把树翻转一份，再和原树逐节点比较相等"，能过，
但要多存一整棵树；或者按层序把每层取出来、判回文，也很容易在 null
位置上出错（空位也要参与比较）。

换个角度：轴对称的本质不是"左右子树相同"，而是"左右子树互为镜像"。
于是问题变成：写一个 `mirror(a, b)`，判断 a 与 b 是否镜像：
  · 都空 → true；
  · 一个空一个不空 → false；
  · 值不同 → false；
  · 否则 a 的左要和 b 的右镜像、a 的右要和 b 的左镜像。
  从 `mirror(root.left, root.right)` 出发即可，不需要额外的翻转。

关键点：
  · 比较对象是"左对右"，不是"左对左"，这是全题唯一容易写错的地方
  · 两个为空的检查必须放在取 val 之前，否则空引用
  · 叶子节点：镜像比较返回 (左空、右空) 的与，结果为 true
  · 根节点自身不需要比较，只需要它的左右子树互为镜像
  · 递归深度 = 树高，节点数 ≤ 1000，安全

以示例 1（root = [1,2,2,3,4,4,3]）为例：
  mirror(2左, 2右)：值相等；比较 (3 与 右3)、(4 与 左4) 都成立；
  逐层向内全部满足 → true ✔
以示例 2（root = [1,2,2,null,3,null,3]）为例：
  mirror(2左, 2右)：值相等；2左的右是 3，而 2右的左是 null → false ✔

复杂度分析：
  时间 O(n)：每个节点只参与一次镜像比较
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：迭代（队列成对出队）【非最优】

官方方法二。用一个队列成对存放"应该镜像的两个节点"，初始放
`(root.left, root.right)`：
  · 弹出 (a, b)：都空则继续；一个空则 false；值不同则 false；
  · 依次压入镜像配对：(a.left, b.right)、(a.right, b.left)。
队列空时说明全部配对成功。

关键点：
  · 空节点也要入队（用 None 表示），否则无法判断"一边有孩子一边没有"
  · 入队顺序必须是"左对右、右对左"，不能写成本身
  · 队列元素是 Option，取出后再判空
  · 空间 O(w)：宽树时队列可能存下整层节点对

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/

// ---------- 解法一：递归（最优解） ----------
pub struct Solution1;

impl Solution1 {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn mirror(
            a: Option<&Rc<RefCell<TreeNode>>>,
            b: Option<&Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (a, b) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(x), Some(y)) => {
                    let (xv, xl, xr) = {
                        let n = x.borrow();
                        (n.val, n.left.clone(), n.right.clone())
                    };
                    let (yv, yl, yr) = {
                        let n = y.borrow();
                        (n.val, n.left.clone(), n.right.clone())
                    };
                    xv == yv && mirror(xl.as_ref(), yr.as_ref()) && mirror(xr.as_ref(), yl.as_ref())
                }
            }
        }

        match root {
            None => true,
            Some(n) => {
                let (left, right) = {
                    let n = n.borrow();
                    (n.left.clone(), n.right.clone())
                };
                mirror(left.as_ref(), right.as_ref())
            }
        }
    }
}

// ---------- 解法二：迭代（队列成对出队） ----------
pub struct Solution2;

impl Solution2 {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };
        let (left, right) = {
            let n = root.borrow();
            (n.left.clone(), n.right.clone())
        };

        let mut q: VecDeque<(
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
        )> = VecDeque::new();
        q.push_back((left, right));
        while let Some((a, b)) = q.pop_front() {
            let (av, al, ar) = match &a {
                None => (None, None, None),
                Some(n) => {
                    let n = n.borrow();
                    (Some(n.val), n.left.clone(), n.right.clone())
                }
            };
            let (bv, bl, br) = match &b {
                None => (None, None, None),
                Some(n) => {
                    let n = n.borrow();
                    (Some(n.val), n.left.clone(), n.right.clone())
                }
            };

            match (av, bv) {
                (None, None) => continue,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(x), Some(y)) => {
                    if x != y {
                        return false;
                    }
                }
            }

            q.push_back((al, br));
            q.push_back((ar, bl));
        }
        true
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

fn check(level_order: &[Option<i32>], want: bool) {
    assert_eq!(Solution1::is_symmetric(build_tree(level_order)), want);
    assert_eq!(Solution2::is_symmetric(build_tree(level_order)), want);
}

fn main() {
    // 示例 1
    check(
        &[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ],
        true,
    );

    // 示例 2
    check(
        &[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)],
        false,
    );

    // 边界：单节点
    check(&[Some(1)], true);

    // 边界：两个孩子的值不同
    check(&[Some(1), Some(2), Some(3)], false);

    // 边界：两个孩子的值相同
    check(&[Some(1), Some(2), Some(2)], true);

    // 边界：一边有内层孩子，另一边没有
    check(&[Some(1), Some(2), Some(2), Some(3), None, None, None], false);

    // 边界：一侧是右孩子、另一侧是左孩子，仍然镜像
    check(&[Some(1), Some(2), Some(2), None, Some(3), Some(3), None], true);

    // 边界：镜像层差 —— 左侧有两个 3，右侧只有一个 3
    check(&[Some(1), Some(2), Some(2), Some(3), Some(3), Some(3), None], false);

    // 边界：完全镜像的四层满二叉树
    check(
        &[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(8),
            Some(7),
            Some(6),
            Some(5),
        ],
        true,
    );

    // 边界：满二叉树里改坏一个叶子
    check(
        &[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(8),
            Some(7),
            Some(6),
            Some(9),
        ],
        false,
    );

    // 边界：负数不影响结构
    check(
        &[
            Some(-100),
            Some(-50),
            Some(-50),
            Some(0),
            Some(1),
            Some(1),
            Some(0),
        ],
        true,
    );

    println!("all tests passed");
}
