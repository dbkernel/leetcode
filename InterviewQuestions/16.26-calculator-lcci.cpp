/*
面试题16.26 计算器

给定一个包含正整数、加(+)、减(-)、乘(*)、除(/)的算数表达式(括号除外)，计算其结果。
表达式仅包含非负整数，+， - ，*，/ 四种运算符和空格。 整数除法仅保留整数部分。

示例 1:
输入: "3+2*2"
输出: 7

示例 2:
输入: " 3/2 "
输出: 1

示例 3:
输入: " 3+5 / 2 "
输出: 5

说明：
- 你可以假设所给定的表达式都是有效的。
- 请不要使用内置的库函数 eval。

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/calculator-lcci

思路：
- 算术运算式先算乘除后算加减。
- 当遇到加法或减法的时候我们没法确定要不要计算，比如2+3*4。
- 当遇到乘法或者除法的时候我们是可以直接计算的，因为他们的优先级比较高。
- 所以我们可以把不能确定是否要计算的先加入到栈中，可以直接计算的先计算，然后再加入到栈中。
*/
#include <stack>
#include <iostream>
#include <cassert>
#include <string>
using namespace std;

class Solution
{
public:
    int calculate(string s)
    {
        char ops = '+';
        stack<int> values;
        int len = s.length();
        int stack_head;

        for (int i = 0; i < len; i++)
        {
            char ch = s[i];
            if (ch == ' ') // skip space
            {
                continue;
            }

            if (ch >= '0' && ch <= '9')
            {
                // 找到连续的数字字符串，把它转化为整数
                int64_t num = 0; // 如果设为int，leetcode提交代码后会溢出报错
                while (i < len && (ch = s[i]) >= '0' && ch <= '9')
                {
                    num = num * 10 + ch - '0'; // 相较于 '0' 的差值，即为真实的数值
                    i++;
                }

                i--; // 抵消上面while循环中最后的 i++，使 i 指向数字的最后一位，比如123中的3

                // 乘法和除法，运算之后在存放到栈中，加法和减法直接存放到栈中。
                switch (ops)
                {
                case '+':
                    values.push(num);
                    break;
                case '-':
                    values.push(-num);
                    break;
                case '*':
                    stack_head = values.top();
                    values.pop();
                    values.push(stack_head * num);
                    break;
                case '/':
                    stack_head = values.top();
                    values.pop();
                    values.push(stack_head / num);
                    break;
                }
            }
            else
            {
                ops = ch;
            }
        }

        //把栈中的所有元素都取出来，计算他们的和
        int sum = 0;
        while (!values.empty())
        {
            sum += values.top();
            values.pop();
        }

        return sum;
    }
};

int main()
{
    Solution s;
    assert(s.calculate("3+2") == 5);
    assert(s.calculate("3/2") == 1);
    assert(s.calculate("3+2*2") == 7);
    assert(s.calculate("3+2*2/3") == 4);
    assert(s.calculate("30+20*4/2") == 70);
}
