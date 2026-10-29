/*
============================================================================
面试题 17.04. 消失的数字（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/missing-number-lcci/
分类：位运算 · 异或抵消 / 求和相减

数组 nums 包含从 0 到 n 的所有整数，但其中缺了一个。请编写代码找出那个
缺失的整数。你有办法在 O(n) 时间内完成吗？

注意：本题相对书上原题稍作改动

示例 1：
输入：[3,0,1]
输出：2

示例 2：
输入：[9,6,4,2,3,5,7,0,1]
输出：8
*/
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：求和后相减【最优解】

为什么慢：不借和、只做「查在不在」的话，对 0..n 每个数都要在数组里找一次，
O(n^2)；先排序再逐个对位也要 O(n log n)。题目要求 O(n)，必须把「查存在」
换成常数代价的一次累加。

换个角度：缺失值不是某个元素的属性，而是两个总量的差 —— 0..n 的和有闭式
n(n+1)/2，数组元素之和与它只差缺失的那个数，一次遍历就能拿到。

关键点：
  · n = nums.size()，不是 size()+1：缺一个数之后长度恰好等于 n
  · 0..n 一共 n+1 个数，用 n*(n+1)/2，别写成 n*(n-1)/2
  · C++ 里和要放在 64 位里累计：n = 10^5 时约为 5*10^9，早已越过 int 上限，
    用 int 累计是有符号溢出（未定义行为）
  · 缺的数可能正好是 n（数组恰为 0..n-1），闭式求和同样成立

以示例 1（nums = [3,0,1]）为例：
  n = 3，0..3 的和是 6
  数组和 3+0+1 = 4
  6 - 4 = 2 ✔

复杂度分析：
  时间 O(n)：一次求和
  空间 O(1)：一个累加器，和后的大小与 n 同阶，与元素个数无关

解法二：异或抵消【最优解，与解法一并列】

为什么慢：求和的线性时间已经到顶，短板在值域 —— 和增长到 O(n^2) 量级，
定长整数放不下，只能靠更宽的整数兜住。

换个角度：异或满足 x^x = 0、x^0 = x，且与顺序无关。把下标 0..n 和数组元素
全部异或到一起，正常元素与它的下标成对抵消，只剩缺失的那个值。

关键点：
  · 循环里同时异或下标 i 和 nums[i]，最后再补上 n
  · 缺的数若是 n，它没有对应的 nums[i]，只能靠最后这一步补进来
  · 结果与数值范围无关，n 再大也不会溢出
  · 前提是「除缺失值外每个值恰好出现一次」，有重复值抵消关系就被破坏

以示例 1（nums = [3,0,1]）为例：
  (0^3) ^ (1^0) ^ (2^1) ^ 3 = 2 ✔

复杂度分析：
  时间 O(n)：一次遍历，每个元素两次异或
  空间 O(1)：一个异或累加器

取舍：两者时间同为 O(n)、空间同为 O(1)，都算最优。求和版每个元素只做一次
加法，常数更小，但对位宽敏感（C++ 要用 64 位承接，否则溢出）；异或版完全
不受数值范围影响，代价是每个元素多一次异或。
*/
class Solution1
{
public:
    // 返回 0..n 中缺失的那个整数
    int missingNumber(vector<int> &nums)
    {
        // 长度即 n：缺一个数之后 0..n 只剩 n 个元素
        long long n = static_cast<long long>(nums.size());
        long long total = n * (n + 1) / 2;

        // 和用 64 位承接，避免 n 较大时 32 位有符号溢出
        long long sum = 0;
        for (int x : nums) sum += x;

        return static_cast<int>(total - sum);
    }
};

class Solution2
{
public:
    int missingNumber(vector<int> &nums)
    {
        int res = 0;
        for (size_t i = 0; i < nums.size(); ++i)
        {
            // 下标与元素成对出现，同一个值异或两次归零
            res ^= static_cast<int>(i);
            res ^= nums[i];
        }

        // 缺的数若正是 n，它没有对应的 nums[i]，只能在这里补上
        res ^= static_cast<int>(nums.size());
        return res;
    }
};

namespace
{
// 两个解法共用同一组用例，逐个断言
void check(vector<int> nums, int want)
{
    Solution1 s1;
    Solution2 s2;
    assert(s1.missingNumber(nums) == want);
    assert(s2.missingNumber(nums) == want);
}
} // namespace

int main()
{
    // 示例 1
    check({3, 0, 1}, 2);

    // 示例 2
    check({9, 6, 4, 2, 3, 5, 7, 0, 1}, 8);

    // 乱序用例：0..9 中缺 7
    check({9, 6, 4, 0, 1, 2, 8, 3, 5}, 7);

    // 边界：缺的正好是 n
    check({0, 1}, 2);

    // 边界：缺的是 0
    check({1, 2, 3}, 0);

    // 边界：单元素，下标 0..1
    check({0}, 1);
    check({1}, 0);

    // 规模：n = 10^5，和越过 32 位，专门验证 64 位承接
    {
        vector<int> nums;
        nums.reserve(100000);
        for (int i = 0; i <= 100000; ++i)
        {
            if (i != 12345) nums.push_back(i);
        }
        check(nums, 12345);
    }

    cout << "all tests passed" << endl;
    return 0;
}
