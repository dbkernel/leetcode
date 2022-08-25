/*
剑指 Offer 58 - II. 左旋转字符串

字符串的左旋转操作是把字符串前面的若干个字符转移到字符串的尾部。请定义一个函数实现字符串左旋转操作的功能。
比如，输入字符串"abcdefg"和数字2，该函数将返回左旋转两位得到的结果"cdefgab"。

示例 1：
输入: s = "abcdefg", k = 2
输出: "cdefgab"

示例 2：
输入: s = "lrloseumgh", k = 6
输出: "umghlrlose"

限制：
1 <= k < s.length <= 10000

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof
*/

#include <cassert>
#include <iostream>

using namespace std;

/*
方法一：字符切分与拼接

解题思路： 获取str[n:] 和 str[:n]子串，目标串target = str[:n] + str[n:];

复杂度分析：
- 时间复杂度： O(N)
- 空间复杂度： O(N)
*/
class Solution1 {
public:
    string reverseLeftWords(string s, int n) {
        if (s.size() == 0)
            return s;
        string str = s.substr(0, n); // 左闭右开
        string res = s.substr(n, s.length() - n) + str;
        return res;
    }
};

/*
方法二：不使用 substr 函数，字符串遍历

解题思路：利用取余操作得到目标串 target 应该是 str 中哪个字符。

主要思路：
1. 首先，n 如果是 str 长度的倍数，那么最终目标串 target 是和 str 一样的，令 left = n % len。
2. 如果 left = 0 ,那么直接返回原串，否则进行字符串遍历。
3. target 每个位置的字符为 str[(left + i) % str.size()] , i代表当前位置。

复杂度分析：
- 时间复杂度： O(N)
- 空间复杂度： O(N)
*/
class Solution2 {
public:
    string moveleft (string s, int r){
        string target = "";
        for(int i = 0; i < s.size(); i++)
            target += s[(i + r) % s.size()];
        return target;
    }

    string reverseLeftWords(string s, int n) {
        if( s.size() == 0)
            return s;

        /* 处理长度超过 str 本身长度的情况 */
        int left = n % s.size();
        if(left == 0)
            return s;
        else
            return moveleft(s, left);
    }
};

/*
方法三：局部翻转+整体翻转

解题思路：首先翻转 str[:n] 子串，其次翻转 str[n:] 子串，最后翻转 str.

主要思路：
- 反转区间为前 n 的子串
- 反转区间为 n 到末尾的子串
- 反转整个字符串

复杂度分析：
- 时间复杂度： O(N)
- 空间复杂度： O(N)
*/
class Solution3 {
public:
    string reverseLeftWords(string s, int n) {
        // 举例：s = "abcdefg", k = 2

        // 反转n前面的字符串，则为 "bacdefg"
        reverse(s.begin(), s.begin() + n);
        // 反转n后面的字符串，则为 "bagfedc"
        reverse(s.begin() + n, s.end());
        // 反转整个字符串，则为 "cdefgab"
        reverse(s.begin(), s.end());
        return s;
    }
};

int main() {
    Solution1 s1;
    assert(s1.reverseLeftWords("abcdefg", 2) == "cdefgab");
    assert(s1.reverseLeftWords("lrloseumgh", 6) == "umghlrlose");

    Solution1 s2;
    assert(s2.reverseLeftWords("abcdefg", 2) == "cdefgab");
    assert(s2.reverseLeftWords("lrloseumgh", 6) == "umghlrlose");

    Solution1 s3;
    assert(s3.reverseLeftWords("abcdefg", 2) == "cdefgab");
    assert(s3.reverseLeftWords("lrloseumgh", 6) == "umghlrlose");

    return 0;
}