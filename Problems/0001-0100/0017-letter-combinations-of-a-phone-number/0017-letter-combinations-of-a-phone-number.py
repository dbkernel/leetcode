"""
============================================================================
17. 电话号码的字母组合（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
分类：回溯 · 多叉树按层枚举

给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按
任意顺序 返回。

给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。

示例 1：
输入：digits = "23"
输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]

示例 2：
输入：digits = ""
输出：[]

示例 3：
输入：digits = "2"
输出：["a","b","c"]

提示：
- 0 <= digits.length <= 4
- digits[i] 是范围 ['2', '9'] 的一个数字
"""


"""
解法：回溯，每层确定一个数字的字母【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：答案最多 4^n 个、每个
长 n，光是写出去就要 Ω(4^n·n)，任何解法都躲不开这个量级。

为什么慢：要列出全部组合，最直白的写法是"第 1 个数字的每个字母后面接上
第 2 个数字的每个字母，再接上第 3 个……"，也就是写几层嵌套循环。可 digits
的长度是运行时才知道的，循环层数无法在源码里定下来，这条路写不出来。

换个角度：把"拼上一个字母"看成树的一层。第 i 层从 digits[i] 对应的字母集合
里挑一个字符挂到路径末尾，走到第 n 层路径长度就等于数字个数，正好是一个
答案。深度优先遍历这棵隐式多叉树，进入节点时压入字符、回溯时弹出，
路径本身就是当前组合，不需要额外的拼接。

关键点：
  · 递归出口是 pos == len(digits)，此时 path 长度必然等于数字个数，
    所以不需要额外判断组合是否"够长"
  · 空输入必须在进入递归之前单独返回空列表 —— 走到出口会产出空串，
    而题面要的是 [] 而不是 [""]
  · 同一层内字母互不相同，因此无需像含重复元素的组合题那样做同层跳过，
    这里的去重压力是零
  · 把字符先存进 list 再 "".join，比反复 `path += c` 少建 n 个中间串

手推一遍：
以示例 1（digits = "23"）为例：
  第 0 层取 'a'，第 1 层依次取 'd'/'e'/'f' → "ad"、"ae"、"af"
  第 0 层取 'b'，第 1 层依次取 'd'/'e'/'f' → "bd"、"be"、"bf"
  第 0 层取 'c'，第 1 层依次取 'd'/'e'/'f' → "cd"、"ce"、"cf"
  共 3 × 3 = 9 个 ✔

复杂度分析：
  时间 O(4^n · n)：每个数字最多 4 个字母，叶子最多 4^n 个，每个叶子要把
    长度为 n 的 path 拼成字符串，拼接是 O(n)
  空间 O(n)：递归栈与 path 都只随数字个数线性增长；返回值不计入
"""

# 下标就是键位数字，0 和 1 留空，省掉一次减法映射
KEY_LETTERS = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv",
               "wxyz"]


class Solution:
    def letterCombinations(self, digits: str) -> list[str]:
        # 空输入没有"层"可走，直接返回空列表，避免产出空串
        if not digits:
            return []

        res: list[str] = []
        path: list[str] = []

        def dfs(pos: int) -> None:
            if pos == len(digits):
                # 走到叶子时 path 已经是完整答案，拼一份再存
                res.append("".join(path))
                return
            # 本层只决定"第 pos 个数字取哪个字母"，不跨层取，故无 start 下标
            for ch in KEY_LETTERS[int(digits[pos])]:
                path.append(ch)
                dfs(pos + 1)
                # 弹出刚压入的字符，让同层下一个字母从干净状态出发
                path.pop()

        dfs(0)
        return res


# 题面允许答案按任意顺序返回，统一排序后再逐项比较
def normalize(v: list[str]) -> list[str]:
    return sorted(v)


def check(digits: str, want: list[str]) -> None:
    got = Solution().letterCombinations(digits)
    assert normalize(got) == normalize(want), f"{digits}: {got} != {want}"


if __name__ == "__main__":
    # 示例 1：两个数字各 3 个字母，共 9 个组合
    check("23", ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"])

    # 示例 2：空输入要返回空列表，而不是含一个空串的列表
    assert Solution().letterCombinations("") == []

    # 示例 3：单个数字
    check("2", ["a", "b", "c"])

    # 边界：7 和 9 对应 4 个字母，数量随输入变化而不是固定 3 的幂
    check("7", ["p", "q", "r", "s"])
    assert len(Solution().letterCombinations("79")) == 16

    # 边界：长度取上限 4，4^4 = 256 个，且每个组合长度都是 4
    big = Solution().letterCombinations("9999")
    assert len(big) == 256
    assert all(len(v) == 4 for v in big)
    # 同一个 token 不重复：4 个数字全取 '9' 只应得到 'w'/'x'/'y'/'z' 的排列
    assert set(big) == {"".join([a, b, c, d])
                        for a in "wxyz" for b in "wxyz"
                        for c in "wxyz" for d in "wxyz"}

    print("all tests passed")
