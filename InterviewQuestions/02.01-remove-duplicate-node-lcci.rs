/*
============================================================================
面试题 02.01. 移除重复节点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicate-node-lcci/
分类：哈希表 · 链表去重

编写代码，移除未排序链表中的重复节点。保留最开始出现的节点。

示例1：
 输入：[1, 2, 3, 3, 2, 1]
 输出：[1, 2, 3]

示例2：
 输入：[1, 1, 1, 1, 2]
 输出：[1, 2]

提示：
- 链表长度在[0, 20000]范围内。
- 链表元素在[0, 20000]范围内。

进阶：
如果不得使用临时缓冲区，该怎么解决？
*/
use std::collections::HashSet;

// 力扣预置的节点定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode
{
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode
{
    #[inline]
    pub fn new(val: i32) -> Self
    {
        ListNode { val, next: None }
    }
}

/*
解法一：哈希集合记录出现过的值（一次遍历）【最优解】

为什么慢：最直接的做法是对每个节点再扫一遍后面的链表，删掉值相同的
节点，时间 O(n^2)。链表未排序，无法像数组那样先排序再相邻去重；
而重复判断只关心"这个值以前出现过没有"，不需要保留先后关系之外的信息。

换个角度：遍历时用一个哈希集合记录已经保留的值。每个节点只问一次
"见过吗"：见过就删，没见过就留下并登记。一次遍历即可完成。

关键点：
  · 集合里存值而不是节点地址：链表节点按地址比较，值相同的节点地址不同
  · 枚举前驱节点，待删节点是它的 next，删除时只改前驱的 next
  · 只有保留节点时才把前驱前移；删除节点时前驱原地不动
  · 头节点一定保留，先把它登记进集合，循环从 head.next 开始
  · Rust 的 Box 在离开作用域时自动释放，被摘掉的节点随之回收

以示例 1（[1,2,3,3,2,1]）为例：
  保留 1，seen={1}；保留 2，seen={1,2}；保留 3，seen={1,2,3}
  下一个 3 已见过，删；下一个 2 已见过，删；下一个 1 已见过，删
  得到 [1,2,3] ✔

复杂度分析：
  时间 O(n)：每个节点最多访问一次
  空间 O(n)：最坏情况下所有值都不同，集合要存 n 个值

解法二：两重循环（时间换空间，对应进阶）【非最优】

不借助缓冲区时，只能让每个保留节点去"清理"它后面所有同值节点：
外层枚举保留节点，内层从该节点开始枚举待删节点的前驱，遇到同值就删。
  · 空间降到 O(1)，但时间升到 O(n^2)
  · 保留的仍是"最开始出现"的节点，因为外层从左往右处理
  · 官方提示 Python 用此解法会超时，这里只作为空间受限时的对照

复杂度分析：
  时间 O(n^2)：最坏（全不相同）每个外层节点都要扫完剩余链表
  空间 O(1)：只用了两个遍历指针
*/

// 单文件 rustc 编译时平台预置的 Solution 不存在，以下类型为本地编译所需
// ---------- 解法一：哈希集合（最优解） ----------
pub struct Solution1;

impl Solution1
{
    pub fn remove_duplicate_nodes(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>
    {
        let mut seen = HashSet::new();
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut cur = head;

        while let Some(mut node) = cur {
            // 先把后继摘出来，避免 node 被移动后拿不到 next
            cur = node.next.take();
            if seen.insert(node.val) {
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

// ---------- 解法二：两重循环（时间换空间） ----------
pub struct Solution2;

impl Solution2
{
    pub fn remove_duplicate_nodes(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>
    {
        let mut outer = head.as_deref_mut();
        while let Some(keep) = outer {
            let val = keep.val;
            // reborrow 而不是移动 keep，循环结束后还要用它取 next
            let mut inner: &mut ListNode = &mut *keep;
            while inner.next.is_some() {
                if inner.next.as_ref().unwrap().val == val {
                    let mut dup = inner.next.take().unwrap();
                    inner.next = dup.next.take();
                } else {
                    inner = inner.next.as_mut().unwrap();
                }
            }
            outer = keep.next.as_deref_mut();
        }
        head
    }
}

fn build_list(vals: &[i32]) -> Option<Box<ListNode>>
{
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32>
{
    let mut vals = Vec::new();
    while let Some(node) = head {
        vals.push(node.val);
        head = node.next;
    }
    vals
}

fn check(input: &[i32], want: &[i32])
{
    let got1 = list_to_vec(Solution1::remove_duplicate_nodes(build_list(input)));
    assert_eq!(got1, want, "hash({:?})", input);

    let got2 = list_to_vec(Solution2::remove_duplicate_nodes(build_list(input)));
    assert_eq!(got2, want, "brute({:?})", input);
}

fn main()
{
    // 示例 1
    check(&[1, 2, 3, 3, 2, 1], &[1, 2, 3]);

    // 示例 2
    check(&[1, 1, 1, 1, 2], &[1, 2]);

    // 边界：空链表
    check(&[], &[]);

    // 边界：单节点 / 两个相同节点
    check(&[1], &[1]);
    check(&[1, 1], &[1]);

    // 边界：重复值分散出现，保留第一次出现的顺序
    check(&[1, 2, 1, 3, 2, 4], &[1, 2, 3, 4]);
    check(&[1, 3, 2, 3, 1, 2, 4], &[1, 3, 2, 4]);

    // 边界：全部相同
    check(&[7, 7, 7, 7], &[7]);

    // 边界：本来就没有重复
    check(&[1, 2, 3, 4], &[1, 2, 3, 4]);

    // 规模：200 个节点，值循环出现 0..9
    let big: Vec<i32> = (0..200).map(|i| i % 10).collect();
    let want: Vec<i32> = (0..10).collect();
    check(&big, &want);

    println!("all tests passed");
}
