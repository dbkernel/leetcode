/*
============================================================================
144. 二叉树的前序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-preorder-traversal/
分类：二叉树 · 遍历（递归 / 栈 / Morris）

给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

示例 1：
输入：root = [1,null,2,3]
输出：[1,2,3]

示例 2：
输入：root = [1,2,3,4,5,null,8,null,null,6,7,9]
输出：[1,2,4,5,6,7,3,8,9]

示例 3：
输入：root = []
输出：[]

示例 4：
输入：root = [1]
输出：[1]

提示：
树中节点数目在范围 [0, 100] 内
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

官方方法一。前序遍历的定义就是「根 → 左 → 右」，直接按定义写递归：
  · 空节点返回；
  · 先把当前节点值加入答案；
  · 再递归左子树、递归右子树。

关键点：
  · 三个步骤的顺序就是前/中/后序的唯一区别，前序是「访问」放在最前面
  · 用辅助函数把结果容器传进去（借用 &mut Vec 即可，不需要搬移所有权）
  · 递归深度 = 树高；最坏（链状树）为 O(n)

复杂度分析：
  时间 O(n)：每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高，平均 O(log n)、最坏 O(n)

解法二：迭代（显式栈模拟）【非最优】

官方方法二。把递归里「隐式的栈」显式写出来：
  · 只要当前节点非空，就访问它、把它压栈，然后一路向左走；
  · 走到空就弹出栈顶、转向它的右子树。
这正好对应递归「先处理根、再处理左、最后处理右」的展开顺序。

关键点：
  · 循环条件是「栈非空 或 当前节点非空」，两个条件缺一不可
  · 入栈的时机是「访问节点时」，出栈后立刻转向它的右子树
  · 与中序遍历的代码只差一行：前序在入栈前就记录答案，中序在出栈时记录
  · 栈的深度最多为树高，同样是 O(h)
  · 栈里存 Rc 克隆（只是引用计数 +1），不会复制节点本身

复杂度分析：
  时间 O(n)
  空间 O(h)：显式栈

解法三：Morris 遍历【最优解】

官方方法三。前序、中序、后序都能用 Morris 做到 O(1) 额外空间（不计答案数组），
思路是利用叶子节点空着的 right 指针当「回退线索」：
  · 若当前节点没有左子节点：直接访问它，然后走向右子节点；
  · 若有左子节点：在左子树里找到当前节点在中序下的前驱（左子树的最右节点）。
      - 前驱的 right 为空 → 说明第一次来到这里：把 right 指向当前节点（建立线索），
        访问当前节点，然后走向左子节点；
      - 前驱的 right 指向当前节点 → 说明左子树已经遍历完：把线索拆掉（恢复原树），
        走向右子节点。
每个有左子树的节点会被访问两次，无左子树的节点只访问一次，总代价仍是 O(n)。

关键点：
  · 前序与中序的 Morris 差别：前序在「建立线索」时就访问当前节点，
    中序在「拆除线索」时才访问
  · 拆线索这一步必须做，否则会破坏原树结构，并且 Rc 会形成引用环（内存泄漏）
  · 找前驱时循环条件是 `right 存在且不等于当前节点`，
    两个条件都要有：既要走到最右，又要在已经建立线索时及时停下
  · 比较「线索是否指向自己」必须用 Rc::ptr_eq（比地址），不能用 ==（比结构）
  · 建立线索后要 continue，因为此时 p1 已经变成左子节点，不能再往下走 p1.right
  · 每次取 right/left 都先 clone 出 Rc 再借用，避免 RefCell 的借用冲突

以 root = [1,null,2,3] 为例（1 只有右子树 2，2 的左孩子是 3）：
  递归/迭代：1 → 右子树 2 → 2 的左孩子 3，得到 [1,2,3]
  Morris：1 没有左孩子 → 访问 1，走向 2；2 有左孩子 3，3 无右孩子 →
          建立线索 3.right=2，访问 2，走向 3；3 无左孩子 → 访问 3，走向线索 2；
          回到 2 时拆线索，走向 2 的右孩子（空）→ 结束。得到 [1,2,3]

复杂度分析：
  时间 O(n)：每个节点至多被访问两次
  空间 O(1)：只用了两个指针，不改动额外数据结构
*/
struct Solution;

impl Solution {
    // ---------- 解法一：递归 ----------
    fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            res.push(n.borrow().val);
            Self::preorder(&n.borrow().left, res);
            Self::preorder(&n.borrow().right, res);
        }
    }

    pub fn preorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::preorder(&root, &mut res);
        res
    }

    // ---------- 解法二：迭代（栈） ----------
    pub fn preorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root;
        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur.clone() {
                res.push(node.borrow().val);
                stack.push(node.clone());
                cur = node.borrow().left.clone();
            }
            let node = stack.pop().unwrap();
            cur = node.borrow().right.clone();
        }
        res
    }

    // ---------- 解法三：Morris 遍历（最优解） ----------
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut p1 = root;

        while let Some(node) = p1.clone() {
            let left = node.borrow().left.clone();
            match left {
                Some(l) => {
                    // 找左子树中最右节点（当前节点在中序下的前驱）
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
                        // 第一次到达：建立线索，访问当前节点，转向左子树
                        res.push(node.borrow().val);
                        p2.borrow_mut().right = Some(node.clone());
                        p1 = node.borrow().left.clone();
                        continue;
                    }
                    // 第二次到达：拆除线索，转向右子树
                    p2.borrow_mut().right = None;
                }
                None => res.push(node.borrow().val),
            }
            p1 = node.borrow().right.clone();
        }
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
        Solution::preorder_traversal_recursive(build_tree(level_order)),
        want,
        "递归({:?})",
        level_order
    );
    assert_eq!(
        Solution::preorder_traversal_iterative(build_tree(level_order)),
        want,
        "迭代({:?})",
        level_order
    );
    assert_eq!(
        Solution::preorder_traversal(build_tree(level_order)),
        want,
        "Morris({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(&[n(1), x, n(2), n(3)], &[1, 2, 3]);

    // 示例 2
    check(
        &[n(1), n(2), n(3), n(4), n(5), x, n(8), x, x, n(6), n(7), n(9)],
        &[1, 2, 4, 5, 6, 7, 3, 8, 9],
    );

    // 示例 3：空树
    check(&[], &[]);

    // 示例 4：单节点
    check(&[n(1)], &[1]);

    // 边界：只有左子树链
    check(&[n(1), n(2), x, n(3)], &[1, 2, 3]);

    // 边界：只有右子树链
    check(&[n(1), x, n(2), x, n(3)], &[1, 2, 3]);

    // 边界：完全二叉树
    check(&[n(1), n(2), n(3), n(4), n(5), n(6), n(7)], &[1, 2, 4, 5, 3, 6, 7]);

    // 边界：负数值
    check(&[n(-100), n(-50), n(100), n(0)], &[-100, -50, 0, 100]);

    // 边界：深层右斜链（深度 100）
    {
        let mut arr = Vec::new();
        for i in 1..=100 {
            arr.push(Some(i));
            arr.push(None);
        }
        let want: Vec<i32> = (1..=100).collect();
        check(&arr, &want);
    }

    // 边界：深层左斜链（深度 100）
    {
        let mut arr = vec![Some(1)];
        for i in 2..=100 {
            arr.push(Some(i));
            arr.push(None);
        }
        let want: Vec<i32> = (1..=100).collect();
        check(&arr, &want);
    }

    println!("all tests passed");
}
