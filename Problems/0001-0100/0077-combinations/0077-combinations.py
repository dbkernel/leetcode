"""
============================================================================
77. 组合（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/combinations/
分类：回溯 · 组合枚举与剪枝

给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。

你可以按 任何顺序 返回答案。

示例 1：
输入：n = 4, k = 2
输出：
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]

示例 2：
输入：n = 1, k = 1
输出：[[1]]

提示：
- 1 <= n <= 20
- 1 <= k <= n
"""
from itertools import combinations


"""
解法一：回溯，不剪枝【非最优】

为什么慢：每次从当前位置往后逐个试，凑够 k 个就收集。这条递归树会把"长度
不到 k 的每一个前缀"都展开一遍 —— 比如 n = 4、k = 2 时，长度 0、1、2 的
前缀全都会走。落在深处的那些前缀之所以走不下去，只是因为剩下的数已经不够
凑满 k 个，可代码要到下次进入循环才发现"没得选"，白白展开了一整棵子树。

换个角度：既然"剩下的数不够"是唯一让分支必然失败的原因，那就把这件事提前
算出来：在循环开始之前就知道本层最多能取到哪个下标，超过它的分支根本不必
进入，递归树自然只剩下"一定能走到叶子"的路径。

关键点：见解法二。

手推一遍：
以 n = 4、k = 2 为例：
  第 0 层取 1，第 1 层取 2、3、4 → [1,2]、[1,3]、[1,4]
  第 0 层取 2，第 1 层取 3、4 → [2,3]、[2,4]
  第 0 层取 3，第 1 层取 4 → [3,4]
  第 0 层取 4 —— 进入后一个数都取不到，这个分支是纯浪费
  共 6 个 = C(4,2) ✔

复杂度分析：
  时间 O(Σ_{j=0}^{k} C(n,j) + C(n,k)·k)：长度 j 的前缀共 C(n,j) 个，全部展开；
    每个叶子还要 O(k) 把 path 拷进答案
  空间 O(k)：递归栈与 path 都不超过 k 层
"""


class Solution1:
    def combine(self, n: int, k: int) -> list[list[int]]:
        res: list[list[int]] = []
        path: list[int] = []

        def dfs(start: int) -> None:
            if len(path) == k:
                res.append(path[:])
                return
            # 上界写成 n 而不是能走到叶子的最大值，k 大时会有大量空转分支
            for i in range(start, n + 1):
                path.append(i)
                dfs(i + 1)
                path.pop()

        dfs(1)
        return res


"""
解法二：回溯 + 剩余元素不足时提前剪枝【最优解】

为什么慢：解法一在循环里对每个 i 都无条件进入下一层，而"下一层还能不能取到
数"只取决于 i 有多大。k 越接近 n，被浪费的分支占比越高：k = n 时整棵树
有 2^n 个前缀，可答案只有 1 个。

换个角度：把"本层还能取哪个数"当成一个可以算出来的区间，而不是试出来。
假设当前 path 里已有 len 个元素，还差 need = k - len 个；从 i 开始（含 i）
到 n 一共有 n - i + 1 个数可选。要凑满 need 个，必须满足
    n - i + 1 >= need
解出 i <= n - need + 1，这就是本层循环的上界。

关键点：
  · 剪枝条件写成 i <= n - (k - len(path)) + 1，其中 k - len(path)
    是"还差几个"，n - 还差几个 + 1 是本层可取的最大值
  · 末尾那个 +1 不能漏：它正是"刚好凑满"的那一个 i。少了它，上界变成
    n - need，会把"i 之后恰好剩 need-1 个数、加上 i 刚好凑满 k 个"的分支
    整个丢掉，答案少一批
  · 剪枝只缩小循环上界，不改递归出口 —— 出口仍然是 len(path) == k
  · 递归传 i + 1 而不是 start + 1：组合按值递增，同一层不能回头取更小的数

手推一遍：
以 n = 4、k = 2 为例：
  len = 0，need = 2，上界 4 - 2 + 1 = 3，本层只取 1、2、3
    取 1（len = 1，need = 1，上界 4 - 1 + 1 = 4）→ 第 1 层取 2、3、4
    取 2（need = 1，上界 4）→ 第 1 层取 3、4
    取 3（need = 1，上界 4）→ 第 1 层取 4
  取 4 的分支在第 0 层就被挡掉了，没有进入
  答案 6 个 = C(4,2) ✔

复杂度分析：
  时间 O(Σ_{j=0}^{k} C(n-k+j, j) + C(n,k)·k)：能展开的前缀必须满足
    "剩下的数够补满"，长度 j 的前缀只有 C(n-k+j, j) 个，是解法一的前缀集合
    的真子集；k = n 时解法一要展开 2^n 个前缀，这里只剩 n + 1 个
  空间 O(k)：递归栈与 path 都不超过 k 层
"""


class Solution2:
    def combine(self, n: int, k: int) -> list[list[int]]:
        res: list[list[int]] = []
        path: list[int] = []

        def dfs(start: int) -> None:
            if len(path) == k:
                res.append(path[:])
                return
            # 还差 need 个，本层最多取到 n - need + 1，把无解分支整棵砍掉
            need = k - len(path)
            for i in range(start, n - need + 2):
                path.append(i)
                dfs(i + 1)
                path.pop()

        dfs(1)
        return res


# 组合集合按题面可以任意顺序返回，内外层都排序后整体比较
def normalize(v: list[list[int]]) -> list[list[int]]:
    return sorted(sorted(inner) for inner in v)


def check(n: int, k: int) -> None:
    want = normalize([list(c) for c in combinations(range(1, n + 1), k)])
    assert normalize(Solution1().combine(n, k)) == want, f"解法一挂在 ({n},{k})"
    assert normalize(Solution2().combine(n, k)) == want, f"解法二挂在 ({n},{k})"


if __name__ == "__main__":
    # 示例 1：C(4,2) = 6
    got = normalize(Solution2().combine(4, 2))
    assert got == normalize([[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]])
    assert len(got) == 6

    # 示例 2：n = k = 1，答案只有一个 [1]
    check(1, 1)
    assert Solution1().combine(1, 1) == [[1]]
    assert Solution2().combine(1, 1) == [[1]]

    # 边界 k = n：只有一种组合，剪枝版应只剩一条链
    check(5, 5)
    check(20, 20)
    assert Solution2().combine(4, 4) == [[1, 2, 3, 4]]

    # 边界 k = 1：答案就是 n 个单元素集合
    check(4, 1)
    assert len(Solution2().combine(20, 1)) == 20

    # 边界 k 居中：C(6,3) = 20、C(20,10) = 184756
    check(6, 3)
    assert len(Solution2().combine(20, 10)) == 184756

    print("all tests passed")
