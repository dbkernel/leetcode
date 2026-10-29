/*
============================================================================
349. 两个数组的交集（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/intersection-of-two-arrays/
分类：哈希表 · 去重与集合判定

给定两个数组 nums1 和 nums2 ，返回 它们的 交集 。输出结果中的每个元素一定是
唯一 的。我们可以 不考虑输出结果的顺序 。

示例 1：
输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]

示例 2：
输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
解释：[4,9] 也是可通过的

提示：
- 1 <= nums1.length, nums2.length <= 1000
- 0 <= nums1[i], nums2[i] <= 1000
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

/*
解法一：两个集合【最优解】

为什么慢：最直白的做法是拿 nums1 的每个元素去 nums2 里线性查找，单次判定 O(n)，
总共 O(m·n)。瓶颈很明确：同一个值被反复比较，而题目关心的只是「它到底在不在」。

换个角度：交集只取决于「哪些值同时出现过」，与出现次数无关。把两个数组各自压成
集合，重复元素自动消失，答案的唯一性也就不用额外判断了。

关键点：
  · 两个数组都要先去重，否则 nums1 里的重复值会让答案出现多次
  · 遍历较小的集合：交集大小不超过它，查找次数因此最少
  · 结果顺序任意，不需要排序；题面明确「不考虑输出结果的顺序」

以示例 2（nums1 = [4,9,5]，nums2 = [9,4,9,8,4]）为例：
  集合为 {4,9,5} 与 {4,8,9}
  遍历较小的 {4,9,5}：4 命中、9 命中、5 未命中 → [4,9] ✔

复杂度分析：
  时间 O(m+n)：建两个集合各扫一遍，再遍历较小的集合做 O(1) 判定
  空间 O(m+n)：两个集合最多各存一个数组的全部不同值

解法二：排序 + 双指针【非最优】

为什么慢：时间 O(m log m + n log n)，比哈希解法的线性慢一个 log。换来的是额外
空间只有排序栈的 O(log m + log n)，比两个集合的 O(m+n) 小。按渐近时间判定，它是
次优的那一个。

换个角度：两边都排好序后，交集元素在各自数组里都是递增出现的，于是各拿一个指针
同步前进即可，不需要任何查找结构。

关键点：
  · 相等时两个指针同时右移，否则只移动指向较小值的那一个
  · 收进答案前与上一个收下的值比较，跳过重复 —— 排序后重复元素必然相邻
  · 比较分「小于 / 大于 / 等于」三支，只写相等分支会让指针卡住

以示例 2（nums1 = [4,9,5]，nums2 = [9,4,9,8,4]）为例：
  排序后 [4,5,9] 与 [4,4,8,9,9]
  i=0,j=0：4 == 4，收 4，双双右移
  5 > 4：j 右移两次到 8
  5 < 8：i 右移到 9
  9 == 9：与已收的 4 不同，收 9，双双右移
  i 越界，结束 → [4,9] ✔

复杂度分析：
  时间 O(m log m + n log n)：两个数组的排序是瓶颈，双指针部分只占 O(m+n)
  空间 O(log m + log n)：排序递归栈；原地排序，不用哈希表
*/
class Solution1
{
public:
    vector<int> intersection(vector<int> &nums1, vector<int> &nums2)
    {
        unordered_set<int> set1(nums1.begin(), nums1.end());
        unordered_set<int> set2(nums2.begin(), nums2.end());

        // 只遍历较小的集合：交集不可能比它更大
        if (set1.size() > set2.size()) swap(set1, set2);

        vector<int> ans;
        for (int x : set1)
        {
            // 两边都已去重，命中即说明该值只该出现一次
            if (set2.count(x)) ans.push_back(x);
        }
        return ans;
    }
};

class Solution2
{
public:
    vector<int> intersection(vector<int> &nums1, vector<int> &nums2)
    {
        // 原地排序：复制一份会把额外空间抬到 O(m+n)
        sort(nums1.begin(), nums1.end());
        sort(nums2.begin(), nums2.end());

        vector<int> ans;
        size_t i = 0, j = 0;
        while (i < nums1.size() && j < nums2.size())
        {
            if (nums1[i] < nums2[j])
            {
                ++i;
            }
            else if (nums1[i] > nums2[j])
            {
                ++j;
            }
            else
            {
                // 与上一个收下的值不同才收，跳过同一值的重复出现
                if (ans.empty() || ans.back() != nums1[i])
                {
                    ans.push_back(nums1[i]);
                }
                ++i;
                ++j;
            }
        }
        return ans;
    }
};

namespace
{
// 题面说明输出顺序任意，比较前统一排序
void check(const vector<int> &nums1, const vector<int> &nums2, vector<int> want)
{
    Solution1 s1;
    vector<int> a1 = nums1, a2 = nums2;
    vector<int> got1 = s1.intersection(a1, a2);
    sort(got1.begin(), got1.end());
    sort(want.begin(), want.end());
    assert(got1 == want);

    // 解法二会原地排序，必须重新拷一份输入
    Solution2 s2;
    vector<int> b1 = nums1, b2 = nums2;
    vector<int> got2 = s2.intersection(b1, b2);
    sort(got2.begin(), got2.end());
    assert(got2 == want);
}
} // namespace

int main()
{
    // 示例 1：nums1 里重复的 1、2 不应让答案重复
    check({1, 2, 2, 1}, {2, 2}, {2});

    // 示例 2：输出顺序任意，这里按升序核对
    check({4, 9, 5}, {9, 4, 9, 8, 4}, {4, 9});

    // 边界：没有任何公共元素
    check({1, 2}, {3, 4}, {});

    // 边界：两个数组完全相同且都有重复
    check({1, 1, 1}, {1, 1}, {1});

    // 边界：值域下限 0 也是合法元素
    check({0, 1}, {0}, {0});

    // 边界：单元素相等
    check({5}, {5}, {5});

    // 边界：长度悬殊，交集只剩一个值
    check({1, 2, 3, 4, 5}, {3}, {3});

    // 反直觉：值域上限 1000 与 0 同时出现
    check({1000, 0}, {0, 1000}, {0, 1000});

    cout << "all tests passed" << endl;
    return 0;
}
