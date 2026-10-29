'''
============================================================================
基数排序（Radix Sort，LSD 十进制）
分类：手写数据结构 · 排序算法
复杂度：时间 O(d(n + 10))；空间 O(n + 10)；稳定性：稳定
        d 为最大值的十进制位数
============================================================================

用途：
  值域很大但位数不多（32 位整数最多 10 位）时，用「按位分配」
  代替「按值比较」；面试常追问「为什么要求稳定」与「负数怎么办」。

核心思想：
  从最低位到最高位，每一位用一次稳定的计数排序把数组按该位排好；
  低位排好的相对次序会被高位排序保留，d 趟之后整体有序。

关键点（不变量）：
  · 子过程必须稳定：同一位数字相同的元素要保持上一趟的相对次序；
  · 负数单独处理：取绝对值排好再反转负数段，不能和非负数混排；
  · exp 每次乘 10，到最高位就停，多乘一次只会空跑还可能溢出；
  · 十进制每趟只有 10 个桶，所以每趟 O(n + 10)，不是 O(n + K)。

手推一遍：
  以 [13, 21, 4] 为例：
  1. 个位：13→3、21→1、4→4，按个位稳定排序得 [21, 13, 4]；
  2. 十位：21→2、13→1、4→0，按十位稳定排序得 [4, 13, 21]；
  3. 十位已是最高位，结束。
  负数同理：[-5, 21, -13] 取绝对值 [5, 13] 排好再反转成
  [-13, -5]，拼上 [21] 得 [-13, -5, 21]。

复杂度分析：
  时间 O(d(n + 10))：共 d 趟，每趟计数 + 前缀和 + 回填都是 O(n + 10)。
    d = 10 时相当于 10 倍常数；数据量小时不一定比快排快。
  空间 O(n + 10)：每趟的输出缓冲 n，计数桶 10 个。
    负数分段还额外占 O(n) 的临时数组，这是「按符号分段」的代价。
  稳定性：稳定，前提是每趟的计数排序都稳定（倒序回填保证）。
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


def _counting_by_digit(a: list[int], buf: list[int], exp: int) -> None:
    """一趟：按 exp 位（1, 10, 100 ...）对非负数组做稳定计数排序。"""
    cnt = [0] * 10
    n = len(a)
    for v in a:
        cnt[(v // exp) % 10] += 1
    for i in range(1, 10):
        cnt[i] += cnt[i - 1]
    # 倒序回填，同一位数字的元素保持原次序 => 这一趟稳定
    for i in range(n - 1, -1, -1):
        d = (a[i] // exp) % 10
        cnt[d] -= 1
        buf[cnt[d]] = a[i]
    a[:] = buf


def _radix_lsd(a: list[int]) -> None:
    """从个位到最高位逐位稳定排序。要求元素全为非负。"""
    n = len(a)
    if n < 2:
        return
    maxv = max(a)
    buf = [0] * n
    exp = 1
    while exp <= maxv:
        _counting_by_digit(a, buf, exp)
        # maxv // 10 之后再乘 10 就越过最高位了，提前收工
        if exp > maxv // 10:
            break
        exp *= 10


def radix_sort(a: list[int]) -> None:
    n = len(a)
    if n < 2:
        return

    # 负数单独处理：按绝对值排好，再把负数段反转（绝对值升序 =>
    # 数值降序）。代价是多一趟分组和 O(n) 临时空间，好处是不必
    # 整体偏移到非负，min 很负、max 很大时也不会因偏移而溢出。
    neg_mag: list[int] = []  # 负数的绝对值
    pos: list[int] = []      # 非负数
    for v in a:
        if v < 0:
            neg_mag.append(-v)  # Python 整数不会溢出
        else:
            pos.append(v)
    _radix_lsd(pos)
    _radix_lsd(neg_mag)

    k = 0
    for v in reversed(neg_mag):  # 绝对值大的负数更小
        a[k] = -v
        k += 1
    for v in pos:
        a[k] = v
        k += 1


# ---------------------------------------------------------------------------
# 自测
# ---------------------------------------------------------------------------
def _print_vec(a: list[int]) -> None:
    print("[" + ", ".join(str(v) for v in a) + "]")


def _check_sorted(src: list[int]) -> None:
    want = sorted(src)  # 仅用于校验：实现部分没有使用内置排序
    got = list(src)
    radix_sort(got)
    assert got == want, f"radix_sort: {src} -> {got}"


def _main() -> None:
    # 1. 正常用例：与上面「手推一遍」的数组一致
    a = [13, 21, 4]
    radix_sort(a)
    print("sorted: ", end="")
    _print_vec(a)
    assert a == [4, 13, 21]

    # 2. 含负数、含 0：负数段反转与 0 的归属都在这里验
    b = [-5, 21, -13, 0, 0, -5]
    radix_sort(b)
    print("with negatives: ", end="")
    _print_vec(b)
    assert b == [-13, -5, -5, 0, 0, 21]

    # 3. 边界：空数组、单元素、全为 0
    empty: list[int] = []
    radix_sort(empty)
    assert empty == []

    one = [-9]
    radix_sort(one)
    assert one == [-9]

    _check_sorted([0] * 200)

    # 4. 大数值：10^9 级别，验证位数循环
    _check_sorted([1000000000, -1000000000, 999999999, 1, -1, 0, 100000000])

    # 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
    _reset_rng()
    for n in (0, 1, 2, 3, 10, 100, 1000):
        for _ in range(20):
            _check_sorted([_next_int(-50, 50) for _ in range(n)])

    # 6. 大值域随机对拍：[-10^9, 10^9]，含负数
    _reset_rng()
    for _ in range(20):
        _check_sorted([_next_int(-1000000000, 1000000000) for _ in range(500)])

    print("all tests passed")


if __name__ == "__main__":
    _main()
