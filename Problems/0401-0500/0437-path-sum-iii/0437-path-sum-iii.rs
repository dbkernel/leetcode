/*
============================================================================
437. 路径总和 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/path-sum-iii/
分类：二叉树 · 前缀和哈希（路径计数）

给定一个二叉树的根节点 root ，和一个整数 targetSum ，求该二叉树里节点值之和
等于 targetSum 的 路径 的数目。

路径 不需要从根节点开始，也不需要在叶子节点结束，但是路径方向必须是向下的
（只能从父节点到子节点）。

示例 1：
输入：root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
输出：3
解释：和等于 8 的路径有 3 条，如图所示。

示例 2：
输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
输出：3

提示：
- 二叉树的节点个数的范围是 [0,1000]
- -10^9 <= Node.val <= 10^9
- -1000 <= targetSum <= 1000
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
解法一：以每个节点为起点向下枚举【非最优】

为什么慢：路径起点可以是任意节点。直接的做法是让每个节点都当一次起点，
沿子树向下累加并统计命中。思路没错，但同一条边会被它上方的每个祖先各扫一次：
链状树上总代价是 1 + 2 + ... + n = O(n^2)，而节点只有 n 个。

换个角度：把"从任意节点出发"改成"以每个节点为终点"，一段向下路径就可以
写成两个"根到某点前缀和"之差。若当前前缀和为 cur，祖先链上某个前缀和为
cur - targetSum，两者之间那段路径的和恰好是 targetSum。这样每个节点只需
查一次表，不必再向下枚举。

关键点（本解法是枚举，关键点在解法二）：
  · 递归参数带入"从起点累加到当前节点"的 cur，起点一旦固定就只往下走
  · 统计完以 root 为起点的路径后，还要加上完全落在左右子树里的路径，
    三类路径互不重叠
  · 值可达 1e9、节点数 1000，累加和最大 1e12，必须用 i64
  · 值可以为负，cur == targetSum 命中后仍要继续往下走
  · 空树返回 0，None 是递归的唯一边界

手推一遍：以示例 1（root = [10,5,-3,3,2,null,11,3,-2,null,1]，
targetSum = 8）为例：
  起点 10：10、15、18、17、... 没有命中
  起点 5：5、8（5+3）命中、7、8（5+2+1）命中 → 2 条
  起点 -3：-3、8（-3+11）命中 → 1 条
  其余叶子单独都不等于 8 → 合计 3 条 ✔

复杂度分析：
  时间 O(n^2)：链状树最坏，每个起点都要扫完自己的整棵子树；
    平衡树上为 O(n log n)
  空间 O(h)：递归栈，另有一层按起点的递归嵌套，栈深仍是 O(h)
*/

// 单文件 rustc 编译时平台不提供 Solution1，需要显式声明
pub struct Solution1;

impl Solution1 {
    pub fn path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
    ) -> i32 {
        // 起点固定为 node，只向下延伸，统计和等于 target 的路径条数
        fn count_from(
            node: Option<&Rc<RefCell<TreeNode>>>,
            cur: i64,
            target: i64,
        ) -> i64 {
            let Some(n) = node else {
                return 0;
            };
            let (val, left, right) = {
                let b = n.borrow();
                (b.val as i64, b.left.clone(), b.right.clone())
            };

            let cur = cur + val;
            let hit = if cur == target { 1 } else { 0 };

            // 值可正可负，命中之后仍可能接着往下凑出新的 target
            hit + count_from(left.as_ref(), cur, target)
                + count_from(right.as_ref(), cur, target)
        }

        fn each_start(
            node: Option<&Rc<RefCell<TreeNode>>>,
            target: i64,
        ) -> i64 {
            let Some(n) = node else {
                return 0;
            };
            let (left, right) = {
                let b = n.borrow();
                (b.left.clone(), b.right.clone())
            };

            // 三类路径互不重叠：以 n 为起点、完全在左子树、完全在右子树
            count_from(Some(n), 0, target)
                + each_start(left.as_ref(), target)
                + each_start(right.as_ref(), target)
        }

        each_start(root.as_ref(), target_sum as i64) as i32
    }
}

/*
解法二：前缀和 + 哈希表回溯【最优解】

为什么慢：解法一对每个起点都重新向下扫，祖先链上同一段和被反复累加。
子树遍历返回后，这些信息被整段丢弃，下一个起点又要从头算。

换个角度：一条向下路径的端点记为 u（上）和 v（下），它的和等于
pref[v] - pref[parent(u)]，其中 pref 是根到该节点的前缀和。DFS 到 v 时，
只要知道祖先链上有多少个节点的前缀和等于 pref[v] - targetSum，就得到
以 v 为终点、起点在 u 处的路径条数。祖先链的前缀和用哈希表边进边记。

关键点：
  · 必须先查再插：若先把当前前缀插入再查询，targetSum = 0 时会把
    "自己减自己"的空路径也算进去（如 [0] 会返回 2 而不是 1）
  · 查的键是 cur - targetSum，方向不能反；前缀和之差才有"区间和"的含义
  · 初始化 cnt[0] = 1，代表空前缀：从根开始的路径靠它才能被数到
  · 退出当前节点前必须把 cnt[cur] 减一（减到 0 就删除），
    否则兄弟分支会看到不属于自己祖先链的前缀和，凭空多算路径
  · 哈希表存的是出现次数而不是布尔值：前缀和可以重复（0 或正负抵消）
  · 值可以为负，cur 不单调，不能排序或用双指针；累加和必须用 i64
  · 先 clone 左右孩子再递归，RefCell 的借用只存在于计算值的那一小段

手推一遍：仍以示例 1（targetSum = 8）为例，cnt 初始 {0:1}：
  进入 10：cur = 10，查 2 无；插入 10 → {0:1, 10:1}
  进入 5：cur = 15，查 7 无；插入 15
  进入 3：cur = 18，查 10 有 1 条 → 路径 5→3 ✔
  进入 3 的左孩子 3：cur = 21，查 13 无
  进入 -2：cur = 16，查 8 无；回溯到 3
  进入 2：cur = 17，查 9 无
  进入 1：cur = 18，查 10 有 1 条 → 路径 5→2→1 ✔
  进入 -3：cur = 7，查 -1 无；插入 7
  进入 11：cur = 18，查 10 有 1 条 → 路径 -3→11 ✔
  其余分支无命中，合计 3 条 ✔

复杂度分析：
  时间 O(n)：每个节点进出一次，哈希表查询与更新均摊 O(1)
  空间 O(n)：哈希表最多同时保存一条根到叶子的前缀和，最坏（斜链）O(n)
*/

// 单文件 rustc 编译时平台不提供 Solution2，需要显式声明
pub struct Solution2;

impl Solution2 {
    pub fn path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
    ) -> i32 {
        fn dfs(
            node: Option<&Rc<RefCell<TreeNode>>>,
            cur: i64,
            target: i64,
            cnt: &mut HashMap<i64, i64>,
        ) -> i64 {
            let Some(n) = node else {
                return 0;
            };
            let (val, left, right) = {
                let b = n.borrow();
                (b.val as i64, b.left.clone(), b.right.clone())
            };

            let cur = cur + val;

            // 先查：此刻 cnt 只含祖先链上的前缀和，不含 node 自己
            let mut res = *cnt.get(&(cur - target)).unwrap_or(&0);

            // 再插：当前前缀供后代查询使用
            *cnt.entry(cur).or_insert(0) += 1;

            res += dfs(left.as_ref(), cur, target, cnt);
            res += dfs(right.as_ref(), cur, target, cnt);

            // 回滚：离开 node 后，它不再是任何待访问节点的祖先
            let remain = {
                let e = cnt.entry(cur).or_insert(0);
                *e -= 1;
                *e
            };
            if remain == 0 {
                cnt.remove(&cur);
            }
            res
        }

        let mut cnt = HashMap::new();
        // 空前缀：让"从根开始"的路径也能被 cur - targetSum = 0 数到
        cnt.insert(0i64, 1i64);
        dfs(root.as_ref(), 0, target_sum as i64, &mut cnt) as i32
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

fn check(level_order: &[Option<i32>], target_sum: i32, want: i32) {
    assert_eq!(
        Solution1::path_sum(build_tree(level_order), target_sum),
        want,
        "brute: {:?}",
        level_order
    );
    assert_eq!(
        Solution2::path_sum(build_tree(level_order), target_sum),
        want,
        "prefix: {:?}",
        level_order
    );
}

fn main() {
    // 示例 1
    check(
        &[
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ],
        8,
        3,
    );

    // 示例 2
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
        3,
    );

    // 边界：空树
    check(&[], 0, 0);

    // 边界：单节点不等于目标和
    check(&[Some(1)], 0, 0);

    // 关键点：单个 0 只有一条路径；先插后查会错误地数出空路径
    check(&[Some(0)], 0, 1);

    // 关键点：前缀和重复出现，哈希表必须记次数
    check(&[Some(0), Some(0), Some(0)], 0, 5);

    // 反直觉：正负抵消，1 + (-1) = 0
    check(&[Some(1), Some(-1)], 0, 1);

    // 反直觉：目标和为负
    check(&[Some(-1), Some(-2), Some(-3)], -3, 2);

    // 关键点：左分支的前缀和不能在右分支复用（漏回滚会多算）
    check(&[Some(1), Some(5), Some(2)], -3, 0);

    // 边界：兄弟节点各自单独成路
    check(&[Some(1), Some(2), Some(3)], 3, 2);

    // 边界：目标和为 0，只有根自己那条路径命中
    check(&[Some(0), Some(1), Some(-1)], 0, 1);

    // 边界：200 层斜链全为 1，目标和 50，共 200-50+1 条
    let mut chain = vec![Some(1)];
    for _ in 2..=200 {
        chain.push(None);
        chain.push(Some(1));
    }
    check(&chain, 50, 151);

    // 边界：值接近 1e9，累加超过 32 位，验证 i64 不溢出
    check(
        &[
            Some(1000000000),
            Some(1000000000),
            None,
            Some(1000000000),
        ],
        1000000000,
        3,
    );

    println!("all tests passed");
}
