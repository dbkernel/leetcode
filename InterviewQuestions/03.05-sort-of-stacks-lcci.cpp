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
#include <algorithm>
#include <cassert>
#include <iostream>
#include <stack>
#include <string>
#include <vector>
using namespace std;

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出两种标准实现（都只用一个临时栈，
不借助数组等其他数据结构），并把实践中更常用的「push 时维护有序」标为最优解。

解法一：push 时用辅助栈插入排序【最优解】

保持主栈 s1 从栈顶到栈底单调递增 —— 即栈顶永远是最小元素。于是 pop / peek /
isEmpty 全都可以直接 O(1) 完成，代价压在 push 上：
  1. 把 s1 中所有「比 val 小」的元素挪到辅助栈 s2（它们会被 val 压得更深）；
  2. 把 val 压入 s1；
  3. 再把 s2 里的元素全部搬回 s1。
搬完后 s1 依然「顶小底大」，不变量得以维持。

关键点：
  · 比较用 s1.top() < val（严格小于）：相等时停下即可，相等元素谁在上都不破坏有序
  · 第 3 步必须把 s2 全部搬回，否则 s2 里还留着数据，下次 push 会乱序
  · 主栈为空时 peek 返回 -1，pop 什么也不做（题面要求）
  · 全程只用两个栈，没有借助数组，符合题面限制

以 push(1)、push(2) 为例：
  push(1)：s1 空 → s1 = [1]
  push(2)：1 < 2，把 1 挪到 s2；s1 空 → 压入 2，s1 = [2]；搬回 1 → s1 = [2, 1]
           （栈顶 1 是最小值）
  peek → 1；pop → 弹出 1；peek → 2

复杂度分析：
  时间：push 最坏 O(n)（要挪动并搬回 n 个元素），pop / peek / isEmpty 均 O(1)
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
class SortedStack
{
public:
    SortedStack() {}

    void push(int val)
    {
        // 比 val 小的元素要被压得更深，先挪到辅助栈
        while (!s1.empty() && s1.top() < val)
        {
            s2.push(s1.top());
            s1.pop();
        }
        s1.push(val);
        // 辅助栈里的元素全部搬回，维持「顶小底大」
        while (!s2.empty())
        {
            s1.push(s2.top());
            s2.pop();
        }
    }

    void pop()
    {
        if (!s1.empty()) s1.pop();
    }

    int peek() { return s1.empty() ? -1 : s1.top(); }

    bool isEmpty() { return s1.empty(); }

private:
    stack<int> s1; // 主栈，栈顶最小
    stack<int> s2; // 临时栈
};

// 解法二：pop / peek 时才整理（惰性排序）
class SortedStackLazy
{
public:
    SortedStackLazy() {}

    void push(int val)
    {
        s1.push(val);
        dirty = true;
    }

    void pop()
    {
        if (s1.empty()) return;
        sortIfNeeded();
        s1.pop();
    }

    int peek()
    {
        if (s1.empty()) return -1;
        sortIfNeeded();
        return s1.top();
    }

    bool isEmpty() { return s1.empty(); }

private:
    void sortIfNeeded()
    {
        if (!dirty) return;
        // 先把 s1 全部倒进 s2（顺序反转）
        while (!s1.empty())
        {
            s2.push(s1.top());
            s1.pop();
        }
        // 再逐个插入到 s1 的正确位置：比 t 小的元素先挪回 s2
        while (!s2.empty())
        {
            int t = s2.top();
            s2.pop();
            while (!s1.empty() && s1.top() < t)
            {
                s2.push(s1.top());
                s1.pop();
            }
            s1.push(t);
        }
        dirty = false;
    }

    stack<int> s1;
    stack<int> s2;
    bool dirty = false;
};

namespace
{
struct Op
{
    string name;
    int value;
};

template <typename Stack>
vector<string> run(const vector<Op> &ops)
{
    Stack stack;
    vector<string> out;
    for (const auto &op : ops)
    {
        if (op.name == "push")
        {
            stack.push(op.value);
            out.push_back("null");
        }
        else if (op.name == "pop")
        {
            stack.pop();
            out.push_back("null");
        }
        else if (op.name == "peek")
        {
            out.push_back(to_string(stack.peek()));
        }
        else
        {
            out.push_back(stack.isEmpty() ? "true" : "false");
        }
    }
    return out;
}

template <typename Stack>
void check(const vector<Op> &ops, const vector<string> &want)
{
    vector<string> got = run<Stack>(ops);
    assert(got == want);
}

// 随机交叉验证：每次 peek / pop 都和「排序后的 vector」比对
template <typename Stack>
void crossCheck()
{
    Stack stack;
    vector<int> expect; // 升序，最小值在末尾（即栈顶）
    unsigned seed = 20240607;
    for (int i = 0; i < 3000; ++i)
    {
        seed = seed * 1103515245 + 12345;
        int r = (seed >> 16) % 10;
        if (r < 5 || expect.empty())
        {
            int value = (seed >> 8) % 20;
            stack.push(value);
            expect.push_back(value);
            sort(expect.begin(), expect.end(), greater<int>());
        }
        else if (r < 8)
        {
            stack.pop();
            expect.pop_back();
        }
        else if (r < 9)
        {
            assert(stack.peek() == expect.back());
        }
        else
        {
            assert(stack.isEmpty() == expect.empty());
        }
    }
}
} // namespace

int main()
{
    // 示例 1
    vector<Op> ops1 = {{"push", 1}, {"push", 2}, {"peek", 0},
                       {"pop", 0},   {"peek", 0}};
    vector<string> want1 = {"null", "null", "1", "null", "2"};
    check<SortedStack>(ops1, want1);
    check<SortedStackLazy>(ops1, want1);

    // 示例 2：空栈 pop 不报错，最终为空
    vector<Op> ops2 = {{"pop", 0}, {"pop", 0}, {"push", 1},
                       {"pop", 0}, {"isEmpty", 0}};
    vector<string> want2 = {"null", "null", "null", "null", "true"};
    check<SortedStack>(ops2, want2);
    check<SortedStackLazy>(ops2, want2);

    // 边界：空栈 peek 返回 -1
    check<SortedStack>({{"peek", 0}}, {"-1"});
    check<SortedStackLazy>({{"peek", 0}}, {"-1"});

    // 边界：含重复元素
    vector<Op> opsDup = {{"push", 3}, {"push", 1}, {"push", 3}, {"push", 1},
                         {"peek", 0},  {"pop", 0},  {"peek", 0},  {"pop", 0},
                         {"peek", 0},  {"pop", 0},  {"peek", 0}};
    vector<string> wantDup = {"null", "null", "null", "null", "1", "null",
                              "1",    "null", "3",    "null", "3"};
    check<SortedStack>(opsDup, wantDup);
    check<SortedStackLazy>(opsDup, wantDup);

    // 边界：连续入栈的逆序数据，每次 peek 都是当前最小值
    vector<Op> opsDesc = {{"push", 5}, {"push", 4}, {"push", 3}, {"push", 2},
                          {"push", 1}, {"peek", 0}, {"pop", 0},  {"peek", 0},
                          {"pop", 0},  {"peek", 0}};
    vector<string> wantDesc = {"null", "null", "null", "null", "null", "1",
                               "null", "2",    "null", "3"};
    check<SortedStack>(opsDesc, wantDesc);
    check<SortedStackLazy>(opsDesc, wantDesc);

    // 随机交叉验证
    crossCheck<SortedStack>();
    crossCheck<SortedStackLazy>();

    cout << "all tests passed" << endl;
    return 0;
}
