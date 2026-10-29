/*
============================================================================
34. 在排序数组中查找元素的第一个和最后一个位置（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/
分类：二分查找 · 左右边界

给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。
请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 target，返回 [-1, -1]。

你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。

示例 1：
输入：nums = [5,7,7,8,8,10], target = 8
输出：[3,4]

示例 2：
输入：nums = [5,7,7,8,8,10], target = 6
输出：[-1,-1]

示例 3：
输入：nums = [], target = 0
输出：[-1,-1]

提示：
- 0 <= nums.length <= 10^5
- -10^9 <= nums[i] <= 10^9
- nums 是一个非递减数组
- -10^9 <= target <= 10^9
*/
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：一次二分命中后向两侧线性扩展【非最优】

为什么慢：二分 O(log n) 就能命中任意一个 target，但左右边界还要从命中点
向两边逐个走。数组里全是 target 时（如 n 个 8），这一步要扫完整个数组，
整体退化成 O(n)，不满足题目要求的 O(log n)。

换个角度：左右边界本质上各自是一次「找第一个满足条件的位置」：
左边界是第一个 >= target 的位置，右边界是第一个 > target 的位置减一。
把「逐个走」换成「再二分一次」，线性项就消失了。

关键点：
  · 先二分找到任意一个 target 作为起点，没找到直接返回 [-1, -1]
  · 向左扩展前先判 lo > 0，再取 nums[lo - 1]，否则会越界
  · 向右扩展同理，先判 hi + 1 < n
  · 扩展用 while 而不是 if，因为连续 target 段可能很长
  · 这段思路只用来对照，规模大且 target 密集时会超时

以示例 1（nums = [5,7,7,8,8,10], target = 8）为例：
  二分命中下标 4（值 8）
  向左：nums[3] = 8 → lo = 3；nums[2] = 7 停
  向右：nums[5] = 10 停 → hi = 4
  返回 [3,4] ✔

复杂度分析：
  时间 O(log n + k)：k 为 target 的出现次数，全相同时退化为 O(n)
  空间 O(1)：只用了常数个下标
*/
class Solution1
{
public:
    vector<int> searchRange(vector<int> &nums, int target)
    {
        int n = static_cast<int>(nums.size());
        int left = 0, right = n - 1, hit = -1;

        while (left <= right)
        {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target)
            {
                hit = mid;
                break;
            }
            if (nums[mid] < target) left = mid + 1;
            else right = mid - 1;
        }

        if (hit == -1) return {-1, -1};

        int lo = hit, hi = hit;
        // 先判下标再取值，避免 hit 正好在端点时越界
        while (lo > 0 && nums[lo - 1] == target) --lo;
        while (hi + 1 < n && nums[hi + 1] == target) ++hi;

        return {lo, hi};
    }
};

namespace
{
// 第一个 >= target 的下标；全部小于 target 时返回 n
int lowerBound(const vector<int> &nums, int target)
{
    int left = 0, right = static_cast<int>(nums.size());
    while (left < right)
    {
        int mid = left + (right - left) / 2;
        if (nums[mid] < target) left = mid + 1;
        else right = mid;
    }
    return left;
}

// 第一个 > target 的下标；全部不大于 target 时返回 n
int upperBound(const vector<int> &nums, int target)
{
    int left = 0, right = static_cast<int>(nums.size());
    while (left < right)
    {
        int mid = left + (right - left) / 2;
        if (nums[mid] <= target) left = mid + 1;
        else right = mid;
    }
    return left;
}
} // namespace

/*
解法二：两次二分分别找左右边界【最优解】

为什么慢：解法一在命中的那一段上仍然线性扫描。target 越密集，这段扫描
越长，最坏 O(n)，没有把「有序」这个前提用满。

换个角度：两个边界都可以用同一个下界函数表达：
  · 左边界 = 第一个 >= target 的位置；
  · 右边界 = 第一个 > target 的位置减一。
所以只要写一个能接受「比较条件」的下界二分，调用两次即可。

关键点：
  · lowerBound 用 `nums[mid] < target` 收缩左界
  · upperBound 用 `nums[mid] <= target` 收缩左界，差别只在等号
  · 先拿 lowerBound 判断存在性：越界或值不等就返回 [-1, -1]，
    这样后面求右边界时不会出现空区间
  · 右边界用 upperBound - 1，而不是再写一个「最后一个 == target」的二分，
    避免两套边界条件记混
  · target 等于 10^9 时也不要写 target + 1 来找上界，直接比较 <= 更安全

以示例 1（nums = [5,7,7,8,8,10], target = 8）为例：
  lowerBound：第一个 >= 8 是下标 3
  upperBound：第一个 > 8 是下标 5，减一得 4
  返回 [3,4] ✔

以示例 2（nums = [5,7,7,8,8,10], target = 6）为例：
  lowerBound 返回 3，nums[3] = 8 != 6 → 返回 [-1,-1] ✔

复杂度分析：
  时间 O(log n)：两次二分，每次区间减半
  空间 O(1)：只用了常数个下标
*/
class Solution2
{
public:
    vector<int> searchRange(vector<int> &nums, int target)
    {
        int lo = lowerBound(nums, target);
        // 越界说明 target 比所有元素都大；值不等说明不存在
        if (lo == static_cast<int>(nums.size()) || nums[lo] != target)
            return {-1, -1};

        // upperBound 指向第一个大于 target 的位置，减一即最后一个 target
        int hi = upperBound(nums, target) - 1;
        return {lo, hi};
    }
};

namespace
{
void check(vector<int> nums, int target, vector<int> want)
{
    Solution1 s1;
    Solution2 s2;
    assert(s1.searchRange(nums, target) == want);
    assert(s2.searchRange(nums, target) == want);
}
} // namespace

int main()
{
    // 示例 1
    check({5, 7, 7, 8, 8, 10}, 8, {3, 4});

    // 示例 2
    check({5, 7, 7, 8, 8, 10}, 6, {-1, -1});

    // 示例 3：空数组
    check({}, 0, {-1, -1});

    // 边界：单元素命中 / 未命中
    check({1}, 1, {0, 0});
    check({1}, 0, {-1, -1});

    // 边界：target 在数组两端
    check({1, 2, 3}, 1, {0, 0});
    check({1, 2, 3}, 3, {2, 2});

    // 边界：target 小于最小值 / 大于最大值
    check({1, 2, 3}, 0, {-1, -1});
    check({1, 2, 3}, 4, {-1, -1});

    // 边界：全部元素相同，解法一的线性扩展会走满整个数组
    check({2, 2, 2, 2}, 2, {0, 3});
    check({2, 2, 2, 2}, 3, {-1, -1});

    // 边界：取值范围的极值
    check({-1000000000, -1000000000, 1000000000}, -1000000000, {0, 1});
    check({-1000000000, 0, 1000000000}, 1000000000, {2, 2});

    // 规模：10^5 个相同元素，验证两次二分不随重复次数退化
    {
        vector<int> nums(100000, 5);
        check(nums, 5, {0, 99999});
        check(nums, 4, {-1, -1});
        check(nums, 6, {-1, -1});
    }

    cout << "all tests passed" << endl;
    return 0;
}
