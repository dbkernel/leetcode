/*
剑指 Offer 10- I. 斐波那契数列

写一个函数，输入 n ，求斐波那契（Fibonacci）数列的第 n 项（即 F(N)）。斐波那契数列的定义如下：

F(0) = 0,   F(1) = 1
F(N) = F(N - 1) + F(N - 2), 其中 N > 1.

斐波那契数列由 0 和 1 开始，之后的斐波那契数就是由之前的两数相加而得出。

答案需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，请返回 1。

示例 1：
输入：n = 2
输出：1

示例 2：
输入：n = 5
输出：5
 
提示：
0 <= n <= 100

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof

官方解题思路：
https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof/solution/fei-bo-na-qi-shu-lie-by-leetcode-solutio-hbss/

*/
#include <iostream>
#include <cassert>
#include <vector>
using namespace std;

class Solution
{
public:
    // 常规方法：递归方式
    // 时间复杂度：O(n)，空间复杂度：O(1)
    int fib_recursive(int n)
    {
        if (n == 1 || n == 2)
            return 1;
        return (fib_recursive(n - 1) + fib_recursive(n - 2)) % mod;
    }

    // 方法一：非递归方式
    // 时间复杂度：O(n)，空间复杂度：O(1)
    int fib1(int n)
    {
        if (n == 1 || n == 2)
            return 1;

        int v1 = 1, v2 = 1, v = 0;
        for (int i = 3; i <= n; i++)
        {
            v = (v1 + v2) % mod;
            v1 = v2;
            v2 = v;
        }
        return v;
    }

    // 方法二：矩阵快速幂
    // 时间复杂度：O(logn) ，空间复杂度：O(1)
    /*
    方法一的时间复杂度是 O(n)O(n)。使用矩阵快速幂的方法可以降低时间复杂度。

    首先我们可以构建这样一个递推关系：
    [1 1][F(n)]          [F(n) + F(n-1)]     [F(n+1)]
    [1 0][F(n-1)]        [F(n)]              [F(n)]

    因此：
    [F(n+1)]             [1 1]n [F(1)]
    [F(n)]               [1 0]  [F(0)]
    令 M =
    [1 1]
    [1 0]
    因此只要我们能快速计算矩阵 M 的 n 次幂，就可以得到 F(n) 的值。如果直接求取 M^n ，时间复杂度是 O(n)，
    可以定义矩阵乘法，然后用快速幂算法来加速这里 M^n 的求取。
    */
    int fib2(int n)
    {
        if (n < 2)
        {
            return n;
        }
        vector<vector<long>> q{{1, 1}, {1, 0}};
        vector<vector<long>> res = pow(q, n - 1);
        return res[0][0];
    }

    vector<vector<long>> pow(vector<vector<long>> &a, int n)
    {
        vector<vector<long>> ret{{1, 0}, {0, 1}};
        while (n > 0)
        {
            if (n & 1)
            {
                ret = multiply(ret, a);
            }
            n >>= 1;
            a = multiply(a, a);
        }
        return ret;
    }

    vector<vector<long>> multiply(vector<vector<long>> &a, vector<vector<long>> &b)
    {
        vector<vector<long>> c{{0, 0}, {0, 0}};
        for (int i = 0; i < 2; i++)
        {
            for (int j = 0; j < 2; j++)
            {
                c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j]) % mod;
            }
        }
        return c;
    }

private:
    int mod = 1000000007;
};

int main()
{
    Solution s;
    assert(s.fib_recursive(2) == 1);
    assert(s.fib_recursive(5) == 5);
    assert(s.fib_recursive(8) == 21);

    assert(s.fib1(2) == 1);
    assert(s.fib1(5) == 5);
    assert(s.fib1(8) == 21);

    assert(s.fib2(2) == 1);
    assert(s.fib2(5) == 5);
    assert(s.fib2(8) == 21);
}