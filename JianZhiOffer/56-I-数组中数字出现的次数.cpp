/*
============================================================================
LCR 177 / 剑指 Offer 56 - I. 撞色搭配（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-lcof/
分类：位运算 · 分组异或 / 最低位的 1

整数数组 sockets 记录了一个袜子礼盒的颜色分布情况，其中 sockets[i] 表示
该袜子的颜色编号。礼盒中除了一款撞色搭配的袜子，每种颜色的袜子均有两只。
请设计一个程序，在时间复杂度 O(n)，空间复杂度O(1) 内找到这双撞色搭配袜子
的两个颜色编号。

示例 1：
输入：sockets = [4, 5, 2, 4, 6, 6]
输出：[2,5] 或 [5,2]

示例 2：
输入：sockets = [1, 2, 4, 1, 4, 3, 12, 3]
输出：[2,12] 或 [12,2]

提示：
- 2 <= sockets.length <= 10000

说明：
- 本题原名「数组中数字出现的次数」，力扣已改号为 LCR 177 并改题名为
  「撞色搭配」，函数签名由 singleNumbers 改为 sockCollocation，题面换成
  袜子配色，数据要求不变。
- 与主站 260 题「只出现一次的数字 III」同题。
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法：分组异或，用最低位的 1 把数组拆成两半【最优解】

为什么慢：直观做法是用哈希表数每个颜色出现的次数，时间 O(n) 但空间也 O(n)，
而题目要求空间 O(1)；对每个颜色再扫描一遍全数组则退化成 O(n^2)。两条路都
不合要求，说明答案必须从「成对」这个结构里挤出来。

换个角度：整组异或会把所有成对出现的颜色消掉，剩下的就是两个答案的异或
a^b。这个值不为 0，取它最低位为 1 的那一位，a 与 b 在这一位上必然一个为 1
一个为 0；用这一位当分组依据，就能把 a、b 分到两个不同组，且每个组内部
其余颜色依旧成对，各组的异或就分别等于 a 和 b。

关键点：
  · 全体异或的结果一定非 0：若为 0 说明 a 与 b 相同，与「两个不同的颜色」矛盾
  · 掩码取最低位的 1：mask = total & -total，不能笼统地用 total 本身分组
  · 分组判据是 x & mask 是否为 0，不是比较大小，也不是看 total 的最高位
  · 成对元素的值相同、掩码位也相同，必然落进同一组，抵消关系不被破坏
  · 结果的顺序任意，题面明确写了 [2,5] 或 [5,2] 都对

手推一遍，以示例 1（sockets = [4,5,2,4,6,6]）为例：
  total = 4^5^2^4^6^6 = 5^2 = 7（二进制 111）
  最低位的 1 是第 0 位，mask = 1
  按 bit0 分组：4、6 的 bit0 为 0 且各出现两次 → 互相抵消
                2 的 bit0 为 0，单独留在 0 组
                5 的 bit0 为 1，单独留在 1 组
  两组异或分别得到 2 和 5 ✔

复杂度分析：
  时间 O(n)：两趟遍历，一趟求 total，一趟分组异或
  空间 O(1)：只用 total、mask 和两个累加器，与数组长度无关
*/
class Solution
{
public:
    // 返回只出现一次的两个颜色编号，顺序任意
    vector<int> sockCollocation(vector<int> &sockets)
    {
        int total = 0;
        for (int x : sockets) total ^= x;

        // 取最低位的 1：a 与 b 在这一位上必然不同，可据此分组。
        // 用无符号运算，避免 total 为 INT_MIN 时 -total 的未定义行为
        unsigned low = static_cast<unsigned>(total) &
                       (0u - static_cast<unsigned>(total));
        int mask = static_cast<int>(low);

        int a = 0, b = 0;
        for (int x : sockets)
        {
            // 同组内部的成对颜色仍会抵消，剩下的就是该组的答案
            if (x & mask) a ^= x;
            else b ^= x;
        }
        return {a, b};
    }
};

namespace
{
// 题面允许任意顺序，统一排序后比较
void check(vector<int> sockets, vector<int> want)
{
    Solution solution;
    vector<int> got = solution.sockCollocation(sockets);
    sort(got.begin(), got.end());
    sort(want.begin(), want.end());
    assert(got == want);
}
} // namespace

int main()
{
    // 示例 1
    check({4, 5, 2, 4, 6, 6}, {2, 5});

    // 示例 2
    check({1, 2, 4, 1, 4, 3, 12, 3}, {2, 12});

    // 指定用例：两个单例分别落在首尾
    check({4, 1, 4, 6}, {1, 6});

    // 边界：长度取下限 2，没有任何成对的元素
    check({1, 2}, {1, 2});

    // 边界：单例的颜色编号较大，掩码落在高位
    check({10000, 1, 10000, 9999}, {1, 9999});

    // 边界：抵消后 total 恰好只有一位为 1（掩码即该位）
    check({8, 3, 3, 8, 5, 13}, {5, 13});

    // 边界：单例之一是 0，异或分组不能把 0 当成「不存在」
    check({0, 7, 7, 4}, {0, 4});

    cout << "all tests passed" << endl;
    return 0;
}
