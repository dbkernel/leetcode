/*
============================================================================
236. 二叉树的最近公共祖先（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/
分类：二叉树 · 后序递归 / 最近公共祖先

给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个节点 p、q，最近公共
祖先表示为一个节点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点
也可以是它自己的祖先）。”

示例 1：
输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
输出：3
解释：节点 5 和节点 1 的最近公共祖先是节点 3 。

示例 2：
输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
输出：5
解释：节点 5 和节点 4 的最近公共祖先是节点 5 。因为根据定义最近公共祖先
节点可以为节点本身。

示例 3：
输入：root = [1,2], p = 1, q = 2
输出：1

提示：
- 树中节点数目在范围 [2, 10^5] 内。
- -10^9 <= Node.val <= 10^9
- 所有 Node.val 互不相同 。
- p != q
- p 和 q 均存在于给定的二叉树中。
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

// 本地 rustc 单文件编译需要显式声明 Solution；平台上该类型已预置
pub struct Solution;

/*
解法：后序递归（每棵子树向上报告"找到了谁"）【最优解】

为什么慢：最直白的做法是"从根往下逐层检查"：对每个节点分别判断 p、q 是否
在它的子树里，第一个"两侧都含"的节点就是答案。可判断一次子树是否含某节点
本身要 O(n)，套在每个节点上最坏 O(n^2)（斜链）。另一种常见写法是用哈希表
记录每个节点的父节点，从 p 向上爬并标记，再从 q 向上找第一个被标记的祖先：
时间 O(n)，但要额外存父指针表，空间 O(n)。

换个角度：递归函数不必回答"是不是"，直接回答"这棵子树里找到了 p 或 q 中的
哪一个"。后序遍历先拿到左右子树的答案，再在当前节点合并：
  · 左右都非空 → p、q 分居两侧，当前节点就是最近公共祖先；
  · 只有一侧非空 → 把这一侧的结果继续向上传；
  · 两侧都空 → 本子树里一个都没找到。
每个节点只走一次，也不需要额外容器。

关键点：
  · 出口先判 Rc::ptr_eq(node, p) || Rc::ptr_eq(node, q) 并立刻返回 node ——
    这一条让"p 是 q 的祖先"时答案停在 p；删掉它就会继续下沉到 q
  · 比较必须用 ptr_eq（节点身份），不能用 val ==，值相等说明不了是同一个节点
  · 合并必须放在后序位置：先拿到左右结果再判断，先序时还不知道两侧情况
  · 只有左右都非空才返回当前节点；一侧为空时返回另一侧，返回 None 会把
    已经找到的节点弄丢
  · 返回值是"节点"而不是布尔值 —— 布尔值只说明找到了，无法区分找到的是谁

以示例 2（root = [3,5,1,6,2,0,8,null,null,7,4]，p = 5，q = 4）为例：
  根 3 的左子树：走到 5 命中 p，立即返回 5，不再看它的孩子；
  根 3 的右子树：既没有 5 也没有 4，返回 None；
  根 3 处左非空、右为空 → 把 5 继续上报，最终答案是 5 ✔
  （若去掉"命中即返回"，左子树会返回 4，答案就错成了 4。）

复杂度分析：
  时间 O(n)：每个节点恰好进入递归一次，合并是常数操作
  空间 O(h)：递归栈只保存当前路径，h 为树高；最坏（链）为 O(n)
*/
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            cur: &Option<Rc<RefCell<TreeNode>>>,
            p: &Rc<RefCell<TreeNode>>,
            q: &Rc<RefCell<TreeNode>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let Some(node) = cur else {
                return None;
            };

            // 命中 p 或 q 就停：p 是 q 的祖先时，答案正是先遇到的这个节点
            if Rc::ptr_eq(node, p) || Rc::ptr_eq(node, q) {
                return Some(Rc::clone(node));
            }

            let (left, right) = {
                let n = node.borrow();
                (n.left.clone(), n.right.clone())
            };
            let l = dfs(&left, p, q);
            let r = dfs(&right, p, q);

            match (l, r) {
                // 两侧都找到了，说明 p、q 从这里分开
                (Some(_), Some(_)) => Some(Rc::clone(node)),
                // 只剩一侧有结果：该结果可能是 LCA，也可能还要继续往上比
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            }
        }

        dfs(&root, p.as_ref()?, q.as_ref()?)
    }
}

// ===== 测试辅助：按层序数组建树、按值找节点 =====
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
    let node = root.as_ref()?;
    if node.borrow().val == val {
        return Some(Rc::clone(node));
    }
    let (left, right) = {
        let n = node.borrow();
        (n.left.clone(), n.right.clone())
    };
    find_node(&left, val).or_else(|| find_node(&right, val))
}

fn check(level_order: &[Option<i32>], p_val: i32, q_val: i32, want: i32) {
    let root = build_tree(level_order);
    let p = find_node(&root, p_val).expect("p 必须存在");
    let q = find_node(&root, q_val).expect("q 必须存在");
    let want_node = find_node(&root, want).expect("期望节点必须存在");

    let got = Solution::lowest_common_ancestor(
        root.clone(),
        Some(Rc::clone(&p)),
        Some(Rc::clone(&q)),
    )
    .expect("答案不应为空");

    // 比节点身份而不是值：值唯一时 ptr_eq 才能证明返回的是那个节点
    assert!(
        Rc::ptr_eq(&got, &want_node),
        "lca({:?}, {}, {}) = {}, want {}",
        level_order,
        p_val,
        q_val,
        got.borrow().val,
        want
    );
}

fn main() {
    let example = [
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ];

    // 示例 1：p、q 分居根的两侧
    check(&example, 5, 1, 3);

    // 示例 2：p 是 q 的祖先，答案必须是 p 本身
    check(&example, 5, 4, 5);

    // 示例 3：根就是其中一个节点
    check(&[Some(1), Some(2)], 1, 2, 1);

    // 反直觉：两个节点都在根的左子树里，不能因为根同时含二者就返回根
    check(&example, 6, 4, 5);
    check(&example, 7, 4, 2);

    // 反直觉：答案藏在右子树内部，且 p、q 分别是左右孙子
    check(&example, 7, 8, 3);

    // 兄弟节点
    check(&[Some(1), Some(2), Some(3)], 2, 3, 1);

    // 参数顺序无关：把 p、q 反过来，答案不变
    check(&example, 4, 5, 5);

    // 边界：只有两个节点的左斜链
    check(&[Some(1), Some(2)], 2, 1, 1);

    // 边界：200 层右斜链，祖先关系与深链两种情况
    let mut chain = vec![Some(1)];
    for i in 2..=200 {
        chain.push(None);
        chain.push(Some(i));
    }
    check(&chain, 1, 200, 1);
    check(&chain, 150, 200, 150);
    check(&chain, 100, 150, 100);

    println!("all tests passed");
}
