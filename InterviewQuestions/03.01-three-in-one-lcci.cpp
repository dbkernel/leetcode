/*
三合一。描述如何只用一个数组来实现三个栈。
你应该实现push(stackNum, value)、pop(stackNum)、isEmpty(stackNum)、peek(stackNum)方法。
stackNum表示栈下标，value表示压入的值。
构造函数会传入一个stackSize参数，代表每个栈的大小。

示例1:
输入：
["TripleInOne", "push", "push", "pop", "pop", "pop", "isEmpty"]
[[1], [0, 1], [0, 2], [0], [0], [0], [0]]
 输出：
[null, null, null, 1, -1, -1, true]
说明：当栈为空时`pop, peek`返回-1，当栈满时`push`不压入元素。

示例2:
 输入：
["TripleInOne", "push", "push", "push", "pop", "pop", "pop", "peek"]
[[2], [0, 1], [0, 2], [0, 3], [0], [0], [0], [0]]
 输出：
[null, null, null, null, 2, 1, -1, -1]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/three-in-one-lcci
*/

#include <vector>
#include <iostream>
#include <cstdlib>
using namespace std;

class TripleInOne
{
public:
    TripleInOne(int stackSize)
    {
        stack = vector<int>(stackSize * 3, 0);
        top[0] = 0;
        top[1] = stackSize;
        top[2] = stackSize * 2;
        this->stackSize = stackSize;
    }

    void push(int stackNum, int value)
    {
        if (top[stackNum] < stackSize * (stackNum + 1))
        {
            stack[top[stackNum]++] = value;
        }
    }

    int pop(int stackNum)
    {
        if (isEmpty(stackNum))
        {
            return -1;
        }

        int value = stack[top[stackNum] - 1];
        top[stackNum]--;
        return value;
    }

    int peek(int stackNum)
    {
        if (isEmpty(stackNum))
        {
            return -1;
        }

        return stack[top[stackNum] - 1];
    }

    bool isEmpty(int stackNum)
    {
        return top[stackNum] == stackSize * stackNum;
    }

private:
    int top[3], stackSize;
    vector<int> stack;
};

int main()
{
    // Your TripleInOne object will be instantiated and called as such:
    int stackSize = rand() % 10 + 1;
    int stackNum = rand() % 3;

    TripleInOne *obj = new TripleInOne(stackSize);
    obj->push(stackNum, 1);
    obj->push(stackNum, 2);

    int value = obj->pop(stackNum);
    cout << value << endl;
    value = obj->pop(stackNum);
    cout << value << endl;
    value = obj->pop(stackNum);
    cout << value << endl;

    int value2 = obj->peek(stackNum);
    cout << value2 << endl;

    bool value3 = obj->isEmpty(stackNum);
    cout << value3 << endl;
}
