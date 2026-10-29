/*
============================================================================
面试题 17.14. 最小 K 个数（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/smallest-k-lcci/
分类：排序 / 堆 / 快速选择（快排思想）

设计一个算法，找出数组中最小的 k 个数。以任意顺序返回这 k 个数均可。

示例：
输入： arr = [1,3,5,7,2,4,6,8], k = 4
输出： [1,2,3,4]

提示：
0 <= len(arr) <= 100000
0 <= k <= min(100000, len(arr))
*/
#include <algorithm>
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

/*
解法一：直接排序后取前 k 个【非最优】

官方方法一。对原数组从小到大排序，前 k 个就是答案。

关键点：
  · 题目允许任意顺序返回，因此排序结果直接截断即可
  · 排序会把整个数组都处理好，但我们只关心最小的 k 个，做了多余的工作
  · k == 0 时要返回空数组，别越界

复杂度分析：
  时间 O(n log n)：排序
  空间 O(log n)：排序递归栈

解法二：大小为 k 的大根堆【非最优】

官方方法二。用一个大根堆实时维护「当前最小的 k 个数」：
  · 先把前 k 个数入堆；
  · 之后每遇到一个比堆顶更小的数，就弹出堆顶、把新数入堆；
  · 扫描结束后，堆里的 k 个数就是答案。

关键点：
  · 必须用大根堆（堆顶是当前 k 小里的最大值），这样才方便「淘汰最大的那个」
  · 只有比堆顶更小才替换；相等时替换与不替换结果都对，不必处理
  · k == 0 时堆为空，必须提前返回，否则堆顶无定义
  · 适合 n 很大、k 很小（或数据流无法整体载入内存）的场景

复杂度分析：
  时间 O(n log k)：每个元素至多一次 O(log k) 的堆操作
  空间 O(k)：堆的容量

解法三：快排思想（随机化快速选择）【最优解】

官方方法三。快排的划分（partition）每做一次，就能确定主元的最终位置 pos，
并且 pos 左边都不大于它、右边都不小于它。因此：
  · pos 左边的元素个数（含 pos）恰好是 pos - l + 1 个「当前区间最小」的数；
  · 若这个数量正好等于 k，说明左边这段就是要找的 k 个数，直接结束；
  · 若 k 更小，只需递归左半边；
  · 若 k 更大，说明左边这段都要，还需在右半边再找 k - num 个。
每次只递归一侧，总代价期望 O(n)。

关键点：
  · 划分以区间末尾 nums[r] 为主元；随机化是把「随机选中的下标」先换到末尾，
    避免在有序/近似有序数据上退化成 O(n^2)
  · partition 用「小于等于主元就往前换」的单向写法，最后把主元换到 i + 1
  · 递归返回后，数组前 k 个位置就是答案（内部无序，但集合正确）
  · k == 0 要提前返回空数组；l >= r 是递归出口
  · 判方向时用的是「左边个数 num = pos - l + 1」，不是绝对下标 pos

以 arr = [1,3,5,7,2,4,6,8]，k = 4 为例：
  随机化划分一次，可能得到 pos 落在 3 附近，左边 {1,3,2,4} 共 4 个 → num == k，结束
  返回前 4 个（某种排列，排序后是 [1,2,3,4]）
  若某次 num > k，则只在左半边继续找 k 个；若 num < k，则右半边再找 k - num 个

复杂度分析：
  时间 期望 O(n)，最坏 O(n^2)（划分点极端时）
  空间 期望 O(log n)，最坏 O(n)（递归栈）

三解对比：
  · 排序：实现最简单，O(n log n)，n = 1e5 完全够用
  · 大根堆：O(n log k)，适合 k 远小于 n 或流式数据
  · 快速选择：期望 O(n)，是本题的渐进最优解
*/
class Solution
{
private:
    // 以 nums[r] 为主元的划分（Lomuto 分区），返回主元最终下标
    int partition(vector<int> &nums, int l, int r)
    {
        int pivot = nums[r];
        int i = l - 1;
        for (int j = l; j < r; ++j)
        {
            if (nums[j] <= pivot) swap(nums[++i], nums[j]);
        }
        swap(nums[i + 1], nums[r]);
        return i + 1;
    }

    // 随机选一个下标与末尾交换，再划分
    int randomizedPartition(vector<int> &nums, int l, int r)
    {
        int i = rand() % (r - l + 1) + l;
        swap(nums[r], nums[i]);
        return partition(nums, l, r);
    }

    // 处理 [l, r]，使得区间内最小的 k 个数落在左端
    void randomizedSelected(vector<int> &arr, int l, int r, int k)
    {
        if (l >= r) return;

        int pos = randomizedPartition(arr, l, r);
        int num = pos - l + 1; // 左段（含 pos）的元素个数
        if (k == num) return;
        if (k < num) randomizedSelected(arr, l, pos - 1, k);
        else randomizedSelected(arr, pos + 1, r, k - num);
    }

public:
    // ---------- 解法一：排序 ----------
    vector<int> smallestKSort(vector<int> arr, int k)
    {
        if (k <= 0) return {};
        sort(arr.begin(), arr.end());
        return vector<int>(arr.begin(), arr.begin() + k);
    }

    // ---------- 解法二：大小为 k 的大根堆 ----------
    vector<int> smallestKHeap(const vector<int> &arr, int k)
    {
        if (k <= 0) return {};

        // 默认就是大根堆（less<int>）
        priority_queue<int> q;
        for (int i = 0; i < static_cast<int>(arr.size()); ++i)
        {
            if (i < k)
            {
                q.push(arr[i]);
            }
            else if (arr[i] < q.top())
            {
                q.pop();
                q.push(arr[i]);
            }
        }

        vector<int> res;
        res.reserve(k);
        while (!q.empty())
        {
            res.push_back(q.top());
            q.pop();
        }
        return res;
    }

    // ---------- 解法三：随机化快速选择（最优解） ----------
    vector<int> smallestK(vector<int> &arr, int k)
    {
        if (k <= 0) return {};
        if (k >= static_cast<int>(arr.size())) return arr;

        randomizedSelected(arr, 0, static_cast<int>(arr.size()) - 1, k);
        return vector<int>(arr.begin(), arr.begin() + k);
    }
};

namespace
{
void check(vector<int> arr, int k)
{
    Solution solution;

    vector<int> want = arr;
    sort(want.begin(), want.end());
    if (k < static_cast<int>(want.size())) want.resize(k);

    auto gotSort = solution.smallestKSort(arr, k);
    auto gotHeap = solution.smallestKHeap(arr, k);

    vector<int> mutableArr = arr;
    auto gotQuick = solution.smallestK(mutableArr, k);

    sort(gotSort.begin(), gotSort.end());
    sort(gotHeap.begin(), gotHeap.end());
    sort(gotQuick.begin(), gotQuick.end());

    assert(gotSort == want);
    assert(gotHeap == want);
    assert(gotQuick == want);
}
} // namespace

int main()
{
    // 固定随机种子，保证本文件的用例可复现（正式提交时可用 time(NULL)）
    srand(20261029);

    // 示例
    check({1, 3, 5, 7, 2, 4, 6, 8}, 4);

    // 边界：k = 0 返回空数组
    check({1, 2, 3}, 0);

    // 边界：k = n 返回全部元素
    check({5, 1, 3}, 3);

    // 边界：空数组
    check({}, 0);

    // 边界：单元素
    check({7}, 1);

    // 边界：全部相同
    check({2, 2, 2, 2, 2}, 3);

    // 边界：负数
    check({-1, -5, 3, 0, -2}, 2);

    // 边界：已经有序 / 逆序
    check({1, 2, 3, 4, 5, 6}, 3);
    check({6, 5, 4, 3, 2, 1}, 3);

    // 数据规模上限 n = 100000，k = 100000 与 k = 1000
    {
        vector<int> big(100000);
        for (int i = 0; i < 100000; ++i) big[i] = (i * 7919) % 100003;
        check(big, 100000);
        check(big, 1000);
    }

    cout << "all tests passed" << endl;
    return 0;
}
