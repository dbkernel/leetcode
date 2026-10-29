/*
============================================================================
889. 根据前序和后序遍历构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
分类：二叉树 · 分治构造（前序 + 后序，答案不唯一）

给定两个整数数组，preorder 和 postorder ，其中 preorder 是一个具有 无重复
值的二叉树的前序遍历，postorder 是同一棵树的后序遍历，重构并返回二叉树。

如果存在多个答案，您可以返回其中 任何 一个。

示例 1：
输入：preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
输出：[1,2,3,4,5,6,7]

示例 2：
输入: preorder = [1], postorder = [1]
输出：[1]

提示：
- 1 <= preorder.length <= 30
- 1 <= preorder[i] <= preorder.length
- preorder 中所有值都 不同
- postorder.length == preorder.length
- 1 <= postorder[i] <= postorder.length
- postorder 中所有值都 不同
- 保证 preorder 和 postorder 是同一棵二叉树的前序遍历和后序遍历
*/
use std::cell::RefCell;
use std::collections::HashMap;
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
解法：递归 + 后序下标哈希表【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每层都必须确定"左子树
占了多长"，而这一步靠查表降到了 O(1)。

为什么慢：前序第一个元素是根、后序最后一个元素也是根，根本身不费事，难在切
分点。左子树的根必然是 `preorder[pre_l + 1]`，它在后序中的位置决定了左子树的
长度；若每层线性扫描后序区间去找这个位置，链状树上有 O(n) 层，退化成
O(n^2)。

换个角度：`preorder[pre_l + 1]` 在后序中的下标 p 就是左子树在后序里的终点。
于是左子树节点数 = `p - post_l + 1`，一次查表就把前序、后序四个端点全部切出来。

关键点：
  · 前序的根在左端 `pre_l`，后序的根在右端 `post_r`，两侧同时收缩
  · 左子树长度必须用**后序下标**算：`pos[preorder[pre_l + 1]] - post_l + 1`，
    用前序下标算会得到同一段长度却对不上边界
  · 递归出口有两个：`pre_l > pre_r` 返回空；`pre_l == pre_r` 时直接返回叶子，
    否则下一步取 `preorder[pre_l + 1]` 会越界
  · 答案不唯一：某节点只有一个孩子时，前序和后序都无法区分这孩子挂左还是挂
    右，本实现一律当左孩子
  · 正因为不唯一，自测不能断言层序，只能把结果重新前序/后序遍历后与原序列
    逐项比对（两者相等当且仅当它是同一棵树的合法还原）
  · Rust 用 `i32` 传下标并统一成「含两端」的闭区间，减少切片开销

以示例 1（preorder = [1,2,4,5,3,6,7]，postorder = [4,5,2,6,7,3,1]）为例：
  根 1，左孩子 2 在后序下标 2 → left_size = 2 - 0 + 1 = 3
  左：前序 [2,4,5]、后序 [4,5,2] → 根 2，左孩子 4 在后序下标 0
      left_size = 1 → 左 [4]、右 [5]
  右：前序 [3,6,7]、后序 [6,7,3] → 根 3，左孩子 6 在后序下标 3
      left_size = 1 → 左 [6]、右 [7]
  得到 [1,2,3,4,5,6,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，每个节点只被创建一次
  空间 O(n)：HashMap 存 n 项；递归栈 O(h)，链状时也是 O(n)
*/

// ---------- 解法：递归 + 后序下标哈希表（最优解） ----------
// 单文件 rustc 编译时平台上并不存在 Solution 类型，这里显式声明
pub struct Solution;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // pos：值 → 后序下标，用来把「左孩子」翻译成「左子树的长度」
        let mut pos: HashMap<i32, i32> =
            HashMap::with_capacity(postorder.len());
        for (i, &v) in postorder.iter().enumerate() {
            pos.insert(v, i as i32);
        }
        let n = preorder.len() as i32;
        Self::build(&preorder, &pos, 0, n - 1, 0, n - 1)
    }

    fn build(
        pre: &[i32],
        pos: &HashMap<i32, i32>,
        pre_l: i32,
        pre_r: i32,
        post_l: i32,
        post_r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_l > pre_r {
            return None;
        }

        let root_val = pre[pre_l as usize];
        // 只剩一个节点时没有孩子，继续取 pre[pre_l + 1] 就越界了
        if pre_l == pre_r {
            return Some(Rc::new(RefCell::new(TreeNode::new(root_val))));
        }

        let left_size = pos[&pre[(pre_l + 1) as usize]] - post_l + 1;

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::build(
                pre,
                pos,
                pre_l + 1,
                pre_l + left_size,
                post_l,
                post_l + left_size - 1,
            ),
            // 右子树的根在后序的 post_r - 1 处，那一位是根，不再划给别人
            right: Self::build(
                pre,
                pos,
                pre_l + left_size + 1,
                pre_r,
                post_l + left_size,
                post_r - 1,
            ),
        })))
    }
}

// ===== 测试辅助：前序/后序遍历、节点计数 =====
fn pre_order(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
    let Some(n) = node else {
        return;
    };
    let n = n.borrow();
    out.push(n.val);
    pre_order(n.left.as_ref(), out);
    pre_order(n.right.as_ref(), out);
}

fn post_order(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
    let Some(n) = node else {
        return;
    };
    let n = n.borrow();
    post_order(n.left.as_ref(), out);
    post_order(n.right.as_ref(), out);
    out.push(n.val);
}

fn count_nodes(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            1 + count_nodes(n.left.as_ref()) + count_nodes(n.right.as_ref())
        }
    }
}

// 答案不唯一，所以不比对层序：只要求还原出的树能重现原始的两条序列
fn check(preorder: &[i32], postorder: &[i32]) {
    let t = Solution::construct_from_pre_post(
        preorder.to_vec(),
        postorder.to_vec(),
    );

    let mut got_pre = Vec::new();
    let mut got_post = Vec::new();
    pre_order(t.as_ref(), &mut got_pre);
    post_order(t.as_ref(), &mut got_post);

    assert_eq!(got_pre, preorder, "postorder {:?}", postorder);
    assert_eq!(got_post, postorder, "preorder {:?}", preorder);
    // 节点数相同才说明没有多建或漏建节点
    assert_eq!(count_nodes(t.as_ref()), preorder.len() as i32);
}

fn main() {
    // 示例 1
    check(
        &[1, 2, 4, 5, 3, 6, 7],
        &[4, 5, 2, 6, 7, 3, 1],
    );

    // 示例 2：单节点，验证 pre_l == pre_r 的出口
    check(&[1], &[1]);

    // 关键点用例：根只有一个左孩子，取 preorder[pre_l + 1] 后必须及时停
    check(&[1, 2], &[2, 1]);

    // 反直觉用例：链 [1,2,3] 与 [3,2,1] 同时对应「全左链」和「全右链」，
    // 这里不校验形状，只要能重现两条序列即可
    check(&[1, 2, 3], &[3, 2, 1]);

    // 不平衡树：3 只有左孩子 6，左子树长度取到最大
    check(&[1, 2, 4, 5, 3, 6], &[4, 5, 2, 6, 3, 1]);

    // 满二叉树 15 个节点，两侧子树长度都不为 0
    check(
        &[1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15],
        &[8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1],
    );

    // 边界：题面上限 30 个节点的单链
    let pre: Vec<i32> = (1..=30).collect();
    let post: Vec<i32> = (1..=30).rev().collect();
    check(&pre, &post);

    println!("all tests passed");
}
