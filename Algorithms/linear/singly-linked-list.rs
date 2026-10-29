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
  find         O(n)   返回下标，找不到返回 None
  reverse      O(n)   就地三指针，额外空间 O(1)
  to_vec       O(n)
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
  · head 永远是哑节点，真实元素从 (*head).next 开始，size 只数真实节点；
  · 空链表时 tail == head，凡是可能改动尾巴的地方都要维护这条；
  · 节点由 Box::into_raw 交出所有权，所以摘除时必须 Box::from_raw 收回
    再 drop，否则就是泄漏；摘除前先把 next 存下来；
  · 删掉的正好是 tail 时要让 tail 退回 prev，否则下次 push_back 会往
    已释放的节点上挂指针。

为什么这里用裸指针：尾插要 O(1)，就得有一个指向尾节点的「别名」，而节点
已经归前驱节点的 Box 所有。安全 Rust 只能靠 Rc<RefCell<..>> 绕，写出来
既慢又难读；这里用裸指针 + 明确的 unsafe 块，语义与 C++ 版本一一对应。

手推一遍：
  链表 1 -> 2 -> 3（哑节点 d 在最前），执行 remove_value(1)：
  1. prev=d 时 d.next=1 命中，cur=节点 1，先存 nxt = 节点 2；
  2. d.next = 节点 2；cur 不是 tail（tail 还是 3），Box::from_raw 回收 cur；
  3. 得到 2 -> 3，size 从 3 变 2。
  再执行 reverse：prev=null、cur=2，每次先把 next 存下来再反向箭头，
  三轮之后链表变 3 -> 2，tail 指向原来的头节点 2。

复杂度分析：
  时间：头尾插入与删除 O(1)，按下标/按值操作 O(n)，反转 O(n)。
  空间：每个元素一个节点，多一个指针字段，总计 O(n)；reverse 只用
    三个指针，额外 O(1)。
  取舍：没有随机访问，按下标访问是 O(n)，换来头尾插入删除 O(1)。
*/

#![allow(dead_code)]

use std::ptr::null_mut;

struct Node {
    val: i32,
    next: *mut Node,
}

struct SinglyLinkedList {
    head: *mut Node, // 哑节点，不存数据
    tail: *mut Node, // 指向最后一个真实节点，空表时等于 head
    size: usize,
}

impl SinglyLinkedList {
    fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            val: 0,
            next: null_mut(),
        }));
        SinglyLinkedList {
            head: dummy,
            tail: dummy,
            size: 0,
        }
    }

    // 新建一个脱离所有权的节点，之后由链表负责回收。
    fn make(v: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            val: v,
            next: null_mut(),
        }))
    }

    fn size(&self) -> usize {
        self.size
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    fn push_front(&mut self, v: i32) {
        let n = Self::make(v);
        unsafe {
            (*n).next = (*self.head).next;
            (*self.head).next = n;
        }
        if self.tail == self.head {
            self.tail = n; // 原本空表，新节点同时也是尾
        }
        self.size += 1;
    }

    fn push_back(&mut self, v: i32) {
        let n = Self::make(v);
        unsafe {
            (*self.tail).next = n; // 空表时 tail 就是哑节点，这一句照样成立
        }
        self.tail = n;
        self.size += 1;
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
            (*n).next = (*prev).next;
            (*prev).next = n;
            if (*n).next.is_null() {
                self.tail = n; // 接在了最后，尾指针跟着走
            }
        }
        self.size += 1;
        true
    }

    fn remove_at(&mut self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }
        let mut prev = self.head;
        unsafe {
            for _ in 0..index {
                prev = (*prev).next;
            }
            let cur = (*prev).next;
            let nxt = (*cur).next; // 先存 next，再回收 cur
            (*prev).next = nxt;
            if cur == self.tail {
                self.tail = prev; // 删的是尾巴，尾指针回退一格
            }
            drop(Box::from_raw(cur));
        }
        self.size -= 1;
        true
    }

    fn remove_value(&mut self, v: i32) -> bool {
        let mut prev = self.head;
        unsafe {
            // 停在「下一个就是目标」的位置，这样拿得到前驱
            while !(*prev).next.is_null() && (*(*prev).next).val != v {
                prev = (*prev).next;
            }
            if (*prev).next.is_null() {
                return false; // 值不存在
            }
            let cur = (*prev).next;
            let nxt = (*cur).next;
            (*prev).next = nxt;
            if cur == self.tail {
                self.tail = prev;
            }
            drop(Box::from_raw(cur));
        }
        self.size -= 1;
        true
    }

    fn find(&self, v: i32) -> Option<usize> {
        let mut i = 0;
        let mut p = unsafe { (*self.head).next };
        while !p.is_null() {
            if unsafe { (*p).val } == v {
                return Some(i);
            }
            p = unsafe { (*p).next };
            i += 1;
        }
        None
    }

    fn reverse(&mut self) {
        let mut prev: *mut Node = null_mut();
        let mut cur = unsafe { (*self.head).next };
        if !cur.is_null() {
            self.tail = cur; // 反转后原来的头变成尾
        }
        while !cur.is_null() {
            unsafe {
                let nxt = (*cur).next; // 三指针：先存后改，才不会断链
                (*cur).next = prev;
                prev = cur;
                cur = nxt;
            }
        }
        unsafe {
            (*self.head).next = prev; // 哑节点重新接上新的头
        }
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut p = unsafe { (*self.head).next };
        while !p.is_null() {
            out.push(unsafe { (*p).val });
            p = unsafe { (*p).next };
        }
        out
    }

    fn clear(&mut self) {
        let mut cur = unsafe { (*self.head).next };
        while !cur.is_null() {
            unsafe {
                let nxt = (*cur).next; // 先存 next 再回收
                drop(Box::from_raw(cur));
                cur = nxt;
            }
        }
        unsafe {
            (*self.head).next = null_mut();
        }
        self.tail = self.head;
        self.size = 0;
    }

    fn print(&self) {
        let parts: Vec<String> =
            self.to_vec().iter().map(|x| x.to_string()).collect();
        println!("[{}]", parts.join(", "));
    }
}

impl Drop for SinglyLinkedList {
    fn drop(&mut self) {
        self.clear();
        unsafe {
            drop(Box::from_raw(self.head)); // 哑节点也要还回去
        }
    }
}

fn main() {
    // 用例 1：头插尾插混用，顺序与 size 都要对。
    let mut a = SinglyLinkedList::new();
    assert!(a.empty() && a.size() == 0);
    a.push_back(2);
    a.push_front(1);
    a.push_back(3);
    assert_eq!(a.to_vec(), vec![1, 2, 3]);
    assert_eq!(a.find(1), Some(0));
    assert_eq!(a.find(3), Some(2));
    assert_eq!(a.find(9), None);
    a.print();

    // 用例 2：insert_at 的头、尾、中间与越界。
    assert!(a.insert_at(0, 0)); // 头
    assert!(a.insert_at(4, 4)); // 尾，index == size
    assert!(a.insert_at(2, 99)); // 中间
    assert!(!a.insert_at(99, 7)); // 越界
    assert_eq!(a.to_vec(), vec![0, 1, 99, 2, 3, 4]);
    assert_eq!(a.size(), 6);

    // 用例 3：删头、删尾、删中间，size 一并核对。
    assert!(a.remove_at(0));
    assert_eq!(a.to_vec(), vec![1, 99, 2, 3, 4]);
    assert!(a.remove_at(4)); // 删的是尾节点
    assert_eq!(a.to_vec(), vec![1, 99, 2, 3]);
    assert!(a.remove_at(1));
    assert_eq!(a.to_vec(), vec![1, 2, 3]);
    assert!(!a.remove_at(3));
    assert_eq!(a.size(), 3);

    // 用例 3b：关键点用例——删掉尾节点后 push_back 必须落在新的尾上。
    // 如果 remove_at 里漏了 tail = prev，这里会往已释放的内存写。
    a.push_back(4);
    assert_eq!(a.to_vec(), vec![1, 2, 3, 4]);

    // 用例 4：重复值只删第一个匹配。
    let mut b = SinglyLinkedList::new();
    b.push_back(1);
    b.push_back(2);
    b.push_back(1);
    b.push_back(3);
    assert!(b.remove_value(1)); // 只删最前面那个 1
    assert_eq!(b.to_vec(), vec![2, 1, 3]);
    assert!(b.remove_value(1));
    assert_eq!(b.to_vec(), vec![2, 3]);
    assert!(!b.remove_value(1)); // 已经没有了
    assert_eq!(b.size(), 2);

    // 用例 5：反转，反转后还能继续头尾插入（tail 必须同步）。
    let mut c = SinglyLinkedList::new();
    for i in 1..=4 {
        c.push_back(i);
    }
    c.reverse();
    assert_eq!(c.to_vec(), vec![4, 3, 2, 1]);
    c.push_back(9); // 反转后 tail 若是旧的，这里就会挂错位置
    assert_eq!(c.to_vec(), vec![4, 3, 2, 1, 9]);
    c.reverse();
    assert_eq!(c.to_vec(), vec![9, 1, 2, 3, 4]);

    // 用例 6：空链表与单元素链表。
    let mut e = SinglyLinkedList::new();
    assert!(!e.remove_value(1)); // 空表删值
    assert!(!e.remove_at(0)); // 空表删下标
    e.reverse(); // 空表反转不能崩
    assert!(e.empty() && e.size() == 0);
    e.push_back(5);
    e.reverse();
    assert_eq!(e.to_vec(), vec![5]);
    assert_eq!(e.find(5), Some(0));
    e.clear();
    assert!(e.empty() && e.to_vec().is_empty());
    e.push_back(6); // clear 之后还能用
    assert_eq!(e.to_vec(), vec![6]);

    println!("all tests passed");
}
