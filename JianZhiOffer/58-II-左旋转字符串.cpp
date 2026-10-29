/*
============================================================================
LCR 182 / 剑指 Offer 58 - II. 动态口令（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/
分类：字符串 · 左旋转（切片 / 遍历拼接 / 三次翻转）

说明：本题原名「左旋转字符串」，leetcode.cn 现已更名为「动态口令」，
函数名改为 dynamicPassword，题面意思不变。
============================================================================

某公司门禁密码使用动态口令技术。初始密码为字符串 password，密码更新均
遵循以下步骤：

设定一个正整数目标值 target
将 password 前 target 个字符按原顺序移动至字符串末尾

请返回更新后的密码字符串。

示例 1：
输入: password = "s3cur1tyC0d3", target = 4
输出: "r1tyC0d3s3cu"

示例 2：
输入: password = "lrloseumgh", target = 6
输出: "umghlrlose"

提示：
1 <= target < password.length <= 10000
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
using namespace std;

/*
解法一：字符串切片（后半段 + 前半段直接拼接）【非最优】

为什么慢：C++ 的 substr 会新建两份字符串，再拼成第三份，一共要 O(N) 的
额外空间（三份拷贝同时存在）。思路最短，但空间用得不必要的多。

换个角度：左旋转 target 位，就是把字符串切成 s1（前 target 个）和
s2（其余）再交换顺序 —— 结果就是 s2 + s1。直接拼即可。

关键点：
  · 第二段起点是 target，长度是 size() - target；写成 size() 也能得到
    到尾的结果，但显式写长度更清楚
  · target 由题设为合法值（1 <= target < n），无需额外取模

以示例 1（password = "s3cur1tyC0d3", target = 4）为例：
  s1 = "s3cu"，s2 = "r1tyC0d3"
  拼接 s2 + s1 → "r1tyC0d3s3cu"

复杂度分析：
  时间 O(N)：两段各拷贝一次，再拼一次
  空间 O(N)：两份切片 + 结果字符串
*/
class Solution1
{
public:
    string dynamicPassword(string password, int target)
    {
        return password.substr(target, password.size() - target) +
               password.substr(0, target);
    }
};

/*
解法二：列表遍历拼接（先写后半段，再写前半段）【非最优】

为什么慢：官方原文的前提是「面试规定不允许使用切片函数」。为了不切片，
就开一个缓冲区按顺序把字符一个个搬过去，额外占 O(N) 空间。

换个角度：切片之所以快，是因为它内部也是按段拷贝。既然不能用切片，那就
手动做同一件事：先把 [target, n) 搬进缓冲区，再把 [0, target) 搬进去。
两段下标还能合并成一段 —— 从 target 开始走 n 步，用 i % n 回绕到开头。

关键点：
  · 两段循环的边界别写反：先 [target, n)，再 [0, target)
  · 用回绕写法时上界是 target + n，不是 n，否则只走了一半
  · 缓冲区预留 n 个字符，避免反复扩容

以示例 2（password = "lrloseumgh", target = 6）为例：
  先搬 [6, 11) → "umghl"
  再搬 [0, 6) → "lrlose"
  结果 "umghlrlose"

复杂度分析：
  时间 O(N)：每个字符搬一次
  空间 O(N)：缓冲区
*/
class Solution2
{
public:
    string dynamicPassword(string password, int target)
    {
        int n = static_cast<int>(password.size());
        string ans;
        ans.reserve(n);

        // 先搬后半段，用求余把「后半段 + 前半段」合并成一个循环
        for (int i = target; i < target + n; ++i)
        {
            ans.push_back(password[i % n]);
        }

        return ans;
    }
};

/*
解法三：字符串遍历拼接（不用缓冲区，直接往结果串上追加）【非最优】

为什么慢：官方原文的前提是「规定不能用 join、只能用 String」。这样每轮
追加都可能触发一次重新分配 —— 在字符串不可变的语言里，这会退化成
O(N^2) 次拷贝。C++ 的 string 在预留容量后是均摊 O(1)，但不能指望这点。

换个角度：与方法二完全相同，只是把「列表」换成「字符串」。它存在的意义
只在于满足语言特性受限的面试场景，不是更优的算法。

关键点：
  · 先用 reserve 预留 n 个字符 —— 少了这一步，追加就可能反复扩容
  · 两段顺序仍是「后半段在前」；顺序反了就变成右旋转
  · 不要在两段之间插入任何分隔符

以示例 2（password = "lrloseumgh", target = 6）为例：
  追加 [6, 11) → "umghl"
  追加 [0, 6) → "umghlrlose"

复杂度分析：
  时间 O(N)：预留容量后每个字符追加一次
  空间 O(N)：结果字符串
*/
class Solution3
{
public:
    string dynamicPassword(string password, int target)
    {
        int n = static_cast<int>(password.size());
        string ans;
        ans.reserve(n);

        for (int i = target; i < n; ++i)
        {
            ans.push_back(password[i]);
        }
        for (int i = 0; i < target; ++i)
        {
            ans.push_back(password[i]);
        }

        return ans;
    }
};

/*
解法四：三次翻转（原地旋转）【最优解】

官方四解中空间最优的一个，也是本题的最优解：时间 O(N) 已是下界，
额外空间 O(1) —— 前三种都要另开一份 O(N) 的缓冲区。

为什么慢：三种写法都在「搬运」。可有意思的是，交换两段顺序这件事，用翻转
就能在原地完成，一次额外内存都不用申请。

换个角度：记 s = s1 s2（|s1| = target），目标是 s2 s1。
  · 整体翻转 → rev(s2) rev(s1)
  · 把前 n - target 个（即 rev(s2)）翻转 → s2 rev(s1)
  · 把后 target 个（即 rev(s1)）翻转 → s2 s1
三次翻转，结果正是左旋转。

关键点：
  · 第二次翻转的区间是 [0, n - target)，第三次是 [n - target, n) ——
    分界点是 n - target，不是 target
  · 三次翻转必须按「整体 → 前段 → 后段」的顺序；先翻某一段再翻整体，
    得到的是右旋转
  · 用 std::reverse 处理左闭右开区间，边界写错一格结果就全错

以示例 1（password = "s3cur1tyC0d3", target = 4）为例：
  整体翻转 → "3d0Cyt1ruc3s"
  前 8 个翻转 → "r1tyC0d3" + "uc3s" → "r1tyC0d3uc3s"
  后 4 个翻转 → "r1tyC0d3" + "s3cu" → "r1tyC0d3s3cu"

复杂度分析：
  时间 O(N)：三次翻转合计扫两遍
  空间 O(1)：只在原串上交换字符
*/
class Solution4
{
public:
    string dynamicPassword(string password, int target)
    {
        int n = static_cast<int>(password.size());
        int cut = n - target;

        reverse(password.begin(), password.end());
        reverse(password.begin(), password.begin() + cut);
        reverse(password.begin() + cut, password.end());

        return password;
    }
};

namespace
{
// 一个用例喂给四种解法：四种写法必须给出同一个结果
void check(const string &password, int target, const string &want)
{
    Solution1 bySlice;
    Solution2 byList;
    Solution3 byString;
    Solution4 byReverse;

    assert(bySlice.dynamicPassword(password, target) == want);
    assert(byList.dynamicPassword(password, target) == want);
    assert(byString.dynamicPassword(password, target) == want);
    assert(byReverse.dynamicPassword(password, target) == want);
}
} // namespace

int main()
{
    // 示例 1：target 小于一半，验证分界点在 n - target
    check("s3cur1tyC0d3", 4, "r1tyC0d3s3cu");

    // 示例 2：target 大于一半，专防「分界点写成 target」
    check("lrloseumgh", 6, "umghlrlose");

    // 边界：target == n - 1，等价于把最后一个字符挪到最前
    check("abc", 2, "cab");

    // 边界：target == 1，等价于把第一个字符挪到最后
    check("abc", 1, "bca");

    // 边界：只有两个字符，四个循环体的最短形式
    check("ab", 1, "ba");

    cout << "all tests passed" << endl;
    return 0;
}
