"""
============================================================================
希尔排序（Shell Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 取决于增量序列：折半序列最坏 O(n^2)，
        Knuth 序列（1, 4, 13, ...）约 O(n^1.5)；空间 O(1)；稳定性：不稳定
============================================================================

用途：
  插入排序的「跳跃版」，用来治插入排序在「远距离逆序」上要一位一位挪的
  毛病。面试里常问「增量序列为什么决定复杂度」和「它为什么不稳定」。

核心思想：
  先取一个大增量 gap，把数组拆成 gap 条「下标模 gap 同余」的子序列，每条
  子序列内部各自做插入排序。gap 逐层折半，最后 gap == 1 时就是普通的插入
  排序，但此时数组已经「基本有序」，插入排序退化成接近 O(n)。

关键点（不变量）：
  · 第 gap 层做的是「步长为 gap 的插入排序」：暂存 a[i] 为 key，凡是比
    key 大的 a[j] 统统搬到 a[j+gap]，最后 key 落在 j+gap。下标每次减 gap，
    所以 j 可能变成负数，必须先判 j >= 0 再看 a[j]。
  · 每层结束时数组是「gap-有序」的：每条模 gap 同余的子序列内部升序。
    gap == 1 时每条子序列就是整个数组，于是整体有序。
  · i 从 gap 开始而不是 0：每条子序列的第一个元素（下标 0..gap-1）没有
    前驱，天然有序，无需处理。
  · 大 gap 阶段元素是跨着跳的，相等元素可能被跳过彼此，所以不稳定
    （反例见下面 [3, 2, 2, 1]）。

手推一遍：
  以 [3, 2b, 2a, 1] 为例（n = 4，折半增量 2 → 1），其中 2b 是下标 1
  的 2，2a 是下标 2 的 2：
  1. gap = 2，两条子序列为 (下标 0, 2) 与 (下标 1, 3)。
     i = 2，key = 2a：a[0] = 3 > 2a，于是 a[2] = 3，落点 0
     → [2a, 2b, 3, 1]。2a 一步跨过了 2b，次序已经反了。
     i = 3，key = 1：a[1] = 2b > 1，于是 a[3] = 2b，落点 1
     → [2a, 1, 3, 2b]。
  2. gap = 1，退化成普通插入排序：key = 1 挪到最前 → [1, 2a, 3, 2b]；
     key = 2b 落到 3 前面 → [1, 2a, 2b, 3]。
  3. 结果数值上确实升序，但 2a 排在 2b 前面：原本下标 2 的 2 到了下标 1，
     原本下标 1 的 2 到了下标 2，相等元素次序反转，故不稳定。

复杂度分析：
  时间：取决于增量序列。折半序列（n/2, n/4, ..., 1）最坏 O(n^2)，
    因为大 gap 阶段只保证少量有序性，最坏情况下小 gap 层仍要大量移动；
    Knuth 序列 1, 4, 13, 40, ...（h = 3h + 1）最坏约 O(n^1.5)，
    本例只用折半序列，不实现 Knuth。
  空间 O(1)：所有移动都在原列表上完成，只用 gap、key、i、j。
  稳定性：不稳定，大 gap 的跳跃会让相等元素越过彼此。
"""

import random

# 仅用于自测：统计比较次数与移动次数，验证「已有序时零移动」。
_cmp_count = 0
_move_count = 0


def shell_sort(a: list[int]) -> None:
    global _cmp_count, _move_count
    n = len(a)
    # 折半增量：n/2, n/4, ..., 1。另有 Knuth 的 3h+1 序列，此处不实现。
    gap = n // 2
    while gap > 0:
        # 对每条模 gap 同余的子序列做插入排序。前 gap 个元素是各子序列的
        # 首元素，没有前驱，所以 i 从 gap 开始。
        for i in range(gap, n):
            key = a[i]  # 先暂存，后面 a[i] 会被 a[i-gap] 覆盖
            j = i - gap  # 同一条子序列里 key 的前一个位置
            while j >= 0:
                _cmp_count += 1  # 这次比较真实发生，无论结果如何都计数
                # 相等就停，同一子序列内的相等元素保持原序。
                if a[j] <= key:
                    break
                a[j + gap] = a[j]  # 跨 gap 后移一格，不是相邻交换
                _move_count += 1
                j -= gap
            a[j + gap] = key  # 落点：j 是最后一个 <= key 的位置
        gap //= 2


def show(a: list[int]) -> str:
    return "[" + ", ".join(str(v) for v in a) + "]"


def check_sorted(input_: list[int], expect: list[int], name: str) -> None:
    a = list(input_)
    shell_sort(a)
    assert a == expect, (
        f"case {name} failed: got {show(a)} expect {show(expect)}"
    )


def main() -> None:
    global _cmp_count, _move_count

    # 1. 正常用例。
    check_sorted([5, 2, 4, 1, 3], [1, 2, 3, 4, 5], "normal")

    # 2. 边界：空数组、单元素、两个元素（gap 只有 1 层）。
    check_sorted([], [], "empty")
    check_sorted([7], [7], "single")
    check_sorted([2, 1], [1, 2], "two")

    # 3. 已有序：每层每趟只比较一次就停，零移动。
    #    n = 8 时各层 gap 为 4、2、1，比较次数 = 4 + 6 + 7 = 17。
    _cmp_count = 0
    _move_count = 0
    check_sorted([1, 2, 3, 4, 5, 6, 7, 8], [1, 2, 3, 4, 5, 6, 7, 8],
                 "already-sorted")
    assert _cmp_count == 17 and _move_count == 0, (
        f"已有序：比较 {_cmp_count} 次、移动 {_move_count} 次，应为 17 和 0"
    )

    # 4. 逆序：最坏情况之一，每层都要大量跨 gap 搬运。
    check_sorted([8, 7, 6, 5, 4, 3, 2, 1], [1, 2, 3, 4, 5, 6, 7, 8],
                 "reverse")

    # 5. 不稳定反例：用伴随数组记录原始下标，跑同一套增量与循环，
    #    观察两个相等的 2 的原始下标是否被打乱（实现只收 int，所以只在
    #    测试里复制控制流，用来观察下标流向）。
    v = [3, 2, 2, 1]
    tag = [0, 1, 2, 3]  # tag 记原始下标
    gap = 2
    while gap > 0:
        for i in range(gap, 4):
            key = v[i]
            ktag = tag[i]
            j = i - gap
            while j >= 0 and v[j] > key:
                v[j + gap] = v[j]
                tag[j + gap] = tag[j]
                j -= gap
            v[j + gap] = key
            tag[j + gap] = ktag
        gap //= 2
    # 排序后值是 [1,2,2,3]，但原来下标 2 的 2 排到了下标 1，
    # 原来下标 1 的 2 排到了下标 2：相等元素次序被反转。
    assert v == [1, 2, 2, 3], f"反例结果不对: {show(v)}"
    assert tag[1] == 2 and tag[2] == 1, f"反例下标流向不对: {show(tag)}"

    # 6. 含负数与大量重复值。
    check_sorted([-1, -5, 0, -1, 3, 0, -5], [-5, -5, -1, -1, 0, 0, 3],
                 "negatives")
    check_sorted([4, 4, 4, 4], [4, 4, 4, 4], "all-same")

    # 7. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
    rng = random.Random(20240501)
    for length in (0, 1, 2, 3, 10, 100):
        for _ in range(20):
            a = [rng.randint(-50, 50) for _ in range(length)]
            # 仅用于校验：实现部分没有使用内置排序
            expect = sorted(a)
            shell_sort(a)
            assert a == expect, (
                f"len={length}: got {show(a)} expect {show(expect)}"
            )

    print("all tests passed")


if __name__ == "__main__":
    main()
