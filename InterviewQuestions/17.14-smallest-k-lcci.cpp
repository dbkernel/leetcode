/*
面试题 17.14. 最小K个数

设计一个算法，找出数组中最小的k个数。以任意顺序返回这k个数均可。

示例：
输入： arr = [1,3,5,7,2,4,6,8], k = 4
输出： [1,2,3,4]

提示：
0 <= len(arr) <= 100000
0 <= k <= min(100000, len(arr))

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/smallest-k-lcci

*/
#include <iostream>
#include <cassert>
#include <vector>
#include <queue>
using namespace std;

/*
方法一：快速排序

复杂度分析：
- 时间复杂度：期望为 O(n) ，由于证明过程很繁琐，所以不在这里展开讲。
  具体证明可以参考《算法导论》第 9 章第 2 小节。最坏情况下的时间复杂度为 O(n^2)。
  情况最差时，每次的划分点都是最大值或最小值，一共需要划分 n−1 次，而一次划分需要线性的时间复杂度，所以最坏情况下时间复杂度为 O(n^2)。
- 空间复杂度：期望为 O(logn)，递归调用的期望深度为 O(logn)，每层需要的空间为 O(1)，只有常数个变量。
  最坏情况下的空间复杂度为 O(n)。最坏情况下需要划分 n 次，即 quickSelect 函数递归调用最深 n−1 层，而每层由于需要 O(1) 的空间，所以一共需要 O(n) 的空间复杂度。
*/
class Solution
{
public:
    vector<int> smallestK(vector<int> &arr, int k)
    {
        quickSelect(arr, 0, arr.size() - 1, k);
        vector<int> b(arr.begin(), arr.begin() + k); // 返回结果集
        return b;
    }

    void quickSelect(vector<int> &arr, int left, int right, int k)
    {
        if (left < right)
        {
            int pos = partition(arr, left, right);
            if (pos == k - 1) // 找到第 k - 1 个下标即停止排序
                return;
            else if (pos < k - 1)
                quickSelect(arr, pos + 1, right, k);
            else
                quickSelect(arr, left, pos - 1, k);
        }
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

/*
From 官方题解：

方法二：堆

我们用一个大根堆实时维护数组的前 k 小值。首先将前 k 个数插入大根堆中，随后从第 k+1 个数开始遍历，
如果当前遍历到的数比大根堆的堆顶的数要小，就把堆顶的数弹出，再插入当前遍历到的数。
最后将大根堆里的数存入数组返回即可。在下面的代码中，由于 C++ 语言中的堆（即优先队列）为大根堆，我们可以这么做。
而 Python 语言中的堆为小根堆，因此我们要对数组中所有的数取其相反数，才能使用小根堆维护前 k 小值。

复杂度分析：
- 时间复杂度：O(nlogk)，其中 n 是数组 arr 的长度。由于大根堆实时维护前 k 小值，
  所以插入删除都是 O(logk) 的时间复杂度，最坏情况下数组里 n 个数都会插入，所以一共需要 O(nlogk) 的时间复杂度。
- 空间复杂度：O(k)，因为大根堆里最多 k 个数。
*/
class Solution1
{
public:
    vector<int> smallestK(vector<int> &arr, int k)
    {
        vector<int> vec(k, 0);
        if (k == 0)
            return vec; // 排除 0 的情况

        priority_queue<int> Q;
        // 将前 k 个数插入大根堆中
        for (int i = 0; i < k; ++i)
            Q.push(arr[i]);

        // 如果当前遍历到的数比大根堆的堆顶的数要小，就把堆顶的数弹出，再插入当前遍历到的数。
        for (int i = k; i < (int)arr.size(); ++i)
        {
            if (Q.top() > arr[i])
            {
                Q.pop();
                Q.push(arr[i]);
            }
        }

        // 获取最小的 K 个元素
        for (int i = 0; i < k; ++i)
        {
            vec[i] = Q.top();
            Q.pop();
        }

        return vec;
    }
};

int main()
{
    vector<int> nums = {10, 6, 8, 1, 4, 7, 9};
    Solution s;

    {
        vector<int> expect1 = {1, 4, 6};
        vector<int> expect2 = {1, 6, 4};
        vector<int> expect3 = {4, 1, 6};
        vector<int> expect4 = {4, 6, 1};
        vector<int> expect5 = {6, 4, 1};
        vector<int> expect6 = {6, 1, 4};
        vector<int> result = s.smallestK(nums, 3);
        assert(result == expect1 || result == expect2 || result == expect3 ||
               result == expect4 || result == expect5 || result == expect6);
    }

    Solution1 s1;
    {
        vector<int> expect1 = {1, 4, 6};
        vector<int> expect2 = {1, 6, 4};
        vector<int> expect3 = {4, 1, 6};
        vector<int> expect4 = {4, 6, 1};
        vector<int> expect5 = {6, 4, 1};
        vector<int> expect6 = {6, 1, 4};
        vector<int> result = s1.smallestK(nums, 3);
        assert(result == expect1 || result == expect2 || result == expect3 ||
               result == expect4 || result == expect5 || result == expect6);
    }

    return 0;
}
