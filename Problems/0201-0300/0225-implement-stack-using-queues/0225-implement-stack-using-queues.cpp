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
#include <cassert>
#include <iostream>
#include <queue>
#include <string>
#include <utility>
#include <vector>
using namespace std;

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
class MyStackTwoQueues
{
public:
    MyStackTwoQueues() {}

    void push(int x)
    {
        // 新元素先进辅助队列，再把旧元素整个接到它后面
        queue2.push(x);
        while (!queue1.empty())
        {
            queue2.push(queue1.front());
            queue1.pop();
        }
        swap(queue1, queue2);
    }

    int pop()
    {
        int top = queue1.front();
        queue1.pop();
        return top;
    }

    int top() { return queue1.front(); }

    bool empty() { return queue1.empty(); }

private:
    queue<int> queue1, queue2;
};

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
class MyStack
{
public:
    MyStack() {}

    void push(int x)
    {
        // n 必须在入队之前取，避免把新元素也轮转一遍
        int n = q.size();
        q.push(x);
        for (int i = 0; i < n; ++i)
        {
            q.push(q.front());
            q.pop();
        }
    }

    int pop()
    {
        int top = q.front();
        q.pop();
        return top;
    }

    int top() { return q.front(); }

    bool empty() { return q.empty(); }

private:
    queue<int> q;
};

namespace
{
// 一次操作：name 为 push/pop/top/empty，value 是 push 的参数
struct Op
{
    string name;
    int value;
};

// 用 function 模板参数让两套实现共用同一份断言代码
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
            out.push_back(to_string(stack.pop()));
        }
        else if (op.name == "top")
        {
            out.push_back(to_string(stack.top()));
        }
        else
        {
            out.push_back(stack.empty() ? "true" : "false");
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

// 两套实现交叉验证：同一条随机操作序列下输出必须完全一致
template <typename Stack>
void crossCheck()
{
    Stack stack;
    vector<int> expect; // 用 vector 当栈做对照
    unsigned seed = 20240607;
    for (int i = 0; i < 2000; ++i)
    {
        seed = seed * 1103515245 + 12345;
        int r = (seed >> 16) % 10;
        if (r < 5 || expect.empty())
        {
            int value = (seed >> 8) % 9 + 1;
            stack.push(value);
            expect.push_back(value);
        }
        else if (r < 8)
        {
            assert(stack.pop() == expect.back());
            expect.pop_back();
        }
        else
        {
            assert(!stack.empty());
            assert(stack.top() == expect.back());
        }
    }
    assert(stack.empty() == expect.empty());
}
} // namespace

int main()
{
    // 示例
    vector<Op> sample = {{"push", 1}, {"push", 2}, {"top", 0},
                         {"pop", 0},  {"empty", 0}};
    vector<string> want = {"null", "null", "2", "2", "false"};
    check<MyStack>(sample, want);
    check<MyStackTwoQueues>(sample, want);

    // 边界：只判空一次
    check<MyStack>({{"empty", 0}}, {"true"});
    check<MyStackTwoQueues>({{"empty", 0}}, {"true"});

    // 边界：压满再逐个弹出，顺序必须是后进先出
    vector<Op> lifo = {{"push", 1}, {"push", 2}, {"push", 3},
                       {"pop", 0},  {"pop", 0},  {"pop", 0},
                       {"empty", 0}};
    vector<string> lifoWant = {"null", "null", "null", "3", "2", "1", "true"};
    check<MyStack>(lifo, lifoWant);
    check<MyStackTwoQueues>(lifo, lifoWant);

    // 随机交叉验证
    crossCheck<MyStack>();
    crossCheck<MyStackTwoQueues>();

    cout << "all tests passed" << endl;
    return 0;
}
