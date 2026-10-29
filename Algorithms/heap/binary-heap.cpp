/*
============================================================================
手写二叉堆（Binary Heap）
分类：手写数据结构 · 堆与优先队列
操作复杂度：
  push   O(log n)  新元素放末尾后上浮，最多走一条树高
  pop    O(log n)  末尾元素补到根后下沉，最多走一条树高
  peek   O(1)      根就是最值
  build  O(n)      自底向上 heapify，不是逐个 push 的 O(n log n)
============================================================================

用途：
  需要反复取最值的场景：优先队列、TopK、Dijkstra、合并 K 个有序链表。
  面试常要求手写堆，而不是直接调用 std::priority_queue。

核心思想：
  把完全二叉树按层序塞进数组，父子关系用下标算出来，省掉指针。
  push 只在末尾追加再上浮；pop 把末尾元素搬到根再下沉，两者都只沿着
  一条从根到叶的路径比较，所以是 O(log n)。

关键点（不变量）：
  · 大根堆：每个父节点 >= 它的两个子节点；小根堆方向相反。
  · 0-based 下标：parent = (i-1)/2，children = 2*i+1、2*i+2。
  · 上浮看到父节点已满足堆序就停；下沉看到两个孩子都不更优就停。
  · build 必须从最后一个非叶节点 n/2-1 倒着下沉，才是 O(n)。

手推一遍：
  以小根堆为例，依次 push 3、1、4：
  1. push 3 -> [3]。
  2. push 1 -> [3,1]，1 比父 3 小，交换 -> [1,3]。
  3. push 4 -> [1,3,4]，4 比父 3 大，停。
  接着 pop：根 1 取出，末尾 4 补到根 -> [4,3]，
  4 与唯一的（较小的）孩子 3 交换 -> [3,4]。

复杂度分析：
  时间 O(log n)：push/pop 各沿树高走一条链，树高 log2(n)。
  空间 O(n)：一个动态数组存 n 个元素，不额外拷贝。
  稳定性：不稳定，等值元素的相对次序由交换决定。
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <random>
#include <vector>

// 手写动态数组：只用 new[]/delete[]，不借用任何标准容器当底座。
class IntArray {
public:
    IntArray() : data_(nullptr), size_(0), cap_(0) {}
    ~IntArray() { delete[] data_; }
    IntArray(const IntArray &) = delete;
    IntArray &operator=(const IntArray &) = delete;

    int size() const { return size_; }
    bool empty() const { return size_ == 0; }

    int operator[](int i) const { return data_[i]; }
    int &operator[](int i) { return data_[i]; }

    void push_back(int value) {
        if (size_ == cap_) grow(cap_ == 0 ? 4 : cap_ * 2);
        data_[size_++] = value;
    }

    void pop_back() {
        if (size_ > 0) --size_;
    }

    int back() const { return data_[size_ - 1]; }

    void clear() { size_ = 0; }

    void assign(const int *src, int n) {
        if (n > cap_) grow(n);
        for (int i = 0; i < n; ++i) data_[i] = src[i];
        size_ = n;
    }

    // 仅供自测：把内部内容拷成 vector 方便检查堆序。
    std::vector<int> to_vector() const {
        return std::vector<int>(data_, data_ + size_);
    }

private:
    void grow(int new_cap) {
        int *fresh = new int[new_cap];
        for (int i = 0; i < size_; ++i) fresh[i] = data_[i];
        delete[] data_;
        data_ = fresh;
        cap_ = new_cap;
    }

    int *data_;
    int size_;
    int cap_;
};

class BinaryHeap {
public:
    explicit BinaryHeap(bool is_max_heap = true) : is_max_heap_(is_max_heap) {}
    BinaryHeap(const BinaryHeap &) = delete;
    BinaryHeap &operator=(const BinaryHeap &) = delete;

    int size() const { return data_.size(); }
    bool empty() const { return data_.empty(); }
    void clear() { data_.clear(); }

    void push(int value) {
        data_.push_back(value);
        sift_up(data_.size() - 1);
    }

    // 空堆时返回 false，调用方不会被塞一个假值。
    bool pop(int &out) {
        if (data_.empty()) return false;
        out = data_[0];
        int last = data_.back();
        data_.pop_back();
        if (!data_.empty()) {
            data_[0] = last;
            sift_down(0);
        }
        return true;
    }

    bool peek(int &out) const {
        if (data_.empty()) return false;
        out = data_[0];
        return true;
    }

    // 自底向上建堆：叶子不用下沉，从最后一个非叶节点往回扫，每个节点
    // 下沉的高度之和是 O(n)，比逐个 push 的 O(n log n) 更快。
    void build(const int *values, int n) {
        data_.assign(values, n);
        for (int i = data_.size() / 2 - 1; i >= 0; --i) sift_down(i);
    }

    // 仅供自测：导出内部数组，用于直接检查不变量。
    std::vector<int> to_vector() const { return data_.to_vector(); }

private:
    // 大根堆里 a 该在上面；小根堆里 b 该在上面。
    bool higher(int a, int b) const { return is_max_heap_ ? a > b : a < b; }

    void sift_up(int i) {
        while (i > 0) {
            int parent = (i - 1) / 2;
            if (!higher(data_[i], data_[parent])) break;
            std::swap(data_[i], data_[parent]);
            i = parent;
        }
    }

    void sift_down(int i) {
        int n = data_.size();
        while (true) {
            int best = i;
            int left = 2 * i + 1;
            int right = 2 * i + 2;
            if (left < n && higher(data_[left], data_[best])) best = left;
            if (right < n && higher(data_[right], data_[best])) best = right;
            // best 还是自己，说明两个孩子都不比它更优，再走就破坏堆序。
            if (best == i) break;
            std::swap(data_[i], data_[best]);
            i = best;
        }
    }

    IntArray data_;
    bool is_max_heap_;
};

// 直接扫一遍数组，验证「父节点与两个孩子」的堆序不变量。
static bool heap_ok(const std::vector<int> &a, bool is_max_heap) {
    int n = static_cast<int>(a.size());
    for (int i = 0; i < n; ++i) {
        int left = 2 * i + 1;
        int right = 2 * i + 2;
        if (left < n && (is_max_heap ? a[left] > a[i] : a[left] < a[i])) {
            return false;
        }
        if (right < n && (is_max_heap ? a[right] > a[i] : a[right] < a[i])) {
            return false;
        }
    }
    return true;
}

// 把堆里所有元素弹出，返回弹出顺序。
static std::vector<int> drain(BinaryHeap &heap) {
    std::vector<int> out;
    int value = 0;
    while (heap.pop(value)) out.push_back(value);
    return out;
}

int main() {
    // 1. 正常用例：大根堆 push/pop 序列。
    {
        BinaryHeap heap(true);
        int input[] = {5, 3, 8, 1, 9, 2};
        for (int v : input) heap.push(v);
        assert(heap.size() == 6);

        int top = 0;
        assert(heap.peek(top) && top == 9);
        assert(heap.pop(top) && top == 9);
        assert(heap.pop(top) && top == 8);
        assert(heap.size() == 4);

        heap.push(7);
        std::vector<int> got = drain(heap);
        std::vector<int> want = {7, 5, 3, 2, 1};
        assert(got == want);
        assert(heap.empty());

        // 小根堆同一组输入，弹出顺序整体反过来。
        BinaryHeap small(false);
        for (int v : input) small.push(v);
        std::vector<int> got2 = drain(small);
        std::vector<int> want2 = {1, 2, 3, 5, 8, 9};
        assert(got2 == want2);
    }

    // 2. 边界用例：空堆 pop/peek 必须返回错误而不是脏值。
    {
        BinaryHeap heap(true);
        int value = 12345;
        assert(!heap.pop(value));
        assert(value == 12345);
        assert(!heap.peek(value));
        assert(heap.empty() && heap.size() == 0);

        // 单个元素：push 完立刻 pop，堆要能回到空态。
        heap.push(-7);
        assert(heap.size() == 1);
        assert(heap.pop(value) && value == -7);
        assert(heap.empty());
        assert(!heap.pop(value));
    }

    // 3. 关键点用例：build 的结果必须满足堆序，且反复 build 不残留旧值。
    {
        BinaryHeap heap(true);
        int values[] = {7, 2, 9, 4, 4, 1, 8};
        heap.build(values, 7);
        assert(heap.size() == 7);
        assert(heap_ok(heap.to_vector(), true));

        int shorter[] = {3, 1, 2};
        heap.build(shorter, 3);
        assert(heap.size() == 3);
        assert(heap_ok(heap.to_vector(), true));
        std::vector<int> got = drain(heap);
        std::vector<int> want = {3, 2, 1};
        assert(got == want);

        // 空数组 build 也不能越界（n/2-1 = -1，循环一次都不进）。
        heap.build(values, 0);
        assert(heap.empty() && heap_ok(heap.to_vector(), true));
    }

    // 4. 全相同元素：既不能死循环，也不能把某个副本弄丢。
    {
        BinaryHeap heap(false);
        for (int i = 0; i < 6; ++i) heap.push(6);
        assert(heap.size() == 6);
        std::vector<int> got = drain(heap);
        assert(static_cast<int>(got.size()) == 6);
        for (int v : got) assert(v == 6);

        BinaryHeap many(true);
        int same[16];
        for (int i = 0; i < 16; ++i) same[i] = 4;
        many.build(same, 16);
        assert(heap_ok(many.to_vector(), true));
        assert(static_cast<int>(drain(many).size()) == 16);
    }

    // 5. 随机对拍：固定种子，弹出序列必须与降序/升序一致。
    {
        std::mt19937 rng(20240501);
        std::uniform_int_distribution<int> dist(-1000, 1000);

        for (int round = 0; round < 5; ++round) {
            int n = 20 + round * 37;
            std::vector<int> raw;
            BinaryHeap big(true);
            BinaryHeap small(false);
            for (int i = 0; i < n; ++i) {
                int v = dist(rng);
                raw.push_back(v);
                big.push(v);
                small.push(v);
            }
            assert(big.size() == n && small.size() == n);

            // 仅用于校验：实现部分没有使用内置排序。
            std::sort(raw.begin(), raw.end());
            std::vector<int> asc = raw;
            std::vector<int> desc(raw.rbegin(), raw.rend());
            assert(drain(big) == desc);
            assert(drain(small) == asc);
            assert(big.empty() && small.empty());
        }

        // 交错 push/pop：size 始终等于有效元素个数。
        BinaryHeap heap(true);
        heap.push(4);
        heap.push(10);
        int v = 0;
        assert(heap.pop(v) && v == 10);
        heap.push(1);
        heap.push(7);
        assert(heap.size() == 3);
        assert(heap.pop(v) && v == 7);
        heap.clear();
        assert(heap.empty() && heap.size() == 0);
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
