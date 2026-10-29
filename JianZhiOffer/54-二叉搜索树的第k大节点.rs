/*
============================================================================
LCR 174 / 剑指 Offer 54. 寻找二叉搜索树中的目标节点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/
分类：二叉搜索树 · 反序中序遍历

某公司组织架构以二叉搜索树形式记录，节点值为处于该职位的员工编号。
请返回第 cnt 大的员工编号。

示例 1：
输入：root = [7, 3, 9, 1, 5], cnt = 2
       7
      / \
     3   9
    / \
   1   5
输出：7

示例 2：
输入: root = [10, 5, 15, 2, 7, null, 20, 1, null, 6, 8], cnt = 4
       10
      /  \
     5    15
    / \     \
   2   7     20
  /   / \
 1   6   8
输出：8

提示：
- 1 ≤ cnt ≤ 二叉搜索树元素个数
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
解法一：反序中序遍历（右 → 根 → 左）【最优解】

为什么慢：最容易想到的是"中序遍历把整棵树存成升序数组，再取倒数第 cnt 个"，
时间是 O(n) 但要多开 O(n) 的数组；如果先求"总节点数 n，再找第 n-cnt+1 小"，
还得额外遍历一次。

换个角度：BST 的中序是升序，"右 → 根 → 左"的遍历就是降序。按降序数到
第 cnt 个就是答案，数到就可以停，不必访问剩余节点。
  · 递归时先走右子树，再处理当前节点，最后走左子树；
  · 每处理一个节点就 `cnt -= 1`，减到 0 时记下答案并停止。

关键点：
  · 顺序是"右、根、左"，写成"左、根、右"就变成第 cnt 小了
  · 计数必须在访问当前节点时做，不能在进入函数时做
  · 找到答案后要能及时收手：`cnt == 0` 时直接 return，
    否则会继续遍历把 cnt 减成负数
  · 用 `&mut i32` 传 ans / remain，避免用 UnsafeCell 或额外结构体
  · 题目保证 1 ≤ cnt ≤ 节点数，因此一定能找到答案

以示例 1（root = [7,3,9,1,5]，cnt = 2）为例：
  右子树 9 → 访问 9，cnt 变 1；
  回到根 7 → 访问 7，cnt 变 0 → 答案是 7 ✔

复杂度分析：
  时间 O(n)：最坏要走到最小节点；平均只需访问前 cnt 大的一小部分
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：迭代中序（右根左）【非最优】

把解法一改成显式栈：先一路向右压栈，弹出时计数，再转向左子树。
  · 这样能避免递归，思路与解法一完全一致；
  · 代码稍长，好处是不受递归深度限制（虽然本题数据量也用不上）。

关键点：
  · 压栈方向是"向右"，弹栈时才是"访问"，别把方向写反
  · 计数到 0 即刻 return，栈里剩下的节点不用再管
  · 与解法一相同，顺序必须是右 → 根 → 左

复杂度分析：
  时间 O(n)
  空间 O(h)：栈保存当前路径
*/

// ---------- 解法一：反序中序遍历（最优解） ----------
pub struct Solution1;

impl Solution1 {
    pub fn find_target_node(root: Option<Rc<RefCell<TreeNode>>>, cnt: i32) -> i32 {
        fn dfs(
            node: Option<&Rc<RefCell<TreeNode>>>,
            remain: &mut i32,
            ans: &mut i32,
        ) {
            let Some(n) = node else {
                return;
            };
            if *remain == 0 {
                return;
            }
            let (right, val, left) = {
                let n = n.borrow();
                (n.right.clone(), n.val, n.left.clone())
            };

            dfs(right.as_ref(), remain, ans);

            *remain -= 1;
            if *remain == 0 {
                *ans = val;
                return;
            }

            dfs(left.as_ref(), remain, ans);
        }

        let mut ans = 0;
        let mut remain = cnt;
        dfs(root.as_ref(), &mut remain, &mut ans);
        ans
    }
}

// ---------- 解法二：迭代中序（右根左） ----------
pub struct Solution2;

impl Solution2 {
    pub fn find_target_node(root: Option<Rc<RefCell<TreeNode>>>, cnt: i32) -> i32 {
        let mut stk: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut node = root;
        let mut cnt = cnt;

        loop {
            while let Some(cur) = node {
                let next = cur.borrow().right.clone();
                stk.push(cur);
                node = next;
            }
            let Some(cur) = stk.pop() else {
                break;
            };
            cnt -= 1;
            if cnt == 0 {
                return cur.borrow().val;
            }
            node = cur.borrow().left.clone();
        }
        -1 // 题目保证不会走到这里
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

fn check(level_order: &[Option<i32>], cnt: i32, want: i32) {
    assert_eq!(
        Solution1::find_target_node(build_tree(level_order), cnt),
        want
    );
    assert_eq!(
        Solution2::find_target_node(build_tree(level_order), cnt),
        want
    );
}

fn main() {
    // 示例 1
    check(&[Some(7), Some(3), Some(9), Some(1), Some(5)], 2, 7);

    // 示例 2
    check(
        &[
            Some(10),
            Some(5),
            Some(15),
            Some(2),
            Some(7),
            None,
            Some(20),
            Some(1),
            None,
            Some(6),
            Some(8),
        ],
        4,
        8,
    );

    // 边界：单节点
    check(&[Some(1)], 1, 1);

    // 剑指 Offer 原示例 1：root = [3,1,4,null,2], k = 1
    check(&[Some(3), Some(1), Some(4), None, Some(2)], 1, 4);

    // 剑指 Offer 原示例 2：root = [5,3,6,2,4,null,null,1], k = 3
    check(
        &[
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ],
        3,
        4,
    );

    // 边界：取最小值（cnt = 节点总数）
    check(
        &[
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ],
        6,
        1,
    );

    // 边界：cnt = 1 就是最大值
    check(
        &[
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ],
        1,
        6,
    );

    // 边界：只有右孩子的 BST，降序就是 3、2、1
    check(&[Some(1), None, Some(2), None, Some(3)], 2, 2);

    // 边界：只有左孩子的 BST，降序就是 3、2、1
    check(&[Some(3), Some(2), None, Some(1)], 3, 1);

    // 边界：100 层右斜链，值 1..100
    let mut chain = vec![Some(1)];
    for i in 2..=100 {
        chain.push(None);
        chain.push(Some(i));
    }
    check(&chain, 1, 100);
    check(&chain, 100, 1);
    check(&chain, 50, 51);

    println!("all tests passed");
}
