/*
============================================================================
225. 用队列实现栈（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/implement-stack-using-queues/
分类：队列 · 设计（队列模拟栈）

请你仅使用两个队列实现一个后入先出（LIFO）的栈，并支持普通栈的全部四种操作
（push、top、pop 和 empty）。

实现 MyStack 类：
  · void push(int x) 将元素 x 压入栈顶。
  · int pop() 移除并返回栈顶元素。
  · int top() 返回栈顶元素。
  · boolean empty() 如果栈是空的，返回 true ；否则，返回 false 。

注意：
  · 你只能使用队列的标准操作 —— 也就是 push to back、peek/pop from front、
    size 和 is empty 这些操作。
  · 你所使用的语言也许不支持队列。 你可以使用 list （列表）或者 deque
    （双端队列）来模拟一个队列 , 只要是标准的队列操作即可。

示例：
输入：
["MyStack", "push", "push", "top", "pop", "empty"]
[[], [1], [2], [], [], []]
输出：
[null, null, null, 2, 2, false]

解释：
MyStack myStack = new MyStack();
myStack.push(1);
myStack.push(2);
myStack.top(); // 返回 2
myStack.pop(); // 返回 2
myStack.empty(); // 返回 False

提示：
1 <= x <= 9
最多调用 100 次 push、pop、top 和 empty
每次调用 pop 和 top 都保证栈不为空

进阶：你能否仅用一个队列来实现栈。
*/
use std::collections::VecDeque;

/*
解法一：两个队列【非最优】

官方方法一。queue1 存栈内全部元素，queue2 作为入栈的辅助队列。
每来一个新元素，先把它塞进 queue2，再把 queue1 里的元素整体搬到 queue2 的尾巴上，
最后交换两个队列 —— 这样 queue1 的队首永远是栈顶，pop / top 直接取队首即可。

关键点：
  · 「搬运」是让新元素排到队首的唯一手段：先把新元素放进去，再把旧元素全部
    接到它后面
  · 交换 queue1 与 queue2 之后，queue2 天然变空，省去手动清理
  · 栈是否为空只需看 queue1

以 push(1)、push(2) 为例：
  push(1)：queue2 = [1]，queue1 空 → 交换 → queue1 = [1]，queue2 = []
  push(2)：queue2 = [2]，把 queue1 的 1 搬到 queue2 → queue2 = [2, 1] → 交换
           得 queue1 = [2, 1]，队首 2 正是栈顶
  pop：取 queue1 队首 2，剩 [1]

复杂度分析：
  时间：push O(n)，pop / top / empty O(1)
  空间 O(n)：两个队列合计存放 n 个元素（其中一个为空）
*/
pub struct MyStackTwoQueues {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

impl MyStackTwoQueues {
    pub fn new() -> Self {
        MyStackTwoQueues {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        // 新元素先进辅助队列，再把旧元素整个接到它后面
        self.queue2.push_back(x);
        while let Some(front) = self.queue1.pop_front() {
            self.queue2.push_back(front);
        }
        // 交换后 queue2 天然为空
        std::mem::swap(&mut self.queue1, &mut self.queue2);
    }

    pub fn pop(&mut self) -> i32 {
        self.queue1.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.queue1.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.queue1.is_empty()
    }
}

impl Default for MyStackTwoQueues {
    fn default() -> Self {
        Self::new()
    }
}

/*
解法二：一个队列【最优解】

官方方法二。既然每次入栈都要把「除了新元素之外的所有元素」轮转一遍，那么只需要
一个队列就够：记住入队前的元素个数 n，把新元素入队，再把这 n 个旧元素依次
出队再入队（相当于把队首搬到队尾），轮转结束后新元素自然来到了队首。

关键点：
  · n 一定要在入队之前取，否则新元素也会被算进去，多轮转一次就错位
  · 轮转 n 次（而不是 n + 1 次），刚好把新元素顶到队首
  · 与解法一相比时间同为 O(n) 入栈，但少了一个队列，空间常数更小

以 push(1)、push(2) 为例：
  push(1)：n = 0，入队后 q = [1]，轮转 0 次 → q = [1]
  push(2)：n = 1，入队后 q = [1, 2]，轮转 1 次（1 搬到队尾）→ q = [2, 1]
           队首 2 即栈顶
  pop：取队首 2，剩 [1]
  empty：q 非空 → false

复杂度分析：
  时间：push O(n)，pop / top / empty O(1)
  空间 O(n)：一个队列存放 n 个元素
*/
pub struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack {
            q: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        // n 必须在入队之前取，避免把新元素也轮转一遍
        let n = self.q.len();
        self.q.push_back(x);
        for _ in 0..n {
            let front = self.q.pop_front().unwrap();
            self.q.push_back(front);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

impl Default for MyStack {
    fn default() -> Self {
        Self::new()
    }
}

// 两套实现共用的小工具 -----------------------------------------------------------------

#[derive(Clone, Copy)]
enum Op {
    Push(i32),
    Pop,
    Top,
    Empty,
}

fn check_one_queue(ops: &[Op], want: &[&str]) {
    let mut stack = MyStack::new();
    let mut got = Vec::new();
    for &op in ops {
        match op {
            Op::Push(x) => {
                stack.push(x);
                got.push("null".to_string());
            }
            Op::Pop => got.push(stack.pop().to_string()),
            Op::Top => got.push(stack.top().to_string()),
            Op::Empty => got.push(stack.empty().to_string()),
        }
    }
    assert_eq!(got, want, "MyStack 输出不符");
}

fn check_two_queues(ops: &[Op], want: &[&str]) {
    let mut stack = MyStackTwoQueues::new();
    let mut got = Vec::new();
    for &op in ops {
        match op {
            Op::Push(x) => {
                stack.push(x);
                got.push("null".to_string());
            }
            Op::Pop => got.push(stack.pop().to_string()),
            Op::Top => got.push(stack.top().to_string()),
            Op::Empty => got.push(stack.empty().to_string()),
        }
    }
    assert_eq!(got, want, "MyStackTwoQueues 输出不符");
}

// 随机交叉验证：用 Vec 当对照栈
fn cross_check() {
    let mut one = MyStack::new();
    let mut two = MyStackTwoQueues::new();
    let mut expect: Vec<i32> = Vec::new();
    let mut seed: u32 = 20240607;
    for _ in 0..2000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let r = (seed >> 16) % 10;
        if r < 5 || expect.is_empty() {
            let value = ((seed >> 8) % 9) as i32 + 1;
            one.push(value);
            two.push(value);
            expect.push(value);
        } else if r < 8 {
            let want = expect.pop().unwrap();
            assert_eq!(one.pop(), want);
            assert_eq!(two.pop(), want);
        } else {
            assert!(!one.empty());
            assert!(!two.empty());
            assert_eq!(one.top(), *expect.last().unwrap());
            assert_eq!(two.top(), *expect.last().unwrap());
        }
    }
    assert_eq!(one.empty(), expect.is_empty());
    assert_eq!(two.empty(), expect.is_empty());
}

fn main() {
    // 示例
    let sample = [Op::Push(1), Op::Push(2), Op::Top, Op::Pop, Op::Empty];
    let want = ["null", "null", "2", "2", "false"];
    check_one_queue(&sample, &want);
    check_two_queues(&sample, &want);

    // 边界：只判空一次
    check_one_queue(&[Op::Empty], &["true"]);
    check_two_queues(&[Op::Empty], &["true"]);

    // 边界：压满再逐个弹出，顺序必须是后进先出
    let lifo = [
        Op::Push(1),
        Op::Push(2),
        Op::Push(3),
        Op::Pop,
        Op::Pop,
        Op::Pop,
        Op::Empty,
    ];
    let lifo_want = ["null", "null", "null", "3", "2", "1", "true"];
    check_one_queue(&lifo, &lifo_want);
    check_two_queues(&lifo, &lifo_want);

    // 随机交叉验证
    cross_check();

    println!("all tests passed");
}
