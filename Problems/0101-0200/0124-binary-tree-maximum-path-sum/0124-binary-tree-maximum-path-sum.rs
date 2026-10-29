/*
============================================================================
124. 二叉树中的最大路径和（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-maximum-path-sum/
分类：二叉树 · 后序递归（单边贡献）

二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。
同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定
经过根节点。

路径和 是路径中各节点值的总和。

给你一个二叉树的根节点 root ，返回其 最大路径和 。

示例 1：
输入：root = [1,2,3]
输出：6
解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6

示例 2：
输入：root = [-10,9,20,null,null,15,7]
输出：42
解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42

提示：
- 树中节点数目范围是 [1, 3 * 10^4]
- -1000 <= Node.val <= 1000
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
解法：后序递归，返回单边最大贡献【最优解】

为什么慢：路径可以在任意节点拐弯，最直白的做法是枚举每个节点当拐点，
再分别求左右子树的最大下行和。若求下行和时每次都重新往下扫一遍，
同一条链会被它的每个祖先各扫一次，斜链上退化成 O(n^2)，而树本身只有 n 个点。

换个角度：把问题拆成"贡献值"——以某节点为起点、只朝一个方向往下走能取得的
最大和。它天然可以后序返回：左、右贡献各自与 0 取 max，挑较大的一边接上
自己。而"以当前节点为拐点"的完整路径和是 左 + 当前 + 右，它只用来更新全局
答案，不参与返回。这样每个节点只算一次。

关键点：
  · 返回给父节点的只能是一条腿：路径一旦向上回到父节点，就不能再分叉，
    同时带两条腿会和父节点组成环
  · 单边贡献与 0 取 max：负数贡献宁可不要，取 0 等价于"从当前节点开始"
  · 全局答案用 左 + 右 + 当前值 更新，并且必须在后序位置（拿到左右结果后）
  · 答案初值取 i32::MIN，不能取 0：全负树必须返回最大的那个负数
  · 更新答案和返回值是两个不同的量，不要用返回值去更新答案
  · 先用 &mut i32 传出答案，并克隆左右孩子后再递归，避免 RefCell 重借用

手推一遍：以示例 2（root = [-10,9,20,null,null,15,7]）为例：
  叶子 15、7：更新 15、7，各自返回 15、7
  叶子 9：更新 9，返回 9
  节点 20：15 + 20 + 7 = 42 更新答案；返回 20 + max(15,7) = 35
  节点 -10：9 + 35 - 10 = 34，不及 42；返回 -10 + 35 = 25
  答案 42 ✔

复杂度分析：
  时间 O(n)：每个节点只进出一次，贡献值一次算清，没有重复扫描
  空间 O(h)：递归栈深度等于树高，最坏（斜链）O(n)
*/

// 单文件 rustc 编译时平台不提供 Solution，需要显式声明
pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 返回以 node 为起点、只朝一个方向向下的最大路径和
        fn gain(node: Option<&Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
            let Some(n) = node else {
                return 0;
            };
            // 借用只在本块内存在，递归前必须释放
            let (val, left, right) = {
                let b = n.borrow();
                (b.val, b.left.clone(), b.right.clone())
            };

            // 负贡献不如不取，取 0 等价于路径从当前节点开始
            let l = gain(left.as_ref(), best).max(0);
            let r = gain(right.as_ref(), best).max(0);

            // 拐点在当前节点：两条腿可以同时要，但只用于更新答案
            *best = (*best).max(val + l + r);

            // 交给父节点只能带一条腿，否则路径在父节点处分叉成环
            val + l.max(r)
        }

        let mut best = i32::MIN;
        gain(root.as_ref(), &mut best);
        best
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
    assert_eq!(
        Solution::max_path_sum(build_tree(level_order)),
        want,
        "level_order = {:?}",
        level_order
    );
}

fn main() {
    // 示例 1
    check(&[Some(1), Some(2), Some(3)], 6);

    // 示例 2
    check(
        &[
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ],
        42,
    );

    // 边界：单节点，答案就是节点值本身
    check(&[Some(-3)], -3);

    // 反直觉：全负树不能返回 0，应取最大的单个节点
    check(&[Some(-5), Some(-2), Some(-3)], -2);

    // 反直觉：负孩子贡献为负，必须丢掉，答案仍是根
    check(&[Some(2), Some(-1)], 2);

    // 反直觉：根为负，最优路径完全在右子树
    check(&[Some(-1), Some(2)], 2);

    // 关键点：路径可以只走一侧，不能强求左右都取
    check(&[Some(1), Some(-2), Some(3)], 4);

    // 关键点：最长路径不经过根，藏在左子树内部
    check(
        &[Some(1), Some(2), Some(3), Some(4), Some(5)],
        11,
    );

    // 边界：全 0，答案为 0
    check(&[Some(0), Some(0), Some(0)], 0);

    // 边界：负根 + 长负链，答案在链中间取到
    check(
        &[Some(-10), Some(-20), Some(-30), None, Some(5)],
        5,
    );

    // 边界：1000 层右斜链全为 1，答案 1000，用来压递归深度
    let mut chain = vec![Some(1)];
    for _ in 2..=1000 {
        chain.push(None);
        chain.push(Some(1));
    }
    check(&chain, 1000);

    println!("all tests passed");
}
