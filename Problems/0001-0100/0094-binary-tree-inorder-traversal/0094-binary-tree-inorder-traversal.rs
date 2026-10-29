/*
============================================================================
94. 二叉树的中序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-inorder-traversal/
分类：二叉树 · 遍历（递归 / 栈 / Morris）

给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。

示例 1：
输入：root = [1,null,2,3]
输出：[1,3,2]

示例 2：
输入：root = []
输出：[]

示例 3：
输入：root = [1]
输出：[1]

提示：
树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100

进阶: 递归算法很简单，你可以通过迭代算法完成吗？
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

官方方法一。中序遍历的定义是「左 → 根 → 右」，直接按定义写递归：
  · 空节点返回；
  · 先递归左子树，再把当前节点值加入答案，最后递归右子树。

关键点：
  · 与 144 前序遍历只差「访问语句的位置」：中序夹在两个递归之间
  · 中序遍历对二叉搜索树有特殊意义——结果一定是升序序列（后续 98 题会用到）
  · 递归深度 = 树高，最坏（链状树）为 O(n)

复杂度分析：
  时间 O(n)：每个节点恰好访问一次
  空间 O(h)：递归栈

解法二：迭代（显式栈模拟）【非最优】

官方方法二。递归的展开顺序是「一路向左、到底后访问、再转向右子树」，
用显式栈照着写即可：
  · 只要当前节点非空，就压栈并一路向左（先不访问，因为左子树还没处理完）；
  · 走到空就弹栈——此时该节点是「左子树已处理完」的节点，访问它，然后转向它的右子树。

关键点：
  · 与前序遍历的唯一差别：前序在入栈前记录答案，中序在出栈时记录
  · 循环条件仍是「栈非空 或 当前节点非空」
  · 内层循环只压栈不访问，这点最容易和先序写混
  · 栈深度最多为树高 O(h)，栈里存 Rc 克隆（引用计数 +1，不复制节点）

复杂度分析：
  时间 O(n)
  空间 O(h)：显式栈

解法三：Morris 中序遍历【最优解】

官方方法三。用叶子节点空着的 right 指针做「回退线索」，把空间降到 O(1)：
  · 若当前节点 x 没有左孩子：访问 x，然后走向右孩子；
  · 若 x 有左孩子：找到 x 左子树的最右节点（即 x 在中序下的前驱 predecessor）。
      - predecessor 的 right 为空 → 说明第一次到达：把 right 指向 x（建立线索），
        然后走向 x 的左孩子（此时不访问 x，因为左子树还没遍历）；
      - predecessor 的 right 指向 x → 说明左子树已遍历完：拆掉线索，
        访问 x，然后走向 x 的右孩子。
  · 重复以上步骤直到 x 为空。

关键点：
  · 与 Morris 前序的关键差别：前序在「建立线索」时访问，中序在「拆除线索」时才访问
  · 拆线索必须做，否则破坏原树结构，并且 Rc 会形成引用环（内存泄漏）
  · 比较「线索是否指向自己」必须用 Rc::ptr_eq（比地址），不能用 ==（比结构）
  · 每次取 left/right 都先 clone 出 Rc 再借用，避免 RefCell 借用冲突

以 root = [1,null,2,3] 为例（1 只有右子树 2，2 的左孩子是 3）：
  中序 = 左(空) → 1 → 右子树的中序 = 3 → 2，结果为 [1,3,2]，与示例一致。
  Morris：1 无左孩子 → 访问 1，走向 2；2 有左孩子 3，3 无右孩子 →
          建立线索 3.right = 2，走向 3；3 无左孩子 → 访问 3，走向线索 2；
          回到 2，发现线索指向自己 → 拆线索，访问 2，走向 2 的右孩子（空）→ 结束。
          得到 [1,3,2]

复杂度分析：
  时间 O(n)
  空间 O(1)：只用两个指针
*/
struct Solution;

impl Solution {
    // ---------- 解法一：递归 ----------
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::inorder(&n.borrow().left, res);
            res.push(n.borrow().val);
            Self::inorder(&n.borrow().right, res);
        }
    }

    pub fn inorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::inorder(&root, &mut res);
        res
    }

    // ---------- 解法二：迭代（栈） ----------
    pub fn inorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            // 一路向左，只压栈不访问
            while let Some(node) = cur.clone() {
                cur = node.borrow().left.clone();
                stack.push(node);
            }
            // 弹出时左子树已处理完，此时才访问
            let node = stack.pop().unwrap();
            res.push(node.borrow().val);
            cur = node.borrow().right.clone();
        }
        res
    }

    // ---------- 解法三：Morris 中序遍历（最优解） ----------
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut x = root;

        while let Some(node) = x.clone() {
            let left = node.borrow().left.clone();
            match left {
                Some(l) => {
                    // 找 x 在中序下的前驱：左子树的最右节点
                    let mut predecessor = l;
                    loop {
                        let next = predecessor.borrow().right.clone();
                        match next {
                            Some(r) if !Rc::ptr_eq(&r, &node) => predecessor = r,
                            _ => break,
                        }
                    }
                    let has_thread = match predecessor.borrow().right.clone() {
                        Some(r) => Rc::ptr_eq(&r, &node),
                        None => false,
                    };

                    if !has_thread {
                        // 第一次到达：建立线索，转向左子树（此刻不访问 x）
                        predecessor.borrow_mut().right = Some(node.clone());
                        x = node.borrow().left.clone();
                    } else {
                        // 左子树已遍历完：拆线索，访问 x，转向右子树
                        res.push(node.borrow().val);
                        predecessor.borrow_mut().right = None;
                        x = node.borrow().right.clone();
                    }
                }
                None => {
                    // 没有左孩子，直接访问并转向右孩子
                    res.push(node.borrow().val);
                    x = node.borrow().right.clone();
                }
            }
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
    assert_eq!(
        Solution::inorder_traversal_recursive(build_tree(level_order)),
        want,
        "递归({:?})",
        level_order
    );
    assert_eq!(
        Solution::inorder_traversal_iterative(build_tree(level_order)),
        want,
        "迭代({:?})",
        level_order
    );
    assert_eq!(
        Solution::inorder_traversal(build_tree(level_order)),
        want,
        "Morris({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(&[n(1), x, n(2), n(3)], &[1, 3, 2]);

    // 示例 2：空树
    check(&[], &[]);

    // 示例 3：单节点
    check(&[n(1)], &[1]);

    // 边界：完全二叉树
    check(&[n(1), n(2), n(3), n(4), n(5), n(6), n(7)], &[4, 2, 5, 1, 6, 3, 7]);

    // 边界：只有左子树链
    check(&[n(3), n(2), x, n(1)], &[1, 2, 3]);

    // 边界：只有右子树链
    check(&[n(1), x, n(2), x, n(3)], &[1, 2, 3]);

    // 边界：负数值（根 -100，左 -50（其左 0），右 100）
    check(&[n(-100), n(-50), n(100), n(0)], &[0, -50, -100, 100]);

    // 边界：二叉搜索树，中序应为升序
    check(&[n(5), n(3), n(8), n(1), n(4), n(7), n(9)], &[1, 3, 4, 5, 7, 8, 9]);

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
        let mut arr = vec![Some(100)];
        for i in (1..=99).rev() {
            arr.push(Some(i));
            arr.push(None);
        }
        let want: Vec<i32> = (1..=100).collect();
        check(&arr, &want);
    }

    println!("all tests passed");
}
