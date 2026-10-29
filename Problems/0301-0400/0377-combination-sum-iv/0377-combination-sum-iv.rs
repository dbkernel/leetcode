/*
============================================================================
377. 组合总和 Ⅳ（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/combination-sum-iv/
分类：动态规划 · 完全背包排列计数

给你一个由 不同 整数组成的数组 nums ，和一个目标整数 target 。请你从 nums 中找出并返回总和为 target 的元素组合的个数。

题目数据保证答案符合 32 位整数范围。

示例 1：
输入：nums = [1,2,3], target = 4
输出：7
解释：
所有可能的组合为：
(1, 1, 1, 1)
(1, 1, 2)
(1, 2, 1)
(1, 3)
(2, 1, 1)
(2, 2)
(3, 1)
请注意，顺序不同的序列被视作不同的组合。

示例 2：
输入：nums = [9], target = 3
输出：0

提示：
- 1 <= nums.length <= 200
- 1 <= nums[i] <= 1000
- nums 中的所有元素 互不相同
- 1 <= target <= 1000

进阶：如果给定的数组中含有负数会发生什么？问题会产生何种变化？如果允许负数出现，需要向题目中添加哪些限制条件？
*/

/*
解法一：记忆化 DFS，自顶向下枚举最后一个数【最优解】

为什么慢：最朴素的搜索是从 0 开始，每一步在 nums 里挑一个数往上加，直到
正好到 target。同一批"剩余金额"会被不同的前缀反复展开，分支数随 target
指数增长——而它们其实只是同一个子问题。

换个角度：只关心"还差多少"。要凑出 rest，就枚举最后一个数 x，剩下的
rest-x 与原问题同构。状态只有一个维度，算过就存。

关键点：
  · dfs(0) = 1 而不是 0：rest 归零表示"序列已经结束且恰好凑满"，空序列
    本身就是一种合法方案，返回 0 会抹掉所有方案的末位
  · 状态只需 rest，不记录"用过哪些数"：本题允许重复取元素，
    且顺序不同算不同方案，所以集合信息没有意义
  · 判重必须在进入循环之前：先查 memo 再展开，否则记忆化只省了赋值
  · 用 i64 累加，不让正确性依赖"答案在 32 位内"这条保证；返回时再收窄

以示例 1（nums = [1,2,3], target = 4）为例：
  f(0) = 1
  f(1) = f(0) = 1
  f(2) = f(1) + f(0) = 2
  f(3) = f(2) + f(1) + f(0) = 4
  f(4) = f(3) + f(2) + f(1) = 4 + 2 + 1 = 7 ✔（f(4-3) = f(1)）

复杂度分析：
  时间 O(n·target)：target+1 个状态，每个状态扫一遍 nums
  空间 O(target)：memo 表 target+1 格，外加深度不超过 target 的递归栈
*/
pub struct Solution1;

impl Solution1 {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        // memo[r]：凑出 r 的排列数，-1 表示尚未计算
        let mut memo = vec![-1i64; target + 1];
        memo[0] = 1;
        Self::dfs(&nums, target, &mut memo) as i32
    }

    fn dfs(nums: &[i32], rest: usize, memo: &mut [i64]) -> i64 {
        if memo[rest] != -1 {
            return memo[rest];
        }

        let mut total = 0i64;
        for &x in nums {
            // 放不下 x 的分支不存在，继续下一枚
            if x as usize > rest {
                continue;
            }
            total += Self::dfs(nums, rest - x as usize, memo);
        }

        memo[rest] = total;
        total
    }
}

/*
解法二：完全背包 DP，容量在外、物品在内【最优解，与解法一并列】

为什么慢：解法一自顶向下递归，栈深等于 target，且每层都要反复做减法与
函数调用；对于"每个金额都可达"的输入，没有状态被跳过，递归的额外开销
全是纯损失。

换个角度：把递推倒过来写成自底向上。dp[j] 表示和为 j 的排列数，枚举
最后一个数 x，则 dp[j] += dp[j-x]。因为 x 可以放在序列的任意末端，
每个"最后一步"都是一类新的排列。

关键点：
  · 容量（金额）必须在外层，物品（nums）在内层。这样 dp[j-x] 统计的是
    "任意开头、以 x 结尾"的全部排列，把 x 接上去就得到新的排列 —— 数的是
    排列数
  · 一旦两层调换（物品在外、容量在内正序），每种元素只在自己那一轮被
    无限取用，得到的就是组合数，即 518 零钱兑换 II。同一组输入
    nums=[1,2,5]、target=5，本题得 9，518 得 4
  · dp[0] = 1：空序列是唯一的底部方案，缺了它所有累加都停在 0
  · 用 i64 累加，中间值可能先于最终答案触到 32 位上限
  · 与解法一同为 O(n·target) 时间、O(target) 空间，故并列最优：解法二
    没有递归栈、常数更小；解法一在大量金额不可达时只展开访问到的状态，
    稀疏输入上更快

以示例 1（nums = [1,2,3], target = 4）为例：
  dp = [1,0,0,0,0]
  j=1：dp[1] += dp[0] → 1
  j=2：dp[2] += dp[1] + dp[0] → 2
  j=3：dp[3] += dp[2] + dp[1] + dp[0] → 4
  j=4：dp[4] += dp[3] + dp[2] + dp[1] → 4 + 2 + 1 = 7 ✔

复杂度分析：
  时间 O(n·target)：target 个金额 × n 个候选数
  空间 O(target)：一行 dp 表，没有递归栈
*/
pub struct Solution2;

impl Solution2 {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0i64; target + 1];
        dp[0] = 1;

        for j in 1..=target {
            for &x in &nums {
                let x = x as usize;
                // dp[j-x] 的所有排列都能在末尾接上 x
                if j >= x {
                    dp[j] += dp[j - x];
                }
            }
        }

        dp[target] as i32
    }
}

// 物品在外、容量在内：数出来的是组合数，用来对照本题的排列数
fn count_combinations(nums: &[i32], target: usize) -> i64 {
    let mut dp = vec![0i64; target + 1];
    dp[0] = 1;

    for &x in nums {
        let x = x as usize;
        for j in x..=target {
            dp[j] += dp[j - x];
        }
    }
    dp[target]
}

fn check(nums: &[i32], target: i32, want: i32) {
    assert_eq!(Solution1::combination_sum4(nums.to_vec(), target), want);
    assert_eq!(Solution2::combination_sum4(nums.to_vec(), target), want);
}

fn main() {
    // 示例 1：官方列的 7 种排列，含 (1,2,1) 与 (2,1,1) 这类换序方案
    check(&[1, 2, 3], 4, 7);

    // 示例 2：唯一元素大于目标，方案数是 0
    check(&[9], 3, 0);

    // 边界：只有面额 1，任何目标都只有一种排列
    check(&[1], 5, 1);

    // 边界：目标由同一个数重复拼成，只有一条排列
    check(&[2], 4, 1);

    // 关键点用例：与 518 对同一输入的结果不同
    //   [1,2] 拼 3：排列 3 种（111、12、21），组合只有 2 种（111、12）
    check(&[1, 2], 3, 3);
    assert_eq!(count_combinations(&[1, 2], 3), 2);

    // 关键点用例：[1,2] 拼 4：排列 5 种，组合 3 种（1111、112、22）
    check(&[1, 2], 4, 5);
    assert_eq!(count_combinations(&[1, 2], 4), 3);

    // 关键点用例：对照 518 的示例 1，同一输入组本题得 9、518 得 4
    check(&[1, 2, 5], 5, 9);
    assert_eq!(count_combinations(&[1, 2, 5], 5), 4);

    // 边界：候选数顺序不影响结果，写反两层循环才会出错
    check(&[3, 1], 4, 3);

    println!("all tests passed");
}
