/*
============================================================================
81. 搜索旋转排序数组 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/search-in-rotated-sorted-array-ii/
分类：二分查找 · 旋转数组 · 重复元素

已知存在一个按非降序排列的整数数组 nums ，数组中的值不必互不相同。

在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行
了 旋转 ，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0],
nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如，
[0,1,2,4,4,4,5,6,6,7] 在下标 5 处经旋转后可能变为
[4,5,6,6,7,0,1,2,4,4] 。

给你 旋转后 的数组 nums 和一个整数 target ，请你编写一个函数来判断给定的
目标值是否存在于数组中。如果 nums 中存在这个目标值 target ，则返回 true ，
否则返回 false 。

你必须尽可能减少整个操作步骤。

示例 1：
输入：nums = [2,5,6,0,0,1,2], target = 0
输出：true

示例 2：
输入：nums = [2,5,6,0,0,1,2], target = 3
输出：false

提示：
- 1 <= nums.length <= 5000
- -10^4 <= nums[i] <= 10^4
- 题目数据保证 nums 在预先未知的某个下标上进行了旋转
- -10^4 <= target <= 10^4

进阶：
这是 搜索旋转排序数组 的延伸题目，本题中的 nums 可能包含重复元素。
这会影响到程序的时间复杂度吗？会有怎样的影响，为什么？
*/
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法：二分 + 判不出有序性时收缩左端【最优解】

为什么慢：允许重复后，形如 [1,0,1,1,1] 的区间两端同值，nums[left] 与
nums[mid] 相等，从端点看不出哪一半升序。此时若硬套 33 的规则，就会把
真正含有答案的那一半扔掉，二分直接失效。若退回线性扫描，时间 O(n)，
又浪费了大部分区间本来仍然有序的性质。

换个角度：唯一"判不出"的情形是 nums[left] == nums[mid]。由于
nums[mid] != target（前面已判过），这个 nums[left] 一定不是答案，可以
安全地右移一格把它丢掉，剩下的区间继续二分。绝大多数轮次仍在减半，只有
大量重复元素时才逐步剥离。

关键点：
  · 区间取左闭右闭 [left, right]，循环条件 left <= right
  · 判断顺序不能换：先判 nums[mid] == target，再判 nums[left] == nums[mid]
  · nums[left] == nums[mid] 时只能 left++，不能 right = mid - 1
    （mid 右侧仍可能藏着答案，而 left 已有 nums[mid] != target 作保）
  · 剩下的情况 nums[left] != nums[mid]，必定有一半有序：
    nums[left] < nums[mid] 左半有序，否则旋转点落在左半、右半有序
  · 有序段的命中判断用闭开 / 开闭区间：左边是
    nums[left] <= target < nums[mid]，右边是 nums[mid] < target <= nums[right]

手推一遍：nums = [1,0,1,1,1]，target = 0
  left=0,right=4,mid=2，nums[2]=1 != 0；nums[0]==nums[2] → left=1
  left=1,right=4,mid=2，nums[2]=1 != 0；nums[1]=0 < nums[2]=1，左半有序，
  0 落在 [0,1) 内 → right=1
  left=1,right=1,mid=1，nums[1]=0 == target → true ✔

另一个反直觉用例：nums = [1,1,3,1,1,1,1]，target = 3
  前两轮 nums[left]==nums[mid]==1 各丢一格，left 走到 2 时
  nums[2]=3 > nums[mid]=1，右半有序、3 不在 [1,1] 内 → right = 3
  再一轮 mid=2 命中 3。若在 left=2 时仍按"相等就 left++"处理，就会
  跳过下标 2，得到错误答案

复杂度分析：
  时间 平均 O(log n)，最坏 O(n)：全相等且 target 不存在时，每轮只剥掉
    一个端点
  空间 O(1)：只用 left / right / mid 三个下标
*/
class Solution
{
public:
    bool search(vector<int> &nums, int target)
    {
        int left = 0;
        int right = (int)nums.size() - 1;

        while (left <= right)
        {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) return true;

            // 判不出哪半有序：nums[left] 已被上面的比较排除，丢掉它是安全的
            if (nums[left] == nums[mid])
            {
                ++left;
            }
            else if (nums[left] < nums[mid])
            {
                // 左半升序，target 落在区间内就只可能在左半
                if (nums[left] <= target && target < nums[mid])
                    right = mid - 1;
                else
                    left = mid + 1;
            }
            else
            {
                // 旋转点在左半，因此 [mid, right] 升序
                if (nums[mid] < target && target <= nums[right])
                    left = mid + 1;
                else
                    right = mid - 1;
            }
        }

        // 区间排空仍无命中
        return false;
    }
};

namespace
{
// 单解法也要覆盖全部用例，失败时能看出是哪一组数据
void check(vector<int> nums, int target, bool want)
{
    Solution s;
    assert(s.search(nums, target) == want);
}
} // namespace

int main()
{
    // 示例 1
    check({2, 5, 6, 0, 0, 1, 2}, 0, true);

    // 示例 2：目标不存在
    check({2, 5, 6, 0, 0, 1, 2}, 3, false);

    // 边界：单元素
    check({1}, 1, true);
    check({1}, 0, false);

    // 边界：完全升序（旋转点在首，相当于旋转了 n 次）
    check({1, 2, 3, 4, 5}, 3, true);
    check({1, 2, 3, 4, 5}, 0, false);

    // 边界：只旋转一次，最小值落在末尾（旋转点在尾）
    check({2, 3, 4, 5, 1}, 1, true);
    check({2, 3, 4, 5, 1}, 5, true);
    check({2, 3, 4, 5, 1}, 6, false);

    // 边界：全相等，命中与不命中各一次（不命中是最坏 O(n) 的形态）
    check({2, 2, 2, 2}, 2, true);
    check({2, 2, 2, 2}, 1, false);

    // 反直觉：两端同值，必须靠 left++ 才能露出答案
    check({1, 0, 1, 1, 1}, 0, true);
    check({1, 0, 1, 1, 1}, 1, true);

    // 反直觉：一旦按"相等就收缩"的顺序写错，会跳过下标 2 上的 3
    check({1, 1, 3, 1, 1, 1, 1}, 3, true);

    // 边界：两个元素
    check({3, 1}, 1, true);
    check({3, 1}, 3, true);
    check({3, 1}, 2, false);

    // 边界：三元素且两端同值
    check({3, 1, 3}, 3, true);
    check({3, 1, 3}, 2, false);

    // 边界：重复的最小值成段出现
    check({2, 2, 2, 0, 1}, 0, true);
    check({2, 2, 2, 0, 1}, 1, true);
    check({2, 2, 2, 0, 1}, 3, false);

    // 说明：整体降序（如 [5,4,3,2,1]）不是升序数组的旋转结果，
    // 不能作为合法输入；降序片段由"只旋转一次"的用例覆盖

    cout << "all tests passed" << endl;
    return 0;
}
