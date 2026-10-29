/*
============================================================================
56. 合并区间（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/merge-intervals/
分类：排序 · 区间合并（贪心）

以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [start_i, end_i] 。
请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。

示例 1：
输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
输出：[[1,6],[8,10],[15,18]]
解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].

示例 2：
输入：intervals = [[1,4],[4,5]]
输出：[[1,5]]
解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。

示例 3：
输入：intervals = [[4,7],[1,4]]
输出：[[1,7]]
解释：区间 [1,4] 和 [4,7] 可被视为重叠区间。

提示：
1 <= intervals.length <= 10^4
intervals[i].length == 2
0 <= start_i <= end_i <= 10^4
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：按左端点排序后贪心合并【最优解】

官方题解只给出这一种方法，也就是本题的最优解：时间 O(n log n)、空间 O(log n)。

为什么慢：如果不排序，一个区间可能和「前面很远」的区间重叠，你必须两两比较，
退化成 O(n^2)；而且合并完的大区间还可能再和别的区间重叠，得反复扫描。

换个角度：只按左端点升序排一次，就能得到一个关键性质——
「所有能合并的区间，在排好序的列表里一定是连续的」。
于是问题变成一次线性扫描：维护一个已合并结果的末尾区间，逐个吞并后续区间即可。
官方的正确性证明用反证法：若排序后 a[i]、a[k] 可合并而中间的 a[j] 没被合并进去，
就会和 a[j] 与 a[k] 的端点关系矛盾。

关键点：
  · 排序键必须用左端点（不能用右端点）；C++ 里 vector 的默认字典序比较即为先比左端点，
    直接 sort 即可
  · 判定「不重叠」的条件是 last[1] < L（当前区间左端点严格大于末尾右端点）。
    注意 [1,4] 与 [4,5] 端点相接也算重叠，所以用严格小于，不能用 <=
  · 合并时右端点取 max(last[1], R)——当前区间可能完全被已合并区间包住
    （例如 [[1,10],[2,3]]），无脑赋值 last[1] = R 会把区间改小
  · 结果天然按左端点有序，且互不重叠

以 [[1,3],[2,6],[8,10],[15,18]] 为例：
  排序后不变
  merged = [[1,3]]
  [2,6]：2 <= 3 → 合并 → merged = [[1,6]]
  [8,10]：8 > 6 → 不重叠，追加 → [[1,6],[8,10]]
  [15,18]：15 > 10 → 追加 → [[1,6],[8,10],[15,18]]
  返回该结果

复杂度分析：
  时间 O(n log n)：排序 O(n log n) + 一次线性扫描 O(n)
  空间 O(log n)：排序所需的递归栈（不计答案数组）
*/
class Solution
{
public:
    vector<vector<int>> merge(vector<vector<int>> &intervals)
    {
        vector<vector<int>> merged;
        if (intervals.empty()) return merged;

        // 按左端点升序；vector<int> 的默认比较就是先比 a[0] 再比 a[1]
        sort(intervals.begin(), intervals.end());

        merged.push_back(intervals[0]);
        for (size_t i = 1; i < intervals.size(); ++i)
        {
            int L = intervals[i][0], R = intervals[i][1];
            // 与末尾区间不重叠（端点相接仍算重叠，故用严格小于）
            if (merged.back()[1] < L)
            {
                merged.push_back({L, R});
            }
            else
            {
                // 当前区间可能被末尾区间完全包住，所以取两者较大值
                merged.back()[1] = max(merged.back()[1], R);
            }
        }
        return merged;
    }
};

namespace
{
void check(vector<vector<int>> intervals, const vector<vector<int>> &want)
{
    Solution solution;
    auto got = solution.merge(intervals);
    assert(got == want);
}
} // namespace

int main()
{
    // 示例 1
    check({{1, 3}, {2, 6}, {8, 10}, {15, 18}}, {{1, 6}, {8, 10}, {15, 18}});

    // 示例 2：端点相接也要合并
    check({{1, 4}, {4, 5}}, {{1, 5}});

    // 示例 3：输入无序
    check({{4, 7}, {1, 4}}, {{1, 7}});

    // 边界：只有一个区间
    check({{1, 1}}, {{1, 1}});

    // 边界：完全互不相交
    check({{1, 2}, {5, 6}, {9, 10}}, {{1, 2}, {5, 6}, {9, 10}});

    // 边界：被完全包含的区间（右端点不能取小值）
    check({{1, 10}, {2, 3}}, {{1, 10}});
    check({{1, 10}, {2, 3}, {4, 5}}, {{1, 10}});

    // 边界：链式重叠，全部合成一个
    check({{1, 4}, {2, 3}, {3, 6}, {5, 7}}, {{1, 7}});

    // 边界：相同区间重复出现
    check({{1, 2}, {1, 2}, {1, 2}}, {{1, 2}});

    // 边界：最大取值范围
    check({{0, 0}, {0, 10000}}, {{0, 10000}});

    cout << "all tests passed" << endl;
    return 0;
}
