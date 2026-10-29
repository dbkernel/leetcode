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
  to_vec            O(n)  正向遍历
  to_vec_reverse    O(n)  反向遍历
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
  · head 与 tail 都是哨兵，互相指向时链表为空，size 只数真实节点；
  · 插入新节点时先接好新节点的 prev/next，再改两个邻居的指针，
    顺序反了会把邻居之间的链子提前断掉；
  · 摘除节点要成对改线：(*nd).prev 的 next 与 (*nd).next 的 prev，
    少改一根，反向遍历立刻错；
  · 有尾哨兵后「删最后一个节点」不是特例，tail 永远不用变。

为什么用裸指针：与 C++ 版本一一对应，节点的所有权仍然只有一份（由链表
统一 Box::from_raw 回收），这里只是多存了一组「别名」指针，不做引用计数。

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
  空间：每个元素两个指针，比单链表多一个指针字段，总计仍是 O(n)。
  取舍：多花指针换删除的便捷，随机访问依旧 O(n)。
*/

#![allow(dead_code)]

use std::ptr::null_mut;

struct DNode {
    val: i32,
    prev: *mut DNode,
    next: *mut DNode,
}

struct DoublyLinkedList {
    head: *mut DNode, // 头哨兵
    tail: *mut DNode, // 尾哨兵
    size: usize,
}

impl DoublyLinkedList {
    fn new() -> Self {
        unsafe {
            let head = Box::into_raw(Box::new(DNode {
                val: 0,
                prev: null_mut(),
                next: null_mut(),
            }));
            let tail = Box::into_raw(Box::new(DNode {
                val: 0,
                prev: null_mut(),
                next: null_mut(),
            }));
            (*head).next = tail; // 空表：两个哨兵直接互指
            (*tail).prev = head;
            DoublyLinkedList {
                head,
                tail,
                size: 0,
            }
        }
    }

    fn make(v: i32) -> *mut DNode {
        Box::into_raw(Box::new(DNode {
            val: v,
            prev: null_mut(),
            next: null_mut(),
        }))
    }

    fn size(&self) -> usize {
        self.size
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    // 把 nd 插到 prev 之后。先接新节点的两根线，再改邻居，避免提前断链。
    fn link_after(&mut self, prev: *mut DNode, nd: *mut DNode) {
        unsafe {
            (*nd).prev = prev;
            (*nd).next = (*prev).next;
            (*(*prev).next).prev = nd; // 必须在改 prev.next 之前
            (*prev).next = nd;
        }
        self.size += 1;
    }

    // 摘链：成对改两个邻居的指针，nd 本身留给调用方回收。
    fn unlink(&mut self, nd: *mut DNode) {
        unsafe {
            (*(*nd).prev).next = (*nd).next;
            (*(*nd).next).prev = (*nd).prev;
        }
        self.size -= 1;
    }

    fn push_front(&mut self, v: i32) {
        let n = Self::make(v);
        let head = self.head;
        self.link_after(head, n);
    }

    fn push_back(&mut self, v: i32) {
        let n = Self::make(v);
        let prev = unsafe { (*self.tail).prev };
        self.link_after(prev, n);
    }

    fn insert_at(&mut self, index: usize, v: i32) -> bool {
        // index == size 等价于尾插（usize 天然排除负下标）。
        if index > self.size {
            return false;
        }
        let mut prev = self.head;
        unsafe {
            for _ in 0..index {
                prev = (*prev).next;
            }
            let n = Self::make(v);
            self.link_after(prev, n);
        }
        true
    }

    // 删除给定节点：O(1)，这正是双链表存在的理由。
    fn remove_node(&mut self, nd: *mut DNode) -> bool {
        if nd.is_null() || nd == self.head || nd == self.tail {
            return false;
        }
        self.unlink(nd);
        unsafe {
            drop(Box::from_raw(nd)); // 收回所有权并释放
        }
        true
    }

    fn remove_at(&mut self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }
        let mut cur = unsafe { (*self.head).next };
        unsafe {
            for _ in 0..index {
                cur = (*cur).next;
            }
        }
        self.remove_node(cur)
    }

    fn remove_value(&mut self, v: i32) -> bool {
        let cur = self.find_node(v);
        if cur.is_null() {
            return false;
        }
        self.remove_node(cur)
    }

    // 返回节点指针，找不到返回空指针；配合 remove_node 才是 O(1) 删除。
    fn find_node(&self, v: i32) -> *mut DNode {
        let mut p = unsafe { (*self.head).next };
        while p != self.tail {
            if unsafe { (*p).val } == v {
                return p;
            }
            p = unsafe { (*p).next };
        }
        null_mut()
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut p = unsafe { (*self.head).next };
        while p != self.tail {
            out.push(unsafe { (*p).val });
            p = unsafe { (*p).next };
        }
        out
    }

    fn to_vec_reverse(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut p = unsafe { (*self.tail).prev };
        while p != self.head {
            out.push(unsafe { (*p).val });
            p = unsafe { (*p).prev };
        }
        out
    }

    fn clear(&mut self) {
        let mut cur = unsafe { (*self.head).next };
        while cur != self.tail {
            unsafe {
                let nxt = (*cur).next;
                drop(Box::from_raw(cur));
                cur = nxt;
            }
        }
        unsafe {
            (*self.head).next = self.tail;
            (*self.tail).prev = self.head;
        }
        self.size = 0;
    }

    fn print(&self) {
        let parts: Vec<String> =
            self.to_vec().iter().map(|x| x.to_string()).collect();
        print!("[{}]", parts.join(", "));
    }
}

impl Drop for DoublyLinkedList {
    fn drop(&mut self) {
        self.clear();
        unsafe {
            drop(Box::from_raw(self.head)); // 两个哨兵最后单独还回去
            drop(Box::from_raw(self.tail));
        }
    }
}

fn main() {
    // 用例 1：头尾交替插入，正向与反向都要对。
    let mut a = DoublyLinkedList::new();
    assert!(a.empty() && a.size() == 0);
    a.push_back(3);
    a.push_front(2);
    a.push_back(4);
    a.push_front(1);
    assert_eq!(a.to_vec(), vec![1, 2, 3, 4]);
    assert_eq!(a.to_vec_reverse(), vec![4, 3, 2, 1]);
    a.print();
    println!();

    // 用例 2：删除中间节点（O(1) 删除给定节点）。
    let mid = a.find_node(2);
    assert!(!mid.is_null());
    assert!(a.remove_node(mid));
    assert_eq!(a.to_vec(), vec![1, 3, 4]);
    assert_eq!(a.to_vec_reverse(), vec![4, 3, 1]);
    assert!(a.find_node(2).is_null()); // 已经删掉

    // 用例 3：删除尾节点——有尾哨兵，不该有任何特判分支出错。
    let last = a.find_node(4);
    assert!(!last.is_null());
    assert!(a.remove_node(last));
    assert_eq!(a.to_vec(), vec![1, 3]);
    assert_eq!(a.to_vec_reverse(), vec![3, 1]);
    a.push_back(5); // 删尾之后再 push_back 必须接在 3 后面
    assert_eq!(a.to_vec(), vec![1, 3, 5]);
    assert_eq!(a.to_vec_reverse(), vec![5, 3, 1]);

    // 用例 4：insert_at 的头、尾、中间，以及它与 push_front/push_back 的等价。
    let mut b = DoublyLinkedList::new();
    assert!(b.insert_at(0, 10)); // 空表上插，等价 push_back
    assert_eq!(b.to_vec(), vec![10]);
    assert!(b.insert_at(0, 9)); // 等价 push_front
    assert!(b.insert_at(2, 11)); // 等价 push_back
    assert!(b.insert_at(2, 99)); // 中间
    assert!(!b.insert_at(9, 7));
    assert_eq!(b.to_vec(), vec![9, 10, 99, 11]);
    assert_eq!(b.to_vec_reverse(), vec![11, 99, 10, 9]);

    // 用例 5：remove_at / remove_value 的头、尾、中间。
    assert!(b.remove_at(0));
    assert_eq!(b.to_vec(), vec![10, 99, 11]);
    assert!(b.remove_at(2));
    assert_eq!(b.to_vec(), vec![10, 99]);
    assert!(b.remove_value(99));
    assert_eq!(b.to_vec(), vec![10]);
    assert!(!b.remove_value(99));
    assert!(!b.remove_at(1));
    assert_eq!(b.size(), 1);

    // 用例 6：空表的每个出口，以及 clear 之后能继续用。
    let mut e = DoublyLinkedList::new();
    assert!(!e.remove_node(null_mut())); // 空句柄
    assert!(!e.remove_at(0));
    assert!(!e.remove_value(1));
    assert!(e.find_node(1).is_null());
    assert!(e.to_vec().is_empty() && e.to_vec_reverse().is_empty());
    e.push_front(7);
    e.push_back(8);
    assert_eq!(e.to_vec(), vec![7, 8]);
    assert_eq!(e.to_vec_reverse(), vec![8, 7]);
    e.clear();
    assert!(e.empty() && e.to_vec().is_empty());
    assert!(e.to_vec_reverse().is_empty());
    e.push_back(9); // clear 之后还能用
    assert_eq!(e.to_vec(), vec![9]);
    // 关键点用例：把节点一个个删空，再重新插入，两个哨兵必须重新互指。
    while !e.empty() {
        assert!(e.remove_at(0));
    }
    assert!(e.to_vec().is_empty() && e.to_vec_reverse().is_empty());
    e.push_front(1);
    e.push_front(2);
    assert_eq!(e.to_vec(), vec![2, 1]);
    assert_eq!(e.to_vec_reverse(), vec![1, 2]);

    println!("all tests passed");
}
