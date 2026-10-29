/*
============================================================================
LCR 139 / 剑指 Offer 21. 调整数组顺序使奇数位于偶数前面（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof/
分类：双指针 · 原地交换（快排 partition 思路）

教练使用整数数组 actions 记录一系列核心肌群训练项目编号。为增强训练趣味性，
需要将所有奇数编号训练项目调整至偶数编号训练项目之前。请将调整后的训练项目编号
以 数组 形式返回。

示例 1：
输入：actions = [1,2,3,4,5]
输出：[1,3,5,2,4]
解释：为正确答案之一

提示：
0 <= actions.length <= 50000
0 <= actions[i] <= 10000

说明：本题为原「剑指 Offer 21. 调整数组顺序使奇数位于偶数前面」（函数名 exchange），
现在在力扣上以 LCR 139「训练计划 I」收录（函数名 trainingPlan），接口语义完全相同。
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：首尾双指针原地交换【最优解】

官方题解只给出这一种方法，也就是本题的最优解：时间 O(n)、空间 O(1)。

为什么慢：最容易想到的是「开一个新数组，先塞奇数再塞偶数」，时间虽然也是 O(n)，
但要多花 O(n) 的额外空间；另一种「遇到偶数就往后找奇数再整体挪动」的写法最坏
会退化成 O(n^2)。

换个角度：题目只要求「奇偶分块」，不要求偶数内部的相对顺序，也不要求奇数内部的
相对顺序。这恰恰是快速排序 partition 的弱化版——只需要一个分界点，让左侧全奇、
右侧全偶，不需要任何稳定性。

关键点：
  · 指针 i 从左往右跳过奇数（停下时指向一个偶数），指针 j 从右往左跳过偶数
    （停下时指向一个奇数），然后交换这两个「站错位置」的元素
  · 内层两个 while 都要带 i < j 的边界判断，否则 i、j 会互相越界
  · 用 x & 1 判断奇偶，等价于 x % 2，但更快；因为数组元素非负，不必担心负数取模
  · 循环条件是 i < j；当 i == j 时中间那个位置天然正确，无需处理
  · 空数组 / 单元素数组不会进入循环，直接返回
  · 本题不要求稳定，所以「原地交换」可行；若要求保持相对顺序（原书完整版），
    则需要辅助数组或冒泡式的稳定移动，复杂度会变差

以 actions = [1,2,3,4,5] 为例：
  i=0 是奇数 → i=1；此时 2 是偶数，i 停住
  j=4 是奇数 → j 停住
  交换 → [1,5,3,4,2]，i=2, j=3
  i=2 是奇数 → i=3；j=3 是偶数 → j 停在原地
  此时 i == j，退出循环
  返回 [1,5,3,4,2]（与示例给出的 [1,3,5,2,4] 同为合法答案）

复杂度分析：
  时间 O(n)：i 与 j 各自单向移动，合计最多走 n 步
  空间 O(1)：只用两个下标
*/
class Solution
{
public:
    // 当前力扣接口名
    vector<int> trainingPlan(vector<int> &actions)
    {
        int i = 0, j = static_cast<int>(actions.size()) - 1;
        while (i < j)
        {
            // 从左往右跳过奇数，停在第一个偶数上
            while (i < j && (actions[i] & 1) == 1) ++i;
            // 从右往左跳过偶数，停在第一个奇数上
            while (i < j && (actions[j] & 1) == 0) --j;
            swap(actions[i], actions[j]);
        }
        return actions;
    }

    // 原「剑指 Offer 21」接口名，语义完全一致
    vector<int> exchange(vector<int> &nums) { return trainingPlan(nums); }
};

namespace
{
// 校验：① 与输入是同一个多重集合 ② 所有奇数都排在所有偶数之前
void validate(const vector<int> &input, const vector<int> &output)
{
    assert(input.size() == output.size());

    vector<int> a = input, b = output;
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    assert(a == b);

    bool seenEven = false;
    for (int x : output)
    {
        if ((x & 1) == 0) seenEven = true;
        else assert(!seenEven); // 奇数不能出现在偶数之后
    }
}
} // namespace

int main()
{
    Solution solution;

    // 示例 1
    {
        vector<int> actions = {1, 2, 3, 4, 5};
        auto out = solution.trainingPlan(actions);
        validate({1, 2, 3, 4, 5}, out);
        assert((out == vector<int>{1, 5, 3, 4, 2}));
    }

    // 原题示例：结果确定
    {
        vector<int> nums = {1, 2, 3, 4};
        auto out = solution.exchange(nums);
        validate({1, 2, 3, 4}, out);
        assert((out == vector<int>{1, 3, 2, 4}));
    }

    // 边界：空数组
    {
        vector<int> empty;
        auto out = solution.trainingPlan(empty);
        assert(out.empty());
        validate({}, out);
    }

    // 边界：单元素
    {
        vector<int> one = {7};
        auto out = solution.trainingPlan(one);
        assert((out == vector<int>{7}));
        validate({7}, out);
    }

    // 边界：已经全部是奇数 / 全部是偶数
    {
        vector<int> allOdd = {1, 3, 5, 7};
        auto out = solution.trainingPlan(allOdd);
        assert((out == vector<int>{1, 3, 5, 7}));
        validate({1, 3, 5, 7}, out);
    }
    {
        vector<int> allEven = {2, 4, 6, 8};
        auto out = solution.trainingPlan(allEven);
        assert((out == vector<int>{2, 4, 6, 8}));
        validate({2, 4, 6, 8}, out);
    }

    // 边界：奇偶交替
    {
        vector<int> alt = {1, 2, 3, 4, 5, 6};
        auto out = solution.trainingPlan(alt);
        validate({1, 2, 3, 4, 5, 6}, out);
        assert((out == vector<int>{1, 5, 3, 4, 2, 6}));
    }

    // 边界：包含 0（偶数）和最大值 10000（偶数）
    {
        vector<int> vals = {0, 10000, 9999, 1};
        auto out = solution.trainingPlan(vals);
        validate({0, 10000, 9999, 1}, out);
    }

    // 较大规模：0..49999 奇偶交替，验证分块性质
    {
        vector<int> big(50000);
        for (int i = 0; i < 50000; ++i) big[i] = i;
        vector<int> original = big;
        auto out = solution.trainingPlan(big);
        validate(original, out);
    }

    cout << "all tests passed" << endl;
    return 0;
}
