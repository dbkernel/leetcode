'''
============================================================================
快速排序（Quick Sort，三路划分）
分类：手写数据结构 · 排序算法
复杂度：时间 平均 O(n log n)、最坏 O(n^2)；空间 O(log n)；稳定性：不稳定
============================================================================

用途：
  给整数数组原地排序，是面试最常要求手写的第一题；变体是
  「第 k 大」「按颜色/奇偶划分」（荷兰国旗问题）。

核心思想：
  随机取一个元素当枢轴，一趟扫描把区间分成「小于 / 等于 / 大于」
  三段；等值段已经落在最终位置，再对两侧各自递归即可。

关键点（不变量）：
  · 扫描中 [lo, lt) 全小于 pivot，[lt, i) 全等于 pivot，
    [gt, hi) 全大于 pivot，i 之前的元素都已归位；
  · a[i] > pivot 时换进来的元素还没看过，所以 i 不能右移；
  · 枢轴随机取（固定种子），全相同或已有序都不会退化成 O(n^2)；
  · 三路划分让「全相同」一趟结束，等于段不再进入递归。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例，假设随机到 pivot = 4（下标 2）：
  1. lt = 0、i = 0、gt = 5；
  2. a[0]=3 < 4：与 a[lt] 交换（自换），lt=1、i=1；
  3. a[1]=1 < 4：同上，lt=2、i=2；
  4. a[2]=4 == 4：只右移 i，i=3；
  5. a[3]=1 < 4：与 a[2] 交换得 [3, 1, 1, 4, 5]，lt=3、i=4；
  6. a[4]=5 > 4：gt 先左移到 4 再交换（自换），此时 i == gt 结束。
  等于段是 [3, 4)，左段 [3, 1, 1] 再排得 [1, 1, 3]，右段为空，
  整体 [1, 1, 3, 4, 5]。

复杂度分析：
  时间：平均 O(n log n)，每层划分共 O(n)，随机化后树高期望 O(log n)。
    最坏 O(n^2)：每次枢轴都取到极值（靠随机化兜底，本例测不到）。
    全相同数组因三路划分只扫一趟，是 O(n) 而不是 O(n^2)。
    小区间（长度 <= 16）改用插入排序能削掉大部分递归与常数开销，
    本例为了只留一份实现没有加，结果不变、只是常数略大。
  空间：原地，只有递归栈。最坏 O(n)（每次都只切掉一个元素），
    随机化后期望 O(log n)。若想严格 O(log n)，可只递归较短的一侧、
    较长的一侧用循环续做，本例保持一份最朴素实现。
  稳定性：不稳定。划分时会把远处的元素直接换到前面，
    相等元素的相对次序被打乱。
'''

_MASK = 0xFFFFFFFF


def _next_u32(s: int) -> int:
    """固定种子的 xorshift32：四个语言跑同一条序列，结果可复现。"""
    s ^= (s << 13) & _MASK
    s ^= s >> 17
    s ^= (s << 5) & _MASK
    return s


# 仅用于自测的统计量：[划分次数, 递归最大深度]
def _new_stats() -> list[int]:
    return [0, 0]


def _partition3(a: list[int], lo: int, hi: int,
                rng: list[int], st: list[int]) -> tuple[int, int]:
    """三路划分（荷兰国旗），返回 (lt, gt)。

    a[lo, lt) < pivot，a[lt, gt) == pivot，a[gt, hi) > pivot。
    """
    st[0] += 1
    # 随机枢轴：下标落在 [lo, hi) 内，先取出值再扫描
    span = hi - lo
    rng[0] = _next_u32(rng[0])
    p = lo + rng[0] % span
    pivot = a[p]

    lt, i, gt = lo, lo, hi
    while i < gt:
        if a[i] < pivot:
            # [lt, i) 全是等值元素，换到 lt 只是把等值段往前挪
            a[lt], a[i] = a[i], a[lt]
            lt += 1
            i += 1
        elif a[i] > pivot:
            gt -= 1
            # 换进来的元素还没看过，所以 i 原地不动
            a[i], a[gt] = a[gt], a[i]
        else:
            i += 1
    return lt, gt


def _qs(a: list[int], lo: int, hi: int, rng: list[int],
        st: list[int], depth: int) -> None:
    if hi - lo < 2:
        return  # 空或单元素：已有序，递归出口
    if depth > st[1]:
        st[1] = depth
    lt, gt = _partition3(a, lo, hi, rng, st)
    _qs(a, lo, lt, rng, st, depth + 1)  # 严格小于段
    _qs(a, gt, hi, rng, st, depth + 1)  # 严格大于段


def quick_sort_with_stats(a: list[int]) -> tuple[int, int]:
    """返回 (划分次数, 递归最大深度)，仅用于自测观察复杂度特征。"""
    st = _new_stats()
    n = len(a)
    if n < 2:
        return 0, 0
    _qs(a, 0, n, [20260101], st, 1)
    return st[0], st[1]


def quick_sort(a: list[int]) -> None:
    quick_sort_with_stats(a)


# ---------------------------------------------------------------------------
# 自测
# ---------------------------------------------------------------------------
def _print_vec(a: list[int]) -> None:
    print("[" + ", ".join(str(v) for v in a) + "]")


def _check_sorted(src: list[int]) -> None:
    want = sorted(src)  # 仅用于校验：实现部分没有使用内置排序
    got = list(src)
    quick_sort(got)
    assert got == want, f"quick_sort: {src} -> {got}"


def _main() -> None:
    # 1. 正常用例：与上面「手推一遍」的数组一致
    a = [3, 1, 4, 1, 5]
    quick_sort(a)
    print("sorted: ", end="")
    _print_vec(a)
    assert a == [1, 1, 3, 4, 5]

    # 2. 边界：空数组、单元素
    empty: list[int] = []
    quick_sort(empty)
    assert empty == []

    one = [7]
    quick_sort(one)
    assert one == [7]

    # 3. 已有序：随机枢轴让它不退化，深度应远小于 n
    asc = list(range(1000))
    _, depth = quick_sort_with_stats(asc)
    assert asc == list(range(1000))
    assert depth <= 64
    print(f"ascending n=1000: depth={depth}")

    # 4. 全相同：三路划分必须一趟结束（划分 1 次、深度 1）
    same = [7] * 1000
    calls, depth = quick_sort_with_stats(same)
    assert calls == 1
    assert depth == 1
    assert same == [7] * 1000

    # 5. 随机对拍：含负数与大量重复，长度覆盖 0/1/2/3/10/100/1000
    rng = [20260101]
    for n in (0, 1, 2, 3, 10, 100, 1000):
        for _ in range(20):
            arr = []
            for _ in range(n):
                rng[0] = _next_u32(rng[0])
                arr.append(-50 + rng[0] % 101)  # [-50, 50]
            _check_sorted(arr)

    print("all tests passed")


if __name__ == "__main__":
    _main()
