"""
============================================================================
739. 每日温度（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/daily-temperatures/
分类：单调栈 · 下一个更大元素

给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer ，
其中 answer[i] 是指对于第 i 天，下一个更高温度出现在几天后。如果气温在这
之后都不会升高，请在该位置用 0 来代替。

示例 1：
输入: temperatures = [73,74,75,71,69,72,76,73]
输出: [1,1,4,2,1,1,0,0]

示例 2：
输入: temperatures = [30,40,50,60]
输出: [1,1,1,0]

示例 3：
输入: temperatures = [30,60,90]
输出: [1,1,0]

提示：
- 1 <= temperatures.length <= 10^5
- 30 <= temperatures[i] <= 100
"""
from typing import List


"""
解法一：反向遍历 + 温度索引表【最优解，与解法二并列】

为什么慢：最朴素的做法是对每一天 i 向后逐个比较，直到遇到更高的温度，
最坏（严格递减）每天都要扫完剩下的全部元素，退化成 O(n^2)。要提速，就得
让"找一个更高温度出现在哪"不必真的扫一遍后缀。

换个角度：反向遍历时，i 右侧的每一天都已经处理过，于是可以只记一张表
next[t] = 温度 t 在 i 右侧最近一次出现的下标。那么"比 temperatures[i] 更
高的温度里，最近的那个位置"就是从 temperatures[i]+1 到上界取 next 的最小
值 —— 后缀里的信息被压缩成 101 个槽位。

关键点：
  · 必须反向遍历：先查表再更新 next[temperatures[i]]，顺序颠倒就会查到
    自己，答案恒为 0
  · 要对所有更高的温度取最小下标，而不是取第一个非空槽位：更靠左的更高
    温度才是"下一个"
  · 上界取题面给的 100，next 开 101 个槽位（下标 0..100）即可
  · 找不到时 next 保持 INF，ans[i] 自然留在 0

以示例 1（temperatures = [73,74,75,71,69,72,76,73]）为例，只看最后三步：
  i=5（72）：更高温度里 next[76]=6 最小 → ans[5]=1
  i=4（69）：next[72]=5 与 next[76]=6 取小 → ans[4]=1
  i=3（71）：next[72]=5 → ans[3]=2 ✔

复杂度分析：
  时间 O(n·m)：m = 101 是温度区间长度，每天都要扫一遍更高的温度
  空间 O(m)：只有 101 个槽位的 next 表
  与解法二的关系：温度区间 m ≤ 100 是题面常数，两条解法时间同为
  O(n)；本解空间 O(m)、比单调栈的 O(n) 小，但时间多一个 m 倍的
  常数因子。一个更省空间、一个更快，故并列最优

解法二：单调栈（栈里留下"还没等到更高温度"的下标）【最优解】

为什么慢：解法一每天都要在 101 个温度上取最小，虽然 m 固定，但仍然是 n
的乘数；而很多温度根本没出现过，扫它们纯属浪费。

换个角度：与其问"i 的下一个更高温度在哪"，不如换个方向 —— 让后来的温度
去"结算"更早的温度。维护一个下标栈，栈内下标对应的温度从栈底到栈顶严格
递减；新来的一天如果比栈顶高，栈顶那天就等到了答案，弹栈并填 ans。每个
下标只会进栈一次、出栈一次，总操作数与温度范围无关。

关键点：
  · 栈里存下标而不是温度值：答案要的是天数差 i - prev_index
  · 用 while 而不是 if：新温度可能一次性结算掉栈里连续多个更低的日子
  · 比较条件是严格大于 temperatures[i] > temperatures[stack[-1]]：
    相等不算"升高"，等温的日子必须留在栈里等更晚的更高温度
  · 结算完再把 i 入栈；遍历结束后栈里剩下的下标就是"之后不会升高"，
    ans 保持 0，不需要额外清空逻辑

以示例 1（temperatures = [73,74,75,71,69,72,76,73]）为例：
  i=0 入栈 [0(73)]
  i=1：74 > 73，弹出 0，ans[0]=1，入栈 [1(74)]
  i=3、i=4：71、69 都更冷，直接入栈 [2(75),3(71),4(69)]
  i=5：72 连续弹出 4、3，ans[4]=1、ans[3]=2，入栈 [2(75),5(72)]
  i=6：76 连续弹出 5、2，ans[5]=1、ans[2]=4，入栈 [6(76)]
  i=7：73 更冷，入栈；结束时 ans = [1,1,4,2,1,1,0,0] ✔

复杂度分析：
  时间 O(n)：每个下标恰好入栈一次、出栈至多一次，while 的总次数是 O(n)
  空间 O(n)：严格递减时 n 个下标全留在栈里
"""


class Solution1:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        n = len(temperatures)
        ans = [0] * n
        # next[t]：温度 t 在当前位置右侧最近一次出现的下标
        next_pos: list = [None] * 101

        for i in range(n - 1, -1, -1):
            # 先查表：此时 next_pos 里只有 i 右侧的信息，不会查到自己
            warmer_index = None
            for t in range(temperatures[i] + 1, 101):
                pos = next_pos[t]
                if pos is not None and (
                    warmer_index is None or pos < warmer_index
                ):
                    warmer_index = pos
            if warmer_index is not None:
                ans[i] = warmer_index - i
            # 查完再登记自己，供更左侧的日子使用
            next_pos[temperatures[i]] = i
        return ans


class Solution2:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        n = len(temperatures)
        ans = [0] * n
        # 存下标；对应温度从栈底到栈顶严格递减
        stack: List[int] = []

        for i in range(n):
            # 严格大于才结算，等温的日子继续等待
            while stack and temperatures[i] > temperatures[stack[-1]]:
                prev_index = stack.pop()
                ans[prev_index] = i - prev_index
            stack.append(i)
        return ans


# 独立的 O(n^2) 对照答案，只用于交叉验证，不参与题解
def brute_force(temperatures: List[int]) -> List[int]:
    n = len(temperatures)
    ans = [0] * n
    for i in range(n):
        for j in range(i + 1, n):
            if temperatures[j] > temperatures[i]:
                ans[i] = j - i
                break
    return ans


# 两个解法都要过同一组用例，期望值由题面定义独立给出
def check(temperatures: List[int], want: List[int]) -> None:
    for cls in (Solution1, Solution2):
        got = cls().dailyTemperatures(list(temperatures))
        assert got == want, f"{cls.__name__}({temperatures}) = {got}"


# 随机交叉验证：以 O(n^2) 暴力为基准
def cross_check() -> None:
    seed = 20240607
    for _ in range(200):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        n = (seed >> 16) % 40 + 1
        temperatures = []
        for _ in range(n):
            seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
            # 温度范围是 30..100，容易撞出大量重复值与等温段
            temperatures.append((seed >> 16) % 71 + 30)
        check(temperatures, brute_force(temperatures))


if __name__ == "__main__":
    # 示例 1：中间有一段"回落后才等来更高温度"的曲折
    check([73, 74, 75, 71, 69, 72, 76, 73], [1, 1, 4, 2, 1, 1, 0, 0])

    # 示例 2：严格递增，每天都是 1 天后升温
    check([30, 40, 50, 60], [1, 1, 1, 0])

    # 示例 3：严格递增的另一样例
    check([30, 60, 90], [1, 1, 0])

    # 边界：只有一天，等不到升温
    check([50], [0])

    # 边界：严格递减，全为 0 —— 栈里会一路堆到 n 个元素
    check([90, 80, 70, 60], [0, 0, 0, 0])

    # 反直觉：全部等温，相等不算"升高"，答案全 0
    check([50, 50, 50], [0, 0, 0])

    # 反直觉：最低温在最后一格时才结算，且一次弹多层
    check([55, 38, 53, 81, 61, 93, 97, 35], [3, 1, 1, 2, 1, 1, 0, 0])

    # 边界：温度只取上下界
    check([30, 100, 30], [1, 0, 0])

    # 随机交叉验证，含大量重复值的等温段
    cross_check()

    print("all tests passed")
