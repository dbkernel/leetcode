/*
============================================================================
374. 猜数字大小（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/guess-number-higher-or-lower/
分类：二分查找 · 交互题

我们正在玩猜数字游戏。猜数字游戏的规则如下：

我会从 1 到 n 随机选择一个数字。 请你猜选出的是哪个数字。（我选的数字在整个游戏中保持不变）。

如果你猜错了，我会告诉你，我选出的数字比你猜测的数字大了还是小了。

你可以通过调用一个预先定义好的接口 int guess(int num) 来获取猜测结果，
返回值一共有三种可能的情况：
  - -1：你猜的数字比我选出的数字大 （即 num > pick）。
  - 1：你猜的数字比我选出的数字小 （即 num < pick）。
  - 0：你猜的数字与我选出的数字相等。（即 num == pick）。

返回我选出的数字。

示例 1：
输入：n = 10, pick = 6
输出：6

示例 2：
输入：n = 1, pick = 1
输出：1

示例 3：
输入：n = 2, pick = 1
输出：1

提示：
1 <= n <= 2^31 - 1
1 <= pick <= n

说明：本题是交互题，力扣预置了 guess 接口。为了让本文件可以独立编译运行，
下面用一个全局变量模拟 pick，并自行实现 guess（提交到力扣时应删掉这一段）。
*/
#include <cassert>
#include <climits>
#include <iostream>
using namespace std;

// ===== 本地模拟的交互接口（提交时删除）=====
namespace mock
{
int pick = 0;
long long calls = 0;
} // namespace mock

int guess(int num)
{
    ++mock::calls;
    if (num > mock::pick) return -1;
    if (num < mock::pick) return 1;
    return 0;
}
// ===== 模拟结束 =====

/*
解法一：二分查找（左闭右闭 + 区间收敛到一点）【最优解】

官方题解只给出这一种方法，也就是本题的最优解：时间 O(log n)、空间 O(1)。

为什么慢：从 1 开始逐个猜，最坏要猜 n 次。但 guess 的返回值给了「方向」信息：
猜大了往小走、猜小了往大走——单调性成立，天然适合二分。

换个角度：维护答案所在的闭区间 [left, right]，初始为 [1, n]。每次猜中点 mid：
  · guess(mid) <= 0 表示 mid >= pick，答案在 [left, mid]；
  · guess(mid) > 0 表示 mid < pick，答案在 [mid + 1, right]。
当 left == right 时区间缩成一点，它就是答案，直接返回 left。

关键点：
  · 循环条件是 left < right（不是 <=），因为退出时 left == right 就是答案，
    这样最后不需要额外判断
  · right = mid 而不是 mid - 1：mid 本身可能是答案，不能排除掉
  · left = mid + 1 可以排除 mid：此时已知 mid < pick
  · mid 必须写成 left + (right - left) / 2。n 可达 2^31 - 1，
    (left + right) 会超出 int 范围而溢出成负数，导致死循环或错误缩区间
  · 返回值用 long long / 64 位中间量也能避免溢出，但改 mid 写法更简单
  · guess 的语义容易记反：-1 表示「猜大了」，1 表示「猜小了」

以 n = 10, pick = 6 为例：
  left=1, right=10 → mid=5，guess(5)=1（猜小）→ left=6
  left=6, right=10 → mid=8，guess(8)=-1（猜大）→ right=8
  left=6, right=8  → mid=7，guess(7)=-1（猜大）→ right=7
  left=6, right=7  → mid=6，guess(6)=0 → right=6
  left == right == 6 → 返回 6

复杂度分析：
  时间 O(log n)：每次把区间长度减半
  空间 O(1)：只用两个边界
*/
class Solution
{
public:
    int guessNumber(int n)
    {
        int left = 1, right = n;
        while (left < right)
        {
            // 防溢出：不要写 (left + right) / 2
            int mid = left + (right - left) / 2;
            if (guess(mid) <= 0)
            {
                // mid >= pick，答案落在 [left, mid]
                right = mid;
            }
            else
            {
                // mid < pick，答案落在 [mid + 1, right]
                left = mid + 1;
            }
        }
        return left;
    }
};

namespace
{
// 对给定的 n、pick 跑一遍，并返回猜测次数
long long run(int n, int pick)
{
    mock::pick = pick;
    mock::calls = 0;
    Solution solution;
    int got = solution.guessNumber(n);
    assert(got == pick);
    return mock::calls;
}
} // namespace

int main()
{
    // 示例 1
    run(10, 6);

    // 示例 2
    run(1, 1);

    // 示例 3
    run(2, 1);

    // 边界：pick 落在两端
    run(100, 1);
    run(100, 100);

    // 遍历小规模的所有 pick，确认每次都能猜中
    for (int n = 1; n <= 50; ++n)
    {
        for (int p = 1; p <= n; ++p) run(n, p);
    }

    // 边界：n 取题目上限 2^31 - 1，pick 取两端与中间
    // 如果 mid 写成 (left + right) / 2，这里会溢出而陷入死循环/出错
    run(INT_MAX, 1);
    run(INT_MAX, INT_MAX);
    run(INT_MAX, 1234567890);
    run(INT_MAX, INT_MAX / 2);

    // 猜测次数应满足 O(log n)：n = 2^31 - 1 时不超过 32 次
    assert(run(INT_MAX, 1234567890) <= 32);

    cout << "all tests passed" << endl;
    return 0;
}
