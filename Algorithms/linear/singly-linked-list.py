"""
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
  to_list      O(n)
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
  · _head 永远是哑节点，真实元素从 _head.next 开始，size 只数真实节点；
  · 空链表时 _tail is _head，凡是可能改动尾巴的地方都要维护这条；
  · 摘除节点前先存好 next，再把指针接好，最后才丢弃 cur；
  · 删掉的正好是 _tail 时要让 _tail 退回 prev，否则下次 push_back 会把
    新节点挂到链表之外，长度和内容立刻对不上。

手推一遍：
  链表 1 -> 2 -> 3（哑节点 d 在最前），执行 remove_value(1)：
  1. prev=d 时 d.next=1 命中，cur=节点 1，先存 nxt = 节点 2；
  2. d.next = 节点 2；cur 不是 tail（tail 还是 3），可以安全丢弃 cur；
  3. 得到 2 -> 3，size 从 3 变 2。
  再执行 reverse：prev=None、cur=2，每次先把 next 存下来再反向箭头，
  三轮之后链表变 3 -> 2，_tail 指向原来的头节点 2。

复杂度分析：
  时间：头尾插入与删除 O(1)，按下标/按值操作 O(n)，反转 O(n)。
  空间：每个元素一个节点对象，总计 O(n)；reverse 只用三个引用，额外 O(1)。
  取舍：没有随机访问，按下标访问是 O(n)，换来头尾插入删除 O(1)。
"""


class _Node:
    def __init__(self, val):
        self.val = val
        self.next = None


class SinglyLinkedList:
    def __init__(self):
        self._head = _Node(0)  # 哑节点，不存数据
        self._tail = self._head  # 空表时尾就是哑节点
        self._size = 0

    def size(self):
        return self._size

    def empty(self):
        return self._size == 0

    def push_front(self, v):
        n = _Node(v)
        n.next = self._head.next
        self._head.next = n
        if self._tail is self._head:
            self._tail = n  # 原本空表，新节点同时也是尾
        self._size += 1

    def push_back(self, v):
        n = _Node(v)
        self._tail.next = n  # 空表时 _tail 就是哑节点，这一句照样成立
        self._tail = n
        self._size += 1

    def insert_at(self, index, v):
        # index == size 等价于尾插；越界直接拒绝。
        if index < 0 or index > self._size:
            return False
        prev = self._head
        for _ in range(index):
            prev = prev.next
        n = _Node(v)
        n.next = prev.next
        prev.next = n
        if n.next is None:
            self._tail = n  # 接在了最后，尾指针跟着走
        self._size += 1
        return True

    def remove_at(self, index):
        if index < 0 or index >= self._size:
            return False
        prev = self._head
        for _ in range(index):
            prev = prev.next
        cur = prev.next
        nxt = cur.next  # 先存 next，再摘除 cur
        prev.next = nxt
        if cur is self._tail:
            self._tail = prev  # 删的是尾巴，尾指针回退一格
        cur.next = None  # 主动断开，避免留下一条指向链表内部的引用
        self._size -= 1
        return True

    def remove_value(self, v):
        prev = self._head
        # 停在「下一个就是目标」的位置，这样拿得到前驱
        while prev.next is not None and prev.next.val != v:
            prev = prev.next
        if prev.next is None:
            return False  # 值不存在
        cur = prev.next
        prev.next = cur.next
        if cur is self._tail:
            self._tail = prev
        cur.next = None
        self._size -= 1
        return True

    def find(self, v):
        i = 0
        p = self._head.next
        while p is not None:
            if p.val == v:
                return i
            p = p.next
            i += 1
        return -1

    def reverse(self):
        prev = None
        cur = self._head.next
        if cur is not None:
            self._tail = cur  # 反转后原来的头变成尾
        while cur is not None:
            nxt = cur.next  # 三指针：先存后改，才不会断链
            cur.next = prev
            prev = cur
            cur = nxt
        self._head.next = prev  # 哑节点重新接上新的头

    def to_list(self):
        out = []
        p = self._head.next
        while p is not None:
            out.append(p.val)
            p = p.next
        return out

    def clear(self):
        self._head.next = None
        self._tail = self._head
        self._size = 0

    def print(self):
        print("[", end="")
        for i, v in enumerate(self.to_list()):
            if i > 0:
                print(", ", end="")
            print(v, end="")
        print("]")


def expect(l, want):
    assert l.to_list() == want, "content mismatch"
    assert l.size() == len(want), "size mismatch"


def main():
    # 用例 1：头插尾插混用，顺序与 size 都要对。
    a = SinglyLinkedList()
    assert a.empty() and a.size() == 0
    a.push_back(2)
    a.push_front(1)
    a.push_back(3)
    expect(a, [1, 2, 3])
    assert a.find(1) == 0 and a.find(3) == 2 and a.find(9) == -1
    a.print()

    # 用例 2：insert_at 的头、尾、中间与越界。
    assert a.insert_at(0, 0)    # 头
    assert a.insert_at(4, 4)    # 尾，index == size
    assert a.insert_at(2, 99)   # 中间
    assert not a.insert_at(99, 7)
    assert not a.insert_at(-1, 7)
    expect(a, [0, 1, 99, 2, 3, 4])
    assert a.size() == 6

    # 用例 3：删头、删尾、删中间，size 一并核对。
    assert a.remove_at(0)
    expect(a, [1, 99, 2, 3, 4])
    assert a.remove_at(4)  # 删的是尾节点
    expect(a, [1, 99, 2, 3])
    assert a.remove_at(1)
    expect(a, [1, 2, 3])
    assert not a.remove_at(3)
    assert not a.remove_at(-1)

    # 用例 3b：关键点用例——删掉尾节点后 push_back 必须落在新的尾上。
    a.push_back(4)
    expect(a, [1, 2, 3, 4])

    # 用例 4：重复值只删第一个匹配。
    b = SinglyLinkedList()
    b.push_back(1)
    b.push_back(2)
    b.push_back(1)
    b.push_back(3)
    assert b.remove_value(1)  # 只删最前面那个 1
    expect(b, [2, 1, 3])
    assert b.remove_value(1)
    expect(b, [2, 3])
    assert not b.remove_value(1)  # 已经没有了
    assert b.size() == 2

    # 用例 5：反转，反转后还能继续头尾插入（_tail 必须同步）。
    c = SinglyLinkedList()
    for i in range(1, 5):
        c.push_back(i)
    c.reverse()
    expect(c, [4, 3, 2, 1])
    c.push_back(9)  # 反转后 _tail 若是旧的，这里就会挂错位置
    expect(c, [4, 3, 2, 1, 9])
    c.reverse()
    expect(c, [9, 1, 2, 3, 4])

    # 用例 6：空链表与单元素链表。
    e = SinglyLinkedList()
    assert not e.remove_value(1)  # 空表删值
    assert not e.remove_at(0)     # 空表删下标
    e.reverse()                   # 空表反转不能崩
    assert e.empty() and e.size() == 0
    e.push_back(5)
    e.reverse()
    expect(e, [5])
    assert e.find(5) == 0
    e.clear()
    assert e.empty() and e.to_list() == []
    e.push_back(6)  # clear 之后还能用
    expect(e, [6])

    print("all tests passed")


if __name__ == "__main__":
    main()
