/*
============================================================================
155. 最小栈（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/min-stack/
分类：栈 · 设计（辅助栈同步记录最小值）

设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

实现 MinStack 类:
  · MinStack() 初始化堆栈对象。
  · void push(int value) 将元素 value 推入堆栈。
  · void pop() 删除堆栈顶部的元素。
  · int top() 获取堆栈顶部的元素。
  · int getMin() 获取堆栈中的最小元素。

示例 1:
输入：
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

输出：
[null,null,null,null,-3,null,0,-2]

解释：
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.

提示：
-2^31 <= val <= 2^31 - 1
pop、top 和 getMin 操作总是在 非空栈 上调用
push, pop, top, and getMin最多被调用 3 * 10^4 次
*/

/*
解法：辅助栈【最优解】

官方题解只给出这一种方法，也就是本题的最优解：四个操作全部 O(1)，空间 O(n)。

为什么慢：如果每次 get_min 都遍历一遍栈，单次就是 O(n)；若用一个变量只记「当前
最小值」，那么当这个最小值被 pop 掉之后，就再也找不回「第二小」，除非重新扫一遍。

换个角度：注意栈的先进后出性质 —— 元素 a 入栈时栈里已有 b、c、d，那么只要 a 还在栈中，
b、c、d 就一定还在。换句话说，「a 在栈中的这段期间，栈内最小值是固定的」。
于是可以让每个元素随身携带「它入栈那一刻的栈内最小值」，再把这份信息按相同的顺序
存进一个辅助栈。

关键点：
  · 辅助栈与数据栈严格同步：push 一起 push，pop 一起 pop，时刻保持一一对应
  · push 时压入 min(当前最小值, 新元素)；新元素不比当前最小值更小，就沿用旧的最小值
  · 辅助栈预置一个 i32::MAX 作为哨兵，省掉第一次 push 的判空特判
  · get_min 直接返回辅助栈栈顶，无需任何遍历
  · 题面保证 pop / top / get_min 都在非空栈上调用，所以不必额外判空

以示例（push -2、push 0、push -3、getMin、pop、top、getMin）为例：
  push(-2)：数据栈 [-2]，辅助栈 [i32::MAX, -2]
  push(0) ：数据栈 [-2, 0]，辅助栈 [..., -2, min(-2, 0) = -2]
  push(-3)：数据栈 [-2, 0, -3]，辅助栈 [..., -2, min(-2, -3) = -3]
  get_min → 辅助栈栈顶 -3
  pop     ：两栈同时弹出 → 数据栈 [-2, 0]，辅助栈 [..., -2]
  top     → 数据栈栈顶 0
  get_min → 辅助栈栈顶 -2

复杂度分析：
  时间：push / pop / top / get_min 均为 O(1)
  空间 O(n)：辅助栈与数据栈等长
*/
pub struct MinStack {
    data_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            data_stack: Vec::new(),
            // 哨兵：让第一次 push 不必特判空栈
            min_stack: vec![i32::MAX],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.data_stack.push(val);
        // 每个元素随身携带「入栈这一刻的栈内最小值」
        let cur_min = *self.min_stack.last().unwrap();
        self.min_stack.push(cur_min.min(val));
    }

    pub fn pop(&mut self) {
        self.data_stack.pop();
        // 两个栈必须同步弹出，才能保持一一对应
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        *self.data_stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
enum Op {
    Push(i32),
    Pop,
    Top,
    GetMin,
}

fn run(ops: &[Op]) -> Vec<String> {
    let mut obj = MinStack::new();
    let mut out = Vec::new();
    for &op in ops {
        match op {
            Op::Push(v) => {
                obj.push(v);
                out.push("null".to_string());
            }
            Op::Pop => {
                obj.pop();
                out.push("null".to_string());
            }
            Op::Top => out.push(obj.top().to_string()),
            Op::GetMin => out.push(obj.get_min().to_string()),
        }
    }
    out
}

fn check(ops: &[Op], want: &[&str]) {
    let want: Vec<String> = want.iter().map(|s| s.to_string()).collect();
    assert_eq!(run(ops), want, "MinStack 输出不符");
}

// 随机交叉验证：用 Vec 当对照，get_min 每次线性求最小
fn cross_check() {
    let mut min_stack = MinStack::new();
    let mut expect: Vec<i32> = Vec::new();
    let mut seed: u32 = 20240607;
    for _ in 0..3000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let r = (seed >> 16) % 10;
        if r < 5 || expect.is_empty() {
            let value = ((seed >> 8) % 200) as i32 - 100;
            min_stack.push(value);
            expect.push(value);
        } else if r < 7 {
            min_stack.pop();
            expect.pop();
        } else if r < 9 {
            assert_eq!(min_stack.top(), *expect.last().unwrap());
        } else {
            assert_eq!(min_stack.get_min(), *expect.iter().min().unwrap());
        }
    }
}

fn main() {
    // 示例 1
    check(
        &[
            Op::Push(-2),
            Op::Push(0),
            Op::Push(-3),
            Op::GetMin,
            Op::Pop,
            Op::Top,
            Op::GetMin,
        ],
        &["null", "null", "null", "-3", "null", "0", "-2"],
    );

    // 边界：单元素，top 与 get_min 相同
    check(&[Op::Push(7), Op::Top, Op::GetMin], &["null", "7", "7"]);

    // 边界：重复最小值，pop 一个后最小值仍应不变
    check(
        &[Op::Push(1), Op::Push(1), Op::GetMin, Op::Pop, Op::GetMin],
        &["null", "null", "1", "null", "1"],
    );

    // 边界：最小值被弹出后要能恢复到第二小
    check(
        &[
            Op::Push(3),
            Op::Push(1),
            Op::Push(2),
            Op::GetMin,
            Op::Pop,
            Op::Pop,
            Op::GetMin,
        ],
        &["null", "null", "null", "1", "null", "null", "3"],
    );

    // 边界：极端值
    check(
        &[Op::Push(i32::MAX), Op::Push(i32::MIN), Op::GetMin, Op::Pop, Op::GetMin],
        &[
            "null",
            "null",
            "-2147483648",
            "null",
            "2147483647",
        ],
    );

    // 随机交叉验证
    cross_check();

    println!("all tests passed");
}
