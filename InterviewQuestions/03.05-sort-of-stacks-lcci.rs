/*
============================================================================
面试题 03.05. 栈排序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-of-stacks-lcci/
分类：栈 · 设计（单调栈 / 一个辅助栈排序）

栈排序。 编写程序，对栈进行排序使最小元素位于栈顶。最多只能使用一个其他的临时栈
存放数据，但不得将元素复制到别的数据结构（如数组）中。该栈支持如下操作：push、
pop、peek 和 isEmpty。当栈为空时，peek 返回 -1。

示例 1：
输入：
["SortedStack", "push", "push", "peek", "pop", "peek"]
[[], [1], [2], [], [], []]
输出：
[null,null,null,1,null,2]

示例 2：
输入：
["SortedStack", "pop", "pop", "push", "pop", "isEmpty"]
[[], [], [], [1], [], []]
输出：
[null,null,null,null,null,true]

提示：
栈中的元素数目在[0, 5000]范围内。
*/

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出两种标准实现（都只用一个临时栈，
不借助数组等其他数据结构），并把实践中更常用的「push 时维护有序」标为最优解。

解法一：push 时用辅助栈插入排序【最优解】

保持主栈 s1 从栈顶到栈底单调递增 —— 即栈顶永远是最小元素。于是 pop / peek /
is_empty 全都可以直接 O(1) 完成，代价压在 push 上：
  1. 把 s1 中所有「比 val 小」的元素挪到辅助栈 s2（它们会被 val 压得更深）；
  2. 把 val 压入 s1；
  3. 再把 s2 里的元素全部搬回 s1。
搬完后 s1 依然「顶小底大」，不变量得以维持。

关键点：
  · 比较用 s1 栈顶 < val（严格小于）：相等时停下即可，相等元素谁在上都不破坏有序
  · 第 3 步必须把 s2 全部搬回，否则 s2 里还留着数据，下次 push 会乱序
  · 主栈为空时 peek 返回 -1，pop 什么也不做（题面要求）
  · 全程只用两个栈，没有借助数组，符合题面限制

以 push(1)、push(2) 为例：
  push(1)：s1 空 → s1 = [1]
  push(2)：1 < 2，把 1 挪到 s2；s1 空 → 压入 2，s1 = [2]；搬回 1 → s1 = [2, 1]
           （栈顶 1 是最小值）
  peek → 1；pop → 弹出 1；peek → 2

复杂度分析：
  时间：push 最坏 O(n)（要挪动并搬回 n 个元素），pop / peek / is_empty 均 O(1)
  空间 O(n)：两个栈合计存放 n 个元素

解法二：pop / peek 时才整理（惰性排序）【非最优】

不变量不变（栈顶最小），但把排序推迟到真正需要读数据的时候：push 只入栈并打上
「脏」标记；pop / peek 前若为脏，就用辅助栈 s2 做一次插入排序，把 s1 整理有序。

关键点：
  · 整理时先把 s1 全部倒进 s2（顺序被反转），再逐个从 s2 取出，边取边插入到 s1 的
    正确位置：比当前元素小的一律先挪回 s2，插入后再搬回来
  · 整理完 s1 有序、s2 必为空，脏标记清除
  · 排序后连续 pop 不需要重排 —— 从有序栈顶弹元素不会破坏有序性

复杂度分析：
  时间：push O(1)，pop / peek 最坏 O(n^2)（一次整理要插入 n 个元素）
  空间 O(n)
  由于 pop / peek 往往比 push 调得更频繁，这种「把代价推到读操作」的做法整体更差，
  因此解法一才是最常用的写法。
*/

// 解法一：push 时维护有序（栈顶最小）
pub struct SortedStack {
    s1: Vec<i32>, // 主栈，Vec 末尾是栈顶，恒为最小
    s2: Vec<i32>, // 临时栈
}

impl SortedStack {
    pub fn new() -> Self {
        SortedStack {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        // 比 val 小的元素要被压得更深，先挪到辅助栈
        while let Some(&top) = self.s1.last() {
            if top < val {
                self.s2.push(top);
                self.s1.pop();
            } else {
                break;
            }
        }
        self.s1.push(val);
        // 辅助栈里的元素全部搬回，维持「顶小底大」
        while let Some(top) = self.s2.pop() {
            self.s1.push(top);
        }
    }

    pub fn pop(&mut self) {
        self.s1.pop();
    }

    pub fn peek(&self) -> i32 {
        *self.s1.last().unwrap_or(&-1)
    }

    pub fn is_empty(&self) -> bool {
        self.s1.is_empty()
    }
}

impl Default for SortedStack {
    fn default() -> Self {
        Self::new()
    }
}

// 解法二：pop / peek 时才整理（惰性排序）
pub struct SortedStackLazy {
    s1: Vec<i32>,
    s2: Vec<i32>,
    dirty: bool,
}

impl SortedStackLazy {
    pub fn new() -> Self {
        SortedStackLazy {
            s1: Vec::new(),
            s2: Vec::new(),
            dirty: false,
        }
    }

    pub fn push(&mut self, val: i32) {
        self.s1.push(val);
        self.dirty = true;
    }

    pub fn pop(&mut self) {
        if self.s1.is_empty() {
            return;
        }
        self.sort_if_needed();
        self.s1.pop();
    }

    pub fn peek(&mut self) -> i32 {
        if self.s1.is_empty() {
            return -1;
        }
        self.sort_if_needed();
        *self.s1.last().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.s1.is_empty()
    }

    fn sort_if_needed(&mut self) {
        if !self.dirty {
            return;
        }
        // 先把 s1 全部倒进 s2（顺序反转）
        while let Some(top) = self.s1.pop() {
            self.s2.push(top);
        }
        // 再逐个插入到 s1 的正确位置：比 t 小的元素先挪回 s2
        while let Some(t) = self.s2.pop() {
            while let Some(&top) = self.s1.last() {
                if top < t {
                    self.s2.push(top);
                    self.s1.pop();
                } else {
                    break;
                }
            }
            self.s1.push(t);
        }
        self.dirty = false;
    }
}

impl Default for SortedStackLazy {
    fn default() -> Self {
        Self::new()
    }
}

// 一次操作：Push/Pop/Peek/IsEmpty
#[derive(Clone, Copy)]
enum Op {
    Push(i32),
    Pop,
    Peek,
    IsEmpty,
}

fn run_main(stack: &mut SortedStack, ops: &[Op]) -> Vec<String> {
    let mut out = Vec::new();
    for &op in ops {
        match op {
            Op::Push(v) => {
                stack.push(v);
                out.push("null".to_string());
            }
            Op::Pop => {
                stack.pop();
                out.push("null".to_string());
            }
            Op::Peek => out.push(stack.peek().to_string()),
            Op::IsEmpty => out.push(stack.is_empty().to_string()),
        }
    }
    out
}

fn run_lazy(stack: &mut SortedStackLazy, ops: &[Op]) -> Vec<String> {
    let mut out = Vec::new();
    for &op in ops {
        match op {
            Op::Push(v) => {
                stack.push(v);
                out.push("null".to_string());
            }
            Op::Pop => {
                stack.pop();
                out.push("null".to_string());
            }
            Op::Peek => out.push(stack.peek().to_string()),
            Op::IsEmpty => out.push(stack.is_empty().to_string()),
        }
    }
    out
}

fn check(ops: &[Op], want: &[&str]) {
    let want: Vec<String> = want.iter().map(|s| s.to_string()).collect();
    let mut main = SortedStack::new();
    let mut lazy = SortedStackLazy::new();
    assert_eq!(run_main(&mut main, ops), want, "SortedStack 输出不符");
    assert_eq!(run_lazy(&mut lazy, ops), want, "SortedStackLazy 输出不符");
}

// 随机交叉验证：每次 peek / pop 都和「排序后的 Vec」比对
fn cross_check() {
    let mut main = SortedStack::new();
    let mut lazy = SortedStackLazy::new();
    let mut expect: Vec<i32> = Vec::new(); // 降序，末尾是最小值（即栈顶）
    let mut seed: u32 = 20240607;
    for _ in 0..3000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let r = (seed >> 16) % 10;
        if r < 5 || expect.is_empty() {
            let value = ((seed >> 8) % 20) as i32;
            main.push(value);
            lazy.push(value);
            expect.push(value);
            expect.sort_by(|a, b| b.cmp(a));
        } else if r < 8 {
            main.pop();
            lazy.pop();
            expect.pop();
        } else if r < 9 {
            let want = *expect.last().unwrap();
            assert_eq!(main.peek(), want);
            assert_eq!(lazy.peek(), want);
        } else {
            assert_eq!(main.is_empty(), expect.is_empty());
            assert_eq!(lazy.is_empty(), expect.is_empty());
        }
    }
}

fn main() {
    // 示例 1
    check(
        &[Op::Push(1), Op::Push(2), Op::Peek, Op::Pop, Op::Peek],
        &["null", "null", "1", "null", "2"],
    );

    // 示例 2：空栈 pop 不报错，最终为空
    check(
        &[Op::Pop, Op::Pop, Op::Push(1), Op::Pop, Op::IsEmpty],
        &["null", "null", "null", "null", "true"],
    );

    // 边界：空栈 peek 返回 -1
    check(&[Op::Peek], &["-1"]);

    // 边界：含重复元素
    check(
        &[
            Op::Push(3),
            Op::Push(1),
            Op::Push(3),
            Op::Push(1),
            Op::Peek,
            Op::Pop,
            Op::Peek,
            Op::Pop,
            Op::Peek,
            Op::Pop,
            Op::Peek,
        ],
        &["null", "null", "null", "null", "1", "null", "1", "null", "3", "null", "3"],
    );

    // 边界：连续入栈的逆序数据，每次 peek 都是当前最小值
    check(
        &[
            Op::Push(5),
            Op::Push(4),
            Op::Push(3),
            Op::Push(2),
            Op::Push(1),
            Op::Peek,
            Op::Pop,
            Op::Peek,
            Op::Pop,
            Op::Peek,
        ],
        &["null", "null", "null", "null", "null", "1", "null", "2", "null", "3"],
    );

    // 随机交叉验证
    cross_check();

    println!("all tests passed");
}
