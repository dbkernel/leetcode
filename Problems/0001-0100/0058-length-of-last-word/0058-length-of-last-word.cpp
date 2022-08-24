/*
58. 最后一个单词的长度

给你一个字符串 s，由若干单词组成，单词前后用一些空格字符隔开。返回字符串中
最后一个 单词的长度。 单词 是指仅由字母组成、不包含任何空格字符的最大子字符串。

示例 1：
输入：s = "Hello World"
输出：5
解释：最后一个单词是“World”，长度为5。

示例 2：
输入：s = "   fly me   to   the moon  "
输出：4
解释：最后一个单词是“moon”，长度为4。

示例 3：
输入：s = "luffy is still joyboy"
输出：6
解释：最后一个单词是长度为6的“joyboy”。

提示：
1 <= s.length <= 10^4
s 仅有英文字母和空格 ' ' 组成
s 中至少存在一个单词

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/length-of-last-word
*/

#include <cassert>
#include <iostream>
#include <string>

using namespace std;

/*
方法一：反向遍历

题目要求得到字符串中最后一个单词的长度，可以反向遍历字符串，寻找最后一个单词并计算其长度。

由于字符串中至少存在一个单词，因此字符串中一定有字母。首先找到字符串中的最后一个字母，
该字母即为最后一个单词的最后一个字母。

从最后一个字母开始继续反向遍历字符串，直到遇到空格或者到达字符串的起始位置。
遍历到的每个字母都是最后一个单词中的字母，因此遍历到的字母数量即为最后一个单词的长度。

复杂度分析
- 时间复杂度：O(n)，其中 n 是字符串的长度。最多需要反向遍历字符串一次。
- 空间复杂度：O(1)。
*/
class Solution {
   public:
    int lengthOfLastWord(string s) {
        // 反向遍历找到右边界，也就是最右边的一个字母
        int right = s.length() - 1;
        while(s[right] == ' ')
            --right;

        // 再继续反向遍历找到左边界的前一个元素，如果是 "abc"，则 left=-1, right=2
        int left = right;
        while(left >= 0 && isalnum(s[left]))
            --left;

        return right - left;
    }
};

int main() {
    Solution s;
    assert(s.lengthOfLastWord("a") == 1);
    assert(s.lengthOfLastWord("Hello World") == 5);
    assert(s.lengthOfLastWord("   fly me   to   the moon  ") == 4);
    assert(s.lengthOfLastWord("luffy is still joyboy") == 6);
    return 0;
}