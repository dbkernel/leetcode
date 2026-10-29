/*
============================================================================
面试题 17.12. BiNode（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binode-lcci/
分类：二叉搜索树 · 中序遍历 / Morris

二叉树数据结构TreeNode可用来表示单向链表（其中left置空，right为下一个链表
节点）。实现一个方法，把二叉搜索树转换为单向链表，要求依然符合二叉搜索树的
性质，转换操作应是原址的，也就是在原始的二叉搜索树上直接修改。

返回转换后的单向链表的头节点。

注意：本题相对原题稍作改动

示例：
输入： [4,2,5,1,3,null,6,0]
输出： [0,null,1,null,2,null,3,null,4,null,5,null,6]

提示：
- 节点数量不会超过 100000。
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
解法一：中序遍历，递归返回 (头, 尾) 再拼接【非最优】

为什么慢：中序遍历本身 O(n)，慢的地方在"怎么原地拼"。如果只保存一个"上一
个访问的节点"，递归回溯时无法在 O(1) 里把左链表接到根上；如果先收集节点再
重连，又要 O(n) 数组。真正的瓶颈是信息不够：拼接需要同时知道一段链表的头
和尾。

换个角度：让递归函数直接返回展开后的 (头, 尾) 一对。根的中序序列就是
"左链表 + 根 + 右链表"，拿到左尾、右头之后，拼接只是两次指针赋值，返回时
把头尾合并即可。

关键点：
  · 返回值必须是 (头, 尾) 一对：只返回头就得 O(n) 找尾才能拼接
  · 尾的取法：右子树非空取右尾，否则取当前节点；不能一律取当前节点
  · 必须先递归完左右子树再改当前节点的指针，否则会丢失子树
  · 拼接后当前节点的 left 必须置空（题目要求单向链表）
  · 空树返回 (None, None)，作为递归出口
  · Rust 先 clone 出左右孩子再递归，避免 `borrow()` 活着时去 `borrow_mut()`

以示例（root = [4,2,5,1,3,null,6,0]）为例：
  节点 0：无子树，返回 (0, 0)；
  节点 1：左尾 0 接到 1，返回 (0, 1)；
  节点 2：左链表 (0,1) 的尾 1 接到 2，右链表 (3,3) 接在 2 后，
          返回 (0, 3)；
  依此向上，最终头是最左的 0，右链为 0→1→…→6 ✔

复杂度分析：
  时间 O(n)：每个节点进入递归一次，拼接是常数次赋值
  空间 O(h)：递归栈，h 为树高；最坏（斜树）O(n)，这正是解法二要省的

解法二：Morris 中序遍历，原地调整指针【最优解】

为什么慢：解法一的递归栈是 O(h)。中序需要栈，只是为了"从左子树回来后知道
该访问谁"；这个"谁"其实就是当前节点，可以用左子树最右节点的空 right 指针
直接指回来（线索），不需要真正的栈。

换个角度：中序里，节点 cur 的前驱是"左子树最右节点"。让前驱的空 right 指向
cur，向左走；等左子树遍历完，顺着这条线索回到 cur，此时才访问 cur，并把
"上一个访问节点"的 right 接成 cur，同时清空 cur 的 left。遍历和重连在同一个
循环里完成。

关键点：
  · 有左孩子时不算访问：先找前驱 pred（左子树最右），pred.right = cur 建
    线索，清空 cur.left，再向左走
  · 没有左孩子时才是访问：prev.right = cur；prev 为空时 cur 就是链表头
  · 线索只在回到 cur 后被 prev.right = cur 覆盖，值相同，不必单独拆链
  · 每个被访问节点的 left 都必须为 None；有左孩子的节点在向左走之前就清空
  · 最后访问的节点是中序最大节点，它的 right 本来就是 None，无需额外处理
  · `take()` 取出左孩子后当前节点的 left 已为 None，不必再单独清空

以示例（root = [4,2,5,1,3,null,6,0]）为例：
  一路向左：4 的前驱 3 指向 4，2 的前驱 1 指向 2，1 的前驱 0 指向 1；
  访问 0（头）、顺线索回 1 并访问、回 2 并访问、访问 3、回 4；
  再依次访问 5、6，右链 0→1→2→3→4→5→6 ✔

复杂度分析：
  时间 O(n)：每条线索（边）最多被走两次，建线索一次、沿线索返回一次
  空间 O(1)：只用 cur、prev、pred 三个指针，不借助栈
*/

// ---------- 解法一：中序遍历，返回 (头, 尾) ----------
// 单文件 rustc 编译时平台不会注入 Solution 类型，这里显式声明
pub struct Solution1;

impl Solution1 {
    pub fn convert_bi_node(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(root).0
    }

    // 返回展开后的 (头, 尾)：只返回头就没法 O(1) 接上左链表
    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> (
        Option<Rc<RefCell<TreeNode>>>,
        Option<Rc<RefCell<TreeNode>>>,
    ) {
        let Some(n) = node else {
            return (None, None);
        };

        // 先取出孩子再递归：borrow 不跨递归，后面才能 borrow_mut 改指针
        let (left, right) = {
            let b = n.borrow();
            (b.left.clone(), b.right.clone())
        };
        let (left_head, left_tail) = Self::dfs(left);
        let (right_head, right_tail) = Self::dfs(right);

        n.borrow_mut().left = None;
        n.borrow_mut().right = right_head;
        if let Some(t) = &left_tail {
            t.borrow_mut().right = Some(Rc::clone(&n));
        }

        let head = left_head.unwrap_or_else(|| Rc::clone(&n));
        // 尾取右链的尾；没有右子树时当前节点就是尾
        let tail = right_tail.unwrap_or_else(|| Rc::clone(&n));
        (Some(head), Some(tail))
    }
}

// ---------- 解法二：Morris 中序遍历，原地调整指针 ----------
pub struct Solution2;

impl Solution2 {
    pub fn convert_bi_node(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root;
        let mut head: Option<Rc<RefCell<TreeNode>>> = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while let Some(node) = cur.clone() {
            let left = node.borrow_mut().left.take();
            match left {
                Some(l) => {
                    // 前驱 = 左子树最右节点；它此刻还没有右孩子
                    let mut pred = Rc::clone(&l);
                    loop {
                        let next = pred.borrow().right.clone();
                        match next {
                            Some(n) => pred = n,
                            None => break,
                        }
                    }
                    pred.borrow_mut().right = Some(Rc::clone(&node)); // 线索

                    cur = Some(l);
                }
                None => {
                    // 访问 cur：把上一个访问节点的 right 接过来
                    match &prev {
                        Some(p) => {
                            p.borrow_mut().right = Some(Rc::clone(&node));
                        }
                        // 第一个访问的就是最左节点，即链表头
                        None => head = Some(Rc::clone(&node)),
                    }
                    prev = Some(Rc::clone(&node));
                    cur = node.borrow().right.clone(); // 可能是线索或原右子树
                }
            }
        }

        head
    }
}

// ===== 测试辅助：按层序数组（None 表示空）建树、按右链收集 =====
const MAX_NODES: usize = 100_000; // 题目上限，用来兜住成环的错误

fn build_tree(level_order: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let first = (*level_order.first()?)?;
    let root = Rc::new(RefCell::new(TreeNode::new(first)));

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

// 沿 right 收集：同时断言 left 全为空、右链没有环
fn collect_right_chain(head: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        let (val, next) = {
            let b = node.borrow();
            assert!(b.left.is_none(), "node {} 残留左孩子", b.val);
            (b.val, b.right.clone())
        };
        values.push(val);
        assert!(values.len() <= MAX_NODES, "right 链成环");
        cur = next;
    }
    values
}

// BST 的中序就是升序，所以期望值 = 输入里的非空值排序
fn sorted_values(level_order: &[Option<i32>]) -> Vec<i32> {
    let mut values: Vec<i32> = level_order.iter().flatten().copied().collect();
    values.sort_unstable();
    values
}

fn check(level_order: &[Option<i32>]) {
    let want = sorted_values(level_order);

    let head1 = Solution1::convert_bi_node(build_tree(level_order));
    assert_eq!(collect_right_chain(head1), want);

    let head2 = Solution2::convert_bi_node(build_tree(level_order));
    assert_eq!(collect_right_chain(head2), want);
}

fn main() {
    // 示例：左右都有子树，且最左节点在第三层
    check(&[
        Some(4),
        Some(2),
        Some(5),
        Some(1),
        Some(3),
        None,
        Some(6),
        Some(0),
    ]);

    // 边界：空树
    check(&[]);

    // 边界：单节点
    check(&[Some(1)]);

    // 边界：只有左孩子的斜链，头是最深的那个
    check(&[Some(3), Some(2), None, Some(1)]);

    // 边界：只有右孩子的斜链，头就是根
    check(&[Some(1), None, Some(2), None, Some(3)]);

    // 反直觉：右子树里有比根小的节点 —— 结果仍必须是全升序
    check(&[Some(1), None, Some(3), Some(2)]);

    // 反直觉：根的两个孩子都不是最值，最左/最右藏在深处
    check(&[
        Some(10),
        Some(5),
        Some(15),
        Some(2),
        Some(7),
        Some(12),
        Some(20),
        Some(1),
        Some(3),
        Some(6),
        Some(8),
        Some(11),
        Some(13),
    ]);

    // 满 BST
    check(&[
        Some(4),
        Some(2),
        Some(6),
        Some(1),
        Some(3),
        Some(5),
        Some(7),
    ]);

    // 边界：2000 层左斜链，验证递归与线索都不改变顺序
    let mut chain = vec![Some(2000)];
    for i in (1..=1999).rev() {
        chain.push(Some(i));
        chain.push(None);
    }
    check(&chain);

    println!("all tests passed");
}
