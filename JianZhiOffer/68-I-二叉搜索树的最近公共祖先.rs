/*
============================================================================
LCR 193 / 剑指 Offer 68 - I. 二叉搜索树的最近公共祖先（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-zui-jin-gong-gong-zu-xian-lcof/
分类：二叉搜索树 · 一次遍历 / 路径比较

给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个结点 p、q，最近公共
祖先表示为一个结点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点
也可以是它自己的祖先）。”

例如，给定如下二叉搜索树:  root = [6,2,8,0,4,7,9,null,null,3,5]

示例 1：
输入：root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
输出：6
解释：节点 2 和节点 8 的最近公共祖先是 6。

示例 2：
输入：root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
输出：2
解释：节点 2 和节点 4 的最近公共祖先是 2, 因为根据定义最近公共祖先节点可以
为节点本身。

说明：
- 所有节点的值都是唯一的。
- p、q 为不同节点且均存在于给定的二叉搜索树中。

注意：本题与主站 235 题相同：
https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/
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
解法一：两次遍历求出根到 p、q 的路径，取最后一个公共前缀【非最优】

为什么慢：时间上两趟下降各 O(h)，并不算慢；代价在空间和多余信息：要各存
一条长度最多 h 的路径（斜树上 O(n)），可题目只关心"两条路径从哪里分开"，
中间那些公共前缀节点存下来之后只用得到最后一个。

换个角度：从根走到某个节点的路径，在 BST 上唯一。p、q 的最近公共祖先就是
两条路径的最后一个相同节点 —— 分叉发生的位置。

关键点：
  · 每走一步先把当前节点压入路径，再与 target 比较 —— 先比较再压入会漏掉
    target 自己，p 是 q 的祖先时会退回上一层
  · 路径必须从根开始，否则公共前缀不完整
  · 两条路径从头比，遇到第一个不同的节点就停，答案取最后一个相同的节点
  · 节点身份必须用 Rc::ptr_eq 判断，比较 val 说明不了是同一个节点

以示例 2（root = [6,2,8,0,4,7,9,null,null,3,5]，p = 2，q = 4）为例：
  到 2 的路径：[6, 2]（2 就是目标，压入后立刻停）
  到 4 的路径：[6, 2, 4]
  公共前缀 6、2，最后一个相同节点是 2 ✔
  （若压入时不包含 target，路径变成 [6] 和 [6,2]，答案会错成 6。）

复杂度分析：
  时间 O(h)：两条路径各下降一次，h 为树高（最坏 n）
  空间 O(h)：两个 Vec 保存路径，最坏（斜树）O(n)

解法二：一次遍历，利用 BST 的比较性质直接找分叉点【最优解】

为什么慢：解法一存了两条完整路径，而 BST 上根本不需要知道路径长什么样：
当前节点的值就能告诉我们 p、q 在哪一侧。

换个角度：从根往下走，p、q 的走向只有三种可能：
  · 两个都小于当前值 → 二者都在左子树，LCA 在左边，向左走；
  · 两个都大于当前值 → 二者都在右子树，LCA 在右边，向右走；
  · 一大一小，或其中之一等于当前值 → 二者从这里分到两侧（或当前节点
    就是其中一个），当前节点就是 LCA，不能再往下走。

关键点：
  · 两个判断都要写成"严格小于 / 严格大于当前值"：只要有一个等于当前节点，
    当前节点就是答案，把等号放进下沉条件会多走一层，返回更深的节点
  · 必须拿 p、q 分别与当前节点比较，不能先比较 p 与 q 的大小再定方向 ——
    "p < q" 说明不了它们相对当前节点在左边还是右边
  · p、q 的先后顺序不可假设，p 完全可以比 q 大，两个条件都要同时成立
  · 下沉到 None 说明输入不合法；题目保证 p、q 存在，所以正常循环内必返回

以示例 2（root = [6,2,8,0,4,7,9,null,null,3,5]，p = 2，q = 4）为例：
  root = 6：2 < 6 且 4 < 6，向左走到 2；
  root = 2：p 等于当前值，不满足"两个都严格小于/大于"，返回 2 ✔

复杂度分析：
  时间 O(h)：每步只下降一层，h 为树高；平衡时 O(log n)，斜树 O(n)
  空间 O(1)：只用了一个游标指针，没有栈也没有 Vec
*/

// ---------- 解法一：两次遍历 + 公共前缀 ----------
pub struct Solution1;

fn collect_path(
    root: &Option<Rc<RefCell<TreeNode>>>,
    target: &Rc<RefCell<TreeNode>>,
) -> Vec<Rc<RefCell<TreeNode>>> {
    let target_val = target.borrow().val;
    let mut path = Vec::new();
    let mut cur = root.clone();
    while let Some(node) = cur {
        path.push(Rc::clone(&node));
        // 路径包含 target 本身，否则"祖先是自己"的情况会被漏掉
        if Rc::ptr_eq(&node, target) {
            break;
        }
        let next = {
            let n = node.borrow();
            if target_val < n.val {
                n.left.clone()
            } else {
                n.right.clone()
            }
        };
        cur = next;
    }
    path
}

impl Solution1 {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p?;
        let q = q?;
        let path_p = collect_path(&root, &p);
        let path_q = collect_path(&root, &q);

        let mut ans = None;
        // 公共前缀一路相同，最后一个相同节点就是分叉前的位置
        for (a, b) in path_p.iter().zip(path_q.iter()) {
            if !Rc::ptr_eq(a, b) {
                break;
            }
            ans = Some(Rc::clone(a));
        }
        ans
    }
}

// ---------- 解法二：一次遍历 ----------
pub struct Solution2;

impl Solution2 {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p?;
        let q = q?;
        let pv = p.borrow().val;
        let qv = q.borrow().val;

        let mut cur = root;
        while let Some(node) = cur {
            let (v, left, right) = {
                let n = node.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            // 两者都严格小于当前值，说明都在左子树，分叉点不可能在这
            if pv < v && qv < v {
                cur = left;
            } else if pv > v && qv > v {
                // 两者都严格大于当前值，说明都在右子树
                cur = right;
            } else {
                // 分居两侧，或其中之一正是当前节点
                return Some(node);
            }
        }
        // 题目保证 p、q 都在树上，只有非法输入才会走到这里
        None
    }
}

// ===== 测试辅助：按层序数组建树、按值找节点 =====
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

fn find_node(
    root: &Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root.as_ref()?;
    if node.borrow().val == val {
        return Some(Rc::clone(node));
    }
    let (left, right) = {
        let n = node.borrow();
        (n.left.clone(), n.right.clone())
    };
    find_node(&left, val).or_else(|| find_node(&right, val))
}

fn check(level_order: &[Option<i32>], p_val: i32, q_val: i32, want: i32) {
    let root = build_tree(level_order);
    let p = find_node(&root, p_val).expect("p 必须存在");
    let q = find_node(&root, q_val).expect("q 必须存在");
    let want_node = find_node(&root, want).expect("期望节点必须存在");

    let got1 = Solution1::lowest_common_ancestor(
        root.clone(),
        Some(Rc::clone(&p)),
        Some(Rc::clone(&q)),
    )
    .expect("解法一不应返回空");
    // 比节点身份而不是值：ptr_eq 成立才证明返回的是那个节点
    assert!(
        Rc::ptr_eq(&got1, &want_node),
        "Solution1({:?}, {}, {}) = {}, want {}",
        level_order,
        p_val,
        q_val,
        got1.borrow().val,
        want
    );

    let got2 = Solution2::lowest_common_ancestor(root, Some(p), Some(q))
        .expect("解法二不应返回空");
    assert!(
        Rc::ptr_eq(&got2, &want_node),
        "Solution2({:?}, {}, {}) = {}, want {}",
        level_order,
        p_val,
        q_val,
        got2.borrow().val,
        want
    );
}

fn main() {
    let sample = [
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];

    // 示例 1：p、q 分居根的两侧
    check(&sample, 2, 8, 6);

    // 示例 2：p 是 q 的祖先，答案必须是 p 自己（路径必须含 target）
    check(&sample, 2, 4, 2);

    // 参数顺序无关：p 比 q 大
    check(&sample, 8, 2, 6);
    check(&sample, 5, 3, 4);

    // 边界：根就是其中一个节点
    check(&sample, 6, 0, 6);

    // 反直觉：分叉点不在根，而在左边的 2 上
    check(&sample, 0, 5, 2);

    // 反直觉：两个都在 2 的右子树里，分叉点是 4
    check(&sample, 3, 5, 4);

    // 边界：右子树内部
    check(&sample, 7, 9, 8);

    // 边界：最小规模，p、q 是根的两个孩子
    check(&[Some(2), Some(1), Some(3)], 1, 3, 2);

    // 边界：最小规模，p 是根
    check(&[Some(2), Some(1), Some(3)], 2, 1, 2);

    // 边界：200 层右斜链，值 1..200
    let mut chain = vec![Some(1)];
    for i in 2..=200 {
        chain.push(None);
        chain.push(Some(i));
    }
    check(&chain, 1, 200, 1);
    check(&chain, 150, 200, 150);
    check(&chain, 120, 130, 120);

    println!("all tests passed");
}
