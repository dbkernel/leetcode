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
  「栈顶」就是 size-1，没有任何额外状态；底层换成数组的顺序存储，
  再套上动态数组的翻倍扩容，就得到均摊 O(1) 的 push。

关键点（不变量）：
  · 0 <= size <= capacity，栈内元素是 data[0..size-1]，栈顶是 data[size-1]；
  · push 必须先扩容（size == capacity 时）再写 data[size]；
  · pop 先判空、再取 data[size-1]、最后才 --size，顺序反了空栈就下溢；
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

#include <cassert>
#include <iostream>
#include <string>

const int kStackInitCap = 4;

class Stack {
public:
    Stack() : data_(new int[kStackInitCap]()), size_(0), cap_(kStackInitCap) {}

    Stack(const Stack&) = delete;
    Stack& operator=(const Stack&) = delete;

    ~Stack() { delete[] data_; }

    int size() const { return size_; }
    int capacity() const { return cap_; }
    bool empty() const { return size_ == 0; }

    void push(int v) {
        if (size_ == cap_) grow();  // 先扩容，再写入
        data_[size_++] = v;
    }

    // 空栈返回 false，且不改动 out；成功则把栈顶写进 out。
    bool pop(int& out) {
        if (size_ == 0) return false;
        out = data_[size_ - 1];
        --size_;
        return true;
    }

    // 与 pop 的差别只有最后一句 --size_。
    bool peek(int& out) const {
        if (size_ == 0) return false;
        out = data_[size_ - 1];
        return true;
    }

    void clear() { size_ = 0; }

private:
    void grow() {
        int new_cap = cap_ * 2;
        int* nd = new int[new_cap]();
        for (int i = 0; i < size_; ++i) nd[i] = data_[i];
        delete[] data_;
        data_ = nd;
        cap_ = new_cap;
    }

    int* data_;
    int size_;
    int cap_;
};

// 应用：括号匹配。栈里存的是左括号的字符编码，用 int 统一类型。
bool is_balanced(const std::string& s) {
    Stack st;
    for (char c : s) {
        if (c == '(' || c == '[' || c == '{') {
            st.push(static_cast<int>(c));
        } else if (c == ')' || c == ']' || c == '}') {
            int top = 0;
            if (!st.peek(top)) return false;  // 右括号多了
            char open = static_cast<char>(top);
            bool ok = (open == '(' && c == ')') || (open == '[' && c == ']') ||
                      (open == '{' && c == '}');
            if (!ok) return false;  // 类型不配对，提前失败
            st.pop(top);
        }
    }
    return st.empty();  // 左括号多了也会在这里失败
}

int main() {
    // 用例 1：空栈的 pop / peek 返回失败而不是崩溃。
    Stack empty;
    int out = -12345;
    assert(empty.empty() && empty.size() == 0);
    assert(!empty.pop(out));
    assert(out == -12345);  // 失败时不能顺手把出参改掉
    assert(!empty.peek(out));
    assert(out == -12345);

    // 用例 2：连续 push 跨多次扩容，再一路 pop 回来。
    Stack a;
    for (int i = 0; i < 100; ++i) a.push(i);
    assert(a.size() == 100);
    assert(a.capacity() == 128);  // 4 -> 8 -> ... -> 128
    assert(!a.empty());
    int top = 0;
    assert(a.peek(top) && top == 99);
    assert(a.size() == 100);  // peek 不改 size
    // 关键点用例：push 与 pop 交错，size 与内容必须始终一致。
    a.push(100);
    assert(a.pop(top) && top == 100);
    assert(a.pop(top) && top == 99);
    assert(a.size() == 99);
    assert(a.peek(top) && top == 98);
    for (int i = 98; i >= 0; --i) {
        assert(a.pop(top));
        assert(top == i);  // 必须是逆序弹出
    }
    assert(a.empty() && a.size() == 0);
    assert(!a.pop(top));

    // 用例 3：clear 之后还能继续用。
    a.push(1);
    a.push(2);
    a.clear();
    assert(a.empty() && a.size() == 0);
    a.push(7);
    assert(a.pop(top) && top == 7);

    // 用例 4：括号匹配的正例。
    assert(is_balanced(""));
    assert(is_balanced("()"));
    assert(is_balanced("()[]{}"));
    assert(is_balanced("{[()]}"));   // 嵌套
    assert(is_balanced("((()))"));
    assert(is_balanced("([]{}({}))"));
    // 用例 5：括号匹配的反例——右括号过多、类型不匹配、左括号过多。
    assert(!is_balanced("("));
    assert(!is_balanced(")"));
    assert(!is_balanced("(]"));
    assert(!is_balanced("([)]"));    // 交叉嵌套
    assert(!is_balanced("(){[]}("));

    std::cout << "all tests passed\n";
    return 0;
}
