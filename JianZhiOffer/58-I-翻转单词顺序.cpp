/*
剑指 Offer 58 - I. 翻转单词顺序

输入一个英文句子，翻转句子中单词的顺序，但单词内字符的顺序不变。为简单起见，标点符号和普通字母一样处理。例如输入字符串"I
am a student. "，则输出"student. a am I"。

示例 1：
输入: "the sky is blue"
输出: "blue is sky the"

示例 2：
输入: "  hello world!  "
输出: "world! hello"
解释: 输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能包括。

示例 3：
输入: "a good   example"
输出: "example good a"
解释: 如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。
 

说明：
- 无空格字符构成一个单词。
- 输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能包括。
- 如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。

注意：本题与主站 151
题相同：https://leetcode.cn/problems/reverse-words-in-a-string/

注意：此题对比原题有改动

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/fan-zhuan-dan-ci-shun-xu-lcof
*/

#include <deque>
#include <iostream>
#include <stack>
#include <string>
using namespace std;

/*
方法一：自行编写 split、reverse、join 函数

复杂度分析：
- 时间复杂度：O(n)，其中 n 为输入字符串的长度。
- 空间复杂度：只需要 O(1) 的额外空间来存放若干变量。
*/
class Solution {
   public:
    string reverseWords(string s) {
        reverse(s.begin(), s.end());  // 翻转整个字符串

        int len = s.size();
        int idx = 0;  // 当字符串头部、尾部存在空格时，需要将字符往左移动
        for (int start = 0; start < len; ++start) {
            if (s[start] != ' ') {
                // 之前的循环处理过单词，需要在其后填一个空格字符，然后将idx移动到下一个单词的开头位置
                if (idx != 0)
                    s[idx++] = ' ';

                // 循环遍历至单词的末尾
                int end = start;
                while (end < len && s[end] != ' ')
                    s[idx++] = s
                        [end++];  // 将字符往左移动，以消除可能存在的空格；如果没有空格，相当于原地赋值，比如
                                  // s[2]=s[2]，编译器应该会做特殊处理。

                // 翻转当前单词
                reverse(s.begin() + idx - (end - start), s.begin() + idx);

                start = end;  // 更新start，去找下一个单词
            }
        }

        // 字符串最右边可能存在空格，需要删除
        s.erase(s.begin() + idx, s.end());
        return s;
    }
};

/*
方法二：双端队列（还不如直接用栈）

由于双端队列支持从队列头部插入的方法，因此我们可以沿着字符串一个一个单词处理，然后将单词压入队列的头部，再将队列转成字符串即可。

复杂度分析
- 时间复杂度：O(n)，其中 n 为输入字符串的长度。
- 空间复杂度：O(n)，双端队列存储单词需要 O(n) 的空间。
*/
class SolutionDQ {
   public:
    string reverseWords(string s) {
        int left = 0, right = s.size() - 1;
        // 去掉字符串开头的空白字符
        while (left <= right && s[left] == ' ')
            ++left;

        // 去掉字符串末尾的空白字符
        while (left <= right && s[right] == ' ')
            --right;

        deque<string> d;
        string word;

        while (left <= right) {
            char c = s[left];
            if (word.size() && c == ' ') {  // 说明当前单词查找完成
                // 将单词 push 到队列的头部
                d.push_front(move(word));
                word = "";
            } else if (c != ' ') {
                word += c;  // 追加到 word 中
            }
            ++left;
        }
        d.push_front(move(word));  // 将最后一个单词存入双端队列头部

        // 取出单词，重新拼接字符串
        string ans;
        while (!d.empty()) {
            ans += d.front();
            d.pop_front();  // 从头部移除
            if (!d.empty())
                ans += ' ';
        }
        return ans;
    }
};

/*
方法三：堆栈

将解析好的单词存入堆栈，并利用堆栈实现单词的后进先出
*/
class SolutionStack {
   public:
    string reverseWords(string s) {
        //栈解决字符串翻转

        stack<string> st;  //辅助栈
        string word;       //用于存放s中的每一个单词
        int left = 0, right = s.size() - 1;
        string res;  //最终返回的结果

        //去除s最前面的空格
        while (left <= right && s[left] == ' ')
            left++;
        //去除s最后面的空格
        while (right >= left && s[right] == ' ')
            right--;

        while (left <= right) {
            if (word.size() >= 1 && s[left] == ' ') {
                st.push(word);
                word.clear();
            } else if (s[left] != ' ')
                word += s[left];
            left++;
        }
        //跳出上面的循环后，word中存放了s中最后一个单词，还没来得及加入栈中
        st.push(word);

        // 取出单词，重新拼接字符串
        while (!st.empty()) {
            string tmp = st.top();
            st.pop();
            res += tmp;
            if (!st.empty())  //只要不是栈中最后一个单词，拼接完后就要加一个空格
                res += " ";
        }
        return res;
    }
};

int main() {
    {
        Solution s;
        string str = "Good Boy   There!  ";
        cout << s.reverseWords(str) << endl;
    }

    {
        SolutionDQ s;
        string str = "Good Boy   There!  ";
        cout << s.reverseWords(str) << endl;
    }

    {
        SolutionStack s;
        string str = "Good Boy   There!  ";
        cout << s.reverseWords(str) << endl;
    }

    return 0;
}
