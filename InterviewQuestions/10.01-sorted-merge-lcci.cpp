/*
============================================================================
面试题 10.01. 合并排序的数组（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sorted-merge-lcci/
分类：数组 · 双指针（归并）

给定两个排序后的数组 A 和 B，其中 A 的末端有足够的缓冲空间容纳 B。 编写一个方法，
将 B 合并入 A 并排序。

初始化 A 和 B 的元素数量分别为 m 和 n。

示例：
输入：
A = [1,2,3,0,0,0], m = 3
B = [2,5,6],       n = 3

输出： [1,2,2,3,5,6]

说明：
A.length == n + m
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：直接合并后排序【非最优】

官方方法一。把 B 直接搬到 A 的尾部，然后对整条 A 排序。

关键点：
  · 写法最短，但没有利用「两个数组各自有序」这一条件
  · 排序 A 的代价是 O((m+n)log(m+n))，比线性归并慢

复杂度分析：
  时间 O((m+n)log(m+n))，空间 O(log(m+n))（排序栈开销）

解法二：双指针（借助临时数组）【非最优】

官方方法二。把 A、B 看成两个队列，每次从两队的队首取较小者放进结果数组。
用一个长度为 m+n 的临时数组接住结果，最后整体拷回 A。

关键点：
  · 利用了「已排序」的性质，时间降为线性
  · 但需要一个额外的长度为 m+n 的数组，空间 O(m+n)
  · 指针 pa 走到 m 或 pb 走到 n 之一时，另一侧要整体搬完（别漏了尾巴）

复杂度分析：
  时间 O(m+n)，空间 O(m+n)

解法三：逆向双指针（原地）【最优解】

官方方法三。既然 A 的后半段本来就是空的，那就不该「从头往尾填」而应该「从尾往头填」：
比较 A 与 B 的最后一个元素，把较大的那个放到 A 的最后一位，然后指针前移。
这样写入位置永远在还没读取的位置之后，不会覆盖 A 中尚未处理的数据。

为什么不会覆盖？设 pa、pb 为当前待比较的下标，tail 为待写入下标。此时
A 中已搬到后半部分的有 m-pa-1 个、B 中已搬的有 n-pb-1 个，而 pa 之后还剩
m+n-pa-1 个位置。由 m+n-pa-1 >= (m-pa-1)+(n-pb-1) 等价于 pb >= -1 恒成立，
所以位置永远够用 —— 这就是「从后往前」能原地完成的原因。

关键点：
  · 从后往前比较、从后往前写入，是大数组归并的标准技巧
  · pa < 0 时只能搬 B，pb < 0 时只能搬 A
  · 若 B 先被搬完，A 前面剩下的元素本来就在正确位置上，什么都不用做
  · 比较用 A[pa] > B[pb]，相等时先取 B（或 A）都可以，结果都正确

以示例（A = [1,2,3,0,0,0], m = 3；B = [2,5,6], n = 3）为例：
  pa = 2 (A[2]=3)、pb = 2 (B[2]=6)、tail = 5
  3 < 6 → A[5] = 6，pb = 1，tail = 4
  3 < 5 → A[4] = 5，pb = 0，tail = 3
  3 > 2 → A[3] = 3，pa = 1，tail = 2
  2 与 2 相等 → 取 B：A[2] = 2，pb = -1，tail = 1
  pb < 0 → A[1] = A[1] = 2，pa = 0，tail = 0
  pb < 0 → A[0] = A[0] = 1，pa = -1，结束
  结果 [1,2,2,3,5,6]

复杂度分析：
  时间 O(m+n)：每个元素恰好搬一次
  空间 O(1)：原地完成，不用临时数组
*/
class Solution
{
public:
    // 解法一：直接合并后排序
    void merge1(vector<int> &A, int m, vector<int> &B, int n)
    {
        for (int i = 0; i < n; ++i) A[m + i] = B[i];
        sort(A.begin(), A.end());
    }

    // 解法二：双指针 + 临时数组
    void merge2(vector<int> &A, int m, vector<int> &B, int n)
    {
        vector<int> sorted;
        sorted.reserve(m + n);
        int pa = 0, pb = 0;
        while (pa < m || pb < n)
        {
            if (pa == m)
            {
                sorted.push_back(B[pb++]);
            }
            else if (pb == n)
            {
                sorted.push_back(A[pa++]);
            }
            else if (A[pa] < B[pb])
            {
                sorted.push_back(A[pa++]);
            }
            else
            {
                sorted.push_back(B[pb++]);
            }
        }
        for (int i = 0; i < m + n; ++i) A[i] = sorted[i];
    }

    // 解法三：逆向双指针（原地，最优解）
    void merge(vector<int> &A, int m, vector<int> &B, int n)
    {
        int pa = m - 1, pb = n - 1;
        int tail = m + n - 1;
        while (pa >= 0 || pb >= 0)
        {
            if (pa == -1)
            {
                A[tail--] = B[pb--];
            }
            else if (pb == -1)
            {
                A[tail--] = A[pa--];
            }
            else if (A[pa] > B[pb])
            {
                A[tail--] = A[pa--];
            }
            else
            {
                A[tail--] = B[pb--];
            }
        }
    }
};

namespace
{
// 由「有效元素」和 B 构造 A（尾部补 0），返回 A
vector<int> build(const vector<int> &values, int extra)
{
    vector<int> A = values;
    A.resize(values.size() + extra, 0);
    return A;
}

void check(const vector<int> &aValues, const vector<int> &bValues, const vector<int> &want)
{
    int m = (int)aValues.size();
    int n = (int)bValues.size();
    Solution solution;

    // 三种解法都要跑到同一份期望结果
    vector<int> a1 = build(aValues, n);
    vector<int> b1 = bValues;
    solution.merge1(a1, m, b1, n);
    assert(a1 == want);

    vector<int> a2 = build(aValues, n);
    vector<int> b2 = bValues;
    solution.merge2(a2, m, b2, n);
    assert(a2 == want);

    vector<int> a3 = build(aValues, n);
    vector<int> b3 = bValues;
    solution.merge(a3, m, b3, n);
    assert(a3 == want);
}
} // namespace

int main()
{
    // 示例
    check({1, 2, 3}, {2, 5, 6}, {1, 2, 2, 3, 5, 6});

    // 边界：A 为空（m = 0），B 整体搬入
    check({}, {1, 2, 3}, {1, 2, 3});

    // 边界：B 为空（n = 0），A 不变
    check({1, 2, 3}, {}, {1, 2, 3});

    // 边界：两者都为空
    check({}, {}, {});

    // 边界：B 全部小于 A
    check({4, 5, 6}, {1, 2, 3}, {1, 2, 3, 4, 5, 6});

    // 边界：B 全部大于 A
    check({1, 2, 3}, {4, 5, 6}, {1, 2, 3, 4, 5, 6});

    // 边界：大量重复元素
    check({1, 1, 1}, {1, 1}, {1, 1, 1, 1, 1});

    // 边界：交错合并
    check({1, 3, 5, 7}, {2, 4, 6, 8}, {1, 2, 3, 4, 5, 6, 7, 8});

    // 边界：含负数
    check({-5, -1, 3}, {-4, 0, 9}, {-5, -4, -1, 0, 3, 9});

    cout << "all tests passed" << endl;
    return 0;
}
