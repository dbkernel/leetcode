/*
============================================================================
插入排序（Insertion Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 最好 O(n)、平均 O(n^2)、最坏 O(n^2)；空间 O(1)；稳定性：稳定
============================================================================

用途：
  小数组或「基本有序」数据的首选排序。面试里常问「为什么它不靠交换」、
  「元素移动多少次」，它也常被当作快排小区间的收尾排序、希尔排序的内层。

核心思想：
  维护已排序前缀 a[0..i-1]，把 a[i] 取出来暂存为 key，然后在前缀里从右
  往左找 key 的落点：比 key 大的元素直接往后挪一格，空出来的位置最后放
  key。前缀逐步扩张，直到整个数组有序。

关键点（不变量）：
  · 进入第 i 轮时 a[0..i-1] 已升序；本轮结束后 a[0..i] 升序。
  · 先暂存 key、再整体后移：a[i] 被 a[i-1] 覆盖之前 key 已经存下来了，
    所以不会丢数据。每轮只做「一次比较 + 一次赋值」，比「比较 + 交换」
    少两次写内存，这是它比冒泡快的原因。
  · 内层条件是 a[j] > key（严格大于）：遇到相等就停下并插在它后面，
    相等元素的次序不变，所以稳定；写成 a[j] >= key 就会变成不稳定。
  · j 可能一路减到 -1，所以循环条件必须先判 j >= 0，再去访问 a[j]，
    顺序反了就越界。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：
  1. i = 1，key = 1：a[0] = 3 > 1 后移 → [3, 3, 4, 1, 5]，j 减到 -1，
     落点 0 → [1, 3, 4, 1, 5]。
  2. i = 2，key = 4：a[1] = 3 不大于 4，立即停，落点 2。
  3. i = 3，key = 1：4、3 依次后移 → [1, 3, 3, 4, 5]，j 减到 -1，
     落点 0 → [1, 1, 3, 4, 5]。
  4. i = 4，key = 5：a[3] = 4 不大于 5，立即停。结束。

复杂度分析：
  时间：最好 O(n)（已有序时每轮只比较一次就停），最坏 O(n^2)（逆序时
    每轮都要挪到最前面），平均 O(n^2)（随机数据平均移动 n^2/4 次）。
  空间 O(1)：只有 key、i、j 三个变量，移动都发生在原数组上。
  稳定性：稳定，见关键点第三条。
*/
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <random>
#include <vector>

// 仅用于自测：统计比较次数，验证「已有序时每轮只比较一次」。
static long long g_cmpCount = 0;
// 仅用于自测：统计元素后移次数，逆序时应当等于 n(n-1)/2。
static long long g_moveCount = 0;

void insertionSort(std::vector<int> &a) {
    int n = static_cast<int>(a.size());
    // a[0] 单独一个元素天然有序，所以从下标 1 开始扩张前缀。
    for (int i = 1; i < n; ++i) {
        int key = a[i];  // 先暂存，后面 a[i] 会被覆盖
        int j = i - 1;
        // 先判 j >= 0 再取 a[j]，顺序反了就会读到 -1。
        while (j >= 0) {
            ++g_cmpCount;  // 这次比较真实发生，无论结果如何都计数
            // 严格小于等于就停：相等元素原地不动，保证稳定。
            if (a[j] <= key) break;
            a[j + 1] = a[j];  // 整体后移一格，不交换
            ++g_moveCount;
            --j;
        }
        a[j + 1] = key;  // 此时 a[j] <= key（或 j == -1），key 落在 j+1
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
    insertionSort(a);
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

    // 3. 已有序：每轮只比较一次就停，总比较次数恰好 n-1，一次移动都没有。
    g_cmpCount = 0;
    g_moveCount = 0;
    checkSorted({1, 2, 3, 4, 5}, {1, 2, 3, 4, 5}, "already-sorted");
    assert(g_cmpCount == 4);
    assert(g_moveCount == 0);

    // 4. 逆序：最坏情况，移动次数 = 0+1+...+(n-1) = n(n-1)/2。
    g_cmpCount = 0;
    g_moveCount = 0;
    checkSorted({5, 4, 3, 2, 1}, {1, 2, 3, 4, 5}, "reverse");
    assert(g_moveCount == 10);
    assert(g_cmpCount == 10);  // 每轮都一路比到 j == -1

    // 5. 重复值：全相同 + 交替重复。全相同时每轮只比较一次，
    //    说明相等元素不会触发后移，稳定性靠的正是这一点。
    checkSorted({7, 7, 7, 7}, {7, 7, 7, 7}, "all-same");
    g_cmpCount = 0;
    checkSorted({7, 7, 7, 7}, {7, 7, 7, 7}, "all-same-cmp");
    assert(g_cmpCount == 3);
    checkSorted({2, 1, 2, 1, 2, 1}, {1, 1, 1, 2, 2, 2}, "duplicates");
    checkSorted({0, 0, -1, -1, 0}, {-1, -1, 0, 0, 0}, "dups-with-negative");

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
            insertionSort(a);
            assert(a == expect);
        }
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
