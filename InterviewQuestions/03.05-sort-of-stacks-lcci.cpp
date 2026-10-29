/*
面试题 03.05. 栈排序

栈排序。 编写程序，对栈进行排序使最小元素位于栈顶。最多只能使用一个其他的临时栈存放数据，但不得将元素复制到别的数据结构（如数组）中。该栈支持如下操作：push、pop、peek 和 isEmpty。当栈为空时，peek 返回 -1。

示例1:
 输入：
["SortedStack", "push", "push", "peek", "pop", "peek"]
[[], [1], [2], [], [], []]
 输出：
[null,null,null,1,null,2]

示例2:
 输入：
["SortedStack", "pop", "pop", "push", "pop", "isEmpty"]
[[], [], [], [1], [], []]
 输出：
[null,null,null,null,null,true]

说明:
栈中的元素数目在[0, 5000]范围内。

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-of-stacks-lcci

思路：
- peek() ==> top()

*/
#include <stack>
using namespace std;

class SortedStack
{
public:
    SortedStack()
    {
    }

    void push(int val)
    {
        if (stk.empty() || stk.top() >= val)
            stk.push(val);
        else
        {
            stack<int> tmp_stk;
            while (true)
            {
                if (!stk.empty() && stk.top() < val)
                {
                    tmp_stk.push(stk.top());
                    stk.pop();
                }
                else
                {
                    stk.push(val);
                    break;
                }
            }

            while (!tmp_stk.empty())
            {
                stk.push(tmp_stk.top());
                tmp_stk.pop();
            }
        }
    }

    void pop()
    {
        if (!stk.empty())
            stk.pop();
    }

    int peek()
    {
        return (stk.empty() ? -1 : stk.top());
    }

    bool isEmpty()
    {
        return stk.empty();
    }

private:
    stack<int> stk;
};

int main()
{
    SortedStack *obj = new SortedStack();
    int val = 1;
    obj->push(val);
    obj->pop();
    int param_3 = obj->peek();
    bool param_4 = obj->isEmpty();
}
