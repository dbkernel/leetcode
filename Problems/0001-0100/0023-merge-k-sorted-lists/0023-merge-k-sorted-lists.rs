/*
============================================================================
23. 合并 K 个升序链表（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/merge-k-sorted-lists/
分类：堆 · K 路归并（分治 / 优先队列）
============================================================================

给你一个链表数组，每个链表都已经按升序排列。

请你将所有链表合并到一个升序链表中，返回合并后的链表。

示例 1：
输入：lists = [[1,4,5],[1,3,4],[2,6]]
输出：[1,1,2,3,4,4,5,6]
解释：链表数组如下：
[
  1->4->5,
  1->3->4,
  2->6
]
将它们合并到一个有序链表中得到。
1->1->2->3->4->4->5->6

示例 2：
输入：lists = []
输出：[]

示例 3：
输入：lists = [[]]
输出：[]

提示：
- k == lists.length
- 0 <= k <= 10^4
- 0 <= lists[i].length <= 500
- -10^4 <= lists[i][j] <= 10^4
- lists[i] 按 升序 排列
- lists[i].length 的总和不超过 10^4
*/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 力扣预置的节点定义；本地用 rustc 单文件编译时需要显式声明
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

// 本题有三种官方解法，按「一解一个结构体」拆成 Solution1 / Solution2 /
// Solution3；提交时把选用的那个改名为 Solution 即可。
struct Solution1;
struct Solution2;
struct Solution3;

// 两条升序链表合并成一条；三个解法都要用它，抽出来避免三份拷贝
fn merge_two(
    mut a: Option<Box<ListNode>>,
    mut b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // 哨兵节点让「结果头」和「后续节点」走同一套赋值逻辑
    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy;

    while a.is_some() && b.is_some() {
        // 只比较值，取出时再 take，避免同时持有两个可变引用
        let take_a = a.as_ref().unwrap().val <= b.as_ref().unwrap().val;
        let mut node = if take_a { a.take().unwrap() } else { b.take().unwrap() };

        // 摘下来的节点还挂着原链表的剩余部分，先还给对应的变量
        if take_a {
            a = node.next.take();
        } else {
            b = node.next.take();
        }

        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }

    // 剩下那段整体有序，直接整段接上，不必逐节点搬运
    tail.next = if a.is_some() { a } else { b };
    dummy.next
}

/*
解法一：顺序合并（拿已合并的长链去接下一个）【非最优】

为什么慢：先把第 1、2 条合并成结果，再拿这个结果去和第 3 条合并，依此类推。
第 i 次合并要把当时的结果链完整走一遍，长度约为 i * n，于是总代价是
n(1 + 2 + ... + k) = O(k^2 n)。慢的地方很具体：已经合并好的长链被反复重扫。

换个角度：合并两条链的代价正比于「两条链长度之和」，所以要减少每条链参与
合并的次数。把 k 条链两两配对，每轮合并的总长度仍然是 n，轮数却是 log k。

关键点：
  · 合并两条链必须用哨兵节点，否则「结果头」要单独判一次空
  · 尾部接剩余段时整段接上，不要再逐节点搬运
  · 这里保留顺序合并，是为了给另外两个解法的复杂度对比一个基准
  · Rust 里 lists 是 Vec<Option<Box<ListNode>>>，用 for 逐个 move 出来，
    不需要 clone 节点

以示例 1（lists = [[1,4,5],[1,3,4],[2,6]]）为例：
  merge([1,4,5], [1,3,4]) → [1,1,3,4,4,5]
  merge([1,1,3,4,4,5], [2,6]) → [1,1,2,3,4,4,5,6]
  第 2 次合并把长度为 6 的结果链又完整扫了一遍

复杂度分析：
  时间 O(k^2 n)：第 i 次合并要走 i * n 个节点
  空间 O(1)：只改指针，不新建节点
*/
impl Solution1 {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut merged: Option<Box<ListNode>> = None;
        for list in lists {
            merged = merge_two(merged, list);
        }
        merged
    }
}

/*
解法二：分治两两合并【最优解】

为什么慢：解法一的浪费不在「合并」这个动作，而在合并的顺序 —— 第 1 条链
被扫了 k 次。只要让每条链参与的合并次数降到 log k 次，总代价就降到
O(n log k)，而这个量级已经是最优的：每个节点至少要读一次，k 条链归并的
比较次数下界就是 Ω(n log k)。

换个角度：把 lists 看成待归并的序列，对半切开、左右各自合并成一条，再合并
这两条。每一层里所有链加起来恰好是 n 个节点，层数是 log k。

关键点：
  · 区间用「切片」表示而不用下标，split_at_mut 直接把所有权切成两半，
    省掉按下标取元素时的 borrow 纠缠
  · len == 1 就是最小切口，用 take() 把节点移出切片，只此一次移动
  · 先判 lists 为空：空切片取不出最小切口，split_at_mut 也会死循环
  · 合并只在递归返回时做，不能先建结果再递归
  · 递归深度是 log2(k) ≤ 14，不会压爆栈

以示例 1（lists = [[1,4,5],[1,3,4],[2,6]]）为例：
  3 条链切成 [1,4,5] 与 [[1,3,4],[2,6]]
  右半边再切成 [1,3,4] 与 [2,6]，合并成 [1,2,3,4,6]
  最后 merge([1,4,5], [1,2,3,4,6]) → [1,1,2,3,4,4,5,6] ✔

复杂度分析：
  时间 O(n log k)：每层合并的总长度是 n，共 log k 层
  空间 O(log k)：递归栈深度；结果链表复用原节点，不额外分配
*/
impl Solution2 {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        merge_range(&mut lists)
    }
}

// 合并整个切片里的链表，切片至少有一个元素
fn merge_range(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
    // 只剩一条链时直接返回，这是分治的唯一出口
    if lists.len() == 1 {
        return lists[0].take();
    }
    let mid = lists.len() / 2;
    let (left, right) = lists.split_at_mut(mid);
    merge_two(merge_range(left), merge_range(right))
}

/*
解法三：优先队列（K 路归并）【最优解，与解法二并列】

为什么慢：若要「每次取出全局最小」，最朴素的写法是每轮扫一遍 k 个头，
O(k) 一次，总共 O(nk)。慢在「k 个头里挑最小」被重复做了 n 次却没有任何
信息被复用。

换个角度：维护一个只装「每条链当前头节点」的小根堆，堆顶就是全局最小。
取走堆顶后，把它的后继补进堆 —— 堆里始终只有 k 个候选，取最小只需 O(log k)。

它与解法二时间同为 O(n log k)，是并列最优：分治不额外占空间、实现更短；
堆只需要 k 个额外指针，且天然支持「链表一条条到达」的流式输入。

关键点：
  · 堆里放的是「每条链尚未接上的头」，不是整条链 —— 否则堆会膨胀到 n
  · 出堆之后才把 node.next 入堆：入堆的一定还没被接过
  · 走完的链不要再入堆，堆为空的时刻就是合并完成的时刻
  · BinaryHeap 默认是大根堆，用 Reverse 包一层才变成小根堆；
    ListNode 没有实现 Ord，所以堆里存 (值, 链表下标)，出堆后按
    下标去 heads 里取节点 —— 同一链表同一时刻只有一个候选在堆里，
    下标足以定位，不需要把节点本身放进堆
  · 相等时谁先出堆不影响结果，题目只要求整体升序

以示例 1（lists = [[1,4,5],[1,3,4],[2,6]]）为例：
  初始堆 = {1(链1), 1(链2), 2(链3)}
  依次弹出 1、1、2，每次弹出后把该链的下一个节点入堆
  接出的顺序是 1,1,2,3,4,4,5,6 ✔

复杂度分析：
  时间 O(n log k)：每个节点进出堆各一次，每次 O(log k)
  空间 O(k)：堆里最多同时存在 k 个候选节点
*/
impl Solution3 {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heads = lists;
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        // 只放每条链的头，堆的大小因此被 k 卡住
        for (i, head) in heads.iter().enumerate() {
            if let Some(node) = head {
                heap.push(Reverse((node.val, i)));
            }
        }

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while let Some(Reverse((_, i))) = heap.pop() {
            // 堆里存的只是 key，节点仍在 heads[i] 上；取走后立即补后继
            let mut node = heads[i].take().expect("heap key without node");
            let next = node.next.take();
            if let Some(n) = &next {
                heap.push(Reverse((n.val, i)));
            }
            heads[i] = next;

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
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

fn build_lists(lists: &[&[i32]]) -> Vec<Option<Box<ListNode>>> {
    lists.iter().map(|values| build(values)).collect()
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

// 一个用例喂给三个解法；输入节点会被接进结果，所以每个解法各建一份
fn check(lists: &[&[i32]], want: &[i32]) {
    let got1 = to_vec(&Solution1::merge_k_lists(build_lists(lists)));
    let got2 = to_vec(&Solution2::merge_k_lists(build_lists(lists)));
    let got3 = to_vec(&Solution3::merge_k_lists(build_lists(lists)));

    assert_eq!(got1, want, "sequential got {:?}", got1);
    assert_eq!(got2, want, "divide got {:?}", got2);
    assert_eq!(got3, want, "heap got {:?}", got3);
}

fn main() {
    // 示例 1：三条链交错，验证每步都取到全局最小
    check(
        &[&[1, 4, 5], &[1, 3, 4], &[2, 6]],
        &[1, 1, 2, 3, 4, 4, 5, 6],
    );

    // 示例 2：一条链都没有
    check(&[], &[]);

    // 示例 3：只有一个空链表，验证「空输入」与「空元素」两条不同的分支
    check(&[&[]], &[]);

    // 边界：多条空链表与有值链表混在一起
    check(&[&[], &[1], &[], &[2, 3], &[]], &[1, 2, 3]);

    // 边界：单条链，不需要任何合并
    check(&[&[-3, -1, 7]], &[-3, -1, 7]);

    // 边界：负数与相同的值，验证比较用 <= 不会漏节点
    check(&[&[-5, -1], &[-3, 0], &[-5, 0]], &[-5, -5, -3, -1, 0, 0]);

    // 边界：每条链都只有一个节点，退化成对 k 个数的排序
    check(&[&[3], &[1], &[2], &[5], &[4]], &[1, 2, 3, 4, 5]);

    // 规模：20 条链各 20 个节点，k 较大时分治与堆都不能出错
    let mut big: Vec<Vec<i32>> = Vec::new();
    let mut want: Vec<i32> = Vec::new();
    for i in 0..20 {
        let mut cur = Vec::new();
        for j in 0..20 {
            cur.push(i + j * 20);
            want.push(i + j * 20);
        }
        big.push(cur);
    }
    want.sort_unstable();
    let refs: Vec<&[i32]> = big.iter().map(|v| v.as_slice()).collect();
    check(&refs, &want);

    println!("all tests passed");
}
