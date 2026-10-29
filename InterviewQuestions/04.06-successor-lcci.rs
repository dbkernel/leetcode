/*
============================================================================
面试题 04.06. 后继者（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/successor-lcci/
分类：二叉搜索树 · 中序后继

设计一个算法，找出二叉搜索树中指定节点的“下一个”节点（也即中序后继）。

如果指定节点没有对应的“下一个”节点，则返回 null。

示例 1:
输入: root = [2,1,3], p = 1
输出: 2

示例 2:
输入: root = [5,3,6,2,4,null,null,1], p = 6
输出: null
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
解法一：中序遍历收集全部节点【非最优】

为什么慢：中序序列天然升序，找到 p 之后的下一个就是答案，思路零门槛；
但要把整棵树存进数组才能取"下一个"，时间 O(n)、额外空间 O(n)，
而且明明只需要一条从根到 p 的路径信息。

换个角度：真正需要的只是"p 之后第一个被访问的节点"。如果能让遍历在
访问到 p 时记下状态，下一个被访问的节点就是答案，就无需存整棵序列。

关键点：
  · 中序遍历的访问顺序是左 → 根 → 右，p 的后继就是 p 之后第一个访问到的节点
  · 用节点的指针相等（`Rc::ptr_eq`）判断是不是 p，别用值比较
  · 找到后继后要能提前收手，否则会被后续节点覆盖答案
  · 递归写法最直观，但额外 O(n) 的数组是它的主要缺点

以示例 1（root = [2,1,3]，p = 1）为例：
  中序为 1、2、3，p = 1 之后是 2 ✔

复杂度分析：
  时间 O(n)：完整中序遍历
  空间 O(n)：保存中序序列（若改成"边遍历边判断"可以降到 O(h)）

解法二：利用 BST 性质（不遍历全树）【最优解】

官方方法二。分两种情况，都不需要访问整棵树：
  1. p 有右子树：p 的中序后继一定是右子树里的**最小**节点，
     也就是右子树一路向左走到底的那个节点；
  2. p 没有右子树：后继在"祖先"里 —— 从根往下找，每当遇到比 p 大的节点，
     它就有可能是答案（先记下来），然后向左子树继续找更近的；
     遇到比 p 小或相等的节点则向右走。走到底记下的最后一个"更大的祖先"
     就是答案。
情况 2 的本质是：后继是"大于 p 的最小祖先"。

关键点：
  · 情况 1 别忘了先判断 p.right 是否为空
  · 情况 2 里 successor 要在"往左走"之前更新，往右走时不更新
  · 比较用值即可（题目保证值互不相同）
  · 都没找到说明 p 是整棵树的最大值，返回 None
  · 只沿一条路径走，时间 O(h)、空间 O(1)，明显优于解法一

以示例 2（root = [5,3,6,2,4,null,null,1]，p = 6）为例：
  6 没有右子树 → 从根找"大于 6 的最小祖先"：
  5 < 6 往右到 6；6 不大于 6 往右到空 → 没有更大的祖先 → 返回 None ✔
以 p = 4 为例：
  4 没有右子树 → 5 > 4，successor = 5，往左到 3；3 < 4 往右到 4；
  4 不大于 4 → 往右到空，返回 successor = 5 ✔

复杂度分析：
  时间 O(h)：h 为树高，最坏（斜链）O(n)
  空间 O(1)：只用几个指针，连递归栈都不需要
*/

// ---------- 解法一：中序遍历收集全部节点 ----------
pub struct Solution1;

impl Solution1 {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<Rc<RefCell<TreeNode>>>) {
            let Some(n) = node else {
                return;
            };
            let (left, right) = {
                let n = n.borrow();
                (n.left.clone(), n.right.clone())
            };
            dfs(&left, order);
            order.push(Rc::clone(n));
            dfs(&right, order);
        }

        let Some(p) = p else {
            return None;
        };
        let mut order = Vec::new();
        dfs(&root, &mut order);

        for i in 0..order.len().saturating_sub(1) {
            if Rc::ptr_eq(&order[i], &p) {
                return Some(Rc::clone(&order[i + 1]));
            }
        }
        None
    }
}

// ---------- 解法二：利用 BST 性质（最优解） ----------
pub struct Solution2;

impl Solution2 {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(p) = p else {
            return None;
        };
        let (p_val, p_right) = {
            let pn = p.borrow();
            (pn.val, pn.right.clone())
        };

        // 情况 1：有右子树，后继是右子树中的最小节点
        if let Some(right) = p_right {
            let mut node = right;
            loop {
                let next = node.borrow().left.clone();
                match next {
                    Some(n) => node = n,
                    None => return Some(node),
                }
            }
        }

        // 情况 2：没有右子树，后继是"大于 p 的最小祖先"
        let mut successor: Option<Rc<RefCell<TreeNode>>> = None;
        let mut node = root;
        while let Some(cur) = node {
            let (val, left, right) = {
                let n = cur.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            if val > p_val {
                successor = Some(Rc::clone(&cur)); // 先记下这个候选
                node = left;
            } else {
                node = right;
            }
        }
        successor
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

fn find_node(
    root: &Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut node = root.clone();
    while let Some(cur) = node {
        let (v, left, right) = {
            let n = cur.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };
        if v == val {
            return Some(cur);
        }
        node = if val < v { left } else { right };
    }
    None
}

fn check(level_order: &[Option<i32>], p_val: i32, want: i32) {
    let tree1 = build_tree(level_order);
    let got1 = Solution1::inorder_successor(tree1.clone(), find_node(&tree1, p_val));
    let v1 = got1.map_or(-1, |n| n.borrow().val);
    assert_eq!(v1, want, "inorder({level_order:?}, {p_val})");

    let tree2 = build_tree(level_order);
    let got2 = Solution2::inorder_successor(tree2.clone(), find_node(&tree2, p_val));
    let v2 = got2.map_or(-1, |n| n.borrow().val);
    assert_eq!(v2, want, "bst({level_order:?}, {p_val})");
}

fn main() {
    // 示例 1
    check(&[Some(2), Some(1), Some(3)], 1, 2);

    // 示例 2
    check(
        &[Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)],
        6,
        -1,
    );

    // 同一个树上的其它后继
    let tree = [
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        None,
        Some(1),
    ];
    check(&tree, 1, 2);
    check(&tree, 2, 3);
    check(&tree, 3, 4);
    check(&tree, 4, 5);
    check(&tree, 5, 6);

    // 边界：单节点，没有后继
    check(&[Some(1)], 1, -1);

    // 边界：有右子树时，后继是右子树的最左节点
    check(&[Some(2), Some(1), Some(3)], 2, 3);

    // 边界：左斜链 [3,2,null,1]，降序中序 1,2,3
    check(&[Some(3), Some(2), None, Some(1)], 1, 2);
    check(&[Some(3), Some(2), None, Some(1)], 2, 3);
    check(&[Some(3), Some(2), None, Some(1)], 3, -1);

    // 边界：负数值
    check(&[Some(0), Some(-1), Some(1)], -1, 0);
    check(&[Some(0), Some(-1), Some(1)], 1, -1);

    // 边界：100 层右斜链 1..100
    let mut chain = vec![Some(1)];
    for i in 2..=100 {
        chain.push(None);
        chain.push(Some(i));
    }
    check(&chain, 1, 2);
    check(&chain, 99, 100);
    check(&chain, 100, -1);

    println!("all tests passed");
}
