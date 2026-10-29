/*
============================================================================
面试题 03.01. 三合一（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/three-in-one-lcci/
分类：栈 · 设计（一个数组实现三个栈）

三合一。描述如何只用一个数组来实现三个栈。

你应该实现 push(stackNum, value)、pop(stackNum)、isEmpty(stackNum)、
peek(stackNum) 方法。stackNum 表示栈下标，value 表示压入的值。

构造函数会传入一个 stackSize 参数，代表每个栈的大小。

示例 1：
输入：
["TripleInOne", "push", "push", "pop", "pop", "pop", "isEmpty"]
[[1], [0, 1], [0, 2], [0], [0], [0], [0]]
输出：
[null, null, null, 1, -1, -1, true]
说明：当栈为空时 pop, peek 返回 -1，当栈满时 push 不压入元素。

示例 2：
输入：
["TripleInOne", "push", "push", "push", "pop", "pop", "pop", "peek"]
[[2], [0, 1], [0, 2], [0, 3], [0], [0], [0], [0]]
输出：
[null, null, null, null, 2, 1, -1, -1]

提示：
0 <= stackNum <= 2
*/

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面按《程序员面试金典》的标准做法给出
两种实现，并把真正满足「只用一个数组」的那一种标为最优解。

解法一：三个独立数组【非最优，不满足题意】

最直白的写法就是开三个 Vec，每个独立管理自己的栈顶。push / pop / peek /
isEmpty 都是 O(1)，但用了三块数组，与题面「只用一个数组」的要求不符，这里只作为
对照，用来说明为什么需要「分区 + 下标换算」。

复杂度分析：
  时间：四个操作均 O(1)
  空间：总 O(3 * stackSize)，但分散在三个数组里

解法二：一个数组分三段 + 三个栈顶指针【最优解】

既然每个栈的大小固定为 stackSize，那就把长度为 3 * stackSize 的一个数组切成三段：
第 s 个栈独占 [s * stackSize, (s + 1) * stackSize) 这一段。
再维护三个「栈顶下标」top[s]，初值取 s * stackSize - 1 —— 这样偶数段时 top[s] 是
段首前一格，「top[s] >= s * stackSize」就等价于「第 s 个栈非空」。

关键点：
  · 判空用 top[s] >= s * stackSize（等价于 top[s] - s * stackSize >= 0），
    不要写成 top[s] >= 0，否则第 1、2 段会被误判为非空
  · 判满用 top[s] < (s + 1) * stackSize - 1，满了就丢弃新元素（题面要求「当栈满时
    push 不压入元素」），不要扩容
  · stackSize == 0 是合法输入：此时数组为空，push 一律被丢弃，pop / peek 一律返回 -1
  · 三个栈共用一块内存但互不越界，这就是「一个数组实现三个栈」的标准答案

以示例 2（stackSize = 2）为例，数组长 6：
  push(0, 1)：top[0] 由 -1 变 0，data[0] = 1
  push(0, 2)：top[0] 变 1，data[1] = 2（第 0 段 [0, 1] 已满）
  push(0, 3)：top[0] = 1 == (0+1)*2 - 1，已满 → 丢弃
  pop(0) → 返回 data[1] = 2，top[0] 变 0
  pop(0) → 返回 data[0] = 1，top[0] 变 -1（空）
  pop(0) → 空 → 返回 -1
  peek(0) → 空 → 返回 -1

复杂度分析：
  时间：四个操作均 O(1)
  空间 O(3 * stackSize)：一个数组 + 三个下标
*/

// 解法二：一个数组分三段 + 三个栈顶指针
pub struct TripleInOne {
    stack_size: i32,
    data: Vec<i32>, // 唯一的数组，切三段给三个栈用
    top: [i32; 3],  // 每个栈的栈顶下标，空栈时是段首前一格
}

impl TripleInOne {
    pub fn new(stack_size: i32) -> Self {
        let n = if stack_size > 0 { 3 * stack_size } else { 0 };
        TripleInOne {
            stack_size,
            data: vec![0; n as usize],
            top: [-1, stack_size - 1, 2 * stack_size - 1],
        }
    }

    pub fn push(&mut self, stack_num: i32, value: i32) {
        let s = stack_num as usize;
        // 满了就丢弃（题面要求），不扩容
        if self.top[s] < (stack_num + 1) * self.stack_size - 1 {
            self.top[s] += 1;
            self.data[self.top[s] as usize] = value;
        }
    }

    pub fn pop(&mut self, stack_num: i32) -> i32 {
        if self.is_empty(stack_num) {
            return -1;
        }
        let s = stack_num as usize;
        let value = self.data[self.top[s] as usize];
        self.top[s] -= 1;
        value
    }

    pub fn peek(&self, stack_num: i32) -> i32 {
        if self.is_empty(stack_num) {
            return -1;
        }
        self.data[self.top[stack_num as usize] as usize]
    }

    pub fn is_empty(&self, stack_num: i32) -> bool {
        self.top[stack_num as usize] < stack_num * self.stack_size
    }
}

// 解法一：三个独立 Vec（仅作对照，不满足「只用一个数组」的要求）
pub struct TripleInOneThreeArrays {
    stack_size: usize,
    data: [Vec<i32>; 3],
}

impl TripleInOneThreeArrays {
    pub fn new(stack_size: i32) -> Self {
        TripleInOneThreeArrays {
            stack_size: stack_size.max(0) as usize,
            data: [Vec::new(), Vec::new(), Vec::new()],
        }
    }

    pub fn push(&mut self, stack_num: i32, value: i32) {
        let s = stack_num as usize;
        if self.data[s].len() < self.stack_size {
            self.data[s].push(value);
        }
    }

    pub fn pop(&mut self, stack_num: i32) -> i32 {
        self.data[stack_num as usize].pop().unwrap_or(-1)
    }

    pub fn peek(&self, stack_num: i32) -> i32 {
        *self.data[stack_num as usize].last().unwrap_or(&-1)
    }

    pub fn is_empty(&self, stack_num: i32) -> bool {
        self.data[stack_num as usize].is_empty()
    }
}

// 一次操作：Push/Pop/Peek/IsEmpty，附栈下标与 push 的值
#[derive(Clone, Copy)]
enum Op {
    Push(i32, i32),
    Pop(i32),
    Peek(i32),
    IsEmpty(i32),
}

fn run_one(t: &mut TripleInOne, ops: &[Op]) -> Vec<String> {
    let mut out = Vec::new();
    for &op in ops {
        match op {
            Op::Push(s, v) => {
                t.push(s, v);
                out.push("null".to_string());
            }
            Op::Pop(s) => out.push(t.pop(s).to_string()),
            Op::Peek(s) => out.push(t.peek(s).to_string()),
            Op::IsEmpty(s) => out.push(t.is_empty(s).to_string()),
        }
    }
    out
}

fn run_three(t: &mut TripleInOneThreeArrays, ops: &[Op]) -> Vec<String> {
    let mut out = Vec::new();
    for &op in ops {
        match op {
            Op::Push(s, v) => {
                t.push(s, v);
                out.push("null".to_string());
            }
            Op::Pop(s) => out.push(t.pop(s).to_string()),
            Op::Peek(s) => out.push(t.peek(s).to_string()),
            Op::IsEmpty(s) => out.push(t.is_empty(s).to_string()),
        }
    }
    out
}

fn check(stack_size: i32, ops: &[Op], want: &[&str]) {
    let mut one = TripleInOne::new(stack_size);
    let mut three = TripleInOneThreeArrays::new(stack_size);
    let want: Vec<String> = want.iter().map(|s| s.to_string()).collect();
    assert_eq!(run_one(&mut one, ops), want, "TripleInOne 输出不符");
    assert_eq!(run_three(&mut three, ops), want, "TripleInOneThreeArrays 输出不符");
}

// 随机交叉验证：用三个 Vec 当对照栈
fn cross_check(stack_size: i32) {
    let mut one = TripleInOne::new(stack_size);
    let mut three = TripleInOneThreeArrays::new(stack_size);
    let mut expect: [Vec<i32>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut seed: u32 = 20240607 + stack_size as u32;
    for _ in 0..3000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let r = (seed >> 16) % 10;
        let s = ((seed >> 8) % 3) as usize;
        if r < 5 {
            let value = ((seed >> 4) % 100) as i32;
            one.push(s as i32, value);
            three.push(s as i32, value);
            // 对照栈同样按「满了丢弃」
            if expect[s].len() < stack_size.max(0) as usize {
                expect[s].push(value);
            }
        } else if r < 8 {
            let want = expect[s].pop().unwrap_or(-1);
            assert_eq!(one.pop(s as i32), want);
            assert_eq!(three.pop(s as i32), want);
        } else if r < 9 {
            let want = *expect[s].last().unwrap_or(&-1);
            assert_eq!(one.peek(s as i32), want);
            assert_eq!(three.peek(s as i32), want);
        } else {
            assert_eq!(one.is_empty(s as i32), expect[s].is_empty());
            assert_eq!(three.is_empty(s as i32), expect[s].is_empty());
        }
    }
}

fn main() {
    // 示例 1：stackSize = 1，push 第二个元素被丢弃
    check(
        1,
        &[
            Op::Push(0, 1),
            Op::Push(0, 2),
            Op::Pop(0),
            Op::Pop(0),
            Op::Pop(0),
            Op::IsEmpty(0),
        ],
        &["null", "null", "1", "-1", "-1", "true"],
    );

    // 示例 2：stackSize = 2，压三个只留两个
    check(
        2,
        &[
            Op::Push(0, 1),
            Op::Push(0, 2),
            Op::Push(0, 3),
            Op::Pop(0),
            Op::Pop(0),
            Op::Pop(0),
            Op::Peek(0),
        ],
        &["null", "null", "null", "2", "1", "-1", "-1"],
    );

    // 边界：stackSize = 0，任何 push 都丢弃、任何取数都返回 -1
    check(
        0,
        &[Op::Push(1, 7), Op::Pop(1), Op::Peek(1), Op::IsEmpty(1)],
        &["null", "-1", "-1", "true"],
    );

    // 边界：三个栈互不干扰，各自后进先出
    check(
        2,
        &[
            Op::Push(0, 1),
            Op::Push(1, 2),
            Op::Push(2, 3),
            Op::Push(1, 4),
            Op::Pop(1),
            Op::Pop(0),
            Op::Pop(2),
            Op::Pop(1),
            Op::Pop(0),
        ],
        &["null", "null", "null", "null", "4", "1", "3", "2", "-1"],
    );

    // 随机交叉验证
    for stack_size in [0, 1, 3, 10] {
        cross_check(stack_size);
    }

    println!("all tests passed");
}
