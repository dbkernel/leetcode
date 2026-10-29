/*
============================================================================
顺序栈（Stack）
分类：手写数据结构 · 线性结构

操作复杂度：
  push    均摊 O(1)  底层动态数组满时 2 倍扩容，总搬移小于 2n
  pop     O(1)
  peek    O(1)       只看栈顶，不弹出
  empty   O(1)
  size    O(1)
  clear   O(1)       只把 size 置 0
============================================================================

用途：
  后进先出的最简容器，也是很多算法的零件：括号匹配、表达式求值、
  单调栈（下一个更大元素）、深搜/回溯、把递归改写成迭代。

核心思想：
  只用一端进出：push 写在 size 位置，pop 读 size-1 再让 size 减一。
  「栈顶」就是 size-1，没有任何额外状态；底层换成 Vec 顺序存储，
  再套上动态数组的翻倍扩容，就得到均摊 O(1) 的 push。

关键点（不变量）：
  · 0 <= size <= capacity，栈内元素是 data[0..size-1]，栈顶是 data[size-1]；
  · push 必须先扩容（size == capacity 时）再写 data[size]；
  · pop 先判空、再取 data[size-1]、最后才 size -= 1，顺序反了空栈就下溢；
  · 空栈取元素绝不能崩：C++ 用 bool + 出参、Go 用 (int, bool)、
    Python 抛 IndexError、Rust 返回 Option<i32>，四种写法是同一个语义
    「没有元素可取」，调用方必须自己处理这条失败路径。

手推一遍：
  依次 push 1、2、3：size 从 0 涨到 3，栈内容自底向上是 [1, 2, 3]。
  再 pop：取 data[2] 得到 3，size 变 2，栈内容 [1, 2]；再 pop 得 2。
  括号匹配里输入 "([)]"：读到 ')' 时栈顶是 '['，与 ')' 不配对，
  立刻返回 false，而不是等扫完再看栈空——这一步省掉了后半段的无用功。

复杂度分析：
  时间：push 均摊 O(1)（扩容代价摊到每次 push），pop/peek/empty/size 都
    是 O(1)；括号匹配扫一遍字符串 O(n)，每个字符最多进出栈一次。
  空间：O(n)，capacity 最多是 size 的 2 倍。
  稳定性：栈是 LIFO，谈不到稳定性。
*/

#![allow(dead_code)]

const STACK_INIT_CAP: usize = 4;

struct Stack {
    data: Vec<i32>, // 长度当 capacity，前 size 个是有效元素
    size: usize,
}

impl Stack {
    fn new() -> Self {
        Stack {
            data: vec![0; STACK_INIT_CAP],
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    fn grow(&mut self) {
        let mut nd = vec![0; self.data.len() * 2];
        nd[..self.size].copy_from_slice(&self.data[..self.size]);
        self.data = nd;
    }

    fn push(&mut self, v: i32) {
        if self.size == self.data.len() {
            self.grow(); // 先扩容，再写入
        }
        self.data[self.size] = v;
        self.size += 1;
    }

    // 空栈返回 None，调用方必须自己处理这条失败路径。
    fn pop(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        Some(self.data[self.size])
    }

    // 与 pop 的差别只有最后一句 size -= 1。
    fn peek(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        Some(self.data[self.size - 1])
    }

    fn clear(&mut self) {
        self.size = 0;
    }
}

// 应用：括号匹配。栈里存左括号的字符编码，用 i32 统一类型。
fn is_balanced(s: &str) -> bool {
    let mut st = Stack::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => st.push(c as i32),
            ')' | ']' | '}' => {
                let top = match st.peek() {
                    Some(v) => v,    // 右括号多了：没有可配的左括号
                    None => return false,
                };
                let open = top as u8 as char;
                // 类型不配对就提前失败，不必扫完
                let ok = (open == '(' && c == ')')
                    || (open == '[' && c == ']')
                    || (open == '{' && c == '}');
                if !ok {
                    return false;
                }
                st.pop();
            }
            _ => {}
        }
    }
    st.empty() // 左括号多了也会在这里失败
}

fn main() {
    // 用例 1：空栈的 pop / peek 返回 None 而不是崩。
    let mut empty = Stack::new();
    assert!(empty.empty() && empty.size() == 0);
    assert_eq!(empty.pop(), None);
    assert_eq!(empty.peek(), None);

    // 用例 2：连续 push 跨多次扩容，再一路 pop 回来。
    let mut a = Stack::new();
    for i in 0..100 {
        a.push(i);
    }
    assert_eq!(a.size(), 100);
    assert_eq!(a.capacity(), 128); // 4 -> 8 -> ... -> 128
    assert!(!a.empty());
    assert_eq!(a.peek(), Some(99));
    assert_eq!(a.size(), 100); // peek 不改 size
    // 关键点用例：push 与 pop 交错，size 与内容必须始终一致。
    a.push(100);
    assert_eq!(a.pop(), Some(100));
    assert_eq!(a.pop(), Some(99));
    assert_eq!(a.size(), 99);
    assert_eq!(a.peek(), Some(98));
    for i in (0..=98).rev() {
        assert_eq!(a.pop(), Some(i)); // 必须是逆序弹出
    }
    assert!(a.empty() && a.size() == 0);
    assert_eq!(a.pop(), None);

    // 用例 3：clear 之后还能继续用。
    a.push(1);
    a.push(2);
    a.clear();
    assert!(a.empty() && a.size() == 0);
    a.push(7);
    assert_eq!(a.pop(), Some(7));

    // 用例 4：括号匹配的正例。
    for s in ["", "()", "()[]{}", "{[()]}", "((()))", "([]{}())"] {
        assert!(is_balanced(s), "should balance: {}", s);
    }
    // 用例 5：括号匹配的反例——右括号过多、类型不匹配、左括号过多。
    for s in ["(", ")", "(]", "([)]", "(){[]}("] {
        assert!(!is_balanced(s), "should not balance: {}", s);
    }

    println!("all tests passed");
}
