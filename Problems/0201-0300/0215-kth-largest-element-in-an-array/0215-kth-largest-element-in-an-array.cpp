/*
215. 数组中的第K个最大元素

给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。
请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

示例 1:
输入: [3,2,1,5,6,4] 和 k = 2
输出: 5

示例 2:
输入: [3,2,3,1,2,4,5,5,6] 和 k = 4
输出: 4
 
提示：
1 <= k <= nums.length <= 104
-104 <= nums[i] <= 104

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/kth-largest-element-in-an-array

思路：

如果采用先对原数组排序，再返回倒数第 k 个位置，这样平均时间复杂度是 O(nlogn)，但其实我们可以做的更快，也就是用快速排序来解决此问题。
快速排序是一个典型的分治算法，对数组 a[l⋯r] 做快速排序的过程是：
- 分解： 将数组 a[l⋯r] 「划分」成两个子数组 a[l⋯q−1]、a[q+1⋯r]，使得 a[l⋯q−1] 中的每个元素小于等于 a[q]，且 a[q] 小于等于 a[q+1⋯r] 中的每个元素。
        其中，计算下标 q 也是「划分」过程的一部分。
- 解决： 通过递归调用快速排序，对子数组 a[l⋯q−1] 和 a[q+1⋯r] 进行排序。
- 合并： 因为子数组都是原址排序的，所以不需要进行合并操作，a[l⋯r] 已经有序。
由此可以发现每次经过「划分」操作后，我们一定可以确定一个元素的最终位置，以只要某次划分的 q 为倒数第 k 个下标的时候，我们就已经找到了答案。

复杂度分析：
- 时间复杂度：O(n)，如上文所述，证明过程可以参考「《算法导论》9.2：期望为线性的选择算法」。
- 空间复杂度：O(logn)，递归使用栈空间的空间代价的期望为 O(logn)。
*/
#include <iostream>
#include <cassert>
#include <vector>
using namespace std;

class Solution
{
public:
    int findKthLargest(vector<int> &nums, int k)
    {
        srand(time(0));
        return quickSelect(nums, 0, nums.size() - 1, nums.size() - k);
    }

private:
    int quickSelect(vector<int> arr, int left, int right, int index)
    {
        int pos = partition(arr, left, right);
        if (pos == index)
            return arr[pos];
        return pos < index ? quickSelect(arr, pos + 1, right, index) : quickSelect(arr, left, pos - 1, index);
    }

    int partition(vector<int> &arr, int left, int right)
    {
        int middle = rand() % (right - left + 1) + left;
        swap(arr[middle], arr[right]); // 交换 arr[right] 与 arr[middle] 的值，后续以 arr[right] 为哨兵
        int pivot = arr[right];
        int i = left - 1;
        for (int j = left; j < right; j++)
        {
            // i一直代表小于 pivot 元素的最后一个索引，当发现有比 pivot 小的 arr[j] 时候，i+1 后交换
            if (arr[j] <= pivot)
                swap(arr[++i], arr[j]);
        }
        swap(arr[i + 1], arr[right]); // 将哨兵，即 arr[right] 切到中间来，左边是小于哨兵的，右边是大于哨兵的
        return i + 1;
    }
};

int main()
{
    vector<int> nums = {10, 6, 8, 1, 4, 7, 9};
    Solution s;
    assert(s.findKthLargest(nums, 2) == 9);
    assert(s.findKthLargest(nums, 4) == 7);
    return 0;
}