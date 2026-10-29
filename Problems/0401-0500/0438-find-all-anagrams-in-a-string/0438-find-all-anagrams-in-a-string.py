"""
============================================================================
438. 找到字符串中所有字母异位词（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-all-anagrams-in-a-string/
分类：滑动窗口 · 定长窗口 + 频次差量

给定两个字符串 s 和 p，找到 s 中所有 p 的 异位词 的子串，返回这些子串的
起始索引。不考虑答案输出的顺序。

示例 1:
输入: s = "cbaebabacd", p = "abc"
输出: [0,6]
解释:
起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词。
起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词。

示例 2:
输入: s = "abab", p = "ab"
输出: [0,1,2]
解释:
起始索引等于 0 的子串是 "ab", 它是 "ab" 的异位词。
起始索引等于 1 的子串是 "ba", 它是 "ab" 的异位词。
起始索引等于 2 的子串是 "ab", 它是 "ab" 的异位词。

提示:
- 1 <= s.length, p.length <= 3 * 10^4
- s 和 p 仅包含小写字母
"""


"""
解法一：排序后逐窗口比较【非最优】

为什么慢：判断一个窗口是否为 p 的异位词，最省事的办法是把两边都排序再
比较；但每个长度为 m 的窗口都要单独排一次序，共有 n-m+1 个窗口，最坏
（m 接近 n）是 O(n·m log m)。相邻窗口的内容几乎相同，重复排序完全是
浪费。

换个角度：异位词的定义就是"字符种类和个数完全相同"。两个串各自排序后
逐位相等，当且仅当它们互为异位词 —— 计数比较被换成了列表比较，一行
比较就能判完。

关键点：
  · p 只排序一次，窗口每次都要重排；两边都排过才能直接比
  · 窗口起点的上界是 n-m，写成 n-m+1 会取到越界的尾部
  · 切片会新拷一份，排序不会改动原串
  · 这个写法只适合 m 很小的情况，价值在于给解法二做对照

手推一遍：以示例 2（s = "abab"，p = "ab"）为例：
  p 排序为 ['a', 'b']
  起点 0："ab" ✓、起点 1："ba" 排序后 "ab" ✓、起点 2："ab" ✓
  答案 [0,1,2] ✔

复杂度分析：
  时间 O((n-m+1)·m log m)：每个窗口排序一次
  空间 O(m)：排序需要一份窗口副本，p 的副本也只占 m
"""


class Solution1:
    def findAnagrams(self, s: str, p: str) -> list:
        n, m = len(s), len(p)
        if m > n:
            return []

        key = sorted(p)
        ans = []
        for i in range(n - m + 1):
            if sorted(s[i:i + m]) == key:
                ans.append(i)
        return ans


"""
解法二：定长滑动窗口 + 频次差量【最优解】

为什么慢：解法一对每个窗口都重新排序。换成每个窗口重新数 26 个字母，
时间降到 O(26(n-m+1))，但仍在"每滑动一格就从头统计"，把相邻窗口几乎
相同的信息又算了一遍。

换个角度：窗口长度固定为 m，右移一格只有两个字符变化：s[i-m] 离开、
s[i] 进入。维护一张差值表 diff[c] = 窗口内 c 的个数 - p 中 c 的个数，
再用一个整数 diff_count 记录 diff 里非零项的个数 —— 全为零就说明窗口
与 p 的字符构成完全一致。每次滑动只改两项，diff_count 随之 O(1) 更新。

关键点：
  · diff_count 的更新规则是"越过零点才算变号"：0 → ±1 时 +1，
    ±1 → 0 时 -1，其余情况不变；写成"只要变动就 +1/-1"会数错
  · 先移出再移入，中间那一刻窗口长度是 m-1，不能拿去判断命中
  · 起点按 i-m+1 算：i 是右端点，i = m-1 时刚好对上起点 0
  · 字符只含小写字母，26 长的列表就够，不必开字典
  · p 比 s 长时直接返回空，否则窗口根本放不下

手推一遍：以示例 1（s = "cbaebabacd"，p = "abc"）为例：
  首个窗口 "cba"：a、b、c 各一个，diff 全零 → 记 0
  逐格右移，直到 i = 8 时窗口为 s[6..8] = "bac"，diff 再次全零 → 记 6
  其余窗口都缺字母或多字母，diff_count 不为零
  答案 [0,6] ✔

复杂度分析：
  时间 O(n + m + 26) = O(n + m)：建表 O(m + 26)，每次滑动只动两个字符
  空间 O(26) = O(1)：差值表与字母表等长，与 n、m 无关
"""


class Solution2:
    def findAnagrams(self, s: str, p: str) -> list:
        n, m = len(s), len(p)
        if m > n:
            return []

        # diff[c] = 窗口内 c 的个数 - p 中 c 的个数，0 表示这一项已配平
        diff = [0] * 26
        for ch in p:
            diff[ord(ch) - 97] -= 1
        for ch in s[:m]:
            diff[ord(ch) - 97] += 1

        diff_count = sum(1 for v in diff if v != 0)

        def change(ch: str, delta: int) -> None:
            nonlocal diff_count
            x = ord(ch) - 97
            # 0 → ±1：这一项从配平变成不配平
            if diff[x] == 0:
                diff_count += 1
            elif diff[x] == -delta:
                # ±1 → 0：这一项刚好回归配平；绝对值更大时只会更偏
                diff_count -= 1
            diff[x] += delta

        ans = []
        if diff_count == 0:
            ans.append(0)

        for i in range(m, n):
            # 一轮换一个字符：先移出左端，再移入右端
            change(s[i - m], -1)
            change(s[i], 1)
            if diff_count == 0:
                ans.append(i - m + 1)

        return ans


def check(s: str, p: str, want: list) -> None:
    assert Solution1().findAnagrams(s, p) == want
    assert Solution2().findAnagrams(s, p) == want


if __name__ == "__main__":
    # 示例 1：命中位置不连续
    check("cbaebabacd", "abc", [0, 6])

    # 示例 2：窗口之间互相重叠，每个位置都可能命中
    check("abab", "ab", [0, 1, 2])

    # 边界：模式比串长，一个窗口都放不下
    check("ab", "abc", [])

    # 边界：单字符对单字符
    check("a", "a", [0])
    check("a", "b", [])

    # 边界：整个串就是一个多重集的重复
    check("aa", "aa", [0])

    # 反直觉：只有第二个位置命中，首窗口不命中
    check("baa", "aa", [1])

    # 压力：3*10^4 个 a 对 "a"，答案覆盖每个起点
    check("a" * 30000, "a", list(range(30000)))

    print("all tests passed")
