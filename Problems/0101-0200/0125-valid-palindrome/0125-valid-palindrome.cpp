/*
125. 验证回文串

给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。
说明：本题中，我们将空字符串定义为有效的回文串。

示例 1:
输入: "A man, a plan, a canal: Panama"
输出: true
解释："amanaplanacanalpanama" 是回文串

示例 2:
输入: "race a car"
输出: false
解释："raceacar" 不是回文串
 
提示：
1 <= s.length <= 2 * 10^5
字符串 s 由 ASCII 字符组成

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/valid-palindrome
*/

#include <cassert>
#include <iostream>
#include <string>
using namespace std;

/*
LeetCode 官方题解有其他 先筛选再判断 的方式，不如直接在原字符串判断。

方法一：在原字符串上直接判断

我们可以对方法一中第二种判断回文串的方法进行优化，就可以得到只使用 O(1)
空间的算法。

我们直接在原字符串 s 上使用双指针。
在移动任意一个指针时，需要不断地向另一指针的方向移动，直到遇到一个字母或数字字符，或者两指针重合为止。
也就是说，我们每次将指针移到下一个字母字符或数字字符，再判断这两个指针指向的字符是否相同。

复杂度分析
- 时间复杂度：O(|s|)，其中 |s| 是字符串 s 的长度。
- 空间复杂度：O(1)。
*/
class Solution {
   public:
    bool isPalindrome(string s) {
        int left = 0, right = s.size() - 1;
        while (left < right) {
            // 比如字符串为 ".,"，则从左到右判断后，left 就会等于 right 了
            while (left < right && !isalnum(s[left]))
                ++left;
            while (left < right && !isalnum(s[right]))
                --right;
            if (left < right && tolower(s[left++]) != tolower(s[right--]))
                return false;
        }
        return true;
    }
};

int main() {
    Solution s;
    assert(s.isPalindrome("A man, a plan, a canal: Panama") == true);
    assert(s.isPalindrome("race a car") == false);
    assert(s.isPalindrome(".,") == true);
    return 0;
}
