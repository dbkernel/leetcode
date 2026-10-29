/*
============================================================================
513. 找树左下角的值（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-bottom-left-tree-value/
分类：二叉树 · 层序/深度遍历（DFS / BFS）

给定一个二叉树的 根节点 root。

请找出该二叉树的 最底层 最左边 节点的值。

示例 1:
输入: root = [2,1,3]
输出: 1
解释: 最后一行是 [1, 3]，因此最左边的值是 1。

示例 2:
输入: [1,2,3,4,null,5,6,null,null,7]
输出: 7
解释: 最后一层只有节点 7。

提示:
- 二叉树的节点个数的范围是 [1, 10^4]
- -2^31 <= Node.val <= 2^31 - 1
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
解法一：DFS（先左后右，记录最大深度）【最优解】

为什么慢：直接按层来做当然可以，但「最底层」意味着必须把每一层都走完才知道
哪层最深；而且同一层还有「最左」的要求。如果每层都额外存下来再比较，
空间就白花了。

换个角度：只用一个变量记住「目前见过的最深深度」和「那个深度最左的值」。
按「先左后右」做 DFS，并且在回溯时判断当前深度是否严格大于已记录深度：
一旦某个节点刷新了最大深度，它一定就是该层最先被访问到的节点，也就是
最左边的节点。

关键点：
  · 必须先递归左、再递归右，否则同一层刷新最大深度的就不是最左节点
  · 判断用严格大于 `height > cur_height`：同一层的后续节点深度相同，不能覆盖
  · 判断写在两个递归之后（后序位置），此时 height 已经是当前节点的深度
  · 用 &mut 引用把 cur_val/cur_height 一路带出来，避免全局变量
  · 递归深度 = 树高，本题最坏 10^4，本地自测压到 200

以示例 2（root = [1,2,3,4,null,5,6,null,null,7]）为例：
  先一路向左到 4（深度 3），刷新 cur_height=3、cur_val=4
  回溯到 2，右空；回到 1，进右子树 3：先 5，再到 7（深度 4）刷新 cur_height=4
  再回溯 6（深度 3，不刷新）
  → 最底层最左是 7

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈；最坏（斜链）O(n)，平均 O(log n)

解法二：BFS（右孩子先入队，最后出队即答案）【非最优】

官方方法二。如果按「右 → 左」的顺序做层序遍历，那么每一层都是从右往左，
整体出队序列的最后一个节点，就是最底层、且是该层最靠左的节点。
  · 队列初始化放根；
  · 每次出队一个节点，先把右孩子入队、再入左孩子；
  · 用变量记录「最近一次出队的值」，循环结束时它就是答案。

关键点：
  · 入队顺序必须是「先右后左」：这样每层的最左节点会最后出队
  · 不需要记录层号，也不需要按层分组，最后一个出队的节点自然满足两个条件
  · 空孩子不要入队；题目保证根非空，所以 ans 一定会被赋值
  · 队列最坏要存一层节点，空间 O(w)，宽树时比 DFS 更费

以示例 2 为例：
  第 1 层出 1（入 3、2），第 2 层出 3（入 6、5）、2（入 4），
  第 3 层出 6、5（入 7）、4，第 4 层出 7 → 最后一个出队的是 7

复杂度分析：
  时间 O(n)
  空间 O(w)：w 为最大层宽
*/
struct Solution;

impl Solution {
    // ---------- 解法一：DFS（最优解） ----------
    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        height: i32,
        cur_val: &mut i32,
        cur_height: &mut i32,
    ) {
        let cur = match node {
            Some(n) => n,
            None => return,
        };
        let height = height + 1;

        let left = cur.borrow().left.clone();
        Self::dfs(left, height, cur_val, cur_height);
        let right = cur.borrow().right.clone();
        Self::dfs(right, height, cur_val, cur_height);

        // 同一层只会被最先到达的节点刷新，因此一定是这一层最左的
        if height > *cur_height {
            *cur_height = height;
            *cur_val = cur.borrow().val;
        }
    }

    pub fn find_bottom_left_value_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cur_val = 0;
        let mut cur_height = 0;
        Self::dfs(root, 0, &mut cur_val, &mut cur_height);
        cur_val
    }

    // ---------- 解法二：BFS，先右后左 ----------
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(r) = root {
            q.push_back(r);
        }

        while let Some(node) = q.pop_front() {
            // 先右后左：每层最左的节点最后出队
            let right = node.borrow().right.clone();
            if let Some(r) = right {
                q.push_back(r);
            }
            let left = node.borrow().left.clone();
            if let Some(l) = left {
                q.push_back(l);
            }
            ans = node.borrow().val;
        }
        ans
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

fn check(level_order: &[Option<i32>], want: i32) {
    assert_eq!(
        Solution::find_bottom_left_value_dfs(build_tree(level_order)),
        want,
        "DFS({:?})",
        level_order
    );
    assert_eq!(
        Solution::find_bottom_left_value(build_tree(level_order)),
        want,
        "BFS({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(&[n(2), n(1), n(3)], 1);

    // 示例 2
    check(
        &[n(1), n(2), n(3), n(4), x, n(5), n(6), x, x, n(7)],
        7,
    );

    // 边界：只有一个节点
    check(&[n(0)], 0);

    // 边界：只有左链，最底层最左是链尾
    check(&[n(1), n(2), x, n(3)], 3);

    // 边界：只有右链，最底层最左同样是链尾（每层只有一个节点）
    check(&[n(1), x, n(2), x, n(3)], 3);

    // 边界：最底层最左节点在左子树上，而不是最靠右的叶子
    check(&[n(1), n(2), n(3), n(4), n(5), n(6), n(7)], 4);

    // 边界：最底层只有一个节点，且是左孩子
    check(&[n(1), n(2), n(3), n(4)], 4);

    // 边界：最后一层的最左节点在右子树深处
    check(&[n(1), n(2), n(3), x, x, n(4), n(5), n(6)], 6);

    // 边界：负数值
    check(&[n(-1), n(-2), n(-3)], -2);

    // 边界：深链（深度 200），最底层最左是最后一个节点
    let mut level_order: Vec<Option<i32>> = Vec::new();
    for i in 1..=200 {
        level_order.push(Some(i));
        level_order.push(None);
    }
    check(&level_order, 200);

    println!("all tests passed");
}
