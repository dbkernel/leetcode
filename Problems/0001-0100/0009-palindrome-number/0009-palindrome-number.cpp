/*
============================================================================
LeetCode 9. 回文数（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/palindrome-number/
分类：数学 · 反转一半数字
============================================================================

给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。

回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

例如，121 是回文，而 123 不是。

示例 1：
输入：x = 121
输出：true

示例 2：
输入：x = -121
输出：false
解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。
因此它不是一个回文数。

示例 3：
输入：x = 10
输出：false
解释：从右向左读, 为 01 。因此它不是一个回文数。

提示：
-2^31 <= x <= 2^31 - 1

进阶：你能不将整数转为字符串来解决这个问题吗？
*/
#include <climits>
#include <cassert>
#include <iostream>
using namespace std;

/*
解法：反转一半数字（边除边拼，与原数在中点相遇）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：只需处理一半的位数，
时间 O(log n) 而非 O(n)，空间 O(1)。

为什么慢：把整数整体反转再和原数比较，看似简单，但反转结果可能超过 int
范围 —— 例如 2147483647 反转后是 7463847412，早已溢出，需要额外的 long
或溢出判断，题目还特意留了「不转字符串」的进阶要求。

换个角度：回文只需要「后半部分反转后等于前半部分」，不必反转全部位数。
于是边取余边拼后半段，当「剩下的前半段 <= 已拼好的后半段」时，说明刚好
走到中点，可以停下来比较。

关键点：
  · 必须先排除负数与「末位为 0 但本身不是 0」的数 —— 后者反转后必然
    以 0 开头，不可能等于原数（10、100 都是反例）
  · 循环条件是 x > reverted，不是 x >= reverted —— 用严格大于来判断
    「前半段更短」，中途停下才不会把中间那位多算一次
  · 位数奇偶要分开收尾：偶数位比 x == reverted，奇数位比
    x == reverted / 10，把中间那一位去掉再比

以示例 1（x = 121）为例：
  x = 121 > 0：reverted = 1，x = 12
  x = 12 > 1：reverted = 12，x = 1
  x = 1 不再大于 12，停止
  x == 1，reverted / 10 == 1 → 相等，返回 true

复杂度分析：
  时间 O(log n)：每轮把 x 除以 10，共处理约 log10(n)/2 位
  空间 O(1)：只用 x 与 reverted 两个整数
*/
class Solution
{
public:
    bool isPalindrome(int x)
    {
        // 负数带 '-' 号，正读反读必然不同；末位是 0 的非零数反过来会丢掉前导位
        // 只有 0 本身满足「末位是 0 又是回文」
        if (x < 0 || (x % 10 == 0 && x != 0))
        {
            return false;
        }

        int reverted = 0;

        // 严格大于：保证 reverted 至少吃到后半段，且不会越过中点
        while (x > reverted)
        {
            reverted = reverted * 10 + x % 10;
            x /= 10;
        }

        // 偶数位：两半完全相等；奇数位：reverted 多吃了中间一位，除掉再比
        return x == reverted || x == reverted / 10;
    }
};

namespace
{
// 一个用例 = 输入 + 期望输出
void check(int x, bool want)
{
    Solution s;
    assert(s.isPalindrome(x) == want);
}
} // namespace

int main()
{
    // 示例 1：奇数位回文，验证中点那一位被正确忽略
    check(121, true);

    // 示例 2：负数一律不是回文
    check(-121, false);

    // 示例 3：末位是 0 的非零数，反转后前导 0 丢失
    check(10, false);

    // 边界：0 是唯一「末位为 0 且是回文」的数，不能被上面的规则误杀
    check(0, true);

    // 边界：个位数全是回文
    check(7, true);

    // 边界：偶数位回文，走 x == reverted 分支
    check(1221, true);

    // 边界：int 上限附近，验证反转过程中不会溢出
    check(2147483647, false);

    // 边界：int 下限，先被负数规则拦下
    check(INT_MIN, false);

    cout << "all tests passed" << endl;
    return 0;
}
