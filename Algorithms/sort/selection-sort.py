"""
============================================================================
选择排序（Selection Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n^2)（最好 / 平均 / 最坏都一样）；空间 O(1)；稳定性：不稳定
============================================================================

用途：
  交换次数只有 O(n) 的排序。面试里常问「比较多少次、交换多少次」、
  「为什么它不稳定」，以及「什么场景下它比插入排序划算」。

核心思想：
  把数组分成「已排序前缀」和「未排序后缀」。每趟在后缀里扫一遍，只记下
  最小值的下标（不搬元素），最后把这一个元素与前缀后面的第一个位置交换。
  一趟只交换一次，后缀长度减一，规模稳定缩小。

关键点（不变量）：
  · 第 i 趟开始前，a[0..i-1] 已经是全局最小的 i 个元素且升序；本趟在
    a[i..n-1] 里找最小值下标 min_idx，与 a[i] 交换后前缀扩成 i+1 个。
  · 比较次数恒为 n(n-1)/2 = (n-1)+(n-2)+...+1，与输入是否有序无关；
    交换次数最多 n-1（min_idx == i 时跳过交换），所以「元素大、写回贵」
    的场景它反而合适。
  · 交换本身会把 a[i] 甩到后缀中间去：若 a[min_idx] == a[i]，两个相等
    元素的先后就被打乱，所以不稳定（见下面 [2, 2, 1] 的反例）。

手推一遍：
  以 [2, 2, 1] 为例，把两个 2 按下标记作 2a（下标 0）、2b（下标 1）：
  1. i = 0：后缀 [2a, 2b, 1]，最小值 1 在下标 2，交换 a[0] 与 a[2]
     → [1, 2b, 2a]。2a 被扔到了 2b 后面，相等元素的次序反转。
  2. i = 1：后缀 [2b, 2a]，a[2] 不小于 a[1]，min_idx 保持 1，不交换。
  3. 结果 [1, 2b, 2a]：数值升序，但两个 2 的先后变了，故不稳定。

复杂度分析：
  时间 O(n^2)：最好 / 平均 / 最坏都是 n(n-1)/2 次比较，因为每趟必须把
    后缀扫完才知道谁最小，没有任何提前退出的余地；移动是 O(n) 次交换。
  空间 O(1)：原地，只有 min_idx、交换用的临时量和循环变量。
  稳定性：不稳定，反例 [2, 2, 1]。
"""

import random

# 仅用于自测：统计比较次数与交换次数，验证两个 O(n^2) / O(n) 的界。
_cmp_count = 0
_swap_count = 0


def selection_sort(a: list[int]) -> None:
    global _cmp_count, _swap_count
    n = len(a)
    # 最后一趟（i == n-1）只剩一个元素，天然就位，不用再扫。
    for i in range(n - 1):
        min_idx = i
        # 只在 a[i..n-1] 里找最小值，前缀已就位，不能碰。
        for j in range(i + 1, n):
            _cmp_count += 1
            if a[j] < a[min_idx]:
                min_idx = j
        # 最小值本来就在原位时跳过交换，省一次写回。
        if min_idx != i:
            a[i], a[min_idx] = a[min_idx], a[i]
            _swap_count += 1


def show(a: list[int]) -> str:
    return "[" + ", ".join(str(v) for v in a) + "]"


def check_sorted(input_: list[int], expect: list[int], name: str) -> None:
    a = list(input_)
    selection_sort(a)
    assert a == expect, (
        f"case {name} failed: got {show(a)} expect {show(expect)}"
    )


def main() -> None:
    global _cmp_count, _swap_count

    # 1. 正常用例。
    check_sorted([5, 2, 4, 1, 3], [1, 2, 3, 4, 5], "normal")

    # 2. 边界：空数组、单元素。
    check_sorted([], [], "empty")
    check_sorted([7], [7], "single")

    # 3. 不稳定反例 [2, 2, 1]：结果数值上是对的 ...
    _cmp_count = 0
    _swap_count = 0
    check_sorted([2, 2, 1], [1, 2, 2], "unstable-counterexample")
    # ... 比较次数恒为 n(n-1)/2 = 3，交换次数不超过 n-1 = 2。
    assert _cmp_count == 3, f"比较 {_cmp_count} 次，应为 3 次"
    assert _swap_count <= 2, f"交换 {_swap_count} 次，不应超过 2 次"

    # 再单独演示「相等元素换了先后」：用伴随数组记录原始下标，
    # 跑同一套选择逻辑，看两个 2 的原始下标是否被交换（实现只收 int，
    # 所以只在测试里复制一份控制流，用来观察下标流向）。
    v = [2, 2, 1]
    tag = [0, 1, 2]  # tag 记原始下标
    for i in range(2):
        m = i
        for j in range(i + 1, 3):
            if v[j] < v[m]:
                m = j
        if m != i:
            v[i], v[m] = v[m], v[i]
            tag[i], tag[m] = tag[m], tag[i]
    # 原始下标 0 的 2 落到下标 2，原始下标 1 的 2 停在下标 1。
    assert tag[1] == 1 and tag[2] == 0, f"反例下标流向不对: {show(tag)}"

    # 4. 逆序、全相同：比较次数与是否有序无关，两种都必须正确。
    _cmp_count = 0
    check_sorted([5, 4, 3, 2, 1], [1, 2, 3, 4, 5], "reverse")
    assert _cmp_count == 10, f"反序比较 {_cmp_count} 次，应为 10 次"  # 5*4/2
    check_sorted([7, 7, 7, 7], [7, 7, 7, 7], "all-same")

    # 5. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
    rng = random.Random(20240501)
    for length in (0, 1, 2, 3, 10, 100):
        for _ in range(20):
            a = [rng.randint(-50, 50) for _ in range(length)]
            # 仅用于校验：实现部分没有使用内置排序
            expect = sorted(a)
            selection_sort(a)
            assert a == expect, (
                f"len={length}: got {show(a)} expect {show(expect)}"
            )

    print("all tests passed")


if __name__ == "__main__":
    main()
