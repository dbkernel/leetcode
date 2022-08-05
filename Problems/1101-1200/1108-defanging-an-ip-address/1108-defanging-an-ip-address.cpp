/*
1108. IP 地址无效化

给你一个有效的 IPv4 地址 address，返回这个 IP 地址的无效化版本。
所谓无效化 IP 地址，其实就是用 "[.]" 代替了每个 "."。

示例 1：
输入：address = "1.1.1.1"
输出："1[.]1[.]1[.]1"

示例 2：
输入：address = "255.100.50.0"
输出："255[.]100[.]50[.]0"
 
提示：
给出的 address 是一个有效的 IPv4 地址

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/defanging-an-ip-address/
*/

#include <string>
using namespace std;

/*
复杂度分析：
- 时间复杂度：O(n)，其中 n 为字符串 s 的长度。需要遍历一遍字符串即可。
- 空间复杂度：O(1)。除返回值外，不需要额外的存储空间。
*/
class Solution {
   public:
    string defangIPaddr(string address) {
        string res;
        for (auto& ch : address) {
            if (ch == '.')
                res.append("[.]");
            else
                res.push_back(ch);
        }
        return res;
    }
};
