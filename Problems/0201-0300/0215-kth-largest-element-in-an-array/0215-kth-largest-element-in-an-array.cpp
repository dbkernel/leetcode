/*
============================================================================
215. 数组中的第 K 个最大元素（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/kth-largest-element-in-an-array/
分类：分治 · 快速选择（quickselect） / 堆

给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。

示例 1:
输入: [3,2,1,5,6,4], k = 2
输出: 5

示例 2:
输入: [3,2,3,1,2,4,5,5,6], k = 4
输出: 4

提示：
1 <= k <= nums.length <= 10^5
-10^4 <= nums[i] <= 10^4
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <utility>
#include <vector>
using namespace std;

/*
解法一：快速选择（基于快速排序的划分）【最优解】

官方方法一，也是题目要求的 O(n) 解法。

为什么慢：先把整个数组排序再取倒数第 k 个，时间是 O(n log n)，且把「不必要的信息」
也算了出来——我们只要一个元素，不需要关心左右两边是否有序。

换个角度：快速排序的划分（partition）每做一次，就能把主元放到它的最终位置上，
并保证左边都不大于它、右边都不小于它。也就是说：一次划分确定一个元素的最终下标 q。
如果 q 正好是目标下标，就直接返回；如果目标在左边，就只递归左边；否则只递归右边。
每次问题规模期望减半，总代价 n + n/2 + n/4 + … = O(n)。

关键点：
  · 目标下标不是 k - 1，而是 n - k：第 k 大 = 升序数组里下标 n - k 的位置
  · 官方用的是「双指针 + do-while」的划分写法（类似 Hoare 划分），
    不在循环内做随机化，实测对各类数据都比较稳
  · 划分循环 `while (i < j)`：i 从左跳过小于主元的，j 从右跳过大于主元的，
    都停下就交换；最后 j 是左半段的右端点
  · 判断方向：k <= j 说明目标在左段及 j，递归 (l, j)；否则递归 (j + 1, r)
  · 递归出口是 l == r（区间只剩一个元素，它就是答案）
  · 注意数组元素可能为负数，比较直接用 < / > 即可，不要用取模之类的技巧

以 nums = [3,2,1,5,6,4]，k = 2 为例：
  目标下标 = 6 - 2 = 4（升序 [1,2,3,4,5,6] 的下标 4 是 5）
  第一次划分取主元 3：划分后左边 {2,1}、右边 {5,6,4}，j 落到 2 附近
  目标下标 4 > j，只需在右段继续划分
  右段继续划分最终把 5 放到下标 4，返回 5

复杂度分析：
  时间 O(n)（期望）：每次只递归一侧，规模期望减半
  空间 O(log n)：递归栈（期望深度）

解法二：建大根堆 + 依次删除堆顶【非最优】

官方方法二。先把数组建成大根堆（O(n)），然后做 k - 1 次「堆顶与末尾交换 + 堆大小减一
+ 下滤」，最后堆顶就是第 k 大。

关键点：
  · 建堆从 heapSize / 2 - 1 往前下滤（叶子节点天然是堆）
  · 删除 k - 1 次就够了：每删掉一个当前最大值，堆顶就变成下一个最大值
  · 循环边界 `i >= n - k + 1` 正好做 k - 1 次
  · 下滤时要先在左右孩子里挑较大的那个再交换

复杂度分析：
  时间 O(n + k log n)，最坏 O(n log n)
  空间 O(log n)：下滤递归栈

对比：堆解法稳定 O(n log n) 但不用划分，不受「最坏划分」影响；
快速选择期望 O(n) 更快，是本体的最优解。
*/
class Solution
{
public:
    // ---------- 解法一：快速选择 ----------
    int quickselect(vector<int> &nums, int l, int r, int k)
    {
        if (l == r) return nums[k];

        int pivot = nums[l];
        int i = l - 1, j = r + 1;
        while (i < j)
        {
            do ++i; while (nums[i] < pivot);
            do --j; while (nums[j] > pivot);
            if (i < j) swap(nums[i], nums[j]);
        }

        // 目标在左段（含 j）就只递归左边，否则只递归右边
        if (k <= j) return quickselect(nums, l, j, k);
        return quickselect(nums, j + 1, r, k);
    }

    // 解法一入口：快速选择（最优解）
    int findKthLargestQuickSelect(vector<int> &nums, int k)
    {
        int n = static_cast<int>(nums.size());
        return quickselect(nums, 0, n - 1, n - k);
    }

    // ---------- 解法二：大根堆 ----------
    void maxHeapify(vector<int> &a, int i, int heapSize)
    {
        int l = i * 2 + 1, r = i * 2 + 2, largest = i;
        if (l < heapSize && a[l] > a[largest]) largest = l;
        if (r < heapSize && a[r] > a[largest]) largest = r;
        if (largest != i)
        {
            swap(a[i], a[largest]);
            maxHeapify(a, largest, heapSize);
        }
    }

    void buildMaxHeap(vector<int> &a, int heapSize)
    {
        for (int i = heapSize / 2 - 1; i >= 0; --i) maxHeapify(a, i, heapSize);
    }

    // 解法二入口：堆
    int findKthLargestHeap(vector<int> &nums, int k)
    {
        int heapSize = static_cast<int>(nums.size());
        buildMaxHeap(nums, heapSize);
        // 删除 k - 1 次堆顶，剩下的堆顶就是第 k 大
        for (int i = static_cast<int>(nums.size()) - 1; i >= static_cast<int>(nums.size()) - k + 1; --i)
        {
            swap(nums[0], nums[i]);
            --heapSize;
            maxHeapify(nums, 0, heapSize);
        }
        return nums[0];
    }
};

namespace
{
void check(vector<int> nums, int k, int want)
{
    Solution solution;

    auto a = nums, b = nums;
    int gotQuick = solution.findKthLargestQuickSelect(a, k);
    int gotHeap = solution.findKthLargestHeap(b, k);
    assert(gotQuick == want);
    assert(gotHeap == want);
}
} // namespace

int main()
{
    // 示例 1
    check({3, 2, 1, 5, 6, 4}, 2, 5);

    // 示例 2：存在重复元素
    check({3, 2, 3, 1, 2, 4, 5, 5, 6}, 4, 4);

    // 边界：只有一个元素
    check({1}, 1, 1);

    // 边界：k = 1（最大值）/ k = n（最小值）
    check({3, 2, 1, 5, 6, 4}, 1, 6);
    check({3, 2, 1, 5, 6, 4}, 6, 1);

    // 边界：全部相同
    check({2, 2, 2, 2}, 3, 2);

    // 边界：负数与极值
    check({-1, -1}, 2, -1);
    check({-10000, 10000, 0}, 2, 0);
    check({10000, -10000}, 1, 10000);

    // 边界：已经升序 / 降序
    check({1, 2, 3, 4, 5}, 2, 4);
    check({5, 4, 3, 2, 1}, 2, 4);

    // 边界：数据规模上限 10^5
    {
        vector<int> nums(100000);
        for (int i = 0; i < 100000; ++i) nums[i] = (i * 37) % 20001 - 10000;
        vector<int> reference = nums;
        sort(reference.begin(), reference.end());

        Solution solution;
        for (int k : {1, 2, 500, 99999, 100000})
        {
            auto a = nums, b = nums;
            int want = reference[reference.size() - k];
            assert(solution.findKthLargestQuickSelect(a, k) == want);
            assert(solution.findKthLargestHeap(b, k) == want);
        }
    }

    // 交叉验证：小数组上对所有 k 逐一比较「排序答案」与两种解法
    {
        vector<int> nums = {7, 1, 5, 3, 6, 4, 2, 8, 0, 9, 5};
        vector<int> reference = nums;
        sort(reference.begin(), reference.end());

        Solution solution;
        for (int k = 1; k <= static_cast<int>(nums.size()); ++k)
        {
            auto a = nums, b = nums;
            int want = reference[reference.size() - k];
            assert(solution.findKthLargestQuickSelect(a, k) == want);
            assert(solution.findKthLargestHeap(b, k) == want);
        }
    }

    cout << "all tests passed" << endl;
    return 0;
}
