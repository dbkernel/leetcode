/*
============================================================================
147. 对链表进行插入排序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/insertion-sort-list/
分类：链表 · 插入排序 / 归并排序

给定单个链表的头 head ，使用 插入排序 对链表进行排序，并返回 排序后链表
的头 。

插入排序 算法的步骤:

1. 插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的
   输出列表。
2. 每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列
   中适当的位置，并将其插入。
3. 重复直到所有输入数据插入完为止。

下面是插入排序算法的一个图形示例。部分排序的列表(黑色)最初只包含列表中的
第一个元素。每次迭代时，从输入数据中删除一个元素(红色)，并就地插入已排序
的列表中。

对链表进行插入排序。

示例 1：
输入: head = [4,2,1,3]
输出: [1,2,3,4]

示例 2：
输入: head = [-1,5,3,4,0]
输出: [-1,0,3,4,5]

提示：
- 列表中的节点数在 [1, 5000] 范围内
- -5000 <= Node.val <= 5000
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

/*
解法一：链表插入排序（官方方法，题面指定算法）【最优解】

为什么慢：数组插入排序的代价在「搬移元素」，链表插入只需改指针，插入本身
是 O(1)；但单链表没有前驱指针，每次都要从表头走一遍才能找到插入点，单次
定位 O(n)，n 次插入累计 O(n^2)。输入已经有序时，一次比较就能把节点接到
已排序段尾部，退化成 O(n)，但逆序输入每次都要从头扫。

换个角度：把链表分成「已排序段」和「待插入段」。已排序段用 dummy 开头，
每轮从原链表摘下一个节点，在已排序段里找第一个严格大于它的位置插进去。

关键点：
  · 从原链表摘节点时先 node.next.take()，再用它自己的 next 接已排序段
  · 查找用 next.val > node.val 作为停止条件：相等时停在它后面，保持稳定
  · Rust 的 Box 所有权唯一，无法像 C++ 那样保留 lastSorted 指针再原地插入；
    这里改为「逐个摘下、插入新有序段」，是同一个插入排序，复杂度不变
  · dummy 节点让节点可以插到表头之前，最终返回 dummy.next

以示例 1（head = [4,2,1,3]）为例：
  摘下 4，有序段 [4]
  摘下 2，插到 4 前面 → [2,4]
  摘下 1，插到最前 → [1,2,4]
  摘下 3，插到 4 前面 → [1,2,3,4]

复杂度分析：
  时间 O(n^2)：每插入一个节点最多扫描 O(n)
  空间 O(1)：只改 next 指针，没有新建节点

解法二：自顶向下归并排序【非最优】

为什么慢：解法一的瓶颈是「每次插入都要重新定位」。链表不能随机访问，插入
排序无法用二分把定位降到 O(log n)。

换个角度：既然最终只要求升序，可以放弃插入排序，改用分治：先数出长度，
从中间切成两半，递归排好左右两半，再合并两个有序链表。

关键点：
  · 按长度对半切，偶数长度时左半段比右半段多一个节点，两半长度差不超过 1
  · 切分时走到左半段最后一个节点，take() 它的 next 得到右半段
  · 合并时用 dummy 承接，比较用 <= 保持稳定
  · 渐近上比插入排序更优，但题面明确要求「使用插入排序」，所以它不算本题
    的解；此处保留只为对照，本题以解法一为准

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归栈深度为 log n；节点本身复用
*/
impl Solution1 {
    pub fn insertion_sort_list(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = head;

        while let Some(mut node) = curr {
            // 先摘下当前节点，原链表剩余部分留给下一轮
            curr = node.next.take();

            // 在已排序段里找第一个严格大于 node 的位置
            let mut p: &mut ListNode = &mut dummy;
            loop {
                let stop = match p.next.as_ref() {
                    Some(next) => next.val > node.val,
                    None => true,
                };
                if stop {
                    break;
                }
                p = p.next.as_mut().unwrap();
            }

            // 先接后继，再让前驱指向 node，顺序反了会丢已排序段
            node.next = p.next.take();
            p.next = Some(node);
        }

        dummy.next
    }
}

impl Solution2 {
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

        // 走到左半段末尾，take() 出右半段
        let right = {
            let mut cur = head.as_mut().unwrap();
            for _ in 1..length / 2 {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next.take()
        };

        let left = Solution2::sort_list(head);
        let right = Solution2::sort_list(right);
        Solution2::merge(left, right)
    }

    // 合并两个有序链表，返回新表头；节点全部复用
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
    let got1 = to_vec(&Solution1::insertion_sort_list(build(input)));
    let got2 = to_vec(&Solution2::sort_list(build(input)));
    assert_eq!(got1, want, "insertion({input:?}) = {got1:?}");
    assert_eq!(got2, want, "merge({input:?}) = {got2:?}");
}

fn main() {
    // 示例 1
    check(&[4, 2, 1, 3], &[1, 2, 3, 4]);

    // 示例 2
    check(&[-1, 5, 3, 4, 0], &[-1, 0, 3, 4, 5]);

    // 边界：空链表（题面保证至少 1 个节点，这里验证健壮性）
    check(&[], &[]);

    // 边界：单节点
    check(&[7], &[7]);

    // 边界：已经有序，插入排序应只比较尾节点
    check(&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]);

    // 边界：完全逆序，每次都要插到最前面
    check(&[5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]);

    // 边界：全部相同，验证稳定性与不会死循环
    check(&[3, 3, 3, 3], &[3, 3, 3, 3]);

    // 边界：重复值夹杂在乱序里
    check(&[2, 1, 2, 1, 2], &[1, 1, 2, 2, 2]);

    // 边界：负数与正数混合
    check(&[0, -5000, 5000, -1, 1], &[-5000, -1, 0, 1, 5000]);

    println!("all tests passed");
}
