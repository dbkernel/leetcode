/*
剑指 Offer 05. 替换空格

请实现一个函数，把字符串 s 中的每个空格替换成"%20"。

示例 1：
输入：s = "We are happy."
输出："We%20are%20happy."

限制：
0 <= s 的长度 <= 10000

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/ti-huan-kong-ge-lcof
*/

#include <iostream>
#include <cassert>

using namespace std;

/*
方法一：原地替换

1. 查找空格数。
2. 扩充原字符串。
3. 替换。

复杂性分析
- 时间复杂度：O(n)。遍历字符串 s 一遍。
- 空间复杂度：O(1)。由于是原地扩展 s 长度，因此使用 O(1) 额外空间。
*/
class Solution {
public:
    string replaceSpace(string s) {
        int count = 0, len = s.length();

        // 查找空格数量
        for(auto c : s)
            if (c == ' ')
                ++count;
        if (count == 0) // 不存在空格，无需替换
            return s;

        // 扩充字符串长度
        s.resize(len + 2 * count);

        // 从右向左遍历，如果非空格，则往后挪；反之，替换空格为 %20
        for(int left = len - 1, right = s.size() - 1; left >= 0; --left, --right) {
            if (s[left] == ' ') {
                s[right] = '0';
                s[--right] = '2';
                s[--right] = '%';
            } else {
                s[right] = s[left];
            }
        }

        return s;
    }
};

int main() {
    Solution s;
    assert(s.replaceSpace("Wearehappy.") == "Wearehappy.");
    assert(s.replaceSpace("We are happy.") == "We%20are%20happy.");
    return 0;
}