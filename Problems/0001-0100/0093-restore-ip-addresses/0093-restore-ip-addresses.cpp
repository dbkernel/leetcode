/*
============================================================================
93. 复原 IP 地址（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/restore-ip-addresses/
分类：回溯 · 定长分段与剪枝

有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有
前导 0），整数之间用 '.' 分隔。

例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、
"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。

给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP
地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能 重新排序或删除 s 中的
任何数字。你可以按 任何 顺序返回答案。

示例 1：
输入：s = "25525511135"
输出：["255.255.11.135","255.255.111.35"]

示例 2：
输入：s = "0000"
输出：["0.0.0.0"]

示例 3：
输入：s = "101023"
输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]

提示：
- 1 <= s.length <= 20
- s 仅由数字组成
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

/*
解法：回溯枚举四段（长度 1~3、无前导零、值不超过 255）【最优解】

为什么慢：最直接的想法是从 n-1 个空隙里任选 3 个插入点，共 C(n-1, 3) 种
切法，再逐段校验。切法本身不多，但它完全不做约束传播：切到 "999" 这样的段
还会继续往下切，白白展开一整棵子树。

换个角度：IP 只有固定四段，每段长度只能是 1~3，于是可以按"段"递归：每层
从 start 处取 1~3 个字符作为当前段，校验通过才往下走，取到第 4 段时要求
恰好走到字符串末尾。深度固定为 4，分支最多 3，搜索树极小。

关键点：
  · 段长超过 1 时不能以 '0' 开头，"0.011" 这类写法要整段砍掉
  · 段值边取边累加，一旦超过 255 就 break：再往后取只会更大，不可能回到合法
  · 第 4 段必须恰好消耗完所有字符；提前用完或还剩字符都不算解
  · 递归前先算剩余字符数 remain，必须满足 remain >= 剩余段数且
    remain <= 3 * 剩余段数，这一步能把长度明显不够或太长的输入直接剪没
  · 拼接用 cur.resize(len) 一次性撤掉"当前段 + 前面那个点"，不会误删旧内容

以示例 1（s = "25525511135"）为例：
  前两段都取 255，剩 "11135" 要拆成两段
  第三段取 "11"，第四段 "135" → 255.255.11.135 ✔
  第三段改取 "111"，第四段 "35" → 255.255.111.35 ✔
  第三段取 "1113" 超过 255 被 break，其余分支同理剪掉

复杂度分析：
  时间 O(3^4 · n)：深度固定为 4、每层最多 3 个分支，叶子最多 81 个，
    每个叶子拼一次长度 O(n) 的字符串；n 再大也不会改变搜索树规模
  空间 O(4)：递归栈只有 4 层，cur 长度不超过 n + 3；结果集不计入
*/
class Solution
{
public:
    // 契约：返回全部合法 IP，顺序任意；s 非法则返回空列表
    vector<string> restoreIpAddresses(string s)
    {
        vector<string> res;
        string cur;
        dfs(s, 0, 0, cur, res);
        return res;
    }

private:
    void dfs(const string &s, int start, int part,
             string &cur, vector<string> &res)
    {
        if (part == 4)
        {
            // 四段用完且字符也正好用完，才是一个完整地址
            if (start == static_cast<int>(s.size())) res.push_back(cur);
            return;
        }

        int remain = static_cast<int>(s.size()) - start;
        int left = 4 - part;
        // 剩余字符太少填不满、太多装不下，这一支直接剪掉
        if (remain < left || remain > 3 * left) return;

        int len = static_cast<int>(cur.size());
        int value = 0;
        for (int k = 1; k <= 3 && start + k <= static_cast<int>(s.size()); ++k)
        {
            // 多位数不允许前导零；再长也还是以 0 开头，直接停
            if (k > 1 && s[start] == '0') break;

            value = value * 10 + (s[start + k - 1] - '0');
            // 已经超过 255，继续取只会更大
            if (value > 255) break;

            if (part > 0) cur.push_back('.');
            cur.append(s, start, k);

            dfs(s, start + k, part + 1, cur, res);

            // 一次砍掉"段 + 点"，回到进入循环前的长度
            cur.resize(len);
        }
    }
};

namespace
{
// 逐字符回验：四段、无前导零、每段在 [0, 255]、拼回去等于原串
bool validIp(const string &ip, const string &s)
{
    vector<string> parts;
    string piece;
    for (char ch : ip)
    {
        if (ch == '.')
        {
            parts.push_back(piece);
            piece.clear();
        }
        else
        {
            piece.push_back(ch);
        }
    }
    parts.push_back(piece);
    if (parts.size() != 4) return false;

    string joined;
    for (const string &p : parts)
    {
        if (p.empty() || p.size() > 3) return false;
        if (p.size() > 1 && p[0] == '0') return false;
        int v = 0;
        for (char ch : p) v = v * 10 + (ch - '0');
        if (v > 255) return false;
        joined += p;
    }
    return joined == s;
}

void check(const string &s, vector<string> want)
{
    Solution sol;
    vector<string> got = sol.restoreIpAddresses(s);

    sort(got.begin(), got.end());
    sort(want.begin(), want.end());
    assert(got == want);

    // 每个输出都要能反解回原串，且四段都合法
    for (const string &ip : got) assert(validIp(ip, s));
}
} // namespace

int main()
{
    // 示例 1
    check("25525511135", {"255.255.11.135", "255.255.111.35"});

    // 示例 2：四个零，唯一解是全单段 0
    check("0000", {"0.0.0.0"});

    // 示例 3
    check("101023", {"1.0.10.23", "1.0.102.3", "10.1.0.23",
                     "10.10.2.3", "101.0.2.3"});

    // 前导零陷阱："010010" 不能拆成 01.00.1.0，只有下面两个解
    check("010010", {"0.10.0.10", "0.100.1.0"});

    // 刚好四位的下界
    check("1111", {"1.1.1.1"});

    // 四段都取满 255
    check("255255255255", {"255.255.255.255"});

    // 长度不足 4：不可能凑出四段
    check("123", {});
    check("0", {});

    // 长度超过 12：四段每段最多 3 位，必然放不下
    check("1234567890123", {});

    // 长度合法但只能靠"一位 + 两位"凑出四段
    check("11111", {"1.1.1.11", "1.1.11.1", "1.11.1.1", "11.1.1.1"});

    // 含连续零的易错串：多位数段不能全零，5 个 0 凑不出四段
    check("00000", {});

    cout << "all tests passed" << endl;
    return 0;
}
