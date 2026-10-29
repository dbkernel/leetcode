/*
============================================================================
309. 最佳买卖股票时机含冷冻期（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/
分类：动态规划 · 状态机（冷冻期）

给定一个整数数组 prices，其中第 prices[i] 表示第 i 天的股票价格 。

设计一个算法计算出最大利润。在满足以下约束条件下，你可以尽可能地完成更多的交易（多次买卖一支股票）:

卖出股票后，你无法在第二天买入股票 (即冷冻期为 1 天)。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

示例 1:
输入: prices = [1,2,3,0,2]
输出: 3
解释: 对应的交易状态为: [买入, 卖出, 冷冻期, 买入, 卖出]

示例 2:
输入: prices = [1]
输出: 0

提示：
- 1 <= prices.length <= 5000
- 0 <= prices[i] <= 1000
*/

/*
解法一：三状态 DP（持有 / 当天卖出 / 空仓可买），按天存表【非最优】

为什么慢：如果只用「当前最大收益」一个数，就丢掉了「昨天刚卖过，今天
不能买」这个信息；若把「哪天卖过」也塞进状态，状态空间随天数膨胀，
枚举所有买卖序列是指数级的。

换个角度：不持股的情形其实有两种 —— 今天刚卖出（明天被冷冻）与今天
没有卖出（可以自由买入）。把这一层拆开，恰好三个状态就能覆盖全部情形，
且每天只从昨天转移。

关键点：
  · hold[i]：收盘持股。只能由 rest[i-1] 买入，绝不能由 sold[i-1] 买入 ——
    卖出后的第二天是冷冻期
  · sold[i]：第 i 天卖出。只能由 hold[i-1] 卖出得到，它代表「明天被冷冻」
  · rest[i]：收盘不持股且今天没卖，包含「本来就没买」与「冷冻期已过」
    两种，因此它可以来自 rest[i-1] 或 sold[i-1]（卖出次日正好落进 rest）
  · 答案取 max(sold[n-1], rest[n-1])：hold 是没卖出的股票，收益不算数
  · 初值：hold[0] = -p[0]，sold[0] 取负无穷（第 1 天没有股票可卖），
    rest[0] = 0

手推一遍
以示例 1（prices = [1,2,3,0,2]）为例：
  第 1 天：hold = -1，sold = -∞，rest = 0
  第 2 天：hold = max(-1, 0-2) = -1，sold = -1+2 = 1，rest = 0
  第 3 天：hold = max(-1, 0-3) = -1，sold = -1+3 = 2，rest = max(0,1) = 1
  第 4 天：hold = max(-1, 1-0) = 1，sold = -1+0 = -1，rest = max(1,2) = 2
  第 5 天：hold = max(1, 2-2) = 1，sold = 1+2 = 3，rest = max(2,-1) = 2
  答案 max(3, 2) = 3

复杂度分析：
  时间 O(n)：每天三次常数转移
  空间 O(n)：三条长度为 n 的表

解法二：把三张表滚成三个变量【最优解】

为什么多占了空间：解法一每天存三张表，而三条转移只读第 i-1 天，更早
的行再也不会被访问，把整个历史留在内存里是浪费。

三个转移都只读第 i-1 天，历史值用完就可以覆盖，于是三张表退化成三个
标量，空间从 O(n) 降到 O(1)，时间不变。这是本题的最优解。

关键点：
  · hold 只从 rest 买入，sold 只从 hold 卖出，rest 吸收昨天的 sold ——
    这三条边方向固定，就是「卖出当天」与「冷冻期后一天」的分界
  · 三个新值互相依赖旧值，必须先算进临时变量再一起覆盖；先改任意一个
    都会污染后面的转移（例如先改 hold，sold 就会用今天的新成本）
  · sold 只存活一天：第二天它的值被并进 rest，冷冻期也随之结束
  · 答案仍是 max(sold, rest)，收盘持股不计入

手推一遍
仍以示例 1 为例，滚动三次后 sold 与 rest 的变化与解法一逐行相同：
  第 5 天结束时 hold = 1、sold = 3、rest = 2 → 答案 3

复杂度分析：
  时间 O(n)：一次遍历
  空间 O(1)：三个标量
*/

const NEG_INF: i32 = -1000000000;

// 力扣提交时只给 Solution，这里用带序号的同名结构体区分多个解法；
// 单文件 rustc 编译没有平台的 Solution 类型，也必须自己声明。
pub struct Solution1;

impl Solution1 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        // hold：收盘持股；sold：当天卖出；rest：不持股且今天没卖
        let mut hold = vec![0; n];
        let mut sold = vec![0; n];
        let mut rest = vec![0; n];

        hold[0] = -prices[0];
        sold[0] = NEG_INF; // 第 1 天之前没有持仓，不可能卖出
        rest[0] = 0;

        for i in 1..n {
            // 买入只能来自 rest：sold 之后的那一天是冷冻期，不能买
            hold[i] = hold[i - 1].max(rest[i - 1] - prices[i]);
            sold[i] = hold[i - 1] + prices[i];
            // 冷冻期这一天记在 rest 里，所以 sold 过一天就转成 rest
            rest[i] = rest[i - 1].max(sold[i - 1]);
        }
        sold[n - 1].max(rest[n - 1])
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut hold = -prices[0];
        let mut sold = NEG_INF;
        let mut rest = 0;

        for i in 1..prices.len() {
            // 三个新值都依赖旧值，先算出来再一起覆盖，否则会互相污染
            let next_hold = hold.max(rest - prices[i]);
            let next_sold = hold + prices[i];
            let next_rest = rest.max(sold);
            hold = next_hold;
            sold = next_sold;
            rest = next_rest;
        }
        sold.max(rest)
    }
}

// 对照用 O(n^2) DP。free_dp[i]：前 i 天处理完、第 i 天可以自由买入时
// 的最大收益（等价于最后一次卖出不晚于第 i-2 天）
fn brute(prices: &[i32]) -> i32 {
    let n = prices.len();
    let mut free_dp = vec![0; n + 1];
    for i in 2..=n {
        free_dp[i] = free_dp[i - 1];
        // 最后一笔在第 j 天买入、第 i-2 天卖出，第 i-1 天正好是冷冻期
        for j in 0..(i - 1) {
            free_dp[i] = free_dp[i].max(free_dp[j] + prices[i - 2] - prices[j]);
        }
    }

    let mut best = free_dp[n];
    // 最后一笔在最后一天卖出，之后不需要再等冷冻期
    for j in 0..(n - 1) {
        best = best.max(free_dp[j] + prices[n - 1] - prices[j]);
    }
    best
}

fn check(prices: &[i32], want: i32) {
    assert_eq!(Solution1::max_profit(prices.to_vec()), want);
    assert_eq!(Solution2::max_profit(prices.to_vec()), want);
    assert_eq!(brute(prices), want);
}

// 三路对拍：暴力 DP 显式刻画了「卖出后隔一天才能买」，最容易暴露
// 把 sold[i-1] 当作买入来源的错误。不引第三方 crate，用 LCG 固定序列。
fn next_rand(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state >> 33
}

fn random_check() {
    let mut state: u64 = 20240104;
    for _ in 0..3000 {
        let n = 1 + (next_rand(&mut state) % 14) as usize;
        let prices: Vec<i32> = (0..n)
            .map(|_| (next_rand(&mut state) % 13) as i32)
            .collect();
        let want = brute(&prices);
        assert_eq!(Solution1::max_profit(prices.clone()), want);
        assert_eq!(Solution2::max_profit(prices), want);
    }
}

fn main() {
    // 示例 1：卖完必须空一天，答案 3
    check(&[1, 2, 3, 0, 2], 3);

    // 示例 2：单日无法交易
    check(&[1], 0);

    // 边界：两日上涨，卖出后没有后续操作，冷冻期不影响答案
    check(&[1, 2], 1);

    // 边界：两日下跌，不交易
    check(&[2, 1], 0);

    // 边界：三日上涨，整段持有优于卖出后再买
    check(&[1, 2, 3], 2);

    // 反直觉：没有冷冻期时能做 2 笔赚 2，有冷冻期只能赚 1
    check(&[2, 1, 2, 1, 2], 1);

    // 反直觉：中间暴涨，值得等冷冻期再换手
    check(&[6, 1, 3, 2, 4, 7], 6);

    // 边界：价格全为 0，任何交易都是 0
    check(&[0, 0, 0, 0], 0);

    // 边界：规模上限，只跑两个 O(n) 解
    let prices: Vec<i32> = (0..5000).map(|i| i * 17 % 1001).collect();
    assert_eq!(
        Solution1::max_profit(prices.clone()),
        Solution2::max_profit(prices)
    );

    random_check();

    println!("all tests passed");
}
