/*
用两个栈实现一个队列。队列的声明如下，请实现它的两个函数 appendTail 和 deleteHead ，
分别完成在队列尾部插入整数和在队列头部删除整数的功能。(若队列中没有元素，deleteHead 操作返回 -1 )


示例 1：
输入：
["CQueue","appendTail","deleteHead","deleteHead"]
[[],[3],[],[]]
输出：[null,null,3,-1]

示例 2：
输入：
["CQueue","deleteHead","appendTail","appendTail","deleteHead","deleteHead"]
[[],[],[5],[2],[],[]]
输出：[null,-1,null,null,5,2]

提示：
1 <= values <= 10000
最多会对 appendTail、deleteHead 进行 10000 次调用

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof
*/

/*
解题思路：
1. stack_in 只push数据
2. stack_out 只pop数据，只有当该栈为空时才从stack_in导入数据
*/
#include <iostream>
#include <stack>
#include <cstdlib>
using namespace std;

class CQueue
{
public:
    CQueue()
    {
    }

    void appendTail(int value)
    {
        stack_in.push(value);
    }

    int deleteHead()
    {
        if (stack_out.empty())
        {
            // in -> out
            while (!stack_in.empty())
            {
                stack_out.push(stack_in.top());
                stack_in.pop();
            }
        }

        if (stack_out.empty())
        {
            return -1;
        }

        int value = stack_out.top();
        stack_out.pop();
        return value;
    }

private:
    stack<int> stack_in, stack_out;
};

int main()
{
    // Your CQueue object will be instantiated and called as such:
    int value = rand() % 10000 + 1;
    CQueue *obj = new CQueue();
    obj->appendTail(value);
    int param_2 = obj->deleteHead();
}