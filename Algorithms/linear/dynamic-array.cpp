/*
============================================================================
动态数组（Dynamic Array）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_back  均摊 O(1)  满时容量翻倍，n 次 push 累计搬移小于 2n
  pop_back   O(1)       缩容只在 size <= capacity/4 时发生
  insert     O(n)       index 及其后的元素整体右移一位
  erase      O(n)       index 之后的元素整体左移一位
  at / []    O(1)       at 做越界检查，[] 不检查
  find       O(n)       线性扫描，返回下标，找不到返回 -1
  clear      O(1)       把 size 置 0，必要时顺带缩容
============================================================================

用途：
  手写一切「长度可变」容器的底座：栈、队列、哈希表的桶都能建在它上面。
  面试常以「实现一个 vector」或「这行为什么退化到 O(n^2)」的形式出现。

核心思想：
  一块连续内存 + size / capacity 两个量。size 是逻辑长度，capacity 是已
  申请的槽位数。写满就申请两倍大的块把元素搬过去；空出来四分之三就减半。

关键点（不变量）：
  · 0 <= size <= capacity，capacity 从 4 开始，每次只翻倍或减半，
    所以它永远是 2 的幂；
  · push_back 必须先扩容再写 data[size]，顺序反了就是越界写；
  · 缩容条件是 size <= capacity/4 且 capacity >= 8。若在 size 刚低于
    capacity/2 时就减半，同一个位置反复 push/pop 会来回搬移，退化成
    O(n)，这叫临界振荡；
  · insert 从后往前搬，erase 从前往后搬，方向反了会覆盖未读元素。

手推一遍：
  以 capacity=4、空数组开始连续 push 0..4：
  1. push 0..3 一路顺序写，写完 size=4，正好等于 capacity；
  2. push 4 时 size==capacity，先扩到 8 并搬走 4 个元素，再写 data[4]；
  3. 累计搬移 4 次，总写入 5 次，摊到每次 push 不到 1 次；
  4. 之后 pop 到 size=2（== 8/4）时减半回 4，此时占用率 1/2，
     再 push 两次才会再扩容，不会在两倍临界点上抖动。

复杂度分析：
  时间：单次 push_back 最坏 O(n)（扩容搬移），但 n 次连续 push 的总搬移
    是 4+8+16+... < 2n，把这份代价摊到每一次 push 上就是均摊 O(1)。
    insert/erase 要整体挪动，是 O(n)；at 与 [] 都是 O(1)。
  空间：O(n)，capacity 至多是 size 的 2 倍，浪费是常数倍。
  为什么不是 1.5 倍：1.5 倍同样能保证均摊 O(1)（总搬移仍是 n 的常数倍），
    只是常数更大、更省内存；选 2 倍纯粹是加法比乘法好写，不影响正确性。
*/

#include <cassert>
#include <iostream>
#include <stdexcept>
#include <vector>

const int kInitCap = 4;

class DynamicArray {
public:
    DynamicArray() : data_(new int[kInitCap]()), size_(0), cap_(kInitCap) {}

    // 自己 new[] 就必须自己 delete[]，所以禁掉拷贝，避免双重释放。
    DynamicArray(const DynamicArray&) = delete;
    DynamicArray& operator=(const DynamicArray&) = delete;

    ~DynamicArray() { delete[] data_; }

    int size() const { return size_; }
    int capacity() const { return cap_; }
    bool empty() const { return size_ == 0; }

    void push_back(int v) {
        if (size_ == cap_) grow();  // 先扩容，再写入
        data_[size_++] = v;
    }

    bool pop_back() {
        if (size_ == 0) return false;
        --size_;
        shrink();
        return true;
    }

    bool insert(int index, int v) {
        // index == size 等价于尾插，仍然合法；其余越界一律拒绝。
        if (index < 0 || index > size_) return false;
        if (size_ == cap_) grow();
        // 从后往前挪一位，给 index 腾位置；反向会覆盖还没搬的元素。
        for (int i = size_; i > index; --i) data_[i] = data_[i - 1];
        data_[index] = v;
        ++size_;
        return true;
    }

    bool erase(int index) {
        if (index < 0 || index >= size_) return false;
        // 从前往后挪，覆盖掉 index 这一格；i+1 < size_ 保证读不越界。
        for (int i = index; i + 1 < size_; ++i) data_[i] = data_[i + 1];
        --size_;
        shrink();
        return true;
    }

    int at(int index) const {
        if (index < 0 || index >= size_) throw std::out_of_range("at");
        return data_[index];
    }

    int& operator[](int index) { return data_[index]; }
    const int& operator[](int index) const { return data_[index]; }

    int find(int v) const {
        for (int i = 0; i < size_; ++i)
            if (data_[i] == v) return i;
        return -1;
    }

    void clear() {
        size_ = 0;
        shrink();  // 清空后通常不会马上塞满，顺手把内存还回去
    }

private:
    void grow() {
        int new_cap = cap_ * 2;
        int* nd = new int[new_cap]();
        for (int i = 0; i < size_; ++i) nd[i] = data_[i];
        delete[] data_;
        data_ = nd;
        cap_ = new_cap;
    }

    // 一次可能连降两级（例如从 16 掉到 2 再掉到 1），所以用 while。
    void shrink() {
        while (cap_ >= 8 && size_ <= cap_ / 4) {
            int new_cap = cap_ / 2;
            int* nd = new int[new_cap]();
            for (int i = 0; i < size_; ++i) nd[i] = data_[i];
            delete[] data_;
            data_ = nd;
            cap_ = new_cap;
        }
    }

    int* data_;
    int size_;
    int cap_;
};

void print(const DynamicArray& a) {
    std::cout << "[";
    for (int i = 0; i < a.size(); ++i) {
        if (i > 0) std::cout << ", ";
        std::cout << a[i];
    }
    std::cout << "]\n";
}

void expect(const DynamicArray& a, const std::vector<int>& want) {
    assert(a.size() == static_cast<int>(want.size()));
    for (int i = 0; i < a.size(); ++i) assert(a[i] == want[i]);
}

int main() {
    // 用例 1：连续 push 跨多次扩容，容量按 4/8/16/... 翻倍。
    DynamicArray a;
    assert(a.capacity() == 4 && a.empty());
    for (int i = 0; i < 100; ++i) a.push_back(i);
    assert(a.size() == 100);
    assert(a.capacity() == 128);  // 4 -> 8 -> 16 -> 32 -> 64 -> 128
    assert(a.at(0) == 0 && a.at(99) == 99 && a[50] == 50);
    assert(a.find(42) == 42 && a.find(-1) == -1);
    print(a);

    // 用例 2：insert 的头、尾、中间与越界。
    DynamicArray b;
    b.push_back(1);
    b.push_back(2);
    b.push_back(3);
    assert(b.insert(0, 0));    // 头插
    assert(b.insert(4, 4));    // 尾插，index == size
    assert(b.insert(2, 99));   // 中间插
    assert(!b.insert(99, 7));  // 越界
    assert(!b.insert(-1, 7));
    expect(b, {0, 1, 99, 2, 3, 4});
    assert(b.size() == 6);

    // 用例 3：erase 的头、尾、中间与越界。
    assert(b.erase(0));
    expect(b, {1, 99, 2, 3, 4});
    assert(b.erase(4));  // 删尾（此时 size=4）
    expect(b, {1, 99, 2, 3});
    assert(b.erase(1));
    expect(b, {1, 2, 3});
    assert(!b.erase(3));   // 下标 == size 已越界
    assert(!b.erase(-1));
    assert(b.size() == 3);

    // 用例 4：缩容后再 push，数据不能丢、容量要能重新长回来。
    DynamicArray c;
    for (int i = 0; i < 16; ++i) c.push_back(i);
    assert(c.capacity() == 16);
    for (int i = 0; i < 12; ++i) assert(c.pop_back());
    assert(c.size() == 4);
    assert(c.capacity() == 8);  // 16 -> 8（size==4 命中 size<=16/4）
    expect(c, {0, 1, 2, 3});
    for (int i = 0; i < 4; ++i) c.push_back(100 + i);
    assert(c.size() == 8 && c.capacity() == 8);  // 占用率回到 1/2 不扩容
    c.push_back(200);                            // 第 9 个才扩容
    assert(c.size() == 9 && c.capacity() == 16);
    expect(c, {0, 1, 2, 3, 100, 101, 102, 103, 200});

    // 用例 5：空数组的每个出口。
    DynamicArray e;
    assert(e.empty() && e.size() == 0 && e.capacity() == 4);
    assert(!e.pop_back());   // 空数组 pop 返回 false，不崩
    assert(e.find(1) == -1);
    assert(!e.erase(0));
    bool threw = false;
    try {
        e.at(0);
    } catch (const std::out_of_range&) {
        threw = true;
    }
    assert(threw);
    e.push_back(7);
    e.clear();
    assert(e.size() == 0 && e.empty());
    e.push_back(8);  // 清空之后还能继续用
    expect(e, {8});

    std::cout << "all tests passed\n";
    return 0;
}
