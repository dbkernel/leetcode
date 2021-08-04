/*
剑指 Offer 31. 栈的压入、弹出序列

输入两个整数序列，第一个序列表示栈的压入顺序，请判断第二个序列是否为该栈的弹出顺序。假设压入栈的所有数字均不相等。例如，序列 {1,2,3,4,5} 是某栈的压栈序列，序列 {4,5,3,2,1} 是该压栈序列对应的一个弹出序列，但 {4,3,5,1,2} 就不可能是该压栈序列的弹出序列。

示例 1：
输入：pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
输出：true
解释：我们可以按以下顺序执行：
push(1), push(2), push(3), push(4), pop() -> 4,
push(5), pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1

示例 2：
输入：pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
输出：false
解释：1 不能在 2 之前弹出。

提示：
0 <= pushed.length == popped.length <= 1000
0 <= pushed[i], popped[i] < 1000
pushed 是 popped 的排列。
题目中假设压入栈的所有数字均不相等。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/zhan-de-ya-ru-dan-chu-xu-lie-lcof

思路：
- 考虑借用一个辅助栈 stackstack ，模拟 压入 / 弹出操作的排列。根据是否模拟成功，即可得到结果。
- 入栈操作： 按照压栈序列的顺序执行。
- 出栈操作： 每次入栈后，循环判断 “栈顶元素 == 弹出序列的当前元素” 是否成立，将符合弹出序列顺序的栈顶元素全部弹出。
*/
#include <vector>
#include <stack>
#include <cassert>
using namespace std;

// 网上有一种精简的解法，少了if判断，而是先入栈再判断，虽然简洁，但会多几次push操作
/*
// Java
class Solution {
    public boolean validateStackSequences(int[] pushed, int[] popped) {
        Stack<Integer> stack = new Stack<>();
        int i = 0;
        for(int num : pushed) {
            stack.push(num); // num 入栈
            while(!stack.isEmpty() && stack.peek() == popped[i]) { // 循环判断与出栈
                stack.pop();
                i++;
            }
        }
        return stack.isEmpty();
    }
}
*/

class Solution
{
public:
    bool validateStackSequences(vector<int> &pushed, vector<int> &popped)
    {
        stack<int> stk;
        int i = 0;

        for (int num : pushed)
        {
            if (num != popped[i])
                stk.push(num);
            else
            {
                // 如果发现 弹出序列的当前元素 与 压入序列的当前元素 相同，没必要先入栈再出栈，只需要将 i++
                i++;
                while (!stk.empty() && stk.top() == popped[i])
                {
                    stk.pop();
                    i++;
                }
            }
        }

        return stk.empty();
    }
};

int main()
{
    Solution s;
    vector<int> left = {1, 2, 3, 4, 5};
    vector<int> right = {4, 5, 3, 2, 1};
    assert(s.validateStackSequences(left, right) == true);
    return 0;
}
