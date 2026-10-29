/*
============================================================================
49. 字母异位词分组（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/group-anagrams/
分类：哈希表 · 排序键 / 计数键

给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回
结果列表。

示例 1:
输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
解释：
- 在 strs 中没有字符串可以通过重新排列来形成 "bat"。
- 字符串 "nat" 和 "tan" 是字母异位词，因为它们可以重新排列以形成彼此。
- 字符串 "ate" 、"eat" 和 "tea" 是字母异位词，因为它们可以重新排列
  以形成彼此。

示例 2:
输入: strs = [""]
输出: [[""]]

示例 3:
输入: strs = ["a"]
输出: [["a"]]

提示：
- 1 <= strs.length <= 10^4
- 0 <= strs[i].length <= 100
- strs[i] 仅包含小写字母
*/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

/*
解法一：排序后的字符串作为键【非最优】

为什么慢：互为字母异位词的字符串排序后完全相同，所以把排序结果当键
可以正确分组。但每个字符串都要排序，长度 k 的排序是 O(k log k)；
而"是不是同一组"只取决于每个字母出现多少次，与顺序无关，排序做了
多余的工作。

换个角度：用长度为 26 的计数数组描述"每种字母出现几次"，它才是字母
异位词真正的等价类标识。计数一次线性扫描即可，不需要排序。

关键点：
  · 键必须能唯一标识一组异位词：排序串和计数数组都满足
  · 排序键直接复用 string 作为哈希键，写法最短
  · 计数键在 C++ 里是 array<int,26>，标准库没有默认哈希，要自定义
  · 自定义哈希只需保证"相等对象哈希相同"，冲突由 unordered_map 处理

以示例 1 中的 "eat"、"tea"、"ate" 为例：
  排序后都是 "aet"，落进同一个桶 ✔
  "tan"、"nat" 排序后都是 "ant"，落进另一个桶 ✔

复杂度分析：
  时间 O(n k log k)：n 个字符串，每个排序 O(k log k)
  空间 O(n k)：哈希表保存所有字符串

解法二：计数数组作为键（最优解）

对每个字符串统计 26 个字母的出现次数，把计数数组整体作为键。
  · 每个字符串 O(k) 统计 + O(26) 生成键，总时间 O(n(k+26)) = O(nk)
  · 空间仍是 O(nk)，但时间比排序键少一个 log k
  · 不要用"26 个质数相乘"当键：前 26 个质数的乘积远超 64 位整数，
    会溢出；即使改用大整数，代价也比计数数组高

关键点：
  · 计数数组按字母顺序固定长度 26，空字符串得到全 0 数组，同样能分组
  · C++ 的 array<int,26> 需要自定义哈希函数，不能直接塞进 unordered_map
  · Go 的 [26]int、Python 的 tuple、Rust 的 [i32;26] 本身可哈希，直接当键
  · 哈希冲突只影响性能，不影响正确性，相等判断仍逐元素比较

以示例 1（strs = ["eat","tea","tan","ate","nat","bat"]）为例：
  "eat"/"tea"/"ate" 的计数都是 a=1,e=1,t=1 → 同一组
  "tan"/"nat" 的计数都是 a=1,n=1,t=1 → 同一组
  "bat" 是 a=1,b=1,t=1 → 单独一组 ✔

复杂度分析：
  时间 O(n(k+26)) = O(nk)：每个字符串线性统计
  空间 O(nk)：哈希表保存所有字符串；每个键额外 26 个整数
*/

// array<int,26> 没有标准哈希，这里按计数逐位混合
struct ArrayHash
{
    size_t operator()(const array<int, 26> &counts) const
    {
        size_t h = 0;
        for (int c : counts) h = h * 131 + static_cast<size_t>(c);
        return h;
    }
};

class Solution1
{
public:
    vector<vector<string>> groupAnagrams(vector<string> &strs)
    {
        unordered_map<string, vector<string>> groups;
        for (const string &s : strs)
        {
            string key = s;
            sort(key.begin(), key.end());
            groups[key].push_back(s);
        }

        vector<vector<string>> ans;
        ans.reserve(groups.size());
        for (auto &kv : groups) ans.push_back(kv.second);
        return ans;
    }
};

class Solution2
{
public:
    vector<vector<string>> groupAnagrams(vector<string> &strs)
    {
        // 键是"每个字母出现几次"，与字符顺序无关
        unordered_map<array<int, 26>, vector<string>, ArrayHash> groups;
        for (const string &s : strs)
        {
            array<int, 26> counts{};
            for (char ch : s) ++counts[ch - 'a'];
            groups[counts].push_back(s);
        }

        vector<vector<string>> ans;
        ans.reserve(groups.size());
        for (auto &kv : groups) ans.push_back(kv.second);
        return ans;
    }
};

namespace
{
// 题面允许任意顺序：组内排序、组间排序后再比较
void normalize(vector<vector<string>> &groups)
{
    for (auto &g : groups) sort(g.begin(), g.end());
    sort(groups.begin(), groups.end());
}

void check(const vector<string> &strs,
           const vector<vector<string>> &want)
{
    vector<string> input1 = strs;
    vector<vector<string>> got1 = Solution1().groupAnagrams(input1);
    vector<vector<string>> exp1 = want;
    normalize(got1);
    normalize(exp1);
    assert(got1 == exp1);

    vector<string> input2 = strs;
    vector<vector<string>> got2 = Solution2().groupAnagrams(input2);
    vector<vector<string>> exp2 = want;
    normalize(got2);
    normalize(exp2);
    assert(got2 == exp2);
}

} // namespace

int main()
{
    // 示例 1
    check({"eat", "tea", "tan", "ate", "nat", "bat"},
          {{"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}});

    // 示例 2
    check({""}, {{""}});

    // 示例 3
    check({"a"}, {{"a"}});

    // 边界：两个空串属于同一组
    check({"", ""}, {{"", ""}});

    // 边界：空串与非空串不能混组
    check({"", "a"}, {{""}, {"a"}});

    // 反直觉：顺序完全不同，但排序键一致
    check({"abc", "bca", "cab", "xyz", "zyx", "foo"},
          {{"abc", "bca", "cab"}, {"xyz", "zyx"}, {"foo"}});

    // 反直觉：两组异位词交错出现
    check({"aab", "aba", "baa", "abb", "bab", "bba"},
          {{"aab", "aba", "baa"}, {"abb", "bab", "bba"}});

    // 边界：完全相同的字符串也要全部保留
    check({"dddddddddddddddddddd", "dddddddddddddddddddd"},
          {{"dddddddddddddddddddd", "dddddddddddddddddddd"}});

    // 边界：长度不同的异位词（前缀相同也不是同组）
    check({"ab", "ba", "abc", "cba", "bac"},
          {{"ab", "ba"}, {"abc", "cba", "bac"}});

    cout << "all tests passed" << endl;
    return 0;
}
