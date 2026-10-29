/*
============================================================================
145. 二叉树的后序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-postorder-traversal/
分类：二叉树 · 遍历（递归 / 栈 / Morris）

给你一棵二叉树的根节点 root ，返回其节点值的 后序遍历 。

示例 1：
输入：root = [1,null,2,3]
输出：[3,2,1]

示例 2：
输入：root = [1,2,3,4,5,null,8,null,null,6,7,9]
输出：[4,6,7,5,2,9,8,3,1]

示例 3：
输入：root = []
输出：[]

示例 4：
输入：root = [1]
输出：[1]

提示：
树中节点的数目在范围 [0, 100] 内
-100 <= Node.val <= 100

进阶：递归算法很简单，你可以通过迭代算法完成吗？
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
解法一：递归【非最优】

官方方法一。后序遍历的定义是「左 → 右 → 根」，直接按定义写递归：
  · 空节点返回；
  · 先递归左子树、再递归右子树，最后把当前节点值加入答案。

关键点：
  · 与前序/中序只差「访问语句的位置」：后序放在两个递归之后
  · 后序的典型用途：「自底向上」汇总信息（子树深度、子树和、是否平衡等），
    因为处理自己时孩子的结果都已经拿到了
  · 递归深度 = 树高，最坏 O(n)

复杂度分析：
  时间 O(n)：每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高，平均 O(log n)、最坏 O(n)

解法二：迭代（辅助栈 + prev 指针）【非最优】

官方方法二。后序比前序、中序都难写：访问一个节点之前，必须确认「右子树已经处理完」。
官方用 prev 记录「上一个被输出的节点」：
  · 一路向左压栈，直到空；
  · 取栈顶（先不弹出），若它没有右孩子、或右孩子就是 prev（说明右子树刚处理完），
    才输出它，并置 prev、把当前指针置空（表示这一支结束）；
  · 否则说明右子树还没处理，把它重新压栈，转去处理右子树。

关键点：
  · 判定条件必须是 `right 为空 || right 与 prev 是同一个节点`，两个分支缺一不可
  · 判断完「还不能访问」时要把节点重新压回去，这是与中序最大的不同
  · prev 的作用是区分「右孩子没处理」和「右孩子处理完了」，只看 right 是否为空不够
  · 每个节点会进栈两次（第一次为了左子树，第二次为了右子树）
  · 栈里存 Rc 克隆（只是引用计数 +1），不会复制节点本身

复杂度分析：
  时间 O(n)
  空间 O(h)：显式栈

解法三：Morris 遍历 + 逆序输出右脊【最优解】

官方方法三。后序的 Morris 与中序共用「线索」骨架，但访问时机变成「拆除线索时」，
并且不是逐个输出，而是把「当前节点左子树的最右一条链（右脊）」整体倒序输出：
  · 若当前节点 p1 有左孩子：找它的中序前驱 p2（左子树最右节点）。
      - p2.right 为空 → 建立线索 p2.right = p1，p1 走向左孩子；
      - p2.right 是 p1 → 拆线索，并把「p1 的左孩子到 p2 这条右链」倒序加到答案里，
        然后 p1 走向右孩子。
  · 若没有左孩子：p1 直接走向右孩子（注意这里不输出）。
  · 主循环结束后，还要把「从根开始的最右一条链」倒序输出一次，收尾。

关键点：
  · add_path 的作用：把一条只往右走的链收集起来再反转，正好等价于后序里
    「先左、再右、最后根」的顺序
  · 主循环里不输出任何单个节点，全部靠 add_path 批量输出，这是后序 Morris 与
    前序/中序 Morris 最本质的区别
  · 最后那次 add_path(root) 不能漏，否则根节点和整条最右链都会丢
  · 线索同样必须拆除，保证不改动原树结构
  · 判断 p2 是否已建立线索时，用 Rc::ptr_eq 比较节点身份（不能比较值）

以 root = [1,null,2,3] 为例（1 的右孩子 2，2 的左孩子 3）：
  递归：左(空) → 右子树：3 的左(空) → 右(空) → 3；然后 2；最后 1 → [3,2,1]
  迭代：同递归的展开顺序，prev 保证 2 在 3 之后输出
  Morris：1 无左孩子 → 走向 2；2 有左孩子 3，3 无右孩子 → 建立线索 3.right=2，
          走向 3；3 无左孩子 → 走向右孩子（线索 2）→ 2 的前驱 p2 = 3 已指向 2 →
          拆线索，倒序输出链 3（即 [3]），走向 2 的右孩子（空）；
          循环结束，add_path(root=1)：链 1 -> 2 倒序 → [2,1]
          合并得到 [3,2,1]

复杂度分析：
  时间 O(n)：每个节点至多被访问两次，add_path 的每条边也只走一次
  空间 O(1)：只用常数个指针（答案数组不计）
*/
struct Solution;

impl Solution {
    // ---------- 解法一：递归 ----------
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(cur) = node {
            let left = cur.borrow().left.clone();
            let right = cur.borrow().right.clone();
            Self::postorder(left, res);
            Self::postorder(right, res);
            res.push(cur.borrow().val);
        }
    }

    pub fn postorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::postorder(root, &mut res);
        res
    }

    // ---------- 解法二：迭代（栈 + prev） ----------
    pub fn postorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while cur.is_some() || !stack.is_empty() {
            // 一路向左压栈
            while let Some(node) = cur.clone() {
                stack.push(node.clone());
                cur = node.borrow().left.clone();
            }
            let node = stack.pop().unwrap();
            let right = node.borrow().right.clone();
            // 右子树为空或已处理完，才输出当前节点
            let right_done = match &right {
                None => true,
                Some(r) => matches!(&prev, Some(p) if Rc::ptr_eq(p, r)),
            };
            if right_done {
                res.push(node.borrow().val);
                prev = Some(node);
                cur = None; // 这一支结束，下一轮从栈里取
            } else {
                // 右子树还没处理，把当前节点压回去，转去处理右子树
                stack.push(node);
                cur = right;
            }
        }
        res
    }

    // ---------- 解法三：Morris 遍历 + 逆序输出右脊（最优解） ----------
    // 把从 node 开始一直往右的链收集起来并倒序追加到 res
    fn add_path(res: &mut Vec<i32>, mut node: Option<Rc<RefCell<TreeNode>>>) {
        let start = res.len();
        while let Some(cur) = node {
            res.push(cur.borrow().val);
            node = cur.borrow().right.clone();
        }
        res[start..].reverse();
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut p1 = root.clone();

        while let Some(node) = p1.clone() {
            let left = node.borrow().left.clone();
            if let Some(l) = left {
                // 找到 p1 的中序前驱：左子树的最右节点
                let mut p2 = l;
                loop {
                    let next = p2.borrow().right.clone();
                    match next {
                        Some(r) if !Rc::ptr_eq(&r, &node) => p2 = r,
                        _ => break,
                    }
                }
                let has_thread = match p2.borrow().right.clone() {
                    Some(r) => Rc::ptr_eq(&r, &node),
                    None => false,
                };
                if !has_thread {
                    // 第一次到达：建立线索，转向左子树
                    p2.borrow_mut().right = Some(node.clone());
                    p1 = node.borrow().left.clone();
                    continue;
                }
                // 第二次到达：拆线索，并把左子树的右脊倒序输出
                p2.borrow_mut().right = None;
                Self::add_path(&mut res, node.borrow().left.clone());
            }
            p1 = node.borrow().right.clone();
        }

        // 收尾：根节点所在的最右链
        Self::add_path(&mut res, root);
        res
    }
}

// ===== 测试辅助：按层序数组建树（None 表示空） =====
fn build_tree(level_order: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if level_order.is_empty() || level_order[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(level_order[0].unwrap())));
    let mut nodes = vec![root.clone()];
    let mut idx = 1usize;
    let mut i = 0usize;

    while i < nodes.len() && idx < level_order.len() {
        if let Some(v) = level_order[idx] {
            let child = Rc::new(RefCell::new(TreeNode::new(v)));
            nodes[i].borrow_mut().left = Some(child.clone());
            nodes.push(child);
        }
        idx += 1;
        if idx >= level_order.len() {
            break;
        }
        if let Some(v) = level_order[idx] {
            let child = Rc::new(RefCell::new(TreeNode::new(v)));
            nodes[i].borrow_mut().right = Some(child.clone());
            nodes.push(child);
        }
        idx += 1;
        i += 1;
    }
    Some(root)
}

fn check(level_order: &[Option<i32>], want: &[i32]) {
    // 每跑一种解法都重新建树：Morris 会临时改动指针
    assert_eq!(
        Solution::postorder_traversal_recursive(build_tree(level_order)),
        want,
        "递归({:?})",
        level_order
    );
    assert_eq!(
        Solution::postorder_traversal_iterative(build_tree(level_order)),
        want,
        "迭代({:?})",
        level_order
    );
    assert_eq!(
        Solution::postorder_traversal(build_tree(level_order)),
        want,
        "Morris({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(&[n(1), x, n(2), n(3)], &[3, 2, 1]);

    // 示例 2
    check(
        &[n(1), n(2), n(3), n(4), n(5), x, n(8), x, x, n(6), n(7), n(9)],
        &[4, 6, 7, 5, 2, 9, 8, 3, 1],
    );

    // 示例 3：空树
    check(&[], &[]);

    // 示例 4：单节点
    check(&[n(1)], &[1]);

    // 边界：完全二叉树
    check(&[n(1), n(2), n(3), n(4), n(5), n(6), n(7)], &[4, 5, 2, 6, 7, 3, 1]);

    // 边界：只有左子树链 / 只有右子树链
    check(&[n(3), n(2), x, n(1)], &[1, 2, 3]);
    check(&[n(1), x, n(2), x, n(3)], &[3, 2, 1]);

    // 边界：负数值（根 -100，左 -50（其左 0），右 100）
    check(&[n(-100), n(-50), n(100), n(0)], &[0, -50, 100, -100]);

    // 边界：深层右斜链（深度 100，后序是降序）
    let mut right: Vec<Option<i32>> = Vec::new();
    for i in 1..=100 {
        right.push(Some(i));
        right.push(None);
    }
    let want_right: Vec<i32> = (1..=100).rev().collect();
    check(&right, &want_right);

    // 边界：深层左斜链（深度 100，后序是升序）
    let mut left: Vec<Option<i32>> = vec![Some(100)];
    for i in (1..=99).rev() {
        left.push(Some(i));
        left.push(None);
    }
    let want_left: Vec<i32> = (1..=100).collect();
    check(&left, &want_left);

    println!("all tests passed");
}
