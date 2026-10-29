'''
============================================================================
桶排序（Bucket Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 平均 O(n)、最坏 O(n^2)；空间 O(n + k)；稳定性：稳定
        k = ceil(sqrt(n)) 为桶数
============================================================================

用途：
  数据在值域上大致均匀时，用「先分桶、桶内小规模排序」换掉全局比较；
  面试常问「k 怎么取」「为什么平均是线性」。

核心思想：
  按值域把 [min, max] 均分成 k 个区间，每个元素按值落进对应桶；
  桶内元素只可能落在本桶的值域里，所以桶内排好后按桶序拼接即可。

关键点（不变量）：
  · 桶下标 idx = (v - min) * k / (max - min + 1)，用 64 位算；
    分母是值域宽度，因此 v = max 时 idx 最大也只会到 k - 1；
  · 同值元素必然进同一个桶，桶内插入排序用严格大于（>）比较，
    相等元素不交换，稳定性的另一半靠这里；
  · 桶按值域区间从左到右拼接，桶间不需要再比较；
  · 全相同或值域极窄时所有元素挤进少数桶，会退化。

手推一遍：
  以 [7, 1, 5, 3] 为例：n = 4，k = 2，值域 [1, 7]，宽度 7。
  1. idx = (v - 1) * 2 / 7：7→1、1→0、5→1、3→0；
  2. 桶 0 = [1, 3]，桶 1 = [5, 7]（桶内插入排序）；
  3. 按桶序拼接得 [1, 3, 5, 7]。

复杂度分析：
  时间：分桶 O(n) + 桶内插入排序 sum(mi^2) + 拼接 O(n)。
    平均 O(n)：均匀输入下每个桶的期望规模 m = n / k；当 k 取到
      n 的量级时 m ≈ 1，桶内 O(1)，总计 O(n)，这是「平均线性」的
      准确前提。本例按题面取 k = ceil(sqrt(n))，m ≈ sqrt(n)，
      桶内约 O(n)、k 个桶合计 O(n sqrt(n))，常数很小、实测很快。
    最坏 O(n^2)：数据全挤进一个桶，退化成一次插入排序。
  空间 O(n + k)：桶里存全部元素共 n 个，桶对象 k 个（k <= n）。
  稳定性：稳定。同值元素同桶 + 桶内插入排序稳定 + 桶序即值域序。
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
    return lo + _next_u32() % (hi - lo + 1)


def _insertion_sort(b: list[int]) -> None:
    """桶内排序用的插入排序。用严格大于比较：相等元素不交换 => 稳定。"""
    m = len(b)
    for i in range(1, m):
        key = b[i]
        j = i - 1
        while j >= 0 and b[j] > key:  # 注意是 >，不是 >=
            b[j + 1] = b[j]
            j -= 1
        b[j + 1] = key


def bucket_sort(a: list[int]) -> None:
    n = len(a)
    if n < 2:
        return

    lo = hi = a[0]
    for v in a:
        if v < lo:
            lo = v
        if v > hi:
            hi = v
    span = hi - lo + 1  # 值域宽度，Python 整数不会溢出

    k = 1
    while k * k < n:
        k += 1  # ceil(sqrt(n))

    buckets: list[list[int]] = [[] for _ in range(k)]
    for v in a:
        # 先乘后除：保证桶边界按值域均匀切分
        idx = (v - lo) * k // span
        buckets[idx].append(v)

    w = 0
    for i in range(k):
        _insertion_sort(buckets[i])
        # 桶 i 里的值都落在第 i 段值域，按桶序写回即全局有序
        for v in buckets[i]:
            a[w] = v
            w += 1


# ---------------------------------------------------------------------------
# 自测
# ---------------------------------------------------------------------------
def _print_vec(a: list[int]) -> None:
    print("[" + ", ".join(str(v) for v in a) + "]")


def _check_sorted(src: list[int]) -> None:
    want = sorted(src)  # 仅用于校验：实现部分没有使用内置排序
    got = list(src)
    bucket_sort(got)
    assert got == want, f"bucket_sort: {src} -> {got}"


def _main() -> None:
    # 1. 正常用例：与上面「手推一遍」的数组一致
    a = [7, 1, 5, 3]
    bucket_sort(a)
    print("sorted: ", end="")
    _print_vec(a)
    assert a == [1, 3, 5, 7]

    # 2. 边界：空数组、单元素
    empty: list[int] = []
    bucket_sort(empty)
    assert empty == []

    one = [7]
    bucket_sort(one)
    assert one == [7]

    # 3. 全相同：所有元素挤进一个桶，退化成一个桶的插入排序
    _check_sorted([42] * 500)

    # 4. 已有序 / 逆序 / 含负数 / 值域极窄
    _check_sorted(list(range(500)))
    _check_sorted(list(range(500, 0, -1)))
    _check_sorted([-7, -100, 0, 5, -3])
    _check_sorted([-1, 0, 1, -1, 0, 1, 0])

    # 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数
    _reset_rng()
    for n in (0, 1, 2, 3, 10, 100, 1000):
        for _ in range(20):
            _check_sorted([_next_int(-50, 50) for _ in range(n)])

    # 6. 值域偏斜：绝大多数元素集中在 [0, 5]，少数跑到 10^6
    _reset_rng()
    for _ in range(20):
        arr = [_next_int(0, 5) for _ in range(300)]
        arr.append(1000000)
        arr.append(-1000000)
        _check_sorted(arr)

    print("all tests passed")


if __name__ == "__main__":
    _main()
