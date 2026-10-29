"""
============================================================================
295. 数据流的中位数（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-median-from-data-stream/
分类：堆 · 对顶堆（数据流中位数）
============================================================================

中位数是有序整数列表中的中间值。如果列表的大小是偶数，则没有中间值，
中位数是两个中间值的平均值。

例如 arr = [2,3,4] 的中位数是 3 。
例如 arr = [2,3] 的中位数是 (2 + 3) / 2 = 2.5 。

实现 MedianFinder 类:
- MedianFinder() 初始化 MedianFinder 对象。
- void addNum(int num) 将数据流中的整数 num 添加到数据结构中。
- double findMedian() 返回到目前为止所有元素的中位数。与实际答案相差
  10^-5 以内的答案将被接受。

示例 1：
输入：
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
输出：
[null, null, null, 1.5, null, 2.0]

解释：
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // 返回 1.5 ((1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0

提示：
-10^5 <= num <= 10^5
在调用 findMedian 之前，数据结构中至少有一个元素
最多 5 * 10^4 次调用 addNum 和 findMedian
"""
import bisect
import heapq
import math
from typing import List


"""
解法一：有序数组 + 二分插入【非最优】

为什么慢：这个解法把「让数组保持有序」当成不变式，于是 addNum 每次都要
把新元素放到正确位置。二分查找只能把「找位置」这一步压到 O(log n)，找到
之后仍要把后面所有元素整体后移一格，那一步是 O(n)。调用 5 * 10^4 次
addNum，最坏就是 5 * 10^4 * 5 * 10^4 次移动。

换个角度：findMedian 只看「中间那一两个元素」，并不需要全体有序。把数据
按大小劈成两半，各自内部只要能让「最靠中间的那个」浮到顶，就够了。

关键点：
  · insort 内部就是「二分找位置 + 整体后移」，等价于官方写法的二分插入
  · 用 bisect.insort 而不是 append 后 sort，是为了避免每次都付 O(n log n)
  · 偶数个元素时取 len/2 与 len/2-1 两处的平均，奇数个时取 len/2
  · findMedian 是 O(1)：下标算出来即可，不需要再排序
  · 这是「有序数组」解法的本地类名，平台上的类名是 MedianFinder

以示例的调用序列为例：
  addNum(1) → [1]，findMedian 时 len = 1，取 [0] → 1.0
  addNum(2) → [1,2]，len = 2，取 (1 + 2) / 2 → 1.5
  addNum(3) → [1,2,3]，len = 3，取 [1] → 2.0 ✔

复杂度分析：
  时间 addNum O(n)（插入时的整体后移），findMedian O(1)
  空间 O(n)：装下所有元素
"""


class MedianFinderSorted:
    def __init__(self):
        self.data: List[int] = []

    def addNum(self, num: int) -> None:
        # 等值元素插在前面也合法，重复值的相对次序不影响中位数
        bisect.insort(self.data, num)

    def findMedian(self) -> float:
        n = len(self.data)
        if n % 2 == 1:
            return float(self.data[n // 2])
        return (self.data[n // 2 - 1] + self.data[n // 2]) / 2


"""
解法二：对顶双堆（大根堆放小的一半，小根堆放大的一个半）【最优解】

契约：low 是大根堆，装较小的一半；high 是小根堆，装较大的一半。不变式是
len(low) == len(high) 或者 len(low) == len(high) + 1 —— 中位数只由两个
堆顶决定。

为什么慢：解法一的瓶颈在「维持全体有序」。可中位数并不关心 low 内部、
high 内部的排序，只关心两条边界：low 的最大值与 high 的最小值。把这两条
边界做成堆顶，其余元素爱怎么排就怎么排。

换个角度：新元素只可能落在 low 的最大值和 high 的最小值之间。所以先比较
一次决定它进哪个堆，再用一次「搬运」把两个堆的大小差拉回 0 或 1。

关键点：
  · 先决定入哪个堆：num <= low 的堆顶进 low，否则进 high；low 为空时直接进 low
  · 平衡只朝一个方向修：low 比 high 多 2 个就搬一个到 high；high 比 low 多
    就搬回 low。两步都是「先 push 对方堆顶、再 pop」，顺序反了会白搬一个元素
  · 两个堆的大小差永远不超过 1，所以 findMedian 只需读两个堆顶
  · heapq 只有最小堆，所以 low 里存相反数：取相反数之后「最大的」变成
    「最小的」，正好落在堆顶；读出时要再取一次相反数
  · 除以 2 得到 float，Python 3 的 / 本身就是真除法，不会截断
  · 平台类名 MedianFinder，构造函数无参数

以示例的调用序列为例：
  addNum(1)：low = [-1]，high = []
  addNum(2)：2 > 堆顶 1，进 high → low = [-1]，high = [2]
    findMedian：两堆等大 → (1 + 2) / 2 = 1.5 ✔
  addNum(3)：3 > 堆顶 2，进 high → high 比 low 多，把 2 搬回 low
    low 里是 [-2,-1]（堆顶 -2 即 2），high = [3]；low 多一个 → 2.0 ✔

复杂度分析：
  时间 addNum O(log n)：最多两次堆操作；findMedian O(1)
  空间 O(n)：两个堆合起来正好装下所有元素
"""


class MedianFinder:
    def __init__(self):
        # low 存相反数，于是 heapq 的堆顶是「较小一半」里的最大值
        self.low: List[int] = []
        # high 直接存原值，堆顶是「较大一半」里的最小值
        self.high: List[int] = []

    def addNum(self, num: int) -> None:
        # 只跟 low 的堆顶比：-low[0] 就是较小那一半的上界
        if not self.low or num <= -self.low[0]:
            heapq.heappush(self.low, -num)
        else:
            heapq.heappush(self.high, num)
        self._rebalance()

    def findMedian(self) -> float:
        if len(self.low) > len(self.high):
            return float(-self.low[0])
        return (-self.low[0] + self.high[0]) / 2

    def _rebalance(self) -> None:
        # 只可能是 low 多出 2 个，或者 high 多出 1 个
        if len(self.low) > len(self.high) + 1:
            heapq.heappush(self.high, -heapq.heappop(self.low))
        elif len(self.high) > len(self.low):
            heapq.heappush(self.low, -heapq.heappop(self.high))


# ===== 测试辅助 =====
def brute_median(prefix: List[int]) -> float:
    v = sorted(prefix)
    n = len(v)
    if n % 2 == 1:
        return float(v[n // 2])
    return (v[n // 2 - 1] + v[n // 2]) / 2


def nearly_equal(a: float, b: float) -> bool:
    return math.fabs(a - b) < 1e-6


def check(nums: List[int], probes: List[int]) -> None:
    """依次 addNum，每到一个检查点就比较两种实现的 findMedian。

    probes 里的数字是「已经喂进去多少个元素」，必须递增。
    """
    sorted_impl = MedianFinderSorted()
    heap_impl = MedianFinder()

    fed = 0
    for c in probes:
        while fed < c:
            sorted_impl.addNum(nums[fed])
            heap_impl.addNum(nums[fed])
            fed += 1

        want = brute_median(nums[:c])
        assert nearly_equal(sorted_impl.findMedian(), want)
        assert nearly_equal(heap_impl.findMedian(), want)


if __name__ == "__main__":
    # 示例 1：按题目给的调用序列回放，1 个、2 个、3 个元素各查一次
    check([1, 2, 3], [1, 2, 3])

    # 边界：只有一个元素，奇偶分支走的是奇数那一支
    check([5], [1])

    # 边界：全部是负数，验证大根堆的比较不会因为符号出错
    check([-1, -2, -3, -4, -5], [1, 2, 3, 4, 5])

    # 边界：全部相同，两个堆的大小差永远是 0 或 1
    check([1, 1, 1, 1], [1, 2, 3, 4])

    # 边界：先大后小地插入，正是「不预先排序」最容易错的地方
    check([100, 90, 80, 70, 60], [1, 2, 3, 4, 5])

    # 边界：范围的上下界，平均值必须用浮点算
    check([-100000, 100000], [1, 2])

    # 边界：相邻两个元素，验证除以 2 不被截断
    check([1, 2], [2])

    # 规模：1000 个元素，每加 100 个查一次，和暴力排序对照
    big_nums = []
    big_probes = []
    for i in range(1000):
        # 固定公式生成，保证每次运行结果一致
        big_nums.append((i * 37) % 201 - 100)
        if (i + 1) % 100 == 0:
            big_probes.append(i + 1)
    check(big_nums, big_probes)

    print("all tests passed")
