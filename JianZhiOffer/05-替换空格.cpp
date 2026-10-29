/*
============================================================================
LCR 122 / 剑指 Offer 05. 路径加密（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/ti-huan-kong-ge-lcof/
分类：字符串 · 逐字符替换

说明：本题原名「替换空格」（把空格换成 %20），leetcode.cn 现已改写成
「路径加密」：把分隔符 "." 换成空格，函数名为 pathEncryption。
解题动作完全一致 —— 遍历一遍，遇到目标字符就写替换值。
============================================================================

假定一段路径记作字符串 path，其中以 "." 作为分隔符。现需将路径加密，加密
方法为将 path 中的分隔符替换为空格 " "，请返回加密后的字符串。

示例 1：
输入：path = "a.aef.qerf.bb"
输出："a aef qerf bb"

限制：
0 <= path.length <= 10000
*/
#include <cassert>
#include <iostream>
#include <string>
using namespace std;

/*
解法：遍历添加（遇到 '.' 写空格，其余原样写入）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每个字符看一次，
时间 O(N) 已是下界；输出串本身就要 O(N) 空间，无法再省。

为什么慢：想「在原串上改」是不行的 —— 很多语言的字符串不可变（Python、Go、
Rust 的 str 都是），C++ 虽然可变但逐位替换和新建一样要扫一遍，还更容易写错
下标。想「split 再 join」则要先切出单词数组，多占一份内存。

换个角度：这就是一次逐字符映射 —— '"' 之外的字符原样搬过去，'.' 搬成空格。
开一个能放 N 个字符的缓冲区，边扫边写即可，不需要切分也没有额外结构。

关键点：
  · 判断条件是「等于 '.'」而不是「不等于 '.'」，写反就把所有其他字符都
    变成空格了
  · 空串要能正常返回空串，不能假定至少有一个字符
  · 缓冲区预留 N 个字符，避免边 push 边扩容

以示例 1（path = "a.aef.qerf.bb"）为例：
  'a' 原样 → "a"
  '.' 换成空格 → "a "
  'a'、'e'、'f' 原样 → "a aef"
  '.' 换成空格 → "a aef "
  继续扫完 → "a aef qerf bb"

复杂度分析：
  时间 O(N)：每个字符处理一次
  空间 O(N)：结果字符串本身
*/
class Solution
{
public:
    string pathEncryption(string path)
    {
        string ans;
        ans.reserve(path.size());

        for (char ch : path)
        {
            if (ch == '.')
            {
                ans.push_back(' ');
            }
            else
            {
                ans.push_back(ch);
            }
        }

        return ans;
    }
};

namespace
{
// 一个用例 = 输入 + 期望输出
void check(const string &path, const string &want)
{
    Solution solution;
    assert(solution.pathEncryption(path) == want);
}
} // namespace

int main()
{
    // 示例 1：多个分隔符，验证每个都被替换
    check("a.aef.qerf.bb", "a aef qerf bb");

    // 边界：空串，必须能返回空串
    check("", "");

    // 边界：没有分隔符，结果应与输入完全相同
    check("abc", "abc");

    // 边界：全是分隔符，结果应全是空格
    check("...", "   ");

    // 边界：分隔符在首尾，不能被漏掉或多加
    check(".a.", " a ");

    cout << "all tests passed" << endl;
    return 0;
}
