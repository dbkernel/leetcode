"""
============================================================================
LCR 165 / 剑指 Offer 46. 解密数字（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/ba-shu-zi-fan-yi-cheng-zi-fu-chuan-lcof/
分类：动态规划 · 计数型线性递推（滚动两变量）

说明：本题原名「把数字翻译成字符串」，leetcode.cn 现已改写为「解密数字」：
参数名为 ciphertext（整数）、函数名为 crackNumber，规则仍是 0-25 对应
a-z、求方案数，示例换成 216612 → 6。
============================================================================

现有一串神秘的密文 ciphertext，经调查，密文的特点和规则如下：

- 密文由非负整数组成
- 数字 0-25 分别对应字母 a-z

请根据上述规则将密文 ciphertext 解密为字母，并返回共有多少种解密结果。

示例 1：
输入：ciphertext = 216612
输出：6
解释：216612 解密后有 6 种不同的形式，分别是 "cbggbc"，"vggbc"，"vggm"，
"cbggm"，"cqgbc" 和 "cqgm"

提示：
0 <= ciphertext < 2^31
"""

"""
解法：动态规划 + 滚动两变量【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：答案是逐位推出来的，
每一位只跟前两位有关，时间 O(d)、额外变量 O(1)。

为什么慢：穷举所有切分方式是 2^d 级别的 —— 每一位都可以选择「自己解」或
「和右边合并成两位」，分支翻倍。而这些分支里有大量重复：走到第 i 位之后
的方案数只取决于前两位的状态，跟前面怎么切过来的无关。

换个角度：从左往右按位推进。第 i 个数字有两种来源 —— 自己单独解一位，
或者与前一位组成两位数一起解。这两种来源互斥（一位不可能既单独又合并），
所以第 i 位的方案数就是这两个来源之和。

关键点：
  · 两位数只允许 10..25：上限 25 是字母 z，下限 10 排除 "06" 这种带前导
    零的组合。判断必须写成 10 <= two <= 25，只写 two <= 25 会把 "06"
    "00" 也放进来
  · 单个数字永远合法（0 也对应字母 a），所以 cur 的初值就是 prev1
  · 滚动顺序：先算出 cur，再 prev2 = prev1、prev1 = cur；写反会把刚算出
    的值当成上一位
  · 输入是整数，先转成字符串再按下标扫，比反复取模求余更直白
  · ciphertext = 0 时循环一次都不执行，返回 1（"a" 这一种解法）

以 ciphertext = 216612 为例：
  第 1 位 "2"：1 种
  第 2 位 "21" 在 10..25 内：1 + 1 = 2 种
  第 3 位 "16" 在区间内：1 + 2 = 3 种
  第 4 位 "66" 不在区间内：3 种（只能单独解）
  第 5 位 "61" 不在区间内：3 种
  第 6 位 "12" 在区间内：3 + 3 = 6 种 ✔

复杂度分析：
  时间 O(d)：d 为位数，每位只做常数次比较
  空间 O(d)：先要把整数转成字符串；滚动变量本身只有两个 int
"""


class Solution:
    def crackNumber(self, ciphertext: int) -> int:
        s = str(ciphertext)

        prev2 = 1  # dp[i-2]
        prev1 = 1  # dp[i-1]：单个数字总能独立成字母
        for i in range(1, len(s)):
            two = int(s[i - 1]) * 10 + int(s[i])
            cur = prev1  # 只把第 i 位单独解
            if 10 <= two <= 25:
                cur += prev2
            prev2, prev1 = prev1, cur
        return prev1


# 指数级枚举：每一位要么单独解，要么与后一位组成两位数，用来对拍
def brute_decode(s: str, i: int) -> int:
    if i == len(s):
        return 1

    ways = brute_decode(s, i + 1)
    if i + 1 < len(s):
        two = int(s[i]) * 10 + int(s[i + 1])
        if 10 <= two <= 25:
            ways += brute_decode(s, i + 2)
    return ways


def check(ciphertext: int, want: int) -> None:
    assert Solution().crackNumber(ciphertext) == want, ciphertext


if __name__ == "__main__":
    # 示例：216612 有 6 种解
    check(216612, 6)

    # 题目要求的用例
    check(12258, 5)
    check(0, 1)
    check(100, 2)

    # 关键点用例：26 超出 10..25，只能逐位解
    check(26, 1)
    check(25, 2)

    # 关键点用例：106 里的 "06" 带前导零，不能当成 6 合并
    check(106, 2)
    check(10, 2)
    check(101, 2)

    # 反直觉：110 的三种解来自 1|1|0、11|0、1|10
    check(110, 3)

    # 位数上界：2^31 - 1 共 10 位，只有 "21" 一处能合并
    check(2147483647, 3)

    # 对拍：0..10000 全部与指数级枚举比对
    for n in range(10001):
        assert Solution().crackNumber(n) == brute_decode(str(n), 0), n

    print("all tests passed")
