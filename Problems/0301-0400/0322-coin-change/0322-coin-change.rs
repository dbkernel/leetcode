/*
============================================================================
322. 零钱兑换（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/coin-change/
分类：动态规划 · 完全背包（最少硬币数）
============================================================================

给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，
表示总金额。

计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合
能组成总金额，返回 -1 。

你可以认为每种硬币的数量是无限的。

示例 1：
输入：coins = [1, 2, 5], amount = 11
输出：3
解释：11 = 5 + 5 + 1

示例 2：
输入：coins = [2], amount = 3
输出：-1

示例 3：
输入：coins = [1], amount = 0
输出：0

提示：
- 1 <= coins.length <= 12
- 1 <= coins[i] <= 2^31 - 1
- 0 <= amount <= 10^4
*/

/*
解法一：记忆化搜索（自顶向下）【最优解】

为什么慢：最直接的递归是 dfs(remain) = min over c (dfs(remain - c) + 1)。
同一个 remain 会被不同顺序的硬币反复展开：以 coins = [1,2,5]、amount = 11
为例，「先拿 1 再拿 2」和「先拿 2 再拿 1」落到同一个 remain，却各算一遍，
路径数随 amount 指数增长。

换个角度：dfs(remain) 的结果只取决于 remain，与「是怎么走到这里的」无关。
把算过的 remain 记下来，同一个金额只展开一次。

关键点：
  · 表大小是 amount + 1，下标就是 remain；用 -2 表示「未计算」，
    不能拿 -1 当哨兵 —— -1 本身就是「凑不出」的合法答案
  · remain == 0 返回 0（不需要硬币）；remain 是 usize，减去面额前必须
    先用 `c as usize > remain` 挡住，否则会下溢 panic
  · 面额 0 会让 dfs(remain) 原地调回自己，必须先剔除脏数据
  · 先算子问题再写表；顺序反了缓存里永远存不到东西
  · 与解法二、三同为 O(amount·k) 时间、O(amount) 空间，并列最优；
    它的好处是不用想清楚遍历顺序，代价是递归栈开销

手推一遍（coins = [1,2,5]，amount = 11）：
  dfs(11) 尝试 1、2、5，分别落到 dfs(10)、dfs(9)、dfs(6)
  dfs(6) 用 5 落地 → dfs(1) = 1，故 dfs(6) = 2
  dfs(10) = dfs(5) + 1 = 2（5+5），dfs(9) = dfs(4) + 1 = 3
  dfs(11) = min(2, 3, 2) + 1 = 3 ✔

复杂度分析：
  时间 O(amount·k)：每个 remain 只展开一次，每次枚举 k 枚硬币
  空间 O(amount)：memo 表；递归栈最坏也是 O(amount)
*/

// rustc 单文件编译时没有平台的 Solution，这里显式定义（结构体方案）
pub struct Solution1;

impl Solution1 {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        // -2 表示未计算；-1 是「凑不出」的合法答案，不能当哨兵
        let mut memo = vec![-2i32; amount + 1];
        memo[0] = 0;

        fn dfs(remain: usize, coins: &[i32], memo: &mut Vec<i32>) -> i32 {
            if memo[remain] != -2 {
                return memo[remain];
            }

            let mut best = -1;
            for &c in coins {
                // 面额 0 会让 remain 原地打转；比 remain 大则无路可走
                if c <= 0 || c as usize > remain {
                    continue;
                }
                let sub = dfs(remain - c as usize, coins, memo);
                if sub >= 0 && (best < 0 || sub + 1 < best) {
                    best = sub + 1;
                }
            }

            memo[remain] = best;
            best
        }

        dfs(amount, &coins, &mut memo)
    }
}

/*
解法二：完全背包，容量在外【最优解，与解法一并列】

为什么慢：二维写法 dp[i][v] 表示「用前 i 种硬币凑出 v」，每一行只依赖本行
更小的 v 和上一行，整张表是 O(amount·k) 空间。把硬币种类这一维压掉后，
一行就够，因为「用哪些硬币」这件事已经被「可以无限次取」抹平了。

换个角度：直接让金额从 1 涨到 amount，dp[v] 只由比 v 小的金额推出来，
天然满足无后效性，连遍历顺序都不用纠结。

关键点：
  · 不可达必须初始化为 amount + 1，不能用 i32::MAX：转移里有 dp[v-c] + 1，
    MAX + 1 在 debug 下 panic、release 下静默回绕；amount + 1 比任何合法
    答案（最多 amount 枚）都大，且加一安全
  · dp[0] = 0；最后 dp[amount] > amount 说明没被更新过，翻译成 -1
  · 内层枚举所有面额不超过 v 的硬币，对应「最后一枚硬币是 c」的分法
  · 面额 0 要跳过，否则 dp[v] = dp[v] + 1 会把自己越改越大
  · 面额大于 amount 的硬币永远选不到，由 c > v 自然过滤

手推一遍（coins = [1,2,5]，amount = 11）：
  dp[1] = 1，dp[2] = 1，dp[3] = 2（2+1），dp[5] = 1，dp[6] = 2（5+1）
  dp[9] = 3（5+2+2），dp[10] = 2（5+5）
  dp[11] = min(dp[10], dp[9], dp[6]) + 1 = min(2, 3, 2) + 1 = 3 ✔

复杂度分析：
  时间 O(amount·k)：每个金额枚举 k 枚硬币
  空间 O(amount)：一维 dp 表
*/

pub struct Solution2;

impl Solution2 {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        // amount + 1 是「不可达」哨兵：最多用 amount 枚，加一不会溢出
        let mut dp = vec![amount as i32 + 1; amount + 1];
        dp[0] = 0;

        for v in 1..=amount {
            // 枚举最后一枚硬币：它拿走 c，剩下的 v - c 已经算好
            for &c in &coins {
                if c <= 0 || c as usize > v {
                    continue;
                }
                let cand = dp[v - c as usize] + 1;
                if cand < dp[v] {
                    dp[v] = cand;
                }
            }
        }

        if dp[amount] > amount as i32 {
            -1
        } else {
            dp[amount]
        }
    }
}

/*
解法三：完全背包，物品在外【最优解，与解法一、二并列】

为什么慢：与解法二是同一个状态定义，只是交换了两层循环。这本身不改变
复杂度，但它把「硬币能否重复使用」这件事显式地写在遍历方向里，换成计数类
问题时差别才会暴露（见关键点）。

换个角度：一枚一枚地加入硬币，dp[v] 表示「用到当前这枚为止，凑出 v 的最少
数量」，内层金额正序，相当于允许这枚硬币反复叠加。

关键点：
  · 内层必须正序：正序时 dp[v - c] 可能刚被同一枚硬币更新过，等价于「再添
    一枚 c」；倒序则退化成 0-1 背包，每种硬币只能用一次
  · 与解法二的区别：解法二固定金额 v，问「最后一枚是哪种硬币」；解法三
    固定硬币 c，问「这枚硬币用不用、用几枚」。两条路径覆盖的组合完全一样，
    最少数量必然相同
  · 对 min 转移，两种顺序可以互换；换成计数就不同了 —— 518「零钱兑换 II」
    必须物品在外才是组合数，容量在外会把同一组合的不同排列重复计数
  · 哨兵与 -1 的翻译同解法二：初始 amount + 1，返回前判一次

手推一遍（coins = [1,2,5]，amount = 11）：
  只用 1：dp = [0,1,2,3,4,5,6,7,8,9,10,11]
  加入 2：dp[3] 降到 2（2+1），dp[6] 降到 3（2+2+2），dp[11] 降到 6
  加入 5：dp[5] 降到 1，dp[6] 降到 2（5+1），dp[10] 降到 2（5+5），
          dp[11] = dp[6] + 1 = 3 ✔

复杂度分析：
  时间 O(amount·k)：每枚硬币扫一遍金额区间
  空间 O(amount)：一维 dp 表
*/

pub struct Solution3;

impl Solution3 {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![amount as i32 + 1; amount + 1];
        dp[0] = 0;

        for &c in &coins {
            // 面额 0 会让 dp[v] 与自己相加，是纯脏数据
            if c <= 0 {
                continue;
            }
            let c = c as usize;
            // 正序：dp[v - c] 可能刚被这枚硬币更新过，等价于再用一枚
            for v in c..=amount {
                let cand = dp[v - c] + 1;
                if cand < dp[v] {
                    dp[v] = cand;
                }
            }
        }

        if dp[amount] > amount as i32 {
            -1
        } else {
            dp[amount]
        }
    }
}

// ===== 测试辅助：小规模暴力枚举，作为对拍的独立真值来源 =====
// 不做任何记忆化，枚举所有「先拿哪枚」的顺序，指数级但金额很小时够用
fn brute_coin(remain: i32, coins: &[i32]) -> i32 {
    if remain == 0 {
        return 0;
    }

    let mut best = -1;
    for &c in coins {
        if c <= 0 || c > remain {
            continue;
        }
        let sub = brute_coin(remain - c, coins);
        if sub >= 0 && (best < 0 || sub + 1 < best) {
            best = sub + 1;
        }
    }
    best
}

// 固定种子的线性同余，保证每次运行的对拍用例一致
fn next_rand(seed: &mut u64) -> u64 {
    *seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *seed >> 33
}

fn check(coins: &[i32], amount: i32, want: i32) {
    assert_eq!(
        Solution1::coin_change(coins.to_vec(), amount),
        want,
        "memo {:?} {}",
        coins,
        amount
    );
    assert_eq!(
        Solution2::coin_change(coins.to_vec(), amount),
        want,
        "byAmount {:?} {}",
        coins,
        amount
    );
    assert_eq!(
        Solution3::coin_change(coins.to_vec(), amount),
        want,
        "byCoin {:?} {}",
        coins,
        amount
    );
}

fn cross_check() {
    let mut seed = 322u64;
    for _ in 0..300 {
        let n = 1 + (next_rand(&mut seed) % 4) as usize;
        let mut coins = Vec::with_capacity(n);
        for _ in 0..n {
            coins.push((next_rand(&mut seed) % 5) as i32);
        }
        let amount = (next_rand(&mut seed) % 13) as i32;
        let want = brute_coin(amount, &coins);
        check(&coins, amount, want);
    }
}

fn main() {
    // 示例 1：11 = 5 + 5 + 1
    check(&[1, 2, 5], 11, 3);

    // 示例 2：只有面额 2，凑不出奇数
    check(&[2], 3, -1);

    // 示例 3：金额为 0，一枚硬币都不需要
    check(&[1], 0, 0);

    // 边界：coins 含 0 的脏数据，0 必须被忽略而不是无限叠加
    check(&[0, 1, 2], 5, 3);
    check(&[0], 5, -1);
    check(&[0], 0, 0);

    // 边界：所有面额都大于金额
    check(&[5], 3, -1);

    // 边界：只有 1 元，答案等于金额本身（最大规模 10^4）
    check(&[1], 10000, 10000);

    // 大面额组合：答案是 20，这组用例专门暴露大数初始化加一溢出
    check(&[186, 419, 83, 408], 6249, 20);

    // 对拍：随机小硬币与小金额，与无记忆化的穷举比对
    cross_check();

    println!("all tests passed");
}
