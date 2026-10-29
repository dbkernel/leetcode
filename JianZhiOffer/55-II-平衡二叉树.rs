/*
============================================================================
LCR 176 / 剑指 Offer 55 - II. 判断是否为平衡二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/ping-heng-er-cha-shu-lcof/
分类：二叉树 · 递归（后序剪枝 / 先序 + 深度）

输入一棵二叉树的根节点，判断该树是不是平衡二叉树。如果某二叉树中
任意节点的左右子树的深度相差不超过 1，那么它就是一棵平衡二叉树。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：true
解释：如下图

示例 2：
输入：root = [1,2,2,3,3,null,null,4,4]
输出：false
解释：如下图

限制：
- 0 <= 树的结点个数 <= 10000

注意：本题与主站 110 题相同：https://leetcode.cn/problems/balanced-binary-tree/
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
解法一：后序遍历 + 剪枝（自底向上）【最优解】

为什么慢：容易想到的做法是"先序判断"—— 对每个节点都调用一次求深度的函数，
比较左右深度差，再递归检查左右子树。但 `depth(x)` 会把 x 的整棵子树走一遍，
而 x 的每个祖先又各自调用了一次，同一批节点被反复数，满二叉树下退化成
O(n log n)，斜链下更差。

换个角度：求深度这件事本身就是后序遍历的自然产物。让递归在返回时把
"子树高度"带上来：
  · 平衡 → 返回真实高度；
  · 不平衡 → 返回 -1 作为标记，并且只要左右任一侧返回 -1 就立即继续向上
    返回 -1（剪枝），不再做多余计算。
这样每个节点只被访问一次，判断和求高度合成一趟。

关键点：
  · -1 是"不平衡"的哨兵值：真实高度非负，不会与它混淆
  · 必须先拿 left，判断 left == -1 后立刻返回，再去拿 right，
    避免在已知不平衡时还白跑另一侧
  · 判断条件是 abs(left - right) > 1，相等（差 0）和差 1 都算平衡
  · 空节点高度为 0；叶子左右都是 0，差 0，返回高度 1
  · 最终答案看 `recur(root) != -1`，不能只看根节点是否"局部平衡"
  · Rust 里借用一个节点时先取 clone 的左右孩子，避免 RefCell 借用冲突

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  9、15、7 返回高度 1；
  20 的左右都是 1，差 0 → 返回 2；
  根 3 的左 1、右 2，差 1 → 返回 3 ≠ -1 → true ✔

以示例 2（root = [1,2,2,3,3,null,null,4,4]）为例：
  节点 3（左分支）的两个孩子 4、4 返回 1 → 3 返回 2；
  节点 2（左分支）的左 2、右 0，差 2 → 返回 -1；
  根 1 收到 -1，剪枝直接返回 -1 → false ✔

复杂度分析：
  时间 O(n)：每个节点最多访问一次（剪枝只会更少）
  空间 O(n)：递归栈，最坏为斜链

解法二：先序遍历 + 判断深度（自顶向下）【非最优】

官方方法二，也最容易想到：
  · 空树视为平衡，返回 true；
  · 否则同时满足三条才算平衡：
      1. |depth(left) - depth(right)| <= 1：当前节点平衡；
      2. 左子树平衡；
      3. 右子树平衡。
  三个条件用 && 连接，短路求值可以让某个条件失败后少走一部分。

关键点：
  · `depth` 就是"二叉树的最大深度"，空树 0；
  · 必须三项都检查：只判断当前节点会漏掉子树内部的高度差；
  · 大量重复计算来自 `depth` 在每一层被反复调用，这是它非最优的原因；
  · 写法直观，适合面试先讲思路，再优化到解法一。

复杂度分析：
  时间 O(n log n)：满二叉树时每层累计 O(n) 的 depth 调用，共 O(log n) 层；
       斜链最坏 O(n^2)
  空间 O(n)：递归栈
*/

// ---------- 解法一：后序剪枝（最优解） ----------
pub struct Solution1;

impl Solution1 {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 平衡则返回子树高度，不平衡返回 -1
        fn recur(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            let Some(n) = node else {
                return 0;
            };
            let (left_node, right_node) = {
                let n = n.borrow();
                (n.left.clone(), n.right.clone())
            };

            let left = recur(left_node.as_ref());
            if left == -1 {
                return -1; // 左子树已不平衡，剪枝
            }
            let right = recur(right_node.as_ref());
            if right == -1 {
                return -1; // 右子树已不平衡，剪枝
            }

            if (left - right).abs() > 1 {
                return -1;
            }
            1 + left.max(right)
        }

        recur(root.as_ref()) != -1
    }
}

// ---------- 解法二：先序遍历 + 判断深度 ----------
pub struct Solution2;

impl Solution2 {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(node) = root.as_ref() else {
            return true;
        };
        let (left, right) = {
            let n = node.borrow();
            (n.left.clone(), n.right.clone())
        };

        (Self::depth(left.as_ref()) - Self::depth(right.as_ref())).abs() <= 1
            && Self::is_balanced(left)
            && Self::is_balanced(right)
    }

    fn depth(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                1 + Self::depth(n.left.as_ref()).max(Self::depth(n.right.as_ref()))
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

fn check(level_order: &[Option<i32>], want: bool) {
    assert_eq!(Solution1::is_balanced(build_tree(level_order)), want);
    assert_eq!(Solution2::is_balanced(build_tree(level_order)), want);
}

fn main() {
    // 示例 1
    check(
        &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        true,
    );

    // 示例 2
    check(
        &[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ],
        false,
    );

    // 边界：空树，视为平衡
    check(&[], true);

    // 边界：单节点
    check(&[Some(1)], true);

    // 边界：只有两个孩子，差 0
    check(&[Some(1), Some(2), Some(3)], true);

    // 边界：一条两层的左斜链，根左右高度差 2
    check(&[Some(1), Some(2), None, Some(3)], false);

    // 边界：链藏在左子树内部，根看似还行
    check(
        &[Some(1), Some(2), Some(3), Some(4), None, None, None, Some(5)],
        false,
    );

    // 边界：完全二叉树，各节点高度差 <= 1
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
        true,
    );

    // 边界：只差一层（差 1 仍算平衡）
    check(&[Some(1), Some(2), Some(3), Some(4), None, None, None], true);

    // 边界：负数值不影响结构
    check(&[Some(-100), Some(-50), Some(100), Some(0), Some(1)], true);

    // 边界：深链 200 层，必然不平衡
    let mut level_order = Vec::new();
    for i in 1..=200 {
        level_order.push(Some(i));
        level_order.push(None);
    }
    check(&level_order, false);

    println!("all tests passed");
}
