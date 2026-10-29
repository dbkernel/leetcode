/*
============================================================================
102. 二叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-level-order-traversal/
分类：二叉树 · 层序遍历（BFS / 队列）

给你二叉树的根节点 root ，返回其节点值的 层序遍历 。
（即逐层地，从左到右访问所有节点）。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：[[3],[9,20],[15,7]]

示例 2：
输入：root = [1]
输出：[[1]]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目在范围 [0, 2000] 内
- -1000 <= Node.val <= 1000
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
解法：BFS + 按层定长出队【最优解】

为什么慢：最朴素的 BFS 只知道「下一个是谁」，并不知道「这一层到哪儿结束」。
如果给每个节点额外挂一个层号，再用哈希表按层号分组，就要多存 O(n) 个层号；
最后还要按下标顺序遍历哈希表，空间和实现都变重。

换个角度：BFS 的队列有一个几乎白送的性质 —— 某一时刻队列里的元素，恰好就是
「当前这一层剩下的节点 + 下一层已经入队的节点」。因此只要在每轮开始时先记下
`s = q.len()`，就锁定了本层的边界：接下来连续出队 s 个，得到的就是完整的一层，
而这 s 个节点新入队的恰好全是下一层。

关键点：
  · `s` 必须在 for 循环之前取，循环中队列长度会增长，不能写成 `i < q.len()`
  · 每轮新建一个 level，循环结束后再 push 到答案，避免共用同一个 Vec
  · 入队仍要「先左后右」，并且跳过空孩子，才能保证同层从左到右
  · 根为空直接返回空 Vec：不要入队再靠循环兜底
  · 队列里存 Rc 克隆（引用计数 +1），不复制节点本身

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  第 1 轮 s=1 → 出 3，入 9、20 → [[3]]
  第 2 轮 s=2 → 出 9，入空；出 20，入 15、7 → [[3],[9,20]]
  第 3 轮 s=2 → 出 15、7 → [[3],[9,20],[15,7]]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案数组外，队列最多同时存两层节点
*/
struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let r = match root {
            Some(r) => r,
            None => return res,
        };

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(r);
        while !q.is_empty() {
            // 本层节点个数必须在出队前锁定，循环中队列还会增长
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
            res.push(level);
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
        Solution::level_order(build_tree(level_order)),
        want,
        "levelOrder({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(
        &[n(3), n(9), n(20), x, x, n(15), n(7)],
        &[vec![3], vec![9, 20], vec![15, 7]],
    );

    // 示例 2：单节点
    check(&[n(1)], &[vec![1]]);

    // 示例 3：空树
    check(&[], &[]);

    // 边界：完全二叉树，层宽递增
    check(
        &[n(1), n(2), n(3), n(4), n(5), n(6), n(7)],
        &[vec![1], vec![2, 3], vec![4, 5, 6, 7]],
    );

    // 边界：左斜链 / 右斜链，每层只有一个节点
    check(&[n(1), n(2), x, n(3)], &[vec![1], vec![2], vec![3]]);
    check(&[n(1), x, n(2), x, n(3)], &[vec![1], vec![2], vec![3]]);

    // 边界：缺左孩子，层内顺序不能乱
    check(
        &[n(1), n(2), n(3), x, n(4), n(5)],
        &[vec![1], vec![2, 3], vec![4, 5]],
    );

    // 边界：负数值
    check(
        &[n(-1000), n(-500), n(1000), n(0)],
        &[vec![-1000], vec![-500, 1000], vec![0]],
    );

    // 边界：满二叉树四层
    let level_order: Vec<Option<i32>> = (1..=15).map(Some).collect();
    let mut want: Vec<Vec<i32>> = Vec::new();
    let mut start = 1;
    let mut width = 1;
    while width <= 8 {
        want.push((start..start + width).collect());
        start += width;
        width <<= 1;
    }
    check(&level_order, &want);

    println!("all tests passed");
}
