/*
15. 三数之和

给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a
+ b + c = 0 ？请你找出所有和为 0 且不重复的三元组。
注意：答案中不可以包含重复的三元组。

示例 1：
输入：nums = [-1,0,1,2,-1,-4]
输出：[[-1,-1,2],[-1,0,1]]

示例 2：
输入：nums = []
输出：[]

示例 3：
输入：nums = [0]
输出：[]

提示：
0 <= nums.length <= 3000
-105 <= nums[i] <= 105

本题与 [ 1. 两数之和 ] 类似，是非常经典的面试题，但是做法不尽相同。

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/3sum

*/
#include <cassert>
#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

/*
方法一：排序 + 双指针

题目中要求找到所有「不重复」且和为 00
的三元组，这个「不重复」的要求使得我们无法简单地使用三重循环枚举所有的三元组。
这是因为在最坏的情况下，数组中的元素全部为 00，即
[0, 0, 0, 0, 0, ..., 0, 0, 0]
任意一个三元组的和都为 0。如果我们直接使用三重循环枚举三元组，会得到 O(N^3)
个满足题目要求的三元组（其中 N 是数组的长度） 时间复杂度至少为 O(N^3)。
在这之后，我们还需要使用哈希表进行去重操作，得到不包含重复三元组的最终答案，又消耗了大量的空间。
这个做法的时间复杂度和空间复杂度都很高，因此我们要换一种思路来考虑这个问题。

可以保持第二重循环不变，而将第三重循环变成一个从数组最右端开始向左移动的指针，从而得到下面的伪代码：

nums.sort()
for first = 0 .. n-1
    if first == 0 or nums[first] != nums[first-1] then
        // 第三重循环对应的指针
        third = n-1
        for second = first+1 .. n-1
            if second == first+1 or nums[second] != nums[second-1] then
                // 向左移动指针，直到 a+b+c 不大于 0
                while nums[first]+nums[second]+nums[third] > 0
                    third = third-1
                // 判断是否有 a+b+c==0
                check(first, second, third)

思路：
1. 先排序，便于去重
2. 用两层循环来处理，第二层循环是双指针

复杂度：
- 时间复杂度：O(N^2)，其中 N 是数组 }nums 的长度。
- 空间复杂度：O(logN)。我们忽略存储答案的空间，额外的排序的空间复杂度为
O(logN)。然而我们修改了输入的数组 nums，在实际情况下不一定允许，
因此也可以看成使用了一个额外的数组存储了 nums 的副本并进行排序，空间复杂度为
O(N)。
*/
class Solution {
   public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        int len = nums.size();
        sort(nums.begin(), nums.end());  // 先排序，便于后续查找、去重
        vector<vector<int>> res;
        // 枚举 a
        for (int first = 0; first < len; ++first) {
            // 需要和上一次枚举的数不相同
            if (first > 0 && nums[first] == nums[first - 1])
                continue;

            // 以下步骤与 【 1. 两数之和 】 双指针方法步骤一样
            // c 对应的指针初始指向数组的最右端
            int third = len - 1;
            int target = -nums[first];
            // 枚举 b
            for (int second = first + 1; second < len; ++second) {
                // 需要和上一次枚举的数不相同
                if (second > first + 1 && nums[second] == nums[second - 1])
                    continue;

                // 需要保证 b 的指针在 c 的指针的左侧
                while (second < third && nums[second] + nums[third] > target)
                    --third;

                // 如果指针重合，随着 b 后续的增加
                // 就不会有满足 a+b+c=0 并且 b<c 的 c 了，可以退出循环
                if (second == third)
                    break;

                if (nums[second] + nums[third] == target)
                    res.push_back({nums[first], nums[second], nums[third]});
            }
        }
        return res;
    }
};
