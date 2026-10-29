/*
============================================================================
148. 排序链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-list/
分类：链表 · 归并排序 / 堆排序

给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。

示例 1：
输入：head = [4,2,1,3]
输出：[1,2,3,4]

示例 2：
输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]

示例 3：
输入：head = []
输出：[]

提示：
- 链表中节点的数目在范围 [0, 5 * 10^4] 内
- -10^5 <= Node.val <= 10^5

进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序
吗？
*/

// 力扣预置的节点定义；本地 rustc 单文件编译时需要显式声明
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

// 本地单文件编译需要显式给出 Solution 类型
struct Solution1;
struct Solution2;
struct Solution3;

/*
解法一：自顶向下归并排序（官方方法一）【非最优】

为什么慢：插入排序在链表上定位插入点要 O(n)，总共 O(n^2)。要达到
O(n log n)，适合链表的是归并排序：合并只改指针，不需要随机访问。

换个角度：先数出长度，从中间切成两半，递归排好左右两半，再合并两个有序
链表。递归深度是 O(log n)，这部分栈空间就是它比解法二多出来的开销。

关键点：
  · 按长度对半切，偶数长度时左半段比右半段多一个节点，两半差不超过 1
  · 切分时走到左半段最后一个节点，take() 它的 next 得到右半段
  · 合并用 dummy 承接，比较用 <= 保持稳定
  · 递归终止条件是空表或单节点

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归调用栈；节点本身复用，没有新建链表

解法二：自底向上归并排序（官方方法二）【最优解】

为什么慢：解法一时间已经最优，但递归栈还要 O(log n)。进阶要求常数级空间，
就差在这层栈上。

换个角度：把「递归切半」换成「按固定步长切段」。sub 从 1 开始，每轮把链表
切成若干长度不超过 sub 的段，两两合并，再让 sub 翻倍，直到超过链表长度。
整个过程只用游标和 take()，没有递归，也没有额外容器。

关键点：
  · 先把链表长度数出来，否则不知道 sub 何时停止
  · 每轮用 tail 记住已合并部分的尾节点，合并完接回去
  · take_first 切段时把段尾的 next 置空，否则 merge 会越过段边界
  · 右半段可能不足 sub，甚至为空；所有推进都要判空
  · 与解法一同为 O(n log n)，但辅助空间从 O(log n) 降到 O(1)，因此最优

复杂度分析：
  时间 O(n log n)：共 O(log n) 轮，每轮合并 O(n)
  空间 O(1)：只用游标，节点原地重连

解法三：堆排序【非最优】

为什么慢：堆排序时间 O(n log n)，与两种归并同阶，本身不慢；慢在空间。
  · 数组上的堆排序可以做到 O(1) 辅助空间，因为下标能直接定位父子节点
  · 链表没有随机访问，必须先遍历一次把节点值收集到 Vec，才能建堆和下沉；
    这一步就是 O(n) 额外空间
  · 排完后还要把值写回原节点，节点身份不变，只有 val 被改写

换个角度：把问题降维成「对一个长为 n 的数组做堆排序」，再写回链表。

关键点：
  · 建堆从最后一个非叶节点 n/2 - 1 开始向前下沉，才是 O(n) 建堆
  · 每轮把堆顶换到末尾，堆大小减一后再从根下沉
  · 下沉时先比较左右孩子取较大者，再和父节点比较；相等不交换
  · 与解法二同为 O(n log n) 时间，但多出 O(n) 数组，故不是最优解

复杂度分析：
  时间 O(n log n)：建堆 O(n)，n 次取堆顶并下沉共 O(n log n)
  空间 O(n)：values 保存全部节点值；链表本身没有额外分配
*/
impl Solution1 {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length = 0;
        {
            let mut p = head.as_ref();
            while let Some(node) = p {
                length += 1;
                p = node.next.as_ref();
            }
        }
        if length < 2 {
            return head;
        }

        let right = {
            let mut cur = head.as_mut().unwrap();
            for _ in 1..length / 2 {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next.take()
        };

        Solution1::merge(
            Solution1::sort_list(head),
            Solution1::sort_list(right),
        )
    }

    fn merge(
        mut a: Option<Box<ListNode>>,
        mut b: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail: &mut ListNode = &mut dummy;

        loop {
            if a.is_none() {
                tail.next = b;
                break;
            }
            if b.is_none() {
                tail.next = a;
                break;
            }

            let take_a = a.as_ref().unwrap().val <= b.as_ref().unwrap().val;
            let mut node = if take_a { a.take() } else { b.take() }.unwrap();
            if take_a {
                a = node.next.take();
            } else {
                b = node.next.take();
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

impl Solution2 {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length = 0;
        {
            let mut p = head.as_ref();
            while let Some(node) = p {
                length += 1;
                p = node.next.as_ref();
            }
        }
        if length < 2 {
            return head;
        }

        let mut list = head;
        let mut sub = 1;
        while sub < length {
            let mut dummy = Box::new(ListNode::new(0));
            // 用裸指针记住结果尾节点：若写成 &mut ListNode，下一轮再给
            // tail.next 赋值时会被借用检查判成与上一轮的重叠借用冲突。
            // 该指针只在单轮内使用，链表始终唯一拥有，不会悬垂。
            let mut tail: *mut ListNode = &mut *dummy;
            let mut rest = list;

            while rest.is_some() {
                let left = take_first(&mut rest, sub);
                let right = take_first(&mut rest, sub);
                let merged = Solution2::merge(left, right);
                unsafe {
                    (*tail).next = merged;
                    let mut p =
                        (*tail).next.as_deref_mut().unwrap() as *mut ListNode;
                    while (*p).next.is_some() {
                        p = (*p).next.as_deref_mut().unwrap() as *mut ListNode;
                    }
                    tail = p;
                }
            }

            list = dummy.next;
            sub <<= 1;
        }
        list
    }

    fn merge(
        mut a: Option<Box<ListNode>>,
        mut b: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail: &mut ListNode = &mut dummy;

        loop {
            if a.is_none() {
                tail.next = b;
                break;
            }
            if b.is_none() {
                tail.next = a;
                break;
            }

            let take_a = a.as_ref().unwrap().val <= b.as_ref().unwrap().val;
            let mut node = if take_a { a.take() } else { b.take() }.unwrap();
            if take_a {
                a = node.next.take();
            } else {
                b = node.next.take();
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

impl Solution3 {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut values = Vec::new();
        {
            let mut p = head.as_ref();
            while let Some(node) = p {
                values.push(node.val);
                p = node.next.as_ref();
            }
        }

        let n = values.len();
        // 从最后一个非叶节点开始下沉，建大根堆是 O(n)
        for i in (0..n / 2).rev() {
            sift_down(&mut values, i, n);
        }

        // 每轮把堆顶换到末尾，缩小堆后重新下沉
        for end in (1..n).rev() {
            values.swap(0, end);
            sift_down(&mut values, 0, end);
        }

        // 节点顺序不变，只把排好的值写回
        let mut p = head.as_mut();
        let mut i = 0;
        while let Some(node) = p {
            node.val = values[i];
            i += 1;
            p = node.next.as_mut();
        }
        head
    }
}

// 从 list 头部切下最多 k 个节点；剩余的仍留在 list 中
fn take_first(
    list: &mut Option<Box<ListNode>>,
    k: usize,
) -> Option<Box<ListNode>> {
    let mut head = list.take();
    if head.is_none() {
        return None;
    }
    {
        let mut cur = head.as_mut().unwrap();
        for _ in 1..k {
            if cur.next.is_none() {
                break;
            }
            cur = cur.next.as_mut().unwrap();
        }
        // 段尾断开，merge 才不会走到下一段
        *list = cur.next.take();
    }
    head
}

fn sift_down(a: &mut [i32], mut i: usize, n: usize) {
    loop {
        let left = 2 * i + 1;
        let right = left + 1;
        let mut largest = i;
        if left < n && a[left] > a[largest] {
            largest = left;
        }
        if right < n && a[right] > a[largest] {
            largest = right;
        }
        if largest == i {
            break;
        }
        a.swap(i, largest);
        i = largest;
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

fn build(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        out.push(node.val);
        cur = node.next.as_ref();
    }
    out
}

fn check(input: &[i32], want: &[i32]) {
    let got1 = to_vec(&Solution1::sort_list(build(input)));
    let got2 = to_vec(&Solution2::sort_list(build(input)));
    let got3 = to_vec(&Solution3::sort_list(build(input)));
    assert_eq!(got1, want, "top-down({input:?}) = {got1:?}");
    assert_eq!(got2, want, "bottom-up({input:?}) = {got2:?}");
    assert_eq!(got3, want, "heap({input:?}) = {got3:?}");
}

fn main() {
    // 示例 1
    check(&[4, 2, 1, 3], &[1, 2, 3, 4]);

    // 示例 2
    check(&[-1, 5, 3, 4, 0], &[-1, 0, 3, 4, 5]);

    // 示例 3：空链表
    check(&[], &[]);

    // 边界：单节点
    check(&[7], &[7]);

    // 边界：已经有序
    check(&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]);

    // 边界：完全逆序
    check(&[5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]);

    // 边界：全部相同，验证合并不会丢掉重复节点
    check(&[3, 3, 3, 3], &[3, 3, 3, 3]);

    // 边界：重复值夹杂在乱序里
    check(&[2, 1, 2, 1, 2], &[1, 1, 2, 2, 2]);

    // 边界：两个节点，验证自底向上时右半段可能为空
    check(&[2, 1], &[1, 2]);

    // 边界：负数与正数混合
    check(&[0, -100000, 100000, -1, 1], &[-100000, -1, 0, 1, 100000]);

    // 边界：奇数长度，验证按长度切分的最后一段不足 sub
    check(&[5, 1, 4, 2, 3], &[1, 2, 3, 4, 5]);

    // 规模用例：1000 个乱序节点，与标准库排序结果对照
    let input: Vec<i32> = (0..1000).map(|i| (i * 37) % 1000 - 500).collect();
    let mut want = input.clone();
    want.sort_unstable();
    check(&input, &want);

    println!("all tests passed");
}
