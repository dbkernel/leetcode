/*
============================================================================
590. N 叉树的后序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-postorder-traversal/
分类：N 叉树 · 遍历（递归 / 栈 / 前序反转）

给定一个 n 叉树的根节点 root ，返回 其节点值的 后序遍历 。

n 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔
（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[5,6,3,2,4,1]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[2,6,14,11,7,3,12,8,4,13,9,10,5,1]

提示：
- 节点总数在范围 [0, 10^4] 内
- 0 <= Node.val <= 10^4
- n 叉树的高度小于或等于 1000

进阶：递归法很简单，你可以使用迭代法完成此题吗?
*/
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// 力扣预置的节点定义（多叉树，注意孩子是一个 Vec）。
// 平台没有为本题提供 Rust 的 Node 类型，这里按同样字段本地定义，
// 以免改变函数签名。
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            children: Vec::new(),
        }
    }
}

/*
解法一：递归【最优解】

官方方法一。后序遍历的定义是「所有孩子从左到右 → 根」：
  · 依次递归每个孩子；
  · 所有孩子处理完之后，再把当前节点值加入答案。

为什么它是最优解：
  · 时间 O(m)，每个节点只访问一次；
  · 辅助空间只有递归栈 O(h)，不需要 HashMap、visited 集合或成批的兄弟节点缓存。
  · 对比之下，方法三要用 O(m) 的 visited 集合，方法四的栈最坏也是 O(m)。

关键点：
  · 访问语句必须在 for 循环之后，这是与前序唯一的区别
  · children 为空时 for 循环自然跳过，叶子直接输出自己
  · 递归深度 = 树高 h，本题 h ≤ 1000，不会爆栈

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  先处理 1 的孩子：3 的孩子 5、6 → 输出 5、6，再输出 3；然后 2；然后 4；
  最后输出 1 → [5,6,3,2,4,1]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：迭代（显式栈 + 哈希表记录孩子下标）【非最优】

官方方法二。与 589 的迭代完全同构，只是「输出时机」从入栈时挪到了孩子全都
处理完之后：
  · 一路沿着第一个孩子向下，只压栈不输出，并在表中记 0；
  · 走到空后看栈顶 p，取下一个未访问孩子的下标 index；
      - index 有效 → 更新下标，转去处理该孩子；
      - index 无效 → 孩子全处理完了，输出 p、弹出、删表项。

关键点：
  · 输出必须发生在「index 无效」分支里；提前输出就变成前序了
  · 最后这个分支弹出的节点，其所有孩子一定已经输出过，这是后序的正确性来源
  · 表项在弹出时删除，保证再次入栈时从 0 重新开始
  · 哈希表让常数和空间都变差，所以不是最优
  · 节点地址用 Rc::as_ptr 转成 usize 作 key

复杂度分析：
  时间 O(m)
  空间 O(h)：栈与 HashMap 都只保存当前路径上的节点

解法三：迭代 + visited 集合（每个节点进出栈两次）【非最优】

官方方法三。换个思路：把后序看成「先压孩子、等孩子处理完再处理自己」。
  · 取栈顶 p：
      - p 是叶子，或 p 已经展开过孩子（visited 里有 p）→ 输出 p 并弹出；
      - 否则把 p 的所有孩子「从右到左」压栈，并把 p 标记为已展开。
  · 因为孩子压在 p 上面，所以孩子一定先于 p 被处理；等 p 再次成为栈顶时，
    visited 告诉我们可以输出它了。

关键点：
  · visited 标记必须在「压完孩子之后」，否则 p 会被当成已完成直接输出
  · 孩子仍要逆序压栈：从右到左入栈，出栈才是从左到右
  · 每个节点会两次成为栈顶（第一次展开孩子、第二次输出），但只输出一次
  · visited 集合最多存 O(m) 个节点，空间不如递归

复杂度分析：
  时间 O(m)
  空间 O(m)：visited 集合 + 栈

解法四：利用前序遍历反转【非最优】

官方方法四。这是本题最巧的做法，观察两个序列的关系：
  · 后序 = [孩子1…, 孩子2…, …, 根]；
  · 把前序的孩子顺序改成「从右到左」，得到 [根, 孩子k…, …, 孩子1…]；
  · 两者恰好互为逆序。
于是用前序的逆序压栈写法得到「根-右-左」，最后整体反转即可。
  · 每次弹出节点就输出；
  · 把孩子「按从左到右」压栈（这样最右孩子最先出栈）。

关键点：
  · 这里的压栈顺序与方法三相反：方法三要逆序压（为了让孩子从左到右处理），
    方法四要正序压（为了让最右孩子先出栈）
  · 翻转必须在遍历结束后整体做一次，不能边遍历边插到答案头部（那样是 O(m²)）
  · 空间最坏 O(m)：星形树时栈里同时存下所有兄弟
  · 代码最短，但空间不如递归，标记为非最优

复杂度分析：
  时间 O(m)：遍历一次 + 反转一次
  空间 O(m)：最坏为星形树，栈同时保存 O(m) 个兄弟节点

-------- 四种写法对比 --------
  递归：O(m) 时间、O(h) 空间，最省空间，标记为【最优解】
  解法二：O(m) 时间、O(h) 空间 + HashMap，常数最大，示范「显式模拟递归」
  解法三：O(m) 时间、O(m) 空间，思路是「孩子处理完再输出自己」
  解法四：O(m) 时间、O(m) 空间，代码最短，最值得记住的迭代写法
  本题 N 叉树没有 Morris 那样的 O(1) 空间遍历写法。
*/
struct Solution;

impl Solution {
    // ---------- 解法一：递归（最优解） ----------
    fn postorder_dfs(node: Option<Rc<RefCell<Node>>>, res: &mut Vec<i32>) {
        if let Some(cur) = node {
            let children = cur.borrow().children.clone();
            for child in children {
                Self::postorder_dfs(Some(child), res);
            }
            res.push(cur.borrow().val);
        }
    }

    pub fn postorder(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::postorder_dfs(root, &mut res);
        res
    }

    // ---------- 解法二：迭代 + HashMap 记录孩子下标 ----------
    pub fn postorder_iterative_with_index(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut next_index: HashMap<usize, usize> = HashMap::new();
        let mut stk: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut cur = root;

        loop {
            // 沿着第一个孩子一路向下，只压栈、不输出
            while let Some(node) = cur.clone() {
                let key = Rc::as_ptr(&node) as usize;
                stk.push(node.clone());
                let children = node.borrow().children.clone();
                if children.is_empty() {
                    cur = None;
                } else {
                    next_index.insert(key, 0);
                    cur = Some(children[0].clone());
                }
            }
            if stk.is_empty() {
                break;
            }

            let node = stk.last().unwrap().clone();
            let key = Rc::as_ptr(&node) as usize;
            // 叶子没有表项，+1 后得到 0，恰好越界
            let index = next_index.get(&key).map(|v| v + 1).unwrap_or(0);
            let children = node.borrow().children.clone();
            if index < children.len() {
                next_index.insert(key, index);
                cur = Some(children[index].clone());
            } else {
                // 孩子全部输出完，才输出自己
                res.push(node.borrow().val);
                stk.pop();
                next_index.remove(&key);
                cur = None;
            }
        }
        res
    }

    // ---------- 解法三：迭代 + visited 集合 ----------
    pub fn postorder_iterative_visited(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let r = match root {
            Some(r) => r,
            None => return res,
        };

        let mut stk = vec![r];
        let mut visited: HashSet<usize> = HashSet::new();
        while let Some(node) = stk.last().cloned() {
            let key = Rc::as_ptr(&node) as usize;
            let children = node.borrow().children.clone();
            // 叶子，或者孩子已经展开过：此时可以输出自己
            if children.is_empty() || visited.contains(&key) {
                res.push(node.borrow().val);
                stk.pop();
                continue;
            }
            // 孩子从右到左入栈，出栈顺序才是从左到右
            for child in children.into_iter().rev() {
                stk.push(child);
            }
            // 标记必须在压完孩子之后：再次成为栈顶时才会走到输出分支
            visited.insert(key);
        }
        res
    }

    // ---------- 解法四：前序遍历反转 ----------
    pub fn postorder_iterative_reverse(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stk: Vec<Rc<RefCell<Node>>> = Vec::new();
        if let Some(r) = root {
            stk.push(r);
        }

        while let Some(node) = stk.pop() {
            res.push(node.borrow().val);
            // 正序压栈：最右孩子最先出栈，得到「根-右-左」
            let children = node.borrow().children.clone();
            for child in children {
                stk.push(child);
            }
        }
        // 整体反转一次
        res.reverse();
        res
    }
}

// ===== 测试辅助：按层序数组建 N 叉树（None 表示空） =====
fn build_nary_tree(level_order: &[Option<i32>]) -> Option<Rc<RefCell<Node>>> {
    if level_order.is_empty() || level_order[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(Node::new(level_order[0].unwrap())));
    let mut nodes = vec![root.clone()];
    // 层序序列规则：根之后先来一个 None，然后是该节点的全部孩子；
    // 每个孩子按 BFS 顺序继续展开，孩子列表以 None 结尾。
    let mut idx = 2usize; // 跳过根与它后面的 None
    let mut p = 0usize;
    while idx < level_order.len() && p < nodes.len() {
        let parent = nodes[p].clone();
        p += 1;
        while idx < level_order.len() {
            match level_order[idx] {
                Some(v) => {
                    let child = Rc::new(RefCell::new(Node::new(v)));
                    parent.borrow_mut().children.push(child.clone());
                    nodes.push(child);
                    idx += 1;
                }
                None => break,
            }
        }
        idx += 1; // 跳过孩子列表的 None 结束符
    }
    Some(root)
}

fn check(level_order: &[Option<i32>], want: &[i32]) {
    assert_eq!(
        Solution::postorder(build_nary_tree(level_order)),
        want,
        "递归({:?})",
        level_order
    );
    assert_eq!(
        Solution::postorder_iterative_with_index(build_nary_tree(level_order)),
        want,
        "迭代+下标({:?})",
        level_order
    );
    assert_eq!(
        Solution::postorder_iterative_visited(build_nary_tree(level_order)),
        want,
        "迭代+visited({:?})",
        level_order
    );
    assert_eq!(
        Solution::postorder_iterative_reverse(build_nary_tree(level_order)),
        want,
        "前序反转({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(&[n(1), x, n(3), n(2), n(4), x, n(5), n(6)], &[5, 6, 3, 2, 4, 1]);

    // 示例 2
    check(
        &[
            n(1), x, n(2), n(3), n(4), n(5), x, x, n(6), n(7), x, n(8), x,
            n(9), n(10), x, x, n(11), x, n(12), x, n(13), x, x, n(14),
        ],
        &[2, 6, 14, 11, 7, 3, 12, 8, 4, 13, 9, 10, 5, 1],
    );

    // 边界：空树
    check(&[], &[]);

    // 边界：单节点
    check(&[n(1)], &[1]);

    // 边界：根节点没有孩子
    check(&[n(1), x], &[1]);

    // 边界：根的孩子都是叶子，验证兄弟间保持从左到右
    check(&[n(1), x, n(2), n(3), n(4)], &[2, 3, 4, 1]);

    // 边界：星形树
    check(&[n(1), x, n(2), n(3), n(4), n(5), n(6)], &[2, 3, 4, 5, 6, 1]);

    // 边界：中间节点没有孩子（2 无孩子，3 有孩子 4）
    check(&[n(1), x, n(2), n(3), x, x, n(4)], &[2, 4, 3, 1]);

    // 边界：深链（每个节点只有一个孩子），深度 200，后序为降序
    let mut level_order: Vec<Option<i32>> = Vec::new();
    for i in 1..=200 {
        level_order.push(Some(i));
        level_order.push(None);
    }
    let want: Vec<i32> = (1..=200).rev().collect();
    check(&level_order, &want);

    println!("all tests passed");
}
