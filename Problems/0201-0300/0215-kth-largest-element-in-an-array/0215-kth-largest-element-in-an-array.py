"""
============================================================================
215. 数组中的第 K 个最大元素（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/kth-largest-element-in-an-array/
分类：分治 · 快速选择（quickselect） / 堆

给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。

示例 1:
输入: [3,2,1,5,6,4], k = 2
输出: 5

示例 2:
输入: [3,2,3,1,2,4,5,5,6], k = 4
输出: 4

提示：
1 <= k <= nums.length <= 10^5
-10^4 <= nums[i] <= 10^4
"""
import heapq


"""
解法一：快速选择（基于快速排序的划分）【最优解】

官方方法一，也是题目要求的 O(n) 解法。

为什么慢：先把整个数组排序再取倒数第 k 个，时间是 O(n log n)，且把「不必要的信息」
也算了出来——我们只要一个元素，不需要关心左右两边是否有序。

换个角度：快速排序的划分（partition）每做一次，就能把主元放到它的最终位置上，
并保证左边都不大于它、右边都不小于它。也就是说：一次划分确定一个元素的最终下标 q。
如果 q 正好是目标下标，就直接返回；如果目标在左边，就只递归左边；否则只递归右边。
每次问题规模期望减半，总代价 n + n/2 + n/4 + … = O(n)。

关键点：
  · 目标下标不是 k - 1，而是 n - k：第 k 大 = 升序数组里下标 n - k 的位置
  · 官方用的是「双指针 + 前后各扩一格」的划分写法（类似 Hoare 划分），
    不在循环内做随机化，实测对各类数据都比较稳
  · 划分循环 while i < j：i 从左跳过小于主元的，j 从右跳过大于主元的，
    都停下就交换；循环结束后 j 是左半段的右端点
  · 判断方向：k <= j 说明目标在左段及 j，递归 (l, j)；否则递归 (j + 1, r)
  · 递归出口是 l == r（区间只剩一个元素，它就是答案）
  · 注意数组元素可能为负数，比较直接用 < / > 即可

以 nums = [3,2,1,5,6,4]，k = 2 为例：
  目标下标 = 6 - 2 = 4（升序 [1,2,3,4,5,6] 的下标 4 是 5）
  第一次划分取主元 3：左边得到 {2,1}，右侧留下 {5,6,4}
  目标下标 4 落在右段，只需递归右段
  右段继续划分最终把 5 放到下标 4，返回 5

复杂度分析：
  时间 O(n)（期望）：每次只递归一侧，规模期望减半
  空间 O(log n)：递归栈（期望深度）

解法二：建大根堆 + 依次删除堆顶【非最优】

官方方法二。先把数组建成大根堆（O(n)），然后做 k - 1 次「堆顶与末尾交换 + 堆大小减一
+ 下滤」，最后堆顶就是第 k 大。下面按官方写法手写堆；实际工程里也可以用
heapq 维护大小为 k 的小根堆（额外 O(k) 空间）。

关键点：
  · 建堆从 heapSize // 2 - 1 往前下滤（叶子节点天然是堆）
  · 删除 k - 1 次就够了：每删掉一个当前最大值，堆顶就变成下一个最大值
  · 循环上界 n - k + 1 正好做 k - 1 次
  · 下滤时要先在左右孩子里挑较大的那个再交换

复杂度分析：
  时间 O(n + k log n)，最坏 O(n log n)
  空间 O(log n)：下滤递归栈

对比：堆解法稳定 O(n log n) 但不受「最坏划分」影响；
快速选择期望 O(n) 更快，是本题的最优解。
"""


class Solution:
    # ---------- 解法一：快速选择 ----------
    def _quickselect(self, nums: list[int], l: int, r: int, k: int) -> int:
        # 官方写法是递归实现；这里改成等价的循环，避免 Python 递归深度上限（1000）的坑
        while l < r:
            pivot = nums[l]
            i, j = l - 1, r + 1
            while i < j:
                i += 1
                while nums[i] < pivot:
                    i += 1
                j -= 1
                while nums[j] > pivot:
                    j -= 1
                if i < j:
                    nums[i], nums[j] = nums[j], nums[i]

            # 目标在左段（含 j）就只处理左边，否则只处理右边
            if k <= j:
                r = j
            else:
                l = j + 1
        return nums[k]

    def findKthLargest_quickselect(self, nums: list[int], k: int) -> int:
        """解法一入口：快速选择（最优解）"""
        return self._quickselect(nums, 0, len(nums) - 1, len(nums) - k)

    # ---------- 解法二：大根堆 ----------
    def _maxHeapify(self, a: list[int], i: int, heap_size: int) -> None:
        l, r, largest = i * 2 + 1, i * 2 + 2, i
        if l < heap_size and a[l] > a[largest]:
            largest = l
        if r < heap_size and a[r] > a[largest]:
            largest = r
        if largest != i:
            a[i], a[largest] = a[largest], a[i]
            self._maxHeapify(a, largest, heap_size)

    def _buildMaxHeap(self, a: list[int], heap_size: int) -> None:
        for i in range(heap_size // 2 - 1, -1, -1):
            self._maxHeapify(a, i, heap_size)

    def findKthLargest_heap(self, nums: list[int], k: int) -> int:
        """解法二入口：手写大根堆"""
        n = len(nums)
        heap_size = n
        self._buildMaxHeap(nums, heap_size)
        # 删除 k - 1 次堆顶，剩下的堆顶就是第 k 大
        for i in range(n - 1, n - k, -1):
            nums[0], nums[i] = nums[i], nums[0]
            heap_size -= 1
            self._maxHeapify(nums, 0, heap_size)
        return nums[0]

    def findKthLargest_heapq(self, nums: list[int], k: int) -> int:
        """解法二（工程写法）：大小为 k 的小根堆，O(n log k) 时间 O(k) 空间"""
        return heapq.nlargest(k, nums)[-1]


def check(nums: list[int], k: int, want: int) -> None:
    solution = Solution()
    for name in ("findKthLargest_quickselect", "findKthLargest_heap", "findKthLargest_heapq"):
        got = getattr(solution, name)(list(nums), k)
        assert got == want, f"{name}({nums}, {k}) = {got}, want {want}"


if __name__ == "__main__":
    # 示例 1
    check([3, 2, 1, 5, 6, 4], 2, 5)

    # 示例 2：存在重复元素
    check([3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4)

    # 边界：只有一个元素
    check([1], 1, 1)

    # 边界：k = 1（最大值）/ k = n（最小值）
    check([3, 2, 1, 5, 6, 4], 1, 6)
    check([3, 2, 1, 5, 6, 4], 6, 1)

    # 边界：全部相同
    check([2, 2, 2, 2], 3, 2)

    # 边界：负数与极值
    check([-1, -1], 2, -1)
    check([-10000, 10000, 0], 2, 0)
    check([10000, -10000], 1, 10000)

    # 边界：已经升序 / 降序
    check([1, 2, 3, 4, 5], 2, 4)
    check([5, 4, 3, 2, 1], 2, 4)

    # 边界：数据规模上限 10^5
    big = [(i * 37) % 20001 - 10000 for i in range(100000)]
    reference = sorted(big)
    for k in (1, 2, 500, 99999, 100000):
        check(big, k, reference[len(reference) - k])

    # 交叉验证：小数组上对所有 k 逐一核对
    small = [7, 1, 5, 3, 6, 4, 2, 8, 0, 9, 5]
    reference = sorted(small)
    for k in range(1, len(small) + 1):
        check(small, k, reference[len(reference) - k])

    print("all tests passed")
