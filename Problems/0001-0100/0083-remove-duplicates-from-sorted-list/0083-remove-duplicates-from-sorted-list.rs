/*
============================================================================
LeetCode 83. 删除排序链表中的重复元素（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-list/
分类：链表 · 一次遍历去重
============================================================================

给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。
返回 已排序的链表 。

示例 1：
输入：head = [1,1,2]
输出：[1,2]

示例 2：
输入：head = [1,1,2,3,3]
输出：[1,2,3]

提示：
链表中节点数目在范围 [0, 300] 内
-100 <= Node.val <= 100
题目数据保证链表已经按升序 排列
*/

// 平台预置的链表节点定义；本地用 rustc 单文件编译时需要显式声明
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// 平台上由判题系统预置 Solution 类型，本地用 rustc 单文件编译时需要显式声明
struct Solution;

/*
解法：一次遍历（当前节点与下一个节点比较）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每个节点访问一次，
时间 O(n) 已是下界，且完全不需要额外空间。

为什么慢：想「收集所有不同值再重建链表」就要 O(n) 额外空间；想「发现重复
就回头找前驱」又得 O(n^2)。而链表本身已经有序，重复元素必然相邻，这个
性质足以让一趟扫描解决问题。

换个角度：站在当前节点上，只需要看它的下一个节点值是否相同：
  · 相同 → 把当前节点的 next 跨过下一个节点（丢弃重复的那个），当前节点
    本身不动，继续和新的 next 比
  · 不同 → 当前节点前进一步
因为链表有序，每个值只会连续出现一段，处理完这段就再也不会遇到它。

关键点：
  · 删除时当前节点不动 —— 否则 [1,1,1] 这种连续重复会漏删
  · 只有值不同才前进 —— 这一步是「不重复时才走」，容易写反
  · 不需要哨兵 —— 头节点永远会被保留，这与 203 题不同
  · 要同时确认 cur 与 cur.next 存在，才能拿到下一个值来比较

Rust 的一处写法细节：
  · 沿 &mut ListNode 逐跳时，每次前进都要重新借用 next；把「读下一个值」
    与「跨过节点」分成两步做，可以避免同时持有两个可变借用
  · 被跨过的 Box 直接丢弃，内存自动回收

以示例 2（head = [1,1,2,3,3]）为例：
  cur = 1：next 也是 1 → 跨过 → [1,2,3,3]（cur 仍是第 1 个 1）
  cur = 1：next 是 2 → 不同，cur 前进到 2
  cur = 2：next 是 3 → 不同，cur 前进到 3
  cur = 3：next 也是 3 → 跨过 → [1,2,3]
  cur = 3：next 为空 → 结束

复杂度分析：
  时间 O(n)：每个节点最多比较一次、被跨越一次
  空间 O(1)：只用一条指针
*/
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur: Option<&mut Box<ListNode>> = head.as_mut();

        // 需要拿到 next 才能比较，所以两个条件都要判
        // 用 take 把当前借用取出，避免「上一轮借用未结束」的移动错误
        while let Some(node) = cur.take() {
            // 先取出「下一个节点的值」，用完立即释放借用
            let next_val = match node.next {
                Some(ref next) => next.val,
                None => break,
            };

            if node.val == next_val {
                // 跨过重复节点；node 本身不动，可能后面还有相同的值
                let removed = node.next.take();
                node.next = removed.and_then(|n| n.next);
                cur = Some(node);
            } else {
                // 值不同才前进
                cur = node.next.as_mut();
            }
        }

        head
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 由切片构造链表，方便写断言
fn build(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

// 按链表顺序展开，用于和期望序列比较
fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        out.push(node.val);
        cur = node.next.as_ref();
    }
    out
}

// 一个用例 = 输入 + 期望输出
fn check(name: &str, values: &[i32], want: &[i32]) {
    let got = to_vec(&Solution::delete_duplicates(build(values)));
    assert_eq!(got, want, "{}: got {:?}", name, got);
}

fn main() {
    // 示例 1：一对重复
    check("example 1", &[1, 1, 2], &[1, 2]);

    // 示例 2：两组重复，验证跨过之后 cur 不动
    check("example 2", &[1, 1, 2, 3, 3], &[1, 2, 3]);

    // 边界：空链表，不能解引用
    check("empty", &[], &[]);

    // 边界：只有一个节点，next 为空，循环一次都不进
    check("single node", &[1], &[1]);

    // 边界：全部相同，连续跨越三次
    check("all equal", &[1, 1, 1, 1], &[1]);

    // 边界：没有重复，指针一路前进，链表不应被改动
    check("all distinct", &[1, 2, 3], &[1, 2, 3]);

    println!("all tests passed");
}
