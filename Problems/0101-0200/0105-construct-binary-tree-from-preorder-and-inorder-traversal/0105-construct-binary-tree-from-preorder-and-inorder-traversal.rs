/*
============================================================================
105. 从前序与中序遍历序列构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
分类：二叉树 · 分治构造（先序 + 中序）

给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历，
inorder 是同一棵树的中序遍历，请构造二叉树并返回其根节点。

示例 1：
输入: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
输出: [3,9,20,null,null,15,7]

示例 2：
输入: preorder = [-1], inorder = [-1]
输出: [-1]

提示：
- 1 <= preorder.length <= 3000
- inorder.length == preorder.length
- -3000 <= preorder[i], inorder[i] <= 3000
- preorder 和 inorder 均 无重复 元素
- inorder 均出现在 preorder
- preorder 保证 为二叉树的前序遍历序列
- inorder 保证 为二叉树的中序遍历序列
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

为什么慢：先序的第一个元素就是根，这一步是 O(1)；真正的开销在于必须知道
"根落在中序的哪个位置"，才能把左右两段切出来。若每层递归都线性扫描中序区间
找根，每层花 O(区间长)，链状树有 O(n) 层，累计退化成 O(n^2)。

换个角度：定位根需要的唯一信息是"值 → 中序下标"，而题目保证所有值互不重复。
先用一次遍历把这张表建好，之后每次定位都是 O(1)，整棵树只需 O(n) 次访问。

关键点：
  · 左子树节点数 `left_len = 中序根下标 - 中序区间左端`，先序与中序两侧必须
    用同一个 left_len 切分；两处各写一套偏移，错一个就整棵树错位
  · 先序右半段的起点是 `pl + left_len + 1`，多出的 1 是被根占掉的那一位
  · 递归出口是区间为空（`pl > pr`），不能写成 `pl == pr` 时返回
  · 值域含负数，不能把值当下标，只能用 HashMap
  · 中序区间与先序区间长度始终相等，可用它自检切分有没有写错
  · Rust 用 `i32` 传下标并把它们统一成「含两端」的闭区间，减少切片开销

以示例 1（preorder = [3,9,20,15,7]，inorder = [9,3,15,20,7]）为例：
  根 3，中序下标 1 → left_len = 1 - 0 = 1
  左：先序 [9]、中序 [9]，left_len = 1 - 0 = 0 → 叶子 9
  右：先序 [20,15,7]、中序 [15,20,7]，根 20，left_len = 2 - 2 = 0
      右半段：先序 [15,7]、中序 [15,7]，根 15，left_len = 0 → 15 带右孩子 7
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，之后每个节点恰被创建一次
  空间 O(n)：HashMap 存 n 项；递归栈 O(h)，斜链时也是 O(n)

解法二：迭代栈，按先序顺序逐个挂节点【最优解】

为什么慢：解法一每次递归都要压一个栈帧，树退化成链时递归深度等于 n，而
哈希表本身又占 O(n) 空间。时间上并没有更省，只是换取少一张哈希表。

换个角度：先序序列本身就是"一路向左下走"的记录。用栈维护当前这条未完成的
左链，栈顶就是"最深的一个还没在中序里出现过的祖先"。于是只需比较栈顶与中序
当前指针：
  · 不相等：新节点是栈顶的左孩子，压栈继续往左下走
  · 相等：左链到底了，连续弹栈跳过所有已在中序出现过的祖先，新节点挂在最后
    弹出的那个节点的右边

关键点：
  · 比较的是「栈顶的值」与 `inorder[in_idx]`，in_idx 从 0 起只增不减
  · 相等时必须连续弹栈，弹到「不相等或栈空」为止再挂右孩子；只弹一个会把
    剩下的祖先错当父节点
  · 挂右孩子用的是最后弹出的节点 `node`，不是弹完之后的栈顶
  · 先序首元素先建根、先入栈，循环从下标 1 开始
  · 与解法一并列最优：时间同为 O(n)，但不再需要哈希表，空间从 O(n) 降到
    O(h)；代价是状态推进更绕，建议先读懂解法一再回来看这一版
  · Rust 里栈元素是 `Rc<RefCell<TreeNode>>` 的克隆，克隆只加引用计数，
    不会深拷贝子树

以示例 1（preorder = [3,9,20,15,7]，inorder = [9,3,15,20,7]）为例：
  i=1，值 9：栈顶 3 ≠ in[0]=9 → 3.left = 9，压栈
  i=2，值 20：栈顶 9 = in[0] → 弹 9、弹 3，in_idx 走到 2；20 挂 3 右边
  i=3，值 15：栈顶 20 ≠ in[2]=15 → 20.left = 15
  i=4，值 7：栈顶 15 = in[2] → 弹 15、弹 20，in_idx 走到 4；7 挂 20 右边
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：每个节点进出栈各一次
  空间 O(h)：栈里只保存当前左链，不再需要哈希表
*/

// ---------- 解法一：递归分治 + 中序下标哈希表（最优解之一） ----------
// 单文件 rustc 编译时平台上并不存在 Solution 类型，这里显式声明
pub struct Solution1;

impl Solution1 {
    pub fn build_tree(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // pos：值 → 中序下标，题目保证无重复值，所以一个值只对应一个位置
        let mut pos: HashMap<i32, i32> = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            pos.insert(v, i as i32);
        }
        let n = preorder.len() as i32;
        Self::build(&preorder, &pos, 0, n - 1, 0, n - 1)
    }

    fn build(
        pre: &[i32],
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
        let root_val = pre[pl as usize];
        let k = pos[&root_val];
        let left_len = k - il;

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            // 先序左半段是 [pl+1, pl+left_len]，右半段从 pl+left_len+1 起
            left: Self::build(pre, pos, pl + 1, pl + left_len, il, k - 1),
            right: Self::build(pre, pos, pl + left_len + 1, pr, k + 1, ir),
        })))
    }
}

// ---------- 解法二：迭代栈 ----------
pub struct Solution2;

impl Solution2 {
    pub fn build_tree(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut stk: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(&root)];
        // in_idx 之后的元素都还没被确认，等于栈顶时说明左链到底
        let mut in_idx = 0usize;

        for &val in preorder.iter().skip(1) {
            let mut node = Rc::clone(stk.last().unwrap());
            if node.borrow().val != inorder[in_idx] {
                // 栈顶尚未在中序出现，先序的这一项只能是它的左孩子
                let child = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(Rc::clone(&child));
                stk.push(child);
            } else {
                // 弹到不相等为止：最后弹出的节点正是新节点的父节点
                while stk
                    .last()
                    .is_some_and(|t| t.borrow().val == inorder[in_idx])
                {
                    node = stk.pop().unwrap();
                    in_idx += 1;
                }
                let child = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(Rc::clone(&child));
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

fn check(preorder: &[i32], inorder: &[i32], want: &[Option<i32>]) {
    let got = Solution1::build_tree(preorder.to_vec(), inorder.to_vec());
    assert_eq!(to_level_order(&got), want, "optimal {:?}", preorder);

    let got = Solution2::build_tree(preorder.to_vec(), inorder.to_vec());
    assert_eq!(to_level_order(&got), want, "iterative {:?}", preorder);
}

fn main() {
    // 示例 1
    check(
        &[3, 9, 20, 15, 7],
        &[9, 3, 15, 20, 7],
        &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
    );

    // 示例 2：单节点，递归与迭代都不该进循环体
    check(&[-1], &[-1], &[Some(-1)]);

    // 边界：左斜链，左子树长度取到最大
    check(&[3, 2, 1], &[1, 2, 3], &[Some(3), Some(2), None, Some(1)]);

    // 边界：右斜链，每层 left_len 都是 0
    check(
        &[1, 2, 3],
        &[1, 2, 3],
        &[Some(1), None, Some(2), None, Some(3)],
    );

    // 关键点用例：左右子树长度不同，切分偏移写错时这里会错位
    check(
        &[1, 2, 4, 3],
        &[4, 2, 3, 1],
        &[Some(1), Some(2), None, Some(4), Some(3)],
    );

    // 满二叉树：每层两侧都非空
    check(
        &[1, 2, 4, 5, 3, 6, 7],
        &[4, 2, 5, 1, 6, 3, 7],
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

    // 边界：含 0 与负数，验证按值定位而不是按下标推算
    check(
        &[0, -3000, 3000],
        &[-3000, 0, 3000],
        &[Some(0), Some(-3000), Some(3000)],
    );

    // 边界：1000 层左斜链，先序降序而中序升序，验证两侧切分方向没写反
    let pre: Vec<i32> = (1..=1000).rev().collect();
    let ino: Vec<i32> = (1..=1000).collect();
    let mut want: Vec<Option<i32>> = vec![Some(1000)];
    for v in (1..=999).rev() {
        want.push(Some(v));
        if v > 1 {
            want.push(None);
        }
    }
    check(&pre, &ino, &want);

    println!("all tests passed");
}
