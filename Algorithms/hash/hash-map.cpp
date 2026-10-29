/*
============================================================================
手写哈希表（Hash Map，链地址法）
分类：手写数据结构 · 哈希表
操作复杂度：
  put      O(1) 均摊  负载因子超过 0.75 就 2 倍扩容 rehash
  get      O(1) 均摊  最坏 O(n)，所有 key 挤在同一个桶
  remove   O(1) 均摊  单链表摘除，最坏 O(n)
  clear    O(n)       逐个 delete 节点
============================================================================

用途：
  需要 O(1) 增删查的 key-value 场景。面试里常要求手写链地址法，
  并追问扩容、rehash、负载因子、为什么容量取 2 的幂。

核心思想：
  桶数组的每个位置挂一条单链表，哈希值决定挂在哪个桶。容量是 2 的幂，
  所以取下标用 h & (cap-1) 代替取模（取模是整数除法，慢一个量级）。
  元素多了就 2 倍扩容并把每个节点重新挂到新桶，元素少了就缩容。

关键点（不变量）：
  · 任何时刻 size <= cap * 0.75，超了就扩容，保证链不太长。
  · 扩容后每个节点必须重新算下标：mask 从 cap-1 变成 2*cap-1，
    同一个 key 的低位变了，旧桶号完全可能失效，照搬就会丢数据。
  · 头插会让同一个桶里的顺序与插入顺序相反，但查找是「整条链扫完」，
    顺序不影响正确性；只有需要遍历有序性时才要改成尾插。
  · remove 必须拿到前驱再改 next，别把后面的整条链一起丢掉。

手推一遍：
  初始容量 8，mask = 7，依次 put(1,10)、put(3,30)、put(10,100)：
  1. mix(1) & 7 = 0，桶 0 空，挂上 -> 桶 0: 1。
  2. mix(3) & 7 = 5，桶 5 空，挂上 -> 桶 5: 3。
  3. mix(10) & 7 = 5，与 3 撞桶，头插 -> 桶 5: 10 -> 3。
  get(3)：先算到桶 5，沿链比 10 不等，再比 3 命中，返回 30。
  注意链上是 10 在前、3 在后，与插入顺序相反，但结果一样。

复杂度分析：
  时间 O(1) 均摊：负载因子封顶 0.75，链长期望长度是常数；扩容代价
  分摊到每次 put，等价 O(1)。最坏 O(n) 是所有 key 同桶。
  空间 O(n)：桶数组 cap 个指针 + 每个元素一个节点，cap <= 4n/3 左右。
  稳定性：不涉及顺序，遍历顺序由哈希值决定，不可依赖。
*/
#include <cassert>
#include <climits>
#include <cstdint>
#include <iostream>

class HashMap {
public:
    HashMap() : buckets_(nullptr), cap_(kMinCapacity), size_(0) {
        buckets_ = new Node *[cap_];
        for (int i = 0; i < cap_; ++i) buckets_[i] = nullptr;
    }

    ~HashMap() {
        free_nodes();
        delete[] buckets_;
    }

    HashMap(const HashMap &) = delete;
    HashMap &operator=(const HashMap &) = delete;

    int size() const { return size_; }
    int capacity() const { return cap_; }

    // 已存在就覆盖值，不算新键，也不触发扩容。
    void put(int key, int value) {
        int index = index_of(key);
        for (Node *node = buckets_[index]; node != nullptr; node = node->next) {
            if (node->key == key) {
                node->value = value;
                return;
            }
        }
        // 先判满再插：size+1 个元素必须装得下 0.75 的负载因子。
        if ((size_ + 1) * 4 > cap_ * 3) {
            rehash(cap_ * 2);
            index = index_of(key);
        }
        Node *node = new Node;
        node->key = key;
        node->value = value;
        node->next = buckets_[index];  // 头插，O(1)
        buckets_[index] = node;
        ++size_;
    }

    bool get(int key, int &out) const {
        for (Node *node = buckets_[index_of(key)]; node != nullptr;
             node = node->next) {
            if (node->key == key) {
                out = node->value;
                return true;
            }
        }
        return false;
    }

    bool contains(int key) const {
        int ignored = 0;
        return get(key, ignored);
    }

    bool remove(int key) {
        int index = index_of(key);
        Node *prev = nullptr;
        Node *node = buckets_[index];
        while (node != nullptr) {
            if (node->key == key) {
                if (prev != nullptr) {
                    prev->next = node->next;
                } else {
                    buckets_[index] = node->next;
                }
                delete node;
                --size_;
                maybe_shrink();
                return true;
            }
            prev = node;
            node = node->next;
        }
        return false;
    }

    void clear() {
        free_nodes();
        size_ = 0;
    }

    // 仅供自测：扫全部桶，检查每个节点都在正确的桶里、节点数等于 size。
    bool check_invariants() const {
        int counted = 0;
        for (int i = 0; i < cap_; ++i) {
            for (Node *node = buckets_[i]; node != nullptr;
                 node = node->next) {
                ++counted;
                if (index_of(node->key) != i) return false;
            }
        }
        if (counted != size_) return false;
        return static_cast<long long>(size_) * 4 <=
               static_cast<long long>(cap_) * 3;
    }

private:
    struct Node {
        int key;
        int value;
        Node *next;
    };

    static const int kMinCapacity = 8;

    // 对 int 的 32 位模式做乘法散列（murmur 风格的 finalizer）。
    // 先当无符号数看，符号位不再有特殊含义，负数 key 也能散匀。
    static uint32_t mix(int key) {
        uint32_t h = static_cast<uint32_t>(key);
        h ^= h >> 16;
        h *= 0x7feb352dU;
        h ^= h >> 15;
        h *= 0x846ca68bU;
        h ^= h >> 16;
        return h;
    }

    // 容量是 2 的幂，用 & 代替 %。
    int index_of(int key) const {
        return static_cast<int>(mix(key) & static_cast<uint32_t>(cap_ - 1));
    }

    void rehash(int new_cap) {
        Node **fresh = new Node *[new_cap];
        for (int i = 0; i < new_cap; ++i) fresh[i] = nullptr;

        uint32_t mask = static_cast<uint32_t>(new_cap - 1);
        for (int i = 0; i < cap_; ++i) {
            Node *node = buckets_[i];
            while (node != nullptr) {
                Node *next = node->next;
                // mask 变了，下标必须重算；沿用旧下标会把节点放错桶。
                uint32_t index = mix(node->key) & mask;
                node->next = fresh[index];
                fresh[index] = node;
                node = next;
            }
        }
        delete[] buckets_;
        buckets_ = fresh;
        cap_ = new_cap;
    }

    void maybe_shrink() {
        // 低于 0.25 才缩，且不低于最小容量；否则 put/remove 交替抖动。
        if (cap_ > kMinCapacity && size_ * 4 < cap_) rehash(cap_ / 2);
    }

    void free_nodes() {
        for (int i = 0; i < cap_; ++i) {
            Node *node = buckets_[i];
            while (node != nullptr) {
                Node *next = node->next;
                delete node;
                node = next;
            }
            buckets_[i] = nullptr;
        }
    }

    Node **buckets_;
    int cap_;
    int size_;
};

static bool is_power_of_two(int value) {
    return value > 0 && (value & (value - 1)) == 0;
}

int main() {
    // 1. 正常用例：put/get/contains，重复 put 只覆盖不新增。
    {
        HashMap map;
        assert(map.size() == 0 && map.capacity() == 8);

        map.put(42, 420);
        map.put(7, 70);
        assert(map.size() == 2);

        int value = 0;
        assert(map.get(42, value) && value == 420);
        assert(map.get(7, value) && value == 70);
        assert(map.contains(7));
        assert(!map.contains(8));

        map.put(42, 999);
        assert(map.size() == 2);
        assert(map.get(42, value) && value == 999);
        assert(map.check_invariants());
    }

    // 2. 边界用例：负数 key、int 两端极值。
    {
        HashMap map;
        int extremes[] = {0, -1, -1024, 1024, INT_MIN, INT_MAX, -2147483647};
        int count = static_cast<int>(sizeof(extremes) / sizeof(extremes[0]));
        // 用异或造值，避免 INT_MIN * 2 这种有符号溢出。
        for (int i = 0; i < count; ++i) {
            map.put(extremes[i], extremes[i] ^ 0x5a5a);
        }
        assert(map.size() == count);

        int value = 0;
        for (int i = 0; i < count; ++i) {
            assert(map.get(extremes[i], value));
            assert(value == (extremes[i] ^ 0x5a5a));
        }
        assert(!map.get(1, value));
        assert(map.check_invariants());
    }

    // 3. 关键点用例：同桶链表（头插逆行）也能查到、摘除中间节点不断链。
    {
        // 容量 8 时 mix(3)、mix(10)、mix(17) 的低三位都是 5，必然同桶。
        HashMap map;
        assert(map.capacity() == 8);
        map.put(3, 30);
        map.put(10, 100);
        map.put(17, 170);
        assert(map.capacity() == 8);
        assert(map.size() == 3);
        assert(map.check_invariants());

        int value = 0;
        assert(map.get(3, value) && value == 30);
        assert(map.get(10, value) && value == 100);
        assert(map.get(17, value) && value == 170);

        // 10 在链中间，摘掉它必须把前后接上。
        assert(map.remove(10));
        assert(!map.contains(10));
        assert(map.size() == 2);
        assert(map.get(3, value) && value == 30);
        assert(map.get(17, value) && value == 170);
        assert(map.check_invariants());

        // remove 不存在的 key 必须返回 false 且不动 size。
        assert(!map.remove(10));
        assert(!map.remove(12345));
        assert(map.size() == 2);
        assert(map.check_invariants());
    }

    // 4. 大量 key：触发多次扩容，全部仍可查到；随后缩容也不丢数据。
    {
        HashMap map;
        const int total = 5000;
        for (int i = 0; i < total; ++i) map.put(i, i * 3 + 1);
        assert(map.size() == total);
        assert(is_power_of_two(map.capacity()));
        assert(map.capacity() >= 8192);  // 5000/8192 < 0.75
        assert(map.check_invariants());

        int value = 0;
        for (int i = 0; i < total; ++i) {
            assert(map.get(i, value) && value == i * 3 + 1);
        }

        // 删掉一半，剩下的必须还能查到（扩容/缩容都要重算下标）。
        for (int i = 0; i < total; i += 2) assert(map.remove(i));
        assert(map.size() == total / 2);
        for (int i = 0; i < total; ++i) {
            bool found = map.get(i, value);
            assert(found == (i % 2 == 1));
            if (found) assert(value == i * 3 + 1);
        }
        assert(map.check_invariants());

        int big_capacity = map.capacity();
        for (int i = 1; i < total; i += 2) {
            if (i >= 1000) assert(map.remove(i));
        }
        assert(map.size() == 500);
        assert(map.capacity() < big_capacity);
        assert(is_power_of_two(map.capacity()));
        for (int i = 1; i < total; i += 2) {
            assert(map.contains(i) == (i < 1000));
        }
        assert(map.check_invariants());

        // clear 之后 size 归零、旧 key 全部查不到，且能继续使用。
        map.clear();
        assert(map.size() == 0);
        assert(!map.contains(1));
        assert(map.check_invariants());

        map.put(123, 456);
        assert(map.size() == 1);
        assert(map.get(123, value) && value == 456);
        assert(map.check_invariants());
    }

    std::cout << "all tests passed" << std::endl;
    return 0;
}
