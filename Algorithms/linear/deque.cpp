/*
============================================================================
双端队列（Deque）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_front  均摊 O(1)  满时 2 倍扩容并重排下标
  push_back   均摊 O(1)  同循环队列的入队
  pop_front   O(1)       只挪 head
  pop_back    O(1)       只改 size，队尾由公式算出
  front       O(1)
  back        O(1)
  empty/size  O(1)
  clear       O(1)
============================================================================

用途：
  两端都要进出的场景：滑动窗口最值（单调队列）、0-1 BFS、回文判断、
  撤销栈、以及工作窃取调度器的本地队列。

核心思想：
  循环队列再加一个「往左长」的能力。队尾仍然是 (head + size - 1)，
  队头插就是把 head 往左挪一格再写；挪的时候要 (head - 1 + capacity)
  再取模，否则下标会变成 -1。于是两端增删都只动下标，不搬元素。

关键点（不变量）：
  · 队首物理下标是 head，队尾物理下标是 (head + size - 1) % capacity；
  · push_front 必须「先左移 head，再写 data[head]」；pop_front 必须
    「先读 data[head]，再右移 head」，两步顺序反了就会写错/读错位置；
  · 头部的下标运算一律写成 (head - 1 + capacity) % capacity。C++/Go/Rust
    的 % 对负数返回负余数，直接写 (head-1)%capacity 会拿到负下标；
  · 扩容时按逻辑顺序把 head..head+size-1 搬到新数组 0..size-1，最后才把
    head 置 0；容量翻倍后旧的取模关系全部失效。

手推一遍：
  容量 4，push_back 1、2、3、4 后 head=0、size=4，队满。
  1. pop_front 取走 1，head 变 1，size=3，逻辑内容 [2, 3, 4]；
  2. push_front(0)：head = (1-1+4)%4 = 0，写 data[0]=0，size 回到 4，
     逻辑内容 [0, 2, 3, 4]，物理上下标 0..3 全占满；
  3. 再 push_back(5)：size==capacity 先扩容到 8，按逻辑顺序搬成
     [0, 2, 3, 4] 占下标 0..3、head 归 0，再写 data[4]=5。

复杂度分析：
  时间：两端插入/删除都是均摊 O(1)；单次触发扩容是 O(n)，但一次扩容至少
    带来 capacity 次入队，摊下来仍是 O(1)。
  空间：O(n)，capacity 最多是 size 的 2 倍，且总是 2 的幂。
  稳定性：双端队列按位置存取，谈不到稳定性；它相对数组的优势就是
    两端 O(1)（数组头插是 O(n)），代价是随机访问变成一次取模。
*/

#include <cassert>
#include <iostream>
#include <vector>

const int kDequeInitCap = 4;

class Deque {
public:
    Deque()
        : data_(new int[kDequeInitCap]()), cap_(kDequeInitCap),
          head_(0), size_(0) {}

    Deque(const Deque&) = delete;
    Deque& operator=(const Deque&) = delete;

    ~Deque() { delete[] data_; }

    int size() const { return size_; }
    int capacity() const { return cap_; }
    int head() const { return head_; }  // 只为自测观察回绕
    bool empty() const { return size_ == 0; }
    bool full() const { return size_ == cap_; }

    // 头插：head 先左移一格，再写入。
    void push_front(int v) {
        if (size_ == cap_) grow();
        // 0-1 会变成 -1，必须先加 cap 再取模
        head_ = (head_ - 1 + cap_) % cap_;
        data_[head_] = v;
        ++size_;
    }

    void push_back(int v) {
        if (size_ == cap_) grow();
        data_[(head_ + size_) % cap_] = v;
        ++size_;
    }

    // 头删：先读再右移，否则读到的是新 head 上的元素。
    bool pop_front(int& out) {
        if (size_ == 0) return false;
        out = data_[head_];
        head_ = (head_ + 1) % cap_;
        --size_;
        return true;
    }

    // 尾删：只需 size 减一，把那一格「逻辑上」丢掉。
    bool pop_back(int& out) {
        if (size_ == 0) return false;
        out = data_[(head_ + size_ - 1) % cap_];
        --size_;
        return true;
    }

    bool front(int& out) const {
        if (size_ == 0) return false;
        out = data_[head_];
        return true;
    }

    bool back(int& out) const {
        if (size_ == 0) return false;
        out = data_[(head_ + size_ - 1) % cap_];
        return true;
    }

    void clear() {
        head_ = 0;
        size_ = 0;
    }

    std::vector<int> to_vector() const {
        std::vector<int> out;
        for (int i = 0; i < size_; ++i)
            out.push_back(data_[(head_ + i) % cap_]);
        return out;
    }

private:
    void grow() {
        int new_cap = cap_ * 2;
        int* nd = new int[new_cap]();
        // 用旧 head_、旧 cap_ 按逻辑顺序搬；搬完才能改动这两个量。
        for (int i = 0; i < size_; ++i) nd[i] = data_[(head_ + i) % cap_];
        delete[] data_;
        data_ = nd;
        cap_ = new_cap;
        head_ = 0;
    }

    int* data_;
    int cap_;
    int head_;
    int size_;
};

void print(const Deque& q) {
    std::vector<int> v = q.to_vector();
    std::cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i > 0) std::cout << ", ";
        std::cout << v[i];
    }
    std::cout << "] size=" << q.size() << " cap=" << q.capacity()
              << " head=" << q.head() << "\n";
}

void expect(const Deque& q, const std::vector<int>& want) {
    assert(q.to_vector() == want);
    assert(q.size() == static_cast<int>(want.size()));
}

int main() {
    // 用例 1：两端交替操作，顺序与两端取值都要对。
    Deque a;
    a.push_back(3);
    a.push_front(2);
    a.push_back(4);
    a.push_front(1);
    expect(a, {1, 2, 3, 4});
    int v = 0;
    assert(a.front(v) && v == 1);
    assert(a.back(v) && v == 4);
    assert(a.pop_front(v) && v == 1);
    assert(a.pop_back(v) && v == 4);
    expect(a, {2, 3});
    a.push_front(1);
    a.push_back(4);
    expect(a, {1, 2, 3, 4});
    print(a);

    // 用例 2：头插到底，跨多次扩容，顺序必须是整体倒序。
    Deque b;
    for (int i = 0; i < 100; ++i) b.push_front(i);
    assert(b.size() == 100 && b.capacity() == 128);
    for (int i = 0; i < 100; ++i) assert(b.to_vector()[i] == 99 - i);
    assert(b.front(v) && v == 99);
    assert(b.back(v) && v == 0);

    // 用例 3：关键点用例——head 不为 0 时扩容，两端数据顺序不能乱。
    Deque c;
    for (int i = 1; i <= 4; ++i) c.push_back(i);  // 满，head=0
    assert(c.pop_front(v) && v == 1);
    assert(c.pop_front(v) && v == 2);            // head 走到 2
    c.push_back(5);                      // 写到下标 0
    c.push_back(6);                      // 写到下标 1
    expect(c, {3, 4, 5, 6});
    assert(c.head() == 2 && c.full());
    c.push_back(7);                      // 触发扩容
    assert(c.capacity() == 8 && c.head() == 0);
    expect(c, {3, 4, 5, 6, 7});
    // 扩容后立刻头插：head 必须回绕到 7，而不是变成 -1。
    c.push_front(2);
    assert(c.head() == 7);
    expect(c, {2, 3, 4, 5, 6, 7});
    assert(c.front(v) && v == 2);
    assert(c.back(v) && v == 7);
    print(c);

    // 用例 4：空队列的每个出口。
    Deque e;
    assert(e.empty() && !e.full() && e.size() == 0);
    int out = -12345;
    assert(!e.pop_front(out));
    assert(!e.pop_back(out));
    assert(out == -12345);  // 失败时不改出参
    assert(!e.front(out));
    assert(!e.back(out));
    expect(e, {});
    e.push_front(1);
    assert(e.front(out) && out == 1);
    assert(e.back(out) && out == 1);  // 单元素时两端相同
    e.clear();
    assert(e.empty() && e.size() == 0 && e.head() == 0);
    e.push_back(2);  // clear 之后还能继续用
    expect(e, {2});

    // 用例 5：长操作序列对拍，两端交替进出，逐步核对 size 与内容。
    Deque q;
    std::vector<int> model;  // 仅用于校验：当作双端队列的参考记录
    for (int step = 0; step < 300; ++step) {
        switch (step % 4) {
            case 0:
                q.push_back(step);
                model.push_back(step);
                break;
            case 1:
                q.push_front(step);
                model.insert(model.begin(), step);
                break;
            case 2:
                if (!model.empty()) {
                    int want = model.front();
                    model.erase(model.begin());
                    assert(q.pop_front(v) && v == want);
                }
                break;
            default:
                if (!model.empty()) {
                    int want = model.back();
                    model.pop_back();
                    assert(q.pop_back(v) && v == want);
                }
                break;
        }
        assert(q.size() == static_cast<int>(model.size()));
        if (step % 37 == 0) expect(q, model);
    }
    expect(q, model);

    std::cout << "all tests passed\n";
    return 0;
}
