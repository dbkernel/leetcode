/*
============================================================================
658. 找到 K 个最接近的元素（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-k-closest-elements/
分类：二分查找 · 定长窗口左端点

给定一个 排序好 的数组 arr ，两个整数 k 和 x ，从数组中找到最靠近 x
（两数之差最小）的 k 个数。返回的结果必须要是按升序排好的。

整数 a 比整数 b 更接近 x 需要满足：
- |a - x| < |b - x| 或者
- |a - x| == |b - x| 且 a < b

示例 1：
输入：arr = [1,2,3,4,5], k = 4, x = 3
输出：[1,2,3,4]

示例 2：
输入：arr = [1,1,2,3,4,5], k = 4, x = -1
输出：[1,1,2,3]

提示：
- 1 <= k <= arr.length
- 1 <= arr.length <= 10^4
- arr 按 升序 排列
- -10^4 <= arr[i], x <= 10^4
*/
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：双指针从两端删除远端元素【非最优】

为什么慢：答案长度为 k，但本解法要把 n-k 个元素逐个删除，每删一个只
排除一个候选，总代价 O(n)。当 n = 10^4、k 很小时，这个线性开销明显
大于「直接定位窗口左端」所需的 O(log(n-k))。

换个角度：排序数组里，最接近 x 的 k 个元素一定是一段长度恰为 k 的连续
子数组。于是可以从 [0, n-1] 出发，每次比较两端谁离 x 更远，删掉更远的
那一端，直到剩下 k 个。被删掉的一端不可能再进入答案。

关键点：
  · 距离相等时删右端、保留左端 —— 题目要求相等时取较小值
  · 循环条件是「剩余长度 > k」，不是 >= k，否则会多删一个
  · 比较用 |arr[l] - x| 与 |arr[r] - x|，不要比较 arr[l] 与 arr[r] 本身
  · 元素值范围 ±10^4，abs 不会溢出 int；若改用更大的值要换 long long

手推一遍：
  以示例 1（arr = [1,2,3,4,5], k = 4, x = 3）为例：
  剩余 5 个：|1-3| = 2，|5-3| = 2，相等 → 删右端 5
  剩下 [1,2,3,4]，长度已是 4 → 答案 [1,2,3,4] ✔

复杂度分析：
  时间 O(n)：最多删 n-k 次，每次 O(1)
  空间 O(1)：除了返回数组不再开额外结构
*/
class Solution1
{
public:
    vector<int> findClosestElements(vector<int> &arr, int k, int x)
    {
        int l = 0;
        int r = static_cast<int>(arr.size()) - 1;

        while (r - l + 1 > k)
        {
            // 相等时删右端，保留较小的左端
            if (abs(arr[l] - x) <= abs(arr[r] - x))
            {
                --r;
            }
            else
            {
                ++l;
            }
        }
        return vector<int>(arr.begin() + l, arr.begin() + r + 1);
    }
};

/*
解法二：二分窗口左端点【最优解】

为什么慢：解法一把窗口从两端收缩，逐个排除 n-k 个元素；但窗口长度固定
为 k 后，左端点只有 n-k+1 种可能，完全可以用二分挑出正确的那一个。

换个角度：设窗口为 arr[s .. s+k-1]。把窗口右移一格，丢出去的是 arr[s]，
收进来的是 arr[s+k]。如果 arr[s] 离 x 更远，那么以 s 为左端点的窗口
不如右移一格，s 可以整体右移；反之 s 应留在左半区。这个比较具有单调性。

关键点：
  · 二分区间是 [0, n-k]，两个端点都可能是答案，用 lo < hi 收敛到唯一位置
  · 比较 x - arr[mid] 与 arr[mid+k] - x；只有左端严格更远才 lo = mid + 1，
    相等时走 hi = mid，保留较小值
  · arr 已排序，两个差值可能同时为负（窗口整体在 x 一侧），同号时比较
    依然给出正确方向，不需要先取绝对值
  · k == n 时区间退化为 [0, 0]，循环一次都不进，直接返回整个数组

手推一遍：
  以示例 2（arr = [1,1,2,3,4,5], k = 4, x = -1）为例：
  n = 6，二分区间 [0, 2]
  mid = 1：左端 arr[1] = 1，右端 arr[5] = 5
    x - 1 = -2，5 - x = 6，-2 > 6 不成立 → hi = 1
  mid = 0：左端 arr[0] = 1，右端 arr[4] = 4
    x - 1 = -2，4 - x = 5，不成立 → hi = 0
  收敛到左端 0 → [1,1,2,3] ✔

复杂度分析：
  时间 O(log(n - k) + k)：二分 O(log(n-k))，复制答案 O(k)
  空间 O(k)：返回数组本身；不计返回值则 O(1)
*/
class Solution2
{
public:
    vector<int> findClosestElements(vector<int> &arr, int k, int x)
    {
        int n = static_cast<int>(arr.size());
        if (n == 0) return {};

        int lo = 0;
        int hi = n - k;
        while (lo < hi)
        {
            int mid = lo + (hi - lo) / 2;
            // 左端严格更远才右移；相等时保留较小值
            if (x - arr[mid] > arr[mid + k] - x)
            {
                ++lo;
            }
            else
            {
                hi = mid;
            }
        }
        return vector<int>(arr.begin() + lo, arr.begin() + lo + k);
    }
};

namespace
{
// 两套解法各用一份副本，顺带验证题目要求的「不修改输入数组」
void check(const vector<int> &arr, int k, int x, const vector<int> &want)
{
    vector<int> a = arr;
    assert(Solution1().findClosestElements(a, k, x) == want);
    assert(a == arr);

    vector<int> b = arr;
    assert(Solution2().findClosestElements(b, k, x) == want);
    assert(b == arr);
}
} // namespace

int main()
{
    // 示例 1：距离相等时保留较小值
    check({1, 2, 3, 4, 5}, 4, 3, {1, 2, 3, 4});

    // 示例 2：x 在数组左侧之外
    check({1, 1, 2, 3, 4, 5}, 4, -1, {1, 1, 2, 3});

    // 边界：k == n，无论 x 在哪都返回整个数组
    check({1, 2, 3}, 3, 10, {1, 2, 3});
    check({1, 2, 3}, 3, -10, {1, 2, 3});

    // 边界：k == 1，距离打平时取较小的元素
    check({1, 3}, 1, 2, {1});
    check({1, 2, 10}, 1, 9, {10});
    check({7}, 1, 0, {7});

    // 边界：x 小于全部元素 / 大于全部元素
    check({1, 2, 3}, 2, -5, {1, 2});
    check({1, 2, 3}, 2, 10, {2, 3});

    // 边界：全是重复值，窗口内元素完全相同
    check({1, 1, 1}, 2, 2, {1, 1});

    // 边界：负数，距离比较不能直接比大小
    check({-5, -3, -1, 0, 2}, 3, -2, {-3, -1, 0});

    // 边界：空数组与 k == 0（题目保证 k >= 1，这里只验证健壮性）
    check({}, 0, 5, {});
    check({1, 2}, 0, 5, {});

    // 边界：规模 10000，x 在左端之外，答案就是前 k 个
    {
        vector<int> big;
        for (int i = 1; i <= 10000; ++i) big.push_back(i);
        vector<int> want(big.begin(), big.begin() + 10);
        check(big, 10, -10000, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
