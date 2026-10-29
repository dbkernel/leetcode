"""
============================================================================
面试题 01.02. 判定是否互为字符重排（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/check-permutation-lcci/
分类：哈希表 · 字符频次统计

给定两个由小写字母组成的字符串 s1 和 s2，请编写一个程序，确定其中一个
字符串的字符重新排列后，能否变成另一个字符串。

示例 1：
输入: s1 = "abc", s2 = "bca"
输出: true

示例 2：
输入: s1 = "abc", s2 = "bad"
输出: false

说明：
- 0 <= len(s1) <= 100
- 0 <= len(s2) <= 100
"""


"""
解法一：排序后比较【非最优】

为什么慢：互为字符重排等价于"两个字符串排序后相等"，思路最直白。
但排序每个字符串要 O(n log n)，而判断重排本质上只需要知道每种字符
出现了几次，不需要知道它们的顺序。

换个角度：把"顺序"这个无关信息直接丢掉，只比较字符频次。排序是丢掉
顺序的一种方式，但代价偏高；频次数组可以线性完成。

关键点：
  · 先比较长度，长度不同一定不是重排，直接返回 false
  · sorted() 返回新列表，不修改调用方传入的字符串
  · 排序后逐字符相等即可，不要只比较字符集合
  · 长度相同且频次相同才是充要条件

以示例 1（s1 = "abc", s2 = "bca"）为例：
  排序后都是 "abc"，相等 → true ✔
以示例 2（s1 = "abc", s2 = "bad"）为例：
  排序后 "abc" 与 "abd"，不相等 → false ✔

复杂度分析：
  时间 O(n log n)：两次排序
  空间 O(n)：排序后的两个列表

解法二：频次数组（一次遍历）【最优解】

用长度为 256 的数组记录频次：先遍历 s1 累加，再遍历 s2 递减。
一旦某个字符的频次变成负数，说明 s2 多用了这个字符，立即返回 false。
长度相等时，总和为 0，因此不需要在最后再扫一遍数组确认全零。

关键点：
  · 先比较长度，避免 s2 比 s1 长时出现"多出的字符恰好抵消"的误判
  · 用 ord(ch) 作下标，范围 0..255，数组开 256 不会越界
  · 递减后立刻检查 < 0，能提前发现不匹配
  · 长度相等时"没有负数"就等价于所有频次恰好为 0

以示例 1（s1 = "abc", s2 = "bca"）为例：
  table[a]=1,b=1,c=1；s2 依次减到全 0，没有负数 → true ✔
以示例 2（s1 = "abc", s2 = "bad"）为例：
  s2 处理到 'd' 时 table['d'] 从 0 减到 -1 → false ✔

复杂度分析：
  时间 O(n)：两次线性遍历
  空间 O(1)：字符集大小固定为 256
"""


class Solution1:
    def CheckPermutation(self, s1: str, s2: str) -> bool:
        if len(s1) != len(s2):
            return False
        return sorted(s1) == sorted(s2)


class Solution2:
    def CheckPermutation(self, s1: str, s2: str) -> bool:
        if len(s1) != len(s2):
            return False

        table = [0] * 256
        for ch in s1:
            table[ord(ch)] += 1
        for ch in s2:
            # 先减再判负：s2 用到了 s1 没有的字符时立刻失败
            table[ord(ch)] -= 1
            if table[ord(ch)] < 0:
                return False
        # 长度相等时频次总和为 0，没有负数就意味着全为 0
        return True


def check(s1: str, s2: str, want: bool) -> None:
    got1 = Solution1().CheckPermutation(s1, s2)
    assert got1 == want, f"sort({s1!r},{s2!r}) = {got1}, want {want}"

    got2 = Solution2().CheckPermutation(s1, s2)
    assert got2 == want, f"count({s1!r},{s2!r}) = {got2}, want {want}"


if __name__ == "__main__":
    # 示例 1
    check("abc", "bca", True)

    # 示例 2
    check("abc", "bad", False)

    # 边界：两个空串互为重排
    check("", "", True)

    # 边界：长度不同，即使前缀相同也不是重排
    check("a", "", False)
    check("abc", "ab", False)

    # 边界：单字符
    check("a", "a", True)
    check("a", "b", False)

    # 反直觉：字符集合相同但频次不同
    check("aab", "abb", False)

    # 反直觉：顺序完全不同，但频次一致
    check("aabbcc", "ccbbaa", True)
    check("abcdefghij", "jihgfedcba", True)

    # 边界：100 个字符的极限长度
    same = "a" * 100
    check(same, same, True)
    check("a" * 99 + "b", same, False)

    print("all tests passed")
