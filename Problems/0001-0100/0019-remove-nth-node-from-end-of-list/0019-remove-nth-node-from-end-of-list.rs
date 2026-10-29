/*
============================================================================
LeetCode 19. 删除链表的倒数第 N 个结点（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
分类：链表 · 倒数定位（长度 / 栈 / 快慢指针）
============================================================================

给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。

示例 1：
输入：head = [1,2,3,4,5], n = 2
输出：[1,2,3,5]

示例 2：
输入：head = [1], n = 1
输出：[]

示例 3：
输入：head = [1,2], n = 1
输出：[1]

提示：
链表中结点的数目为 sz
1 <= sz <= 30
0 <= Node.val <= 100
1 <= n <= sz

进阶：你能尝试使用一趟扫描实现吗？
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
解法一：计算链表长度（两趟扫描）【非最优】

为什么慢：倒数第 n 个，换成正数就是第 L - n + 1 个；而 L 事先不知道，
必须先把链表走一遍数长度，再走第二遍删节点。两趟扫描本身仍是 O(L)，但
题目在进阶里明确问「能否一趟扫描」。

换个角度：要删的节点必须知道它的前驱。长度为 L 时，前驱是正数第
L - n 个。加上哨兵后，从头数 L - n 步就停在前驱上，把 next 跨过去即可。

关键点：
  · 必须用哨兵 —— 否则删的正好是头节点（L == n）时，没有前驱可改
  · 步数是 L - n，不是 L - n + 1 —— 从哨兵出发，走 L - n 步落点就是前驱
  · 删除只改指针；在 Rust 里被摘下的 Box 会随作用域自动释放

Rust 的一处写法细节：
  · 沿 &ListNode 只读遍历数长度很方便；但要改 next 就必须拿到 &mut，
    所以定位阶段用 as_mut() 逐跳

以示例 1（head = [1,2,3,4,5], n = 2）为例：
  L = 5，需要删除正数第 4 个节点（值 4），其前驱是第 3 个（值 3）
  从哨兵走 L - n = 3 步，停在 3 上
  把 3 的 next 指向 5 → [1,2,3,5]

复杂度分析：
  时间 O(L)：数长度一趟，定位一趟
  空间 O(1)：只用哨兵和两个整数
*/
impl Solution1 {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 哨兵让「删除头节点」和「删除中间节点」走同一套逻辑
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        // 数长度：只读遍历，用 & 借用即可
        let mut length = 0;
        let mut cur = dummy.next.as_ref();
        while let Some(node) = cur {
            length += 1;
            cur = node.next.as_ref();
        }

        // 停在待删节点的前驱上：从哨兵走 length - n 步
        let steps = length - n;
        let mut prev: &mut ListNode = &mut dummy;
        for _ in 0..steps {
            prev = prev.next.as_mut().unwrap();
        }

        // 跨越被删节点：被摘下的 Box 在这里被丢弃，内存自动回收
        let mut removed = prev.next.take();
        prev.next = removed.as_mut().unwrap().next.take();

        dummy.next
    }
}

/*
解法二：栈（边走边压，弹出 n 个后栈顶即前驱）【非最优】

为什么慢：为了「从后往前数 n 个」这种反向需求，把整条链的节点地址都压进
栈里，额外占用 O(L) 空间。时间和一趟扫描一样是 O(L)，但空间不如快慢指针。

换个角度：栈天然就是「倒序访问」。把哨兵的 next 与所有节点地址依次压栈后，
栈顶是最后一个节点；弹出 n 个，栈顶就正好是待删节点的前驱。

关键点：
  · 哨兵的地址也要入栈 —— 否则删除头节点时链路会断（栈里没有前驱可退）
  · 弹出 n 个之后不要再弹，此时的栈顶就是前驱
  · 删除后要返回哨兵的 next，而不是原来的 head —— head 可能已经被删掉

Rust 的一处写法细节：
  · 用裸指针 *mut ListNode 入栈，是为了绕开「同时持有多个可变借用」的限制；
    这些指针都指向同一个链表内部，作用域内不会与其它借用重叠

以示例 1（head = [1,2,3,4,5], n = 2）为例：
  栈内自底向上为：哨兵、1、2、3、4、5
  弹出 2 个（5、4），栈顶是 3
  3 的 next 指向 3.next.next，即 5 → [1,2,3,5]

复杂度分析：
  时间 O(L)：每个节点入栈一次、出栈 n 次
  空间 O(L)：栈里存放全部节点地址
*/
impl Solution2 {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        // 收集所有节点地址（含哨兵），自底向上
        let mut nodes: Vec<*mut ListNode> = Vec::new();
        nodes.push(&mut *dummy as *mut ListNode);
        let mut cur = dummy.next.as_mut();
        while let Some(node) = cur {
            nodes.push(&mut **node as *mut ListNode);
            cur = node.next.as_mut();
        }

        // 弹出 n 个节点后，栈顶就是待删节点的前驱
        for _ in 0..n {
            nodes.pop();
        }
        let prev = *nodes.last().unwrap();
        // SAFETY: prev 指向本函数内的链表节点，生命周期覆盖此次写入
        unsafe {
            let mut removed = (*prev).next.take();
            (*prev).next = removed.as_mut().unwrap().next.take();
        }

        dummy.next
    }
}

/*
解法三：双指针（快指针先走 n 步，一趟扫描）【最优解】

官方三解中时间空间都最优的一个，也是本题的最优解：一趟扫描 O(L)，
额外空间 O(1)，正好回答题目进阶里的问题。

为什么慢：解法一要两趟，解法二要多占 O(L) 空间。二者的问题都是「必须先把
整条链看一遍才知道前驱在哪」。

换个角度：让 fast 先走 n 步，之后 fast 和 slow 同步前进。当 fast 走到
末尾时，slow 与 fast 相隔 n 个节点，正好停在待删节点的前驱上。
「倒数第 n 个」这个距离关系，可以直接用两条指针的脚步差来表达。

关键点：
  · fast 必须先走 n 步再开始同步 —— 顺序反了差值就变成 0
  · 走 n 步的起点是哨兵而不是 head —— 这样 n == L 时 fast 恰好走到尾节点，
    slow 停在哨兵上，删除头节点不需要额外分支
  · 循环条件是 fast.next.is_some()，不是 fast.is_some() —— 后者会让 slow
    多走一格，落到待删节点本身而不是它的前驱上

Rust 的一处写法细节：
  · 两条指针要对同一链表做可变访问，用裸指针最省事，读写都集中在
    一个 unsafe 块里，作用域清晰

以示例 1（head = [1,2,3,4,5], n = 2）为例：
  fast 从哨兵走 2 步，落在节点 2 上
  fast.next 非空就同步走：fast 到 3 时 slow 到哨兵；fast 到 4 时 slow 到 1；
  fast 到 5 时 slow 到 2；fast.next 为空时停止，slow 停在 3
  3 是待删节点 4 的前驱 → [1,2,3,5]

复杂度分析：
  时间 O(L)：fast 恰好走完一遍链表
  空间 O(1)：只用哨兵和两条指针
*/
impl Solution3 {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        // SAFETY: fast、slow 都指向 dummy 内部的节点，作用域内不与其它借用重叠
        unsafe {
            let mut fast: *mut ListNode = &mut *dummy;
            let mut slow: *mut ListNode = &mut *dummy;

            // fast 先走 n 步：从哨兵出发，保证 n == 链表长度时 slow 落在哨兵上
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap().as_mut() as *mut ListNode;
            }

            // 同步前进，fast 走到尾节点时 slow 恰好是待删节点的前驱
            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap().as_mut() as *mut ListNode;
                slow = (*slow).next.as_mut().unwrap().as_mut() as *mut ListNode;
            }

            let mut removed = (*slow).next.take();
            (*slow).next = removed.as_mut().unwrap().next.take();
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

// 一个用例喂给三种解法：三种写法必须得到同一条链表
fn check(name: &str, values: &[i32], n: i32, want: &[i32]) {
    // 每种解法都会改链表结构，各用一份新建的节点
    let got1 = to_vec(&Solution1::remove_nth_from_end(build(values), n));
    let got2 = to_vec(&Solution2::remove_nth_from_end(build(values), n));
    let got3 = to_vec(&Solution3::remove_nth_from_end(build(values), n));

    assert_eq!(got1, want, "{}: solution1 got {:?}", name, got1);
    assert_eq!(got2, want, "{}: solution2 got {:?}", name, got2);
    assert_eq!(got3, want, "{}: solution3 got {:?}", name, got3);
}

fn main() {
    // 示例 1：删除中间节点
    check("example 1", &[1, 2, 3, 4, 5], 2, &[1, 2, 3, 5]);

    // 示例 2：只剩一个节点且要删它，结果必须为空
    check("example 2", &[1], 1, &[]);

    // 示例 3：删除尾节点
    check("example 3", &[1, 2], 1, &[1]);

    // 边界：删除头节点（n == 链表长度），专防「没有前驱」的写法
    check("remove head", &[1, 2, 3], 3, &[2, 3]);

    // 边界：n == 1 且链表较长，验证快慢指针不会删错尾部
    check("remove tail", &[1, 2, 3, 4], 1, &[1, 2, 3]);

    println!("all tests passed");
}
