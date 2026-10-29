"""
============================================================================
面试题 17.11. 单词距离（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-closest-lcci/
分类：双指针 · 一次遍历记录最近下标

有个内含单词的超大文本文件，给定任意两个不同的单词，找出在这个文件中这两个
单词的最短距离(相隔单词数)。如果寻找过程在这个文件中会重复多次，而每次寻找
的单词不同，你能对此优化吗?

示例：
输入：words = ["I","am","a","student","from","a","university","in","a","city"],
      word1 = "a", word2 = "student"
输出：1

提示：
- words.length <= 100000
"""


"""
解法一：枚举两个词出现位置的所有组合【非最优】

为什么慢：把 word1 出现的每个下标和 word2 出现的每个下标配对，取下标差的
最小值。两个词都很常见时（比如 "a" 与 "the"），配对数是 O(k1*k2)，最坏
O(n^2)，10^5 规模的文本根本跑不完。慢的原因是它没有用上"位置天然有序"这个
事实：一旦某个下标差已经确定，绝大多数配对都只可能更差。

换个角度：把所有配对的差值看成一个有序结构。两个词的下标各自升序排列，
离得最近的一对一定出现在"两个序列交错的地方"，于是可以像归并两个有序数组
一样，两个指针并进，每步只前进一侧。

关键点：
  · 两词可能相同，此时 i == j 的配对下标差为 0，会把答案直接压成 0，
    必须跳过同一位置，只允许"两个不同位置"配对
  · 差值就是下标之差的绝对值，题面问的"相隔单词数"按官方示例是 |i-j|，
    不是 |i-j|-1
  · 若某个词没出现，题面不会给出这种输入；这里返回 0 只是不让极大值
    外泄，不要把 0 当成合法答案

以示例（words 中含 a 于下标 2、5、8，student 于下标 3）为例：
  (2,3) 差 1 → 答案 1 ✔

复杂度分析：
  时间 O(k1*k2)：最坏 O(n^2)
  空间 O(1)：只存两个下标

解法二：哈希表存每个词的出现位置 + 双指针归并【非最优】

官方进阶解法。先扫一遍文本，把"词 -> 出现位置列表"存进哈希表，查询时取出
两个词的列表，再用双指针求两个有序数组的最小差（与「最小差」那题同一套
收割逻辑）。文本不变而查询很多次时，建表只付一次钱，每次查询只付 O(k1+k2)，
这就是"重复查询"的优化点。

关键点：
  · 单次查询时它并不比解法三好：建表已经花了 O(n) 时间和 O(n) 空间，
    而解法三同样 O(n) 时间却只要 O(1) 额外空间，所以单次场景判它非最优
  · 两词相同时不能拿同一个列表和自己的同一个位置去比，退化为"同一列表中
    相邻两个元素的最小间隔"
  · 取到 0 就可以停：下标差是非负整数，0 已是下界

以示例为例：
  建表后 a → [2,5,8]，student → [3]
  双指针：(2,3) 差 1 → 0 之外没有更小，答案 1 ✔
  若查 a 与 a：列表 [2,5,8] 相邻间隔 3、3，答案 3

复杂度分析：
  时间 O(n + k1 + k2)：建表 O(n)，单次查询 O(k1+k2)
  空间 O(n)：位置表要存下每个词的全部下标

解法三：一次遍历，同时记录两个词最近一次出现的下标【最优解】

官方解法。只扫一遍文本，维护 last1、last2 两个"最近出现的下标"。当前词是
word1 时，它与最近的 word2 构成的差值就是 i - last2，用它更新答案后再把
last1 挪到 i。扫完即可，不需要回答任何"以后再查"的问题。

关键点：
  · 更新答案必须在"挪动当前词的下标之前"做，否则会拿自己和自己比出 0
  · 每遇到一次出现只需与"另一个词最近的一次"比较：更早的出现只会给出
    更大的下标差
  · 两词相同时（题面保证不同，但接口允许），上面的规则会退化成自己比自己，
    所以单独分支处理：只用 last 记录上一次出现的位置，答案取相邻两次出现
    的最小间隔；只看相邻是因为非相邻的间隔一定更大
  · 某个词只在文末出现一次时，last 一直是 -1，不会被误当成差值参与比较

以示例（a 于 2、5、8，student 于 3）为例：
  i=2 遇到 a：last2 还是 -1，跳过；last1=2
  i=3 遇到 student：i-last1 = 1 → 答案 1；last2=3
  i=5 遇到 a：i-last2 = 2 > 1；last1=5
  i=8 遇到 a：i-last2 = 5 > 1；last1=8
  扫完返回 1 ✔

复杂度分析：
  时间 O(n)：每个单词只处理一次，比较是常数次
  空间 O(1)：只存两个下标和答案
"""


class Solution1:
    def findClosest(self, words: list, word1: str, word2: str) -> int:
        n = len(words)
        best = None
        for i in range(n):
            for j in range(n):
                # 同一位置配对在两词相同时会给出假的 0
                if i == j:
                    continue
                if words[i] == word1 and words[j] == word2:
                    d = abs(i - j)
                    if best is None or d < best:
                        best = d
        return 0 if best is None else best


class Solution2:
    def findClosest(self, words: list, word1: str, word2: str) -> int:
        # 一次建表，之后每次查询都只读这张表
        pos = {}
        for i, w in enumerate(words):
            pos.setdefault(w, []).append(i)

        p1 = pos.get(word1, [])
        p2 = pos.get(word2, [])

        # 相同词时同一下标会与自己配对，退化为求相邻出现的最小间隔
        if word1 == word2:
            best = None
            for i in range(1, len(p1)):
                d = p1[i] - p1[i - 1]
                if best is None or d < best:
                    best = d
            return 0 if best is None else best

        i, j = 0, 0
        best = None
        while i < len(p1) and j < len(p2):
            d = abs(p1[i] - p2[j])
            if best is None or d < best:
                best = d
            if best == 0:
                break

            # 小的一侧往后挪只会更远，所以这一侧的这个下标记可以判死
            if p1[i] < p2[j]:
                i += 1
            else:
                j += 1
        return 0 if best is None else best


class Solution3:
    def findClosest(self, words: list, word1: str, word2: str) -> int:
        n = len(words)

        # 两词相同时两个 last 会指向同一下标，自己比自己恒为 0，必须分开算
        if word1 == word2:
            last = -1
            best = None
            for i in range(n):
                if words[i] != word1:
                    continue
                # 非相邻的两处间隔一定更大，所以只和上一次出现比
                if last >= 0 and (best is None or i - last < best):
                    best = i - last
                last = i
            return 0 if best is None else best

        last1 = last2 = -1
        best = None
        for i in range(n):
            if words[i] == word1:
                # 先算差值再挪 last1：顺序颠倒就成了自己减自己
                if last2 >= 0 and (best is None or i - last2 < best):
                    best = i - last2
                last1 = i
            elif words[i] == word2:
                if last1 >= 0 and (best is None or i - last1 < best):
                    best = i - last1
                last2 = i
        return 0 if best is None else best


# ===== 测试辅助 =====
def check(words: list, word1: str, word2: str, want: int) -> None:
    got = Solution1().findClosest(words, word1, word2)
    assert got == want, f"brute({word1},{word2}) = {got}, want {want}"
    got = Solution2().findClosest(words, word1, word2)
    assert got == want, f"index-map({word1},{word2}) = {got}, want {want}"
    got = Solution3().findClosest(words, word1, word2)
    assert got == want, f"one-pass({word1},{word2}) = {got}, want {want}"


if __name__ == "__main__":
    text = ["I", "am", "a", "student", "from", "a",
            "university", "in", "a", "city"]

    # 示例：a 在下标 2，student 在下标 3，差 1
    check(text, "a", "student", 1)

    # 顺序无关：把两个词换过来，答案仍是 1
    check(text, "student", "a", 1)

    # 两词相同：a 出现在下标 2、5、8，最近两处间隔 3
    check(text, "a", "a", 3)

    # 只出现一次：I 在下标 0，city 在下标 9
    check(text, "I", "city", 9)

    # 各自只出现一次且相邻
    check(text, "student", "from", 1)

    # 相邻的两次出现：差 1（0 只会来自"同一下标自己比自己"，必须排除）
    check(["x", "a", "a", "y"], "a", "a", 1)

    # 相同词只出现一次：没有第二个位置可比，按约定返回 0
    check(["x", "a", "y"], "a", "a", 0)

    # 反直觉：最近的 b 在 a 之前，只记录"后出现的那次"会漏掉更近的候选
    check(["a", "b", "a"], "a", "b", 1)

    print("all tests passed")
