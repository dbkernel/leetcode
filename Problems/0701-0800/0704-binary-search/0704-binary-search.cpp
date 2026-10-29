/*
704. 二分查找

给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。

示例 1:
输入: nums = [-1,0,3,5,9,12], target = 9
输出: 4
解释: 9 出现在 nums 中并且下标为 4

示例 2:
输入: nums = [-1,0,3,5,9,12], target = 2
输出: -1
解释: 2 不存在 nums 中因此返回 -1
 
提示：
你可以假设 nums 中的所有元素是不重复的。
n 将在 [1, 10000]之间。
nums 的每个元素都将在 [-9999, 9999]之间。

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-search

*/
#include <iostream>
#include <cassert>
#include <vector>
using namespace std;

/*
复杂度分析:
- 时间复杂度：O(logn)，其中 n 是数组的长度。
- 空间复杂度：O(1)。
*/
class Solution
{
public:
    int search(vector<int> &nums, int target)
    {
        int left = 0, right = nums.size() - 1;
        while (left <= right) // 可以避免 left=0、right=-1 及 left=len、right=len-1 的情况
        {
            int middle = (right - left) / 2 + left; // middle 是相对的，因此要加上 left
            if (nums[middle] == target)
                return middle;
            else if (nums[middle] > target)
                right = middle - 1;
            else
                left = middle + 1;
        }
        return -1;
    }
};

int main()
{
    vector<int> nums = {-1, 0, 3, 5, 9, 12};
    Solution s;
    assert(s.search(nums, 9) == 4);
    return 0;
}
