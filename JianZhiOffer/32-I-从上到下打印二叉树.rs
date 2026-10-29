/*
============================================================================
LCR 149 / 剑指 Offer 32 - I. 从上到下打印二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/
分类：二叉树 · 层序遍历（BFS / 队列）

一棵圣诞树记作根节点为 root 的二叉树，节点值为该位置装饰彩灯的颜色编号。
请按照从 左 到 右 的顺序返回每一层彩灯编号。

说明：本题原名「从上到下打印二叉树」，leetcode.cn 现已改用
decorateRecord 作为函数名，题面意思不变。

示例 1：
输入：root = [8,17,21,18,null,null,6]
输出：[8,17,21,18,6]

提示：
- 节点总数 <= 1000
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
解法：BFS + 队列【最优解】

为什么慢：如果按「深度优先」去搜，虽然每个节点只访问一次，但得到的顺序是
一条路走到黑（根-左子树整条-右子树整条），要还原成层序还得额外记层号再排序，
反而把简单问题复杂化。

换个角度：题目要的正是「先来先访问」的顺序 —— 先遇到第 1 层，再遇到第 2 层。
这正是先进先出队列的语义：把根入队，每次从队头取出一个节点输出，并把它
非空的左右孩子依次接到队尾。

关键点：
  · 入队顺序必须是「先左后右」，否则同一层内的顺序会反
  · 孩子为空时不要入队，否则会把 None 带进队列，后续取值崩溃
  · 根为空要单独处理：直接返回空 Vec，不要入队
  · 队列里同时最多存两层的节点（当前层剩余 + 下一层），空间 O(w)，
    w 为最大层宽；本题不要求按层分组，所以不需要记录层大小
  · 用 VecDeque：pop_front 是 O(1)，Vec 的 remove(0) 是 O(n)
  · 队列里存 Rc 克隆（引用计数 +1），不复制节点本身

以示例 1（root = [8,17,21,18,null,null,6]）为例：
  队列 [8] → 输出 8，入队 17、21
  输出 17，入队 18 → 输出 21，入队 6
  输出 18、6 → [8,17,21,18,6]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为树的最大层宽，最坏（完全二叉树最后一层）O(n)
*/
struct Solution;

impl Solution {
    pub fn decorate_record(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let r = match root {
            Some(r) => r,
            None => return res,
        };

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(r);
        while let Some(node) = q.pop_front() {
            res.push(node.borrow().val);

            // 先左后右，保证同层内从左到右
            let left = node.borrow().left.clone();
            if let Some(l) = left {
                q.push_back(l);
            }
            let right = node.borrow().right.clone();
            if let Some(r) = right {
                q.push_back(r);
            }
        }
        res
    }
}

// ===== 测试辅助：按层序数组（None 表示空）建树 =====
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
        Solution::decorate_record(build_tree(level_order)),
        want,
        "decorateRecord({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(&[n(8), n(17), n(21), n(18), x, x, n(6)], &[8, 17, 21, 18, 6]);

    // 边界：空树
    check(&[], &[]);

    // 边界：单节点
    check(&[n(1)], &[1]);

    // 边界：完全二叉树，验证同层从左到右
    check(
        &[n(1), n(2), n(3), n(4), n(5), n(6), n(7)],
        &[1, 2, 3, 4, 5, 6, 7],
    );

    // 边界：左斜链 / 右斜链
    check(&[n(1), n(2), x, n(3)], &[1, 2, 3]);
    check(&[n(1), x, n(2), x, n(3)], &[1, 2, 3]);

    // 边界：右孩子缺失、左孩子存在（不能因为右空就提前结束本层）
    check(&[n(1), n(2), n(3), n(4), x, x, n(5)], &[1, 2, 3, 4, 5]);

    // 边界：负数值
    check(&[n(-100), n(-50), n(100), n(0)], &[-100, -50, 100, 0]);

    // 边界：满二叉树三层，层序就是 1..15
    let level_order: Vec<Option<i32>> = (1..=15).map(Some).collect();
    let want: Vec<i32> = (1..=15).collect();
    check(&level_order, &want);

    println!("all tests passed");
}
