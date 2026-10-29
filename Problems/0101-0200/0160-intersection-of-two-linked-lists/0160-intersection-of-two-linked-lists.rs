/*
============================================================================
160. 相交链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/intersection-of-two-linked-lists/
分类：链表 · 双指针（哈希集合 / 走对方的路）

给你两个单链表的头节点 headA 和 headB ，请你找出并返回两个单链表相交的
起始节点。如果两个链表不存在相交节点，返回 null 。

题目数据 保证 整个链式结构中不存在环。

注意，函数返回结果后，链表必须 保持其原始结构 。

自定义评测：
评测系统 的输入如下（你设计的程序 不适用 此输入）：
- intersectVal - 相交的起始节点的值。如果不存在相交节点，这一值为 0
- listA - 第一个链表
- listB - 第二个链表
- skipA - 在 listA 中（从头节点开始）跳到交叉节点的节点数
- skipB - 在 listB 中（从头节点开始）跳到交叉节点的节点数

评测系统将根据这些输入创建链式数据结构，并将两个头节点 headA 和 headB
传递给你的程序。如果程序能够正确返回相交节点，那么你的解决方案将被
视作正确答案。

示例 1：
输入：intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5],
      skipA = 2, skipB = 3
输出：Intersected at '8'
解释：相交节点的值为 8（注意，如果两个链表相交则不能为 0）。
从各自的表头开始算起，链表 A 为 [4,1,8,4,5]，链表 B 为 [5,6,1,8,4,5]。
在 A 中，相交节点前有 2 个节点；在 B 中，相交节点前有 3 个节点。

示例 2：
输入：intersectVal = 2, listA = [1,9,1,2,4], listB = [3,2,4],
      skipA = 3, skipB = 1
输出：Intersected at '2'

示例 3：
输入：intersectVal = 0, listA = [2,6,4], listB = [1,5],
      skipA = 3, skipB = 2
输出：No intersection

提示：
listA 中节点数目为 m
listB 中节点数目为 n
1 <= m, n <= 3 * 10^4
1 <= Node.val <= 10^5
0 <= skipA <= m
0 <= skipB <= n
如果 listA 和 listB 没有交点，intersectVal 为 0
如果 listA 和 listB 有交点，
intersectVal == listA[skipA] == listB[skipB]

进阶：你能否设计一个时间复杂度 O(m + n)、仅用 O(1) 内存的解决方案？

实现说明（Rust）：
leetcode.cn 目前没有为本题提供 Rust 语言模板（可用语言里不含 rust）。
而本题的两条链表共享同一段尾部，用 Rust 惯用的 Option<Box<ListNode>>
根本无法表达「两个所有者指向同一节点」，所以这里按官方 C 语言判题的口径，
用裸指针 *mut ListNode 建模。函数签名与平台上其他语言的语义一致：
返回相交的起始节点，无交点返回空指针。
*/
use std::collections::HashSet;
use std::ptr;

// 与平台其他语言一致的节点定义；本题用裸指针共享节点
#[repr(C)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

// 本题有两种官方解法，按「一解一个结构体」拆成 Solution1 / Solution2。
struct Solution1;
struct Solution2;

/*
解法一：哈希集合（把 A 的所有节点存起来，再扫 B）【非最优】

为什么慢：时间上已经是 O(m + n)，但额外用了 O(m) 的空间存 A 的全部节点。
对 3 万节点的问题不算大，可题目「进阶」明确要求 O(1) 内存，这一解不满足。

换个角度：判断「相交」的本质是判断两个链表有没有同一个节点（按地址比较，
不是按值比较）。既然要快速判断「B 的当前节点在不在 A 里」，最直接的就是
先把 A 的节点放进一个集合，再逐个查 B。

关键点：
  · 比较的是节点地址而不是 val —— 示例 1 里 A、B 都有值为 1 的节点，
    但它们是两个不同对象，按值比较会返回错误的节点
  · 裸指针不能直接进 HashSet（没有实现 Hash），先转成 usize 再存
  · 不用访问 val，也不修改任何 next，天然满足「保持原始结构」

以示例 1（A = [4,1,8,4,5], B = [5,6,1,8,4,5]，公共部分 [8,4,5]）为例：
  集合里存入 A 的五个节点地址
  扫 B：5 不在 → 6 不在 → 1 不在（B 的这个 1 与 A 的 1 是不同对象）
        → 8 在 → 返回该节点，即公共尾部的第一个节点

复杂度分析：
  时间 O(m + n)：两条链表各扫一遍
  空间 O(m)：集合存 A 的全部节点
*/
impl Solution1 {
    pub fn get_intersection_node(head_a: *mut ListNode, head_b: *mut ListNode) -> *mut ListNode {
        let mut visited: HashSet<usize> = HashSet::new();

        let mut p = head_a;
        while !p.is_null() {
            visited.insert(p as usize);
            // 安全前提：p 非空且指向本函数调用方保证有效的链表节点
            p = unsafe { (*p).next };
        }

        let mut p = head_b;
        while !p.is_null() {
            if visited.contains(&(p as usize)) {
                return p;
            }
            p = unsafe { (*p).next };
        }

        ptr::null_mut()
    }
}

/*
解法二：双指针（走完自己走对方的链）【最优解】

题目「进阶」要求 O(1) 内存，官方这一解正是为此设计，故标为【最优解】：
时间 O(m + n)，空间 O(1)。

为什么慢：解法一的空间开销可以用「抵消长度差」的思路省掉。麻烦之处在于
不知道两条链谁长、长多少，先求长度差再出发又要多写一层计数。

换个角度：设 A 的独有部分是 a 个节点，B 的独有部分是 b 个节点，公共尾部是
c 个节点。让指针 pA 走完 A 再接着走 B，pB 走完 B 再接着走 A：
  · pA 到达相交点共走 a + c + b 步
  · pB 到达相交点共走 b + c + a 步
两边步数相等，所以它们必然同时到达相交点；若无交点（c = 0），则两者同时
走成空指针。换句话说，「把对方的路也走一遍」自动消掉了长度差。

关键点：
  · 指针为空时才切换到另一条链的头
  · 循环条件是 pA != pB（比较地址），而不是比较 val
  · 无交点时两者最后都变成空指针，循环自然结束并返回空指针，
    不需要额外判断
  · 只移动指针，不改动任何 next，符合「保持原始结构」的要求

以示例 2（A = [1,9,1,2,4], B = [3,2,4]，公共 [2,4]）为例：
  a = 3, b = 1, c = 2
  pA 走 A 的 5 个节点后接到 B 头，再走 1 步到相交点，共 6 步
  pB 走 B 的 3 个节点后接到 A 头，再走 3 步到相交点，共 6 步
  第 6 步时两者指向同一个节点 2

复杂度分析：
  时间 O(m + n)：每个指针最多走 m + n 步
  空间 O(1)：只用两个指针
*/
impl Solution2 {
    pub fn get_intersection_node(head_a: *mut ListNode, head_b: *mut ListNode) -> *mut ListNode {
        let mut pa = head_a;
        let mut pb = head_b;

        // 走到空指针就换到对方链表的头部，长度差被自动抵消
        while pa != pb {
            pa = if pa.is_null() {
                head_b
            } else {
                unsafe { (*pa).next }
            };
            pb = if pb.is_null() {
                head_a
            } else {
                unsafe { (*pb).next }
            };
        }

        // 有交点时返回相交节点；无交点时两者同为空指针
        pa
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 统一管理本用例分配出的节点，避免交叉链表重复释放
struct Fixture {
    owned: Vec<*mut ListNode>,
}

impl Fixture {
    fn make(&mut self, val: i32) -> *mut ListNode {
        let node = Box::into_raw(Box::new(ListNode {
            val,
            next: ptr::null_mut(),
        }));
        self.owned.push(node);
        node
    }

    // 把 values 依次接在 tail 前面，返回新的头节点
    fn chain(&mut self, values: &[i32], tail: *mut ListNode) -> *mut ListNode {
        let mut head = tail;
        for &v in values.iter().rev() {
            let node = self.make(v);
            unsafe {
                (*node).next = head;
            }
            head = node;
        }
        head
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // 每个节点只被分配一次，也只在这里回收一次
        for &p in &self.owned {
            unsafe {
                drop(Box::from_raw(p));
            }
        }
    }
}

// 两条链表共享 common 这段尾部，验证返回的必须正是那个共享节点
fn check(name: &str, prefix_a: &[i32], prefix_b: &[i32], common: &[i32]) {
    let mut fixture = Fixture { owned: Vec::new() };

    // 先建公共尾部，再由它往前接各自的独有前缀
    let tail = fixture.chain(common, ptr::null_mut());
    let head_a = fixture.chain(prefix_a, tail);
    let head_b = fixture.chain(prefix_b, tail);

    let got1 = Solution1::get_intersection_node(head_a, head_b);
    let got2 = Solution2::get_intersection_node(head_a, head_b);

    if common.is_empty() {
        // 无交点必须返回空指针，而不是「值碰巧相等的某个节点」
        assert!(got1.is_null(), "{}: by_hash expected null", name);
        assert!(got2.is_null(), "{}: two_pointers expected null", name);
    } else {
        // 必须按地址命中公共尾部的第一个节点
        assert!(!got1.is_null(), "{}: by_hash returned null", name);
        assert!(!got2.is_null(), "{}: two_pointers returned null", name);
        assert_eq!(got1, tail, "{}: by_hash returned wrong node", name);
        assert_eq!(got2, tail, "{}: two_pointers returned wrong node", name);
    }
}

fn main() {
    // 示例 1：两链都有值为 1 的节点，但不是同一个节点，只能命中 8
    check("example 1", &[4, 1], &[5, 6, 1], &[8, 4, 5]);

    // 示例 2：两链长度差较大（3 对 1），验证长度差被抵消
    check("example 2", &[1, 9, 1], &[3], &[2, 4]);

    // 示例 3：完全不相交，两解都必须返回空指针
    check("example 3", &[2, 6, 4], &[1, 5], &[]);

    // 边界：公共部分只有一个节点，且它正好是两链的尾节点
    check("single common node", &[1], &[2, 3], &[9]);

    // 边界：两条链表从头就是同一个节点（前缀都为空）
    check("shared from head", &[], &[], &[7, 8]);

    // 边界：一侧前缀为空，验证「走完即切换」不会跳过头
    check("one empty prefix", &[], &[1, 4, 6], &[5]);

    println!("all tests passed");
}
