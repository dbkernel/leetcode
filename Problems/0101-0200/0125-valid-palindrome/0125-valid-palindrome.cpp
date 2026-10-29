/*
============================================================================
LeetCode 125. 验证回文串（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/valid-palindrome/
分类：字符串 · 双指针回文判断（筛选 / 原地）
============================================================================

如果在将所有大写字符转换为小写字符、并移除所有非字母和非数字字符之后，
短语正着读和反着读都一样。则可以认为该短语是一个 回文串 。

字母和数字都属于字母数字字符。

给你一个字符串 s，如果它是 回文串 ，返回 true ；否则，返回 false 。

示例 1：
输入: s = "A man, a plan, a canal: Panama"
输出：true
解释："amanaplanacanalpanama" 是回文串。

示例 2：
输入: s = "race a car"
输出：false
解释："raceacar" 不是回文串。

示例 3：
输入: s = " "
输出：true
解释：在移除非字母数字字符之后，s 是一个空字符串 "" 。
由于空字符串正着反着读都一样，所以是回文串。

提示：
1 <= s.length <= 2 * 10^5
s 仅由可打印的 ASCII 字符组成
*/
#include <string>
#include <vector>
#include <cctype>
#include <cassert>
#include <iostream>
using namespace std;

/*
解法一：筛选 + 判断（先造干净串，再双指针比对）【非最优】

为什么慢：需要在原串之外再造一个只含字母数字的串，最坏情况（原串本来就全是
字母数字）要额外占 O(|s|) 空间。时间上并不慢，慢的是空间。

换个角度：回文只关心「第 k 个有效字符」和「倒数第 k 个有效字符」是否相同。
先把有效字符按顺序抄到新串里，问题就退化成「判断一个普通字符串是否回文」，
可以用任何标准写法解决。

关键点：
  · 判定有效字符用 isalnum，比较前统一 tolower，两个动作都要做
  · isalnum / tolower 必须传 unsigned char 提升后的值 —— 直接传 char 在
    某些平台会把负值当成 EOF 之外的值，行为未定义

官方在这一步给了两种判断方式：反转后整体比较（sgood == reverse(sgood)），
或对 sgood 双指针。两者等价，这里取双指针，因为它和方法二的动作一致，
对比时只看「在哪里跳字符」这一处差别。

以示例 1（s = "A man, a plan, a canal: Panama"）为例：
  筛出 "amanaplanacanalpanama"，左右指针向中间比对，始终相等 → true

复杂度分析：
  时间 O(|s|)：筛选一遍，比对一遍
  空间 O(|s|)：最坏情况下新串与原串等长
*/
class Solution1
{
public:
    bool isPalindrome(string s)
    {
        // 只保留字母和数字，并统一转成小写
        string clean;
        clean.reserve(s.size());
        for (char ch : s)
        {
            unsigned char c = static_cast<unsigned char>(ch);
            if (isalnum(c))
            {
                clean.push_back(static_cast<char>(tolower(c)));
            }
        }

        int left = 0;
        int right = static_cast<int>(clean.size()) - 1;
        while (left < right)
        {
            if (clean[left] != clean[right])
            {
                return false;
            }
            ++left;
            --right;
        }
        return true;
    }
};

/*
解法二：在原字符串上直接判断（双指针 + 跳字符）【最优解】

官方两解中空间最优的一个，也是本题的最优解：时间 O(|s|) 已是下界，
空间可以压到 O(1)，不必为「干净串」额外付费。

为什么慢：解法一为了回避「跳过杂字符」的麻烦，先复制出一份干净串。
但复制本身不是必需 —— 指针完全可以自己去跳过那些字符。

换个角度：在「比较哪一对」这件事上，原串和干净串的顺序完全一致。于是让
左右指针从原串两端出发，各自向内跳到下一个字母或数字，再比较这两个字符。

关键点：
  · 两个内层 while 都必须带 left < right 守卫 —— 否则全杂字符的串会让
    指针越界，或越过中点后继续乱跳
  · 跳完后再判一次 left < right，只有仍在中点左侧才说明这是一对真实字符
  · 比较前统一大小写，不要只对大写或只对小写做转换

以示例 1（s = "A man, a plan, a canal: Panama"）为例：
  left 停在 'A'，right 停在 'a' → 都转小写后相等，各自内移
  中间跳过空格、逗号、冒号，逐对比对，始终相等 → true

复杂度分析：
  时间 O(|s|)：每个字符最多被两个指针各访问一次
  空间 O(1)：只用两个下标
*/
class Solution2
{
public:
    bool isPalindrome(string s)
    {
        int left = 0;
        int right = static_cast<int>(s.size()) - 1;

        while (left < right)
        {
            // 守卫不可省：全杂字符时能防止越界，也能防止越过中点
            while (left < right &&
                   !isalnum(static_cast<unsigned char>(s[left])))
            {
                ++left;
            }
            while (left < right &&
                   !isalnum(static_cast<unsigned char>(s[right])))
            {
                --right;
            }

            // 跳过之后可能已经交汇，此时没有可比的一对字符
            if (left < right)
            {
                char a = static_cast<char>(
                    tolower(static_cast<unsigned char>(s[left])));
                char b = static_cast<char>(
                    tolower(static_cast<unsigned char>(s[right])));
                if (a != b)
                {
                    return false;
                }
                ++left;
                --right;
            }
        }

        return true;
    }
};

namespace
{
// 同一个用例喂给两种解法：官方两解必须给出相同结论
void check(const string &s, bool want)
{
    Solution1 filter;
    Solution2 inplace;
    assert(filter.isPalindrome(s) == want);
    assert(inplace.isPalindrome(s) == want);
}
} // namespace

int main()
{
    // 示例 1：带空格与标点的经典回文
    check("A man, a plan, a canal: Panama", true);

    // 示例 2：内容相同但顺序不对，必须判 false
    check("race a car", false);

    // 示例 3：只有空白，筛完是空串，按约定算回文
    check(" ", true);

    // 边界：只有杂字符，两个指针一路跳到交汇，不能越界
    check(".,!?", true);

    // 边界：只有数字，字母数字都要算作有效字符
    check("0P", false);

    // 边界：单字符，left == right，直接判 true
    check("a", true);

    cout << "all tests passed" << endl;
    return 0;
}
