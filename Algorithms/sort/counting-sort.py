'''
============================================================================
计数排序（Counting Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n + K)；空间 O(n + K)；稳定性：稳定
        K 为值域宽度 max - min + 1，不是元素个数
============================================================================

用途：
  值域不大（K 与 n 同量级）时能突破比较排序的 O(n log n) 下界；
  也是基数排序每一位的稳定子过程。面试常问「为什么不用比较」。

核心思想：
  数出每个值出现几次，再做前缀和得到「该值应该放在哪一段」，
  最后按原数组顺序把元素填进输出数组：值本身决定位置，不用比较。

关键点（不变量）：
  · 统一减去 min 做偏移，负数也能当数组下标用；
  · 前缀和之后 cnt[i] 表示「值 <= i + min 的元素个数」，也是右边界；
  · 回填必须倒序遍历原数组：这样同值元素按原次序落在连续区间的
    后半段，前一个同值元素再填到它前面，稳定性由此保证；
  · K 是值域宽度：值域极宽时计数数组比数据还大，会慢且费内存。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：min = 1，K = 5。
  1. 计数：值 1、2、3、4、5 依次出现 2、0、1、1、1 次，
     cnt = [2, 0, 1, 1, 1]；
  2. 前缀和：cnt = [2, 2, 3, 4, 5]；
  3. 倒序回填：a[4] = 5 放到 out[4]；a[3] = 1 放到 out[1]；
     a[2] = 4 放到 out[3]；a[1] = 1 放到 out[0]；a[0] = 3 放到
     out[2]；
  4. out = [1, 1, 3, 4, 5]，搬回 a。

复杂度分析：
  时间 O(n + K)：计数一趟、前缀和 K 趟、回填一趟，与比较无关。
    K 远大于 n 时（比如 n = 100、值域跨 10^9），光清零计数数组就要
    10^9 步，比 O(n log n) 的比较排序慢得多，也更费内存。
  空间 O(n + K)：输出数组 n、计数数组 K。
  稳定性：稳定。倒序回填让同值元素保持原有的相对次序。
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


def counting_sort(a: list[int]) -> None:
    n = len(a)
    if n < 2:
        return

    lo = hi = a[0]
    for v in a:  # 先求值域，负数靠偏移 v - lo 变成合法下标
        if v < lo:
            lo = v
        if v > hi:
            hi = v
    k = hi - lo + 1  # 值域宽度

    cnt = [0] * k
    for v in a:
        cnt[v - lo] += 1

    # 前缀和：cnt[i] 变成「值 <= i + lo 的元素个数」，即该值右边界
    for i in range(1, k):
        cnt[i] += cnt[i - 1]

    out = [0] * n
    # 倒序回填：同值元素里靠后的先落位，靠前的再填到它前面，
    # 于是同值元素保持了原数组中的先后次序（稳定）
    for i in range(n - 1, -1, -1):
        v = a[i]
        cnt[v - lo] -= 1
        out[cnt[v - lo]] = v
    a[:] = out


# ---------------------------------------------------------------------------
# 自测
# ---------------------------------------------------------------------------
def _print_vec(a: list[int]) -> None:
    print("[" + ", ".join(str(v) for v in a) + "]")


def _check_sorted(src: list[int]) -> None:
    want = sorted(src)  # 仅用于校验：实现部分没有使用内置排序
    got = list(src)
    counting_sort(got)
    assert got == want, f"counting_sort: {src} -> {got}"


def _main() -> None:
    # 1. 正常用例：与上面「手推一遍」的数组一致
    a = [3, 1, 4, 1, 5]
    counting_sort(a)
    print("sorted: ", end="")
    _print_vec(a)
    assert a == [1, 1, 3, 4, 5]

    # 2. 含负数：偏移写错（漏减 min）就会越界
    b = [-7, 3, -1, 0, -7, 2]
    counting_sort(b)
    print("with negatives: ", end="")
    _print_vec(b)
    assert b == [-7, -7, -1, 0, 2, 3]
    _check_sorted([-1000000, -999999, -1000001])

    # 3. 边界：空数组、单元素、全相同
    empty: list[int] = []
    counting_sort(empty)
    assert empty == []

    one = [-42]
    counting_sort(one)
    assert one == [-42]

    _check_sorted([5] * 300)

    # 4. 值域极窄：只有两个取值，计数数组长度为 2
    _check_sorted([i % 2 for i in range(400)])

    # 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
    _reset_rng()
    for n in (0, 1, 2, 3, 10, 100, 1000):
        for _ in range(20):
            _check_sorted([_next_int(-50, 50) for _ in range(n)])

    print("all tests passed")


if __name__ == "__main__":
    _main()
