/*
============================================================================
114. 二叉树展开为链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/
分类：二叉树 · 前序遍历 / 原地指针重连

给你二叉树的根结点 root ，请你将它展开为一个单链表：

- 展开后的单链表应该同样使用 TreeNode ，其中 right 子指针指向链表中下一个
  结点，而左子指针始终为 null 。
- 展开后的单链表应该与二叉树 先序遍历 顺序相同。

示例 1：
输入：root = [1,2,5,3,4,null,6]
输出：[1,null,2,null,3,null,4,null,5,null,6]

示例 2：
输入：root = []
输出：[]

示例 3：
输入：root = [0]
输出：[0]

提示：
- 树中结点数在范围 [0, 2000] 内
- -100 <= Node.val <= 100

进阶：你可以使用原地算法（O(1) 额外空间）展开这棵树吗？
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
解法一：前序遍历把节点存进 Vec，再按 Vec 顺序重连【非最优】

为什么慢：遍历本身是 O(n)，额外代价是 O(n) 的节点 Vec，而且重连时又要把
Vec 扫一遍。题目进阶要求 O(1) 额外空间，这个 Vec 正是要被去掉的东西。

换个角度：展开后的链表顺序就是前序序列。先把序列完整拿到手，再统一把每个
节点的 left 清空、right 指向序列里的下一个节点，指针就不会在遍历途中被
改坏。

关键点：
  · 必须先把整棵树的节点收集完再动指针：边遍历边改会丢掉还没访问的子树
  · Vec 里存的是 Rc 句柄而不是值 —— 值会重复，句柄才指向要重连的节点
  · 每个节点的 left 都要清空，包括链表最后一个节点
  · 最后一个节点的 right 必须置空，否则会把输入树里的旧指针带进链表
  · 借用要在语句内结束：先 clone 出 left/right 再操作，不要跨循环持有
    borrow

以示例 1（root = [1,2,5,3,4,null,6]）为例：
  前序序列：1、2、3、4、5、6；
  依次令 1.right=2、2.right=3 …… 6.right=None，且所有 left 置空；
  结果 1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：前序访问一次，重连扫一次 Vec
  空间 O(n)：保存全部节点句柄，正是进阶要消除的部分

解法二：前序 + 显式栈，边遍历边接链【非最优】

为什么慢：省掉了 Vec，但栈的深度仍可能到 O(n)：左斜链上整条路径的节点都
在栈里。空间是 O(h)，最坏仍是 O(n)，依旧不是 O(1)。

换个角度：前序的"下一个访问节点"可以由栈给出。只要保证左孩子先出栈，弹栈
顺序就与递归前序一致，于是可以一边弹一边把上一个访问节点接到当前节点上。

关键点：
  · 先压右孩子、再压左孩子 —— 栈是后进先出，反过来弹栈顺序就变成"根→右→左"
  · 孩子必须在清空 left 之前入栈，否则子树丢失
  · 接链发生在出栈时：prev.right = cur，并把 cur.left 置空
  · 空树直接返回，不要往栈里压 None
  · 借用 prev 时用 as_ref()，否则 Option 会被 move 走

以示例 1（root = [1,2,5,3,4,null,6]）为例：
  弹 1（栈：5、2），前驱为空；把 5 的右 6、2 的右 4 与左 3 入栈；
  弹 2（栈：5、6、4、3），1.right=2；再弹 3、4、5、6，
  依次得到 1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：每个节点入栈、出栈各一次
  空间 O(h)：栈只保存尚未访问的右子树，最坏（左斜链）O(n)

解法三：寻找前驱节点（原地展开，O(1) 额外空间）【最优解】

为什么慢：前两种方法的辅助容器都源于同一个需求 —— 前序访问完左子树后要
记得回来走右子树。容器就是在替我们记这件事。

换个角度：把"回来"的路径直接写进树里。对当前节点 cur：
  · 若 cur 没有左孩子，它已经在正确位置，走向右孩子即可；
  · 否则，展开后 cur 的下一个节点是左子树的第一个节点，而原右子树应该接在
    "左子树最右节点（前序前驱）"的后面。于是：
      pred = 左子树最右节点；pred.right = cur.right；
      cur.right = cur.left；cur.left = None。
  左子树被整体挪到右边后，cur 的右孩子就是下一个待处理节点，继续往右走。

关键点：
  · 顺序不能反：必须先找 pred、先把 cur.right 挂到 pred 上，最后才覆盖
    cur.right；反了右子树就丢了
  · 找 pred 只在左子树的右链上走，条件就是 `p.right.is_some()`
  · 每处理一个节点都要把 left 置空，题目要求最终左指针全为 null
  · 处理完当前节点后只往右走，不要回头再进左子树 —— 它已经被挪到右边，
    整棵树变成一条右链，沿右指针走一遍即可
  · 每个节点只被当作 cur 处理一次，找 pred 走过的边之后不会再走，均摊 O(n)

以示例 1（root = [1,2,5,3,4,null,6]）为例：
  cur=1：pred 是左子树最右的 4，4.right=5，1.right=2，1.left=None；
  cur=2：pred 是 3，3.right=4，2.right=3，2.left=None；
  cur=3：无左孩子，走到 4；4 无左孩子，走到 5；5 无左孩子，走到 6；
  右链 1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：每个节点作为 cur 一次；找 pred 的指针移动总量也是 O(n)
  空间 O(1)：只用 cur、pred 两个指针，没有栈、没有 Vec
*/

// ---------- 解法一：前序遍历 + 数组重建 ----------
pub struct Solution1;

impl Solution1 {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn preorder(
            node: &Option<Rc<RefCell<TreeNode>>>,
            out: &mut Vec<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = node {
                out.push(Rc::clone(n));
                // clone 出孩子，避免借着 borrow 递归
                let (left, right) = {
                    let b = n.borrow();
                    (b.left.clone(), b.right.clone())
                };
                preorder(&left, out);
                preorder(&right, out);
            }
        }

        let mut nodes = Vec::new();
        preorder(root, &mut nodes);

        for i in 0..nodes.len() {
            let next = if i + 1 < nodes.len() {
                Some(Rc::clone(&nodes[i + 1]))
            } else {
                None
            };
            let mut b = nodes[i].borrow_mut();
            // left 一律清空；right 指向序列里的下一个，末尾为 None
            b.left = None;
            b.right = next;
        }
    }
}

// ---------- 解法二：前序 + 显式栈 ----------
pub struct Solution2;

impl Solution2 {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let Some(root_node) = root.clone() else {
            return;
        };

        let mut stk = vec![root_node];
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while let Some(cur) = stk.pop() {
            let (left, right) = {
                let b = cur.borrow();
                (b.left.clone(), b.right.clone())
            };

            // 先压右再压左，左孩子才会先出栈，弹栈顺序才是前序
            if let Some(rn) = right {
                stk.push(rn);
            }
            if let Some(ln) = left {
                stk.push(ln);
            }

            // 孩子已入栈，此时清空 left 不会丢子树
            if let Some(p) = prev.as_ref() {
                p.borrow_mut().right = Some(Rc::clone(&cur));
            }
            cur.borrow_mut().left = None;
            prev = Some(cur);
        }
    }
}

// ---------- 解法三：寻找前驱节点（原地） ----------
pub struct Solution3;

impl Solution3 {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut cur = root.clone();

        while let Some(node) = cur {
            let left = node.borrow().left.clone();
            if let Some(l) = left {
                // 前驱 = 左子树最右节点，即前序中 cur 的前一个节点
                let mut pred = Rc::clone(&l);
                loop {
                    let next = pred.borrow().right.clone();
                    match next {
                        Some(n) => pred = n,
                        None => break,
                    }
                }

                // 先用 pred 收留原右子树，再覆盖 cur.right，顺序反了会丢树
                let right = node.borrow().right.clone();
                pred.borrow_mut().right = right;
                let l2 = node.borrow().left.clone();
                node.borrow_mut().right = l2;
                node.borrow_mut().left = None;
            }

            // 左子树已挪到右边，沿右链继续即可
            cur = node.borrow().right.clone();
        }
    }
}

// ===== 测试辅助：按层序数组建树、按右链收集 =====
const MAX_NODES: usize = 2000; // 题目上限，用来兜住成环的错误

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

// 沿 right 收集：同时检查 left 全为空、右链没有环
fn collect_right_chain(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut cur = root.clone();
    while let Some(node) = cur {
        let (val, left, right) = {
            let b = node.borrow();
            (b.val, b.left.clone(), b.right.clone())
        };
        assert!(left.is_none(), "left 未清空: val={}", val);
        values.push(val);
        // 指针接错可能成环，限制步数让断言失败而不是死循环
        assert!(values.len() <= MAX_NODES, "右链成环");
        cur = right;
    }
    values
}

fn check(level_order: &[Option<i32>], want: &[i32]) {
    let mut t1 = build_tree(level_order);
    Solution1::flatten(&mut t1);
    assert_eq!(collect_right_chain(&t1), want, "Solution1 {:?}", level_order);

    let mut t2 = build_tree(level_order);
    Solution2::flatten(&mut t2);
    assert_eq!(collect_right_chain(&t2), want, "Solution2 {:?}", level_order);

    let mut t3 = build_tree(level_order);
    Solution3::flatten(&mut t3);
    assert_eq!(collect_right_chain(&t3), want, "Solution3 {:?}", level_order);
}

fn main() {
    // 示例 1：左子树内部还有左右孩子，右子树非空
    check(
        &[
            Some(1),
            Some(2),
            Some(5),
            Some(3),
            Some(4),
            None,
            Some(6),
        ],
        &[1, 2, 3, 4, 5, 6],
    );

    // 示例 2：空树
    check(&[], &[]);

    // 示例 3：单节点
    check(&[Some(0)], &[0]);

    // 边界：只有左孩子的斜链
    check(&[Some(1), Some(2), None, Some(3)], &[1, 2, 3]);

    // 边界：只有右孩子的斜链
    check(
        &[Some(1), None, Some(2), None, Some(3)],
        &[1, 2, 3],
    );

    // 反直觉：根有左子树，同时右子树非空 —— 右子树必须挂到前驱后面
    check(
        &[Some(2), Some(1), Some(4), None, None, Some(3)],
        &[2, 1, 4, 3],
    );

    // 反直觉：根右子树非空，左子树有右链，前驱不是左孩子本身
    check(
        &[
            Some(1),
            Some(2),
            Some(5),
            None,
            Some(4),
            None,
            Some(6),
        ],
        &[1, 2, 4, 5, 6],
    );

    // 满二叉树：前序是 4,2,1,3,5,6
    check(
        &[
            Some(4),
            Some(2),
            Some(5),
            Some(1),
            Some(3),
            None,
            Some(6),
        ],
        &[4, 2, 1, 3, 5, 6],
    );

    // 左斜链 200 层
    {
        let mut level_order = vec![Some(1), Some(2)];
        // 每层只有一个左孩子：成对写入 (None, 下一个值)
        for i in 3..=200 {
            level_order.push(None);
            level_order.push(Some(i));
        }
        let want: Vec<i32> = (1..=200).collect();
        check(&level_order, &want);
    }

    // 右斜链 200 层
    {
        let mut level_order = vec![Some(1)];
        for i in 2..=200 {
            level_order.push(None);
            level_order.push(Some(i));
        }
        let want: Vec<i32> = (1..=200).collect();
        check(&level_order, &want);
    }

    println!("all tests passed");
}
