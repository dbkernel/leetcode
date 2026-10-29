/*
============================================================================
LeetCode 876. 链表的中间结点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/middle-of-the-linked-list/
分类：链表 · 中间结点（数组 / 单指针 / 快慢指针）
============================================================================

给你单链表的头结点 head ，请你找出并返回链表的中间结点。

如果有两个中间结点，则返回第二个中间结点。

示例 1：
输入：head = [1,2,3,4,5]
输出：[3,4,5]
解释：链表只有一个中间结点，值为 3 。

示例 2：
输入：head = [1,2,3,4,5,6]
输出：[4,5,6]
解释：该链表有两个中间结点，值分别为 3 和 4 ，返回第二个结点。

提示：
链表的结点数范围是 [1, 100]
1 <= Node.val <= 100
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

// 本题有三种官方解法，按「一解一个结构体」拆成 Solution1 / Solution2 / Solution3；
// 提交时把选用的那个改名为 Solution 即可。
struct Solution1;
struct Solution2;
struct Solution3;

/*
解法一：数组（把所有节点存下来，取下标 n / 2）【非最优】

为什么慢：为了「按下标取中间」这个便利，把全部节点地址塞进 Vec，
额外占用 O(n) 空间。时间仍是 O(n)，但空间没有必要地放大了。

换个角度：链表不支持随机访问，那就先用数组补上这个能力 —— 一次遍历后，
中间结点就是 nodes[n / 2]。这是最直白的「先转成数组」写法。

关键点：
  · 下标是 n / 2 而不是 (n - 1) / 2 —— 题目要求两个中间结点时取后一个，
    整数除法向下取整正好满足
  · Vec 里存的是节点地址，取出的那个就是原链表中的结点

以示例 2（head = [1,2,3,4,5,6]）为例：
  n = 6，n / 2 = 3，nodes[3] 是值为 4 的结点 → [4,5,6]

复杂度分析：
  时间 O(n)：遍历一遍收集，再 O(1) 取下标
  空间 O(n)：Vec 存放全部节点地址
*/
impl Solution1 {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 本题只读不写，为了简单起见直接按所有权的形式收集，再按需返回
        let mut nodes: Vec<Box<ListNode>> = Vec::new();
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();
            nodes.push(node);
        }

        // 把后缀整体接回第 n/2 个结点上，得到从中间开始的链表
        let mid = nodes.len() / 2;
        let mut result: Option<Box<ListNode>> = None;
        while nodes.len() > mid {
            let mut node = nodes.pop().unwrap();
            node.next = result;
            result = Some(node);
        }
        result
    }
}

/*
解法二：单指针法（先数长度，再走一半）【非最优】

为什么慢：空间回到了 O(1)，但为了知道「一半」有多远，必须先完整遍历一遍
数长度，再从头走一遍，共两趟。题目本身只需要一趟就能定位。

换个角度：中间结点是正数第 n / 2 个（0 起算）。既然已经知道长度，
从 head 出发走 n / 2 步就落在它上面。

关键点：
  · 步数是 n / 2，不是 (n - 1) / 2 —— 偶数长度要取后一个中间结点
  · 计数与前进要分开写，不能让同一个指针身兼两职

以示例 2（head = [1,2,3,4,5,6]）为例：
  先数出 n = 6；再从 head 走 6 / 2 = 3 步，落在值为 4 的结点 → [4,5,6]

复杂度分析：
  时间 O(n)：数长度一趟，前进一趟
  空间 O(1)：只用两条指针和一个计数
*/
impl Solution2 {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 数长度：只借用不移动
        let mut n = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            n += 1;
            cur = node.next.as_ref();
        }

        // 从头走 n/2 步
        let mut cur = head;
        for _ in 0..n / 2 {
            cur = cur.and_then(|node| node.next);
        }
        cur
    }
}

/*
解法三：快慢指针（快走两步，慢走一步）【最优解】

官方三解中综合最优的一个，也是本题的最优解：一趟扫描 O(n)，额外空间 O(1)。
解法一多花 O(n) 空间，解法二要扫两趟，这一种两者都省。

为什么慢：前面两种写法都建立在「先知道长度」之上 —— 要么把节点全存下来，
要么先数一遍。可「中间」这个概念其实可以由速度差直接表达。

换个角度：让 fast 每次走两步、slow 每次走一步。fast 走到末尾时，
slow 走过的路程正好是 fast 的一半，也就停在中间。不需要知道 n。

关键点：
  · 循环条件是 fast 非空且 fast.next 非空 ——
    少了后半个判断，偶数长度时就会在 None 上取 next
  · 返回 slow，不是 fast —— fast 停在末尾（或末尾的下一个空位）
  · 偶数长度时 slow 恰好落在后一个中间结点，与题目要求一致

Rust 的一处写法细节：
  · 接口按值交出整条链表的所有权，无法「只返回中间的指针」，必须把所有权
    前移到中间位置。所以这里用快慢指针先求出中间是第几步（只借用，不移动），
    再按这个步数前进；两个阶段都只用常数额外空间

以示例 2（head = [1,2,3,4,5,6]）为例：
  初始 slow = 1，fast = 1
  第 1 轮：slow = 2，fast = 3
  第 2 轮：slow = 3，fast = 5
  第 3 轮：fast.next 是 6，非空 → slow = 4，fast = 空
  此时 fast 为空，停止，slow 停在 4 → [4,5,6]

复杂度分析：
  时间 O(n)：fast 恰好走完全程
  空间 O(1)：只用两条指针
*/
impl Solution3 {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 快慢指针：先只借用，求出中间结点是第几步
        let mut steps = 0;
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        // 两个条件都要判：fast.next 存在才敢再走两步
        while let Some(f) = fast {
            if f.next.is_none() {
                break;
            }
            slow = slow.and_then(|node| node.next.as_ref());
            fast = f.next.as_ref().and_then(|node| node.next.as_ref());
            steps += 1;
        }

        // slow 已指向中间结点，但所有权还在 head 里；按步数把所有权移过去
        let mut rest = head;
        for _ in 0..steps {
            rest = rest.and_then(|node| node.next);
        }
        rest
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 由切片构造链表，方便写断言
fn build_with_len(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

// 从某个结点开始按顺序展开，用于和期望序列比较
fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        out.push(node.val);
        cur = node.next.as_ref();
    }
    out
}

// 一个用例喂给三种解法：三种写法必须指向同一个中间结点
fn check(name: &str, values: &[i32], want: &[i32]) {
    // 每种解法都会消耗输入，各用一份新建的链表
    let got1 = to_vec(&Solution1::middle_node(build_with_len(values)));
    let got2 = to_vec(&Solution2::middle_node(build_with_len(values)));
    let got3 = to_vec(&Solution3::middle_node(build_with_len(values)));

    assert_eq!(got1, want, "{}: solution1 got {:?}", name, got1);
    assert_eq!(got2, want, "{}: solution2 got {:?}", name, got2);
    assert_eq!(got3, want, "{}: solution3 got {:?}", name, got3);
}

fn main() {
    // 示例 1：奇数长度，中间结点唯一
    check("example 1", &[1, 2, 3, 4, 5], &[3, 4, 5]);

    // 示例 2：偶数长度，必须返回后一个中间结点
    check("example 2", &[1, 2, 3, 4, 5, 6], &[4, 5, 6]);

    // 边界：只有一个结点，slow 就是 head，循环一次都不进
    check("single node", &[1], &[1]);

    // 边界：两个结点，返回第二个结点，专门验证 n / 2 的取整方向
    check("two nodes", &[1, 2], &[2]);

    println!("all tests passed");
}
