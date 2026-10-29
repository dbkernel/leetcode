/*
============================================================================
双向链表（Doubly Linked List）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_front        O(1)  插在头哨兵之后
  push_back         O(1)  插在尾哨兵之前
  remove_node       O(1)  拿到节点指针就能直接摘，不用找前驱
  remove_at         O(n)  按下标要先走
  remove_value      O(n)  按值要先找
  insert_at         O(n)  按下标要先走
  to_vector         O(n)  正向遍历
  to_vector_reverse O(n)  反向遍历
  size/empty        O(1)
============================================================================

用途：
  LRU 缓存那类「要 O(1) 摘掉任意一个已知节点」的场景；浏览器前进后退、
  编辑器的 undo/redo、以及需要双向遍历的任何链表题。

核心思想：
  每个节点同时记 prev 和 next，再在两端各放一个哨兵。有了 prev，删除
  一个已知节点只要改两个邻居的指针，一步到位；再加尾哨兵，删尾、插尾
  都变成普通操作，不必特判。

关键点（不变量）：
  · head_ 与 tail_ 都是哨兵，互相指向时链表为空，size 只数真实节点；
  · 插入新节点时先接好新节点的 prev/next，再改两个邻居的指针，
    顺序反了会把邻居之间的链子提前断掉；
  · 摘除节点要成对改线：n->prev->next = n->next 与 n->next->prev = n->prev，
    少改一根，反向遍历立刻错；
  · 有尾哨兵后「删最后一个节点」不是特例，tail_ 永远不用变。

手推一遍：
  链表 head <-> 1 <-> 2 <-> 3 <-> tail，执行 remove_node(节点 2)：
  1. node->prev 是 1、node->next 是 3；
  2. 1->next = 3、3->prev = 1；
  3. 得到 head <-> 1 <-> 3 <-> tail，size 3 -> 2，两个方向都还自洽。
  若换成删尾节点 3：3->next 是 tail 哨兵，于是 2->next = tail 且
  tail->prev = 2，一句话覆盖了单链表里需要特判 tail 的分支。

复杂度分析：
  时间：头尾插入 O(1)，删除「给定的节点」O(1)，这是它相对单链表的唯一
    核心优势（单链表删一个节点得先花 O(n) 找到前驱）。按下标/按值操作
    仍然要遍历，是 O(n)。
  空间：每个元素两个指针，比单链表多 8 字节/个，总计仍是 O(n)。
  稳定性：谈不到；取舍是多花指针换删除的便捷，随机访问依旧 O(n)。
*/

#include <cassert>
#include <iostream>
#include <vector>

struct DNode {
    int val;
    DNode* prev;
    DNode* next;
    explicit DNode(int v) : val(v), prev(nullptr), next(nullptr) {}
};

class DoublyLinkedList {
public:
    DoublyLinkedList() : head_(new DNode(0)), tail_(new DNode(0)), size_(0) {
        head_->next = tail_;  // 空表：两个哨兵直接互指
        tail_->prev = head_;
    }

    DoublyLinkedList(const DoublyLinkedList&) = delete;
    DoublyLinkedList& operator=(const DoublyLinkedList&) = delete;

    ~DoublyLinkedList() {
        clear();
        delete head_;  // 两个哨兵最后单独释放
        delete tail_;
    }

    int size() const { return size_; }
    bool empty() const { return size_ == 0; }

    void push_front(int v) { link_after(head_, new DNode(v)); }
    void push_back(int v) { link_after(tail_->prev, new DNode(v)); }

    // 按下标插入，index == size_ 等价于尾插。
    bool insert_at(int index, int v) {
        if (index < 0 || index > size_) return false;
        DNode* prev = head_;
        for (int i = 0; i < index; ++i) prev = prev->next;
        link_after(prev, new DNode(v));
        return true;
    }

    // 删除给定节点：O(1)，这正是双链表存在的理由。
    bool remove_node(DNode* node) {
        if (node == nullptr || node == head_ || node == tail_) return false;
        unlink(node);
        delete node;
        return true;
    }

    bool remove_at(int index) {
        if (index < 0 || index >= size_) return false;
        DNode* cur = head_->next;
        for (int i = 0; i < index; ++i) cur = cur->next;
        unlink(cur);
        delete cur;
        return true;
    }

    bool remove_value(int v) {
        DNode* cur = find_node(v);
        if (cur == nullptr) return false;
        unlink(cur);
        delete cur;
        return true;
    }

    // 返回节点句柄，找不到返回 nullptr；配合 remove_node 才是 O(1) 删除。
    DNode* find_node(int v) const {
        for (DNode* p = head_->next; p != tail_; p = p->next)
            if (p->val == v) return p;
        return nullptr;
    }

    std::vector<int> to_vector() const {
        std::vector<int> out;
        for (DNode* p = head_->next; p != tail_; p = p->next)
            out.push_back(p->val);
        return out;
    }

    std::vector<int> to_vector_reverse() const {
        std::vector<int> out;
        for (DNode* p = tail_->prev; p != head_; p = p->prev)
            out.push_back(p->val);
        return out;
    }

    void clear() {
        DNode* cur = head_->next;
        while (cur != tail_) {
            DNode* nxt = cur->next;  // 先存 next，再 delete
            delete cur;
            cur = nxt;
        }
        head_->next = tail_;
        tail_->prev = head_;
        size_ = 0;
    }

private:
    // 把 n 插到 prev 之后。先接新节点的两根线，再改邻居，避免提前断链。
    void link_after(DNode* prev, DNode* n) {
        n->prev = prev;
        n->next = prev->next;
        prev->next->prev = n;  // 顺序要紧：这一步必须在改 prev->next 之前
        prev->next = n;
        ++size_;
    }

    // 摘链：成对改两个邻居的指针，node 本身留给调用方回收。
    void unlink(DNode* node) {
        node->prev->next = node->next;
        node->next->prev = node->prev;
        --size_;
    }

    DNode* head_;  // 头哨兵
    DNode* tail_;  // 尾哨兵
    int size_;
};

void print(const DoublyLinkedList& l) {
    std::vector<int> v = l.to_vector();
    std::cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i > 0) std::cout << ", ";
        std::cout << v[i];
    }
    std::cout << "]";
}

// 正反两个方向必须互为逆序，任何一根指针写错都会在这里暴露。
void expect(const DoublyLinkedList& l, const std::vector<int>& want) {
    assert(l.to_vector() == want);
    std::vector<int> rev(want.rbegin(), want.rend());
    assert(l.to_vector_reverse() == rev);
    assert(l.size() == static_cast<int>(want.size()));
}

int main() {
    // 用例 1：头尾交替插入，正向与反向都要对。
    DoublyLinkedList a;
    assert(a.empty() && a.size() == 0);
    a.push_back(3);
    a.push_front(2);
    a.push_back(4);
    a.push_front(1);
    expect(a, {1, 2, 3, 4});
    print(a);
    std::cout << "\n";

    // 用例 2：删除中间节点（O(1) 删除给定节点）。
    DNode* mid = a.find_node(2);
    assert(mid != nullptr);
    assert(a.remove_node(mid));
    expect(a, {1, 3, 4});
    assert(a.find_node(2) == nullptr);  // 已经删掉

    // 用例 3：删除尾节点——有尾哨兵，不该有任何特判分支出错。
    DNode* last = a.find_node(4);
    assert(last != nullptr);
    assert(a.remove_node(last));
    expect(a, {1, 3});
    a.push_back(5);  // 删尾之后再 push_back 必须接在 3 后面
    expect(a, {1, 3, 5});

    // 用例 4：insert_at 的头、尾、中间，以及它与 push_front/push_back 的等价。
    DoublyLinkedList b;
    assert(b.insert_at(0, 10));  // 空表上插，等价 push_back
    expect(b, {10});
    assert(b.insert_at(0, 9));   // 等价 push_front
    assert(b.insert_at(2, 11));  // 等价 push_back
    assert(b.insert_at(2, 99));  // 中间
    assert(!b.insert_at(9, 7));
    assert(!b.insert_at(-1, 7));
    expect(b, {9, 10, 99, 11});

    // 用例 5：remove_at / remove_value 的头、尾、中间。
    assert(b.remove_at(0));
    expect(b, {10, 99, 11});
    assert(b.remove_at(2));
    expect(b, {10, 99});
    assert(b.remove_value(99));
    expect(b, {10});
    assert(!b.remove_value(99));
    assert(!b.remove_at(1));
    assert(b.size() == 1);

    // 用例 6：空表的每个出口，以及 clear 之后能继续用。
    DoublyLinkedList e;
    assert(!e.remove_node(nullptr));  // 空句柄
    assert(!e.remove_at(0));
    assert(!e.remove_value(1));
    assert(e.find_node(1) == nullptr);
    assert(e.to_vector().empty() && e.to_vector_reverse().empty());
    e.push_front(7);
    e.push_back(8);
    expect(e, {7, 8});
    e.clear();
    assert(e.empty() && e.to_vector().empty());
    assert(e.to_vector_reverse().empty());
    e.push_back(9);  // clear 之后还能用
    expect(e, {9});
    // 关键点用例：把节点一个个删空，再重新插入，两个哨兵必须重新互指。
    while (!e.empty()) assert(e.remove_at(0));
    expect(e, {});
    e.push_front(1);
    e.push_front(2);
    expect(e, {2, 1});

    std::cout << "all tests passed\n";
    return 0;
}
