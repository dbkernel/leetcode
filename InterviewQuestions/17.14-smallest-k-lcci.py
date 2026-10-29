"""
============================================================================
面试题 17.14. 最小 K 个数（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/smallest-k-lcci/
分类：排序 / 堆 / 快速选择（快排思想）

设计一个算法，找出数组中最小的 k 个数。以任意顺序返回这 k 个数均可。

示例：
输入： arr = [1,3,5,7,2,4,6,8], k = 4
输出： [1,2,3,4]

提示：
0 <= len(arr) <= 100000
0 <= k <= min(100000, len(arr))
"""
import heapq
import random


"""
解法一：直接排序后取前 k 个【非最优】

官方方法一。对原数组从小到大排序，前 k 个就是答案。

关键点：
  · 题目允许任意顺序返回，因此排序结果直接截断即可
  · 排序会把整个数组都处理好，但我们只关心最小的 k 个，做了多余的工作
  · k == 0 时要返回空数组，别越界

复杂度分析：
  时间 O(n log n)：排序
  空间 O(n)：sorted 复制出的列表

解法二：大小为 k 的大根堆【非最优】

官方方法二。用一个大根堆实时维护「当前最小的 k 个数」：
  · 先把前 k 个数入堆；
  · 之后每遇到一个比堆顶更小的数，就弹出堆顶、把新数入堆；
  · 扫描结束后，堆里的 k 个数就是答案。

关键点：
  · 必须用大根堆（堆顶是当前 k 小里的最大值），这样才方便「淘汰最大的那个」；
    Python 的 heapq 是小根堆，所以入堆时存相反数
  · 只有比堆顶更小才替换；相等时替换与不替换结果都对，不必处理
  · k == 0 时堆为空，必须提前返回，否则堆顶无定义
  · 适合 n 很大、k 很小（或数据流无法整体载入内存）的场景

复杂度分析：
  时间 O(n log k)：每个元素至多一次 O(log k) 的堆操作
  空间 O(k)：堆的容量

解法三：快排思想（随机化快速选择）【最优解】

官方方法三。快排的划分（partition）每做一次，就能确定主元的最终位置 pos，
并且 pos 左边都不大于它、右边都不小于它。因此：
  · pos 左边的元素个数（含 pos）恰好是 pos - l + 1 个「当前区间最小」的数；
  · 若这个数量正好等于 k，说明左边这段就是要找的 k 个数，直接结束；
  · 若 k 更小，只需递归左半边；
  · 若 k 更大，说明左边这段都要，还需在右半边再找 k - num 个。
每次只递归一侧，总代价期望 O(n)。

关键点：
  · 划分以区间末尾 nums[r] 为主元；随机化是把「随机选中的下标」先换到末尾，
    避免在有序/近似有序数据上退化成 O(n^2)
  · partition 用「小于等于主元就往前换」的单向写法，最后把主元换到 i + 1
  · 递归返回后，数组前 k 个位置就是答案（内部无序，但集合正确）
  · k == 0 要提前返回空数组；l >= r 是递归出口
  · 判方向时用的是「左边个数 num = pos - l + 1」，不是绝对下标 pos

以 arr = [1,3,5,7,2,4,6,8]，k = 4 为例：
  随机化划分一次，可能得到 pos 落在 3 附近，左边 {1,3,2,4} 共 4 个 → num == k，结束
  返回前 4 个（某种排列，排序后是 [1,2,3,4]）
  若某次 num > k，则只在左半边继续找 k 个；若 num < k，则右半边再找 k - num 个

复杂度分析：
  时间 期望 O(n)，最坏 O(n^2)（划分点极端时）
  空间 期望 O(log n)，最坏 O(n)（递归栈）

三解对比：
  · 排序：实现最简单，O(n log n)，n = 1e5 完全够用
  · 大根堆：O(n log k)，适合 k 远小于 n 或流式数据
  · 快速选择：期望 O(n)，是本题的渐进最优解
"""


class Solution:
    # ---------- 解法一：排序 ----------
    def smallestK_sort(self, arr: list[int], k: int) -> list[int]:
        if k <= 0:
            return []
        return sorted(arr)[:k]

    # ---------- 解法二：大小为 k 的大根堆（取反存小根堆） ----------
    def smallestK_heap(self, arr: list[int], k: int) -> list[int]:
        if k <= 0:
            return []

        hp = [-x for x in arr[:k]]
        heapq.heapify(hp)
        for x in arr[k:]:
            if -hp[0] > x:
                heapq.heappop(hp)
                heapq.heappush(hp, -x)
        return [-x for x in hp]

    # ---------- 解法三：随机化快速选择（最优解） ----------
    def _partition(self, nums: list[int], l: int, r: int) -> int:
        pivot = nums[r]
        i = l - 1
        for j in range(l, r):
            if nums[j] <= pivot:
                i += 1
                nums[i], nums[j] = nums[j], nums[i]
        nums[i + 1], nums[r] = nums[r], nums[i + 1]
        return i + 1

    def _randomized_partition(self, nums: list[int], l: int, r: int) -> int:
        i = random.randint(l, r)
        nums[r], nums[i] = nums[i], nums[r]
        return self._partition(nums, l, r)

    def _randomized_selected(self, arr: list[int], l: int, r: int, k: int) -> None:
        if l >= r:
            return
        pos = self._randomized_partition(arr, l, r)
        num = pos - l + 1  # 左段（含 pos）的元素个数
        if k == num:
            return
        if k < num:
            self._randomized_selected(arr, l, pos - 1, k)
        else:
            self._randomized_selected(arr, pos + 1, r, k - num)

    def smallestK(self, arr: list[int], k: int) -> list[int]:
        if k <= 0:
            return []
        if k >= len(arr):
            return arr
        self._randomized_selected(arr, 0, len(arr) - 1, k)
        return arr[:k]


def check(arr: list[int], k: int) -> None:
    solution = Solution()
    want = sorted(arr)[:k]

    got_sort = sorted(solution.smallestK_sort(list(arr), k))
    got_heap = sorted(solution.smallestK_heap(list(arr), k))
    got_quick = sorted(solution.smallestK(list(arr), k))

    assert got_sort == want, f"sort({arr}, {k}) = {got_sort}, want {want}"
    assert got_heap == want, f"heap({arr}, {k}) = {got_heap}, want {want}"
    assert got_quick == want, f"quickselect({arr}, {k}) = {got_quick}, want {want}"


if __name__ == "__main__":
    # 固定随机种子，保证本文件的用例可复现
    random.seed(20261029)

    # 示例
    check([1, 3, 5, 7, 2, 4, 6, 8], 4)

    # 边界：k = 0 返回空数组
    check([1, 2, 3], 0)

    # 边界：k = n 返回全部元素
    check([5, 1, 3], 3)

    # 边界：空数组
    check([], 0)

    # 边界：单元素
    check([7], 1)

    # 边界：全部相同
    check([2, 2, 2, 2, 2], 3)

    # 边界：负数
    check([-1, -5, 3, 0, -2], 2)

    # 边界：已经有序 / 逆序
    check([1, 2, 3, 4, 5, 6], 3)
    check([6, 5, 4, 3, 2, 1], 3)

    # 数据规模上限 n = 100000，k = 100000 与 k = 1000
    big = [(i * 7919) % 100003 for i in range(100000)]
    check(big, 100000)
    check(big, 1000)

    print("all tests passed")
