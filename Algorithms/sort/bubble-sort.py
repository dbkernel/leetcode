"""
============================================================================
冒泡排序（Bubble Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 最好 O(n)、平均 O(n^2)、最坏 O(n^2)；空间 O(1)；稳定性：稳定
============================================================================

用途：
  把数组按升序排好。面试里常以「手写一个稳定排序」或「冒泡为什么能提前
  退出」的形式出现，也常用来热身，考察对不变量和边界下标的把控。

核心思想：
  每一趟只在相邻元素之间比较，逆序就交换，于是一个元素每趟最多向右挪
  一格，最大的那个必然被一路「推」到本趟未排序区的末尾。趟数增加时未
  排序区同步变短，问题规模一路缩到只剩一个元素。

关键点（不变量）：
  · 不变量：第 k 趟结束后，末尾 k 个元素已经就位（是全局最大的 k 个，
    且内部升序）。代码里外层变量 i 从 0 开始，表示「已跑完 i 趟」，
    于是本趟未排序区是 a[0..n-1-i]，内层上界取 n-1-i：最后一次比较
    发生在 a[n-2-i] 与 a[n-1-i] 之间，正好把最大值顶到 a[n-1-i]。
  · 一趟里一次交换都没发生，说明所有相邻对都满足 a[j] <= a[j+1]，
    整个数组已有序，可以直接跳出，这就是最好 O(n) 的来源。
  · 交换条件是严格大于 a[j] > a[j+1]，相等时不换，相等元素的相对次序
    因此不会改变，所以排序稳定。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例（n = 5）：
  1. 第 0 趟，j 走 0..3：(3,1) 换 → [1,3,4,1,5]；(3,4) 不换；
     (4,1) 换 → [1,3,1,4,5]；(4,5) 不换。末尾 1 个元素 5 就位。
  2. 第 1 趟，j 走 0..2：(1,3) 不换；(3,1) 换 → [1,1,3,4,5]；
     (3,4) 不换。末尾 2 个元素 4、5 就位。
  3. 第 2 趟，j 走 0..1，一次交换都没有，swapped 仍为 False，退出。

复杂度分析：
  时间 O(n^2)：最坏（逆序）与平均都是约 n 趟、每趟线性比较；最好 O(n)，
    已有序时只扫一趟就提前退出。
  空间 O(1)：只有 swapped、循环变量和交换用的临时量。
  稳定性：稳定，相等元素不交换。
"""

import random

# 仅用于自测：统计相邻比较次数，用来验证「已有序时只扫一趟」。
_cmp_count = 0


def bubble_sort(a: list[int]) -> None:
    global _cmp_count
    n = len(a)
    # 最多 n-1 趟：每趟至少让一个元素就位，剩 1 个元素时自然有序。
    for i in range(n - 1):
        swapped = False
        # 未排序区是 a[0..n-1-i]，相邻对的最大下标是 n-1-i，
        # 所以 j 上界取 n-1-i，再大就越过已就位区了。
        for j in range(n - 1 - i):
            _cmp_count += 1
            if a[j] > a[j + 1]:
                a[j], a[j + 1] = a[j + 1], a[j]
                swapped = True
        # 没发生交换说明已经整体有序，继续跑纯属浪费。
        if not swapped:
            break


def show(a: list[int]) -> str:
    return "[" + ", ".join(str(v) for v in a) + "]"


def check_sorted(input_: list[int], expect: list[int], name: str) -> None:
    a = list(input_)
    bubble_sort(a)
    assert a == expect, (
        f"case {name} failed: got {show(a)} expect {show(expect)}"
    )


def main() -> None:
    global _cmp_count

    # 1. 正常用例。
    check_sorted([5, 2, 4, 1, 3], [1, 2, 3, 4, 5], "normal")

    # 2. 边界：空数组、单元素。
    check_sorted([], [], "empty")
    check_sorted([7], [7], "single")

    # 3. 已有序：只允许扫一趟，也就是恰好 n-1 次相邻比较。
    #    如果去掉 swapped 提前退出，这里会变成 (n-1)+(n-2)+... 次。
    sorted_input = [1, 2, 3, 4, 5]
    _cmp_count = 0
    bubble_sort(sorted_input)
    assert sorted_input == [1, 2, 3, 4, 5]
    assert _cmp_count == 4, f"已有序数组比较 {_cmp_count} 次，应为 4 次"

    # 4. 逆序：最坏情况，内层每趟都跑满。
    check_sorted([5, 4, 3, 2, 1], [1, 2, 3, 4, 5], "reverse")

    # 5. 大量重复值：含全相同、以及交替重复。
    check_sorted([7, 7, 7, 7], [7, 7, 7, 7], "all-same")
    check_sorted([2, 1, 2, 1, 2, 1], [1, 1, 1, 2, 2, 2], "duplicates")

    # 6. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
    rng = random.Random(20240501)
    for length in (0, 1, 2, 3, 10, 100):
        for _ in range(20):
            a = [rng.randint(-50, 50) for _ in range(length)]
            # 仅用于校验：实现部分没有使用内置排序
            expect = sorted(a)
            bubble_sort(a)
            assert a == expect, (
                f"len={length}: got {show(a)} expect {show(expect)}"
            )

    print("all tests passed")


if __name__ == "__main__":
    main()
