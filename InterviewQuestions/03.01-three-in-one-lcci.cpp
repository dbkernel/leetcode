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
#include <cassert>
#include <iostream>
#include <stack>
#include <string>
#include <vector>
using namespace std;

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面按《程序员面试金典》的标准做法给出
两种实现，并把真正满足「只用一个数组」的那一种标为最优解。

解法一：三个独立数组【非最优，不满足题意】

最直白的写法就是开三个 vector，每个独立管理自己的栈顶。push / pop / peek /
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
// 解法一：三个独立数组（仅作对照，不满足「只用一个数组」的要求）
class TripleInOneThreeArrays
{
public:
    explicit TripleInOneThreeArrays(int stackSize) : capacity(stackSize) {}

    void push(int stackNum, int value)
    {
        if ((int)st[stackNum].size() < capacity) st[stackNum].push(value);
    }

    int pop(int stackNum)
    {
        if (st[stackNum].empty()) return -1;
        int value = st[stackNum].top();
        st[stackNum].pop();
        return value;
    }

    int peek(int stackNum) { return st[stackNum].empty() ? -1 : st[stackNum].top(); }

    bool isEmpty(int stackNum) { return st[stackNum].empty(); }

private:
    stack<int> st[3];
    int capacity = 0;
};

// 解法二：一个数组分三段 + 三个栈顶指针
class TripleInOne
{
public:
    explicit TripleInOne(int stackSize) : stackSize(stackSize), data(3 * stackSize)
    {
        for (int s = 0; s < 3; ++s) top[s] = s * stackSize - 1;
    }

    void push(int stackNum, int value)
    {
        // 满了就丢弃（题面要求），不扩容
        if (top[stackNum] < (stackNum + 1) * stackSize - 1)
        {
            data[++top[stackNum]] = value;
        }
    }

    int pop(int stackNum)
    {
        if (isEmpty(stackNum)) return -1;
        return data[top[stackNum]--];
    }

    int peek(int stackNum)
    {
        if (isEmpty(stackNum)) return -1;
        return data[top[stackNum]];
    }

    bool isEmpty(int stackNum) { return top[stackNum] < stackNum * stackSize; }

private:
    int stackSize;
    vector<int> data; // 唯一的数组，切三段给三个栈用
    int top[3];       // 每个栈的栈顶下标，空栈时是段首前一格
};

namespace
{
// 一次操作：name 为 push/pop/peek/isEmpty，stackNum 是栈下标，value 是 push 的值
struct Op
{
    string name;
    int stackNum;
    int value;
};

// 用 function 风格的两套实现跑同一份操作序列
template <typename Triple>
vector<string> run(int stackSize, const vector<Op> &ops)
{
    Triple triple(stackSize);
    vector<string> out;
    for (const auto &op : ops)
    {
        if (op.name == "push")
        {
            triple.push(op.stackNum, op.value);
            out.push_back("null");
        }
        else if (op.name == "pop")
        {
            out.push_back(to_string(triple.pop(op.stackNum)));
        }
        else if (op.name == "peek")
        {
            out.push_back(to_string(triple.peek(op.stackNum)));
        }
        else
        {
            out.push_back(triple.isEmpty(op.stackNum) ? "true" : "false");
        }
    }
    return out;
}

template <typename Triple>
void check(int stackSize, const vector<Op> &ops, const vector<string> &want)
{
    vector<string> got = run<Triple>(stackSize, ops);
    assert(got == want);
}

// 随机交叉验证：用三个标准栈当对照
template <typename Triple>
void crossCheck(int stackSize)
{
    Triple triple(stackSize);
    stack<int> expect[3];
    unsigned seed = 20240607 + stackSize;
    for (int i = 0; i < 3000; ++i)
    {
        seed = seed * 1103515245 + 12345;
        int r = (seed >> 16) % 10;
        int s = (seed >> 8) % 3;
        if (r < 5)
        {
            int value = (seed >> 4) % 100;
            triple.push(s, value);
            // 对照栈同样按「满了丢弃」
            if ((int)expect[s].size() < stackSize) expect[s].push(value);
        }
        else if (r < 8)
        {
            int want = expect[s].empty() ? -1 : expect[s].top();
            if (!expect[s].empty()) expect[s].pop();
            assert(triple.pop(s) == want);
        }
        else if (r < 9)
        {
            int want = expect[s].empty() ? -1 : expect[s].top();
            assert(triple.peek(s) == want);
        }
        else
        {
            assert(triple.isEmpty(s) == expect[s].empty());
        }
    }
}
} // namespace

int main()
{
    // 示例 1：stackSize = 1，push 第二个元素被丢弃
    vector<Op> ops1 = {{"push", 0, 1}, {"push", 0, 2}, {"pop", 0, 0},
                       {"pop", 0, 0},  {"pop", 0, 0},  {"isEmpty", 0, 0}};
    vector<string> want1 = {"null", "null", "1", "-1", "-1", "true"};
    check<TripleInOne>(1, ops1, want1);
    check<TripleInOneThreeArrays>(1, ops1, want1);

    // 示例 2：stackSize = 2，压三个只留两个
    vector<Op> ops2 = {{"push", 0, 1}, {"push", 0, 2}, {"push", 0, 3}, {"pop", 0, 0},
                       {"pop", 0, 0},  {"pop", 0, 0},  {"peek", 0, 0}};
    vector<string> want2 = {"null", "null", "null", "2", "1", "-1", "-1"};
    check<TripleInOne>(2, ops2, want2);
    check<TripleInOneThreeArrays>(2, ops2, want2);

    // 边界：stackSize = 0，任何 push 都丢弃、任何取数都返回 -1
    vector<Op> ops0 = {{"push", 1, 7}, {"pop", 1, 0}, {"peek", 1, 0}, {"isEmpty", 1, 0}};
    vector<string> want0 = {"null", "-1", "-1", "true"};
    check<TripleInOne>(0, ops0, want0);
    check<TripleInOneThreeArrays>(0, ops0, want0);

    // 边界：三个栈互不干扰，各自后进先出
    vector<Op> opsMix = {{"push", 0, 1}, {"push", 1, 2}, {"push", 2, 3},
                         {"push", 1, 4}, {"pop", 1, 0},  {"pop", 0, 0},
                         {"pop", 2, 0},  {"pop", 1, 0},  {"pop", 0, 0}};
    vector<string> wantMix = {"null", "null", "null", "null", "4", "1", "3", "2", "-1"};
    check<TripleInOne>(2, opsMix, wantMix);
    check<TripleInOneThreeArrays>(2, opsMix, wantMix);

    // 随机交叉验证
    for (int stackSize : {0, 1, 3, 10})
    {
        crossCheck<TripleInOne>(stackSize);
        crossCheck<TripleInOneThreeArrays>(stackSize);
    }

    cout << "all tests passed" << endl;
    return 0;
}
