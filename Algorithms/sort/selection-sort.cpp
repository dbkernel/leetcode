/*
============================================================================
选择排序（Selection Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n^2)（最好 / 平均 / 最坏都一样）；空间 O(1)；稳定性：不稳定
============================================================================

用途：
  交换次数只有 O(n) 的排序。面试里常问「比较多少次、交换多少次」、
  「为什么它不稳定」，以及「什么场景下它比插入排序划算」。

核心思想：
  把数组分成「已排序前缀」和「未排序后缀」。每趟在后缀里扫一遍，只记下
  最小值的下标（不搬元素），最后把这一个元素与前缀后面的第一个位置交换。
  一趟只交换一次，后缀长度减一，规模稳定缩小。

关键点（不变量）：
  · 第 i 趟开始前，a[0..i-1] 已经是全局最小的 i 个元素且升序；本趟在
    a[i..n-1] 里找最小值下标 minIdx，与 a[i] 交换后前缀扩成 i+1 个。
  · 比较次数恒为 n(n-1)/2 = (n-1)+(n-2)+...+1，与输入是否有序无关；
    交换次数最多 n-1（minIdx == i 时跳过交换），所以「元素大、写回贵」
    的场景它反而合适。
  · 交换本身会把 a[i] 甩到后缀中间去：若 a[minIdx] == a[i]，两个相等
    元素的先后就被打乱，所以不稳定（见下面 [2, 2, 1] 的反例）。

手推一遍：
  以 [2, 2, 1] 为例，把两个 2 按下标记作 2a（下标 0）、2b（下标 1）：
  1. i = 0：后缀 [2a, 2b, 1]，最小值 1 在下标 2，交换 a[0] 与 a[2]
     → [1, 2b, 2a]。2a 被扔到了 2b 后面，相等元素的次序反转。
  2. i = 1：后缀 [2b, 2a]，a[2] 不小于 a[1]，minIdx 保持 1，不交换。
  3. 结果 [1, 2b, 2a]：数值升序，但两个 2 的先后变了，故不稳定。

复杂度分析：
  时间 O(n^2)：最好 / 平均 / 最坏都是 n(n-1)/2 次比较，因为每趟必须把
    后缀扫完才知道谁最小，没有任何提前退出的余地；移动是 O(n) 次交换。
  空间 O(1)：原地，只有 minIdx、交换用的临时量和循环变量。
  稳定性：不稳定，反例 [2, 2, 1]。
*/
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <random>
#include <vector>

// 仅用于自测：统计比较次数与交换次数，验证两个 O(n^2) / O(n) 的界。
static long long g_cmpCount = 0;
static long long g_swapCount = 0;

void selectionSort(std::vector<int> &a) {
    int n = static_cast<int>(a.size());
    // 最后一趟（i == n-1）只剩一个元素，天然就位，不用再扫。
    for (int i = 0; i < n - 1; ++i) {
        int minIdx = i;
        // 只在 a[i..n-1] 里找最小值，前缀已就位，不能碰。
        for (int j = i + 1; j < n; ++j) {
            ++g_cmpCount;
            if (a[j] < a[minIdx]) minIdx = j;
        }
        // 最小值本来就在原位时跳过交换，省一次写回。
        if (minIdx != i) {
            std::swap(a[i], a[minIdx]);
            ++g_swapCount;
        }
    }
}

static void printVec(const std::vector<int> &a) {
    std::cout << "[";
    for (size_t i = 0; i < a.size(); ++i) {
        if (i != 0) std::cout << ", ";
        std::cout << a[i];
    }
    std::cout << "]";
}

static void checkSorted(const std::vector<int> &input,
                        const std::vector<int> &expect, const char *name) {
    std::vector<int> a = input;
    selectionSort(a);
    if (a != expect) {
        std::cout << "case " << name << " failed\n  input  ";
        printVec(input);
        std::cout << "\n  got    ";
        printVec(a);
        std::cout << "\n  expect ";
        printVec(expect);
        std::cout << std::endl;
        std::abort();
    }
}

int main() {
    // 1. 正常用例。
    checkSorted({5, 2, 4, 1, 3}, {1, 2, 3, 4, 5}, "normal");

    // 2. 边界：空数组、单元素。
    checkSorted({}, {}, "empty");
    checkSorted({7}, {7}, "single");

    // 3. 不稳定反例 [2, 2, 1]：结果数值上是对的 ...
    g_cmpCount = 0;
    g_swapCount = 0;
    checkSorted({2, 2, 1}, {1, 2, 2}, "unstable-counterexample");
    // ... 比较次数恒为 n(n-1)/2 = 3，交换次数不超过 n-1 = 2。
    assert(g_cmpCount == 3);
    assert(g_swapCount <= 2);

    // 再单独演示「相等元素换了先后」：用伴随数组记录原始下标，
    // 跑同一套选择逻辑，看两个 2 的原始下标是否被交换（实现只收 int，
    // 所以只在测试里复制一份控制流，用来观察下标流向）。
    {
        int v[3] = {2, 2, 1};
        int tag[3] = {0, 1, 2};  // tag 记原始下标
        for (int i = 0; i < 2; ++i) {
            int m = i;
            for (int j = i + 1; j < 3; ++j) {
                if (v[j] < v[m]) m = j;
            }
            if (m != i) {
                std::swap(v[i], v[m]);
                std::swap(tag[i], tag[m]);
            }
        }
        // 原始下标 0 的 2 落到下标 2，原始下标 1 的 2 停在下标 1。
        assert(tag[1] == 1 && tag[2] == 0);
    }

    // 4. 逆序、全相同：比较次数与是否有序无关，两种都必须正确。
    g_cmpCount = 0;
    checkSorted({5, 4, 3, 2, 1}, {1, 2, 3, 4, 5}, "reverse");
    assert(g_cmpCount == 10);  // 5 * 4 / 2
    checkSorted({7, 7, 7, 7}, {7, 7, 7, 7}, "all-same");

    // 5. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
    std::mt19937 rng(20240501u);
    std::uniform_int_distribution<int> dist(-50, 50);
    const int lens[] = {0, 1, 2, 3, 10, 100};
    for (int len : lens) {
        for (int round = 0; round < 20; ++round) {
            std::vector<int> a(static_cast<size_t>(len));
            for (int i = 0; i < len; ++i) a[i] = dist(rng);
            std::vector<int> expect = a;
            // 仅用于校验：实现部分没有使用内置排序
            std::sort(expect.begin(), expect.end());
            selectionSort(a);
            assert(a == expect);
        }
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
