/*
============================================================================
55. 跳跃游戏（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/jump-game/
分类：贪心 · 最远可达位置

给你一个非负整数数组 nums ，你最初位于数组的 第一个下标 。数组中的每个
元素代表你在该位置可以跳跃的最大长度。

判断你是否能够到达最后一个下标，如果可以，返回 true ；否则，返回 false 。

示例 1：
输入：nums = [2,3,1,1,4]
输出：true
解释：可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达
     最后一个下标。

示例 2：
输入：nums = [3,2,1,0,4]
输出：false
解释：无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ，
     所以永远不可能到达最后一个下标。

提示：
- 1 <= nums.length <= 10^4
- 0 <= nums[i] <= 10^5
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法：贪心维护最远可达位置【最优解】

为什么慢：把所有跳跃方案枚举一遍是回溯，方案数随「每步可选几个落点」
指数增长；退一步写 dp[i] 表示「能否到达 i」，也要对每个 i 回看它之前的
所有位置，时间 O(n^2)。瓶颈是「同一个可达性被反复重算」。

换个角度：可达位置永远是一个前缀。若下标 x 可达，那么把最后一跳跳短，
0..x 之间的任何位置都可达 —— 可达集合的形状是固定的，用一个整数就能
描述，即「目前能到达的最远下标」rightmost。

关键点：
  · 先判 i > rightmost：此时 i 及其之后全部不可达，循环可以直接结束，
    不必再读 nums[i]
  · 只在 i <= rightmost 时用 i + nums[i] 更新 rightmost：不可达位置的
    跳跃长度没有意义，用它去扩张会凭空造出可达区间
  · 命中条件写成 rightmost >= n - 1：最后一个下标无需再作为起跳点
  · n == 1 时第一轮就满足命中条件，返回 true，无需特判
  · 不能只看「当前格是否为 0」：能不能跨过 0 取决于前面累积的最远距离

以示例 1（nums = [2,3,1,1,4]）为例：
  i = 0：rightmost = 0 + 2 = 2
  i = 1：1 <= 2，rightmost = max(2, 1 + 3) = 4 >= 4 → 返回 true ✔
以示例 2（nums = [3,2,1,0,4]）为例：
  i = 0：rightmost = 3
  i = 1、2、3 都在范围内，但 i + nums[i] 都不超过 3
  i = 4：4 > 3，跳出循环 → 返回 false ✔

复杂度分析：
  时间 O(n)：每个下标最多读一次，更新与比较都是常数次
  空间 O(1)：只用 rightmost 与循环变量
*/
class Solution
{
public:
    bool canJump(vector<int> &nums)
    {
        int n = static_cast<int>(nums.size());
        int rightmost = 0;

        for (int i = 0; i < n; ++i)
        {
            // i 已经越过最远可达位置，后面的下标同样到不了
            if (i > rightmost) break;

            rightmost = max(rightmost, i + nums[i]);
            if (rightmost >= n - 1) return true;
        }
        return false;
    }
};

// ===== 测试辅助 =====
namespace
{
void check(vector<int> nums, bool want)
{
    Solution s;
    assert(s.canJump(nums) == want);
}
} // namespace

int main()
{
    // 示例 1：中途落在 1，靠 3 一步跨到终点
    check({2, 3, 1, 1, 4}, true);

    // 示例 2：下标 3 的 0 截断了去路
    check({3, 2, 1, 0, 4}, false);

    // 边界：单元素，起点即终点
    check({0}, true);
    check({5}, true);

    // 边界：两步之间断开
    check({0, 1}, false);
    check({1, 0, 1}, false);

    // 边界：首格一步跨过所有 0
    check({2, 0, 0}, true);
    check({2, 0, 0, 0}, false);

    // 反直觉：当前格是 0 但仍在可达范围内，不影响结论
    check({3, 0, 0, 1, 0}, true);

    // 边界：全程只能一格一格走
    {
        vector<int> nums(10000, 1);
        check(nums, true);
    }

    // 边界：长度 10^4，只有首格能跳一步，其余全是 0
    {
        vector<int> nums(10000, 0);
        nums[0] = 1;
        check(nums, false);
    }

    cout << "all tests passed" << endl;
    return 0;
}
