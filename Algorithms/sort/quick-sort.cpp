/*
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
    最坏 O(n^2)：每次枢轴都取到极值（本例几乎不可能，靠随机化兜底）。
    全相同数组因三路划分只扫一趟，是 O(n) 而不是 O(n^2)。
    小区间（长度 <= 16）改用插入排序能削掉大部分递归与常数开销，
    本例为了只留一份实现没有加，结果不变、只是常数略大。
  空间：原地，只有递归栈。最坏 O(n)（每次都只切掉一个元素），
    随机化后期望 O(log n)。若想严格 O(log n)，可只递归较短的一侧、
    较长的一侧用循环续做，本例保持一份最朴素实现。
  稳定性：不稳定。划分时会把远处的元素直接换到前面，
    相等元素的相对次序被打乱。
*/
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <vector>

// 三路划分的结果：a[lo, lt) < pivot，a[lt, gt) == pivot，
// a[gt, hi) > pivot。
struct Split {
    int lt;
    int gt;
};

// 一次快速排序的统计量，仅用于自测（验证「一趟结束」与递归深度）。
struct QuickStats {
    int calls;  // 三路划分被调用的次数
    int depth;  // 递归最大深度
};

// 固定种子的 xorshift32：四个语言跑同一条序列，对拍结果可复现。
static uint32_t nextU32(uint32_t& s) {
    s ^= s << 13;
    s ^= s >> 17;
    s ^= s << 5;
    return s;
}

// 三路划分（荷兰国旗）。返回值把区间切成三段，等于段不再递归。
static Split partition3(std::vector<int>& a, int lo, int hi,
                        uint32_t& rng, int& calls) {
    ++calls;
    // 随机枢轴：下标落在 [lo, hi) 内，先取出值再扫描
    uint32_t span = static_cast<uint32_t>(hi - lo);
    int p = lo + static_cast<int>(nextU32(rng) % span);
    int pivot = a[p];

    int lt = lo, i = lo, gt = hi;
    while (i < gt) {
        if (a[i] < pivot) {
            // [lt, i) 全是等值元素，换到 lt 只是把等值段往前挪一格
            std::swap(a[lt], a[i]);
            ++lt;
            ++i;
        } else if (a[i] > pivot) {
            --gt;
            // 换进来的元素还没看过，所以 i 原地不动
            std::swap(a[i], a[gt]);
        } else {
            ++i;
        }
    }
    return Split{lt, gt};
}

static void qs(std::vector<int>& a, int lo, int hi, uint32_t& rng,
               QuickStats& st, int depth) {
    if (hi - lo < 2) return;  // 空或单元素：已有序，递归出口
    if (depth > st.depth) st.depth = depth;
    Split s = partition3(a, lo, hi, rng, st.calls);
    qs(a, lo, s.lt, rng, st, depth + 1);  // 严格小于段
    qs(a, s.gt, hi, rng, st, depth + 1);  // 严格大于段
}

// 返回 (划分次数, 递归最大深度)，仅用于自测观察复杂度特征。
QuickStats quickSortWithStats(std::vector<int>& a) {
    QuickStats st{0, 0};
    int n = static_cast<int>(a.size());
    if (n < 2) return st;
    uint32_t rng = 20260101u;
    qs(a, 0, n, rng, st, 1);
    return st;
}

void quickSort(std::vector<int>& a) { quickSortWithStats(a); }

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

static void checkSorted(const std::vector<int>& src) {
    std::vector<int> want = src;
    // 仅用于校验：实现部分没有使用内置排序
    std::sort(want.begin(), want.end());
    std::vector<int> a = src;
    quickSort(a);
    assert(a == want);
}

int main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        std::vector<int> a = {3, 1, 4, 1, 5};
        quickSort(a);
        std::printf("sorted: ");
        printVec(a);
        std::printf("\n");
        assert((a == std::vector<int>{1, 1, 3, 4, 5}));
    }

    // 2. 边界：空数组、单元素
    {
        std::vector<int> empty;
        quickSort(empty);
        assert(empty.empty());

        std::vector<int> one = {7};
        quickSort(one);
        assert((one == std::vector<int>{7}));
    }

    // 3. 已有序：随机枢轴让它不退化，深度应远小于 n
    {
        std::vector<int> asc;
        for (int i = 0; i < 1000; ++i) asc.push_back(i);
        QuickStats st = quickSortWithStats(asc);
        for (int i = 1; i < 1000; ++i) assert(asc[i - 1] <= asc[i]);
        assert(st.depth <= 64);
        std::printf("ascending n=1000: depth=%d\n", st.depth);
    }

    // 4. 全相同：三路划分必须一趟结束（划分 1 次、深度 1）
    {
        std::vector<int> same(1000, 7);
        QuickStats st = quickSortWithStats(same);
        assert(st.calls == 1);
        assert(st.depth == 1);
        assert((same == std::vector<int>(1000, 7)));
    }

    // 5. 随机对拍：含负数与大量重复，长度覆盖 0/1/2/3/10/100/1000
    {
        uint32_t rng = 20260101u;
        const int lens[] = {0, 1, 2, 3, 10, 100, 1000};
        for (int len : lens) {
            for (int t = 0; t < 20; ++t) {
                std::vector<int> a;
                for (int i = 0; i < len; ++i) {
                    uint32_t span = 101;  // [-50, 50] 共 101 个取值
                    a.push_back(-50 + static_cast<int>(nextU32(rng) % span));
                }
                checkSorted(a);
            }
        }
    }

    std::printf("all tests passed\n");
    return 0;
}
