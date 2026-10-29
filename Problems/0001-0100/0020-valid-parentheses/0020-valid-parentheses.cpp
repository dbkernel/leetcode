/*
============================================================================
LeetCode 20. 有效的括号（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/valid-parentheses/
分类：栈 · 括号匹配
============================================================================

给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。

有效字符串需满足：

左括号必须用相同类型的右括号闭合。
左括号必须以正确的顺序闭合。
每个右括号都有一个对应的相同类型的左括号。

示例 1：
输入：s = "()"
输出：true

示例 2：
输入：s = "()[]{}"
输出：true

示例 3：
输入：s = "(]"
输出：false

示例 4：
输入：s = "([])"
输出：true

示例 5：
输入：s = "([)]"
输出：false

提示：
1 <= s.length <= 10^4
s 仅由括号 '()[]{}' 组成
*/
#include <cassert>
#include <iostream>
#include <string>
#include <unordered_map>
using namespace std;

/*
解法：栈（左括号入栈，右括号与栈顶比对）【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每个字符进出栈各一次，
时间 O(n) 已是下界，空间 O(n) 也是必须的（最坏情况全是左括号）。

为什么慢：想不用栈就得反复扫描找「最近一个未匹配的左括号」，每对括号
O(n) 次回溯，整体退化成 O(n^2)；用计数器又会丢掉「类型」信息 ——
"([)]" 的左右数量完全配平，却是无效的。

换个角度：括号匹配的本质是「后进先出」—— 后出现的左括号必须先被闭合。
这正是栈的定义。扫到左括号就压栈，扫到右括号就和栈顶比：栈顶必须是同类
左括号，否则立即判无效。

关键点：
  · 用哈希表存「右括号 -> 左括号」的映射，比写一长串 if-else 更清楚；
    映射方向要和取栈顶的动作一致
  · 栈空时遇到右括号要直接判 false —— 否则取栈顶会越界（空栈没有 top）
  · 遍历结束后栈必须为空 —— 否则说明还有左括号没被闭合，例如 "(" 或 "(("
  · 奇数长度可以直接提前返回 false，省掉一次完整扫描

以示例 5（s = "([)]"）为例：
  '(' 入栈 → 栈 ['(']
  '[' 入栈 → 栈 ['(', '[']
  ')' 期望栈顶是 '('，实际是 '[' → 判 false

复杂度分析：
  时间 O(n)：每个字符最多入栈、出栈各一次
  空间 O(n)：最坏情况（全为左括号）栈里存 n 个字符
*/
class Solution
{
public:
    bool isValid(string s)
    {
        // 奇数长度不可能成对匹配，直接返回
        if (s.size() % 2 == 1)
        {
            return false;
        }

        // 右括号 -> 对应的左括号
        unordered_map<char, char> pairs = {
            {')', '('},
            {']', '['},
            {'}', '{'},
        };

        string stack;
        stack.reserve(s.size());

        for (char ch : s)
        {
            auto it = pairs.find(ch);
            if (it == pairs.end())
            {
                // 左括号：压栈，等对应的右括号来收
                stack.push_back(ch);
                continue;
            }

            // 右括号：栈不能为空，且栈顶必须是同类型的左括号
            if (stack.empty() || stack.back() != it->second)
            {
                return false;
            }
            stack.pop_back();
        }

        // 还有未闭合的左括号
        return stack.empty();
    }
};

namespace
{
// 一个用例 = 输入 + 期望输出
void check(string s, bool want)
{
    Solution solution;
    assert(solution.isValid(s) == want);
}
} // namespace

int main()
{
    // 示例 1：最基础的一对
    check("()", true);

    // 示例 2：多组括号平铺
    check("()[]{}", true);

    // 示例 3：类型不匹配
    check("(]", false);

    // 示例 4：正确嵌套
    check("([])", true);

    // 示例 5：数量配平但嵌套顺序错误，专防「只数个数」的写法
    check("([)]", false);

    // 边界：只有左括号，遍历完栈不为空
    check("(", false);

    // 边界：只有右括号，栈空时取栈顶会越界
    check(")", false);

    // 边界：单字符且长度为奇数，走提前返回分支
    check("]", false);

    cout << "all tests passed" << endl;
    return 0;
}
