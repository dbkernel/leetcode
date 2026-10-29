/*
============================================================================
LCR 153 / 剑指 Offer 34. 二叉树中和为某一值的路径（中等）
来源：剑指 Offer（第 2 版）
链接：https://leetcode.cn/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/
分类：二叉树 · DFS 回溯（根到叶子路径）

给你二叉树的根节点 root 和一个整数目标和 targetSum ，找出所有 从根节点到
叶子节点 路径总和等于给定目标和的路径。

叶子节点 是指没有子节点的节点。

示例 1：
输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
输出：[[5,4,11,2],[5,8,4,5]]

示例 2：
输入：root = [1,2,3], targetSum = 5
输出：[]

示例 3：
输入：root = [1,2], targetSum = 0
输出：[]

提示：
- 树中节点总数在范围 [0, 5000] 内
- -1000 <= Node.val <= 1000
- -1000 <= targetSum <= 1000
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
解法：DFS 回溯（进入压栈，离开弹栈）【最优解】

为什么慢：要枚举所有根到叶子的路径，路径条数最坏是 O(n)（叶子数），每条
路径长 O(h)。若每到一个叶子都从头重建一次路径，或者每层都把 path 拷贝一份
往下传，重复付出的代价会到 O(n*h)；而答案本身最多也只有 O(n*h) 个元素。

换个角度：所有根到叶子的路径共享前缀。维护一条 path，进入节点时压入、
离开时弹出，它自然就等于"当前根到节点的链"。命中时只拷贝一次，
没命中就原地复用，前缀部分不再重复构造。

关键点：
  · 弹出 path 末尾元素必须做，且放在左右递归之后 —— 漏掉它，兄弟分支
    会带着上一分支的尾巴，路径越积越长
  · 只在该节点是叶子（左右都空）且剩余和恰为 0 时收集；在非叶子处收集，
    会把"根到中间节点"误当成一条完整路径
  · 剩余和用 targetSum 沿途递减，避免每次重新求和
  · 值可以为负，remain < 0 不能剪枝，remain == 0 也不能提前返回 ——
    后面接着负数仍可能把和拉回 0
  · 收集时 clone path，不能把借用存进答案
  · 先把 val/left/right 从 RefCell 借出并克隆再递归，避免同一个节点被
    borrow 两次

手推一遍：以示例 1（root = [5,4,8,11,null,13,4,7,2,null,null,5,1]，
targetSum = 22）为例：
  5 → 4 → 11 → 7，remain = -5，是叶子但归不了零，弹出 7
  回到 11 → 2，remain = 0，是叶子 → 收集 [5,4,11,2]，再弹出 2、11、4
  5 → 8 → 13，remain = -4，是叶子，弹出
  回到 8 → 4 → 5，remain = 0，是叶子 → 收集 [5,8,4,5]
  回到根，path 清空 → 共两条 ✔

复杂度分析：
  时间 O(n^2)：节点进出各一次是 O(n)，但每次命中要拷贝长度 O(h) 的路径，
    最坏（斜链且处处命中）为 O(n^2)，输出本身也有这么多元素
  空间 O(h)：path 与递归栈都只保存当前一条根到叶子的链，最坏 O(n)
*/

// 单文件 rustc 编译时平台不提供 Solution，需要显式声明
pub struct Solution;

impl Solution {
    pub fn path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
    ) -> Vec<Vec<i32>> {
        fn dfs(
            node: Option<&Rc<RefCell<TreeNode>>>,
            remain: i32,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            let Some(n) = node else {
                return;
            };
            // 借用只在本块内存在，递归前必须释放，否则子树再 borrow 会 panic
            let (val, left, right) = {
                let b = n.borrow();
                (b.val, b.left.clone(), b.right.clone())
            };

            path.push(val);
            let remain = remain - val;

            // 叶子且差值归零，才是一条符合题意的路径
            if left.is_none() && right.is_none() && remain == 0 {
                ans.push(path.clone());
            }

            // 负值存在，两条分支都必须走，不能按 remain 剪枝
            dfs(left.as_ref(), remain, path, ans);
            dfs(right.as_ref(), remain, path, ans);

            // 撤销压栈：兄弟分支不能继承这条路径的尾巴
            path.pop();
        }

        let mut path = Vec::new();
        let mut ans = Vec::new();
        dfs(root.as_ref(), target_sum, &mut path, &mut ans);
        ans
    }
}

// ===== 测试辅助：按层序数组（None 表示空）建树、路径归一化 =====
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

// 题面没有约定路径顺序，统一排序后再比较
fn normalize(mut paths: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for p in paths.iter_mut() {
        p.sort_unstable();
    }
    paths.sort();
    paths
}

fn check(level_order: &[Option<i32>], target_sum: i32, want: Vec<Vec<i32>>) {
    let got = Solution::path_sum(build_tree(level_order), target_sum);
    assert_eq!(normalize(got), normalize(want));
}

fn main() {
    // 示例 1
    check(
        &[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ],
        22,
        vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]],
    );

    // 示例 2
    check(&[Some(1), Some(2), Some(3)], 5, vec![]);

    // 示例 3：目标和为 0，但两条路径分别是 1 和 3，都不等于 0
    check(&[Some(1), Some(2)], 0, vec![]);

    // 边界：空树
    check(&[], 0, vec![]);

    // 反直觉：单个 0 既是叶子又等于目标和，应收集一条路径
    check(&[Some(0)], 0, vec![vec![0]]);

    // 反直觉：正负抵消，1 + (-1) = 0，路径跨两层
    check(&[Some(1), Some(-1)], 0, vec![vec![1, -1]]);

    // 反直觉：全负值，目标和也为负
    check(&[Some(-2), None, Some(-3)], -5, vec![vec![-2, -3]]);

    // 关键点：根节点和已等于 1，但它不是叶子，不能收集
    check(&[Some(1), Some(2), Some(2)], 1, vec![]);

    // 关键点：remain 中途归零后还要继续走（后续节点是负数）
    check(
        &[Some(1), Some(-1), Some(-1)],
        0,
        vec![vec![1, -1], vec![1, -1]],
    );

    // 关键点：左分支结束后必须弹干净，右分支路径不能带左分支的值
    check(&[Some(1), Some(2), Some(3), Some(4)], 7, vec![vec![1, 2, 4]]);

    // 边界：200 层斜链全为 1，目标和 200 只有一整条路径
    let mut chain = Vec::new();
    for i in 1..=200 {
        chain.push(Some(1));
        if i < 200 {
            chain.push(None);
        }
    }
    check(&chain, 200, vec![vec![1; 200]]);

    println!("all tests passed");
}
