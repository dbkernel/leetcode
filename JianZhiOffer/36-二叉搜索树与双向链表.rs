/*
============================================================================
剑指 Offer 36. 二叉搜索树与双向链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/
分类：二叉搜索树 · 中序遍历 / 双向循环链表

将一个 二叉搜索树 就地转化为一个 已排序的双向循环链表 。

对于双向循环列表，你可以将左右孩子指针作为双向循环链表的前驱和后继指针，
第一个节点的前驱是最后一个节点，最后一个节点的后继是第一个节点。

特别地，我们希望可以 就地 完成转换操作。当转化完成以后，树中节点的左指针
需要指向前驱，树中节点的右指针需要指向后继。还需要返回链表中最小元素的
指针。

示例 1：
输入：root = [4,2,5,1,3]
输出：[1,2,3,4,5]
解释：下图显示了转化后的二叉搜索树，实线表示后继关系，虚线表示前驱关系。

示例 2：
输入：root = [2,1,3]
输出：[1,2,3]

示例 3：
输入：root = []
输出：[]
解释：输入是空树，所以输出也是空链表。

示例 4：
输入：root = [1]
输出：[1]

提示：
- -1000 <= Node.val <= 1000
- Node.left.val < Node.val < Node.right.val
- Node.val 的所有值都是独一无二的
- 0 <= Number of Nodes <= 2000

注意：本题与主站 426 题相同：
https://leetcode.cn/problems/convert-binary-search-tree-to-sorted-doubly-linked-list/
*/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// 力扣预置的节点定义
#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

/*
解法一：中序遍历 + 全局前驱指针【最优解】

为什么慢：BST 的中序序列就是升序，转换的难点不在"排序"，而在"改指针时不能
丢节点"。最直白的做法是先中序收集所有节点到数组，再按数组重连：正确但要多
一个 O(n) 的数组，而题目明确要求就地转换。

换个角度：中序遍历每次访问节点时，恰好知道它的前一个节点是谁。只要在访问
当前节点 node 时保存着 prev，就能当场把 prev.right = node、node.left = prev
接好，指针改完再递归右子树，右子树根本没被碰过。

关键点：
  · 先递归左子树、再改指针、最后递归右子树 —— 顺序固定，提前改指针会丢掉
    还没访问的右子树或左子树
  · prev 为空时才赋值 head：中序第一个访问的节点就是最小值，即链表头
  · 递归结束后 prev 停在中序最后一个节点（最大值），此时才补首尾互指：
    head.left = prev、prev.right = head，少这一步得到的是单向链表
  · 空树必须提前返回 None，否则 head 一直为空，补互指时会解引用空指针
  · 返回的是 head（最小元素），不是 root —— 根在中序中间，位置会变
  · Rc<RefCell<Node>> 里改指针要 borrow_mut，且不能同时持有同一节点的
    可变与不可变借用；先把 left/right 克隆出来，再改当前节点

以示例 1（root = [4,2,5,1,3]）为例：
  中序访问顺序 1、2、3、4、5；
  访问 1 时 prev 为空 → head = 1；访问 2 时 1.right = 2、2.left = 1；
  依此接到 5，结束时 prev = 5，补 5.right = 1、1.left = 5 ✔
  （若漏掉补互指，5 的 right 仍是原来的空指针，链表不成环。）

复杂度分析：
  时间 O(n)：每个节点访问一次，接线是常数次赋值
  空间 O(h)：递归栈深度等于树高，最坏（斜树）O(n)

解法二：显式栈中序【非最优】

为什么慢：思路与解法一完全相同，只是把系统递归栈换成手写栈。时间、空间
同为 O(n)/O(h)，与解法一同为最优；标为非最优只是避免重复计最优。它的实际
好处是递归深度不再受语言默认栈限制，2000 层的斜树也不会栈溢出。

换个角度：中序的"下一个访问节点"就是栈顶。一路向左压栈到底，弹栈即访问，
访问完把游标移到右孩子再重复压栈动作。

关键点：
  · 压栈用 while 走完整条左链，不能只压一层 —— 弹出时才保证是当前最小
  · 弹栈后立刻接 prev.right 和 cur.left，此时 cur 的右孩子还没被动过，
    所以 cur = cur.right 拿到的仍是原右子树
  · head 同样只在 prev 为空时赋值，即中序第一个节点
  · 循环条件是 cur.is_some() || !stack.is_empty()：游标空了还要继续弹栈，
    写成 cur.is_some() 会漏掉整条右链
  · 结束后与解法一一样补首尾互指

以示例 2（root = [2,1,3]）为例：
  压 2、1 → 弹 1（head = 1）→ cur 为 1 的右孩子（空）；
  弹 2，接 1.right = 2、2.left = 1 → cur 为 3；
  压 3、弹 3，接 2.right = 3、3.left = 2；
  补 3.right = 1、1.left = 3 ✔

复杂度分析：
  时间 O(n)：每个节点入栈、出栈、接线各一次
  空间 O(h)：栈最多保存一条根到叶的路径，最坏（斜树）O(n)
*/
// rustc 单文件编译时平台不会提供 Solution，需要自行声明
struct Solution1;

impl Solution1 {
    pub fn tree_to_doubly_list(
        root: Option<Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        if root.is_none() {
            return None;
        }

        let mut head = None;
        let mut prev = None;
        Self::inorder(root, &mut head, &mut prev);

        // 首尾互指：prev 停在中序最后一个节点（最大值）
        if let (Some(h), Some(t)) = (head.as_ref(), prev.as_ref()) {
            h.borrow_mut().left = Some(t.clone());
            t.borrow_mut().right = Some(h.clone());
        }
        head
    }

    fn inorder(
        node: Option<Rc<RefCell<Node>>>,
        head: &mut Option<Rc<RefCell<Node>>>,
        prev: &mut Option<Rc<RefCell<Node>>>,
    ) {
        let cur = match node {
            Some(node) => node,
            None => return,
        };

        // 先取出左孩子再往下递归，避免与后面的 borrow_mut 重叠
        let left = cur.borrow().left.clone();
        Self::inorder(left, head, prev);

        // 中序第一个节点没有前驱，它的位置就是链表头
        match prev.as_ref() {
            Some(p) => {
                p.borrow_mut().right = Some(cur.clone());
                cur.borrow_mut().left = Some(p.clone());
            }
            None => *head = Some(cur.clone()),
        }
        *prev = Some(cur.clone());

        // 右子树尚未被改动，克隆指针后递归进去
        let right = cur.borrow().right.clone();
        Self::inorder(right, head, prev);
    }
}

struct Solution2;

impl Solution2 {
    pub fn tree_to_doubly_list(
        root: Option<Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        if root.is_none() {
            return None;
        }

        let mut head = None;
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            // 一路向左到底，栈顶就是当前最小的未访问节点
            while let Some(node) = cur.clone() {
                cur = node.borrow().left.clone();
                stack.push(node);
            }

            let node = stack.pop().unwrap();

            match prev.as_ref() {
                Some(p) => {
                    p.borrow_mut().right = Some(node.clone());
                    node.borrow_mut().left = Some(p.clone());
                }
                None => head = Some(node.clone()),
            }
            prev = Some(node.clone());

            // node 的右孩子没被改过，这里拿到的是原右子树
            cur = node.borrow().right.clone();
        }

        if let (Some(h), Some(t)) = (head.as_ref(), prev.as_ref()) {
            h.borrow_mut().left = Some(t.clone());
            t.borrow_mut().right = Some(h.clone());
        }
        head
    }
}

// ===== 测试辅助：按层序数组（None 表示空）建树、校验双向循环链表 =====
const MAX_NODES: usize = 2000; // 题目上限，用来兜住成环的错误

fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<Node>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(Node::new(values[0].unwrap())));
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < values.len() {
        let parent = queue.pop_front().unwrap();
        if i < values.len() {
            if let Some(v) = values[i] {
                let child = Rc::new(RefCell::new(Node::new(v)));
                parent.borrow_mut().left = Some(child.clone());
                queue.push_back(child);
            }
            i += 1;
        }
        if i < values.len() {
            if let Some(v) = values[i] {
                let child = Rc::new(RefCell::new(Node::new(v)));
                parent.borrow_mut().right = Some(child.clone());
                queue.push_back(child);
            }
            i += 1;
        }
    }
    Some(root)
}

fn verify(head: Option<Rc<RefCell<Node>>>, want: &[i32]) {
    if want.is_empty() {
        assert!(head.is_none());
        return;
    }

    let n = want.len();
    assert!(n <= MAX_NODES);

    let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(n);
    let mut cur = head.clone();
    for i in 0..n {
        // 走到 None 说明链表长度不足，接错指针时会在这里暴露
        let node = cur.expect("链表提前断开");
        assert_eq!(node.borrow().val, want[i]);
        nodes.push(node.clone());
        cur = node.borrow().right.clone();
    }
    // 走满 n 步必须回到 head，多出节点或断链都不满足循环链表定义
    let back = cur.expect("链表没有回到 head");
    assert!(Rc::ptr_eq(&back, head.as_ref().unwrap()));

    for i in 0..n {
        // 头的前驱是尾、尾的后继是头，正好由下标取模表达
        let prev = &nodes[(i + n - 1) % n];
        let next = &nodes[(i + 1) % n];
        let node = nodes[i].borrow();
        assert!(Rc::ptr_eq(node.left.as_ref().unwrap(), prev));
        assert!(Rc::ptr_eq(node.right.as_ref().unwrap(), next));
    }

    // 环会让 Rc 永不释放，测完主动断开所有指针
    for node in &nodes {
        let mut node = node.borrow_mut();
        node.left = None;
        node.right = None;
    }
}

fn check(level_order: &[Option<i32>]) {
    let mut want: Vec<i32> = level_order.iter().filter_map(|v| *v).collect();
    want.sort_unstable();

    verify(Solution1::tree_to_doubly_list(build_tree(level_order)), &want);
    verify(Solution2::tree_to_doubly_list(build_tree(level_order)), &want);
}

fn main() {
    // 示例 1：左右子树都有节点，最小值在最左端
    check(&[Some(4), Some(2), Some(5), Some(1), Some(3)]);

    // 示例 2：只有三个节点
    check(&[Some(2), Some(1), Some(3)]);

    // 示例 3：空树，必须返回 None 而不是崩溃
    check(&[]);

    // 示例 4：单节点，首尾都是它自己
    check(&[Some(1)]);

    // 反直觉：最小值不在根的左孩子上，而在左孩子的左孩子上
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

    // 边界：只有左孩子的斜链，头是最深的那个、尾是根
    check(&[Some(3), Some(2), None, Some(1)]);

    // 边界：只有右孩子的斜链，头是根、尾是最深的那个
    check(&[Some(1), None, Some(2), None, Some(3)]);

    // 边界：2000 层左斜链，压一压递归与栈的深度
    let mut left_chain: Vec<Option<i32>> = vec![Some(2000)];
    for i in (1..2000).rev() {
        left_chain.push(Some(i));
        left_chain.push(None);
    }
    check(&left_chain);

    // 边界：2000 层右斜链
    let mut right_chain: Vec<Option<i32>> = vec![Some(1)];
    for i in 2..=2000 {
        right_chain.push(None);
        right_chain.push(Some(i));
    }
    check(&right_chain);

    println!("all tests passed");
}
