"""
============================================================================
面试题 16.11. 跳水板（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/diving-board-lcci/
分类：数学 · 等差数列枚举

你正在使用一堆木板建造跳水板。有两种类型的木板，其中长度较短的木板长度为
shorter，长度较长的木板长度为 longer。你必须正好使用 k 块木板。编写一个方法，
生成跳水板所有可能的长度。

返回的长度需要从小到大排列。

示例 1：
输入：
shorter = 1
longer = 2
k = 3
输出：[3,4,5,6]
解释：
可以使用 3 次 shorter，得到结果 3；使用 2 次 shorter 和 1 次 longer，
得到结果 4 。以此类推，得到最终结果。

提示：
- 0 < shorter <= longer
- 0 <= k <= 100000
"""
from typing import List


"""
解法：枚举长木板的使用块数【最优解】

为什么慢：如果真去枚举每块板选短还是选长，共 2^k 种组合，再逐个求和、
去重、排序，k 一大就指数爆炸。可这些组合里大量结果完全相同，重复计算
没有意义。

换个角度：最终长度只由"用了几块长木板"决定。设用了 i 块长木板，则短
木板用了 k-i 块，长度就是 shorter*(k-i) + longer*i。i 从 0 到 k，最多
k+1 种取值。

关键点：
  · k == 0 时一块板都用不了，返回空数组，不能返回 [0]
  · shorter == longer 时 k+1 种组合算出来是同一个数，必须只留一个
  · 两种板不相等时，i 每增加 1，长度就增加 longer-shorter > 0，
    因此 i 从 0 到 k 得到的 k+1 个长度严格递增，天然有序且互不相同
  · 先算乘法再相加，避免 shorter*(k-i) 先减后乘的写法产生负数下标

手推一遍（shorter = 1，longer = 2，k = 3）：
  i = 0：1*3 + 2*0 = 3
  i = 1：1*2 + 2*1 = 4
  i = 2：1*1 + 2*2 = 5
  i = 3：1*0 + 2*3 = 6
  得到 [3,4,5,6] ✔

复杂度分析：
  时间 O(k)：逐个计算 k+1 个长度
  空间 O(1)：除返回数组外只用常数个变量
"""


class Solution:
    def divingBoard(self, shorter: int, longer: int, k: int) -> List[int]:
        # 没有木板就没有长度，空列表而不是 [0]
        if k == 0:
            return []

        # 两种板等长时所有组合退化成同一个长度
        if shorter == longer:
            return [shorter * k]

        # i 块长木板 + (k-i) 块短木板，i 递增则长度严格递增
        return [shorter * (k - i) + longer * i for i in range(k + 1)]


# ===== 测试辅助 =====
def brute_board(shorter: int, longer: int, k: int) -> List[int]:
    """与公式解独立的参照实现：枚举 2^k 种选板方案，去重后排序。"""
    if k == 0:
        return []
    sums = set()
    for mask in range(1 << k):
        total = 0
        for bit in range(k):
            total += longer if (mask >> bit) & 1 else shorter
        sums.add(total)
    return sorted(sums)


def check(shorter: int, longer: int, k: int, want: List[int]) -> None:
    got = Solution().divingBoard(shorter, longer, k)
    assert got == want, (shorter, longer, k, got, want)
    assert brute_board(shorter, longer, k) == want, (
        shorter, longer, k, "brute")


if __name__ == "__main__":
    # 示例 1：三种板数组合，长度连续
    check(1, 2, 3, [3, 4, 5, 6])

    # 边界：k = 0，一块板都没有
    check(1, 2, 0, [])

    # 边界：两种板等长，k+1 种组合只留一个结果
    check(2, 2, 3, [6])

    # 边界：k = 1，恰好两个长度
    check(1, 2, 1, [1, 2])

    # 边界：shorter == longer 且 k = 0，空数组优先于去重
    check(5, 5, 0, [])

    # 边界：等长且 k = 1
    check(4, 4, 1, [4])

    # 一般情况：公差为 longer - shorter 的等差数列
    check(3, 7, 4, [12, 16, 20, 24, 28])

    # 一般情况：短长只差 1，结果是一段连续整数
    check(2, 3, 5, [10, 11, 12, 13, 14, 15])

    # 较大长度：验证不会因为中间量产生负数或错位
    check(1, 100000, 1, [1, 100000])

    # 小规模穷举对拍：覆盖所有 k <= 8 的组合
    for shorter in range(1, 5):
        for longer in range(shorter, 7):
            for k in range(9):
                assert (Solution().divingBoard(shorter, longer, k)
                        == brute_board(shorter, longer, k))

    print("all tests passed")
