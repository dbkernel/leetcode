/*
============================================================================
1502. 判断能否形成等差数列（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/can-make-arithmetic-progression-from-sequence/
分类：排序 · 模拟

给你一个数字数组 arr 。

如果一个数列中，任意相邻两项的差总等于同一个常数，那么这个数列就称为 等差数列 。

如果可以重新排列数组形成等差数列，请返回 true ；否则，返回 false 。

示例 1：
输入：arr = [3,5,1]
输出：true
解释：对数组重新排序得到 [1,3,5] 或者 [5,3,1] ，任意相邻两项的差分别为 2 或 -2 ，
可以形成等差数列。

示例 2：
输入：arr = [1,2,4]
输出：false
解释：无法通过重新排序得到等差数列。

提示：
2 <= arr.length <= 1000
-10^6 <= arr[i] <= 10^6
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：排序后模拟【最优解】

官方题解只给出这一种方法，也就是本题的最优解：时间 O(n log n)、空间 O(log n)。

为什么慢：如果直接按「原顺序」去比较相邻两项的差，是不对的——题目允许重新排列，
原顺序相邻的差相等并不代表整个数组能排成等差数列。必须先把顺序「规整」一下。

换个角度：能排成等差数列的数组，排序之后必然是等差数列。所以排序就是本题的
「归约」步骤：一旦有序，等差数列的性质就变成一条可以逐项检验的局部条件。

关键点：
  · 有序后，等差数列 ⇔ 对每个 i ∈ [1, n-2] 都有 a[i] - a[i-1] == a[i+1] - a[i]；
    也等价于官方写的 a[i] * 2 == a[i-1] + a[i+1]
  · 只检查相邻的三个数就够，因为公差只需一致一次就对全局一致（相邻区间共享端点）
  · 用「减去前一项」而不是「乘 2」，可以避免 √ 题面数值下的溢出顾虑；
    |a[i]| ≤ 1e6 时两者都不溢出，但写法更直观
  · 循环边界是 n - 1（i 最大取 n-2），因为要访问 a[i+1]

以 arr = [3,5,1] 为例：
  排序 → [1,3,5]
  i=1：3-1=2，5-3=2 → 相等
  返回 true

以 arr = [1,2,4] 为例：
  排序 → [1,2,4]
  i=1：2-1=1，4-2=2 → 不等 → 返回 false

复杂度分析：
  时间 O(n log n)：排序 O(n log n) + 线性扫描 O(n)
  空间 O(log n)：排序所需的递归栈（原地排序，无额外数组）
*/
class Solution
{
public:
    bool canMakeArithmeticProgression(vector<int> &arr)
    {
        sort(arr.begin(), arr.end());
        for (int i = 1; i + 1 < static_cast<int>(arr.size()); ++i)
        {
            if (arr[i] - arr[i - 1] != arr[i + 1] - arr[i]) return false;
        }
        return true;
    }
};

namespace
{
void check(vector<int> arr, bool want)
{
    Solution solution;
    assert(solution.canMakeArithmeticProgression(arr) == want);
}
} // namespace

int main()
{
    // 示例 1
    check({3, 5, 1}, true);

    // 示例 2
    check({1, 2, 4}, false);

    // 边界：只有两个元素，一定能形成等差数列
    check({1, 2}, true);
    check({-1000000, 1000000}, true);

    // 边界：已有序 / 逆序
    check({1, 2, 3, 4}, true);
    check({4, 3, 2, 1}, true);

    // 边界：公差为 0（全部相等）
    check({7, 7, 7, 7}, true);

    // 边界：中间某项破坏公差
    check({1, 3, 7}, false);
    check({0, 2, 4, 7}, false);

    // 较大规模：1..1000 的公差为 1 的数列打乱顺序
    vector<int> big(1000);
    for (int i = 0; i < 1000; ++i) big[i] = (i * 617) % 1000;
    check(big, true);

    // 较大规模但破坏一项
    big[500] = 1000000;
    check(big, false);

    cout << "all tests passed" << endl;
    return 0;
}
