/*
面试题 10.01. 合并排序的数组

给定两个排序后的数组 A 和 B，其中 A 的末端有足够的缓冲空间容纳 B。 编写一个方法，将 B 合并入 A 并排序。
初始化 A 和 B 的元素数量分别为 m 和 n。

示例:

输入:
A = [1,2,3,0,0,0], m = 3
B = [2,5,6],       n = 3

输出: [1,2,2,3,5,6]

说明:
A.length == n + m

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sorted-merge-lcci

*/
#include <iostream>
#include <cassert>
#include <vector>
using namespace std;

/*
双指针法：太普通了。。。。。

这一方法将两个数组看作队列，每次从两个数组头部取出比较小的数字放到结果中。

时间复杂度：O(m+n)。
指针移动单调递增，最多移动 m+n 次，因此时间复杂度为 O(m+n)。

空间复杂度：O(m+n)。
需要建立长度为 m+n 的中间数组 sorted。
*/
class SolutionDoublePointer
{
public:
    void merge(vector<int> &A, int m, vector<int> &B, int n)
    {
        int pa = 0, pb = 0;
        int sorted[m + n]; // 开辟新数组用于存放临时结果
        int cur;
        while (pa < m || pb < n)
        {
            if (pa == m)
                cur = B[pb++];
            else if (pb == n)
                cur = A[pa++];
            else if (A[pa] < B[pb])
                cur = A[pa++];
            else
                cur = B[pb++];
            sorted[pa + pb - 1] = cur;
        }
        // 将临时数组数据拷贝回 vector A
        for (int i = 0; i < m + n; i++)
            A[i] = sorted[i];
    }
};

/*
逆向双指针：非常巧妙
上一方法中，之所以要使用临时变量，是因为如果直接合并到数组 A 中，A 中的元素可能会在取出之前被覆盖。那么如何直接避免覆盖 A 中的元素呢？
观察可知，A 的后半部分是空的，可以直接覆盖而不会影响结果。因此可以指针设置为从后向前遍历，每次取两者之中的较大者放进 A 的最后面。

严格来说，在此遍历过程中的任意一个时刻，A 数组中有 m−pa−1 个元素被放入 A 的后半部，B 数组中有 n−pb−1 个元素被放入 A 的后半部，
而在指针 pa 的后面，A 数组有 m+n−pa−1 个位置。由于 m+n−pa−1≥m−pa−1+n−pb−1 等价于 pb≥−1 永远成立，
因此 pa 后面的位置永远足够容纳被插入的元素，不会产生 pa 的元素被覆盖的情况。

时间复杂度：O(m+n)。
指针移动单调递减，最多移动 m+n 次，因此时间复杂度为 O(m+n)。

空间复杂度：O(1)。
直接对数组 A 原地修改，不需要额外空间。
*/
class Solution
{
public:
    void merge(vector<int> &A, int m, vector<int> &B, int n)
    {
        int pa = m - 1, pb = n - 1, p = m + n - 1;
        while (pa >= 0 || pb >= 0)
        {
            if (pa == -1)
                A[p--] = B[pb--];
            else if (pb == -1)
                A[p--] = A[pa--];
            else if (A[pa] > B[pb])
                A[p--] = A[pa--];
            else
                A[p--] = B[pb--];
        }
    }
};

int main()
{
    {
        vector<int> vec1 = {1, 3, 5, 6, 0, 0, 0};
        vector<int> vec2 = {2, 3, 7};
        vector<int> expect = vector<int>{1, 2, 3, 3, 5, 6, 7};

        SolutionDoublePointer s;
        s.merge(vec1, 4, vec2, 3);
        assert(vec1 == expect);
    }

    {
        vector<int> vec1 = {1, 3, 5, 6, 0, 0, 0};
        vector<int> vec2 = {2, 3, 7};
        vector<int> expect = vector<int>{1, 2, 3, 3, 5, 6, 7};

        Solution s;
        s.merge(vec1, 4, vec2, 3);
        assert(vec1 == expect);
    }

    return 0;
}
