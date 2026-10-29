/*
============================================================================
494. 目标和（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/target-sum/
分类：动态规划 · 0-1 背包（方案计数）
============================================================================

给你一个整数数组 nums 和一个整数 target 。

向数组中的每个整数前添加 '+' 或 '-' ，然后串联起所有整数，可以构造一个
表达式 ：

例如，nums = [2, 1] ，可以在 2 之前添加 '+' ，在 1 之前添加 '-' ，
然后串联起来得到表达式 "+2-1" 。

返回可以通过上述方法构造的、运算结果等于 target 的不同 表达式 的数目。

示例 1：
输入：nums = [1,1,1,1,1], target = 3
输出：5
解释：一共有 5 种方法让最终目标和为 3 。
-1 + 1 + 1 + 1 + 1 = 3
+1 - 1 + 1 + 1 + 1 = 3
+1 + 1 - 1 + 1 + 1 = 3
+1 + 1 + 1 - 1 + 1 = 3
+1 + 1 + 1 + 1 - 1 = 3

示例 2：
输入：nums = [1], target = 1
输出：1

提示：
- 1 <= nums.length <= 20
- 0 <= nums[i] <= 1000
- 0 <= sum(nums[i]) <= 1000
- -1000 <= target <= 1000
*/
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：回溯枚举所有符号【非最优】

为什么慢：每个数都有 + 和 - 两种选择，一共 2^n 条路径。n = 20 时约 10^6，
勉强能过；但其中大量路径的前缀是重复的 —— 「已经用了前 i 个数、还要凑
remain」这个状态被反复展开，规模一变就撑不住。

换个角度：决定方案数的只有「还剩哪些数」和「还要凑多少」，与「前面的符号
是怎么排的」无关。把这两个量当作状态记下来，同一状态只算一次。

关键点：
  · 叶子在「所有数都带完符号」处判断，不能中途判断 target，因为后面还有
    数可以正负抵消
  · nums[i] = 0 时 +0 与 -0 是两条不同的表达式，必须分别递归，计数会翻倍
  · 剪枝用后缀和：|remain| 比剩余所有数之和还大时，这条分支整体无解
  · 前缀和剪枝只能砍枝，不能替代状态记忆，最坏仍是 2^n

以示例 1（nums = [1,1,1,1,1]，target = 3）为例：
  全取 + 时和为 5，要降到 3 需要恰好把一个 +1 改成 -1（和减 2）
  5 个位置任选一个 → 5 种表达式 ✔

复杂度分析：
  时间 O(2^n)：每条符号路径都要走到底，剪枝只削减常数
  空间 O(n)：递归栈深度等于元素个数
*/
class Solution1
{
public:
    int findTargetSumWays(vector<int> &nums, int target)
    {
        this->nums = nums;
        int n = static_cast<int>(nums.size());
        // suffix[i]：nums[i..n-1] 之和，用来判断 remain 是否还有救
        suffix.assign(n + 1, 0);
        for (int i = n - 1; i >= 0; --i) suffix[i] = suffix[i + 1] + nums[i];
        return dfs(0, target);
    }

private:
    vector<int> nums;
    vector<int> suffix;

    int dfs(int i, int remain)
    {
        // 剩余数全取同号也够不到 remain，整棵子树可以剪掉
        if (abs(remain) > suffix[i]) return 0;
        // 符号已全部确定，只有当 remain 归零才算一种表达式
        if (i == static_cast<int>(nums.size())) return remain == 0 ? 1 : 0;
        // 当前数取 + 时 remain 减少，取 - 时 remain 增加，两条都要走
        return dfs(i + 1, remain - nums[i]) + dfs(i + 1, remain + nums[i]);
    }
};

/*
解法二：记忆化搜索（状态 i 与 remain）【非最优】

为什么慢：解法一的瓶颈是同一状态被重复展开。以 [1,1,1,1,1] 为例，
「用了 2 个数、还要凑 1」这组状态会被 C(5,2) 条不同的符号排列各走一遍。

换个角度：dfs(i, remain) 的返回值只取决于 (i, remain)，把它存进表里，
第二次遇到直接取。状态数被压到 (n+1) 乘 (sum+1)。

关键点：
  · remain 用绝对值当 key：把剩余所有符号同时取反，方案数不变，所以
    f(i, r) 与 f(i, -r) 相等，只存一个方向就够了
  · memo 用 -1 表示「没算过」，不能拿 0 当哨兵 —— 0 是合法答案
  · 先判 remain > suffix[i] 再查表，保证下标不会越过 sum
  · 递归返回后立刻写回 memo，否则同一状态还是会重复进入
  · 与解法三时间同为 O(n·sum)，但要多存一张 n 倍的表，因此不是最优

手推一遍（nums = [1,1]，target = 0）：
  f(0, 0) = f(1, 1) + f(1, 1) = 1 + 1 = 2
  f(1, 1) 只有「用 1 取 + 凑掉」一条路 → 1
  两个 1 一正一负，两种排列 → 2 ✔

复杂度分析：
  时间 O(n·sum)：状态数 (n+1)×(sum+1)，每个状态 O(1) 转移
  空间 O(n·sum)：memo 表；递归栈另占 O(n)
*/
class Solution2
{
public:
    int findTargetSumWays(vector<int> &nums, int target)
    {
        this->nums = nums;
        n = static_cast<int>(nums.size());
        suffix.assign(n + 1, 0);
        for (int i = n - 1; i >= 0; --i) suffix[i] = suffix[i + 1] + nums[i];

        // |target| 已经超过总和，任何符号组合都到不了，直接返回 0
        if (abs(target) > suffix[0]) return 0;

        // memo[i][r]：用 nums[i..] 凑出 r 的方案数；-1 表示尚未计算
        memo.assign(n + 1, vector<int>(suffix[0] + 1, -1));
        return dfs(0, abs(target));
    }

private:
    int n;
    vector<int> nums;
    vector<int> suffix;
    vector<vector<int>> memo;

    int dfs(int i, int remain)
    {
        // 剩余数的总和都够不到，直接 0，同时也保证 remain 不越界
        if (remain > suffix[i]) return 0;
        if (i == n) return remain == 0 ? 1 : 0;

        int &slot = memo[i][remain];
        if (slot != -1) return slot;

        // 取 + 后还需 |remain - nums[i]|；取 - 后还需 remain + nums[i]
        slot = dfs(i + 1, abs(remain - nums[i])) + dfs(i + 1, remain + nums[i]);
        return slot;
    }
};

/*
解法三：数学转化 + 一维 0-1 背包【最优解】

为什么慢：解法二把「当前和」当状态，范围是 [-sum, sum]，还多背了 i 这个
维度。其实符号不用逐个决定 —— 记取正号的数之和为 P，取负号的数之和为 N，
则 P - N = target 且 P + N = total，两个式子一解就得到 P = (total+target)/2。

换个角度：问题变成「从 nums 里选出若干个数，使它们的和恰为 P，有多少种
选法」。每个数选或不选、且只能用一次，这是标准的 0-1 背包计数。

关键点：
  · 先判 P 合法：(total + target) 必须非负且为偶数，否则无解；P 还须
    不超过 total
  · dp[0] = 1：一个都不选、和为 0，这是 1 种方案。它是所有计数的起点，
    若写成 dp[0] = 0，整张表会永远是 0
  · 内层倒序：dp[j] += dp[j - nums[i]]，倒序时 dp[j - nums[i]] 还是上一轮
    的值，保证每个数只用一次；正序会把它当成无限个，方案数被放大
  · nums[i] = 0 时每个 j 都执行 dp[j] += dp[j]，等于整体乘 2，正好对应
    +0 与 -0 两种写法
  · 也可以数负号：N = (total - target)/2，由「取补集」与数正号一一对应，
    两种数法结果相同

手推一遍（nums = [1,1,1,1,1]，target = 3）：
  total = 5，P = (5 + 3)/2 = 4
  逐个加入 1 后的 dp（下标 0..4）：
    初始 [1,0,0,0,0]
    第 1 个 [1,1,0,0,0]
    第 2 个 [1,2,1,0,0]
    第 3 个 [1,3,3,1,0]
    第 4 个 [1,4,6,4,1]
    第 5 个 [1,5,10,10,5] → dp[4] = 5 ✔

复杂度分析：
  时间 O(n·P)：n 件物品 × P 个和值，P 不超过 total ≤ 1000
  空间 O(P)：只留一行；比赛法二少了 n 倍
*/
class Solution3
{
public:
    int findTargetSumWays(vector<int> &nums, int target)
    {
        int total = 0;
        for (int v : nums) total += v;

        // P - N = target，P + N = total ⇒ P = (total + target) / 2
        int twice = total + target;
        // P 必须是非负整数：twice 为负或为奇数时无解
        if (twice < 0 || twice % 2 != 0) return 0;
        int positive = twice / 2;
        // 正号部分最多取到全部元素，超了说明 target 太大
        if (positive > total) return 0;

        // dp[j]：从已处理的数里选出和为 j 的方案数
        vector<int> dp(positive + 1, 0);
        dp[0] = 1;

        for (int value : nums)
        {
            // 倒序：value 只能用一次，dp[j - value] 必须停在上一轮
            for (int j = positive; j >= value; --j)
            {
                dp[j] += dp[j - value];
            }
        }
        return dp[positive];
    }
};

// ===== 测试辅助：小规模暴力枚举，作为对拍的独立真值来源 =====
namespace
{
// 2^n 枚举每个数前的符号，统计等于 target 的表达式数
int bruteForce(const vector<int> &nums, int target)
{
    int n = static_cast<int>(nums.size());
    int count = 0;
    for (int mask = 0; mask < (1 << n); ++mask)
    {
        int sum = 0;
        for (int i = 0; i < n; ++i)
        {
            sum += ((mask & (1 << i)) != 0) ? nums[i] : -nums[i];
        }
        if (sum == target) ++count;
    }
    return count;
}

void check(const vector<int> &nums, int target, int want)
{
    Solution1 s1;
    Solution2 s2;
    Solution3 s3;
    vector<int> a = nums;
    vector<int> b = nums;
    vector<int> c = nums;
    assert(s1.findTargetSumWays(a, target) == want);
    assert(s2.findTargetSumWays(b, target) == want);
    assert(s3.findTargetSumWays(c, target) == want);
}

// 固定种子的线性同余，保证每次运行的对拍用例一致
unsigned nextRand(unsigned &seed)
{
    seed = seed * 1103515245u + 12345u;
    return seed;
}

void crossCheck()
{
    unsigned seed = 494u;
    for (int round = 0; round < 300; ++round)
    {
        int n = 1 + static_cast<int>(nextRand(seed) % 9u);
        vector<int> nums(n);
        for (int i = 0; i < n; ++i)
        {
            nums[i] = static_cast<int>(nextRand(seed) % 4u);
        }
        int target = static_cast<int>(nextRand(seed) % 9u) - 4;
        check(nums, target, bruteForce(nums, target));
    }
}
} // namespace

int main()
{
    // 示例 1：五个 1 里挑一个取负号
    check({1, 1, 1, 1, 1}, 3, 5);

    // 示例 2：单元素只能取 +
    check({1}, 1, 1);

    // 边界：单元素取负号
    check({1}, -1, 1);

    // 边界：target 超过总和，一条表达式都凑不出
    check({1, 2}, 5, 0);
    check({1, 2}, -5, 0);

    // 边界：总和与 target 奇偶不匹配，(total+target) 不是偶数
    check({1, 2}, 0, 0);

    // 边界：含 0 元素，+0 与 -0 是两条不同表达式，计数翻倍
    check({0, 0, 0}, 0, 8);
    check({0}, 0, 2);
    check({0}, 1, 0);

    // 边界：单元素取本身的值，恰好一种
    check({1000}, 1000, 1);
    check({1000}, 999, 0);

    // 最大规模：20 个 1，target = 0 需要 10 正 10 负，C(20,10)
    {
        vector<int> nums(20, 1);
        check(nums, 0, 184756);
    }

    // 对拍：随机小数组（含 0）与暴力枚举逐组比对
    crossCheck();

    cout << "all tests passed" << endl;
    return 0;
}
