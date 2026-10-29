/*
============================================================================
LCR 141 / 剑指 Offer 24. 训练计划 III（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/fan-zhuan-lian-biao-lcof/
分类：链表 · 三指针迭代 / 递归

给定一个头节点为 head 的单链表用于记录一系列核心肌群训练编号，请将该
系列训练编号 倒序 记录于链表并返回。

示例 1：
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]

示例 2：
输入：head = [1,2]
输出：[2,1]

示例 3：
输入：head = []
输出：[]

提示：
- 链表中节点的数目范围是 [0, 5000]
- -5000 <= Node.val <= 5000

注意：本题与主站 206 题相同：
https://leetcode.cn/problems/reverse-linked-list/

说明：本题在 leetcode.cn 已改号为 LCR 141，题名由「反转链表」改为
「训练计划 III」，函数签名由 `reverseList(ListNode*)` 改为
`trainningPlan(ListNode*)`（官方 slug 与函数名均保留 trainning 拼写）。
*/
// 力扣预置的节点定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/*
解法一：三指针原地迭代【最优解】

为什么慢：如果新建一条链表做头插，时间仍是 O(n)，但每个节点都要复制
一份，额外空间 O(n)。题目只要求把原来的 next 指针反过来，不需要动节点值。

换个角度：遍历时把当前节点的 next 指向前一个节点。为了断链后还能继续，
需要三个指针：prev（已反转部分的头）、curr（当前节点）、next（尚未处理
部分的头）。

关键点：
  · 必须先保存 curr.next，再改 curr.next；顺序反了，后半条链表就丢了
  · 循环结束时 curr 为 None，prev 才是新链表的头，不能返回 head
  · 空链表或单节点时循环不执行/只执行一次，返回 prev 正好是原头
  · Rust 里用 `next.take()` 把后继从节点中摘下，才能同时持有两边

以 1 -> 2 -> 3 为例：
  初始 prev = None，curr = 1
  存 next = 2，1.next = None，prev = 1，curr = 2
  存 next = 3，2.next = 1，prev = 2，curr = 3
  存 next = None，3.next = 2，prev = 3，curr = None
  返回 prev = 3，链表 3 -> 2 -> 1  ✔

复杂度分析：
  时间 O(n)：每个节点改一次 next 指针
  空间 O(1)：只用了三个指针，没有新建节点
*/
// rustc 单文件编译时需要显式的类型名，平台提交时合并进 Solution
pub struct Solution1;

impl Solution1 {
    pub fn trainning_plan(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut current) = curr {
            // 先记住剩余部分，改完 next 后就找不到它了
            curr = current.next.take();
            current.next = prev;
            prev = Some(current);
        }

        // curr 为 None 时，prev 指向原链表的尾，也就是新头
        prev
    }
}

/*
解法二：递归反转【非最优】

为什么慢：递归写法代码短，但每层调用都要保存当前节点和返回地址，
空间是 O(n) 的调用栈；链表很长时可能爆栈，做不到迭代版本的 O(1)。

换个角度：把链表分成「当前节点」和「剩余部分」，假设剩余部分已经反转，
只要把当前节点接到反转后链表的头部即可。递归一层层把节点摘下，交给
下一层处理，回溯时整条链表就反过来了。

关键点：
  · 递归基是剩余部分为空，此时累加器 acc 就是新头
  · 每层必须先 `next.take()` 把后继摘下，再把 current.next 指向 acc；
    顺序反了会丢掉剩余部分
  · 官方 C++/Go 版写的是 `head.next.next = head`，Rust 的 Box 所有权
    不允许同时借两个节点，这里改用等价的累加器式递归，栈深仍是 O(n)
  · 必须把节点的 next 重新指向 acc，否则原头和新尾会形成环

以 1 -> 2 -> 3 为例：
  摘 1，1.next = None，剩余 2 -> 3，acc = 1
  摘 2，2.next = 1，剩余 3，acc = 2 -> 1
  摘 3，3.next = 2，剩余 None，acc = 3 -> 2 -> 1
  返回 3 -> 2 -> 1  ✔

复杂度分析：
  时间 O(n)：每个节点做常数次指针操作
  空间 O(n)：递归栈深度等于链表长度
*/
pub struct Solution2;

impl Solution2 {
    pub fn trainning_plan(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn reverse(
            node: Option<Box<ListNode>>,
            acc: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match node {
                None => acc,
                Some(mut current) => {
                    // 先把剩余部分摘下来，否则改 next 后就丢了
                    let rest = current.next.take();
                    current.next = acc;
                    reverse(rest, Some(current))
                }
            }
        }

        reverse(head, None)
    }
}

// 按数组顺序建链表，返回头节点；空数组返回 None
fn build_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &value in values.iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }
    head
}

fn to_vec(head: Option<&Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut node = head;
    while let Some(current) = node {
        values.push(current.val);
        node = current.next.as_ref();
    }
    values
}

// 两个解法各自建一条新链表，反转后按顺序收集再比较
fn check(values: &[i32], want: &[i32]) {
    let r1 = Solution1::trainning_plan(build_list(values));
    assert_eq!(to_vec(r1.as_ref()), want, "iterative({:?})", values);

    let r2 = Solution2::trainning_plan(build_list(values));
    assert_eq!(to_vec(r2.as_ref()), want, "recursive({:?})", values);
}

fn main() {
    // 示例 1
    check(&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1]);

    // 示例 2
    check(&[1, 2], &[2, 1]);

    // 示例 3：空链表
    check(&[], &[]);

    // 边界：单节点，反转后还是自己
    check(&[7], &[7]);

    // 边界：负数和零
    check(&[-5000, 0, 5000], &[5000, 0, -5000]);

    // 反直觉：两个节点时，递归版若忘记把 head.next 置空会成环
    check(&[1, 2], &[2, 1]);

    // 边界：最长 5000 个节点，验证递归深度
    let values: Vec<i32> = (1..=5000).collect();
    let want: Vec<i32> = values.iter().rev().copied().collect();
    check(&values, &want);

    println!("all tests passed");
}
