/*
============================================================================
LCR 140 / 剑指 Offer 22. 训练计划 II（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/
分类：链表 · 倒数第 k 个节点（两次遍历 / 快慢双指针）

说明：本题原名「链表中倒数第 k 个节点」，leetcode.cn 现已改写成场景题
「训练计划 II」，函数名为 trainingPlan，参数名改为 cnt。
============================================================================

给定一个头节点为 head 的链表用于记录一系列核心肌群训练项目编号，请查找并
返回倒数第 cnt 个训练项目编号对应的节点。

示例 1：
输入：head = [2,4,7,8], cnt = 1
输出：8

提示：
1 <= head.length <= 100
0 <= head[i] <= 100
1 <= cnt <= head.length
*/

// 平台预置的链表节点定义；本地用 rustc 单文件编译时需要显式声明
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// 本题有两种官方解法，按「一解一个结构体」拆成 Solution1 / Solution2；
// 提交时把选用的那个改名为 Solution 即可。
struct Solution1;
struct Solution2;

/*
解法一：顺序查找（先数长度，再走到第 n - cnt 个）【非最优】

为什么慢：要走两遍。第一遍只为了得到长度 n，第二遍才真正定位；对链表这种
只能单向遍历的结构，等于把前 n - cnt 个节点白扫了一次。时间仍是 O(n)，
但常数上明显多一倍。

换个角度：倒数第 cnt 个，就是正数第 n - cnt 个（从 0 开始计数）。只要知道
n，一条循环走过去即可。

关键点：
  · 第 n - cnt 个是 0 基下标，走 n - cnt 步
  · cnt 是 1 基的（倒数第 1 个是尾节点），不要多走一步
  · 题目保证 1 <= cnt <= n，无需处理越界

Rust 的一处写法细节：
  · 返回值是拥有的 Option<Box<ListNode>>，没法直接返回输入内部的引用，
    所以先借不可变引用数出 n，再按 n - cnt 步「移动」head 前进；
    每步用 and_then 取走 next，被跳过的那截随作用域结束自动释放

以示例 1（head = [2,4,7,8], cnt = 1）为例：
  第一遍数出 n = 4
  从 head 走 n - cnt = 3 步 → 停在 8

复杂度分析：
  时间 O(n)：两遍遍历
  空间 O(1)：只用两个指针
*/
impl Solution1 {
    pub fn training_plan(head: Option<Box<ListNode>>, cnt: i32) -> Option<Box<ListNode>> {
        // 第一遍只借引用数长度，不移动任何节点
        let mut n = 0i32;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            n += 1;
            cur = node.next.as_ref();
        }

        // 倒数第 cnt 个 = 正数第 n - cnt 个（0 基）
        let skip = (n - cnt) as usize;
        let mut head = head;
        for _ in 0..skip {
            head = head.and_then(|mut node| node.next.take());
        }

        head
    }
}

/*
解法二：快慢双指针（只走一遍）【最优解】

官方两解中只需一次遍历的一个，因此是本题的最优解：时间 O(n)（每个节点只被
访问一次），空间 O(1)。

为什么慢：解法一的第二遍是从头重新走。其实「倒数第 cnt 个」这个位置信息，
可以用两指针之间的固定间距表达出来，不必先知道 n。

换个角度：让 fast 先走 cnt 步，此时 fast 与 slow 之间正好隔着 cnt 个节点。
之后两者同步前进，当 fast 越过尾节点变成 None 时，slow 与 fast 相隔 cnt 步，
而 fast 已经走完了整条链表 —— slow 停下的位置正是倒数第 cnt 个。

关键点：
  · fast 先走 cnt 步，不是 cnt - 1 步；走完之后 fast 指向正数第 cnt + 1 个
  · 同步前进的终止条件是 fast 变成 None（而非 next 变成 None），
    因为 fast 要走到「尾节点的下一个」
  · cnt == n 时 fast 先走成 None，同步阶段一步都不走，slow 就是 head

Rust 的一处写法细节：
  · 双指针阶段用不可变引用完成，只累计 slow 需要走的步数；
    最后再按这个步数移动 head。语义与 C++ 的双指针完全等价，
    这样既不必绕开借用检查器，也不会引入 unsafe

以示例 1（head = [2,4,7,8], cnt = 1）为例：
  fast 走 1 步 → 指向 4（此时与 slow 相隔 1 个节点）
  两者同步前进：fast 4→7→8→None，slow 2→4→7→8
  fast 变 None 时 slow 停在 8 —— 倒数第 1 个

复杂度分析：
  时间 O(n)：fast 走 n 步，slow 走 n - cnt 步，合计不到 2n
  空间 O(1)：只用两个指针
*/
impl Solution2 {
    pub fn training_plan(head: Option<Box<ListNode>>, cnt: i32) -> Option<Box<ListNode>> {
        // 第一步：fast 独自向前 cnt 步，与 slow 拉开间距
        let mut fast = head.as_ref();
        let mut remaining = cnt;
        while remaining > 0 {
            match fast {
                Some(node) => {
                    fast = node.next.as_ref();
                    remaining -= 1;
                }
                // cnt 超过链长时提前收手，题目保证不会发生
                None => break,
            }
        }

        // 第二步：fast 与 slow 同步前进，只剩 fast 的步数需要计数
        let mut skip = 0usize;
        while let Some(node) = fast {
            fast = node.next.as_ref();
            skip += 1;
        }

        // 第三步：slow 走了 skip 步，按同样步数移动 head 得到答案
        let mut head = head;
        for _ in 0..skip {
            head = head.and_then(|mut node| node.next.take());
        }

        head
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

// 从某个节点开始展开，用于核对返回的节点及其后续
fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        out.push(node.val);
        cur = node.next.as_ref();
    }
    out
}

// 一个用例喂给两种解法：两次遍历与双指针必须返回同一个节点
fn check(name: &str, values: &[i32], cnt: i32, want: &[i32]) {
    let got1 = to_vec(&Solution1::training_plan(build(values), cnt));
    let got2 = to_vec(&Solution2::training_plan(build(values), cnt));

    assert_eq!(got1, want, "{}: by_length got {:?}", name, got1);
    assert_eq!(got2, want, "{}: two_pointers got {:?}", name, got2);
}

fn main() {
    // 示例 1：cnt = 1，返回尾节点
    check("example 1", &[2, 4, 7, 8], 1, &[8]);

    // 边界：cnt == n，返回头节点，验证 fast 先走成 None 时 slow 不动
    check("cnt equals n", &[2, 4, 7, 8], 4, &[2, 4, 7, 8]);

    // 边界：单节点链表，cnt 只能是 1
    check("single node", &[7], 1, &[7]);

    // 一般情况：返回中间节点，后续部分要一并正确
    check("middle node", &[1, 2, 3, 4, 5], 2, &[4, 5]);

    // 边界：cnt == n - 1，验证间距算的是 cnt 而不是 cnt - 1
    check("cnt near end", &[1, 2, 3], 2, &[2, 3]);

    println!("all tests passed");
}
