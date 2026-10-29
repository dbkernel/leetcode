/*
============================================================================
面试题 04.03. 特定深度节点链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/list-of-depth-lcci/
分类：二叉树 · 层序遍历（BFS）

给定一棵二叉树，设计一个算法，创建含有某一深度上所有节点的链表（比如，若
一棵树的深度为 D，则会创建出 D 个链表）。返回一个包含所有深度的链表的数组。

示例：
输入：[1,2,3,4,5,null,7,8]

        1
       /  \
      2    3
     / \    \
    4   5    7
   /
  8

输出：[[1],[2,3],[4,5,7],[8]]
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
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/*
解法：BFS 逐层建链表【最优解】

为什么慢：要找"每一层"，最直接的想法是 DFS 记录深度，把节点塞进 depth 对应
的数组，最后每层再建链表。但 DFS 先访问的节点不一定来自同一层，必须额外维护
depth 数组；它把"分层"这件事交给调用方，函数参数里到处是深度。本质上缺的是
"同一层节点连续处理"的顺序。

换个角度：层序遍历本身就把同一层节点排在一起。BFS 队列里当前这批节点正好是
一整层：先记下队列长度 size，只弹出 size 个节点建一条链表，同时把它们的左右
孩子入队，下一轮队列里就是下一层。每轮产出一条链表，自然对应一个深度。

关键点：
  · 每轮开始先固定 size = queue.len()，循环中用 0..size 而不是队列实时长度，
    否则本层入队的孩子会把下一层节点卷进来
  · 用哨兵 dummy 建链表：dummy.next 就是本层头节点，tail 负责尾插，
    省掉"第一个节点特殊处理"
  · 孩子入队前判空，空孩子不入队，否则会凭空多出一层
  · 空树直接返回空 Vec，不能进循环，否则会把 None 入队
  · 返回的是每层链表头节点组成的 Vec，顺序从浅到深
  · Rc<RefCell<TreeNode>> 只需要在入队时 clone 指针，不复制节点本身

以示例（[1,2,3,4,5,null,7,8]）为例：
  第一轮 size = 1：弹出 1，建 [1]，2、3 入队；
  第二轮 size = 2：弹出 2、3，建 [2,3]，4、5、7 入队；
  第三轮 size = 3：弹出 4、5、7，建 [4,5,7]，只有 4 带左孩子 8；
  第四轮 size = 1：弹出 8，建 [8]，无孩子，队列空；
  返回 [[1],[2,3],[4,5,7],[8]] ✔

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次，每个节点新建一个链表节点
  空间 O(n)：队列最宽一层最多 n/2 个节点，输出本身也占 O(n)
*/
// rustc 单文件编译时平台不会提供 Solution，需要自行声明
struct Solution;

impl Solution {
    pub fn list_of_depth(
        tree: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut levels: Vec<Option<Box<ListNode>>> = Vec::new();
        let root = match tree {
            Some(node) => node,
            None => return levels,
        };

        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            // 先固定本层大小：本层孩子入队后 len() 会变大
            let size = queue.len();
            let mut dummy = ListNode::new(0);
            let mut tail: &mut ListNode = &mut dummy;

            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                tail.next = Some(Box::new(ListNode::new(node.val)));
                tail = tail.next.as_mut().unwrap();

                // 只 clone Rc 指针，左右子树本身不复制
                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }

            // dummy.next 是本层链表头，空层不会出现（size >= 1）
            levels.push(dummy.next);
        }
        levels
    }
}

// ===== 测试辅助：按层序数组（None 表示空）建树、收集每层值 =====
fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < values.len() {
        let parent = queue.pop_front().unwrap();
        if i < values.len() {
            if let Some(v) = values[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                parent.borrow_mut().left = Some(child.clone());
                queue.push_back(child);
            }
            i += 1;
        }
        if i < values.len() {
            if let Some(v) = values[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                parent.borrow_mut().right = Some(child.clone());
                queue.push_back(child);
            }
            i += 1;
        }
    }
    Some(root)
}

fn collect_levels(levels: Vec<Option<Box<ListNode>>>) -> Vec<Vec<i32>> {
    levels
        .into_iter()
        .map(|head| {
            let mut values = Vec::new();
            let mut cur = head;
            while let Some(node) = cur {
                values.push(node.val);
                cur = node.next;
            }
            values
        })
        .collect()
}

fn check(level_order: &[Option<i32>], want: Vec<Vec<i32>>) {
    let got = collect_levels(Solution::list_of_depth(build_tree(level_order)));
    assert_eq!(got, want, "listOfDepth({:?})", level_order);
}

fn main() {
    // 示例：第四层只有一个节点，最容易漏层
    check(
        &[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8)],
        vec![vec![1], vec![2, 3], vec![4, 5, 7], vec![8]],
    );

    // 边界：空树，返回空 Vec 而不是含一个空链表的 Vec
    check(&[], vec![]);

    // 边界：单节点，只有一层
    check(&[Some(1)], vec![vec![1]]);

    // 满二叉树，最宽一层有 4 个节点
    check(
        &[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)],
        vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]],
    );

    // 边界：只有左孩子的斜链，每层一个节点
    check(
        &[Some(1), Some(2), None, Some(3), None, Some(4)],
        vec![vec![1], vec![2], vec![3], vec![4]],
    );

    // 边界：只有右孩子的斜链，每层一个节点
    check(
        &[Some(1), None, Some(2), None, Some(3)],
        vec![vec![1], vec![2], vec![3]],
    );

    // 反直觉：中间层缺孩子，但下一层仍有节点
    check(
        &[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)],
        vec![vec![1], vec![2, 3], vec![4, 5]],
    );

    // 反直觉：左右孩子交叉分布，层内顺序必须从左到右
    check(
        &[Some(1), Some(2), Some(3), Some(4), None, None, Some(5)],
        vec![vec![1], vec![2, 3], vec![4, 5]],
    );

    // 边界：负数
    check(
        &[Some(-1), Some(-2), Some(-3)],
        vec![vec![-1], vec![-2, -3]],
    );

    println!("all tests passed");
}
