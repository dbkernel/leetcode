/*
============================================================================
295. 数据流的中位数（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-median-from-data-stream/
分类：堆 · 对顶堆（数据流中位数）
============================================================================

中位数是有序整数列表中的中间值。如果列表的大小是偶数，则没有中间值，
中位数是两个中间值的平均值。

例如 arr = [2,3,4] 的中位数是 3 。
例如 arr = [2,3] 的中位数是 (2 + 3) / 2 = 2.5 。

实现 MedianFinder 类:
- MedianFinder() 初始化 MedianFinder 对象。
- void addNum(int num) 将数据流中的整数 num 添加到数据结构中。
- double findMedian() 返回到目前为止所有元素的中位数。与实际答案相差
  10^-5 以内的答案将被接受。

示例 1：
输入：
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
输出：
[null, null, null, 1.5, null, 2.0]

解释：
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // 返回 1.5 ((1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0

提示：
-10^5 <= num <= 10^5
在调用 findMedian 之前，数据结构中至少有一个元素
最多 5 * 10^4 次调用 addNum 和 findMedian
*/
#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

/*
解法一：有序数组 + 二分插入【非最优】

为什么慢：这个解法把「让数组保持有序」当成不变式，于是 addNum 每次都要
把新元素放到正确位置。二分查找只能把「找位置」这一步压到 O(log n)，找到
之后仍要把后面所有元素整体后移一格，那一步是 O(n)。调用 5 * 10^4 次
addNum，最坏就是 5 * 10^4 * 5 * 10^4 次移动。

换个角度：findMedian 只看「中间那一两个元素」，并不需要全体有序。把数据
按大小劈成两半，各自内部只要能让「最靠中间的那个」浮到顶，就够了。

关键点：
  · 用 lower_bound 找第一个 >= num 的位置；用 upper_bound 也合法，
    重复值的相对次序不影响中位数
  · 插入必须真的扩进去（vector::insert），不能只覆盖某个位置
  · 偶数个元素时取 size / 2 与 size / 2 - 1 两处的平均，奇数个时取
    size / 2；两个下标都要在插入之后才成立
  · findMedian 是 O(1)：下标算出来即可，不需要再排序

以示例的调用序列为例：
  addNum(1) → [1]，findMedian 时 size = 1，取 [0] → 1.0
  addNum(2) → [1,2]，size = 2，取 (1 + 2) / 2 → 1.5
  addNum(3) → [1,2,3]，size = 3，取 [1] → 2.0 ✔

复杂度分析：
  时间 addNum O(n)（插入时的整体后移），findMedian O(1)
  空间 O(n)：装下所有元素
*/
class MedianFinderSorted
{
public:
    MedianFinderSorted() = default;

    void addNum(int num)
    {
        // 找第一个不小于 num 的位置，等值元素都排在它后面
        auto pos = lower_bound(data.begin(), data.end(), num);
        data.insert(pos, num);
    }

    double findMedian()
    {
        size_t n = data.size();
        if (n % 2 == 1)
        {
            return data[n / 2];
        }
        // 用 double 相加，避免两个大整数先加后除时溢出或截断
        return (static_cast<double>(data[n / 2 - 1]) + data[n / 2]) / 2.0;
    }

private:
    vector<int> data;
};

/*
解法二：对顶双堆（大根堆放小的一半，小根堆放大的一个半）【最优解】

契约：lo 是大根堆，装较小的一半；hi 是小根堆，装较大的一半。不变式是
|lo| == |hi| 或者 |lo| == |hi| + 1 —— 中位数只由两个堆顶决定。

为什么慢：解法一的瓶颈在「维持全体有序」。可中位数并不关心 lo 内部、hi
内部的次序，只关心两条边界：lo 的最大值与 hi 的最小值。把这两条边界做成
堆顶，其余元素爱怎么排就怎么排。

换个角度：新元素只可能落在 lo 的最大值和 hi 的最小值之间。所以先比较一次
决定它进哪个堆，再用一次「搬运」把两个堆的大小差拉回 0 或 1。

关键点：
  · 先决定入哪个堆：num <= lo.top() 进 lo，否则进 hi；lo 为空时直接进 lo
  · 平衡只朝一个方向修：lo 比 hi 多 2 个就搬一个到 hi；hi 比 lo 多就搬回 lo。
    两步都用「先 push 对方堆顶、再 pop」，顺序反了会白搬一个元素
  · 两个堆的大小差永远不超过 1，所以 findMedian 只需读两个堆顶
  · 返回时用 2.0 而不是 2，否则会做整除；相加前先转 double，
    避免两个接近 10^5 的数相加后仍被当成 int
  · findMedian 在空结构上不成立，题面保证调用前至少有一个元素

以示例的调用序列为例：
  addNum(1)：lo = [1]，hi = []
  addNum(2)：2 > 堆顶 1，进 hi → lo = [1]，hi = [2]
    findMedian：两堆等大 → (1 + 2) / 2 = 1.5 ✔
  addNum(3)：3 > 堆顶 2，进 hi → hi 比 lo 多，把 2 搬回 lo
    lo = [2,1]（堆顶 2），hi = [3]；lo 多一个 → 取 lo.top() = 2.0 ✔

复杂度分析：
  时间 addNum O(log n)：最多两次堆操作；findMedian O(1)
  空间 O(n)：两个堆合起来正好装下所有元素
*/
class MedianFinder
{
public:
    MedianFinder() = default;

    void addNum(int num)
    {
        // 只跟 lo 的堆顶比：lo 的最大值就是「较小那一半」的上界
        if (low.empty() || num <= low.top())
        {
            low.push(num);
        }
        else
        {
            high.push(num);
        }
        rebalance();
    }

    double findMedian()
    {
        if (low.size() > high.size())
        {
            return low.top();
        }
        return (static_cast<double>(low.top()) + high.top()) / 2.0;
    }

private:
    // priority_queue 默认大根堆，正好用来取「较小一半」的最大值
    priority_queue<int> low;
    // 小根堆：取「较大一半」的最小值
    priority_queue<int, vector<int>, greater<int>> high;

    void rebalance()
    {
        // 只可能是 lo 多出 2 个，或者 hi 多出 1 个
        if (low.size() > high.size() + 1)
        {
            high.push(low.top());
            low.pop();
        }
        else if (high.size() > low.size())
        {
            low.push(high.top());
            high.pop();
        }
    }
};

// ===== 测试辅助 =====
namespace
{
double bruteMedian(vector<int> v)
{
    sort(v.begin(), v.end());
    size_t n = v.size();
    if (n % 2 == 1)
    {
        return v[n / 2];
    }
    return (static_cast<double>(v[n / 2 - 1]) + v[n / 2]) / 2.0;
}

bool nearlyEqual(double a, double b)
{
    return fabs(a - b) < 1e-6;
}

// 依次 addNum，每到一个检查点就比较两种实现的 findMedian
// probes 里的数字是「已经喂进去多少个元素」，必须递增
void check(const vector<int> &nums, const vector<int> &probes)
{
    MedianFinderSorted sorted_impl;
    MedianFinder heap_impl;

    size_t fed = 0;
    for (int c : probes)
    {
        while (fed < static_cast<size_t>(c))
        {
            sorted_impl.addNum(nums[fed]);
            heap_impl.addNum(nums[fed]);
            ++fed;
        }

        vector<int> prefix(nums.begin(), nums.begin() + c);
        double want = bruteMedian(prefix);
        assert(nearlyEqual(sorted_impl.findMedian(), want));
        assert(nearlyEqual(heap_impl.findMedian(), want));
    }
}
} // namespace

int main()
{
    // 示例 1：按题目给的调用序列回放，1 个、2 个、3 个元素各查一次
    check({1, 2, 3}, {1, 2, 3});

    // 边界：只有一个元素，奇偶分支走的是奇数那一支
    check({5}, {1});

    // 边界：全部是负数，验证大根堆的比较不会因为符号出错
    check({-1, -2, -3, -4, -5}, {1, 2, 3, 4, 5});

    // 边界：全部相同，两个堆的大小差永远是 0 或 1
    check({1, 1, 1, 1}, {1, 2, 3, 4});

    // 边界：先大后小地插入，正是「不预先排序」最容易错的地方
    check({100, 90, 80, 70, 60}, {1, 2, 3, 4, 5});

    // 边界：坐标范围的上下界，平均值必须用 double 算
    check({-100000, 100000}, {1, 2});

    // 边界：奇数个元素的两个相邻值，验证 (a + b) / 2.0 不被截断
    check({1, 2}, {2});

    // 规模：1000 个元素，每加 100 个查一次，和暴力排序对照
    {
        vector<int> nums;
        vector<int> probes;
        for (int i = 0; i < 1000; ++i)
        {
            // 固定公式生成，保证每次运行结果一致
            nums.push_back((i * 37) % 201 - 100);
            if ((i + 1) % 100 == 0)
            {
                probes.push_back(i + 1);
            }
        }
        check(nums, probes);
    }

    cout << "all tests passed" << endl;
    return 0;
}
