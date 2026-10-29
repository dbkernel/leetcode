/*
============================================================================
234. 回文链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/palindrome-linked-list/
分类：链表 · 回文判断（复制到数组 / 递归 / 快慢指针 + 反转后半）

给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回
true ；否则，返回 false 。

示例 1：
输入：head = [1,2,2,1]
输出：true

示例 2：
输入：head = [1,2]
输出：false

提示：
链表中节点数目在范围 [1, 10^5] 内
0 <= Node.val <= 9

进阶：你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
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

// 本题有三种官方解法，按「一解一个结构体」拆成 Solution1 ~ Solution3；
// 提交时把选用的那个改名为 Solution 即可。
struct Solution1;
struct Solution2;
struct Solution3;

/*
解法一：把值复制到数组，再用双指针夹逼【非最优】

为什么慢：链表只能单向访问，没法像数组那样从尾部回看，所以先把值抄进数组，
问题立刻退化成「数组是否回文」。代价是 O(n) 的额外空间，不满足题目「进阶」
要求的 O(1) 空间。

换个角度：判断回文需要「同时从两端向中间走」。数组给了随机访问，天然满足；
链表没这个能力，所以要么先把它「数组化」，要么把后半段物理翻转。

关键点：
  · 复制的是 val，不是节点 —— 比较节点身份永远不相等
  · 双指针用 i < j 而不是 i <= j，奇数长度时中间的节点不需要比较
  · 空链表或单节点直接为真（比较循环一次都不执行）

以示例 1（[1,2,2,1]）为例：
  数组为 [1,2,2,1]，i=0/j=3 相等，i=1/j=2 相等 → 返回 true

复杂度分析：
  时间 O(n)：复制一遍 + 比较半遍
  空间 O(n)：值数组
*/
impl Solution1 {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vals: Vec<i32> = Vec::new();
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            vals.push(node.val);
            cur = node.next.as_ref();
        }

        let n = vals.len();
        for i in 0..n / 2 {
            if vals[i] != vals[n - 1 - i] {
                return false;
            }
        }

        true
    }
}

/*
解法二：递归（借调用栈实现「反向遍历」）【非最优】

为什么慢：递归确实能让我们「先走到尾节点，再在回溯时从后往前比较」，看起来
很省事；但这份便利是用调用栈换来的 —— 空间仍是 O(n)，链长上限 10^5 时
还有爆栈风险。

换个角度：回文比较本质是「第 i 个与第 n-1-i 个相等」。递归正好提供了
「反向取节点」的能力：递归函数先处理 next（向尾部前进），回溯时再拿当前节点
和外部的前向指针 front 比对，然后让 front 前进一步。

关键点：
  · front 必须放在递归函数外面（这里用 &mut 参数一路传下去），
    否则每次回溯拿不到同一份进度
  · 比较要在递归调用「之后」做 —— 放在之前就变成正序和自己比了
  · 一旦子问题返回 false 要立刻短路，别继续推进 front
  · 全程只借引用，不需要克隆节点

以示例 2（[1,2]）为例：
  递归到 2 的回溯，front 指向 1，2 != 1 → 返回 false

复杂度分析：
  时间 O(n)：每个节点参与一次比较
  空间 O(n)：递归栈深度等于链长
*/
impl Solution2 {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // front 借用整条链（生命周期与 head 一致），随回溯逐格前进
        let mut front = head.as_ref();
        Self::check(head.as_ref(), &mut front)
    }

    fn check<'a>(
        cur: Option<&'a Box<ListNode>>,
        front: &mut Option<&'a Box<ListNode>>,
    ) -> bool {
        match cur {
            None => true,
            Some(node) => {
                if !Self::check(node.next.as_ref(), front) {
                    return false;
                }
                let expected = match *front {
                    Some(f) => f,
                    // 递归层数不可能多于节点数，这里不会发生
                    None => return false,
                };
                if expected.val != node.val {
                    return false;
                }
                // 先把下一格取出来再写回，避免同时持有两份可变借用
                *front = expected.next.as_ref();
                true
            }
        }
    }
}

/*
解法三：快慢指针找中点 + 反转后半段【最优解】

满足题目「进阶」的 O(n) 时间、O(1) 空间，是三个官方解中最优的一个。

为什么慢：解法一把值抄进数组，空间 O(n)；解法二用调用栈反向遍历，空间也是
O(n)。想在 O(1) 里完成比较，就得让「从后往前」这件事不再依赖额外容器。

换个角度：回文的后半段读起来正是前半段的镜像。把链表在中点处切成两段、就地
翻转后半段，再从两段的头同时向后走，逐个比较即可 —— 空间只用了几个指针。

关键点：
  · 找中点时，慢指针走 ceil(n/2) 步；这一段留在前半段里
  · 反转的是「中点之后」那一段；奇数长度时正中间的节点归前半段，
    因此后半段永远不比前半段长，比较循环以后半段走完为终止条件
  · 空链表与单节点要提前返回

Rust 的一处写法细节：
  · 官方在比较结束后会把后半段再翻一次接回去。Rust 里 Box 所有权唯一，
    把两段重新接回需要额外遍历持有尾节点的可变引用，收益很低；
    本题只要求返回判断结果（判题不检查输入是否被还原），因此这里省略复原。
    如需复原，把 second 再反转一次、接到 first 的尾节点即可，复杂度不变

以示例 1（1 → 2 → 2 → 1）为例：
  长度 4，前半段取 2 个节点（1 → 2），后半段是 2 → 1
  反转后半段得到 1 → 2
  逐对比较 1==1、2==2，全部相等 → true

复杂度分析：
  时间 O(n)：找中点 + 反转 + 比较，合计线性
  空间 O(1)：只用几个指针（不计节点本身）
*/
impl Solution3 {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let n = Self::length(&head);
        if n <= 1 {
            return true;
        }

        // 1. 在中点处切成两段：前半段含 ceil(n/2) 个节点
        let (first, second) = Self::split(head, (n + 1) / 2);
        // 2. 就地反转后半段
        let second = Self::reverse(second);

        // 3. 从头同时向后走，逐对比较
        let mut p1 = first.as_ref();
        let mut p2 = second.as_ref();
        while let Some(b) = p2 {
            match p1 {
                Some(a) if a.val == b.val => {
                    p1 = a.next.as_ref();
                    p2 = b.next.as_ref();
                }
                // 前半段不应提前走完，这里一并当作不匹配处理
                _ => return false,
            }
        }

        true
    }

    fn length(head: &Option<Box<ListNode>>) -> usize {
        let mut n = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            n += 1;
            cur = node.next.as_ref();
        }
        n
    }

    // 把链表的前 k 个节点与其余部分拆开；k 超过长度时后半段为空
    fn split(
        mut head: Option<Box<ListNode>>,
        k: usize,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let mut rest = head.take();
        let mut taken = 0;

        while taken < k {
            match rest {
                Some(mut node) => {
                    rest = node.next.take();
                    tail.next = Some(node);
                    tail = tail.next.as_mut().unwrap();
                    taken += 1;
                }
                None => break,
            }
        }

        (dummy.next, rest)
    }

    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
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

// 按链表顺序展开，用于核对结果
fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        out.push(node.val);
        cur = node.next.as_ref();
    }
    out
}

// 一个用例喂给三种解法
fn check(name: &str, values: &[i32], want: bool) {
    let got1 = Solution1::is_palindrome(build(values));
    let got2 = Solution2::is_palindrome(build(values));
    let got3 = Solution3::is_palindrome(build(values));

    assert_eq!(got1, want, "{}: by_array got {}", name, got1);
    assert_eq!(got2, want, "{}: by_recursion got {}", name, got2);
    assert_eq!(got3, want, "{}: two_pointers got {}", name, got3);
}

fn main() {
    // 示例 1：偶数长度回文
    check("example 1", &[1, 2, 2, 1], true);

    // 示例 2：两个不同值，必须在第一对就返回 false
    check("example 2", &[1, 2], false);

    // 边界：单节点，天然回文（解法三需提前返回）
    check("single node", &[7], true);

    // 边界：奇数长度回文，正中间的节点不参与比较
    check("odd palindrome", &[1, 2, 1], true);

    // 边界：奇数长度非回文
    check("odd non-palindrome", &[1, 2, 3], false);

    // 边界：全部相同
    check("all same", &[1, 1, 1, 1], true);

    // 边界：只有最外层一对不同，验证比较从首尾开始而不是从中间开始
    check("outer pair differs", &[9, 2, 3, 3, 2, 1], false);

    // 边界：前半段相同而后半段相反，专防「只比了前半段」
    check("latter half differs", &[1, 2, 3, 2, 2], false);

    // 顺带确认辅助函数没把链表改坏
    let head = build(&[1, 2, 3]);
    assert_eq!(to_vec(&head), vec![1, 2, 3]);

    println!("all tests passed");
}
