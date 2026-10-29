/*
============================================================================
LCR 125 / 剑指 Offer 09. 图书整理 II（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/
分类：栈 · 设计（双栈模拟队列）

读者来到图书馆排队借还书，图书管理员使用两个书车来完成整理借还书的任务。
书车中的书从下往上叠加存放，图书管理员每次只能拿取书车顶部的书。
排队的读者会有两种操作：
  · push(bookID)：把借阅的书籍还到图书馆。
  · pop()：从图书馆中借出书籍。

为了保持图书的顺序，图书管理员每次取出供读者借阅的书籍是 最早 归还到图书馆的书籍。
你需要返回 每次读者借出书的值 。
如果没有归还的书可以取出，返回 -1 。

示例 1：
输入：
["BookQueue", "push", "push", "pop"]
[[], [1], [2], []]
输出：[null,null,null,1]
解释：
MyQueue myQueue = new MyQueue();
myQueue.push(1); // queue is: [1]
myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
myQueue.pop(); // return 1, queue is [2]

提示：
1 <= bookID <= 10000
最多会对 push、pop 进行 10000 次调用

说明：题面里的 push / pop 在判题模板中对应的函数名是 appendTail / deleteHead，
下面沿用平台模板的函数签名（Rust 侧按蛇形命名）。
*/
use std::collections::VecDeque;

/*
解法：双栈（输入栈 + 输出栈）【最优解】

官方题解只给出这一种方法，也就是本题的最优解：均摊时间 O(1)、空间 O(n)。

为什么慢：如果只用一个栈，要取出最早进来的元素就得把整栈倒一遍（把栈顶挪开
再放回去），每次出队都是 O(n)。而要求「先入先出」恰恰是栈做不了的事。

换个角度：一次颠倒改变不了顺序，那就颠倒两次 —— 把输入栈的数据整体倒进输出栈，
「先进」就变成了「在输出栈的栈顶」。于是：
  · in_stack 只负责 push（进队）；
  · out_stack 只负责 pop（出队）；
  · 只有当 out_stack 空了，才把 in_stack 整个倒过去。

关键点：
  · 「输出栈为空才整体倒一次」是均摊 O(1) 的关键：每个元素一生只在两个栈之间
    搬一次，所以 m 次操作总共 O(m)，而不是每次出队都 O(n)
  · 输出栈非空时绝不能倒 —— 否则新倒进来的元素会压在老的上面，顺序就乱了
  · 两个栈都空时返回 -1
  · 倒的时候要「依次弹出再压入」，倒完 in_stack 必须是空的

以示例 2（deleteHead、appendTail(5)、appendTail(2)、deleteHead、deleteHead）为例：
  初始：in = []，out = []
  deleteHead：out 空、in 也空 → 返回 -1
  appendTail(5)：in = [5]
  appendTail(2)：in = [5, 2]（2 在栈顶）
  deleteHead：out 空 → 把 in 倒入 out，out = [2, 5]（5 在栈顶）→ 弹出 5
  deleteHead：out 非空 → 直接弹出 2
  结果 [null, -1, null, null, 5, 2]

复杂度分析：
  时间：单次 delete_head 最坏 O(n)，但每个元素最多被搬运一次，
        因此 m 次操作的均摊复杂度为 O(1)
  空间 O(n)：两个栈合计存放 n 个元素
*/
pub struct CQueue {
    in_stack: Vec<i32>,  // 只用 push 尾插模拟入栈
    out_stack: Vec<i32>, // 只从尾部 pop 模拟出栈
}

impl CQueue {
    pub fn new() -> Self {
        CQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    pub fn append_tail(&mut self, value: i32) {
        // 进队只往输入栈压
        self.in_stack.push(value);
    }

    pub fn delete_head(&mut self) -> i32 {
        // 输出栈空了才整体倒一次；非空时绝不能倒，否则顺序被打乱
        if self.out_stack.is_empty() {
            while let Some(value) = self.in_stack.pop() {
                self.out_stack.push(value);
            }
        }

        // 两个栈都空，说明没有书可以借出
        match self.out_stack.pop() {
            Some(value) => value,
            None => -1,
        }
    }
}

impl Default for CQueue {
    fn default() -> Self {
        Self::new()
    }
}

// 记录一次操作：name 是 "appendTail"/"deleteHead"，value 是参数
type Op<'a> = (&'a str, i32);

// 按操作序列跑一遍，返回每次操作的输出（append_tail 记 None 表示 null）
fn run(ops: &[Op]) -> Vec<Option<i32>> {
    let mut queue = CQueue::new();
    let mut out = Vec::new();
    for &(name, value) in ops {
        if name == "appendTail" {
            queue.append_tail(value);
            out.push(None);
        } else {
            out.push(Some(queue.delete_head()));
        }
    }
    out
}

fn check(ops: &[Op], want: &[Option<i32>]) {
    let got = run(ops);
    assert_eq!(got, want, "run({:?}) = {:?}, want {:?}", ops, got, want);
}

// 用 VecDeque 当对照答案，随机操作序列做交叉验证
fn cross_check() {
    let mut queue = CQueue::new();
    let mut expect: VecDeque<i32> = VecDeque::new();
    let mut seed: u32 = 20240607;
    for _ in 0..2000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        if (seed >> 16) % 10 < 7 {
            let value = ((seed >> 8) % 10000) as i32 + 1;
            queue.append_tail(value);
            expect.push_back(value);
        } else if expect.is_empty() {
            assert_eq!(queue.delete_head(), -1);
        } else {
            assert_eq!(queue.delete_head(), expect.pop_front().unwrap());
        }
    }
}

fn main() {
    // 示例 1
    check(
        &[("appendTail", 1), ("appendTail", 2), ("deleteHead", 0)],
        &[None, None, Some(1)],
    );

    // 示例 2
    check(
        &[
            ("deleteHead", 0),
            ("appendTail", 5),
            ("appendTail", 2),
            ("deleteHead", 0),
            ("deleteHead", 0),
        ],
        &[Some(-1), None, None, Some(5), Some(2)],
    );

    // 边界：建队后连续出队，全部返回 -1
    check(&[("deleteHead", 0), ("deleteHead", 0)], &[Some(-1), Some(-1)]);

    // 边界：入队后清空，再重新入队仍能正确出队
    check(
        &[
            ("appendTail", 1),
            ("appendTail", 2),
            ("deleteHead", 0),
            ("deleteHead", 0),
            ("appendTail", 3),
            ("deleteHead", 0),
        ],
        &[None, None, Some(1), Some(2), None, Some(3)],
    );

    // 随机交叉验证
    cross_check();

    println!("all tests passed");
}
