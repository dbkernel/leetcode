"""
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
  「栈顶」就是 size-1，没有任何额外状态；底层换成预分配的 list 顺序存储，
  再套上动态数组的翻倍扩容，就得到均摊 O(1) 的 push。

关键点（不变量）：
  · 0 <= size <= capacity，栈内元素是 _data[0..size-1]，栈顶是 _data[size-1]；
  · push 必须先扩容（size == capacity 时）再写 _data[size]；
  · pop 先判空、再取 _data[size-1]、最后才 size -= 1，顺序反了空栈就下溢；
  · 空栈取元素绝不能崩：C++ 用 bool + 出参、Go 用 (int, bool)、
    Python 抛 IndexError、Rust 返回 Option<i32>，四种写法是同一个语义
    「没有元素可取」，调用方必须自己处理这条失败路径。

手推一遍：
  依次 push 1、2、3：size 从 0 涨到 3，栈内容自底向上是 [1, 2, 3]。
  再 pop：取 _data[2] 得到 3，size 变 2，栈内容 [1, 2]；再 pop 得 2。
  括号匹配里输入 "([)]"：读到 ')' 时栈顶是 '['，与 ')' 不配对，
  立刻返回 False，而不是等扫完再看栈空——这一步省掉了后半段的无用功。

复杂度分析：
  时间：push 均摊 O(1)（扩容代价摊到每次 push），pop/peek/empty/size 都
    是 O(1)；括号匹配扫一遍字符串 O(n)，每个字符最多进出栈一次。
  空间：O(n)，capacity 最多是 size 的 2 倍。
  稳定性：栈是 LIFO，谈不到稳定性。
"""

STACK_INIT_CAP = 4


class Stack:
    def __init__(self):
        self._data = [0] * STACK_INIT_CAP
        self._size = 0

    def size(self):
        return self._size

    def capacity(self):
        return len(self._data)

    def empty(self):
        return self._size == 0

    def _grow(self):
        nd = [0] * (len(self._data) * 2)
        for i in range(self._size):
            nd[i] = self._data[i]
        self._data = nd

    def push(self, v):
        if self._size == len(self._data):
            self._grow()  # 先扩容，再写入
        self._data[self._size] = v
        self._size += 1

    def pop(self):
        # 空栈抛 IndexError，调用方必须自己处理，不能指望返回 0。
        if self._size == 0:
            raise IndexError("pop from empty stack")
        self._size -= 1
        return self._data[self._size]

    def peek(self):
        # 与 pop 的差别只有最后一句 self._size -= 1。
        if self._size == 0:
            raise IndexError("peek from empty stack")
        return self._data[self._size - 1]

    def clear(self):
        self._size = 0


def _matches(open_ch, close_ch):
    # 不用 dict，逐对比较，保持「只用 list 当裸数组」的约束。
    if open_ch == "(" and close_ch == ")":
        return True
    if open_ch == "[" and close_ch == "]":
        return True
    if open_ch == "{" and close_ch == "}":
        return True
    return False


def is_balanced(s):
    """应用：括号匹配。栈里存左括号的字符编码，用 int 统一类型。"""
    st = Stack()
    for c in s:
        if c == "(" or c == "[" or c == "{":
            st.push(ord(c))
        elif c == ")" or c == "]" or c == "}":
            if st.empty():
                return False  # 右括号多了
            if not _matches(chr(st.peek()), c):
                return False  # 类型不配对，提前失败
            st.pop()
    return st.empty()  # 左括号多了也会在这里失败


def main():
    # 用例 1：空栈的 pop / peek 抛 IndexError 而不是崩到别的异常。
    empty = Stack()
    assert empty.empty() and empty.size() == 0
    try:
        empty.pop()
        raise AssertionError("pop on empty should raise IndexError")
    except IndexError:
        pass
    try:
        empty.peek()
        raise AssertionError("peek on empty should raise IndexError")
    except IndexError:
        pass

    # 用例 2：连续 push 跨多次扩容，再一路 pop 回来。
    a = Stack()
    for i in range(100):
        a.push(i)
    assert a.size() == 100
    assert a.capacity() == 128  # 4 -> 8 -> ... -> 128
    assert not a.empty()
    assert a.peek() == 99
    assert a.size() == 100  # peek 不改 size
    # 关键点用例：push 与 pop 交错，size 与内容必须始终一致。
    a.push(100)
    assert a.pop() == 100
    assert a.pop() == 99
    assert a.size() == 99
    assert a.peek() == 98
    for i in range(98, -1, -1):
        assert a.pop() == i  # 必须是逆序弹出
    assert a.empty() and a.size() == 0
    try:
        a.pop()
        raise AssertionError("pop on drained stack should raise")
    except IndexError:
        pass

    # 用例 3：clear 之后还能继续用。
    a.push(1)
    a.push(2)
    a.clear()
    assert a.empty() and a.size() == 0
    a.push(7)
    assert a.pop() == 7

    # 用例 4：括号匹配的正例。
    for s in ["", "()", "()[]{}", "{[()]}", "((()))", "([]{}())"]:
        assert is_balanced(s), "should balance: %s" % s
    # 用例 5：括号匹配的反例——右括号过多、类型不匹配、左括号过多。
    for s in ["(", ")", "(]", "([)]", "(){[]}("]:
        assert not is_balanced(s), "should not balance: %s" % s

    print("all tests passed")


if __name__ == "__main__":
    main()
