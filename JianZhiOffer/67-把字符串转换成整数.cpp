/*
剑指 Offer 67. 把字符串转换成整数

写一个函数 StrToInt，实现把字符串转换成整数这个功能。不能使用 atoi 或者其他类似的库函数。

首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。

当我们寻找到的第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字组合起来，作为该整数的正负号；
假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。

该字符串除了有效的整数部分之后也可能会存在多余的字符，这些字符可以被忽略，它们对于函数不应该造成影响。

注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换。

在任何情况下，若函数不能进行有效的转换时，请返回 0。

说明：

假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为[−2^31, 2^31 − 1]。
如果数值超过这个范围，请返回 INT_MAX (2^31 − 1) 或 INT_MIN (−2^31) 。

示例 1:
输入: "42"
输出: 42

示例 2:
输入: "   -42"
输出: -42
解释: 第一个非空白字符为 '-', 它是一个负号。
     我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。

示例 3:
输入: "4193 with words"
输出: 4193
解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。

示例 4:
输入: "words and 987"
输出: 0
解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
     因此无法执行有效的转换。

示例 5:
输入: "-91283472332"
输出: -2147483648
解释: 数字 "-91283472332" 超过 32 位有符号整数范围。
     因此返回 INT_MIN (−2^31) 。

注意：本题与主站 8 题相同：https://leetcode-cn.com/problems/string-to-integer-atoi/

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/ba-zi-fu-chuan-zhuan-huan-cheng-zheng-shu-lcof
*/

#include <cassert>
#include <climits>
#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>

using namespace std;

/*
方法一：精细化处理（耗时 0ms，超过 100%；内存占用也超过 100%）
*/
class Solution1 {
public:
    int strToInt(string str) {
        int i = 0, flag = 1;
        long res = 0; //默认flag = 1，正数

        // 跳过空白
        while (str[i] == ' ')
            ++i;

        // 处理正负符号
        if (str[i] == '-')
            flag = -1;
        if (str[i] == '-' || str[i] == '+')
            ++i;

        // 处理数字
        for (; i < str.size() && isdigit(str[i]); ++i)  {
            res = res * 10 + (str[i] - '0');
            if (res >= INT_MAX && flag == 1)
                return INT_MAX;
            if (res > INT_MAX && flag == -1)
                return INT_MIN;
        }
        return flag * res;
    }
};

/*
方法二：自动状态机（思路简单清晰，但耗时较旧，最少 8ms）

思路

字符串处理的题目往往涉及复杂的流程以及条件情况，如果直接上手写程序，一不小心就会写出极其臃肿的代码。

因此，为了有条理地分析每个输入字符的处理方法，我们可以使用自动机这个概念：

我们的程序在每个时刻有一个状态 s，每次从序列中输入一个字符 c，并根据字符 c 转移到下一个状态 s'。
这样，我们只需要建立一个覆盖所有情况的从 s 与 c 映射到 s' 的表格即可解决题目中的问题。

算法

本题可以建立如下表格所示的自动机（比如，当 start 遇到 +/- 时，则需要进入 signed 状态）：

          ' '      +/-	    number      other
start     start    signed	in_number	end
signed	  end      end	    in_number	end
in_number end      end	    in_number	end
end       end      end	    end         end

接下来编程部分就非常简单了：我们只需要把上面这个状态转换表抄进代码即可。

另外自动机也需要记录当前已经输入的数字，只要在 s' 为 in_number 时，更新我们输入的数字，即可最终得到输入的数字。

复杂度分析
- 时间复杂度：O(n)，其中 n 为字符串的长度。我们只需要依次处理所有的字符，处理每个字符需要的时间为 O(1)。
- 空间复杂度：O(1)。自动机的状态只需要常数空间存储。

作者：LeetCode-Solution
链接：https://leetcode.cn/problems/ba-zi-fu-chuan-zhuan-huan-cheng-zheng-shu-lcof/solution/ba-zi-fu-chuan-zhuan-huan-cheng-zheng-sh-epeo/
来源：力扣（LeetCode）
*/

class Automaton {
private:
    string state = "start";
    unordered_map<string, vector<string> > table = {
            {"start", {"start", "signed", "in_number", "end"}},
            {"signed", {"end", "end", "in_number", "end"}},
            {"in_number", {"end", "end", "in_number", "end"}},
            {"end", {"end", "end", "end", "end"}}
    };

    // 根据当前字符的类型返回 table 中对应的 state 下标
    int get_col(char c) {
        if (isspace(c)) return 0;
        if (c == '+' or c == '-') return 1;
        if (isdigit(c)) return 2;
        return 3;
    }

public:
    int sign = 1;
    long long value = 0;

    Automaton() {}
    ~Automaton() {}

    void get(char c) {
        state = table[state][get_col(c)];
        if (state == "in_number") {
            value = value * 10 + c - '0'; // 求值
            // 处理溢出的情况
            value = (sign == 1 ? min(value, (long long)INT_MAX) : min(value, -(long long)INT_MIN));
        } else if (state == "signed")
            sign = (c == '-' ? -1 : 1); // 处理正负符号
    }
};

class Solution2 {
public:
    int strToInt(string str) {
        Automaton automaton;
        for (char c : str)
            automaton.get(c);
        return automaton.sign * automaton.value;
    }
};

int main() {
    Solution1 s1;
    assert(s1.strToInt("42") == 42);
    assert(s1.strToInt("-42") == -42);
    assert(s1.strToInt("4193 with words") == 4193);
    assert(s1.strToInt("words and 987") == 0);
    assert(s1.strToInt("-91283472332") == -2147483648); // INT_MIN

    Solution1 s2;
    assert(s2.strToInt("42") == 42);
    assert(s2.strToInt("-42") == -42);
    assert(s2.strToInt("4193 with words") == 4193);
    assert(s2.strToInt("words and 987") == 0);
    assert(s2.strToInt("-91283472332") == -2147483648); // INT_MIN

    return 0;
}