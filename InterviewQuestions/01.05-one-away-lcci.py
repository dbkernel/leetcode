"""
============================================================================
面试题 01.05. 一次编辑（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/one-away-lcci/
分类：字符串 · 双指针

字符串有三种编辑操作:插入一个英文字符、删除一个英文字符或者替换一个英文字符。
给定两个字符串，编写一个函数判定它们是否只需要一次(或者零次)编辑。

示例 1：
输入：
first = "pale"
second = "ple"
输出：True

示例 2：
输入：
first = "pales"
second = "pal"
输出：False
"""


"""
解法：按长度分情况，双指针最多允许一次跳过【最优解】

为什么慢：如果对每个位置都尝试插入、删除、替换再逐字比较，单次尝试就要
O(n)，每个位置都试一遍就退化成 O(n^2)，而且大量分支互相重复。真正需要
判断的只是"两串在哪个位置开始分叉，分叉之后还能不能对齐"。

换个角度：先固定 first 不短于 second。长度差超过 1 直接不可能；长度相等
时只允许一个位置不同；长度差 1 时，长串要能跳过恰好一个字符后与短串完全
相同。这三种情况可以合并成一次双指针扫描。

关键点：
  · 先交换保证 first 更长，避免为"谁长谁短"写两套逻辑
  · 长度差大于 1 必须立即返回 False，不能留到扫描里才发现
  · 遇到第一处不同时：等长则两个指针都前进，长度差 1 则只让长串前进
  · 第二处不同直接 False —— 此时已经用了两次编辑
  · 扫描结束仍返回 True：零处不同（零次编辑）或只有一处（一次编辑）

手推一遍（first = "pale"，second = "ple"）：
  p=p、a 与 l 不同 → 长串跳过 a，first 到 l，second 停在 l
  之后 l=l、e=e，扫描结束 → True ✔

反例（first = "ab"，second = "ba"）：
  a 与 b 不同，等长同时前进 → b 与 a 又不同 → False ✔

复杂度分析：
  时间 O(n)：两个指针各自单调前进，总步数不超过两串长度之和
  空间 O(1)：只用两个下标和一个"是否已经用过编辑"的标记
"""


class Solution:
    def oneEditAway(self, first: str, second: str) -> bool:
        m, n = len(first), len(second)

        # 统一成 first 更长，后续只需处理"等长"和"first 多一个"两种情形
        if m < n:
            return self.oneEditAway(second, first)

        # 长度差超过 1，一次编辑无论如何补不回来
        if m - n > 1:
            return False

        i = j = 0
        used = False
        while i < m and j < n:
            if first[i] == second[j]:
                i += 1
                j += 1
                continue

            # 第二处不同：已经用掉两次编辑
            if used:
                return False
            used = True

            # 等长是替换，两个指针都要走；first 更长是删除，只走 first
            if m == n:
                i += 1
                j += 1
            else:
                i += 1
        return True


# ===== 测试辅助 =====
def brute_one_edit_away(first: str, second: str) -> bool:
    """与双指针解独立的参照实现：按长度分情况，逐一尝试删除或统计不同字符。"""
    m, n = len(first), len(second)
    if m < n:
        return brute_one_edit_away(second, first)
    if m - n > 1:
        return False

    if m == n:
        return sum(x != y for x, y in zip(first, second)) <= 1

    # first 比 second 多一个字符：枚举删掉 first 的哪一位
    return any(first[:skip] + first[skip + 1:] == second for skip in range(m))


def check(first: str, second: str, want: bool) -> None:
    got = Solution().oneEditAway(first, second)
    assert got == want, (first, second, got, want)
    assert brute_one_edit_away(first, second) == want, (first, second, "brute")


def all_strings(max_len: int):
    out = [""]
    layer = [""]
    for _ in range(max_len):
        layer = [prefix + c for prefix in layer for c in "ab"]
        out.extend(layer)
    return out


if __name__ == "__main__":
    # 示例 1：删除一个字符
    check("pale", "ple", True)

    # 示例 2：长度差 2，直接不可能
    check("pales", "pal", False)

    # 零次编辑：完全相同
    check("pale", "pale", True)

    # 替换一个字符
    check("pale", "bale", True)

    # 两个位置不同，超出一次替换
    check("pale", "bake", False)

    # 插入一个字符（交换参数后即删除）
    check("ple", "pale", True)

    # 删除末尾字符
    check("abc", "ab", True)

    # 交换中间两位，两处不同
    check("abc", "acb", False)

    # 边界：两个空串，零次编辑
    check("", "", True)

    # 边界：空串与单字符，插入一次
    check("", "a", True)
    check("a", "", True)

    # 边界：空串与两字符，长度差 2
    check("", "ab", False)

    # 边界：单字符替换
    check("a", "b", True)

    # 反直觉：等长但顺序互换，不是一次编辑
    check("ab", "ba", False)

    # 反直觉：长串中间插入一个字符
    check("teacher", "treacher", True)

    # 反直觉：长度相同但差很多
    check("intention", "execution", False)

    # 穷举对拍：长度不超过 4、字母表 {a,b} 的全部字符串两两组合
    strings = all_strings(4)
    for a in strings:
        for b in strings:
            assert Solution().oneEditAway(a, b) == brute_one_edit_away(a, b)

    print("all tests passed")
