/*
============================================================================
106. 从中序与后序遍历序列构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
分类：二叉树 · 分治构造（中序 + 后序）

给定两个整数数组 inorder 和 postorder ，其中 inorder 是二叉树的中序遍历，
postorder 是同一棵树的后序遍历，请你构造并返回这颗 二叉树 。

示例 1：
输入：inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
输出：[3,9,20,null,null,15,7]

示例 2：
输入：inorder = [-1], postorder = [-1]
输出：[-1]

提示：
- 1 <= inorder.length <= 3000
- postorder.length == inorder.length
- -3000 <= inorder[i], postorder[i] <= 3000
- inorder 和 postorder 都由 不同 的值组成
- postorder 中每一个值都在 inorder 中
- inorder 保证是树的中序遍历
- postorder 保证是树的后序遍历
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
解法一：递归分治 + 中序下标哈希表【最优解】

为什么慢：后序的最后一个元素就是根，这一步 O(1)；开销全在"根在中序的哪个
位置"。若每层递归都线性扫描中序区间找根，每层花 O(区间长)，左斜链与右斜链
都有 O(n) 层，累计退化成 O(n^2)。

换个角度：根的位置由"值 → 中序下标"决定，而题目保证值互不重复。先建一张表，
每次定位 O(1)，整棵树 O(n) 次访问就够。

关键点：
  · 后序的根在区间**右端**（`pr`），先序题在左端，这是两题唯一的切换点
  · 右子树节点数 `right_len = 中序区间右端 - 中序根下标`；后序里右半段紧贴
    根之前，即 [pl + left_len, pr - 1]
  · 左半段是 [pl, pl + left_len - 1]，left_len 与中序左段长度必须相等
  · 递归出口是区间为空（`pl > pr`），长度 1 时仍要建出叶子
  · 值域含负数，只能用 HashMap 存下标
  · Rust 用 `i32` 传下标并统一成「含两端」的闭区间，减少切片开销

以示例 1（inorder = [9,3,15,20,7]，postorder = [9,15,7,20,3]）为例：
  根 3（后序末尾），中序下标 1 → left_len = 1
  左：中序 [9]、后序 [9] → 叶子 9
  右：中序 [15,20,7]、后序 [15,7,20]，根 20（后序末尾），left_len = 1
      左边中序 [15]、后序 [15] → 15；右边中序 [7]、后序 [7] → 7
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，每个节点恰被创建一次
  空间 O(n)：HashMap 存 n 项；递归栈 O(h)，斜链时也是 O(n)

解法二：迭代栈，从后序末尾倒着走【最优解】

为什么慢：解法一要递归 O(h) 层，再额外占一张 O(n) 的哈希表；树深时递归栈
同样到 O(n)，时间上也不比解法二省。

换个角度：后序是「左 → 右 → 根」，倒着读就是「根 → 右 → 左」，等价于把
先序题的树做左右镜像后照抄。于是栈维护的是当前未完成的**右链**，中序指针
从末尾往前扫：
  · 栈顶与 `inorder[in_idx]` 不等：新节点是栈顶的右孩子，压栈继续往右下走
  · 相等：右链到底，连续弹栈跳过已在中序出现过的祖先，新节点挂在最后弹出的
    节点的**左边**

关键点：
  · in_idx 从 `n - 1` 开始、只减不增，方向与栈的「右链」一致
  · 相等时必须连续弹栈，弹到「不相等或栈空」再挂左孩子
  · 挂左孩子用的是最后弹出的节点，不是弹完后的栈顶
  · 后序末元素先建根、先入栈，循环从下标 n - 2 往下走
  · 与解法一并列最优：时间同为 O(n)，但不再需要哈希表，空间从 O(n) 降到
    O(h)；它是解法一的镜像写法，可用来对照「根在末尾」这一侧的边界
  · Rust 里栈元素是 `Rc<RefCell<TreeNode>>` 的克隆，克隆只加引用计数

以示例 1（inorder = [9,3,15,20,7]，postorder = [9,15,7,20,3]）为例：
  i=3，值 20：栈顶 3 ≠ in[4]=7 → 3.right = 20，压栈
  i=2，值 7：栈顶 20 ≠ in[4]=7 → 20.right = 7，压栈
  i=1，值 15：栈顶 7 = in[4] → 弹 7、弹 20，in_idx 走到 2；15 挂 20 左边
  i=0，值 9：栈顶 15 = in[2] → 弹 15、弹 3，in_idx 走到 0；9 挂 3 左边
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：每个节点进出栈各一次
  空间 O(h)：栈里只保存当前右链，不再需要哈希表
*/

// ---------- 解法一：递归分治 + 中序下标哈希表（最优解之一） ----------
// 单文件 rustc 编译时平台上并不存在 Solution 类型，这里显式声明
pub struct Solution1;

impl Solution1 {
    pub fn build_tree(
        inorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // pos：值 → 中序下标，题目保证无重复值
        let mut pos: HashMap<i32, i32> = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            pos.insert(v, i as i32);
        }
        let n = postorder.len() as i32;
        Self::build(&postorder, &pos, 0, n - 1, 0, n - 1)
    }

    fn build(
        post: &[i32],
        pos: &HashMap<i32, i32>,
        pl: i32,
        pr: i32,
        il: i32,
        ir: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // 空区间才是出口：长度为 1 时仍需建出那个叶子
        if pl > pr {
            return None;
        }
        let root_val = post[pr as usize];
        let k = pos[&root_val];
        let left_len = k - il;

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            // 左半段在前，右半段紧贴根，两段长度由 left_len 唯一确定
            left: Self::build(post, pos, pl, pl + left_len - 1, il, k - 1),
            right: Self::build(post, pos, pl + left_len, pr - 1, k + 1, ir),
        })))
    }
}

// ---------- 解法二：迭代栈，从后序末尾倒着走 ----------
pub struct Solution2;

impl Solution2 {
    pub fn build_tree(
        inorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let n = postorder.len();
        let root = Rc::new(RefCell::new(TreeNode::new(postorder[n - 1])));
        let mut stk: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(&root)];
        // 中序指针从末尾往前：与「根 → 右 → 左」的倒序后序同向
        let mut in_idx = n - 1;

        for i in (0..n - 1).rev() {
            let mut node = Rc::clone(stk.last().unwrap());
            if node.borrow().val != inorder[in_idx] {
                // 栈顶尚未在中序出现，倒序后序的这一项只能是它的右孩子
                let child = Rc::new(RefCell::new(TreeNode::new(postorder[i])));
                node.borrow_mut().right = Some(Rc::clone(&child));
                stk.push(child);
            } else {
                // 弹到不相等为止：最后弹出的节点正是新节点的父节点
                while stk
                    .last()
                    .is_some_and(|t| t.borrow().val == inorder[in_idx])
                {
                    node = stk.pop().unwrap();
                    in_idx -= 1;
                }
                let child = Rc::new(RefCell::new(TreeNode::new(postorder[i])));
                node.borrow_mut().left = Some(Rc::clone(&child));
                stk.push(child);
            }
        }
        Some(root)
    }
}

// ===== 测试辅助：层序序列化（None 表示空） =====
fn to_level_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut out: Vec<Option<i32>> = Vec::new();
    let Some(r) = root else {
        return out;
    };

    // 用队列逐个展开：空节点也入队一次，保证位置不塌陷
    let mut q: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![Some(Rc::clone(r))];
    let mut i = 0;
    while i < q.len() {
        // 先取出来再处理，避免一边读 q[i] 一边往 q 里 push
        match q[i].clone() {
            None => out.push(None),
            Some(node) => {
                let (val, left, right) = {
                    let n = node.borrow();
                    (n.val, n.left.clone(), n.right.clone())
                };
                out.push(Some(val));
                q.push(left);
                q.push(right);
            }
        }
        i += 1;
    }

    // 末尾的空孩子不改变结构，裁掉才能和题面输出逐项比较
    while matches!(out.last(), Some(None)) {
        out.pop();
    }
    out
}

fn check(inorder: &[i32], postorder: &[i32], want: &[Option<i32>]) {
    let got = Solution1::build_tree(inorder.to_vec(), postorder.to_vec());
    assert_eq!(to_level_order(&got), want, "optimal {:?}", inorder);

    let got = Solution2::build_tree(inorder.to_vec(), postorder.to_vec());
    assert_eq!(to_level_order(&got), want, "iterative {:?}", inorder);
}

fn main() {
    // 示例 1
    check(
        &[9, 3, 15, 20, 7],
        &[9, 15, 7, 20, 3],
        &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
    );

    // 示例 2：单节点，两个解法都不该进循环体
    check(&[-1], &[-1], &[Some(-1)]);

    // 边界：左斜链，后序升序
    check(&[1, 2, 3], &[1, 2, 3], &[Some(3), Some(2), None, Some(1)]);

    // 边界：右斜链，后序降序
    check(
        &[1, 2, 3],
        &[3, 2, 1],
        &[Some(1), None, Some(2), None, Some(3)],
    );

    // 关键点用例：根在末尾且右子树非空，右半段偏移写错时会挂到左边
    check(
        &[4, 2, 3, 1],
        &[4, 3, 2, 1],
        &[Some(1), Some(2), None, Some(4), Some(3)],
    );

    // 满二叉树
    check(
        &[4, 2, 5, 1, 6, 3, 7],
        &[4, 5, 2, 6, 7, 3, 1],
        &[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ],
    );

    // 边界：含 0 与负数，验证按值定位
    check(
        &[-3000, 0, 3000],
        &[-3000, 3000, 0],
        &[Some(0), Some(-3000), Some(3000)],
    );

    // 边界：1000 层左斜链，中序升序、后序也升序
    let ino: Vec<i32> = (1..=1000).collect();
    let post: Vec<i32> = (1..=1000).collect();
    let mut want: Vec<Option<i32>> = vec![Some(1000)];
    for v in (1..=999).rev() {
        want.push(Some(v));
        if v > 1 {
            want.push(None);
        }
    }
    check(&ino, &post, &want);

    // 边界：1000 层右斜链，中序升序、后序降序
    let post: Vec<i32> = (1..=1000).rev().collect();
    let mut want: Vec<Option<i32>> = vec![Some(1)];
    for v in 2..=1000 {
        want.push(None);
        want.push(Some(v));
    }
    check(&ino, &post, &want);

    println!("all tests passed");
}
