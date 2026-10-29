/*
============================================================================
98. 验证二叉搜索树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/validate-binary-search-tree/
分类：二叉搜索树 · 递归 / 中序遍历 / Morris

给你一个二叉树的根节点 root ，判断其是否是一个有效的二叉搜索树。

有效 二叉搜索树定义如下：
- 节点的左子树只包含 严格小于 当前节点的数。
- 节点的右子树只包含 严格大于 当前节点的数。
- 所有左子树和右子树自身必须也是二叉搜索树。

示例 1：
输入：root = [2,1,3]
输出：true

示例 2：
输入：root = [5,1,4,null,null,3,6]
输出：false
解释：根节点的值是 5 ，但是右子节点的值是 4 。

提示：
- 树中节点数目范围在 [1, 10^4] 内
- -2^31 <= Node.val <= 2^31 - 1
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
解法一：递归 + 上下界【非最优】

为什么慢：很多人第一反应是"只比较父子"—— 判断 left.val < root.val 且
root.val < right.val。但 BST 约束的是**整棵子树**的取值范围，不只是直接
父子。比如 [5,1,4,null,null,3,6] 里 4 是 5 的右孩子，比 5 小，直接父子比较
能发现；但在 [10,5,15,null,null,6,20] 里，6 是 15 的左孩子、比 15 小，
父子比较会放过它，然而 6 < 10 违反根约束。仅靠父子比较是不完备的。

换个角度：给每个节点带上"它必须落在的开区间 (lower, upper)"：
  · 根的范围是 (-∞, +∞)；
  · 进入左子树时，上界收紧为当前值；
  · 进入右子树时，下界收紧为当前值；
  · 任何节点不满足 `lower < val < upper` 即失败。
  这等价于"中序遍历必须严格递增"，但不需要真正遍历序列。

关键点：
  · 用 Option<i32> 表示开区间的两端，None 即 ±∞，天然避免 ±2^31 溢出
  · 判断用严格不等号：BST 不允许重复值
  · 只递归一层不够，必须把收紧后的界传下去
  · 空节点返回 true，作为递归出口

以示例 2（root = [5,1,4,null,null,3,6]）为例：
  5 的区间 (-∞, +∞)；
  4 作为右孩子，区间 (5, +∞)，4 不满足 → false ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：中序遍历（显式栈）【非最优】

官方方法二。BST 的中序遍历结果必须严格递增。于是：
  · 用栈做迭代中序：一路向左压栈，弹出时比较当前值与前一个值；
  · 若 `cur <= prev` 则返回 false；否则更新 prev，转向右子树。
  不需要真的保存整个序列，只留前一个值。

关键点：
  · prev 用 Option<i32>，None 表示还没有前驱，避免极值比较出错
  · 比较必须是 `>`，等于也不合法
  · 迭代顺序：压左 → 弹栈处理 → 转右，弹栈时才是"访问"
  · 可以在发现逆序时提前返回，省掉剩余遍历

复杂度分析：
  时间 O(n)：每个节点入栈、出栈各一次
  空间 O(h)：栈只保存当前路径

解法三：Morris 中序遍历【最优解】

官方方法三。中序遍历之所以需要栈，是因为要记住"从某个节点回来后该去右子树"。
Morris 用叶子节点的空右指针把这条"返回路径"直接存进树里（线索）：
  · 若当前节点没有左孩子，直接访问它，转向右孩子；
  · 否则找左子树的最右节点 p（中序前驱）：
      - p.right 为空 → 指向当前节点（建立线索），当前节点向左走；
      - p.right 就是当前节点 → 说明左子树已遍历完，断开线索（还原树），
        访问当前节点，转向右孩子。
  访问节点时比较它与前驱的值是否严格递增即可。

关键点：
  · 线索的建立与断开必须成对出现，否则会破坏输入树
  · 找最右节点时 `p.right != node` 是循环条件，
    防止在已有线索的树上绕圈
  · 判断是否逆序发生在"访问"位置，与普通中序完全一致
  · 发现逆序时先用 bool 记下来、继续走完全程再返回：
    提前 return 会把"已建立但未断开"的线索留在树里，树就被改坏了
  · 全程只用几个指针，辅助空间 O(1)，因此是最优解

以示例 1（root = [2,1,3]）为例：
  node=2 有左孩子，最右节点是 1，1.right 为空 → 建线索 1.right=2，向左走到 1；
  node=1 没有左孩子 → 访问 1（prev 为空），转向右，回到 2；
  node=2 的最右节点 1 的 right 正好指向 2 → 断开线索，访问 2（1 < 2 成立），转右；
  node=3 无左孩子 → 访问 3（2 < 3 成立）→ 中序 1,2,3 严格递增 → true ✔

复杂度分析：
  时间 O(n)：每条边最多走两遍（建线索、断线索）
  空间 O(1)：只用了几个指针，不借助栈

-------- 三种写法对比 --------
  递归上下界：O(n) 时间、O(h) 空间，最直观，适合面试第一版
  中序栈：O(n) 时间、O(h) 空间，把"BST ⇔ 中序递增"直接用出来
  Morris：O(n) 时间、O(1) 空间，标记为【最优解】，但会临时改动树结构
*/

// ---------- 解法一：递归 + 上下界 ----------
pub struct Solution1;

impl Solution1 {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(
            node: Option<&Rc<RefCell<TreeNode>>>,
            lower: Option<i32>,
            upper: Option<i32>,
        ) -> bool {
            let Some(n) = node else {
                return true;
            };
            let n = n.borrow();
            let v = n.val;
            if lower.is_some_and(|l| v <= l) || upper.is_some_and(|u| v >= u) {
                return false;
            }
            check(n.left.as_ref(), lower, Some(v)) && check(n.right.as_ref(), Some(v), upper)
        }

        check(root.as_ref(), None, None)
    }
}

// ---------- 解法二：中序遍历（显式栈） ----------
pub struct Solution2;

impl Solution2 {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stk: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut node = root.clone();
        let mut prev: Option<i32> = None;

        loop {
            while let Some(cur) = node {
                let next = cur.borrow().left.clone();
                stk.push(cur);
                node = next;
            }
            let Some(cur) = stk.pop() else {
                break;
            };
            let (v, next) = {
                let n = cur.borrow();
                (n.val, n.right.clone())
            };
            if prev.is_some_and(|p| v <= p) {
                return false;
            }
            prev = Some(v);
            node = next;
        }
        true
    }
}

// ---------- 解法三：Morris 中序遍历 ----------
pub struct Solution3;

impl Solution3 {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut node = root.clone();
        let mut prev: Option<i32> = None;
        let mut valid = true;

        while let Some(cur) = node.clone() {
            let left = cur.borrow().left.clone();
            if left.is_none() {
                // 访问 cur
                let v = cur.borrow().val;
                if prev.is_some_and(|p| p >= v) {
                    valid = false;
                }
                prev = Some(v);
                node = cur.borrow().right.clone();
            } else {
                // 找左子树的最右节点（中序前驱）
                let mut p = left.clone();
                loop {
                    let next = p.as_ref().unwrap().borrow().right.clone();
                    match next {
                        Some(nx) if !Rc::ptr_eq(&nx, &cur) => p = Some(nx),
                        _ => break,
                    }
                }

                let p = p.unwrap();
                let p_right = p.borrow().right.clone();
                match p_right {
                    None => {
                        // 建立线索，向左
                        p.borrow_mut().right = Some(Rc::clone(&cur));
                        node = left;
                    }
                    Some(link) => {
                        debug_assert!(Rc::ptr_eq(&link, &cur));
                        // 左子树走完，断开线索，访问 cur，转右
                        p.borrow_mut().right = None;
                        let v = cur.borrow().val;
                        if prev.is_some_and(|pv| pv >= v) {
                            valid = false;
                        }
                        prev = Some(v);
                        node = cur.borrow().right.clone();
                    }
                }
            }
        }

        // 注意：即使发现逆序也走完全程，把所有线索还原，保持输入树完整
        valid
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
    assert_eq!(Solution1::is_valid_bst(build_tree(level_order)), want);
    assert_eq!(Solution2::is_valid_bst(build_tree(level_order)), want);
    assert_eq!(Solution3::is_valid_bst(build_tree(level_order)), want);
}

fn main() {
    // 示例 1
    check(&[Some(2), Some(1), Some(3)], true);

    // 示例 2
    check(
        &[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
        false,
    );

    // 边界：单节点
    check(&[Some(1)], true);

    // 边界：只有左孩子，且小于根
    check(&[Some(2), Some(1)], true);

    // 边界：只有右孩子，但小于根
    check(&[Some(2), None, Some(1)], false);

    // 边界：重复值不合法（BST 要求严格）
    check(&[Some(2), Some(2), Some(2)], false);
    check(&[Some(1), Some(1)], false);

    // 边界：根满足、子树内部违反 —— 只比较父子会漏掉
    check(
        &[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)],
        false,
    );

    // 边界：右子树里的节点小于根
    check(
        &[Some(5), Some(4), Some(6), None, None, Some(3), Some(7)],
        false,
    );

    // 边界：合法满 BST
    check(
        &[
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(4),
            Some(6),
        ],
        true,
    );

    // 边界：满 BST 中右子树放了个小值
    check(
        &[
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(6),
            Some(4),
        ],
        false,
    );

    // 边界：int 极值
    check(&[Some(i32::MIN)], true);
    check(&[Some(i32::MAX)], true);
    check(&[Some(i32::MIN), None, Some(i32::MAX)], true);
    check(&[Some(i32::MIN), Some(i32::MIN)], false);

    // 边界：100 层右斜链，递增合法
    let mut chain = vec![Some(1)];
    for i in 2..=100 {
        chain.push(None);
        chain.push(Some(i));
    }
    check(&chain, true);

    println!("all tests passed");
}
