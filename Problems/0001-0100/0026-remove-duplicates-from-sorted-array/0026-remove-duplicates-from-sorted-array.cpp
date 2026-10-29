/*
============================================================================
LeetCode 26. 删除有序数组中的重复项（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
分类：双指针 · 快慢指针去重
============================================================================

给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个
元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。
然后返回 nums 中唯一元素的个数。

考虑 nums 的唯一元素的数量为 k。去重后，返回唯一元素的数量 k。

nums 的前 k 个元素应包含 排序后 的唯一数字。下标 k - 1 之后的剩余元素可以
忽略。

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
解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为
1, 2 。不需要考虑数组中超出新长度后面的元素。

示例 2：
输入：nums = [0,0,1,1,1,2,2,3,3,4]
输出：5, nums = [0,1,2,3,4,_,_,_,_,_]
解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为
0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。

提示：
1 <= nums.length <= 3 * 10^4
-100 <= nums[i] <= 100
nums 已按 非递减 顺序排列。
*/
#include <vector>
#include <cassert>
#include <iostream>
using namespace std;

/*
解法：双指针（快指针读、慢指针写）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每个元素读一次，
O(n) 已是下界，O(1) 额外空间也已是最优。

为什么慢：想「删掉中间某个元素」就得把后面所有元素整体前移，一次删除
O(n)，最坏要删 n 次，退化成 O(n^2)；若另开数组去重，又违反「原地」要求。

换个角度：数组已经有序，相等的元素必然连续成块。于是每块只需保留第一个，
后面同块的都跳过 —— 用快指针负责「发现新块的开头」，慢指针负责「写入位置」。
被覆盖的位置一定是慢指针自己之前走过的，不会丢掉还没读到的数据。

关键点：
  · 两个指针都从下标 1 起步，下标 0 天然是第一个块的头，直接保留
  · 比较对象是 nums[fast] 与 nums[fast - 1]，不是 nums[fast] 与 nums[slow] ——
    后者在 slow 落后多格时会比错对象
  · 快指针不管是否写入都要前进；慢指针只在写入后前进

以示例 2（nums = [0,0,1,1,1,2,2,3,3,4]）为例：
  slow = 1
  fast = 1：nums[1] == nums[0] → 跳过
  fast = 2：nums[2] = 1 != nums[1] = 0 → nums[1] = 1，slow = 2
  fast = 3、4：与前一元素相同 → 跳过
  fast = 5：nums[5] = 2 != nums[4] = 1 → nums[2] = 2，slow = 3
  依此类推，最终 slow = 5，前 5 个元素为 0、1、2、3、4

复杂度分析：
  时间 O(n)：快指针把数组扫一遍，慢指针只会跟在后面
  空间 O(1)：只用两个下标
*/
class Solution
{
public:
    int removeDuplicates(vector<int> &nums)
    {
        int n = static_cast<int>(nums.size());
        // 空数组只能返回 0；题目约束 n >= 1，这里保留是为了不依赖该约束
        if (n == 0)
        {
            return 0;
        }

        // slow：下一个不同元素要写入的位置，同时也就是已确认的唯一元素个数
        int slow = 1;
        for (int fast = 1; fast < n; ++fast)
        {
            // 与前一个元素不同，说明进入了新的一块，保留它
            if (nums[fast] != nums[fast - 1])
            {
                nums[slow] = nums[fast];
                ++slow;
            }
        }

        // 前 slow 个元素已是不重复的有序序列，长度就是 slow
        return slow;
    }
};

namespace
{
// 一个用例 = 输入 + 期望输出序列；判题标准只看前 k 个元素
void check(vector<int> nums, const vector<int> &want)
{
    Solution s;
    int k = s.removeDuplicates(nums);

    assert(k == static_cast<int>(want.size()));
    for (int i = 0; i < k; ++i)
    {
        assert(nums[i] == want[i]);
    }
}
} // namespace

int main()
{
    // 示例 1：最短的含重复输入
    check({1, 1, 2}, {1, 2});

    // 示例 2：多块重复，验证慢指针不会漏写
    check({0, 0, 1, 1, 1, 2, 2, 3, 3, 4}, {0, 1, 2, 3, 4});

    // 边界：只有一个元素，slow 初始值就是答案
    check({1}, {1});

    // 边界：全都不重复，快慢指针始终同步，数组不应被改动
    check({1, 2, 3}, {1, 2, 3});

    // 边界：全部相同，只剩一个元素
    check({2, 2, 2, 2}, {2});

    cout << "all tests passed" << endl;
    return 0;
}
