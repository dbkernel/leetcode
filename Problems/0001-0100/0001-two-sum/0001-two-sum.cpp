/*
============================================================================
LeetCode 1. 两数之和（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/two-sum/
分类：哈希表 · 两数之和查找（暴力枚举 / 一次遍历）
============================================================================

给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出
和为目标值 target 的那 两个 整数，并返回它们的数组下标。

你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。

你可以按任意顺序返回答案。

示例 1：
输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。

示例 2：
输入：nums = [3,2,4], target = 6
输出：[1,2]

示例 3：
输入：nums = [3,3], target = 6
输出：[0,1]

提示：
2 <= nums.length <= 10^4
-10^9 <= nums[i] <= 10^9
-10^9 <= target <= 10^9
只会存在一个有效答案

进阶：你可以想出一个时间复杂度小于 O(n^2) 的算法吗？
*/
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <cassert>
#include <iostream>
using namespace std;

/*
解法一：暴力枚举（固定左端，只向右找）

为什么慢：对每个 x 都要在数组里找 target - x。若让内层循环从头扫到尾，
每个 x 都要扫 n 个位置，其中一半是和之前已经匹配过的元素重复比较，
总计 O(n^2) 对。

换个角度：(i, j) 与 (j, i) 是同一次配对，只要限定 i < j，每对就只枚举
一次，既不会漏解也不会重复比较。

关键点：
  · 内层从 i + 1 起步 —— 它同时排除了「同一个元素用两次」和重复配对
  · 命中立刻返回 —— 题目保证只有一个答案，继续扫也不会改变结果

以示例 1（nums = [2,7,11,15], target = 9）为例：
  i = 0, x = 2：j = 1 查 2 + 7 = 9 → 命中，返回 [0, 1]

复杂度分析：
  时间 O(n^2)：最坏比较 n(n-1)/2 对
  空间 O(1)：除返回值外只用两个下标
*/
class Solution1
{
public:
    // 返回一组合法下标 (i, j)，且保证 i < j
    vector<int> twoSum(vector<int> &nums, int target)
    {
        const int n = static_cast<int>(nums.size());

        for (int i = 0; i < n; ++i)
        {
            // j 从 i + 1 起步：同一个元素不会被用两次
            for (int j = i + 1; j < n; ++j)
            {
                // nums[i] + nums[j] 最大 2*10^9，未超过 int 上限，不会溢出
                if (nums[i] + nums[j] == target)
                {
                    // 题目保证只有一个答案，此处可以安全地直接返回
                    return {i, j};
                }
            }
        }

        // 题目保证有解，走不到这里；保留是为了让控制流完整
        return {};
    }
};

/*
解法二：哈希表（一次遍历）

为什么慢：解法一每次查找 target - x 都要重新扫一遍数组，单次 O(n)。

换个角度：对每个新元素 x，只需在「已遍历的前缀」里反向查找 target - x。
于是把前缀的「值 → 下标」记入哈希表，查找就从 O(n) 降到 O(1)。

关键点：先查后插，两句话顺序不能反。
  · 先查 —— 表里只有下标 < i 的元素，天然排除「拿自己配自己」
  · 后插 —— nums[i] 要到第 i 步之后才对后续元素可见，保证 i != j
所以 [3, 3], target = 6 也能得到 [0, 1]：第二个 3 查到的是第一个 3。

同一数值只需留一个下标：题目保证答案唯一，同值元素互换不影响合法性，
因此重复写入覆盖旧下标不会丢失答案。

以示例 1（nums = [2,7,11,15], target = 9）为例：
  i = 0, x = 2：查 9 - 2 = 7 → 未命中；写入 {2: 0}
  i = 1, x = 7：查 9 - 7 = 2 → 命中下标 0；返回 [0, 1]

复杂度分析：
  时间 O(n)：每个元素进出表各一次，哈希表摊还 O(1)
  空间 O(n)：最坏情况下前 n-1 个数都留在表里
*/
class Solution2
{
public:
    // 返回一组合法下标 (i, j)，满足 nums[i] + nums[j] == target 且 i != j
    vector<int> twoSum(vector<int> &nums, int target)
    {
        // 值 → 下标，只保存「已经扫过」的那部分元素
        unordered_map<int, int> seen;

        for (int i = 0; i < static_cast<int>(nums.size()); ++i)
        {
            int complement = target - nums[i];

            // 只在前缀里找：命中时 seen 中存的下标必然 < i，不会是自己
            auto it = seen.find(complement);
            if (it != seen.end())
            {
                return {it->second, i};
            }

            // 未命中则登记当前元素，供后续元素匹配
            // seen[x] = i 会覆盖旧下标，insert / emplace 不会；本题两者都正确，
            // 因为同值元素互换后仍是同一组解
            seen[nums[i]] = i;
        }

        // 题目保证有解，走不到这里；保留是为了让控制流完整
        return {};
    }
};

namespace
{
// 题目允许任意顺序返回，先排序再比较，断言就不依赖实现的返回顺序
bool samePair(vector<int> a, vector<int> b)
{
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    return a == b;
}

// 同一个用例喂给两种解法：官方两解都必须得到同一组下标
void check(const vector<int> &nums, int target, const vector<int> &want)
{
    Solution1 brute;
    Solution2 hash;

    // twoSum 参数是非 const 引用，各传一份副本，避免解法之间互相影响
    vector<int> a = nums;
    vector<int> b = nums;

    assert(samePair(brute.twoSum(a, target), want));
    assert(samePair(hash.twoSum(b, target), want));

    // 两种解法都不应改写输入，否则上面的副本会先被断言暴露
    assert(a == nums);
    assert(b == nums);
}
} // namespace

int main()
{
    // 示例 1：2 + 7 == 9
    check({2, 7, 11, 15}, 9, {0, 1});

    // 示例 2：答案不落在开头
    check({3, 2, 4}, 6, {1, 2});

    // 示例 3：重复值 —— 解法二「先查后插」的关键用例
    check({3, 3}, 6, {0, 1});

    // 边界：数组含 0，且答案用到下标 0，专防「按下标真假判断」的写法
    check({0, 4, 3, 0}, 0, {0, 3});

    // 边界：负数参与求和，且答案不与 0 号下标相邻
    check({-3, 4, 3, 90}, 0, {0, 2});

    // 边界：取到约束上限，验证两数相加不会溢出
    check({1000000000, 1000000000}, 2000000000, {0, 1});

    cout << "all tests passed" << endl;
    return 0;
}
