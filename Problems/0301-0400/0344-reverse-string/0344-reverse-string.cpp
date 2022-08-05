/*
344. 反转字符串

编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 s
的形式给出。 不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1)
的额外空间解决这一问题。

示例 1：
输入：s = ["h","e","l","l","o"]
输出：["o","l","l","e","h"]

示例 2：
输入：s = ["H","a","n","n","a","h"]
输出：["h","a","n","n","a","H"]
 
提示：
1 <= s.length <= 105
s[i] 都是 ASCII 码表中的可打印字符

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-string
*/

#include <vector>
using namespace std;

/*
  题解一：很容易想到，但不够优雅
  实际上与题解二在内存消耗、执行用时基本是一样的
*/
class SolutionEasy {
   public:
    void reverseString(vector<char>& s) {
        auto len = s.size();
        for (int i = 0; i < len / 2; ++i) {
            swap(s[i], s[len - 1 - i]);
        }
    }
};

/*
  题解二：很优雅
*/
class Solution {
   public:
    void reverseString(vector<char>& s) {
        auto len = s.size();
        for (int left = 0, right = len - 1; left < right; ++left, --right) {
            swap(s[left], s[right]);
        }
    }
};