/*
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

// 让 a[i] 在「有效长度 n」的堆里下沉到合适位置。
// 循环版：不递归，额外空间才是 O(1)。
static void siftDown(std::vector<int>& a, int i, int n) {
    while (true) {
        int l = 2 * i + 1;
        if (l >= n) break;  // 没有孩子，已经落到叶子
        int big = l;
        int r = l + 1;
        if (r < n && a[r] > a[l]) big = r;  // 挑较大的孩子
        if (a[i] >= a[big]) break;  // 父结点已经不小于孩子
        std::swap(a[i], a[big]);
        i = big;  // 继续检查被换下去的元素
    }
}

// 自底向上建大根堆：从最后一个非叶结点往前扫。
// 为什么是 O(n)：高度 h 的结点最多 n/2^(h+1) 个，下沉代价 h，
// 总和 sum(h / 2^h) 收敛到常数，故为 O(n) 而不是 O(n log n)。
void buildMaxHeap(std::vector<int>& a) {
    int n = static_cast<int>(a.size());
    for (int i = n / 2 - 1; i >= 0; --i) siftDown(a, i, n);
}

void heapSort(std::vector<int>& a) {
    int n = static_cast<int>(a.size());
    if (n < 2) return;
    buildMaxHeap(a);
    for (int end = n - 1; end > 0; --end) {
        std::swap(a[0], a[end]);   // 当前最大值放到它最终的位置
        siftDown(a, 0, end);       // 边界是 end：末尾已归位，不再参与
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

// 逐结点检查大根堆性质：每个父结点都不小于它的孩子。
static bool isMaxHeap(const std::vector<int>& a) {
    int n = static_cast<int>(a.size());
    for (int i = 0; i < n; ++i) {
        int l = 2 * i + 1, r = l + 1;
        if (l < n && a[i] < a[l]) return false;
        if (r < n && a[i] < a[r]) return false;
    }
    return true;
}

static void checkSorted(const std::vector<int>& src) {
    std::vector<int> want = src;
    // 仅用于校验：实现部分没有使用内置排序
    std::sort(want.begin(), want.end());
    std::vector<int> a = src;
    heapSort(a);
    assert(a == want);
}

int main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        std::vector<int> a = {3, 1, 4, 1, 5};
        heapSort(a);
        std::printf("sorted: ");
        printVec(a);
        std::printf("\n");
        assert((a == std::vector<int>{1, 1, 3, 4, 5}));
    }

    // 2. 边界：空数组、单元素
    {
        std::vector<int> empty;
        heapSort(empty);
        assert(empty.empty());

        std::vector<int> one = {7};
        heapSort(one);
        assert((one == std::vector<int>{7}));
    }

    // 3. 建堆后数组必须满足大根堆性质（关键点写错时这条会挂）
    {
        gSeed = 20260101u;
        for (int t = 0; t < 100; ++t) {
            std::vector<int> a;
            int n = 1 + static_cast<int>(nextU32() % 200);
            for (int i = 0; i < n; ++i) a.push_back(nextInt(-100, 100));
            buildMaxHeap(a);
            assert(isMaxHeap(a));
        }
        std::vector<int> sorted = {1, 2, 3, 4, 5};
        buildMaxHeap(sorted);  // 有序输入也要能建成大根堆
        assert(isMaxHeap(sorted));
    }

    // 4. 已有序 / 逆序 / 全相同
    {
        std::vector<int> asc, desc;
        for (int i = 0; i < 500; ++i) asc.push_back(i);
        for (int i = 500; i > 0; --i) desc.push_back(i);
        checkSorted(asc);
        checkSorted(desc);
        checkSorted(std::vector<int>(500, 9));
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

    std::printf("all tests passed\n");
    return 0;
}
