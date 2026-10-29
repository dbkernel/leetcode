/*
============================================================================
手写优先队列（Priority Queue）
分类：手写数据结构 · 堆与优先队列
操作复杂度：
  push   O(log n)  追加到末尾后上浮
  pop    O(log n)  末尾补根后下沉
  peek   O(1)      根就是当前最优任务
  size/empty O(1)
============================================================================

用途：
  任务调度、事件模拟、带权最短路的取最小边。面试里常接着问一句
  「同优先级怎么保证先来先服务」，这就是下面的稳定化写法。

核心思想：
  和二叉堆一样用数组存完全二叉树，但比较的是二元组 (优先级, 插入序号)：
  优先级大的先出；优先级相同时插入序号小的先出。序号随 push 单调递增，
  于是同优先级天然按入队顺序出队，不需要额外队列。

关键点（不变量）：
  · 比较必须两级：先比 priority，相等再比 seq，且 seq 小的优先。
    只比 priority 时，等值元素的先后由堆的交换决定，顺序不稳定。
  · seq 由队列自己发号，调用方不能自己传，否则稳定性无从保证。
  · 上浮/下沉只走一条根到叶的路径，停的条件是「父不再更优」。
  · pop 后必须先把末尾元素搬到根再下沉，不能留空洞。

手推一遍：
  任务依次入队：(2,a)、(3,b)、(3,c)、(1,d)，序号分别是 0..3。
  1. a 进 -> 根 a。
  2. b 优先级 3 比 a 的 2 大，b 上浮成根。
  3. c 与 b 同级但序号 2 更大，b 仍在上，c 挂在下面。
  4. d 优先级最小，沉在叶子上。
  出队顺序：b(3,#1)、c(3,#2)、a(2,#0)、d(1,#3)，即 b c a d。

复杂度分析：
  时间 O(log n)：每次 push/pop 只沿一条链走，链长不超过树高 log2(n)。
  空间 O(n)：一个任务数组，元素为 (priority, seq, id) 三个 int。
  稳定性：稳定（靠 seq 次关键字），不是排序算法本身稳定。
*/
#include <cassert>
#include <iostream>
#include <vector>

struct Task {
    int priority;
    int seq;
    int id;
};

// 手写动态数组：只用 new[]/delete[]，不借用任何标准容器当底座。
class TaskArray {
public:
    TaskArray() : data_(nullptr), size_(0), cap_(0) {}
    ~TaskArray() { delete[] data_; }
    TaskArray(const TaskArray &) = delete;
    TaskArray &operator=(const TaskArray &) = delete;

    int size() const { return size_; }
    bool empty() const { return size_ == 0; }

    const Task &operator[](int i) const { return data_[i]; }
    Task &operator[](int i) { return data_[i]; }

    void push_back(const Task &task) {
        if (size_ == cap_) grow(cap_ == 0 ? 4 : cap_ * 2);
        data_[size_++] = task;
    }

    void pop_back() {
        if (size_ > 0) --size_;
    }

    const Task &back() const { return data_[size_ - 1]; }

    void clear() { size_ = 0; }

private:
    void grow(int new_cap) {
        Task *fresh = new Task[new_cap];
        for (int i = 0; i < size_; ++i) fresh[i] = data_[i];
        delete[] data_;
        data_ = fresh;
        cap_ = new_cap;
    }

    Task *data_;
    int size_;
    int cap_;
};

class PriorityQueue {
public:
    PriorityQueue() : next_seq_(0) {}

    int size() const { return data_.size(); }
    bool empty() const { return data_.empty(); }

    void clear() {
        data_.clear();
        next_seq_ = 0;
    }

    void push(int priority, int id) {
        Task task;
        task.priority = priority;
        task.seq = next_seq_++;
        task.id = id;
        data_.push_back(task);
        sift_up(data_.size() - 1);
    }

    // 空队列返回 false，调用方不用猜一个默认优先级。
    bool pop(int &priority, int &id) {
        if (data_.empty()) return false;
        priority = data_[0].priority;
        id = data_[0].id;
        Task last = data_.back();
        data_.pop_back();
        if (!data_.empty()) {
            data_[0] = last;
            sift_down(0);
        }
        return true;
    }

    bool peek(int &priority, int &id) const {
        if (data_.empty()) return false;
        priority = data_[0].priority;
        id = data_[0].id;
        return true;
    }

private:
    // 先比优先级，再比插入序号：这就是「堆 + 稳定化」的全部秘密。
    bool higher(const Task &a, const Task &b) const {
        if (a.priority != b.priority) return a.priority > b.priority;
        return a.seq < b.seq;
    }

    void sift_up(int i) {
        while (i > 0) {
            int parent = (i - 1) / 2;
            if (!higher(data_[i], data_[parent])) break;
            swap_task(i, parent);
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
            // 两个孩子都不更优就停，再往下扫只会破坏堆序。
            if (best == i) break;
            swap_task(i, best);
            i = best;
        }
    }

    void swap_task(int i, int j) {
        Task tmp = data_[i];
        data_[i] = data_[j];
        data_[j] = tmp;
    }

    TaskArray data_;
    int next_seq_;
};

// 小应用：按优先级从高到低输出任务名，同级保持入队顺序。
static std::vector<const char *> schedule(const int *priorities,
                                          const char *const *names, int n) {
    PriorityQueue queue;
    for (int i = 0; i < n; ++i) queue.push(priorities[i], i);

    std::vector<const char *> order;
    int priority = 0;
    int id = 0;
    while (queue.pop(priority, id)) order.push_back(names[id]);
    return order;
}

static void print_order(const char *title,
                        const std::vector<const char *> &order) {
    std::cout << title;
    for (size_t i = 0; i < order.size(); ++i) {
        std::cout << (i ? " " : "") << order[i];
    }
    std::cout << std::endl;
}

int main() {
    // 1. 正常用例 + 稳定性：同优先级必须按原顺序输出。
    {
        int priorities[] = {3, 1, 3, 2};
        const char *names[] = {"write", "coffee", "review", "test"};
        std::vector<const char *> got = schedule(priorities, names, 4);
        std::vector<const char *> want = {"write", "review", "test", "coffee"};
        assert(got == want);
        print_order("schedule order:", got);
    }

    // 2. 边界用例：空队列的 pop/peek 返回 false，clear 后序号重置。
    {
        PriorityQueue queue;
        int priority = -1;
        int id = -1;
        assert(!queue.pop(priority, id));
        assert(!queue.peek(priority, id));
        assert(queue.empty() && queue.size() == 0);

        queue.push(5, 100);
        assert(queue.size() == 1);
        assert(queue.pop(priority, id) && priority == 5 && id == 100);
        assert(queue.empty());
        assert(!queue.pop(priority, id));

        // 清空后可继续使用，序号从 0 重新发，顺序依然正确。
        queue.push(1, 0);
        queue.push(2, 1);
        assert(queue.pop(priority, id) && id == 1);
        queue.clear();
        assert(queue.empty() && queue.size() == 0);
    }

    // 3. 关键点用例：只比优先级会挂——三个同级任务的顺序必须原样。
    {
        PriorityQueue queue;
        for (int i = 0; i < 5; ++i) queue.push(7, i);
        assert(queue.size() == 5);
        for (int i = 0; i < 5; ++i) {
            int priority = 0;
            int id = -1;
            assert(queue.pop(priority, id));
            assert(priority == 7);
            assert(id == i);
        }
        assert(queue.empty());

        // 交错 push/pop：序号继续递增，后进的同级任务排在后面。
        queue.push(4, 10);
        queue.push(4, 11);
        int priority = 0;
        int id = -1;
        assert(queue.pop(priority, id) && id == 10);
        queue.push(4, 12);
        assert(queue.pop(priority, id) && id == 11);
        assert(queue.pop(priority, id) && id == 12);
    }

    // 4. 随机优先级序列：期望序列用两重循环按「优先级降序、原序」构造。
    {
        // 固定种子，保证失败可复现。
        unsigned seed = 20240501u;
        auto next_rand = [&seed]() -> unsigned {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            return seed;
        };

        const int n = 120;
        int priorities[120];
        const char *names[120];
        static const char *pool[] = {"task-a", "task-b", "task-c", "task-d"};
        int min_priority = 10;
        int max_priority = -1;
        for (int i = 0; i < n; ++i) {
            int value = static_cast<int>(next_rand() % 6);
            priorities[i] = value;
            names[i] = pool[i % 4];
            if (value < min_priority) min_priority = value;
            if (value > max_priority) max_priority = value;
        }

        // 期望值：优先级从高到低，同级按入队顺序扫一遍，不用任何排序。
        std::vector<int> expect_ids;
        for (int p = max_priority; p >= min_priority; --p) {
            for (int i = 0; i < n; ++i) {
                if (priorities[i] == p) expect_ids.push_back(i);
            }
        }

        PriorityQueue queue;
        for (int i = 0; i < n; ++i) queue.push(priorities[i], i);
        assert(queue.size() == n);

        std::vector<int> got_ids;
        int priority = 0;
        int id = -1;
        while (queue.pop(priority, id)) got_ids.push_back(id);
        assert(got_ids == expect_ids);

        // 弹出来的优先级必须是单调不增的，这一条独立于期望序列。
        int last = 1 << 30;
        for (size_t i = 0; i < got_ids.size(); ++i) {
            int p = priorities[got_ids[i]];
            assert(p <= last);
            last = p;
        }
        assert(queue.empty());
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
