'''
============================================================================
归并排序（Merge Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n log n)；空间 O(n)；稳定性：稳定
============================================================================

用途：
  给整数数组做稳定排序，也是「分治 + 双指针合并」的入门题。
  面试里常以「排序链表」「求逆序对」的形式出现。

核心思想：
  把数组一分为二，两半各自有序后用两个指针从头各取较小者写进
  辅助数组，一趟线性扫描完成合并；长度 1 的区间天然有序。

关键点（不变量）：
  · 合并时相等取左半（用 <= 判断），稳定性就靠这一处；
  · 辅助数组只分配一次，递归各层复用同一块；
  · 子区间用左闭右开 [lo, hi)，hi - lo 即长度，空区间直接返回；
  · 自底向上版让 width 从 1 翻倍，按层合并，不需要递归栈。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：
  1. 拆成 [3, 1] 和 [4, 1, 5]；
  2. [3, 1] 拆成 [3]、[1]，合并成 [1, 3]；
  3. [4, 1, 5] 拆成 [4]、[1, 5]，[1, 5] 已有序；
  4. 合并 [4] 与 [1, 5]：先取 1，再取 4，再取 5，得 [1, 4, 5]；
  5. 合并 [1, 3] 与 [1, 4, 5]：两个 1 相等，先取左半的 1
     （稳定性），再取 3、4、5，得 [1, 1, 3, 4, 5]。

复杂度分析：
  时间 O(n log n)：递归树高 log n，每层合并共 O(n)，好坏平均一致。
  空间 O(n)：辅助数组；递归栈 O(log n) 被 O(n) 吸收。
  稳定性：稳定。相等时先取左半，而左半元素原本就在前面；
    自底向上版同样只在合并处比较，所以也稳定。
'''

# ---------------------------------------------------------------------------
# 固定种子的 xorshift32：四个语言跑同一条序列，对拍结果可复现。
# ---------------------------------------------------------------------------
_rng_state = 20260101
_MASK = 0xFFFFFFFF


def _reset_rng() -> None:
    global _rng_state
    _rng_state = 20260101


def _next_u32() -> int:
    global _rng_state
    s = _rng_state
    s ^= (s << 13) & _MASK
    s ^= s >> 17
    s ^= (s << 5) & _MASK
    _rng_state = s
    return s


def _next_int(lo: int, hi: int) -> int:
    # 返回闭区间 [lo, hi] 内的整数
    span = hi - lo + 1
    return lo + _next_u32() % span


def _merge_range(a: list[int], buf: list[int],
                 lo: int, mid: int, hi: int) -> None:
    """合并 a[lo, mid) 与 a[mid, hi)：先写进 buf，再整体搬回 a。

    只有 a[i] <= a[j] 时才取左半，相等元素的先后次序因此不变。
    """
    i, j, k = lo, mid, lo
    while i < mid and j < hi:
        if a[i] <= a[j]:
            buf[k] = a[i]
            i += 1
        else:
            buf[k] = a[j]
            j += 1
        k += 1
    while i < mid:
        buf[k] = a[i]
        i += 1
        k += 1
    while j < hi:
        buf[k] = a[j]
        j += 1
        k += 1
    for t in range(lo, hi):
        a[t] = buf[t]


def _msort(a: list[int], buf: list[int], lo: int, hi: int) -> None:
    """自顶向下：先排两半再合并；buf 由调用方分配，各层共用。"""
    if hi - lo < 2:
        return  # 空或单元素即有序，也是递归出口
    mid = lo + (hi - lo) // 2
    _msort(a, buf, lo, mid)
    _msort(a, buf, mid, hi)
    _merge_range(a, buf, lo, mid, hi)


def merge_sort(a: list[int]) -> None:
    n = len(a)
    if n < 2:
        return
    buf = [0] * n  # 只分配这一次，各层复用
    _msort(a, buf, 0, n)


def merge_sort_bottom_up(a: list[int]) -> None:
    """自底向上：width = 1, 2, 4 ...，每轮合并相邻两段。

    好处是没有递归栈；链表排序用同一套按层合并的思路，
    只改 next 指针，不用按下标取元素。
    """
    n = len(a)
    if n < 2:
        return
    buf = [0] * n
    width = 1
    while width < n:
        lo = 0
        while lo < n:
            mid = lo + width
            if mid >= n:
                break  # 只剩一段，本轮无需合并
            hi = min(lo + 2 * width, n)
            _merge_range(a, buf, lo, mid, hi)
            lo += 2 * width
        width *= 2


# ---------------------------------------------------------------------------
# 自测
# ---------------------------------------------------------------------------
def _print_vec(a: list[int]) -> None:
    print("[" + ", ".join(str(v) for v in a) + "]")


def _check_both(src: list[int]) -> None:
    """同一组输入分别跑两个版本，都与内置排序的结果比对。"""
    want = sorted(src)  # 仅用于校验：实现部分没有使用内置排序

    top = list(src)
    merge_sort(top)
    assert top == want, f"merge_sort: {src} -> {top}"

    bottom = list(src)
    merge_sort_bottom_up(bottom)
    assert bottom == want, f"merge_sort_bottom_up: {src} -> {bottom}"


def _main() -> None:
    # 1. 正常用例：与上面「手推一遍」的数组一致
    a = [3, 1, 4, 1, 5]
    merge_sort(a)
    print("sorted: ", end="")
    _print_vec(a)
    assert a == [1, 1, 3, 4, 5]

    # 2. 边界：空数组、单元素，两个版本都不能越界
    empty: list[int] = []
    merge_sort(empty)
    merge_sort_bottom_up(empty)
    assert empty == []

    one = [7]
    merge_sort(one)
    merge_sort_bottom_up(one)
    assert one == [7]

    # 3. 逆序：每层都要真的搬运，漏合并的写法会在这里挂
    _check_both(list(range(1000, 0, -1)))

    # 4. 大量重复：全相同 + 只有三种取值
    _check_both([42] * 500)
    _check_both([i % 3 for i in range(300)])

    # 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
    _reset_rng()
    for n in (0, 1, 2, 3, 10, 100, 1000):
        for _ in range(20):
            _check_both([_next_int(-50, 50) for _ in range(n)])

    print("all tests passed")


if __name__ == "__main__":
    _main()
