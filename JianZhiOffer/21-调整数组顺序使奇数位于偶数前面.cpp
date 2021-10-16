/*
剑指 Offer 21. 调整数组顺序使奇数位于偶数前面

输入一个整数数组，实现一个函数来调整该数组中数字的顺序，使得所有奇数位于数组的前半部分，所有偶数位于数组的后半部分。

示例：
输入：nums = [1,2,3,4]
输出：[1,3,2,4]
注：[3,1,2,4] 也是正确的答案之一。

提示：
0 <= nums.length <= 50000
0 <= nums[i] <= 10000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof

思路：
首尾双指针法
*/
#include <iostream>
#include <cassert>
#include <vector>
using namespace std;

class Solution
{
public:
    vector<int> exchange(vector<int> &nums)
    {
        int left = 0, right = nums.size() - 1;
        while (left < right)
        {
            while (left < right && (nums[left] & 1) == 1) // 从左找到第一个偶数
                left++;
            while (left < right && (nums[right] & 1) == 0) // 从右找到第一个奇数
                right--;
            swap(nums[left++], nums[right--]);
        }
        return nums;
    }
};

int main()
{
    Solution s;
    vector<int> nums = {1, 2, 3, 4};
    vector<int> expect = {1, 3, 2, 4};
    assert(s.exchange(nums) == expect);
    return 0;
}
