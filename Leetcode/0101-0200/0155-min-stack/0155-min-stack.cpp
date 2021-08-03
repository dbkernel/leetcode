/*
155. 最小栈

设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

push(x) —— 将元素 x 推入栈中。
pop() —— 删除栈顶的元素。
top() —— 获取栈顶元素。
getMin() —— 检索栈中的最小元素。

示例:

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
- pop、top 和 getMin 操作总是在 非空栈 上调用。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/min-stack

思路：
- 对于栈来说，如果一个元素 a 在入栈时，栈里有其它的元素 b, c, d，那么无论这个栈在之后经历了什么操作，只要 a 在栈中，b, c, d 就一定在栈中，因为在 a 被弹出之前，b, c, d 不会被弹出。
- 因此，在操作过程中的任意一个时刻，只要栈顶的元素是 a，那么我们就可以确定栈里面现在的元素一定是 a, b, c, d。
- 那么，我们可以在每个元素 a 入栈时把当前栈的最小值 m 存储起来。在这之后无论何时，如果栈顶元素是 a，我们就可以直接返回存储的最小值 m。
*/
#include <stack>
using namespace std;

class MinStack
{
public:
    /** initialize your data structure here. */
    MinStack()
    {
    }

    void push(int val)
    {
        stk.push(val);
        if (min_stk.empty() || min_stk.top() > val)
            min_stk.push(val);
        else
            min_stk.push(min_stk.top());
    }

    void pop()
    {
        if (!stk.empty())
        {
            stk.pop();
            min_stk.pop();
        }
    }

    int top()
    {
        return (stk.empty() ? -1 : stk.top());
    }

    int getMin()
    {
        return (min_stk.empty() ? -1 : min_stk.top());
    }

private:
    stack<int> stk;
    stack<int> min_stk; // 存放当前当前stk中对应key的最小值
};

int main()
{
    MinStack *obj = new MinStack();
    int val = 100;
    obj->push(val);
    obj->pop();
    int param_3 = obj->top();
    int param_4 = obj->getMin();
}
