"""
============================================================================
双向链表（Doubly Linked List）
分类：手写数据结构 · 线性结构

操作复杂度：
  push_front        O(1)  插在头哨兵之后
  push_back         O(1)  插在尾哨兵之前
  remove_node       O(1)  拿到节点引用就能直接摘，不用找前驱
  remove_at         O(n)  按下标要先走
  remove_value      O(n)  按值要先找
  insert_at         O(n)  按下标要先走
  to_list           O(n)  正向遍历
  to_list_reverse   O(n)  反向遍历
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
  · _head 与 _tail 都是哨兵，互相指向时链表为空，size 只数真实节点；
  · 插入新节点时先接好新节点的 prev/next，再改两个邻居的指针，
    顺序反了会把邻居之间的链子提前断掉；
  · 摘除节点要成对改线：node.prev.next = node.next 与
    node.next.prev = node.prev，少改一根，反向遍历立刻错；
  · 有尾哨兵后「删最后一个节点」不是特例，_tail 永远不用变。

手推一遍：
  链表 head <-> 1 <-> 2 <-> 3 <-> tail，执行 remove_node(节点 2)：
  1. node.prev 是 1、node.next 是 3；
  2. 1.next = 3、3.prev = 1；
  3. 得到 head <-> 1 <-> 3 <-> tail，size 3 -> 2，两个方向都还自洽。
  若换成删尾节点 3：3.next 是 tail 哨兵，于是 2.next = tail 且
  tail.prev = 2，一句话覆盖了单链表里需要特判 tail 的分支。

复杂度分析：
  时间：头尾插入 O(1)，删除「给定的节点」O(1)，这是它相对单链表的唯一
    核心优势（单链表删一个节点得先花 O(n) 找到前驱）。按下标/按值操作
    仍然要遍历，是 O(n)。
  空间：每个元素两个引用，比单链表多一个字段，总计仍是 O(n)。
  取舍：多花指针换删除的便捷，随机访问依旧 O(n)。
"""


class _DNode:
    def __init__(self, val):
        self.val = val
        self.prev = None
        self.next = None


class DoublyLinkedList:
    def __init__(self):
        self._head = _DNode(0)  # 头哨兵
        self._tail = _DNode(0)  # 尾哨兵
        self._head.next = self._tail  # 空表：两个哨兵直接互指
        self._tail.prev = self._head
        self._size = 0

    def size(self):
        return self._size

    def empty(self):
        return self._size == 0

    def _link_after(self, prev, node):
        # 先接新节点的两根线，再改邻居，避免提前断链。
        node.prev = prev
        node.next = prev.next
        prev.next.prev = node  # 顺序要紧：这一步必须在改 prev.next 之前
        prev.next = node
        self._size += 1

    def _unlink(self, node):
        # 成对改两个邻居的指针，少改一根反向遍历就错。
        node.prev.next = node.next
        node.next.prev = node.prev
        self._size -= 1

    def push_front(self, v):
        self._link_after(self._head, _DNode(v))

    def push_back(self, v):
        self._link_after(self._tail.prev, _DNode(v))

    def insert_at(self, index, v):
        # index == size 等价于尾插。
        if index < 0 or index > self._size:
            return False
        prev = self._head
        for _ in range(index):
            prev = prev.next
        self._link_after(prev, _DNode(v))
        return True

    def remove_node(self, node):
        """删除给定节点：O(1)，这正是双链表存在的理由。"""
        if node is None or node is self._head or node is self._tail:
            return False
        self._unlink(node)
        node.prev = None
        node.next = None  # 主动断开，避免留下指向链表内部的引用
        return True

    def remove_at(self, index):
        if index < 0 or index >= self._size:
            return False
        cur = self._head.next
        for _ in range(index):
            cur = cur.next
        return self.remove_node(cur)

    def remove_value(self, v):
        cur = self.find_node(v)
        if cur is None:
            return False
        return self.remove_node(cur)

    def find_node(self, v):
        """返回节点引用，找不到返回 None；配合 remove_node 才是 O(1) 删除。"""
        p = self._head.next
        while p is not self._tail:
            if p.val == v:
                return p
            p = p.next
        return None

    def to_list(self):
        out = []
        p = self._head.next
        while p is not self._tail:
            out.append(p.val)
            p = p.next
        return out

    def to_list_reverse(self):
        out = []
        p = self._tail.prev
        while p is not self._head:
            out.append(p.val)
            p = p.prev
        return out

    def clear(self):
        self._head.next = self._tail
        self._tail.prev = self._head
        self._size = 0

    def print(self):
        print("[", end="")
        for i, v in enumerate(self.to_list()):
            if i > 0:
                print(", ", end="")
            print(v, end="")
        print("]", end="")


def expect(l, want):
    # 正反两个方向必须互为逆序，任何一根指针写错都会在这里暴露。
    assert l.to_list() == want, "forward mismatch"
    assert l.to_list_reverse() == want[::-1], "backward mismatch"
    assert l.size() == len(want), "size mismatch"


def main():
    # 用例 1：头尾交替插入，正向与反向都要对。
    a = DoublyLinkedList()
    assert a.empty() and a.size() == 0
    a.push_back(3)
    a.push_front(2)
    a.push_back(4)
    a.push_front(1)
    expect(a, [1, 2, 3, 4])
    a.print()
    print()

    # 用例 2：删除中间节点（O(1) 删除给定节点）。
    mid = a.find_node(2)
    assert mid is not None
    assert a.remove_node(mid)
    expect(a, [1, 3, 4])
    assert a.find_node(2) is None  # 已经删掉

    # 用例 3：删除尾节点——有尾哨兵，不该有任何特判分支出错。
    last = a.find_node(4)
    assert last is not None
    assert a.remove_node(last)
    expect(a, [1, 3])
    a.push_back(5)  # 删尾之后再 push_back 必须接在 3 后面
    expect(a, [1, 3, 5])

    # 用例 4：insert_at 的头、尾、中间，以及它与 push_front/push_back 的等价。
    b = DoublyLinkedList()
    assert b.insert_at(0, 10)   # 空表上插，等价 push_back
    expect(b, [10])
    assert b.insert_at(0, 9)    # 等价 push_front
    assert b.insert_at(2, 11)   # 等价 push_back
    assert b.insert_at(2, 99)   # 中间
    assert not b.insert_at(9, 7)
    assert not b.insert_at(-1, 7)
    expect(b, [9, 10, 99, 11])

    # 用例 5：remove_at / remove_value 的头、尾、中间。
    assert b.remove_at(0)
    expect(b, [10, 99, 11])
    assert b.remove_at(2)
    expect(b, [10, 99])
    assert b.remove_value(99)
    expect(b, [10])
    assert not b.remove_value(99)
    assert not b.remove_at(1)
    assert b.size() == 1

    # 用例 6：空表的每个出口，以及 clear 之后能继续用。
    e = DoublyLinkedList()
    assert not e.remove_node(None)  # 空句柄
    assert not e.remove_at(0)
    assert not e.remove_value(1)
    assert e.find_node(1) is None
    assert e.to_list() == [] and e.to_list_reverse() == []
    e.push_front(7)
    e.push_back(8)
    expect(e, [7, 8])
    e.clear()
    assert e.empty() and e.to_list() == [] and e.to_list_reverse() == []
    e.push_back(9)  # clear 之后还能用
    expect(e, [9])
    # 关键点用例：把节点一个个删空，再重新插入，两个哨兵必须重新互指。
    while not e.empty():
        assert e.remove_at(0)
    expect(e, [])
    e.push_front(1)
    e.push_front(2)
    expect(e, [2, 1])

    print("all tests passed")


if __name__ == "__main__":
    main()
