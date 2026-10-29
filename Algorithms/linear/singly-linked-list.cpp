/*
============================================================================
单链表（Singly Linked List）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_front   O(1)   插到哑节点后面，不用判断空表
  push_back    O(1)   靠 tail 指针直接接在尾部
  insert_at    O(n)   先走到 index-1 再改指针
  remove_at    O(n)   同样要先走到前驱
  remove_value O(n)   遍历找第一个值相等的节点
  find         O(n)   返回下标，找不到返回 -1
  reverse      O(n)   就地三指针，额外空间 O(1)
  to_vector    O(n)
  size/empty   O(1)
============================================================================

用途：
  面试最高频的手写结构。所有「改指针」的题都建在它上面：反转、找中点、
  判环、合并两个有序链表、删除倒数第 k 个。

核心思想：
  每个节点只记 next。头插和删头本来要改外部的 head 指针、到处是 if，
  于是在最前面放一个不存数据的哑节点（sentinel），所有位置就统一成
  「改前一个节点的 next」，头尾一视同仁；再用 tail 指针把尾插降到 O(1)。

关键点（不变量）：
  · head_ 永远是哑节点，真实元素从 head_->next 开始，size 只数真实节点；
  · 空链表时 tail_ == head_，凡是可能改动尾巴的地方都要维护这条；
  · 删除节点前必须先保存 cur->next，delete 之后那块内存不能再读；
  · 删掉的正好是 tail_ 时 tail_ 要退回 prev，否则下次 push_back 会往
    已释放的节点上挂指针（这是最典型的 use-after-free）。

手推一遍：
  链表 1 -> 2 -> 3（哑节点 d 在最前），执行 remove_value(1)：
  1. prev=d 时 d->next=1 命中，cur=节点 1，先存 nxt = 节点 2；
  2. d->next = 节点 2；cur 不是 tail（tail 还是 3），delete cur；
  3. 得到 2 -> 3，size 从 3 变 2。
  再执行 reverse：prev=null、cur=2，每次先把 next 存下来再反向箭头，
  三轮之后链表变 3 -> 2，tail_ 指向原来的头节点 2。

复杂度分析：
  时间：头尾插入与删除 O(1)，按下标/按值操作 O(n)，反转 O(n)。
  空间：每个元素一个节点，指针开销 8 字节/个，总计 O(n)；reverse 只用
    三个指针，额外 O(1)。
  稳定性：谈不到稳定性；但没有随机访问，按下标访问是 O(n)，这是它和
    动态数组的核心取舍（插入删除便宜、访问贵）。
*/

#include <cassert>
#include <iostream>
#include <vector>

struct Node {
    int val;
    Node* next;
    explicit Node(int v) : val(v), next(nullptr) {}
};

class SinglyLinkedList {
public:
    SinglyLinkedList() : head_(new Node(0)), tail_(head_), size_(0) {}

    // 节点是裸指针，禁掉拷贝，避免两个对象指向同一批节点后二次释放。
    SinglyLinkedList(const SinglyLinkedList&) = delete;
    SinglyLinkedList& operator=(const SinglyLinkedList&) = delete;

    ~SinglyLinkedList() {
        clear();
        delete head_;  // 哑节点也是 new 出来的，最后单独释放
    }

    int size() const { return size_; }
    bool empty() const { return size_ == 0; }

    void push_front(int v) {
        Node* n = new Node(v);
        n->next = head_->next;
        head_->next = n;
        if (tail_ == head_) tail_ = n;  // 原本空表，新节点同时也是尾
        ++size_;
    }

    void push_back(int v) {
        Node* n = new Node(v);
        tail_->next = n;  // 空表时 tail_ 就是哑节点，这一句照样成立
        tail_ = n;
        ++size_;
    }

    bool insert_at(int index, int v) {
        // index == size_ 等价于尾插；越界直接拒绝。
        if (index < 0 || index > size_) return false;
        Node* prev = head_;
        for (int i = 0; i < index; ++i) prev = prev->next;
        Node* n = new Node(v);
        n->next = prev->next;
        prev->next = n;
        if (n->next == nullptr) tail_ = n;  // 接在了最后，尾指针跟着走
        ++size_;
        return true;
    }

    bool remove_at(int index) {
        if (index < 0 || index >= size_) return false;
        Node* prev = head_;
        for (int i = 0; i < index; ++i) prev = prev->next;
        Node* cur = prev->next;
        Node* nxt = cur->next;  // 先存 next，再 delete，顺序不能反
        prev->next = nxt;
        if (cur == tail_) tail_ = prev;  // 删的是尾巴，尾指针回退一格
        delete cur;
        --size_;
        return true;
    }

    bool remove_value(int v) {
        Node* prev = head_;
        // 停在「下一个就是目标」的位置，这样拿得到前驱
        while (prev->next != nullptr && prev->next->val != v) prev = prev->next;
        if (prev->next == nullptr) return false;  // 值不存在
        Node* cur = prev->next;
        Node* nxt = cur->next;
        prev->next = nxt;
        if (cur == tail_) tail_ = prev;
        delete cur;
        --size_;
        return true;
    }

    int find(int v) const {
        int i = 0;
        for (Node* p = head_->next; p != nullptr; p = p->next, ++i)
            if (p->val == v) return i;
        return -1;
    }

    void reverse() {
        Node* prev = nullptr;
        Node* cur = head_->next;
        if (cur != nullptr) tail_ = cur;  // 反转后原来的头变成尾
        while (cur != nullptr) {
            Node* nxt = cur->next;  // 三指针：先存后改，才不会断链
            cur->next = prev;
            prev = cur;
            cur = nxt;
        }
        head_->next = prev;  // 哑节点重新接上新的头
    }

    std::vector<int> to_vector() const {
        std::vector<int> out;
        for (Node* p = head_->next; p != nullptr; p = p->next)
            out.push_back(p->val);
        return out;
    }

    void clear() {
        Node* cur = head_->next;
        while (cur != nullptr) {
            Node* nxt = cur->next;  // 同样是先存 next 再 delete
            delete cur;
            cur = nxt;
        }
        head_->next = nullptr;
        tail_ = head_;
        size_ = 0;
    }

private:
    Node* head_;  // 哑节点，不存数据
    Node* tail_;  // 指向最后一个真实节点，空表时等于 head_
    int size_;
};

void print(const SinglyLinkedList& l) {
    std::vector<int> v = l.to_vector();
    std::cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i > 0) std::cout << ", ";
        std::cout << v[i];
    }
    std::cout << "]\n";
}

void expect(const SinglyLinkedList& l, const std::vector<int>& want) {
    assert(l.to_vector() == want);
    assert(l.size() == static_cast<int>(want.size()));
}

int main() {
    // 用例 1：头插尾插混用，顺序与 size 都要对。
    SinglyLinkedList a;
    assert(a.empty() && a.size() == 0);
    a.push_back(2);
    a.push_front(1);
    a.push_back(3);
    expect(a, {1, 2, 3});
    assert(a.find(1) == 0 && a.find(3) == 2 && a.find(9) == -1);
    print(a);

    // 用例 2：insert_at 的头、尾、中间与越界。
    assert(a.insert_at(0, 0));   // 头
    assert(a.insert_at(4, 4));   // 尾，index == size
    assert(a.insert_at(2, 99));  // 中间
    assert(!a.insert_at(99, 7));
    assert(!a.insert_at(-1, 7));
    expect(a, {0, 1, 99, 2, 3, 4});
    assert(a.size() == 6);

    // 用例 3：删头、删尾、删中间，size 一并核对。
    assert(a.remove_at(0));
    expect(a, {1, 99, 2, 3, 4});
    assert(a.remove_at(4));  // 删的是尾节点
    expect(a, {1, 99, 2, 3});
    assert(a.remove_at(1));
    expect(a, {1, 2, 3});
    assert(!a.remove_at(3));
    assert(!a.remove_at(-1));

    // 用例 3b：关键点用例——删掉尾节点后 push_back 必须落在新的尾上。
    // 如果 remove_at 里漏了 tail_ = prev，这里会往已释放的内存写。
    a.push_back(4);
    expect(a, {1, 2, 3, 4});

    // 用例 4：重复值只删第一个匹配。
    SinglyLinkedList b;
    b.push_back(1);
    b.push_back(2);
    b.push_back(1);
    b.push_back(3);
    assert(b.remove_value(1));  // 只删最前面那个 1
    expect(b, {2, 1, 3});
    assert(b.remove_value(1));
    expect(b, {2, 3});
    assert(!b.remove_value(1));  // 已经没有了
    assert(b.size() == 2);

    // 用例 5：反转，反转后还能继续头尾插入（tail_ 必须同步）。
    SinglyLinkedList c;
    for (int i = 1; i <= 4; ++i) c.push_back(i);
    c.reverse();
    expect(c, {4, 3, 2, 1});
    c.push_back(9);  // 反转后 tail_ 若是旧的会挂到 1 后面，结果就错了
    expect(c, {4, 3, 2, 1, 9});
    c.reverse();
    expect(c, {9, 1, 2, 3, 4});

    // 用例 6：空链表与单元素链表。
    SinglyLinkedList e;
    assert(!e.remove_value(1));  // 空表删值
    assert(!e.remove_at(0));     // 空表删下标
    e.reverse();                 // 空表反转不能崩
    assert(e.empty() && e.empty());
    e.push_back(5);
    e.reverse();
    expect(e, {5});
    assert(e.find(5) == 0);
    e.clear();
    assert(e.empty() && e.to_vector().empty());
    e.push_back(6);  // clear 之后还能用
    expect(e, {6});

    std::cout << "all tests passed\n";
    return 0;
}
