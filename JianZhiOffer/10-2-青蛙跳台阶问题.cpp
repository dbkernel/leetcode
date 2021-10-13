/*
剑指 Offer 10- II. 青蛙跳台阶问题

一只青蛙一次可以跳上1级台阶，也可以跳上2级台阶。求该青蛙跳上一个 n 级的台阶总共有多少种跳法。
答案需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，请返回 1。

示例 1：
输入：n = 2
输出：2

示例 2：
输入：n = 7
输出：21

示例 3：
输入：n = 0
输出：1

提示：
0 <= n <= 100

注意：本题与主站 70 题相同：https://leetcode-cn.com/problems/climbing-stairs/

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/qing-wa-tiao-tai-jie-wen-ti-lcof


解题思路：

此类求 多少种可能性 的题目一般都有 递推性质 ，即 f(n) 和 f(n−1)…f(1) 之间是有联系的。

设跳上 n 级台阶有 f(n) 种跳法。在所有跳法中，青蛙的最后一步只有两种情况： 跳上 1 级或 2 级台阶。
当为 1 级台阶： 剩 n−1 个台阶，此情况共有 f(n−1) 种跳法；
当为 2 级台阶： 剩 n−2 个台阶，此情况共有 f(n−2) 种跳法。
f(n) 为以上两种情况之和，即 f(n)=f(n-1)+f(n-2) ，以上递推性质为斐波那契数列。

本题可转化为 求斐波那契数列第 n 项的值 ，与 面试题10- I. 斐波那契数列 等价，唯一的不同在于起始数字不同。
青蛙跳台阶问题： f(0)=1 , f(1)=1 , f(2)=2 ；
斐波那契数列问题： f(0)=0 , f(1)=1 , f(2)=1 。

*/

#include <iostream>
#include <cassert>
using namespace std;

class Solution
{
public:
    int numWaysRecursive(int n)
    {
        if (n == 0 || n == 1)
            return 1;
        return (numWaysRecursive(n - 1) + numWaysRecursive(n - 2)) % mod;
    }

    int numWays(int n)
    {
        if (n == 0 || n == 1)
            return 1;
        int v1 = 1, v2 = 1, v = 0;
        for (int i = 2; i <= n; i++)
        {
            v = (v1 + v2) % mod;
            v1 = v2;
            v2 = v;
        }
        return v;
    }

private:
    int mod = 1000000007;
};

int main()
{
    Solution s;
    assert(s.numWaysRecursive(0) == 1);
    assert(s.numWaysRecursive(2) == 2);
    assert(s.numWaysRecursive(7) == 21);

    assert(s.numWays(0) == 1);
    assert(s.numWays(2) == 2);
    assert(s.numWays(7) == 21);
}