/*
============================================================================
538. 把二叉搜索树转换为累加树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/convert-bst-to-greater-tree/
分类：二叉搜索树 · 反序中序遍历 / Morris

给出二叉 搜索 树的根节点，该树的节点值各不相同，请你将其转换为累加树
（Greater Sum Tree），使每个节点 node 的新值等于原树中大于或等于
node.val 的值之和。

提醒一下，二叉搜索树满足下列约束条件：
- 节点的左子树仅包含键 小于 节点键 的节点。
- 节点的右子树仅包含键 大于 节点键 的节点。
- 左右子树也必须是二叉搜索树。

示例 1：
输入：[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
输出：[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]

示例 2：
输入：root = [0,null,1]
输出：[1,null,1]

示例 3：
输入：root = [1,0,2]
输出：[3,3,2]

示例 4：
输入：root = [3,2,4,1]
输出：[7,9,4,10]

提示：
- 树中的节点数介于 0 和 10^4 之间。
- 每个节点的值介于 -10^4 和 10^4 之间。
- 树中的所有值 互不相同 。
- 给定的树为二叉搜索树。

注意：本题和 1038 题相同。
*/
use std::cell::RefCell;
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
解法一：反序中序遍历（递归）【非最优】

为什么慢：如果按"每个节点都重新遍历一遍整棵树，把大于等于它的值加起来"
来写，是 O(n^2)。即使用中序遍历拿到升序数组再算后缀和，也要 O(n) 额外数组。

换个角度：BST 的中序是升序，那么"右 → 根 → 左"就是降序。降序遍历时，
当前节点需要的"所有大于等于它的值之和"恰好就是**已经累加过的部分**：
  · 维护一个累加和 sum；
  · 每访问一个节点，先 `sum += node.val`，再把 `node.val = sum`。
  因为降序保证先处理的一定更大，累加和天然满足"大于等于自己"。

关键点：
  · 顺序必须是右 → 根 → 左，写反了累加方向就变成"小于等于"了
  · 必须先累加再赋值，否则当前值会被覆盖两次
  · 不需要额外数组，也不需要知道整棵树的总和
  · 递归深度 = 树高，节点数 ≤ 10^4，斜链下递归栈可能到 10^4 层，
    因此官方还给了 Morris 版本

以示例 4（root = [3,2,4,1]）为例：
  访问 4：sum = 4，节点变 4；
  访问 3：sum = 7，节点变 7；
  访问 2：sum = 9，节点变 9；
  访问 1：sum = 10，节点变 10；
  结果 [7,9,4,10] ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：Morris 反序中序遍历【最优解】

官方方法二。把解法一搬到 Morris 上，就能去掉递归栈，做到 O(1) 辅助空间。
思路是普通 Morris 的镜像版本：
  · 若当前节点没有右孩子，访问它，转向左孩子；
  · 否则找右子树的最左节点 p（反序中序的前驱）：
      - p.left 为空 → 指向当前节点（建立线索），当前节点向右走；
      - p.left 就是当前节点 → 说明右子树已遍历完，断开线索（还原树），
        访问当前节点，再向左走。
  "访问"做的事和递归版一样：`sum += val; val = sum`。

关键点：
  · 这是 Morris 的镜像版本：普通中序用 right 指针当线索、向左找最右；
    反序中序用 left 指针当线索、向右找最左
  · 线索的建立与断开必须成对，否则会把输入树改坏
  · 找最左节点时 `p.left != node` 防止绕圈
  · 不需要提前退出，所以天然会把所有线索还原干净
  · 辅助空间只有几个指针，是最优解
  · Rust 里借用修改 val 时，先结束不可变借用，再 borrow_mut

以示例 4（root = [3,2,4,1]）为例：
  3 有右孩子：最左节点是 4，4.left 空 → 建线索 4.left=3，向右到 4；
  4 无右孩子 → 访问 4（sum=4），向左回 3；
  3 的最左节点 4 的 left 指向自己 → 断线索，访问 3（sum=7），向左到 2；
  2 无右孩子 → 访问 2（sum=9），向左到 1；
  1 无右孩子 → 访问 1（sum=10）；
  结果 [7,9,4,10] ✔

复杂度分析：
  时间 O(n)：每条边最多走两遍（建线索、断线索）
  空间 O(1)：只用了几个指针

-------- 两种写法对比 --------
  递归反序中序：O(n) 时间、O(h) 空间，写法最短，优先记忆
  Morris 反序中序：O(n) 时间、O(1) 空间，标记为【最优解】，代价是代码更绕
*/

// ---------- 解法一：反序中序遍历（递归） ----------
pub struct Solution1;

impl Solution1 {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            let Some(n) = node else {
                return;
            };
            let (right, val, left) = {
                let n = n.borrow();
                (n.right.clone(), n.val, n.left.clone())
            };

            dfs(&right, sum);
            *sum += val;
            n.borrow_mut().val = *sum;
            dfs(&left, sum);
        }

        let mut sum = 0;
        dfs(&root, &mut sum);
        root
    }
}

// ---------- 解法二：Morris 反序中序遍历（最优解） ----------
pub struct Solution2;

impl Solution2 {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        let mut node = root.clone();

        while let Some(cur) = node.clone() {
            let right = cur.borrow().right.clone();
            if right.is_none() {
                // 访问 cur
                sum += cur.borrow().val;
                cur.borrow_mut().val = sum;
                node = cur.borrow().left.clone();
            } else {
                // 找右子树的最左节点（反序中序的前驱）
                let mut p = right.clone();
                loop {
                    let next = p.as_ref().unwrap().borrow().left.clone();
                    match next {
                        Some(nx) if !Rc::ptr_eq(&nx, &cur) => p = Some(nx),
                        _ => break,
                    }
                }

                let p = p.unwrap();
                let p_left = p.borrow().left.clone();
                match p_left {
                    None => {
                        // 建立线索，向右
                        p.borrow_mut().left = Some(Rc::clone(&cur));
                        node = cur.borrow().right.clone();
                    }
                    Some(link) => {
                        debug_assert!(Rc::ptr_eq(&link, &cur));
                        // 右子树走完，断开线索，访问 cur，向左
                        p.borrow_mut().left = None;
                        sum += cur.borrow().val;
                        cur.borrow_mut().val = sum;
                        node = cur.borrow().left.clone();
                    }
                }
            }
        }
        root
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
        &Solution1::convert_bst(build_tree(input)),
        &build_tree(want)
    ));
    assert!(same_tree(
        &Solution2::convert_bst(build_tree(input)),
        &build_tree(want)
    ));
}

fn main() {
    // 示例 1
    check(
        &[
            Some(4),
            Some(1),
            Some(6),
            Some(0),
            Some(2),
            Some(5),
            Some(7),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            Some(8),
        ],
        &[
            Some(30),
            Some(36),
            Some(21),
            Some(36),
            Some(35),
            Some(26),
            Some(15),
            None,
            None,
            None,
            Some(33),
            None,
            None,
            None,
            Some(8),
        ],
    );

    // 示例 2
    check(&[Some(0), None, Some(1)], &[Some(1), None, Some(1)]);

    // 示例 3
    check(&[Some(1), Some(0), Some(2)], &[Some(3), Some(3), Some(2)]);

    // 示例 4
    check(
        &[Some(3), Some(2), Some(4), Some(1)],
        &[Some(7), Some(9), Some(4), Some(10)],
    );

    // 边界：空树
    check(&[], &[]);

    // 边界：单节点，新值等于自身
    check(&[Some(5)], &[Some(5)]);

    // 边界：只有右孩子的链
    check(
        &[Some(1), None, Some(2), None, Some(3)],
        &[Some(6), None, Some(5), None, Some(3)],
    );

    // 边界：只有左孩子的链
    check(
        &[Some(3), Some(2), None, Some(1)],
        &[Some(3), Some(5), None, Some(6)],
    );

    // 边界：负数
    check(
        &[Some(-1), Some(-2), Some(0)],
        &[Some(-1), Some(-3), Some(0)],
    );

    // 边界：200 层右斜链，值 1..200，节点 v 的新值是 v..200 之和
    let mut chain = vec![Some(1)];
    let mut want = vec![Some(200 * 201 / 2)];
    for i in 2..=200 {
        chain.push(None);
        chain.push(Some(i));
        want.push(None);
        want.push(Some((200 + i) * (201 - i) / 2));
    }
    check(&chain, &want);

    println!("all tests passed");
}
