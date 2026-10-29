/*
============================================================================
141. 环形链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/linked-list-cycle/
分类：链表 · 判环（哈希表 / Floyd 快慢指针）

给你一个链表的头节点 head ，判断链表中是否有环。

如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。
为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中
的位置（索引从 0 开始）。注意：pos 不作为参数进行传递。仅仅是为了标识链表
的实际情况。

如果链表中存在环，则返回 true 。 否则，返回 false 。

示例 1：
输入：head = [3,2,0,-4], pos = 1
输出：true
解释：链表中有一个环，其尾部连接到第二个节点。

示例 2：
输入：head = [1,2], pos = 0
输出：true
解释：链表中有一个环，其尾部连接到第一个节点。

示例 3：
输入：head = [1], pos = -1
输出：false
解释：链表中没有环。

提示：
链表中节点的数目范围是 [0, 10^4]
-10^5 <= Node.val <= 10^5
pos 为 -1 或者链表中的一个 有效索引 。

进阶：你能用 O(1)（即，常量）内存解决此问题吗？

实现说明（Rust）：
leetcode.cn 目前没有为本题提供 Rust 语言模板（可用语言里不含 rust）。
而且带环的链表用 Rust 惯用的 Option<Box<ListNode>> 根本无法构造（每个节点
只能有一个所有者），所以这里按官方 C 语言判题的口径，用裸指针
*mut ListNode 建模。函数签名与平台上其他语言的语义一致。
*/
use std::collections::HashSet;
use std::ptr;

// 与平台其他语言一致的节点定义；本题用裸指针构造环
#[repr(C)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

// 本题有两种官方解法，按「一解一个结构体」拆成 Solution1 / Solution2。
struct Solution1;
struct Solution2;

/*
解法一：哈希表（记录访问过的节点，再次遇到即有环）【非最优】

为什么慢：时间 O(n) 没有问题，问题在空间 O(n) —— 要为每个节点存一份指针。
题目「进阶」明确要求 O(1) 内存，这一解不满足。

换个角度：有环等价于「沿着 next 一直走，永远走不到头」。想快速发现「又回到
老地方」，最直接的做法就是把走过的节点记下来，每次到达时查一下是否访问过。

关键点：
  · 存的是节点地址而不是 val —— 值相同的不同节点很常见，按值判断会误报
  · 裸指针不能直接进 HashSet（没有实现 Hash），先转成 usize 再存
  · 判断「是否已存在」要在插入之前做
  · 空链表直接不进循环，返回 false

以示例 1（head = [3,2,0,-4], pos = 1）为例：
  依次访问 3、2、0、-4，四点都进集合
  从 -4 回到下标 1 的节点 2，发现它已在集合中 → 返回 true

复杂度分析：
  时间 O(n)：每个节点最多访问一次
  空间 O(n)：集合存节点地址
*/
impl Solution1 {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut seen: HashSet<usize> = HashSet::new();

        let mut p = head;
        while !p.is_null() {
            if !seen.insert(p as usize) {
                // insert 返回 false 说明这个地址此前已经出现过
                return true;
            }
            // 安全前提：p 非空且指向调用方保证有效的链表节点
            p = unsafe { (*p).next };
        }

        false
    }
}

/*
解法二：快慢指针（Floyd 判圈）【最优解】

题目「进阶」要求 O(1) 内存，官方这一解正是为此设计，故标为【最优解】：
时间 O(n)，空间 O(1)。

为什么慢：解法一只需一次遍历，时间已经最优；但它必须把每个节点都记下来。
换个角度看，判断「有没有环」其实只需要知道「能不能一直走下去」，
而这个事实可以用两个速度不同的指针来探测。

换个角度：慢指针每步走 1 格，快指针每步走 2 格。
  · 无环：快指针一定先撞到尾节点之后（空指针），此时立刻返回 false
  · 有环：两者先后进入环，快指针每轮净追 1 格，必定追上慢指针
这个「净追 1 格」就是它能保证相遇的原因。

关键点：
  · 官方写法把 slow 放在 head、fast 放在 head->next，这样 while 循环条件
    「是否相遇」就有意义；若两者都放 head，循环体会一次都不执行
  · fast 的两步跳必须先后判空：先判 fast 为空，再判 fast->next 为空，
    顺序反了会解引用空指针
  · 单节点自环（pos = 0）时 head->next 指向自己，不能被开头的
    「head->next 为空」误判成无环
  · 空链表直接返回 false

以示例 2（head = [1,2], pos = 0）为例：
  slow = 节点 1，fast = 节点 2
  一轮后 slow = 节点 2，fast 走两步回到节点 2
  两者重合 → 返回 true

以示例 3（head = [1], pos = -1）为例：
  head->next 为空 → 直接返回 false

复杂度分析：
  时间 O(n)：慢指针最多绕环一圈，快指针不超过两圈
  空间 O(1)：只用两个指针
*/
impl Solution2 {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        if head.is_null() {
            return false;
        }
        // 安全前提：head 非空且有效
        let next = unsafe { (*head).next };
        if next.is_null() {
            return false;
        }

        // fast 起步领先一步，使得 while 的「是否相遇」条件有意义
        let mut slow = head;
        let mut fast = next;

        while slow != fast {
            // 先判 fast，再判 fast->next，顺序不能反
            if fast.is_null() {
                return false;
            }
            let fast_next = unsafe { (*fast).next };
            if fast_next.is_null() {
                return false;
            }

            slow = unsafe { (*slow).next };
            fast = unsafe { (*fast_next).next };
        }

        true
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 统一管理本用例分配出的节点；链表可能有环，不能靠遍历释放
struct Fixture {
    owned: Vec<*mut ListNode>,
}

impl Fixture {
    // 按 values 建链，再把尾节点接到下标 pos 的节点上；pos < 0 表示无环
    fn build(&mut self, values: &[i32], pos: i32) -> *mut ListNode {
        let mut nodes: Vec<*mut ListNode> = Vec::with_capacity(values.len());
        for &v in values {
            let node = Box::into_raw(Box::new(ListNode {
                val: v,
                next: ptr::null_mut(),
            }));
            self.owned.push(node);
            nodes.push(node);
        }

        for i in 0..nodes.len().saturating_sub(1) {
            unsafe {
                (*nodes[i]).next = nodes[i + 1];
            }
        }

        if !nodes.is_empty() && pos >= 0 {
            let last = *nodes.last().unwrap();
            unsafe {
                (*last).next = nodes[pos as usize];
            }
        }

        nodes.first().copied().unwrap_or(ptr::null_mut())
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

// 一个用例喂给两种解法
fn check(name: &str, values: &[i32], pos: i32, want: bool) {
    let mut fixture = Fixture { owned: Vec::new() };
    // 两种解法只移动各自的指针变量，不会改动链表，可以共用同一份结构
    let head = fixture.build(values, pos);

    let got1 = Solution1::has_cycle(head);
    let got2 = Solution2::has_cycle(head);

    assert_eq!(got1, want, "{}: by_hash got {}", name, got1);
    assert_eq!(got2, want, "{}: two_pointers got {}", name, got2);
}

fn main() {
    // 示例 1：环的入口在中间
    check("example 1", &[3, 2, 0, -4], 1, true);

    // 示例 2：环的入口是头节点（整条链是环）
    check("example 2", &[1, 2], 0, true);

    // 示例 3：单节点无环
    check("example 3", &[1], -1, false);

    // 边界：空链表，两解都必须直接返回 false
    check("empty", &[], -1, false);

    // 边界：单节点自环，验证不会被 head->next 判空提前判死
    check("self loop", &[1], 0, true);

    // 边界：尾节点自环，环长为 1
    check("tail loop", &[1, 2, 3], 2, true);

    // 边界：多节点无环，验证快指针会正常撞到尾部
    check("no loop", &[1, 2, 3, 4], -1, false);

    println!("all tests passed");
}
