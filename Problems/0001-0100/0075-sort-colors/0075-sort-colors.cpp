/*
75. 颜色分类

给定一个包含红色、白色和蓝色，一共 n 个元素的数组，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
此题中，我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。

示例 1：
输入：nums = [2,0,2,1,1,0]
输出：[0,0,1,1,2,2]

示例 2：
输入：nums = [2,0,1]
输出：[0,1,2]

示例 3：
输入：nums = [0]
输出：[0]

示例 4：
输入：nums = [1]
输出：[1]

提示：
n == nums.length
1 <= n <= 300
nums[i] 为 0、1 或 2
 
进阶：
- 你可以不使用代码库中的排序函数来解决这道题吗？
- 你能想出一个仅使用常数空间的一趟扫描算法吗？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sort-colors

思路：
继续采用首尾双指针法。
在遍历的过程中，找出所有的 0 交换至数组的头部，找出所有的 2 交换至数组的尾部。
由于此时其中一个指针 right 是从右向左移动的，因此当我们在从左向右遍历整个数组时，如果遍历到的位置超过了 right，那么就可以直接停止遍历了。
具体地，我们从左向右遍历整个数组，设当前遍历到的位置为 i，对应的元素为 nums[i]；
- 如果找到了 0，那么与前面两种方法类似，将其与 nums[left] 进行交换，并将 left 向后移动一个位置；
- 如果找到了 2，那么将其与 nums[right] 进行交换，并将 right 向前移动一个位置。
这样做是正确的吗？可以发现，对于第二种情况，当我们将 nums[i] 与 nums[right] 进行交换之后，新的 nums[i] 可能仍然是 2，也可能是 0。
然而此时我们已经结束了交换，开始遍历下一个元素 nums[i+1]，不会再考虑 nums[i] 了，这样我们就会得到错误的答案。
因此，当我们找到 2 时，我们需要不断地将其与 nums[right] 进行交换，直到新的 nums[i] 不为 2。
此时，如果 nums[i] 为 0，那么对应着第一种情况；如果 nums[i] 为 1，那么就不需要进行任何后续的操作。

复杂度分析：
- 时间复杂度：O(n)，其中 n 是数组 nums 的长度。
- 空间复杂度：O(1)。
*/
#include <iostream>
#include <cassert>
#include <vector>
using namespace std;

class Solution
{
public:
    void sortColors(vector<int> &nums)
    {
        // 首尾双指针 写法一 From 官方
        // 遇到2与right交换，遇到0与left交换
        int left = 0, right = nums.size() - 1;
        for (int i = 0; i <= right; ++i)
        {
            /*
             当我们将 nums[i] 与 nums[right] 进行交换之后，新的 nums[i] 可能仍然是 2，也可能是 0。
             然而此时我们已经结束了交换，开始遍历下一个元素 nums[i+1]，不会再考虑 nums[i] 了，这样我们就会得到错误的答案。
             因此，当我们找到 2 时，我们需要不断地将其与 nums[right] 进行交换，直到新的 nums[i] 不为 2。
            */
            while (i <= right && nums[i] == 2)
                swap(nums[i], nums[right--]);
            if (nums[i] == 0)
                swap(nums[i], nums[left++]);
        }

        // 首尾双指针法 写法二 From 民间
        // 遇到2与right交换，遇到0与left交换，遇到1则回退一格
        // int left = 0, right = nums.size() - 1;
        // for (int i = 0; i <= right; i++)
        // {
        //     if (nums[i] == 0)
        //         swap(nums[left++], nums[i]);
        //     if (nums[i] == 2)
        //     {
        //         swap(nums[right--], nums[i]);
        //         if (nums[i] != 1) // 个人理解还是为了处理2，与写法一种的while类似
        //             --i;
        //     }
        // }
    }
};

int main()
{
    Solution s;
    {
        vector<int> nums = {2, 2, 0, 1, 1, 0, 1, 1, 0, 2};
        vector<int> expect = {0, 0, 0, 1, 1, 1, 1, 2, 2, 2};
        s.sortColors(nums);
        assert(nums == expect);
    }
    {
        vector<int> nums = {0};
        vector<int> expect = {0};
        s.sortColors(nums);
        assert(nums == expect);
    }
    {
        vector<int> nums = {1};
        vector<int> expect = {1};
        s.sortColors(nums);
        assert(nums == expect);
    }
    {
        vector<int> nums = {2};
        vector<int> expect = {2};
        s.sortColors(nums);
        assert(nums == expect);
    }
    return 0;
}
