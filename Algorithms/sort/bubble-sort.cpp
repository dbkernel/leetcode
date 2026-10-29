/*
============================================================================
冒泡排序（Bubble Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 最好 O(n)、平均 O(n^2)、最坏 O(n^2)；空间 O(1)；稳定性：稳定
============================================================================

用途：
  把数组按升序排好。面试里常以「手写一个稳定排序」或「冒泡为什么能提前
  退出」的形式出现，也常用来热身，考察对不变量和边界下标的把控。

核心思想：
  每一趟只在相邻元素之间比较，逆序就交换，于是一个元素每趟最多向右挪
  一格，最大的那个必然被一路「推」到本趟未排序区的末尾。趟数增加时未
  排序区同步变短，问题规模一路缩到只剩一个元素。

关键点（不变量）：
  · 不变量：第 k 趟结束后，末尾 k 个元素已经就位（是全局最大的 k 个，
    且内部升序）。代码里外层变量 i 从 0 开始，表示「已跑完 i 趟」，
    于是本趟未排序区是 a[0..n-1-i]，内层上界取 n-1-i：最后一次比较
    发生在 a[n-2-i] 与 a[n-1-i] 之间，正好把最大值顶到 a[n-1-i]。
  · 一趟里一次交换都没发生，说明所有相邻对都满足 a[j] <= a[j+1]，
    整个数组已有序，可以直接跳出，这就是最好 O(n) 的来源。
  · 交换条件是严格大于 a[j] > a[j+1]，相等时不换，相等元素的相对次序
    因此不会改变，所以排序稳定。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例（n = 5）：
  1. 第 0 趟，j 走 0..3：(3,1) 换 → [1,3,4,1,5]；(3,4) 不换；
     (4,1) 换 → [1,3,1,4,5]；(4,5) 不换。末尾 1 个元素 5 就位。
  2. 第 1 趟，j 走 0..2：(1,3) 不换；(3,1) 换 → [1,1,3,4,5]；
     (3,4) 不换。末尾 2 个元素 4、5 就位。
  3. 第 2 趟，j 走 0..1，一次交换都没有，swapped 仍为 false，退出。

复杂度分析：
  时间 O(n^2)：最坏（逆序）与平均都是约 n 趟、每趟线性比较；最好 O(n)，
    已有序时只扫一趟就提前退出。
  空间 O(1)：只有 swapped、循环变量和交换用的临时量。
  稳定性：稳定，相等元素不交换。
*/
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <random>
#include <vector>

// 仅用于自测：统计相邻比较次数，用来验证「已有序时只扫一趟」。
static long long g_cmpCount = 0;

void bubbleSort(std::vector<int> &a) {
    int n = static_cast<int>(a.size());
    // 最多 n-1 趟：每趟至少让一个元素就位，剩 1 个元素时自然有序。
    for (int i = 0; i < n - 1; ++i) {
        bool swapped = false;
        // 未排序区是 a[0..n-1-i]，相邻对的最大下标是 n-1-i，
        // 所以 j 上界取 n-1-i，再大就越过已就位区了。
        for (int j = 0; j < n - 1 - i; ++j) {
            ++g_cmpCount;
            if (a[j] > a[j + 1]) {
                std::swap(a[j], a[j + 1]);
                swapped = true;
            }
        }
        // 没发生交换说明已经整体有序，继续跑纯属浪费。
        if (!swapped) break;
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
    bubbleSort(a);
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

    // 3. 已有序：只允许扫一趟，也就是恰好 n-1 次相邻比较。
    //    如果去掉 swapped 提前退出，这里会变成 (n-1)+(n-2)+... 次。
    std::vector<int> sortedInput = {1, 2, 3, 4, 5};
    g_cmpCount = 0;
    bubbleSort(sortedInput);
    assert(sortedInput == std::vector<int>({1, 2, 3, 4, 5}));
    assert(g_cmpCount == 4);

    // 4. 逆序：最坏情况，内层每趟都跑满。
    checkSorted({5, 4, 3, 2, 1}, {1, 2, 3, 4, 5}, "reverse");

    // 5. 大量重复值：含全相同、以及交替重复。
    checkSorted({7, 7, 7, 7}, {7, 7, 7, 7}, "all-same");
    checkSorted({2, 1, 2, 1, 2, 1}, {1, 1, 1, 2, 2, 2}, "duplicates");

    // 6. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
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
            bubbleSort(a);
            assert(a == expect);
        }
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
