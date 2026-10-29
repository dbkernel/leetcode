/*
============================================================================
973. 最接近原点的 K 个点（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/k-closest-points-to-origin/
分类：堆 · Top-K 选择（大根堆 / 排序）
============================================================================

给定一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点，
并且是一个整数 k ，返回离原点 (0,0) 最近的 k 个点。

这里，平面上两点之间的距离是 欧几里德距离。

你可以按 任何顺序 返回答案。除了点坐标要被确定外，答案里点的顺序是任意的。

示例 1：
输入：points = [[1,3],[-2,2]], k = 1
输出：[[-2,2]]
解释：
(1, 3) 和原点之间的距离为 sqrt(10)，
(-2, 2) 和原点之间的距离为 sqrt(8)，
由于 sqrt(8) < sqrt(10)，(-2, 2) 离原点更近。
我们只需要距离原点最近的 K = 1 个点，所以答案就是 [[-2,2]]。

示例 2：
输入：points = [[3,3],[5,-1],[-2,4]], k = 2
输出：[[3,3],[-2,4]]
（答案 [[-2,4],[3,3]] 也会被接受。）

提示：
1 <= k <= points.length <= 10^4
-10^4 < xi, yi < 10^4
*/
#include <algorithm>
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

namespace
{
// 距离的比较只需要「谁更近」，平方与开方同序，所以全程用平方避免浮点误差
long long dist2(const vector<int> &point)
{
    long long x = point[0];
    long long y = point[1];
    return x * x + y * y;
}
} // namespace

/*
解法一：全部排序后取前 k 个【非最优】

为什么慢：把 n 个点按到原点的距离排一遍，再取前 k 个，时间 O(n log n)。
但 k 可能远小于 n（比如 n = 10^4、k = 1），为了一次排名把后面的 n - k 个
也排到精确位置，做的比较绝大多数都是白做。

换个角度：只要前 k 名，就只维护 k 个候选。用一个能随时丢掉「当前最差者」
的结构，把每个点拿来和门槛比一次即可。

关键点：
  · 比较用距离的平方，不用 sqrt —— 平方是单调的，既不改变顺序，也不引入
    浮点误差（坐标｜x｜< 10^4，平方和最大 2×10^8，long long 足够）
  · 排序键必须包含距离，不能只比较 x 或 y
  · 距离相同时谁在前都可以：题目只要求「点坐标被确定」这一层唯一性

以示例 2（points = [[3,3],[5,-1],[-2,4]], k = 2）为例：
  距离平方：18、26、20
  排序后为 18、20、26，取前两个 → [[3,3],[-2,4]] ✔

复杂度分析：
  时间 O(n log n)：完整排序
  空间 O(n)：索引数组；若直接排 points 则是 O(log n) 栈
*/
class Solution1
{
public:
    vector<vector<int>> kClosest(vector<vector<int>> &points, int k)
    {
        vector<int> order(points.size());
        for (size_t i = 0; i < order.size(); ++i)
        {
            order[i] = static_cast<int>(i);
        }

        sort(order.begin(), order.end(),
             [&points](int a, int b)
             {
                 return dist2(points[a]) < dist2(points[b]);
             });

        vector<vector<int>> result;
        result.reserve(static_cast<size_t>(k));
        for (int i = 0; i < k; ++i)
        {
            result.push_back(points[order[i]]);
        }
        return result;
    }
};

/*
解法二：大小为 k 的大根堆【最优解】

为什么慢：解法一把「筛选」和「排序」合并成了一件事。筛选只需要知道
「第 k 近的距离是多少」，也就是一个阈值，不需要后面元素的精确次序。

换个角度：维护一个容量为 k 的大根堆，堆顶是候选里**最远**的那个点。
新点比堆顶更近就替换堆顶，否则直接丢掉。扫描结束时堆里剩下的就是最近的
k 个点，代价从 O(n log n) 降到 O(n log k)。

关键点：
  · 这里必须是大根堆（C++ priority_queue 的默认取向）：堆顶是最差的候选，
    才有「谁该被淘汰」这个可直接读出的答案
  · 距离相等时不必替换 —— 题目保证点坐标的集合唯一，替换与否不影响结果
  · 入堆要带上坐标，只存距离的话最后还得回查，白白多一次映射
  · k == n 时堆退化成全集，逻辑不变，不需要特判

以示例 2（points = [[3,3],[5,-1],[-2,4]], k = 2）为例：
  放入 (18,[3,3])、(26,[5,-1])，堆顶是 26
  再来 (20,[-2,4])：20 < 26，弹掉堆顶再放入 → 堆内 18、20
  结果 [[3,3],[-2,4]] ✔

复杂度分析：
  时间 O(n log k)：每个点最多一次入堆，堆大小不超过 k
  空间 O(k)：堆里只有 k 个候选
*/
class Solution2
{
public:
    vector<vector<int>> kClosest(vector<vector<int>> &points, int k)
    {
        // 大根堆：堆顶是候选中最远的点，也就是最该被淘汰的那个
        priority_queue<pair<long long, vector<int>>> heap;

        for (const vector<int> &point : points)
        {
            long long d = dist2(point);
            if (static_cast<int>(heap.size()) < k)
            {
                heap.push({d, point});
            }
            else if (d < heap.top().first)
            {
                // 只有更近才替换：相等时保留原候选，集合仍然唯一
                heap.pop();
                heap.push({d, point});
            }
        }

        vector<vector<int>> result;
        result.reserve(static_cast<size_t>(k));
        while (!heap.empty())
        {
            result.push_back(heap.top().second);
            heap.pop();
        }
        return result;
    }
};

// ===== 测试辅助 =====
namespace
{
// 题目不要求顺序，把两个结果都按坐标排序后再比较
vector<vector<int>> normalized(vector<vector<int>> v)
{
    sort(v.begin(), v.end());
    return v;
}

// 用暴力排序算出应有的集合，作为期望值
vector<vector<int>> brute(const vector<vector<int>> &points, int k)
{
    vector<vector<int>> sorted_points = points;
    sort(sorted_points.begin(), sorted_points.end(),
         [](const vector<int> &a, const vector<int> &b)
         {
             return dist2(a) < dist2(b);
         });
    sorted_points.resize(static_cast<size_t>(k));
    return sorted_points;
}

void check(const vector<vector<int>> &points, int k)
{
    vector<vector<int>> expect = normalized(brute(points, k));

    Solution1 s1;
    vector<vector<int>> in1 = points;
    vector<vector<int>> got1 = s1.kClosest(in1, k);
    assert(normalized(got1) == expect);
    assert(static_cast<int>(got1.size()) == k);

    Solution2 s2;
    vector<vector<int>> in2 = points;
    vector<vector<int>> got2 = s2.kClosest(in2, k);
    assert(normalized(got2) == expect);
    assert(static_cast<int>(got2.size()) == k);
}
} // namespace

int main()
{
    // 示例 1：k = 1，验证只取最近的那一个
    check({{1, 3}, {-2, 2}}, 1);

    // 示例 2：三个点取两个，两种输出顺序都合法
    check({{3, 3}, {5, -1}, {-2, 4}}, 2);

    // 边界：只有一个点
    check({{0, 0}}, 1);

    // 边界：k 等于点的个数，必须原样返回全部
    check({{1, 1}, {2, 2}, {-3, -3}}, 3);

    // 边界：重复坐标，堆里会并存同一个点，不能去重也不能少放
    check({{1, 1}, {1, 1}, {5, 5}}, 2);

    // 边界：坐标里有 0 与负数，验证平方不会把符号丢掉
    check({{-3, 0}, {0, -4}, {3, 3}, {-1, -1}}, 2);

    // 边界：等距点全部落在 k 之内，答案集合仍然唯一
    check({{1, 1}, {-1, -1}, {2, 2}}, 2);

    // 规模：1000 个点取 10 个，和暴力排序的结果对照
    {
        vector<vector<int>> points;
        for (int i = 0; i < 1000; ++i)
        {
            // 用固定公式造点，保证每次运行结果一致
            int x = (i * 37) % 4001 - 2000;
            int y = (i * 91) % 4001 - 2000;
            points.push_back({x, y});
        }
        check(points, 10);
    }

    cout << "all tests passed" << endl;
    return 0;
}
