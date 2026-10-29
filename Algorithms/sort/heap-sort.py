'''
============================================================================
堆排序（Heap Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n log n)；空间 O(1)；稳定性：不稳定
============================================================================

用途：
  原地、最坏也能保证 O(n log n) 的排序；面试里常和「第 k 大」
  「数据流中位数」一起出现，考的是 heapify 和下沉写不写对。

核心思想：
  数组天然是一棵完全二叉树（下标 i 的孩子是 2i+1、2i+2）。
  先把数组调成大根堆，堆顶就是最大值；把它换到末尾，堆的规模减一，
  再让新堆顶下沉复位，重复 n-1 次就得到升序数组。

关键点（不变量）：
  · 下沉时只在「比最大的孩子还小」时才交换，否则立刻停；
  · 建堆从最后一个非叶结点 n/2 - 1 往前走，叶子不必下沉；
  · 交换后堆的有效长度变成 end，下沉的边界必须用 end 而不是 n；
  · 大根堆 + 堆顶换到末尾 = 升序；想降序就建小根堆。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例（n = 5）：
  1. i = 1 下沉：1 的孩子是 a[3]=1、a[4]=5，与 5 交换，
     得 [3, 5, 4, 1, 1]；
  2. i = 0 下沉：3 的孩子是 5、4，与 5 交换得 [5, 3, 4, 1, 1]，
     3 落到下标 1 后孩子都是 1，停下，堆建成；
  3. 堆顶 5 换到末尾，前 4 个下沉复位得 [4, 3, 1, 1 | 5]；
  4. 再换 4、3，最后两个 1 交换，整体 [1, 1, 3, 4, 5]。

复杂度分析：
  时间 O(n log n)：建堆 O(n)，之后 n-1 次下沉，每次最多走树高
    O(log n)，合计 O(n log n)，与输入是否有序无关。
  空间 O(1)：原地交换，下沉写成循环，不占递归栈。
  稳定性：不稳定。堆顶换到末尾、孩子与父结点交换都是远距离搬运，
    相等元素的前后次序无法保持。
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


def _sift_down(a: list[int], i: int, n: int) -> None:
    """让 a[i] 在「有效长度 n」的堆里下沉到合适位置。

    循环版：不递归，额外空间才是 O(1)。
    """
    while True:
        l = 2 * i + 1
        if l >= n:
            break  # 没有孩子，已经落到叶子
        big = l
        r = l + 1
        if r < n and a[r] > a[l]:
            big = r  # 挑较大的孩子
        if a[i] >= a[big]:
            break  # 父结点已经不小于孩子
        a[i], a[big] = a[big], a[i]
        i = big  # 继续检查被换下去的元素


def build_max_heap(a: list[int]) -> None:
    """自底向上建大根堆：从最后一个非叶结点往前扫。

    为什么是 O(n)：高度 h 的结点最多 n/2^(h+1) 个，下沉代价 h，
    总和 sum(h / 2^h) 收敛到常数，故为 O(n) 而不是 O(n log n)。
    """
    n = len(a)
    for i in range(n // 2 - 1, -1, -1):
        _sift_down(a, i, n)


def heap_sort(a: list[int]) -> None:
    n = len(a)
    if n < 2:
        return
    build_max_heap(a)
    for end in range(n - 1, 0, -1):
        a[0], a[end] = a[end], a[0]  # 当前最大值放到最终位置
        _sift_down(a, 0, end)        # 边界是 end：末尾已归位


# ---------------------------------------------------------------------------
# 自测
# ---------------------------------------------------------------------------
def _print_vec(a: list[int]) -> None:
    print("[" + ", ".join(str(v) for v in a) + "]")


def _is_max_heap(a: list[int]) -> bool:
    """逐结点检查大根堆性质：每个父结点都不小于它的孩子。"""
    n = len(a)
    for i in range(n):
        l = 2 * i + 1
        if l < n and a[i] < a[l]:
            return False
        r = l + 1
        if r < n and a[i] < a[r]:
            return False
    return True


def _check_sorted(src: list[int]) -> None:
    want = sorted(src)  # 仅用于校验：实现部分没有使用内置排序
    got = list(src)
    heap_sort(got)
    assert got == want, f"heap_sort: {src} -> {got}"


def _main() -> None:
    # 1. 正常用例：与上面「手推一遍」的数组一致
    a = [3, 1, 4, 1, 5]
    heap_sort(a)
    print("sorted: ", end="")
    _print_vec(a)
    assert a == [1, 1, 3, 4, 5]

    # 2. 边界：空数组、单元素
    empty: list[int] = []
    heap_sort(empty)
    assert empty == []

    one = [7]
    heap_sort(one)
    assert one == [7]

    # 3. 建堆后数组必须满足大根堆性质（关键点写错时这条会挂）
    _reset_rng()
    for _ in range(100):
        n = 1 + _next_u32() % 200
        arr = [_next_int(-100, 100) for _ in range(n)]
        build_max_heap(arr)
        assert _is_max_heap(arr), f"建堆后的堆性质: {arr}"
    ordered = [1, 2, 3, 4, 5]
    build_max_heap(ordered)  # 有序输入也要能建成大根堆
    assert _is_max_heap(ordered)

    # 4. 已有序 / 逆序 / 全相同
    _check_sorted(list(range(500)))
    _check_sorted(list(range(500, 0, -1)))
    _check_sorted([9] * 500)

    # 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
    _reset_rng()
    for n in (0, 1, 2, 3, 10, 100, 1000):
        for _ in range(20):
            _check_sorted([_next_int(-50, 50) for _ in range(n)])

    print("all tests passed")


if __name__ == "__main__":
    _main()
