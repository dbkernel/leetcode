/*
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

// 一趟：按 exp 位（1, 10, 100 ...）对非负数组做稳定计数排序。
static void countingByDigit(std::vector<long long>& a,
                           std::vector<long long>& buf, long long exp) {
    int cnt[10] = {0};
    int n = static_cast<int>(a.size());
    for (long long v : a) ++cnt[(v / exp) % 10];
    for (int i = 1; i < 10; ++i) cnt[i] += cnt[i - 1];
    // 倒序回填，同一位数字的元素保持原次序 => 这一趟稳定
    for (int i = n - 1; i >= 0; --i) {
        int d = static_cast<int>((a[i] / exp) % 10);
        --cnt[d];
        buf[cnt[d]] = a[i];
    }
    for (int i = 0; i < n; ++i) a[i] = buf[i];
}

// LSD：从个位到最高位，逐位稳定排序。要求元素全为非负。
static void radixLsd(std::vector<long long>& a) {
    int n = static_cast<int>(a.size());
    if (n < 2) return;
    long long maxv = a[0];
    for (long long v : a) {
        if (v > maxv) maxv = v;
    }
    std::vector<long long> buf(a.size());
    long long exp = 1;
    while (exp <= maxv) {
        countingByDigit(a, buf, exp);
        // maxv / 10 之后再乘 10 就越过最高位了，提前收工避免溢出
        if (exp > maxv / 10) break;
        exp *= 10;
    }
}

void radixSort(std::vector<int>& a) {
    int n = static_cast<int>(a.size());
    if (n < 2) return;

    // 负数单独处理：按绝对值排好，再把负数段反转（绝对值升序 =>
    // 数值降序）。代价是多一趟分组和 O(n) 临时空间，好处是不必
    // 整体偏移到非负，min 很负、max 很大时也不会因偏移而溢出。
    std::vector<long long> negMag;  // 负数的绝对值
    std::vector<long long> pos;     // 非负数
    for (int v : a) {
        if (v < 0) {
            // 先转 64 位再取负，INT_MIN 也不会溢出
            negMag.push_back(-static_cast<long long>(v));
        } else {
            pos.push_back(v);
        }
    }
    radixLsd(pos);
    radixLsd(negMag);

    int k = 0;
    for (int i = static_cast<int>(negMag.size()) - 1; i >= 0; --i) {
        a[k++] = static_cast<int>(-negMag[i]);  // 绝对值大的负数更小
    }
    for (long long v : pos) a[k++] = static_cast<int>(v);
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
    radixSort(a);
    assert(a == want);
}

int main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        std::vector<int> a = {13, 21, 4};
        radixSort(a);
        std::printf("sorted: ");
        printVec(a);
        std::printf("\n");
        assert((a == std::vector<int>{4, 13, 21}));
    }

    // 2. 含负数、含 0：负数段反转与 0 的归属都在这里验
    {
        std::vector<int> a = {-5, 21, -13, 0, 0, -5};
        radixSort(a);
        printVec(a);
        std::printf("\n");
        assert((a == std::vector<int>{-13, -5, -5, 0, 0, 21}));
    }

    // 3. 边界：空数组、单元素、含 0 且全为 0
    {
        std::vector<int> empty;
        radixSort(empty);
        assert(empty.empty());

        std::vector<int> one = {-9};
        radixSort(one);
        assert((one == std::vector<int>{-9}));

        checkSorted(std::vector<int>(200, 0));
    }

    // 4. 大数值：10^9 级别，验证位数循环与 int 边界
    {
        std::vector<int> a = {1000000000, -1000000000, 999999999,
                              1, -1, 0, 100000000};
        checkSorted(a);
    }

    // 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
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

    // 6. 大值域随机对拍：[-10^9, 10^9]，含负数
    {
        gSeed = 20260101u;
        for (int t = 0; t < 20; ++t) {
            std::vector<int> a;
            for (int i = 0; i < 500; ++i) {
                a.push_back(nextInt(-1000000000, 1000000000));
            }
            checkSorted(a);
        }
    }

    std::printf("all tests passed\n");
    return 0;
}
