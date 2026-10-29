/*
============================================================================
328. 奇偶链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/odd-even-linked-list/
分类：链表 · 按位置分组（分离后拼接）

给定单链表的头节点 head ，将所有索引为奇数的节点和索引为偶数的节点分别
分组，保持它们原有的相对顺序，然后把偶数索引节点分组连接到奇数索引节点分组
之后，返回重新排序的链表。

第一个节点的索引被认为是 奇数 ， 第二个节点的索引为 偶数 ，以此类推。

请注意，偶数组和奇数组内部的相对顺序应该与输入时保持一致。

你必须在 O(1) 的额外空间复杂度和 O(n) 的时间复杂度下解决这个问题。

示例 1:
输入: head = [1,2,3,4,5]
输出: [1,3,5,2,4]

示例 2:
输入: head = [2,1,3,5,6,4,7]
输出: [2,3,6,7,1,5,4]

提示:
n == 链表中的节点数
0 <= n <= 10^4
-10^6 <= Node.val <= 10^6
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
解法：分离节点后合并（奇链表 + 偶链表一次成型）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：题目直接要求 O(n) 时间
与 O(1) 额外空间，本解一次遍历、只用几个指针，正好满足。

为什么慢：最直观的想法是「把值按奇偶位置抄成两个数组，再依次写回链表」——
时间 O(n)，但要多开 O(n) 空间，不满足题目要求；而且要改值而不是改链，
节点身份也随之改变。

换个角度：奇偶位置的节点本来就在一条链上交替出现。只要在遍历时把 next
指针重新接线，让「奇数位节点」连成一条链、「偶数位节点」连成另一条链，
最后把偶链接到奇链尾部即可 —— 一条链一次遍历，不需要任何额外容器。

关键点：
  · 偶链的头必须提前保存（这里就是 even 哑节点后面的第一个节点），
    否则遍历到后面就找不回偶链的起点了
  · 循环到链表走空即分离完毕
  · 最后把偶链整体接到奇链尾部的 next 上
  · 空链表直接返回，不需要特判

Rust 的一处写法细节：
  · 官方写法靠 odd->next = even->next 这类原地接线完成分离，而 Box 所有权
    唯一，在循环里同时持有两个节点的可变引用会被借用检查器拒绝。
    这里改用官方另一篇题解里的「虚拟头结点 + 尾插法」：两个哑节点分别攒出
    奇链和偶链，再把偶链挂到奇链尾部。同样只遍历一次、额外空间 O(1)
    （两个栈上的哑节点），分离与拼接的效果和官方写法完全一致

以示例 1（1 → 2 → 3 → 4 → 5）为例：
  位次 1、3、5 依次尾插到奇链 → 1 → 3 → 5
  位次 2、4 依次尾插到偶链 → 2 → 4
  偶链接到奇链尾部 → 1 → 3 → 5 → 2 → 4

以示例 2（2 → 1 → 3 → 5 → 6 → 4 → 7）为例：
  奇链为 2 → 3 → 6 → 7，偶链为 1 → 5 → 4
  拼接后 → 2 → 3 → 6 → 7 → 1 → 5 → 4

复杂度分析：
  时间 O(n)：每个节点只被拆下并尾插一次
  空间 O(1)：只用两个哑节点和几个指针
*/
impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 两条链各用一个哑节点攒节点，避免处理「第一个节点」的特例
        let mut odd_dummy = ListNode::new(0);
        let mut even_dummy = ListNode::new(0);
        let mut odd_tail = &mut odd_dummy;
        let mut even_tail = &mut even_dummy;

        let mut take_odd = true;
        let mut curr = head.take();

        while let Some(mut node) = curr {
            // 先把后继摘下来，再决定这个节点接到哪条链上
            curr = node.next.take();

            if take_odd {
                odd_tail.next = Some(node);
                odd_tail = odd_tail.next.as_mut().unwrap();
            } else {
                even_tail.next = Some(node);
                even_tail = even_tail.next.as_mut().unwrap();
            }

            take_odd = !take_odd;
        }

        // 偶链整体接到奇链尾部；奇链一定非空，除非输入为空
        odd_tail.next = even_dummy.next;

        odd_dummy.next
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

// 一个用例 = 输入序列 + 期望序列
fn check(name: &str, values: &[i32], want: &[i32]) {
    let got = to_vec(&Solution::odd_even_list(build(values)));
    assert_eq!(got, want, "{}: got {:?}", name, got);
}

fn main() {
    // 示例 1：奇数长度，最后一个节点落在奇链上
    check("example 1", &[1, 2, 3, 4, 5], &[1, 3, 5, 2, 4]);

    // 示例 2：奇数长度且值无规律，专防「按值排序」的误写
    check("example 2", &[2, 1, 3, 5, 6, 4, 7], &[2, 3, 6, 7, 1, 5, 4]);

    // 边界：空链表，必须直接返回 None
    check("empty", &[], &[]);

    // 边界：单节点，偶链为空，拼接时不能解引用空指针
    check("single node", &[1], &[1]);

    // 边界：两个节点，偶链只有一个节点
    check("two nodes", &[1, 2], &[1, 2]);

    // 边界：偶数长度，最后一个节点落在偶链上
    check("even length", &[1, 2, 3, 4], &[1, 3, 2, 4]);

    // 边界：三节点，验证最后一步拼接
    check("three nodes", &[1, 2, 3], &[1, 3, 2]);

    println!("all tests passed");
}
