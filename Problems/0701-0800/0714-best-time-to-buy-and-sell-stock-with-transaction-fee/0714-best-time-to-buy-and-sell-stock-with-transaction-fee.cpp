/*
============================================================================
714. 买卖股票的最佳时机含手续费（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
分类：动态规划 · 状态机 / 贪心（买入预扣手续费）

给定一个整数数组 prices，其中 prices[i] 表示第 i 天的股票价格；整数 fee 代表了交易股票的手续费用。

你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。

返回获得利润的最大值。

注意：这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。

示例 1：
输入：prices = [1, 3, 2, 8, 4, 9], fee = 2
输出：8
解释：能够达到的最大利润:
在此处买入 prices[0] = 1
在此处卖出 prices[3] = 8
在此处买入 prices[4] = 4
在此处卖出 prices[5] = 9
总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8

示例 2：
输入：prices = [1,3,7,5,10,3], fee = 3
输出：6

提示：
- 1 <= prices.length <= 5 * 10^4
- 1 <= prices[i] < 5 * 10^4
- 0 <= fee < 5 * 10^4
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <random>
#include <vector>
using namespace std;

/*
解法一：DP 状态机（手续费在卖出时扣）【最优解】

为什么慢：最直接的想法是枚举「哪些买入日与卖出日配对」，再给每一对扣
一次手续费。交易笔数不限，配对方式随 n 指数增长，光判断两个区间是否
重叠就要额外的维度，找不到既好写又不超时的枚举方式。

换个角度：手续费不改变「今天收盘持不持股」这件事，只改变卖出的收益。
于是仍然只需要两个状态，把 fee 写进卖出的那一步即可。

关键点：
  · hold[i] = max(hold[i-1], cash[i-1] - p[i])：买入时的花费不含手续费
  · cash[i] = max(cash[i-1], hold[i-1] + p[i] - fee)：卖出时扣一次
  · 手续费只能在买入或卖出的一头扣，两头都扣等于一笔交易付了两次
  · 两个转移都读第 i-1 天的旧值，先算完再统一覆盖
  · 答案取 cash：收盘还持股等于没卖出，不能计入收益

手推一遍
以示例 1（prices = [1,3,2,8,4,9]，fee = 2）为例：
  第 1 天：cash = 0，hold = -1
  第 2 天：cash = max(0, -1+3-2) = 0，hold = max(-1, -3) = -1
  第 3 天：cash = max(0, -1+2-2) = 0，hold = max(-1, -2) = -1
  第 4 天：cash = max(0, -1+8-2) = 5，hold = max(-1, 5-8) = -1
  第 5 天：cash = max(5, -1+4-2) = 5，hold = max(-1, 5-4) = 1
  第 6 天：cash = max(5, 1+9-2) = 8 → 答案 8

复杂度分析：
  时间 O(n)：每天常数次比较
  空间 O(1)：两个状态变量

解法二：贪心，买入时预扣手续费【最优解，与解法一并列】

为什么慢：直接把 122 的贪心搬过来（累加每天的正涨幅）会多扣手续费：
连续上涨被拆成多次买入，每一段都收一次费用，收益被重复扣减。

换个角度：把手续费一次性算进买入成本，卖出就变成「纯收入」。此时每天
收盘只有两个数有意义：already 是已落袋的最大收益，hold 是「已落袋收益
减去含费买入成本」的最大值。两者各自只保留更优的那个，等价于对「持有
成本」做贪心。

关键点：
  · hold = max(hold, already - p[i] - fee)：手续费在买入时一次扣清
  · already = max(already, hold + p[i])：卖出不扣费，否则一笔交易收两次
  · 两式都用同一轮的旧值：若先更新 already 再算 hold，等于允许当天
    卖出再买回，fee ≥ 0 时这不会提升收益，但用旧值能免去这层论证
  · 与解法一时间同为 O(n)、空间同为 O(1)，并列最优；贪心少一个减法项，
    但要记住「费用只在一头扣」这个前提，改动方向时会比 DP 更易错

手推一遍
仍以示例 1（fee = 2）为例，hold 的初值是 -1-2 = -3：
  第 2 天：already = max(0, -3+3) = 0，hold = max(-3, -5) = -3
  第 4 天：already = max(0, -3+8) = 5，hold = max(-3, -5) = -3
  第 5 天：already = max(5, -3+4) = 5，hold = max(-3, 5-4-2) = -1
  第 6 天：already = max(5, -1+9) = 8 → 答案 8

复杂度分析：
  时间 O(n)：扫描一遍
  空间 O(1)：两个变量
*/
class Solution1
{
public:
    int maxProfit(vector<int> &prices, int fee)
    {
        // cash：收盘空仓的最大收益；hold：收盘持股的最大收益
        int cash = 0;
        int hold = -prices[0];
        for (size_t i = 1; i < prices.size(); ++i)
        {
            // 手续费只在卖出时扣一次，买入侧保持原价
            int nextCash = max(cash, hold + prices[i] - fee);
            int nextHold = max(hold, cash - prices[i]);
            cash = nextCash;
            hold = nextHold;
        }
        return cash;
    }
};

class Solution2
{
public:
    int maxProfit(vector<int> &prices, int fee)
    {
        // already：已落袋收益；hold：落袋收益减去「含费买入成本」
        int already = 0;
        int hold = -prices[0] - fee;
        for (size_t i = 1; i < prices.size(); ++i)
        {
            int nextAlready = max(already, hold + prices[i]);
            int nextHold = max(hold, already - prices[i] - fee);
            already = nextAlready;
            hold = nextHold;
        }
        return already;
    }
};

namespace
{
// 对照用 O(n^2) DP：dp[i] 是前 i 天的最大收益，枚举最后一笔的买入日
int brute(const vector<int> &prices, int fee)
{
    int n = static_cast<int>(prices.size());
    vector<int> dp(static_cast<size_t>(n) + 1, 0);
    for (int i = 1; i <= n; ++i)
    {
        dp[i] = dp[i - 1];
        // 第 j 天买入、第 i-1 天卖出，这笔交易付一次 fee
        for (int j = 0; j < i; ++j)
        {
            int sell = dp[j] + prices[i - 1] - prices[j] - fee;
            dp[i] = max(dp[i], sell);
        }
    }
    return dp[n];
}

void check(vector<int> prices, int fee, int want)
{
    Solution1 s1;
    Solution2 s2;
    assert(s1.maxProfit(prices, fee) == want);
    assert(s2.maxProfit(prices, fee) == want);
    assert(brute(prices, fee) == want);
}

// 三路对拍：fee 取 0 时退化成 122，可以用同一套数组验证扣费位置没错
void randomCheck()
{
    mt19937 rng(20240103);
    uniform_int_distribution<int> len(1, 20);
    uniform_int_distribution<int> val(1, 20);
    uniform_int_distribution<int> feeDist(0, 6);
    for (int t = 0; t < 3000; ++t)
    {
        vector<int> prices(static_cast<size_t>(len(rng)));
        for (int &x : prices) x = val(rng);
        int fee = feeDist(rng);

        Solution1 s1;
        Solution2 s2;
        int expected = brute(prices, fee);
        assert(s1.maxProfit(prices, fee) == expected);
        assert(s2.maxProfit(prices, fee) == expected);
    }
}
} // namespace

int main()
{
    // 示例 1：两笔交易，每笔各扣 2
    check({1, 3, 2, 8, 4, 9}, 2, 8);

    // 示例 2：手续费吃掉一部分涨幅
    check({1, 3, 7, 5, 10, 3}, 3, 6);

    // 边界：只有一天，没有可卖的第二日
    check({5}, 3, 0);

    // 边界：两天差价刚好等于手续费，交易与不交易一样，答案为 0
    check({1, 5}, 4, 0);

    // 边界：两天差价比手续费多 1
    check({1, 5}, 3, 1);

    // 边界：fee = 0 时退化为 122 的规则，涨跌都要吃到
    check({1, 3, 2, 8}, 0, 8);

    // 反直觉：手续费很高时宁可少做几笔，答案小于无费情形
    check({1, 3, 4, 5}, 3, 1);

    // 边界：规模上限，只跑两个 O(n) 解
    {
        vector<int> prices(50000);
        for (size_t i = 0; i < prices.size(); ++i)
        {
            prices[i] = static_cast<int>(i % 1000) + 1;
        }
        Solution1 s1;
        Solution2 s2;
        assert(s1.maxProfit(prices, 2) == s2.maxProfit(prices, 2));
    }

    randomCheck();

    cout << "all tests passed" << endl;
    return 0;
}
