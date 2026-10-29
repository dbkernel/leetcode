/*
============================================================================
104. 二叉树的最大深度（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-depth-of-binary-tree/
分类：二叉树 · 深度（DFS / BFS）

给定一个二叉树 root ，返回其最大深度。

二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：3

示例 2：
输入：root = [1,null,2]
输出：2

提示：
- 树中节点的数量在 [0, 10^4] 区间内。
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
解法一：DFS（后序递归取较大值）【最优解】

为什么慢：如果按层去数，必须真的把每一层都走完、并在每层结束时计数；
虽然也是 O(n)，但要多维护一个队列，空间随层宽增长。

换个角度：最大深度满足递归关系 —— 空树深度 0，非空树深度是
`1 + max(左子树深度, 右子树深度)`。把问题拆成两个子问题再合并，
一次后序遍历即可。

关键点：
  · 直接 `Option<&Rc<RefCell<TreeNode>>>` 递归借用即可，无需 clone
  · 空节点返回 0，这是递归的边界，也是叶子返回 1 的来源
  · 必须取 max 而不是固定走某一侧；写成 `left + right` 就变成求直径了
  · 后序位置才相加：先拿到左右子树的深度，再 +1 计当前节点
  · 递归深度 = 结果高度，最坏（斜链）O(n)

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  叶子 9、15、7 返回 1；节点 20 返回 1 + max(1,1) = 2；
  根 3 返回 1 + max(1,2) = 3

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈；最坏（斜链）O(n)

解法二：BFS 按层计数【非最优】

官方方法二。层序遍历时每处理完一层就把层数加一，循环结束时层数就是最大深度。
  · 用队列，每轮先记下本层节点数 size；
  · 弹出 size 个节点并把孩子入队，然后 depth += 1。

关键点：
  · size 必须在 for 循环之前锁定，循环中队列还会增长
  · 空树返回 0：不要入队再靠循环兜底
  · 每层结束才 depth += 1，不能每弹出一个节点就加
  · 队列元素用 clone 的 Rc，避免在遍历时被借用冲突
  · 空间 O(w)：宽树时队列可能存下整层，比 DFS 更费

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/

// ---------- 解法一：DFS（最优解） ----------
pub struct Solution1;

impl Solution1 {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left = dfs(n.left.as_ref());
                    let right = dfs(n.right.as_ref());
                    1 + left.max(right)
                }
            }
        }
        dfs(root.as_ref())
    }
}

// ---------- 解法二：BFS 按层计数 ----------
pub struct Solution2;

impl Solution2 {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let mut depth = 0;
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                let node = node.borrow();
                if let Some(left) = &node.left {
                    q.push_back(Rc::clone(left));
                }
                if let Some(right) = &node.right {
                    q.push_back(Rc::clone(right));
                }
            }
            depth += 1;
        }
        depth
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

fn check(level_order: &[Option<i32>], want: i32) {
    assert_eq!(Solution1::max_depth(build_tree(level_order)), want);
    assert_eq!(Solution2::max_depth(build_tree(level_order)), want);
}

fn main() {
    // 示例 1
    check(
        &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        3,
    );

    // 示例 2：只有右孩子
    check(&[Some(1), None, Some(2)], 2);

    // 边界：空树
    check(&[], 0);

    // 边界：单节点
    check(&[Some(1)], 1);

    // 边界：左斜链 / 右斜链，深度等于节点数
    check(&[Some(1), Some(2), None, Some(3), None, Some(4)], 4);
    check(&[Some(1), None, Some(2), None, Some(3)], 3);

    // 边界：完全二叉树，深度 3
    check(
        &[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ],
        3,
    );

    // 边界：左子树深、右子树浅（不能只看右边）
    check(
        &[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ],
        4,
    );

    // 边界：负数值不影响结构
    check(&[Some(-100), Some(-50), Some(100), Some(0)], 3);

    // 边界：深链 200 层
    let mut level_order = Vec::new();
    for i in 1..=200 {
        level_order.push(Some(i));
        level_order.push(None);
    }
    check(&level_order, 200);

    println!("all tests passed");
}
