/*
============================================================================
LCR 151 / 剑指 Offer 32 - III. 从上到下打印二叉树 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/
分类：二叉树 · 层序遍历（BFS / 双端填充）

一棵圣诞树记作根节点为 root 的二叉树，节点值为该位置装饰彩灯的颜色编号。
请按照如下规则记录彩灯装饰结果：

- 第一层按照从左到右的顺序记录
- 除第一层外每一层的记录顺序均与上一层相反。即第一层为从左到右，
  第二层为从右到左。

说明：本题原名「从上到下打印二叉树 III」，leetcode.cn 现已改用
decorateRecord 作为函数名，题面意思不变。

示例 1：
输入：root = [8,17,21,18,null,null,6]
输出：[[8],[21,17],[18,6]]

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
解法一：BFS 按层收集 + 奇数层反转【非最优】

为什么慢：直接的 BFS 只能保证「从左到右」的顺序；要得到之字形，最顺手的
补丁是每层收完再整体翻转一次。代码最短，但奇数层的元素被多写了一遍，
每一层都是一次额外的 O(层宽) 操作，总多出 O(n) 的搬运。

换个角度：与其收完再整体翻转，不如「收的时候就放到正确的一端」。
一层要么全从头填，要么全从尾填，用双端队列（或预分配数组 + 首尾指针）
就能一步到位。

关键点：
  · 判断方向只能看层号，不能看节点值或队列长度
  · 反转只作用于奇数层，层号从 0 开始数
  · 保证本题正确性的核心仍是先左后右入队

复杂度分析：
  时间 O(n)：遍历 O(n)，最坏反转总量也不超过 O(n)
  空间 O(w)：队列与当前层

解法二：BFS + 按方向从两端填充【最优解】

按层处理，每层开始前记录本层节点数 size，然后：
  · 偶数层（0、2、4…）：从数组左端向右填，得到从左到右；
  · 奇数层（1、3、5…）：从数组右端向左填，得到从右到左。
孩子入队的方向（左、右）始终不变 —— 只翻转当前层的输出，不翻转遍历顺序。

关键点：
  · 需要翻转的是「当前层写入的位置」，而不是「孩子入队的顺序」
  · 遍历节点的顺序永远是左孩子在前，否则下一层的父子关系都会乱
  · size 必须在 for 循环之前锁定，循环中队列长度会增长
  · 预分配长度为 size 的 Vec，两个下标从两端向中间靠，天然 O(1) 写入；
    这等价于双端队列的 push_front，只是省掉了 VecDeque 的封装

以示例 1（root = [8,17,21,18,null,null,6]）为例：
  第 0 层 size=1，从左往右填 → [8]
  第 1 层 size=2，从右往左填：17 放最后 → [_,17]，21 放它前面 → [21,17]
  第 2 层 size=2，从左往右填：18、6 → [18,6]
  → [[8],[21,17],[18,6]]

复杂度分析：
  时间 O(n)：每个节点入队、出队、写入当前层各一次
  空间 O(w)：BFS 队列与本层数组各占 O(w)
*/
struct Solution;

impl Solution {
    // ---------- 解法一：BFS + 奇数层反转 ----------
    pub fn decorate_record_reverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let r = match root {
            Some(r) => r,
            None => return res,
        };

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(r);
        let mut left_to_right = true;
        while !q.is_empty() {
            let size = q.len();
            let mut level = Vec::with_capacity(size);
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                level.push(node.borrow().val);

                let left = node.borrow().left.clone();
                if let Some(l) = left {
                    q.push_back(l);
                }
                let right = node.borrow().right.clone();
                if let Some(r) = right {
                    q.push_back(r);
                }
            }
            // 奇数层整体翻转
            if !left_to_right {
                level.reverse();
            }
            res.push(level);
            left_to_right = !left_to_right;
        }
        res
    }

    // ---------- 解法二：BFS + 按方向从两端填充（最优解） ----------
    pub fn decorate_record(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let r = match root {
            Some(r) => r,
            None => return res,
        };

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(r);
        let mut left_to_right = true;
        while !q.is_empty() {
            // 本层节点个数必须在出队前锁定
            let size = q.len();
            let mut level = vec![0; size];
            let (mut lo, mut hi) = (0usize, size);
            for _ in 0..size {
                let node = q.pop_front().unwrap();

                // 只改变写入当前层的位置，孩子入队顺序始终先左后右
                if left_to_right {
                    level[lo] = node.borrow().val;
                    lo += 1;
                } else {
                    hi -= 1;
                    level[hi] = node.borrow().val;
                }

                let left = node.borrow().left.clone();
                if let Some(l) = left {
                    q.push_back(l);
                }
                let right = node.borrow().right.clone();
                if let Some(r) = right {
                    q.push_back(r);
                }
            }
            res.push(level);
            left_to_right = !left_to_right;
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

fn check(level_order: &[Option<i32>], want: &[Vec<i32>]) {
    assert_eq!(
        Solution::decorate_record_reverse(build_tree(level_order)),
        want,
        "反转版({:?})",
        level_order
    );
    assert_eq!(
        Solution::decorate_record(build_tree(level_order)),
        want,
        "双端填充({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(
        &[n(8), n(17), n(21), n(18), x, x, n(6)],
        &[vec![8], vec![21, 17], vec![18, 6]],
    );

    // 边界：空树
    check(&[], &[]);

    // 边界：单节点
    check(&[n(1)], &[vec![1]]);

    // 边界：只有一层两个节点
    check(&[n(1), n(2), n(3)], &[vec![1], vec![3, 2]]);

    // 边界：满二叉树三层
    check(
        &[n(1), n(2), n(3), n(4), n(5), n(6), n(7)],
        &[vec![1], vec![3, 2], vec![4, 5, 6, 7]],
    );

    // 边界：满二叉树四层（第 4 层从右到左）
    let level_order: Vec<Option<i32>> = (1..=15).map(Some).collect();
    check(
        &level_order,
        &[
            vec![1],
            vec![3, 2],
            vec![4, 5, 6, 7],
            vec![15, 14, 13, 12, 11, 10, 9, 8],
        ],
    );

    // 边界：左斜链 / 右斜链，每层只有一个节点，方向翻转不影响结果
    check(&[n(1), n(2), x, n(3)], &[vec![1], vec![2], vec![3]]);
    check(&[n(1), x, n(2), x, n(3)], &[vec![1], vec![2], vec![3]]);

    // 边界：缺左孩子，奇数层只有单个节点，验证不会重复写入
    check(
        &[n(1), n(2), n(3), x, n(4), n(5)],
        &[vec![1], vec![3, 2], vec![4, 5]],
    );

    // 边界：负数值
    check(
        &[n(-100), n(-50), n(100), n(0)],
        &[vec![-100], vec![100, -50], vec![0]],
    );

    println!("all tests passed");
}
