/*
============================================================================
337. 打家劫舍 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/house-robber-iii/
分类：动态规划 · 树形 DP

小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为 root 。

除了 root 之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果 两个直接相连的房子在同一天晚上被打劫 ，房屋将自动报警。

给定二叉树的 root 。返回 在不触动警报的情况下 ，小偷能够盗取的最高金额 。

示例 1:
输入: root = [3,2,3,null,3,null,1]
输出: 7 
解释: 小偷一晚能够盗取的最高金额 3 + 3 + 1 = 7

示例 2:
输入: root = [3,4,5,1,3,null,1]
输出: 9
解释: 小偷一晚能够盗取的最高金额 4 + 5 = 9

提示：
- 树的节点数在 [1, 10^4] 范围内
- 0 <= Node.val <= 10^4
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
解法一：树上 DP，后序一次返回 (选, 不选)【最优解】

为什么慢：把数组上的递推照搬到树上，最直白的式子是对每个节点比较
"选它 + 孙子们的最优" 和 "两个孩子的最优"。但"两个孩子的最优"本身又要
递归求孙子，同一个子树会在两条分支里各展开一次，节点被重复进入的次数
随深度指数增长。

换个角度：每个节点只需要向父节点汇报两个数——以它为根的子树里，
"选它"的最大值与"不选它"的最大值。父节点拿到孩子的这两个数就能自己
做决定，完全不需要知道子树内部怎么选的。于是后序一次遍历即可。

关键点：
  · 选 x 时孩子只能"不选"：take(x) = x.val + skip(左) + skip(右)
  · 不选 x 时孩子选不选都行：skip(x) = max(take(左), skip(左))
    + max(take(右), skip(右))
  · 返回二元组的两个分量顺序必须在整棵树里统一，父子两侧写反是这类题
    最常见的错
  · 空节点返回 (0, 0)：缺失的孩子既不能被选，也不会贡献金额
  · 答案是根节点两个值的 max，不能只看"选根"那一个
  · Rust 里先把左右孩子 clone 出来再递归，避免借着父节点的 RefCell
    借用时继续向下展开

以示例 1（root = [3,2,3,null,3,null,1]）为例，从叶子往上：
  val=3 的叶子： (3, 0)
  左孩子 2（右孩子是上面那个 3）：take = 2 + 0 + 0 = 2，
    skip = max(3,0) + 0 = 3 → (2, 3)
  右孩子 3（右孩子是 val=1 的叶子）：take = 3 + 0 + 0 = 3，
    skip = 0 + max(1,0) = 1 → (3, 1)
  根 3：take = 3 + skip(左)=3 + skip(右)=1 = 7；
    skip = max(2,3) + max(3,1) = 6 → max(7,6) = 7 ✔

复杂度分析：
  时间 O(n)：每个节点只被后序访问一次，汇报两个整数
  空间 O(h)：递归栈等于树高，斜树最坏 O(n)，平衡树 O(log n)
*/
pub struct Solution1;

impl Solution1 {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (take, skip) = Self::dfs(root.as_ref());
        take.max(skip)
    }

    // 返回 (选当前节点的最大值, 不选当前节点的最大值)
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        let Some(rc) = node else {
            return (0, 0);
        };
        let (val, left, right) = {
            let n = rc.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        let (left_take, left_skip) = Self::dfs(left.as_ref());
        let (right_take, right_skip) = Self::dfs(right.as_ref());

        // 选当前节点：两个孩子都只能跳过
        let take = val + left_skip + right_skip;
        // 不选当前节点：每个孩子都可以在"选"与"不选"之间取较优
        let skip = left_take.max(left_skip) + right_take.max(right_skip);
        (take, skip)
    }
}

/*
解法二：记忆化递归，按节点缓存它的最优值【非最优】

为什么慢：解法一的式子在"不选 x"时要取孩子的 max，"选 x"时要取孙子的值，
如果直接递归而不缓存，同一个节点的最优值会被它的父、祖父、曾祖父反复
求解，深度为 h 的链上退化成指数级。

换个角度：把"以 x 为根的子树的最优值"当成唯一状态，算过就存。于是与解法
一一样每节点只算一次，但缓存用的是节点地址而不是返回值结构。

关键点：
  · 缓存键取 Rc::as_ptr 的地址：节点值会重复，地址不会
  · 进入函数要立刻查表，查表放在递归调用之后等于没缓存
  · 取孙子时要走一层 Option：child 本身可能是 None

复杂度分析：
  时间 O(n)：每个节点的最优值只计算一次
  空间 O(n)：哈希表给每个节点存一个整数，比解法一多一个数量级；
        递归栈 O(h) 相对次要
*/
pub struct Solution2;

impl Solution2 {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut memo: HashMap<usize, i32> = HashMap::new();
        Self::dfs(root.as_ref(), &mut memo)
    }

    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        memo: &mut HashMap<usize, i32>,
    ) -> i32 {
        let Some(rc) = node else {
            return 0;
        };
        let key = Rc::as_ptr(rc) as usize;
        if let Some(&v) = memo.get(&key) {
            return v;
        }

        let (val, left, right) = {
            let n = rc.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        // 不选当前节点
        let skip =
            Self::dfs(left.as_ref(), memo) + Self::dfs(right.as_ref(), memo);
        // 选当前节点：只能再从孙子们身上取
        let mut take = val;
        if let Some(l) = &left {
            let l = l.borrow();
            take += Self::dfs(l.left.as_ref(), memo)
                + Self::dfs(l.right.as_ref(), memo);
        }
        if let Some(r) = &right {
            let r = r.borrow();
            take += Self::dfs(r.left.as_ref(), memo)
                + Self::dfs(r.right.as_ref(), memo);
        }

        let best = take.max(skip);
        memo.insert(key, best);
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
    assert_eq!(Solution1::rob(build_tree(level_order)), want);
    assert_eq!(Solution2::rob(build_tree(level_order)), want);
}

fn main() {
    // 示例 1：3 + 3 + 1，根与右孩子、右孩子的右孩子互不相邻
    check(
        &[Some(3), Some(2), Some(3), None, Some(3), None, Some(1)],
        7,
    );

    // 示例 2：4 + 5 优于 3 + 1 + 3 + 1
    check(
        &[Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)],
        9,
    );

    // 边界：单节点，必选
    check(&[Some(1)], 1);

    // 边界：值为 0 的节点，答案是 0 而不是 -1
    check(&[Some(0)], 0);

    // 关键点用例：隔代能同选，根 5 与两个孙子 4、6 一起拿共 15
    check(&[Some(5), Some(1), None, Some(4), Some(6)], 15);

    // 反直觉：3 与 4 分属不同分支，中间隔着一层，3+4=7 比 2+4=6 更好
    check(&[Some(2), Some(1), Some(3), None, Some(4)], 7);

    // 关键点用例：全零树，任何选法都是 0
    check(&[Some(0), Some(0), Some(0), Some(0), Some(0)], 0);

    // 边界：右斜链 200 个节点，等价于 198 处理 [1..200]，偶数位之和更大
    let mut level_order = Vec::new();
    for i in 1..=200 {
        level_order.push(Some(i));
        level_order.push(None);
    }
    check(&level_order, 10100);

    println!("all tests passed");
}
