/*
26. 删除有序数组中的重复项

给你一个 升序排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，
返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。

由于在某些语言中不能改变数组的长度，所以必须将结果放在数组nums的第一部分。
更规范地说，如果在删除重复项之后有 k 个元素，那么 nums 的前 k 个元素应该保存最终结果。

将最终结果插入 nums 的前 k 个位置后返回 k 。

不要使用额外的空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。

判题标准:

系统会用下面的代码来测试你的题解:

int[] nums = [...]; // 输入数组
int[] expectedNums = [...]; // 长度正确的期望答案

int k = removeDuplicates(nums); // 调用

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
如果所有断言都通过，那么您的题解将被 通过。

示例 1：
输入：nums = [1,1,2]
输出：2, nums = [1,2,_]
解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。

示例 2：
输入：nums = [0,0,1,1,1,2,2,3,3,4]
输出：5, nums = [0,1,2,3,4]
解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。

提示：
1 <= nums.length <= 3 * 10^4
-10^4 <= nums[i] <= 10^4
nums 已按 升序 排列

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-array
*/

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

/*
方法一：双指针

这道题目的要求是：对给定的有序数组 nums 删除重复元素，在删除重复元素之后，每个元素只出现一次，并返回新的长度，
上述操作必须通过原地修改数组的方法，使用 O(1) 的空间复杂度完成。

由于给定的数组 nums 是有序的，因此对于任意 i<j，如果 nums[i]=nums[j]，则对任意 i ≤ k ≤ j，必有 nums[i]=nums[k]=nums[j]，
即相等的元素在数组中的下标一定是连续的。利用数组有序的特点，可以通过双指针的方法删除重复元素。

如果数组 nums 的长度为 0，则数组不包含任何元素，因此返回 0。

当数组 nums 的长度大于 0 时，数组中至少包含一个元素，在删除重复元素之后也至少剩下一个元素，因此 nums[0] 保持原状即可，从下标 1 开始删除重复元素。

复杂度分析
- 时间复杂度：O(n)，其中 n 是数组的长度。快指针和慢指针最多各移动 n 次。
- 空间复杂度：O(1)。只需要使用常数的额外空间。
*/
class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int len = nums.size();
        if (len == 0)
            return 0;

        // 举例：nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
        // 当 fast 第一次找到新的元素时，就拷贝到 slow 位置
        int slow = 1, fast = 1;
        while(fast < len) {
            if (nums[fast] == nums[fast - 1])
                ++fast;
            else
                nums[slow++] = nums[fast++];
        }
        return slow;
    }
};

int main() {
    Solution s;
    {
        vector<int> nums = {1, 1, 2}; // C++11
        int len = s.removeDuplicates(nums);
        vector<int> want = {1, 2}; // C++11
        for(int i = 0; i < len; ++i)
            assert(want[i] == nums[i]);
        assert(len == 2);
    }
    {
        vector<int> nums{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}; // C++11
        int len = s.removeDuplicates(nums);
        vector<int> want{0, 1, 2, 3, 4}; // C++11
        for(int i = 0; i < len; ++i)
            assert(want[i] == nums[i]);
        assert(len == 5);
    }
    return 0;
}