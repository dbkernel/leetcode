/*
============================================================================
LCR 167 / 剑指 Offer 48. 招式拆解 I（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zui-chang-bu-han-zhong-fu-zi-fu-de-zi-zi-fu-chuan-lcof/
分类：滑动窗口 · 动态规划（最长无重复子串）
说明：本题即《剑指 Offer（第 2 版）》第 48 题「最长不含重复字符的子字符
      串」，原题面是「请从字符串中找出一个最长的不包含重复字符的子字符
      串，计算该最长子字符串的长度」。力扣现行题面改名为「招式拆解 I」，
      参数名改为 arr，签名由 lengthOfLongestSubstring(s) 改为
      dismantlingAction(arr)。

某套连招动作记作序列 arr，其中 arr[i] 为第 i 个招式的名字。请返回 arr 中
最多可以出连续不重复的多少个招式。

示例 1：
输入：arr = "dbascDdad"
输出：6
解释：因为连续且最长的招式序列是 "dbascD" 或 "bascDd"，所以其长度为 6。

示例 2：
输入：arr = "KKK"
输出：1
解释：因为无重复字符的最长子串是 "K"，所以其长度为 1。

示例 3：
输入：arr = "pwwkew"
输出：3
解释：因为连续且最长的招式序列是 "wke"，所以其长度为 3。
请注意区分 子串 与 子序列 的概念：你的答案必须是 连续招式 的长度，也就是
子串。而 "pwke" 是一个非连续的 子序列，不是 子串。

提示：
- 0 <= arr.length <= 40000
- arr 由英文字母、数字、符号和空格组成。

注意：本题与主站 3 题相同：
https://leetcode.cn/problems/longest-substring-without-repeating-characters/
*/
#include <algorithm>
#include <cassert>
#include <cstddef>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

/*
解法一：动态规划 + 上次出现位置【最优解】

为什么慢：枚举所有子串再逐一检查是否有重复字符，子串数是 O(n^2)，每个
再花 O(len) 检查，最坏 O(n^3)。就算用"固定右端点、向左扩到第一个重复为
止"的办法，左端点也可能被反复回退，n = 40000 时不可接受。

换个角度：只关心"以 i 结尾的最长无重复子串有多长"，记作 dp[i]。它只取
决于 dp[i-1] 和 arr[i] 上一次出现的位置 —— 若上次的位置在 dp[i-1] 这段
有效区间之外，整段都能接上；若在区间之内，长度就被截断到两次出现的距离。
整个状态转移只往左看一步，一维就够。

关键点：
  · last[c] 的初值必须是 -1 而不是 0：下标 0 的字符出现时，距离
    d = i - (-1) 才是对的
  · d = i - last[c]；d <= dp[i-1] 表示旧位置还压在有效区间里，dp[i] 只能
    等于 d（区间左界被顶到旧位置 +1）；d > dp[i-1] 时旧位置已在区间外，
    dp[i] = dp[i-1] + 1
  · 决定分支的是 d 与 dp[i-1] 的比较，不是"字符是否出现过"：出现过但
    离得够远，仍然可以整段接上
  · 答案要边走边取最大，最长的一段未必以最后一个字符结尾
  · 转移只用到 dp[i-1]，用滚动变量即可，不必真的开 dp 数组

手推一遍：以示例 1（arr = "dbascDdad"）为例，dp 依次为：
  d 1、b 2、a 3、s 4、c 5、D 6（大小写不同，视为不同字符）
  第 7 位 d：last['d'] = 0，d = 6 <= 6 → dp = 6
  第 8 位 a：last['a'] = 2，d = 5 <= 6 → dp = 5
  第 9 位 d：last['d'] = 6，d = 2 <= 5 → dp = 2
  最大值为 6 ✔（答案在第 6、7 位都取到，不在末尾）

复杂度分析：
  时间 O(n)：每个字符只做常数次查表和比较
  空间 O(Σ)：一张按字符集大小（这里取 ASCII 的 256）开的整数表，
    与字符串长度无关
*/

class Solution1
{
public:
    int dismantlingAction(string arr)
    {
        // last[c]：字符 c 上一次出现的下标，-1 表示从未出现
        vector<int> last(256, -1);
        int cur = 0;   // 以当前字符结尾的最长无重复子串长度
        int best = 0;

        for (int i = 0; i < static_cast<int>(arr.size()); ++i)
        {
            unsigned char c = static_cast<unsigned char>(arr[i]);
            int prev = last[c];
            // 距离够远说明旧位置已被甩出有效区间，整段都能接上
            if (prev >= 0 && i - prev <= cur) cur = i - prev;
            else ++cur;

            last[c] = i;
            best = max(best, cur);
        }
        return best;
    }
};

/*
解法二：滑动窗口 + 字符集合【最优解，与解法一并列】

为什么慢：解法一已经是最优时间，这里换一条思路只是为了对照。若窗口的
左界每次都从 0 重新数，或者用"枚举左端点、向右检查"的写法，右端点会大
量回退，退化成 O(n^2)。

换个角度：维护一个"内部没有重复字符"的窗口 [left, right]。right 右移一
格时，若新字符已在窗口里，只需把 left 右移到那个字符旧位置之后 —— 窗口
仍然保持无重复。left 与 right 都只前进，每个字符入窗、出窗各一次。

关键点：
  · 收缩必须用 while 而不是 if：新字符的旧位置可能远在 left 右边，
    只移一格并没有把它挤出去
  · 窗口里每个字符至多一个，所以布尔表就够，不必用计数表
  · 先收缩再标记入窗：顺序反了会让"刚加进去的字符"把自己判成重复
  · right - left + 1 要在入窗之后取，答案才包含当前字符
  · 与解法一的取舍：时间同为 O(n)，空间同为按字符集大小的常数表
    （这里 256 个字节 vs 256 个整数），渐近上并列；DP 不需要维护窗口
    集合、天然支持流式输入，滑窗的状态更直白、左界只进不退更好证

手推一遍：以示例 3（arr = "pwwkew"）为例：
  p：窗口 [p]，长 1
  w：窗口 [pw]，长 2
  w：已在窗口内 → left 移到第二个 w，窗口 [w]，长 1
  k：窗口 [wk]，长 2
  e：窗口 [wke]，长 3
  w：已在窗口内 → left 移到 e，窗口 [ew]，长 2
  最大值 3 ✔

复杂度分析：
  时间 O(n)：right 走 n 步，left 累计也只走 n 步，均摊每个字符进出一次
  空间 O(Σ)：一张 256 个布尔值的表，与字符串长度无关
*/

class Solution2
{
public:
    int dismantlingAction(string arr)
    {
        vector<bool> inWin(256, false);
        int left = 0;
        int best = 0;

        for (int right = 0; right < static_cast<int>(arr.size()); ++right)
        {
            unsigned char c = static_cast<unsigned char>(arr[right]);
            // 旧位置还留在窗口里，左界必须一路推到它之后
            while (inWin[c])
            {
                inWin[static_cast<unsigned char>(arr[left])] = false;
                ++left;
            }
            inWin[c] = true;
            best = max(best, right - left + 1);
        }
        return best;
    }
};

namespace
{
void check(const string &arr, int want)
{
    Solution1 s1;
    Solution2 s2;
    assert(s1.dismantlingAction(arr) == want);
    assert(s2.dismantlingAction(arr) == want);
}

} // namespace

int main()
{
    // 示例 1：大小写算不同字符，答案取到两次
    check("dbascDdad", 6);

    // 示例 2：全部相同，答案是 1
    check("KKK", 1);

    // 示例 3：子串不能跨过重复的 w
    check("pwwkew", 3);

    // 原题经典用例：最长段在中间，不在末尾
    check("abcabcbb", 3);
    check("bbbbb", 1);

    // 边界：空串，长度为 0
    check("", 0);

    // 边界：单个空格，符号与空格也计入字符
    check(" ", 1);

    // 反直觉：a 的旧位置被 b 挡住后，左界要一次跳两格
    check("abba", 2);

    // 边界：40000 个相同字符，答案 1，验证没有退化成 O(n^2)
    check(string(40000, 'a'), 1);

    cout << "all tests passed" << endl;
    return 0;
}
