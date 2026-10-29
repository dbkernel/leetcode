/*
============================================================================
LeetCode 2. 两数相加（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/add-two-numbers/
分类：链表 · 竖式加法（一次遍历）
============================================================================

给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的
方式存储的，并且每个节点只能存储 一位 数字。

请你将两个数相加，并以相同形式返回一个表示和的链表。

你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例 1：
输入：l1 = [2,4,3], l2 = [5,6,4]
输出：[7,0,8]
解释：342 + 465 = 807.

示例 2：
输入：l1 = [0], l2 = [0]
输出：[0]

示例 3：
输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
输出：[8,9,9,9,0,0,0,1]

提示：
每个链表中的节点数在范围 [1, 100] 内
0 <= Node.val <= 9
题目数据保证列表表示的数字不含前导零
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

struct Solution;

/*
解法：模拟（竖式加法，一次遍历）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：结果的每一位都必须
被写出来，答案链表长度是 O(max(m,n))，因此 O(max(m,n)) 的时间已经到底。

为什么慢：先把两条链表还原成整数、相加、再拆回链表，看起来只扫两遍，
但节点数最多 100 位，远超 i64；要么改用大整数或字符串加法，要么在转换
中反复处理进位，代码和复杂度都不划算。真正需要的信息只有「同一位置的
两位数字 + 来自低位的进位」。

换个角度：链表按逆序存储，表头就是个位，两条链表同一下标天然对齐到
同一位。于是把竖式加法直接搬到链表上：边走边算 n1 + n2 + carry，
当前位留下 sum % 10，进位 sum / 10 留给下一位。

关键点：
  · 进位必须参与当前位求和 —— 漏掉它，5 + 5 会得到 0 而不是 0 进 1
  · 循环条件用 l1.is_some() || l2.is_some()，短链表按 0 参与
  · 循环结束后 carry > 0 要补一个节点 —— 否则 [9,9] + [1] 会少最高位

Rust 的两处写法差异：
  · 输入是 Option<Box<ListNode>>，取当前位用 as_ref().map_or(0, |n| n.val)
  · 用哑节点 dummy 持有表头，tail: &mut ListNode 沿 next 前进；
    否则「第一个节点」和「后续节点」要写两套赋值逻辑

以示例 1（l1 = [2,4,3], l2 = [5,6,4]，即 342 + 465）为例：
  个位：3 + 4 + 0 = 7，carry = 0 → 答案第 1 位 7
  十位：4 + 6 + 0 = 10，carry = 1 → 答案第 2 位 0
  百位：2 + 5 + 1 = 8，carry = 0 → 答案第 3 位 8
  结果 7 → 0 → 8，即 807

复杂度分析：
  时间 O(max(m,n))：两条链表各走一遍，每位只做常数次运算
  空间 O(1)：除返回值外只用了 dummy / carry 等常数个变量
             （返回值本身占 O(max(m,n))，按官方口径不计入）
*/
impl Solution {
    // 返回一条新链表，表示 l1 与 l2 所代表整数之和，不修改输入链表
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        // 哑节点：让「追加到表尾」不必区分第一条边和后续边
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        // 任一链表还有位就继续：短链表缺的位按 0 处理
        while l1.is_some() || l2.is_some() {
            let n1 = l1.as_ref().map_or(0, |node| node.val);
            let n2 = l2.as_ref().map_or(0, |node| node.val);

            // 进位来自低位，必须和当前两位一起相加
            let sum = n1 + n2 + carry;

            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();

            // 留给下一位；sum < 20，所以 carry 只可能是 0 或 1
            carry = sum / 10;

            // and_then 把 Option 的「有值才继续」和取 next 合成一步
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        // 最高位仍可能产生进位，例如 [9,9] + [1] = [0,0,1]
        if carry > 0 {
            tail.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy.next
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 由切片构造链表：从最高位开始插到表头，正好还原成逆序存储
fn build(digits: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &d in digits.iter().rev() {
        head = Some(Box::new(ListNode { val: d, next: head }));
    }
    head
}

// 按题目约定的顺序展开链表：表头是个位，表尾是最高位
fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        out.push(node.val);
        cur = node.next.as_ref();
    }
    out
}

// 一个用例 = 两组输入 + 一组期望输出
fn check(name: &str, a: &[i32], b: &[i32], want: &[i32]) {
    let got = to_vec(&Solution::add_two_numbers(build(a), build(b)));
    assert_eq!(got, want, "{}: got {:?}, want {:?}", name, got, want);
}

fn main() {
    // 示例 1：342 + 465 = 807，逐位无跨位进位
    check("example 1", &[2, 4, 3], &[5, 6, 4], &[7, 0, 8]);

    // 示例 2：两个 0 相加，答案必须仍是单节点 [0]
    check("example 2", &[0], &[0], &[0]);

    // 示例 3：答案比两个输入都长，专门验证末尾补进位
    check(
        "example 3",
        &[9, 9, 9, 9, 9, 9, 9],
        &[9, 9, 9, 9],
        &[8, 9, 9, 9, 0, 0, 0, 1],
    );

    // 边界：5 + 5 = 10，最短的「进位导致多一位」
    check("carry grows length", &[5], &[5], &[0, 1]);

    // 边界：短链表先走完，长链表剩余位按 0 继续加
    check("uneven length", &[1], &[9, 9], &[0, 0, 1]);

    println!("all tests passed");
}
