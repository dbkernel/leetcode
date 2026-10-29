/*
============================================================================
LeetCode 1. 两数之和（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/two-sum/
分类：哈希表 · 一次遍历
============================================================================

给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那 两个 整数，并返回它们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
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
解法：哈希表（一次遍历）

朴素做法：对每个 x 再向后扫一遍找 target - x，单次查找 O(n)，总计 O(n^2)。
换个角度看：对每个新元素 x，只需在「已遍历的前缀」里反向查找 target - x。
于是把前缀的「值 → 下标」记入哈希表，查找就从 O(n) 降到 O(1)。

关键：先查后插，两句话顺序不能反。
  · 先查 —— 表里只有下标 < i 的元素，天然排除「拿自己配自己」
  · 后插 —— nums[i] 要到第 i 步之后才对后续元素可见，保证 i != j
所以 [3, 3], target = 6 也能得到 [0, 1]：第二个 3 查到的是第一个 3。

同一数值只需留一个下标：题目只要求任意一组解，同值元素互换不影响合法性，
因此重复写入覆盖旧下标不会丢失答案。

以示例 1（nums = [2,7,11,15], target = 9）为例：
  i = 0, x = 2：查 9 - 2 = 7 → 未命中；写入 {2: 0}
  i = 1, x = 7：查 9 - 7 = 2 → 命中下标 0；返回 [0, 1]

复杂度分析：
  时间 O(n)：每个元素进出表各一次，哈希表摊还 O(1)
  空间 O(n)：最坏情况下前 n-1 个数都留在表里
*/
class Solution
{
public:
    // 返回任意一组两个下标 (i, j)，满足 nums[i] + nums[j] == target 且 i != j
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

        // 题目保证有解，走不到这里；保留是为了让控制流完整，避免落到无返回值的路径
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
} // namespace

int main()
{
    Solution s;

    // 示例 1：2 + 7 == 9
    vector<int> n1 = {2, 7, 11, 15};
    assert(samePair(s.twoSum(n1, 9), {0, 1}));

    // 示例 2：答案不落在开头
    vector<int> n2 = {3, 2, 4};
    assert(samePair(s.twoSum(n2, 6), {1, 2}));

    // 示例 3：重复值 —— 「先查后插」的关键用例
    vector<int> n3 = {3, 3};
    assert(samePair(s.twoSum(n3, 6), {0, 1}));

    // 重复值本身不是答案：重复写入覆盖旧下标，但不得丢失解
    vector<int> n4 = {3, 3, 5};
    assert(samePair(s.twoSum(n4, 8), {1, 2}));

    cout << "all tests passed" << endl;
    return 0;
}
