/*
============================================================================
循环队列（Circular Queue）
分类：手写数据结构 · 线性结构

操作复杂度：
  enqueue   均摊 O(1)  满时 2 倍扩容并重排下标
  dequeue   O(1)       只动 head，不搬元素
  front     O(1)       队首元素
  rear      O(1)       队尾元素，由 (head+size-1) 算出
  empty     O(1)
  full      O(1)
  size      O(1)
  clear     O(1)
============================================================================

用途：
  BFS 的待访问队列、任务调度、滑动窗口最值、生产者/消费者之间的环形
  缓冲（音频流、日志缓冲都是这个形状）。

核心思想：
  一块环形数组，只维护 head 和 size 两个量：队尾就是
  (head + size) % capacity，不用单独存 tail。出队只把 head 往前走一格，
  元素一个都不搬，于是 dequeue 是 O(1) 而不是数组删除的 O(n)。

关键点（不变量）：
  · 队首逻辑下标是 head，队尾逻辑下标是 (head + size - 1) % capacity，
    一律用公式算，不要另存一个 tail 指针去维护两套状态；
  · 用 size 区分空满，而不是「留一个空槽」：留空槽法最多只能存
    capacity-1 个元素，且每次判满都要绕一圈算 (tail+1)%cap == head；
  · 下标回绕统一写 (i + 1) % capacity 这类形式，往左走必须写成
    (i - 1 + capacity) % capacity，C++ 的 % 对负数会给出负余数；
  · 扩容时必须先按「旧 head、旧 capacity」把元素按逻辑顺序搬到新数组的
    0..size-1，循环结束后才能把 head 置 0、capacity 翻倍；先改 head
    或先改 capacity，取模映射就全错，元素顺序会乱。

手推一遍：
  容量 4，依次 enqueue 1、2、3、4 后 size=4、head=0，队满。
  1. dequeue 两次，取走 1、2，head 走到 2，size=2，逻辑内容 [3, 4]；
  2. 再 enqueue 5：写入下标 (2+2)%4 = 0；enqueue 6：写入 (2+3)%4 = 1；
     物理上是 [5, 6, 3, 4]，逻辑上仍是 [3, 4, 5, 6]，这就是回绕；
  3. 此时再 enqueue 7 就满了，触发扩容：按逻辑顺序搬出 3、4、5、6、7
     到新数组下标 0..4，head 置 0，capacity 变 8。

复杂度分析：
  时间：enqueue 均摊 O(1)（扩容搬移摊到每次入队，n 次入队总搬移小于 2n），
    dequeue/front/rear 都是 O(1)，因为不搬元素只挪下标。
  空间：O(n)，capacity 最多是 size 的 2 倍。
  稳定性：队列是 FIFO，谈不到稳定性。
  与「数组模拟队列」的对比：不做循环就得每次出队整体左移，那是 O(n)；
  循环队列把这份 O(n) 摊成 O(1)，代价只是多一次取模。
*/

#include <cassert>
#include <iostream>
#include <vector>

const int kQueueInitCap = 4;

class CircularQueue {
public:
    CircularQueue()
        : data_(new int[kQueueInitCap]()), cap_(kQueueInitCap),
          head_(0), size_(0) {}

    CircularQueue(const CircularQueue&) = delete;
    CircularQueue& operator=(const CircularQueue&) = delete;

    ~CircularQueue() { delete[] data_; }

    int size() const { return size_; }
    int capacity() const { return cap_; }
    int head() const { return head_; }  // 只为自测观察回绕，不参与逻辑
    bool empty() const { return size_ == 0; }
    bool full() const { return size_ == cap_; }

    bool enqueue(int v) {
        if (size_ == cap_) grow();  // 先扩容，再写入
        // 队尾逻辑位置：head 往右数 size 格，取模把尾巴绕回头部
        data_[(head_ + size_) % cap_] = v;
        ++size_;
        return true;
    }

    // 空队列返回 false 且不改 out。
    bool dequeue(int& out) {
        if (size_ == 0) return false;
        out = data_[head_];
        head_ = (head_ + 1) % cap_;  // 只挪下标，不搬元素
        --size_;
        return true;
    }

    bool front(int& out) const {
        if (size_ == 0) return false;
        out = data_[head_];
        return true;
    }

    bool rear(int& out) const {
        if (size_ == 0) return false;
        // size-1 是队尾的偏移量；size >= 1 在这里已由判空保证
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
        // 关键：这一步用的是旧 head_、旧 cap_，所以映射关系一定正确。
        for (int i = 0; i < size_; ++i) nd[i] = data_[(head_ + i) % cap_];
        delete[] data_;
        data_ = nd;
        // 搬完之后才能改这两个量：先改就再也算不出旧下标了。
        cap_ = new_cap;
        head_ = 0;
    }

    int* data_;
    int cap_;
    int head_;  // 队首所在的物理下标
    int size_;  // 元素个数，空满全靠它区分
};

void print(const CircularQueue& q) {
    std::vector<int> v = q.to_vector();
    std::cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i > 0) std::cout << ", ";
        std::cout << v[i];
    }
    std::cout << "] size=" << q.size() << " cap=" << q.capacity()
              << " head=" << q.head() << "\n";
}

void expect(const CircularQueue& q, const std::vector<int>& want) {
    assert(q.to_vector() == want);
    assert(q.size() == static_cast<int>(want.size()));
}

int main() {
    // 用例 1：出两个进两个，反复 6 轮，让 head 回绕多圈（容量保持 4）。
    CircularQueue a;
    std::vector<int> m;  // 仅用于校验：当作 FIFO 的参考记录
    int next = 1;
    int v = 0;
    for (int i = 0; i < 4; ++i) {
        assert(a.enqueue(next));
        m.push_back(next);
        ++next;
    }
    assert(a.full() && a.size() == 4);
    for (int round = 0; round < 6; ++round) {
        for (int k = 0; k < 2; ++k) {
            assert(a.dequeue(v));
            assert(v == m.front());  // 回绕之后出队顺序依然正确
            m.erase(m.begin());
        }
        for (int k = 0; k < 2; ++k) {
            assert(a.enqueue(next));
            m.push_back(next);
            ++next;
        }
        assert(a.full() && a.size() == 4);
        expect(a, m);
        assert(a.head() == ((round + 1) * 2) % 4);  // head 每轮前进两格
    }
    assert(a.capacity() == 4);  // 全程没超过容量，没触发扩容
    print(a);

    // 用例 2：半满时再入队，物理下标会绕回头部。
    CircularQueue b;
    for (int i = 1; i <= 4; ++i) b.enqueue(i);  // 满，head=0
    assert(b.dequeue(v) && v == 1);
    assert(b.dequeue(v) && v == 2);             // head 走到 2
    assert(b.enqueue(5));                       // 写到下标 0
    assert(b.enqueue(6));                       // 写到下标 1
    expect(b, {3, 4, 5, 6});
    assert(b.head() == 2 && b.full());
    print(b);

    // 用例 3：满队列再入队触发扩容，head 归零且顺序不变。
    assert(b.enqueue(7));
    assert(b.capacity() == 8);
    assert(b.head() == 0);
    expect(b, {3, 4, 5, 6, 7});
    assert(b.front(v) && v == 3);  // 扩容后队首没变
    assert(b.rear(v) && v == 7);
    print(b);
    // 扩容后继续绕：出两个再进两个，不应该再扩容。
    assert(b.dequeue(v) && v == 3);
    assert(b.dequeue(v) && v == 4);
    assert(b.enqueue(8));
    assert(b.enqueue(9));
    assert(b.capacity() == 8);
    expect(b, {5, 6, 7, 8, 9});

    // 用例 4：空队列的每个出口。
    CircularQueue e;
    assert(e.empty() && !e.full() && e.size() == 0);
    int out = -12345;
    assert(!e.dequeue(out));
    assert(out == -12345);  // 失败时不能把出参改掉
    assert(!e.front(out));
    assert(!e.rear(out));
    expect(e, {});
    assert(e.enqueue(1));
    assert(e.front(out) && out == 1 && e.rear(out) && out == 1);
    e.clear();
    assert(e.empty() && e.size() == 0 && e.head() == 0);
    assert(e.enqueue(2));  // clear 之后还能继续用
    expect(e, {2});

    // 用例 5：长操作序列，size 与出队顺序逐步核对（参考模型是 vec）。
    CircularQueue q;
    std::vector<int> model;  // 仅用于校验：当作 FIFO 的参考记录
    int pushed = 0;
    for (int step = 0; step < 200; ++step) {
        if (step % 3 != 2) {
            assert(q.enqueue(pushed));
            model.push_back(pushed);
            ++pushed;
        } else if (!model.empty()) {
            int want = model.front();
            model.erase(model.begin());  // 参考模型可以直接 O(n) 删
            assert(q.dequeue(v) && v == want);
        }
        assert(q.size() == static_cast<int>(model.size()));
    }
    expect(q, model);

    std::cout << "all tests passed\n";
    return 0;
}
