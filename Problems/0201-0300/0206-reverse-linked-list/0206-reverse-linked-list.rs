/*
============================================================================
206. 反转链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-linked-list/
分类：链表 · 指针反转（迭代 / 递归）

给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。

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
链表中节点的数目范围是 [0, 5000]
-5000 <= Node.val <= 5000

进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
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

// 本题有两种官方解法，按「一解一个结构体」拆成 Solution1 / Solution2；
// 提交时把选用的那个改名为 Solution 即可。
struct Solution1;
struct Solution2;

/*
解法一：迭代（逐个把 next 指回前驱）【最优解】

官方两解中空间最优的一个，因此是本题的最优解：时间 O(n) 已是下界
（每个节点的 next 都要改一次），空间 O(1)。

为什么慢：最直接的想法是「把节点值倒着放进数组再写回去」——时间虽然也是
O(n)，但要多开 O(n) 空间，而且题目要求的是改链而不是改值（节点身份会变）。

换个角度：反转的本质就是「让每个节点的 next 指向它的前驱」。难点在于：改掉
next 之后就找不到后继了，所以必须先保存它，再动指针。

关键点：
  · 必须先把 curr.next 存到 next，再改 curr.next，否则后半个链就丢了
  · prev 初值是 None —— 反转头节点的 next 要指向空
  · 循环结束后返回 prev（不是 curr，此时 curr 已经是 None）
  · 空链表不进循环，直接返回 None，行为正确无需特判

Rust 的一处写法细节：
  · 用 take() 把 curr 从「上一轮的 next」里摘出来，所有权一路向后传递；
    node.next 先被 take 空（得到后继），再填成 prev —— 全程没有克隆节点

以示例 1（1 → 2 → 3 → 4 → 5）为例：
  prev=None，curr=1：存 next=2，1.next=None，prev=1，curr=2
  prev=1，curr=2：存 next=3，2.next=1，prev=2，curr=3
  依此推进到 5.next=4，最后 prev=5，curr=None → 返回 5

复杂度分析：
  时间 O(n)：每个节点处理一次
  空间 O(1)：只用 prev / curr 两个 Option（不含节点本身）
*/
impl Solution1 {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            // 先把后继取走，再让当前节点指回 prev
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        // 结束时 curr 为 None，新的头是 prev
        prev
    }
}

/*
解法二：递归（先反转后面，再把自己接到尾巴上）【非最优】

为什么慢：反转次数一样，时间 O(n)；慢在空间 —— 递归深度等于链长，最坏要
n 层栈空间。链长上限 5000，虽然平台能过，但不如迭代稳妥。

换个角度：把「反转整条链」拆成「反转 head 之后的部分」，再把 head 接到反转
结果的末尾。官方 C++ 写法靠 head->next->next = head 直接回指，而 Rust 的
Box 所有权唯一，拿不到「子链尾节点」的可变引用 —— 于是换一种等价的递归表达：
先递归反转剩余部分并把它当作已完成的累加器，再把当前节点头插到累加器前面。
思路与官方一致（都是「反转后面的，再处理当前节点」），递归深度与复杂度也一致。

关键点：
  · 终止条件：链表为空时返回累加器 acc
  · 每层先把 node.next 取空（拿到剩余部分），再把 node 挂到 acc 前面
  · 必须用辅助函数携带 acc —— 否则递归返回后拿不到尾节点
  · 空链表会在一开始就走 None 分支返回 None，不会 panic

以示例 2（1 → 2）为例：
  第 1 层：node = 1，rest = [2]，acc = None → 1.next = None，递归(rest, [1])
  第 2 层：node = 2，rest = []，acc = [1] → 2.next = [1]，递归([], [2,1])
  第 3 层：rest 为空 → 返回 [2,1]

复杂度分析：
  时间 O(n)：每个节点的 next 改一次
  空间 O(n)：递归栈深度等于链长
*/
impl Solution2 {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::prepend_reversed(head, None)
    }

    // 把 head 逐个头插到 acc 前面；head 走空时 acc 就是完整反转结果
    fn prepend_reversed(
        head: Option<Box<ListNode>>,
        acc: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => acc,
            Some(mut node) => {
                // 先取下剩余部分，再把当前节点挂到已反转部分的头部
                let rest = node.next.take();
                node.next = acc;
                Self::prepend_reversed(rest, Some(node))
            }
        }
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

// 一个用例喂给两种解法：迭代与递归必须得到同一条链表
fn check(name: &str, values: &[i32], want: &[i32]) {
    // 两种解法都会改动节点，各用一份新建的链表
    let got1 = to_vec(&Solution1::reverse_list(build(values)));
    let got2 = to_vec(&Solution2::reverse_list(build(values)));

    assert_eq!(got1, want, "{}: iterative got {:?}", name, got1);
    assert_eq!(got2, want, "{}: recursive got {:?}", name, got2);
}

fn main() {
    // 示例 1：五个节点
    check("example 1", &[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1]);

    // 示例 2：两个节点，验证尾节点的 next 被正确置空
    check("example 2", &[1, 2], &[2, 1]);

    // 示例 3：空链表，必须返回 None 而不是崩溃
    check("example 3", &[], &[]);

    // 边界：单节点，递归走终止分支
    check("single node", &[7], &[7]);

    // 边界：含重复值的三段链，验证只改指针不改值
    check("three nodes", &[-1, 0, -1], &[-1, 0, -1]);

    // 边界：严格递增的三节点，防止出现环（反序列化时会死循环）
    check("no cycle", &[1, 2, 3], &[3, 2, 1]);

    println!("all tests passed");
}
