/*
============================================================================
LeetCode 203. 移除链表元素（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-linked-list-elements/
分类：链表 · 删除指定值（递归 / 迭代）
============================================================================

给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足
Node.val == val 的节点，并返回 新的头节点 。

示例 1：
输入：head = [1,2,6,3,4,5,6], val = 6
输出：[1,2,3,4,5]

示例 2：
输入：head = [], val = 1
输出：[]

示例 3：
输入：head = [7,7,7,7], val = 7
输出：[]

提示：
列表中的节点数目在范围 [0, 10^4] 内
1 <= Node.val <= 50
0 <= val <= 50
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
解法一：递归（先处理后面的，再决定自己留不留）【非最优】

为什么慢：思路不慢，时间仍是 O(n)；慢在空间 —— 递归深度等于链表长度，
最坏 10^4 层，有爆栈风险。迭代写法不需要任何额外栈空间。

换个角度：整条链「删掉 val 之后长什么样」，等于「我先看后面删完的结果，
再判断我自己要不要留下」。问题结构相同、规模小一，可以直接递归。

关键点：
  · 递归出口是空节点 —— 空链表删完还是空
  · 先递归再判断自己 —— 顺序反了会丢掉后续的处理结果
  · 自己等于 val 时直接返回子问题的结果（等于把自己摘掉）

Rust 的一处写法细节：
  · Option 按值传入按值返回，天然就是「拿走 → 处理 → 还回」的语义，
    不需要 take/替换之类的技巧

以示例 1（head = [1,2,6,3,4,5,6], val = 6）为例：
  递归到最深处返回空，回溯时 6 被丢弃、5 保留、4 保留 …… 依此类推
  最终得到 [1,2,3,4,5]

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(n)：递归深度等于链表长度
*/
impl Solution1 {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // 空链表删完还是空
        let mut node = match head {
            None => return None,
            Some(node) => node,
        };

        // 先让后面的部分删干净
        node.next = Self::remove_elements(node.next.take(), val);

        // 再决定自己留不留：等于 val 就把自己摘掉，只还回后面那段
        if node.val == val {
            node.next
        } else {
            Some(node)
        }
    }
}

/*
解法二：迭代（哨兵节点 + 一个前驱指针）【最优解】

官方两解中空间最优的一个，也是本题的最优解：时间 O(n) 已是下界，
空间 O(1)，不占用调用栈，长链表也不会爆栈。

为什么慢：解法一的额外开销全在调用栈上。改成循环就能把这份空间省掉。

换个角度：遍历时始终握着「前一个节点」，看到下一个节点该删，就把它跨过去。
麻烦的只是头节点没有前驱 —— 加一个哨兵就统一了。

关键点：
  · 必须用哨兵 —— 否则「头节点等于 val」要单独写一段循环处理，
    连续多个头节点都要删时容易漏
  · 只有「没删」时才移动 prev —— 删完之后 prev 还站在原处，
    下一个节点可能也要删（如 [7,7,7]）
  · 循环要一路走到链表末尾，必须检查到最后一个节点本身

Rust 的一处写法细节：
  · 用两个裸指针 prev/cur 在同一链表上读写，避免借用检查器争用；
    所有解引用都集中在一个 unsafe 块里，作用域清晰

以示例 1（head = [1,2,6,3,4,5,6], val = 6）为例：
  哨兵指向 1，prev 停在哨兵
  1 不等于 6 → prev 移到 1；2 不等于 6 → prev 移到 2
  6 等于 6 → 把 2 的 next 指向 3，prev 仍停在 2
  继续扫过 3、4、5，最后 5 的 next 跳过末尾的 6 → [1,2,3,4,5]

复杂度分析：
  时间 O(n)：每个节点最多被检查一次
  空间 O(1)：只用哨兵与两条指针
*/
impl Solution2 {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        // SAFETY: prev、cur 都指向 dummy 内部的节点，作用域内不与其它借用重叠
        unsafe {
            let mut prev: *mut ListNode = &mut *dummy;
            let mut cur: *mut ListNode = (*prev).next.as_mut().map_or(
                std::ptr::null_mut(),
                |node| node.as_mut() as *mut ListNode,
            );

            while !cur.is_null() {
                if (*cur).val == val {
                    // 跨过待删节点；prev 不动，下一个节点可能也要删
                    let mut removed = (*prev).next.take();
                    (*prev).next = removed.as_mut().unwrap().next.take();
                } else {
                    prev = cur;
                }
                cur = (*prev).next.as_mut().map_or(
                    std::ptr::null_mut(),
                    |node| node.as_mut() as *mut ListNode,
                );
            }
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

// 一个用例喂给两种解法：递归与迭代必须得到同一条链表
fn check(name: &str, values: &[i32], val: i32, want: &[i32]) {
    // 解法会复用输入节点，各用一份新建的链表
    let got1 = to_vec(&Solution1::remove_elements(build(values), val));
    let got2 = to_vec(&Solution2::remove_elements(build(values), val));

    assert_eq!(got1, want, "{}: recursive got {:?}", name, got1);
    assert_eq!(got2, want, "{}: iterative got {:?}", name, got2);
}

fn main() {
    // 示例 1：中间与末尾都有待删节点
    check(
        "example 1",
        &[1, 2, 6, 3, 4, 5, 6],
        6,
        &[1, 2, 3, 4, 5],
    );

    // 示例 2：空链表，不能解引用空指针
    check("example 2", &[], 1, &[]);

    // 示例 3：全部待删，结果必须为空
    check("example 3", &[7, 7, 7, 7], 7, &[]);

    // 边界：头部连续多个待删节点，专防「只删一个头」的写法
    check("leading duplicates", &[6, 6, 1, 6], 6, &[1]);

    // 边界：没有节点需要删除，链表应原样返回
    check("nothing to remove", &[1, 2, 3], 4, &[1, 2, 3]);

    println!("all tests passed");
}
