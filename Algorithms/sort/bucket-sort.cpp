/*
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

// 桶内排序用的插入排序。用严格大于比较：相等元素不交换 => 稳定。
static void insertionSort(std::vector<int>& b) {
    int m = static_cast<int>(b.size());
    for (int i = 1; i < m; ++i) {
        int key = b[i];
        int j = i - 1;
        while (j >= 0 && b[j] > key) {  // 注意是 >，不是 >=
            b[j + 1] = b[j];
            --j;
        }
        b[j + 1] = key;
    }
}

void bucketSort(std::vector<int>& a) {
    int n = static_cast<int>(a.size());
    if (n < 2) return;

    int lo = a[0], hi = a[0];
    for (int v : a) {
        if (v < lo) lo = v;
        if (v > hi) hi = v;
    }
    // 值域宽度用 64 位：hi - lo + 1 在 int 上可能溢出
    long long span = static_cast<long long>(hi) - lo + 1;

    int k = 1;
    while (static_cast<long long>(k) * k < n) ++k;  // ceil(sqrt(n))

    std::vector<std::vector<int>> buckets(static_cast<size_t>(k));
    for (int v : a) {
        // 乘以 k 后再除，先乘后除保证桶边界均匀；大值域下必须 64 位
        long long idx = (static_cast<long long>(v) - lo) * k / span;
        buckets[static_cast<size_t>(idx)].push_back(v);
    }

    int w = 0;
    for (int i = 0; i < k; ++i) {
        insertionSort(buckets[static_cast<size_t>(i)]);
        // 桶 i 里的值都落在第 i 段值域，按桶序写回即全局有序
        for (int v : buckets[static_cast<size_t>(i)]) a[w++] = v;
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

static void checkSorted(const std::vector<int>& src) {
    std::vector<int> want = src;
    // 仅用于校验：实现部分没有使用内置排序
    std::sort(want.begin(), want.end());
    std::vector<int> a = src;
    bucketSort(a);
    assert(a == want);
}

int main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        std::vector<int> a = {7, 1, 5, 3};
        bucketSort(a);
        std::printf("sorted: ");
        printVec(a);
        std::printf("\n");
        assert((a == std::vector<int>{1, 3, 5, 7}));
    }

    // 2. 边界：空数组、单元素
    {
        std::vector<int> empty;
        bucketSort(empty);
        assert(empty.empty());

        std::vector<int> one = {7};
        bucketSort(one);
        assert((one == std::vector<int>{7}));
    }

    // 3. 全相同：所有元素挤进一个桶，退化成一个桶的插入排序
    {
        std::vector<int> same(500, 42);
        checkSorted(same);
    }

    // 4. 已有序 / 逆序 / 含负数 / 值域极窄
    {
        std::vector<int> asc, desc;
        for (int i = 0; i < 500; ++i) asc.push_back(i);
        for (int i = 500; i > 0; --i) desc.push_back(i);
        checkSorted(asc);
        checkSorted(desc);
        checkSorted(std::vector<int>{-7, -100, 0, 5, -3});
        checkSorted(std::vector<int>{-1, 0, 1, -1, 0, 1, 0});
    }

    // 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数
    {
        gSeed = 20260101u;
        const int lens[] = {0, 1, 2, 3, 10, 100, 1000};
        for (int len : lens) {
            for (int t = 0; t < 20; ++t) {
                std::vector<int> a;
                for (int i = 0; i < len; ++i) a.push_back(nextInt(-50, 50));
                checkSorted(a);
            }
        }
    }

    // 6. 值域偏斜：绝大多数元素集中在 [0, 5]，少数跑到 10^6
    {
        gSeed = 20260101u;
        for (int t = 0; t < 20; ++t) {
            std::vector<int> a;
            for (int i = 0; i < 300; ++i) a.push_back(nextInt(0, 5));
            a.push_back(1000000);
            a.push_back(-1000000);
            checkSorted(a);
        }
    }

    std::printf("all tests passed\n");
    return 0;
}
