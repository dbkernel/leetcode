/*
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
*/
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <vector>

// ---------------------------------------------------------------------------
// 固定种子的 xorshift32：四个语言跑同一条序列，对拍结果可复现。
// ---------------------------------------------------------------------------
static uint32_t gSeed = 20260101u;

static void resetRng() { gSeed = 20260101u; }

static uint32_t nextU32() {
    uint32_t s = gSeed;
    s ^= s << 13;
    s ^= s >> 17;
    s ^= s << 5;
    gSeed = s;
    return s;
}

// 返回闭区间 [lo, hi] 内的整数。
static int nextInt(int lo, int hi) {
    uint32_t span = static_cast<uint32_t>(hi - lo + 1);
    return lo + static_cast<int>(nextU32() % span);
}

// 合并 a[lo, mid) 与 a[mid, hi)：先写进 buf，再整体搬回 a。
// 只有 a[i] <= a[j] 时才取左半，相等元素的先后次序因此不变。
static void mergeRange(std::vector<int>& a, std::vector<int>& buf,
                       int lo, int mid, int hi) {
    int i = lo, j = mid, k = lo;
    while (i < mid && j < hi) {
        if (a[i] <= a[j]) {
            buf[k++] = a[i++];
        } else {
            buf[k++] = a[j++];
        }
    }
    while (i < mid) buf[k++] = a[i++];
    while (j < hi) buf[k++] = a[j++];
    for (int t = lo; t < hi; ++t) a[t] = buf[t];
}

// 自顶向下：先排两半再合并；buf 由调用方分配，各层共用。
static void msort(std::vector<int>& a, std::vector<int>& buf,
                  int lo, int hi) {
    if (hi - lo < 2) return;       // 空或单元素即有序，也是递归出口
    int mid = lo + (hi - lo) / 2;  // 这样写不会因 lo + hi 溢出
    msort(a, buf, lo, mid);
    msort(a, buf, mid, hi);
    mergeRange(a, buf, lo, mid, hi);
}

void mergeSort(std::vector<int>& a) {
    int n = static_cast<int>(a.size());
    if (n < 2) return;
    std::vector<int> buf(a.size());  // 只分配这一次，各层复用
    msort(a, buf, 0, n);
}

// 自底向上：width = 1, 2, 4 ...，每轮合并相邻两段长度 width 的区间。
// 好处是没有递归栈；链表排序用同一套按层合并的思路，只改 next 指针。
void mergeSortBottomUp(std::vector<int>& a) {
    int n = static_cast<int>(a.size());
    if (n < 2) return;
    std::vector<int> buf(a.size());
    for (int width = 1; width < n; width *= 2) {
        for (int lo = 0; lo < n; lo += 2 * width) {
            int mid = lo + width;
            if (mid >= n) break;  // 只剩一段，本轮无需合并
            int hi = lo + 2 * width;
            if (hi > n) hi = n;
            mergeRange(a, buf, lo, mid, hi);
        }
    }
}

// ---------------------------------------------------------------------------
// 自测
// ---------------------------------------------------------------------------
static void printVec(const std::vector<int>& a) {
    std::printf("[");
    for (size_t i = 0; i < a.size(); ++i) {
        std::printf(i ? ", %d" : "%d", a[i]);
    }
    std::printf("]");
}

// 同一组输入分别喂给两个版本，都与内置排序的结果比对。
static void checkBoth(const std::vector<int>& src) {
    std::vector<int> want = src;
    // 仅用于校验：实现部分没有使用内置排序
    std::sort(want.begin(), want.end());

    std::vector<int> top = src;
    mergeSort(top);
    assert(top == want);

    std::vector<int> bottom = src;
    mergeSortBottomUp(bottom);
    assert(bottom == want);
}

int main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        std::vector<int> a = {3, 1, 4, 1, 5};
        mergeSort(a);
        std::printf("sorted: ");
        printVec(a);
        std::printf("\n");
        assert((a == std::vector<int>{1, 1, 3, 4, 5}));
    }

    // 2. 边界：空数组、单元素，两个版本都不能越界
    {
        std::vector<int> empty;
        mergeSort(empty);
        mergeSortBottomUp(empty);
        assert(empty.empty());

        std::vector<int> one = {7};
        mergeSort(one);
        mergeSortBottomUp(one);
        assert((one == std::vector<int>{7}));
    }

    // 3. 逆序：每层都要真的搬运，漏合并的写法会在这里挂
    {
        std::vector<int> rev;
        for (int i = 1000; i >= 1; --i) rev.push_back(i);
        checkBoth(rev);
    }

    // 4. 大量重复：全相同 + 只有三种取值
    {
        checkBoth(std::vector<int>(500, 42));
        std::vector<int> few;
        for (int i = 0; i < 300; ++i) few.push_back(i % 3);
        checkBoth(few);
    }

    // 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
    {
        resetRng();
        const int lens[] = {0, 1, 2, 3, 10, 100, 1000};
        for (int len : lens) {
            for (int t = 0; t < 20; ++t) {
                std::vector<int> a;
                for (int i = 0; i < len; ++i) a.push_back(nextInt(-50, 50));
                checkBoth(a);
            }
        }
    }

    std::printf("all tests passed\n");
    return 0;
}
