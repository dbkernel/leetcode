/*
============================================================================
25. K 个一组翻转链表（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-nodes-in-k-group/
分类：链表 · 分组反转（模拟 + 哑节点）

给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。

k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，
那么请将最后剩余的节点保持原有顺序。

你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。

示例 1：
输入：head = [1,2,3,4,5], k = 2
输出：[2,1,4,3,5]

示例 2：
输入：head = [1,2,3,4,5], k = 3
输出：[3,2,1,4,5]

提示：
链表中的节点数目为 n
1 <= k <= n <= 5000
0 <= Node.val <= 1000

进阶：你可以设计一个只用 O(1) 额外内存空间的算法解决这个问题吗？
*/
use std::vec::Vec;

// 平台预置的链表节点定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/*
解法：模拟（哑节点 + 按组反转再拼接）【最优解】

官方题解只给出这一种方法，且它满足「进阶」要求的 O(1) 额外空间，因此是本题
的最优解：时间 O(n)（每个节点恰好被翻转一次），空间 O(1)。

为什么慢：最直观的想法是「先求出长度，把节点按组塞进数组，反转每组再重新
连线」——要多开 O(n) 空间。而直接硬写又会掉进两个坑：第一组前面没有前驱节点；
翻转后整个链表的头节点变了，返回值不好确定。

换个角度：那就「创造」一个前驱 —— 在头节点前接一个哑节点 hair。
  · 有了 hair，第一组也有 pre 可以挂接，不必特判；
  · 无论有没有翻转，hair.next 始终是最终结果的头节点，直接返回它即可。

关键点：
  · 每轮先探路：从 pre 出发走 k 步定为本组的尾节点，若中途走空说明剩余不足
    k 个，直接把剩余链挂回已处理部分的尾部并结束
  · 反转的写法沿用「反转链表」：把 prev 从小段的后继起步，逐个摘下节点头插，
    这样反转完成后尾部自然接上了后面未处理的部分
  · 反转后原来的组头变成组尾、组尾变成组头；组的尾节点需要在反转前就记住
    （Rust 里用裸指针保存，才能以 O(1) 追加下一组，不必为了找尾再遍历一遍）

以示例 1（1 → 2 → 3 → 4 → 5, k = 2）为例：
  第一组探路得尾节点 2，反转 1、2 得 2 → 1，接回得 hair → 2 → 1 → 3...
  第二组探路得尾节点 4，反转 3、4 得 4 → 3，接回 → 2 → 1 → 4 → 3 → 5
  第三组只剩 1 个节点，不足 2 个 → 剩余 5 保持原序
  最终 2 → 1 → 4 → 3 → 5

以示例 2（1 → 2 → 3 → 4 → 5, k = 3）为例：
  第一组尾节点 3，反转得 3 → 2 → 1，接回 → 3 → 2 → 1 → 4 → 5
  第二组只剩 2 个节点，不足 3 个 → 直接结束，结果 3 → 2 → 1 → 4 → 5

复杂度分析：
  时间 O(n)：探路与反转各扫一遍，合计不超过 2n
  空间 O(1)：只用 hair 和几个指针
*/
struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;

        // 哑节点：既充当第一组的前驱，也保证返回值稳定
        let mut hair = Box::new(ListNode::new(0));
        hair.next = head;

        // 已处理部分的尾节点；用裸指针保存，才能以 O(1) 追加下一组
        let mut pre: *mut ListNode = &mut *hair;
        // 尚未处理的链表
        let mut rest: Option<Box<ListNode>> = hair.next.take();

        loop {
            // 1. 探路：剩余节点是否够 k 个
            let mut probe = rest.as_ref();
            let mut cnt = 0;
            while cnt < k {
                match probe {
                    Some(node) => {
                        probe = node.next.as_ref();
                        cnt += 1;
                    }
                    None => break,
                }
            }
            if cnt < k {
                break;
            }

            // 2. 反转前 k 个节点；组头反转后会变成组尾，先把它的地址记下来
            let mut cur = rest.take();
            let group_tail: *mut ListNode = cur.as_mut().map(|n| &mut **n as *mut ListNode).unwrap();
            let mut prev: Option<Box<ListNode>> = None;
            for _ in 0..k {
                let mut node = match cur.take() {
                    Some(node) => node,
                    None => break,
                };
                cur = node.next.take();
                node.next = prev;
                prev = Some(node);
            }

            // 3. 把反转好的这一组挂到已处理部分的尾部
            unsafe { (*pre).next = prev };
            pre = group_tail;
            rest = cur;
        }

        // 4. 不足 k 个的尾巴保持原序
        unsafe { (*pre).next = rest };
        hair.next
    }
}

// 由切片构造链表，方便写断言
fn build(values: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut *dummy;
    for &v in values {
        tail.next = Some(Box::new(ListNode::new(v)));
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

// 按链表顺序展开，用于和期望序列比较
fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    while let Some(node) = head {
        values.push(node.val);
        head = node.next;
    }
    values
}

// 一个用例 = 输入序列 + k + 期望序列
fn check(values: &[i32], k: i32, want: &[i32]) {
    let got = to_vec(Solution::reverse_k_group(build(values), k));
    assert_eq!(got, want, "reverseKGroup({:?}, {}) = {:?}, want {:?}", values, k, got, want);
}

fn main() {
    // 示例 1：k = 2，最后剩一个节点保持原序
    check(&[1, 2, 3, 4, 5], 2, &[2, 1, 4, 3, 5]);

    // 示例 2：k = 3，最后剩两个节点保持原序
    check(&[1, 2, 3, 4, 5], 3, &[3, 2, 1, 4, 5]);

    // 边界：k == 1，每组一个节点，结果应与输入完全相同
    check(&[1, 2, 3], 1, &[1, 2, 3]);

    // 边界：k == n，整条链翻转
    check(&[1, 2, 3, 4, 5], 5, &[5, 4, 3, 2, 1]);

    // 边界：单节点，k 只能是 1
    check(&[7], 1, &[7]);

    // 边界：长度恰好整除 k，最后一组也要被翻转
    check(&[1, 2, 3, 4], 2, &[2, 1, 4, 3]);

    // 边界：长度不足 k，整条链保持原序
    check(&[1, 2], 3, &[1, 2]);

    println!("all tests passed");
}
