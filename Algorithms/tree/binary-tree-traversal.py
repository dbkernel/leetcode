r"""
============================================================================
二叉树遍历（Binary Tree Traversal）
分类：手写数据结构 · 树
复杂度：时间 O(n)：四种遍历都恰好访问每个节点一次；
        空间 O(h)：递归栈或显式栈，h 为树高；Morris 中序 O(1)
============================================================================

用途：
  树的入门动作，也是序列化 / 反序列化、树形 DP 的底座。面试常问
  「不用递归写后序」「不用栈和递归遍历，空间 O(1)」。

核心思想：
  三种深度优先序的差别只在「输出根」放在两次递归的哪一侧：前序在
  进左子树之前，中序在左子树回来之后，后序在右子树回来之后。把递归
  栈换成一个自己维护的数组就是迭代版，层序把栈换成先进先出的循环
  队列。Morris 中序借叶子结点空闲的右指针指向「中序后继」，走完左
  子树后顺着这根线回到祖先，用完立刻拆掉。

关键点（不变量）：
  · 迭代前序：先压右孩子再压左孩子，栈顶才是左孩子；
  · 迭代后序：last 记录上一个真正输出的节点，只有右孩子为空或
    右孩子 == last 时才允许输出栈顶，否则先把右子树走完；
  · Morris：prev.right == cur 表示第二次到达 cur，必须把
    prev.right 还原成 None，否则树里留下环；
  · 层序队列用 count 记录元素个数：回绕后 head == tail 无法区分
    空与满，只看 head / tail 会误判。

手推一遍：
  以层序数组 {1,2,3,4,5,-1,6,-1,-1,7} 建出的树：

            1
          /   \
         2     3
        / \     \
       4   5     6
          /
         7

  1. 前序：输出 1，进左 2，进左 4 输出 4，回 2 进右 5，进左 7
     输出 7，回到 5 输出 5，回到 2 输出 2，回根进右 3，进右 6，
     得 1 2 4 5 7 3 6；
  2. 中序：一路向左到 4 输出，回 2 输出，进 5 先到 7 输出，
     得 4 2 7 5 1 3 6；
  3. 后序得 4 7 5 2 6 3 1；层序按出队顺序得 1 2 3 4 5 6 7。

复杂度分析：
  时间 O(n)：每节点进栈、出栈各一次；Morris 每条边最多走两遍。
  空间 O(h)：递归栈 / 显式栈；层序 O(w)，w 为最大层宽；
        Morris 只用两个指针，O(1)。
"""

class Node:
    __slots__ = ("val", "left", "right")

    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class RingQueue:
    """自己写的循环队列：count 区分空与满，容量不够就翻倍。"""

    __slots__ = ("buf", "head", "tail", "count")

    def __init__(self):
        self.buf = [None] * 8
        self.head = 0
        self.tail = 0
        self.count = 0

    def empty(self):
        return self.count == 0

    def push(self, x):
        if self.count == len(self.buf):
            n = len(self.buf)
            nb = [None] * (n * 2)
            for i in range(self.count):
                nb[i] = self.buf[(self.head + i) % n]
            self.buf = nb
            self.head = 0
            self.tail = self.count
        self.buf[self.tail] = x
        self.tail = (self.tail + 1) % len(self.buf)
        self.count += 1

    def pop(self):
        x = self.buf[self.head]
        self.head = (self.head + 1) % len(self.buf)
        self.count -= 1
        return x


class BinTree:
    def __init__(self):
        self.root = None

    def build(self, a):
        """层序数组建树，-1 表示空位。仅用于构造。"""
        self.root = None
        if not a or a[0] == -1:
            return
        self.root = Node(a[0])
        q = RingQueue()
        q.push(self.root)
        i = 1
        while not q.empty() and i < len(a):
            cur = q.pop()
            if a[i] != -1:
                cur.left = Node(a[i])
                q.push(cur.left)
            i += 1
            if i < len(a):
                if a[i] != -1:
                    cur.right = Node(a[i])
                    q.push(cur.right)
                i += 1

    def build_left_chain(self, n):
        """只有左孩子的链，值从根往下是 n, n-1, ..., 1"""
        self.root = None
        for v in range(1, n + 1):
            self.root = Node(v, left=self.root)

    def build_right_chain(self, n):
        """只有右孩子的链，值从根往下同样是 n, n-1, ..., 1"""
        self.root = None
        for v in range(1, n + 1):
            self.root = Node(v, right=self.root)

    def to_level_array(self):
        """层序序列化，空位写 -1 并去掉尾部空位。"""
        out = []
        q = RingQueue()
        q.push(self.root)
        while not q.empty():
            t = q.pop()
            if t is None:
                out.append(-1)
                continue
            out.append(t.val)
            q.push(t.left)
            q.push(t.right)
        while out and out[-1] == -1:
            out.pop()
        return out

    # ---------- 递归版 ----------
    def preorder(self):
        out = []
        self._pre(self.root, out)
        return out

    def _pre(self, t, out):
        if t is None:
            return
        out.append(t.val)
        self._pre(t.left, out)
        self._pre(t.right, out)

    def inorder(self):
        out = []
        self._in(self.root, out)
        return out

    def _in(self, t, out):
        if t is None:
            return
        self._in(t.left, out)
        out.append(t.val)
        self._in(t.right, out)

    def postorder(self):
        out = []
        self._post(self.root, out)
        return out

    def _post(self, t, out):
        if t is None:
            return
        self._post(t.left, out)
        self._post(t.right, out)
        out.append(t.val)

    # ---------- 显式栈迭代版 ----------
    def preorder_iter(self):
        out = []
        if self.root is None:
            return out
        st = [self.root]
        while st:
            t = st.pop()
            out.append(t.val)
            # 先压右再压左：后进先出，左孩子先被处理
            if t.right is not None:
                st.append(t.right)
            if t.left is not None:
                st.append(t.left)
        return out

    def inorder_iter(self):
        out = []
        st = []
        cur = self.root
        while cur is not None or st:
            while cur is not None:
                st.append(cur)
                cur = cur.left
            cur = st.pop()
            out.append(cur.val)
            cur = cur.right
        return out

    def postorder_iter(self):
        out = []
        st = []
        cur = self.root
        last = None  # 上一个真正输出的节点
        while cur is not None or st:
            while cur is not None:
                st.append(cur)
                cur = cur.left
            top = st[-1]
            # 右子树存在且还没走完，先转过去，top 留在栈上
            if top.right is not None and top.right is not last:
                cur = top.right
            else:
                out.append(top.val)
                last = top
                st.pop()
        return out

    # ---------- 层序：自己写的循环队列 ----------
    def level_order(self):
        out = []
        if self.root is None:
            return out
        q = RingQueue()
        q.push(self.root)
        while not q.empty():
            t = q.pop()
            out.append(t.val)
            if t.left is not None:
                q.push(t.left)
            if t.right is not None:
                q.push(t.right)
        return out

    # ---------- Morris 中序：O(1) 额外空间 ----------
    def morris_inorder(self):
        out = []
        cur = self.root
        while cur is not None:
            if cur.left is None:
                out.append(cur.val)
                cur = cur.right
                continue
            prev = cur.left
            while prev.right is not None and prev.right is not cur:
                prev = prev.right
            if prev.right is None:
                prev.right = cur  # 穿线
                cur = cur.left
            else:
                prev.right = None  # 拆线，树恢复原状
                out.append(cur.val)
                cur = cur.right
        return out


def main():
    # 1. 固定形状的树，验证四种遍历序列
    t = BinTree()
    t.build([1, 2, 3, 4, 5, -1, 6, -1, -1, 7])
    pre = [1, 2, 4, 5, 7, 3, 6]
    ino = [4, 2, 7, 5, 1, 3, 6]
    post = [4, 7, 5, 2, 6, 3, 1]
    lvl = [1, 2, 3, 4, 5, 6, 7]
    assert t.preorder() == pre
    assert t.inorder() == ino
    assert t.postorder() == post
    assert t.preorder_iter() == pre
    assert t.inorder_iter() == ino
    assert t.postorder_iter() == post
    assert t.level_order() == lvl
    # Morris 必须与递归中序一致
    assert t.morris_inorder() == ino
    # 遍历（含 Morris 穿线拆线）之后树结构不能变
    assert t.to_level_array() == [1, 2, 3, 4, 5, -1, 6, -1, -1, 7]
    assert t.preorder() == pre
    print("fixed tree preorder:", t.preorder())

    # 2. 单节点
    t = BinTree()
    t.build([5])
    assert t.preorder() == [5]
    assert t.inorder() == [5]
    assert t.postorder() == [5]
    assert t.preorder_iter() == [5]
    assert t.inorder_iter() == [5]
    assert t.postorder_iter() == [5]
    assert t.level_order() == [5]
    assert t.morris_inorder() == [5]
    assert t.to_level_array() == [5]

    # 3. 空树
    t = BinTree()
    assert t.preorder() == []
    assert t.postorder() == []
    assert t.preorder_iter() == []
    assert t.inorder_iter() == []
    assert t.postorder_iter() == []
    assert t.level_order() == []
    assert t.morris_inorder() == []
    assert t.to_level_array() == []
    # 空树里 -1 只表示空位，不产生节点
    t.build([-1, 1, 2])
    assert t.to_level_array() == []

    # 4. 只有左 / 右孩子的长链：验证显式栈不塌
    n = 500
    asc = list(range(1, n + 1))
    desc = list(range(n, 0, -1))
    lt = BinTree()
    lt.build_left_chain(n)
    assert lt.preorder_iter() == desc
    assert lt.inorder_iter() == asc
    assert lt.postorder_iter() == asc
    assert lt.level_order() == desc
    assert lt.morris_inorder() == asc

    rt = BinTree()
    rt.build_right_chain(n)
    assert rt.preorder_iter() == desc
    assert rt.inorder_iter() == desc
    assert rt.postorder_iter() == asc
    assert rt.level_order() == desc
    assert rt.morris_inorder() == desc

    print("all tests passed")


if __name__ == "__main__":
    main()
