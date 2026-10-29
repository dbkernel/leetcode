"""
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
"""
from collections import defaultdict
from typing import List


"""
解法一：排序后的字符串作为键【非最优】

为什么慢：互为字母异位词的字符串排序后完全相同，所以把排序结果当键
可以正确分组。但每个字符串都要排序，长度 k 的排序是 O(k log k)；
而"是不是同一组"只取决于每个字母出现多少次，与顺序无关，排序做了
多余的工作。

换个角度：用长度为 26 的计数数组描述"每种字母出现几次"，它才是字母
异位词真正的等价类标识。计数一次线性扫描即可，不需要排序。

关键点：
  · 键必须能唯一标识一组异位词：排序串和计数数组都满足
  · sorted(s) 返回列表，join 后得到稳定的字符串键
  · 计数键在 Python 里要把 list 转成 tuple，list 不可哈希
  · 哈希冲突只影响性能，不影响正确性

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
    会溢出；Python 虽有大整数，但代价同样比计数数组高

关键点：
  · 计数数组按字母顺序固定长度 26，空字符串得到全 0 数组，同样能分组
  · Python 的 list 不可哈希，必须 tuple(counts) 才能当字典键
  · 遍历字符串后 counts[ord(ch)-ord('a')] 落在 0..25
  · 结果顺序任意，最后只需把字典的值收集成列表

以示例 1（strs = ["eat","tea","tan","ate","nat","bat"]）为例：
  "eat"/"tea"/"ate" 的计数都是 a=1,e=1,t=1 → 同一组
  "tan"/"nat" 的计数都是 a=1,n=1,t=1 → 同一组
  "bat" 是 a=1,b=1,t=1 → 单独一组 ✔

复杂度分析：
  时间 O(n(k+26)) = O(nk)：每个字符串线性统计
  空间 O(nk)：哈希表保存所有字符串；每个键额外 26 个整数
"""


class Solution1:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for s in strs:
            key = "".join(sorted(s))
            groups[key].append(s)
        return list(groups.values())


class Solution2:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for s in strs:
            counts = [0] * 26
            for ch in s:
                counts[ord(ch) - ord("a")] += 1
            # list 不可哈希，转成 tuple 才能作为字典键
            groups[tuple(counts)].append(s)
        return list(groups.values())


# 题面允许任意顺序：组内排序、组间排序后再比较
def normalize(groups: List[List[str]]) -> List[List[str]]:
    return sorted(sorted(g) for g in groups)


def check(strs: List[str], want: List[List[str]]) -> None:
    got1 = normalize(Solution1().groupAnagrams(list(strs)))
    expected = normalize(want)
    assert got1 == expected, f"sort({strs}) = {got1}, want {expected}"

    got2 = normalize(Solution2().groupAnagrams(list(strs)))
    assert got2 == expected, f"count({strs}) = {got2}, want {expected}"


if __name__ == "__main__":
    # 示例 1
    check(
        ["eat", "tea", "tan", "ate", "nat", "bat"],
        [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]],
    )

    # 示例 2
    check([""], [[""]])

    # 示例 3
    check(["a"], [["a"]])

    # 边界：两个空串属于同一组
    check(["", ""], [["", ""]])

    # 边界：空串与非空串不能混组
    check(["", "a"], [[""], ["a"]])

    # 反直觉：顺序完全不同，但排序键一致
    check(
        ["abc", "bca", "cab", "xyz", "zyx", "foo"],
        [["abc", "bca", "cab"], ["xyz", "zyx"], ["foo"]],
    )

    # 反直觉：两组异位词交错出现
    check(
        ["aab", "aba", "baa", "abb", "bab", "bba"],
        [["aab", "aba", "baa"], ["abb", "bab", "bba"]],
    )

    # 边界：完全相同的字符串也要全部保留
    check(
        ["dddddddddddddddddddd", "dddddddddddddddddddd"],
        [["dddddddddddddddddddd", "dddddddddddddddddddd"]],
    )

    # 边界：长度不同的异位词（前缀相同也不是同组）
    check(
        ["ab", "ba", "abc", "cba", "bac"],
        [["ab", "ba"], ["abc", "cba", "bac"]],
    )

    print("all tests passed")
