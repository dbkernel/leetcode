/*
============================================================================
面试题 05.01. 插入（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/insert-into-bits-lcci/
分类：位运算 · 掩码清零 + 移位拼接

给定两个整型数字 N 与 M，以及表示比特位置的 i 与 j（i <= j，且从 0 位开始
计算）。

编写一种方法，使 M 对应的二进制数字插入 N 对应的二进制数字的第 i ~ j 位区域，
不足之处用 0 补齐。具体插入过程如图所示。

题目保证从 i 位到 j 位足以容纳 M， 例如：M = 10011，则 i～j 区域至少可容纳
5 位。

示例 1：
输入：N = 1024(10000000000), M = 19(10011), i = 2, j = 6
输出：N = 1100(10001001100)

示例 2：
输入：N = 0, M = 31(11111), i = 0, j = 4
输出：N = 31(11111)
*/
#include <cassert>
#include <iostream>
using namespace std;

/*
解法：掩码清零 [i, j] 位，再把 M 左移 i 位或上去【最优解】

官方提示给的就是这条路：先把注意力放在「清除合适的位」上，构造一个形如
111…000…111 的位掩码，再合并两边。它同时是本题的最优解 —— 指令数与区间
长度无关，常数条就够。

为什么慢：逐位把 M 的第 t 位搬到 N 的第 i + t 位，需要 (j - i + 1) 轮
「读一位、写一位」，每轮两次移位加一次掩码，指令数随区间长度线性增长；
而这件事本质上是「整体平移 + 整体覆盖」。

换个角度：插入 = 先把目标区间清零，再把 M 整体左移 i 位并上去。清零只需要
一个掩码，平移只需要一次左移，两步都是常数指令。

关键点：
  · 掩码的构造顺序：先算低 k 位全 1（k = j - i + 1），左移 i 位得到
    「区间内全 1」，再按位取反即得「区间内全 0」
  · 取反必须作用在 32 位无符号上：先转 unsigned 再 ~，否则结果带着符号位
    的含义，与 N 相与时容易把高位也抹掉
  · k 可能等于 32（i = 0、j = 31），1 << 32 在 C++ 里是未定义行为 ——
    必须用 1ULL << k 这类 64 位中间量，不能写 1 << k
  · 清零与或的顺序不能反：先或再清零会把刚写进去的 M 一起抹掉
  · 题目保证区间足够容纳 M，所以不必再截断 M 的高位

手推一遍（N = 1024，M = 19，i = 2，j = 6）：
  k = 5，低 5 位全 1 → 0b11111，左移 2 位 → 0b1111100
  取反 → 高位的 1 全保留，第 2 ~ 6 位为 0
  N = 10000000000 的第 2 ~ 6 位本来就是 0，清零后仍是 1024
  M << 2 = 10011 << 2 = 1001100 = 76
  1024 | 76 = 1100（10001001100）✔

以 i = j = 2、N = 15、M = 0 为例：
  k = 1，掩码清掉第 2 位 → 1111 变 1011 = 11
  M << 2 = 0 → 结果 11 ✔（区间长度为 1 时不能把左右邻居一起抹掉）

复杂度分析：
  时间 O(1)：固定几次移位与位运算，与区间长度无关
  空间 O(1)：只用一个掩码和一个中间结果
*/
class Solution
{
public:
    int insertBits(int N, int M, int i, int j)
    {
        // k 是区间长度，最大可取到 32（i = 0、j = 31）
        unsigned long long k = static_cast<unsigned long long>(j - i + 1);

        // 低 k 位全 1 的掩码；用 64 位中间量，k = 32 时也不会触发 1 << 32 的 UB
        unsigned long long low = (1ULL << k) - 1ULL;

        // 左移 i 位后取反：区间内为 0、区间外为 1，正是「只清这一段」的掩码
        unsigned int mask =
            ~static_cast<unsigned int>(low << static_cast<unsigned>(i));

        // 先清零再或：顺序反了会把刚写进去的 M 一起抹掉
        unsigned int cleared = static_cast<unsigned int>(N) & mask;
        unsigned int merged = static_cast<unsigned int>(M) << i;
        return static_cast<int>(cleared | merged);
    }
};

namespace
{
// 结果可能落在第 31 位上：统一按 32 位无符号比较，避开 int 的符号差异
void check(int N, int M, int i, int j, unsigned int wantBits)
{
    Solution solution;
    unsigned int got =
        static_cast<unsigned int>(solution.insertBits(N, M, i, j));
    assert(got == wantBits);
}
} // namespace

int main()
{
    // 示例 1：区间内的位原本全 0，只需把 M 平移进来
    check(1024, 19, 2, 6, 1100u);

    // 示例 2：整个 N 都被覆盖
    check(0, 31, 0, 4, 31u);

    // 边界：i == j，只替换一位 —— 写错区间长度会把左右邻居一起抹掉
    check(15, 0, 2, 2, 11u);
    check(0, 1, 3, 3, 8u);

    // 边界：区间内原本全是 1，必须被 M 完全覆盖
    check(static_cast<int>(0xFFFFFFFFu), 0, 8, 15, 0xFFFF00FFu);

    // 边界：区间顶到第 31 位，结果最高位为 1
    check(0, 15, 28, 31, 0xF0000000u);

    // 边界：k = 32（i = 0、j = 31），1 << 32 若写成 int 就是 UB
    check(static_cast<int>(0xFFFFFFFFu), 5, 0, 31, 5u);

    // 边界：只清最高两位，低位必须原样保留
    check(static_cast<int>(0xFFFFFFFFu), 0, 30, 31, 0x3FFFFFFFu);

    // 边界：区间在中间，左右两侧的高低位都不能被动
    check(static_cast<int>(0xFFFFFFFFu), 0, 4, 7, 0xFFFFFF0Fu);

    cout << "all tests passed" << endl;
    return 0;
}
