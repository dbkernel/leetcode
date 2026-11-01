/*
============================================================================
LeetCode 5. 最长回文子串（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/longest-palindromic-substring/
分类：动态规划 · 回文串（区间 DP / 中心扩展 / Manacher）
============================================================================

给你一个字符串 s，找到 s 中最长的 回文 子串。

示例 1：
输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。

示例 2：
输入：s = "cbbd"
输出："bb"

提示：
1 <= s.length <= 1000
s 仅由数字和英文字母组成
*/
#include <string>
#include <vector>
#include <algorithm>
#include <cassert>
#include <iostream>
using namespace std;

/*
解法一：区间 DP（按长度从小到大填表）【非最优】

官方三解中的第一解。

为什么慢：判断 s[i..j] 是不是回文，最直白的办法是双指针往中间收。
单次判定 O(n)，n^2 / 2 个子串全判一遍就是 O(n^3)，1000 长度会超时。

换个角度：回文去掉首尾仍然是回文。于是

  s[i..j] 是回文  ⟺  s[i] == s[j] 且 s[i+1..j-1] 是回文

子问题比原问题短 2，只要把小长度的答案存下来，大长度就能 O(1) 查表。

关键点：
  · 长度 1 与长度 2 先写成初值 —— 它们没有「内层区间」可查，
    硬套递推式会读到 dp[i+1][j-1] 这种 i+1 > j-1 的非法格子
  · 外层必须按长度枚举 —— dp[i][j] 依赖 dp[i+1][j-1]，
    若按 i 正序、j 正序枚举，内层那格还没算出来，读到的是假

以示例 1（s = "babad"）为例：
  长度 1：五个单字符全为真
  长度 2："ba" "ab" "ba" "ad" 两端都不等，全为假
  长度 3：i = 0 的 "bab" 两端相等且内层 dp[1][1] 为真 → 长度 3、起点 0
          i = 1 的 "aba" 也为真，长度没变长，不覆盖答案
          i = 2 的 "bad" 两端 'b' 与 'd' 不等，为假
  长度 4、5：仍无回文，答案固定在 "bab"

复杂度分析：
  时间 O(n^2)：要填 n^2 / 2 个格子，每格 O(1)
  空间 O(n^2)：整张二维表都要留着，1000 长度约 1MB 位图
*/
class Solution1
{
public:
    // 返回任意一个最长回文子串；同长度的多个答案都算对
    string longestPalindrome(string s)
    {
        const int n = static_cast<int>(s.size());
        if (n < 2)
        {
            return s;
        }

        // dp[i][j] 表示 s[i..j] 是否回文，只填 i <= j 的上三角
        vector<vector<bool>> dp(n, vector<bool>(n, false));

        int bestStart = 0;
        int bestLen = 1;

        // 先写长度 1 与长度 2 的初值，它们不依赖任何内层区间
        for (int i = 0; i < n; ++i)
        {
            dp[i][i] = true;
        }
        for (int i = 0; i + 1 < n; ++i)
        {
            if (s[i] == s[i + 1])
            {
                dp[i][i + 1] = true;
                if (bestLen < 2)
                {
                    bestLen = 2;
                    bestStart = i;
                }
            }
        }

        // 长度从 3 起，保证 j - 1 >= i + 1 的内层区间已经算过
        for (int len = 3; len <= n; ++len)
        {
            for (int i = 0; i + len - 1 < n; ++i)
            {
                const int j = i + len - 1;

                // 两端相等只是必要条件，内层也得是回文（按长度序已在表里）
                if (s[i] == s[j] && dp[i + 1][j - 1])
                {
                    dp[i][j] = true;

                    // 长度严格变大才覆盖：同长度保留先出现的那个，结果更稳定
                    if (len > bestLen)
                    {
                        bestLen = len;
                        bestStart = i;
                    }
                }
            }
        }

        return s.substr(bestStart, bestLen);
    }
};

/*
解法二：中心扩展（2n-1 个中心各向外走一圈）【非最优】

官方三解中的第二解。它不需要那张 O(n^2) 的表，是空间上的赢家。

为什么慢：解法一存了 n^2 个「是否回文」的答案，但真正被用到的只有
「内层」那一条链。回文串唯一确定它自己的中心，反过来从不回文也推不出什么，
这张表有一大半空间是白占的。

换个角度：与其从两端往中间缩，不如从一个中心往两边长。每个中心最多走
O(n) 步，命中即停。长度为奇数时中心是一个字符（n 个），长度为偶数时中心
是相邻两字符之间的空隙（n - 1 个），一共 2n - 1 个。

关键点：
  · 奇数与偶数要合成同一轮 —— 用 c 遍历 0..2n-2，
    取 l = c / 2、r = l + c % 2：c 为偶数时 l == r（奇中心），
    c 为奇数时 r == l + 1（偶中心），不会漏掉任何一种
  · 循环停下的位置决定长度 —— 退出时 l、r 指向的就是第一对不相等的字符
    （或越界），所以真实长度是 r - l - 1、起点是 l + 1，
    不要写成 r - l

以示例 2（s = "cbbd"）为例，只看 c = 3（偶中心，l = 1、r = 2）：
  s[1] == s[2] == 'b' → 收缩到 l = 0、r = 3
  s[0] 是 'c'、s[3] 是 'd'，不等，停
  长度 3 - 0 - 1 = 2，起点 0 + 1 = 1，得到 "bb"

复杂度分析：
  时间 O(n^2)：最坏是 "aaaa...a"，每个中心都要走到边界，合计 n^2 / 4 步
  空间 O(1)：只有左右指针和答案的两个下标
  注：空间比解法一省，但时间仍是 O(n^2)，所以不是本题渐进最优
*/
class Solution2
{
public:
    // 返回任意一个最长回文子串；同长度的多个答案都算对
    string longestPalindrome(string s)
    {
        const int n = static_cast<int>(s.size());
        if (n < 2)
        {
            return s;
        }

        int bestStart = 0;
        int bestLen = 1;

        // c 同时编码中心与奇偶：2n - 1 个中心一轮扫完
        for (int c = 0; c < 2 * n - 1; ++c)
        {
            int l = c / 2;
            int r = l + c % 2;

            // 越界即停，所以循环条件必须先判 l >= 0 再判 r < n
            while (l >= 0 && r < n && s[l] == s[r])
            {
                --l;
                ++r;
            }

            // 停在第一对不相等的字符上，l 多退了一格、r 多进了一格
            const int len = r - l - 1;
            if (len > bestLen)
            {
                bestLen = len;
                bestStart = l + 1;
            }
        }

        return s.substr(bestStart, bestLen);
    }
};

/*
解法三：Manacher（马拉车，把中心扩展的重复计算摊掉）【最优解】

官方三解中的第三解，也是本题唯一的线性解法。

为什么慢：解法二每个中心都从零开始向外扩，而在 "aaaa...a" 这种串上，
第 i 个中心走过的路，第 i+1 个中心基本要重走一遍，白白多出 O(n) 倍。

换个角度：回文串左右对称。若回文的中心是 c、右边界是 right，
那么区间里任意位置 i 关于 c 的镜像 2c - i 处的半径，可以直接搬过来用——
能搬多少取决于镜像半径有没有顶到 right，这正是 min(p[mirror], right - i)。

关键点：先插分隔符，把奇偶两种中心统一成一种。
  · 在每两个字符之间插入 '#'，得到长度 2n + 1 的 t，那么 t 上任意位置的
    回文在 s 里都对应一个回文，奇数长度和偶数长度被抹平成同一套代码
  · p[i] 只记「纯半径」，即 t[i - p[i] .. i + p[i]] 是回文，
    于是 s 里的长度恰好等于 p[i]，起点是 (i - p[i]) / 2
  · 首尾要放两个哨兵（'^' 与 '$'），让 t[i - p[i] - 1] 与
    t[i + p[i] + 1] 永远不越界，循环里就不用写边界判断

以示例 1（s = "babad"）为例，t = ^ # b # a # b # a # d # $：
  i = 4 是 'a'：向右依次扩过 '#' 'b' '#'，第 4 圈撞上哨兵 '^' 停下，
    得 p[4] = 3，即 "b#a#b"；s 里长度 3、起点 (4 - 3) / 2 = 0，得到 "bab"
  i = 6 是 'b'：落在 right = 7 之内，镜像 2 * 4 - 6 = 2 只肯借出 1，
    剩下两圈仍得自己扩，最终同样是 p = 3、得到同长的 "aba"，不覆盖答案
  i = 8 是 'a'：镜像 2 * 6 - 8 = 4 的半径是 3，但 right - i = 1，
    只借到 1；再往外 t[6] = 'b' 与 t[10] = 'd' 不等，正好停在 O(1) 步

复杂度分析：
  时间 O(n)：right 全程只向右移动，while 里每成功一次 right 就推进一格
  空间 O(n)：p 数组与 t 各 O(n)
*/
class Solution3
{
public:
    // 返回任意一个最长回文子串；同长度的多个答案都算对
    string longestPalindrome(string s)
    {
        if (s.size() < 2)
        {
            return s;
        }

        // 首尾哨兵用 ^ 与 $：输入只有数字和字母，这两个字符必然不参与匹配
        string t = "^#";
        for (char ch : s)
        {
            t += ch;
            t += '#';
        }
        t += '$';

        const int m = static_cast<int>(t.size());
        vector<int> p(m, 0);

        // center 是当前最靠右的回文的中心，right 是它的右边界（不含）
        int center = 0;
        int right = 0;
        int bestLen = 1;
        int bestStart = 0;

        for (int i = 1; i < m - 1; ++i)
        {
            // 镜像半径顶到 right 时只能借到 right - i，剩下的靠下一行自己扩
            if (i < right)
            {
                p[i] = min(p[2 * center - i], right - i);
            }

            // 从已知半径继续外扩；哨兵保证不会越界，while 必然能停
            while (t[i - p[i] - 1] == t[i + p[i] + 1])
            {
                ++p[i];
            }

            // 右边界被推远了才更新中心，否则镜像关系会指错
            if (i + p[i] > right)
            {
                center = i;
                right = i + p[i];
            }

            // 半径就是 s 里的长度；同长不覆盖，保留先出现的那个
            if (p[i] > bestLen)
            {
                bestLen = p[i];
                bestStart = (i - p[i]) / 2;
            }
        }

        return s.substr(bestStart, bestLen);
    }
};

namespace
{
// 判断 t 本身是不是回文：只比前半段与后半段的反序，奇长度中间那个不用管
bool isPalindrome(const string &t)
{
    return equal(t.begin(), t.begin() + t.size() / 2, t.rbegin());
}

// 题目允许多个同长答案，所以只校验三件事：长度对、真取自 s、确实是回文
void check(const string &s, size_t wantLen)
{
    Solution1 dp;
    Solution2 expand;
    Solution3 manacher;

    const vector<string> got = {dp.longestPalindrome(s),
                                expand.longestPalindrome(s),
                                manacher.longestPalindrome(s)};

    for (const string &t : got)
    {
        // 长度是「最长」唯一可比的指标，"bab" 与 "aba" 都算对
        assert(t.size() == wantLen);
        // 必须是 s 的连续子串，防止实现自己拼一个同长回文蒙混过关
        assert(s.find(t) != string::npos);
        assert(isPalindrome(t));
    }
}

} // namespace

int main()
{
    // 示例 1：奇数长度的答案，官方注明 "aba" 也对
    check("babad", 3);

    // 示例 2：偶数长度的答案 —— 只处理奇中心的写法会在这里挂
    check("cbbd", 2);

    // 边界：单字符，三种解法都该原样返回
    check("a", 1);

    // 边界：没有长度 >= 2 的回文，答案退化成任意一个单字符
    check("ac", 1);

    // 关键用例：全同字符 —— 顺带暴露解法一「按 i、j 正序枚举」的错误顺序
    check("aaaa", 4);

    // 关键用例：交替串，最长回文跨越多个中心，考的是 p 数组的镜像复用
    check("abababab", 7);

    // 常规用例：最长回文落在中间，两端都要被切掉
    check("forgeeksskeegfor", 10);

    // 边界：取到约束上限 n = 1000，验证三者都不会退化成 O(n^2) 以上的常数
    check(string(1000, 'a'), 1000);

    cout << "all tests passed" << endl;
    return 0;
}
