/*
242. 有效的字母异位词

给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。

示例 1:
输入: s = "anagram", t = "nagaram"
输出: true

示例 2:
输入: s = "rat", t = "car"
输出: false

提示:
1 <= s.length, t.length <= 5 * 104
s 和 t 仅包含小写字母
 
进阶: 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/valid-anagram

方法一：排序（太普通。。。）

t 是 s 的异位词等价于「两个字符串排序后相等」。因此我们可以对字符串 s 和 t 分别排序，
看排序后的字符串是否相等即可判断。此外，如果 s 和 t 的长度不同，t 必然不是 s 的异位词。

复杂度分析：
- 时间复杂度：O(nlogn)，其中 n 为 s 的长度。排序的时间复杂度为 O(nlogn)，
- 比较两个字符串是否相等时间复杂度为 O(n)，因此总体时间复杂度为 O(nlogn+n)=O(nlogn)。

空间复杂度：O(logn)。排序需要 O(logn) 的空间复杂度。


方法二：哈希表

从另一个角度考虑，t 是 s 的异位词等价于「两个字符串中字符出现的种类和次数均相等」。
由于字符串只包含 26 个小写字母，因此我们可以维护一个长度为 26 的频次数组 table，
先遍历记录字符串 s 中字符出现的频次，然后遍历字符串 t，减去 table 中对应的频次，
如果出现 table[i]<0，则说明 t 包含一个不在 s 中的额外字符，返回 false 即可。

复杂度分析：
- 时间复杂度：O(n)，其中 n 为 s 的长度。
- 空间复杂度：O(S)，其中 S 为字符集大小，此处 S=26。
*/
#include <iostream>
#include <cassert>
#include <vector>
using namespace std;

class Solution
{
public:
    bool isAnagram(string s, string t)
    {
        if (s.length() != t.length())
            return false;
        vector<int> table(26, 0);
        for (auto &ch : s)
        {
            table[ch - 'a']++;
        }
        for (auto &ch : t)
        {
            table[ch - 'a']--;
            if (table[ch - 'a'] < 0)
                return false;
        }
        return true;
    }
};

int main()
{
    Solution s;
    assert(s.isAnagram("raftraft", "tfartfar"));
    assert(!s.isAnagram("raft", "raftt"));
}