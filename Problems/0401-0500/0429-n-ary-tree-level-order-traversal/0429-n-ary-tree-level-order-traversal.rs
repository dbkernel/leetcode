/*
============================================================================
429. N 叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-level-order-traversal/
分类：N 叉树 · 层序遍历（BFS / 队列）

给定一个 N 叉树，返回其节点值的层序遍历。（即从左到右，逐层遍历）。

树的序列化输入是用层序遍历，每组子节点都由 null 值分隔（参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[[1],[3,2,4],[5,6]]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]

提示：
- 树的高度不会超过 1000
- 树的节点总数在 [0, 10^4] 之间
*/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// 力扣预置的节点定义（多叉树，注意孩子是一个 Vec）。
// 平台没有为本题提供 Rust 的 Node 类型，这里按同样字段本地定义。
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
解法：BFS + 按层定长出队【最优解】

为什么慢：层序遍历要求「同层从左到右、不同层分开」。如果只用一个普通队列
逐个弹出，虽然节点顺序是对的，但不知道哪几个属于同一层；若给每个节点挂
层号再按层号分组，就要多存 O(n) 个层号。

换个角度：BFS 队列在某一时刻恰好是「当前层剩余节点 + 下一层已入队节点」。
每轮开始先记下 cnt = q.len()，再连续弹出 cnt 个节点，这批就是完整的一层；
而这 cnt 个节点入队的孩子，恰好全部是下一层，并且按父节点的顺序排好。

关键点：
  · cnt 必须在 for 循环之前取，循环中队列还会增长
  · 每轮新建 level 再 push 到答案，避免各层共用同一个 Vec
  · N 叉树没有「先左后右」，只要按 children 的下标顺序入队，层内自然有序
  · 空树要单独返回：不要入队再靠循环兜底，否则会多出一个空层
  · 队列里存 Rc 克隆（引用计数 +1），不复制节点本身

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  第 1 轮 cnt=1 → 出 1，入孩子 3、2、4 → [[1]]
  第 2 轮 cnt=3 → 出 3（入 5、6）、2、4 → [[1],[3,2,4]]
  第 3 轮 cnt=2 → 出 5、6 → [[1],[3,2,4],[5,6]]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案外，队列最多同时存两层节点
*/
struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let r = match root {
            Some(r) => r,
            None => return ans,
        };

        let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        q.push_back(r);
        while !q.is_empty() {
            // 本层节点个数必须在出队前锁定
            let cnt = q.len();
            let mut level = Vec::with_capacity(cnt);
            for _ in 0..cnt {
                let cur = q.pop_front().unwrap();
                level.push(cur.borrow().val);
                let children = cur.borrow().children.clone();
                for child in children {
                    q.push_back(child);
                }
            }
            ans.push(level);
        }
        ans
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

fn check(level_order: &[Option<i32>], want: &[Vec<i32>]) {
    assert_eq!(
        Solution::level_order(build_nary_tree(level_order)),
        want,
        "levelOrder({:?})",
        level_order
    );
}

fn main() {
    let n = |x: i32| Some(x);
    let x = None::<i32>;

    // 示例 1
    check(
        &[n(1), x, n(3), n(2), n(4), x, n(5), n(6)],
        &[vec![1], vec![3, 2, 4], vec![5, 6]],
    );

    // 示例 2
    check(
        &[
            n(1), x, n(2), n(3), n(4), n(5), x, x, n(6), n(7), x, n(8), x,
            n(9), n(10), x, x, n(11), x, n(12), x, n(13), x, x, n(14),
        ],
        &[
            vec![1],
            vec![2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13],
            vec![14],
        ],
    );

    // 边界：空树
    check(&[], &[]);

    // 边界：单节点
    check(&[n(1)], &[vec![1]]);

    // 边界：根节点没有孩子
    check(&[n(1), x], &[vec![1]]);

    // 边界：星形树，第二层顺序就是孩子数组顺序
    check(
        &[n(1), x, n(2), n(3), n(4), n(5), n(6)],
        &[vec![1], vec![2, 3, 4, 5, 6]],
    );

    // 边界：某层节点有的有孩子、有的没有（None 占位）
    check(
        &[n(1), x, n(2), n(3), x, x, n(4)],
        &[vec![1], vec![2, 3], vec![4]],
    );

    // 边界：深链（每层一个节点），深度 200
    let mut level_order: Vec<Option<i32>> = Vec::new();
    let mut want: Vec<Vec<i32>> = Vec::new();
    for i in 1..=200 {
        level_order.push(Some(i));
        level_order.push(None);
        want.push(vec![i]);
    }
    check(&level_order, &want);

    println!("all tests passed");
}
