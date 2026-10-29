r"""
============================================================================
二叉搜索树（Binary Search Tree）
分类：手写数据结构 · 树
操作复杂度：
  insert    O(h)  沿一条路径下降；重复值只把 cnt 加一
  find      O(h)
  erase     O(h)  双孩子要沿右子树最左一路找后继
  min/max   O(h)
  height    O(n)  递归量一遍（缓存子树高度可做到 O(1)）
  size      O(n)  递归把 cnt 累加（维护计数器可做到 O(1)）
  h 为树高，随机形态期望 O(log n)，退化成链时 O(n)
============================================================================

用途：
  「有序 + 动态插删」的最小可用结构，也是 AVL / 红黑树的原型。
  面试常考：插入、查找、删除节点（尤其双孩子）、验证 BST、第 k 小。

核心思想：
  对每个节点，左子树所有值都比它小，右子树所有值都比它大，于是查找
  每次都能砍掉一半子树。插入和删除都只沿一条从根到叶的路径走，改动
  只发生在路径上。删除双孩子节点时，把中序后继（右子树最小值）搬到
  当前位置，问题就退化成「删一个最多只有一个孩子的节点」。

关键点（不变量）：
  · 重复值策略选「计数」：相等时只把 cnt 加一，不新建节点，因此
    每个不同的值在树里恰好占一个节点，中序遍历按 cnt 重复输出；
  · 双孩子删除必须用中序后继：它是右子树最小值，天然大于左子树
    全部、小于右子树其余全部；且它没有左孩子，物理摘除退化成
    「至多一个孩子」的最简情况。随便挑一个节点会破坏 BST 有序性；
  · cnt > 1 时删除只减计数、绝不动结构，否则会把同值的其它出现
    一起删掉；
  · erase 找不到值时原树必须原样返回，不能顺手改指针。

手推一遍：
  依次插入 50 30 70 20 40 60 80，得到：

            50
          /    \
        30      70
       /  \    /  \
      20  40  60  80

  1. 插入 40 时走 50 -> 30 -> 40，40 比 30 大且 30 的右孩子为空，
     挂成 30 的右孩子；
  2. 删除 50：它有两个孩子，进右子树一路向左找到 60，把 60 搬到
     根，70 的左孩子置空；中序仍是 20 30 40 60 70 80，高度 3；
  3. 再插入一个 40：走到已有节点 40，只把它的 cnt 变成 2，节点数
     不变，中序变成 20 30 40 40 60 70 80。

复杂度分析：
  时间 O(h)：insert / find / erase / min / max 都只走一条路径；
        顺序插入会退化成链，h = n，这就是要 AVL 的原因。
  空间 O(h)：递归栈；节点本身 O(n)。
"""


class Node:
    __slots__ = ("val", "cnt", "left", "right")

    def __init__(self, val):
        self.val = val
        self.cnt = 1  # 该值出现次数，重复值不新建节点
        self.left = None
        self.right = None


class BST:
    def __init__(self):
        self.root = None

    # 返回 True 表示新建了节点；False 表示只是 cnt 加一
    def insert(self, v):
        created = False

        def rec(n):
            nonlocal created
            if n is None:
                created = True
                return Node(v)
            if v < n.val:
                n.left = rec(n.left)
            elif v > n.val:
                n.right = rec(n.right)
            else:
                n.cnt += 1  # 重复值只计数，保持「一值一节点」
            return n

        self.root = rec(self.root)
        return created

    def find(self, v):
        n = self.root
        while n is not None:
            if v < n.val:
                n = n.left
            elif v > n.val:
                n = n.right
            else:
                return True
        return False

    def count(self, v):
        """返回该值出现次数，不存在返回 0"""
        n = self.root
        while n is not None:
            if v < n.val:
                n = n.left
            elif v > n.val:
                n = n.right
            else:
                return n.cnt
        return 0

    def min_val(self):
        if self.root is None:
            return None
        n = self.root
        while n.left is not None:
            n = n.left
        return n.val

    def max_val(self):
        if self.root is None:
            return None
        n = self.root
        while n.right is not None:
            n = n.right
        return n.val

    @staticmethod
    def _pop_min(n):
        """摘掉子树里最小的节点：它没有左孩子，用右孩子顶替。

        返回 (新子树根, 被摘下的节点)。
        """
        if n.left is None:
            return n.right, n
        nxt, out = BST._pop_min(n.left)
        n.left = nxt
        return n, out

    # 返回 True 表示确实删掉了一次出现
    def erase(self, v):
        removed = False

        def rec(n):
            nonlocal removed
            if n is None:
                return None  # 空树删除：原样返回，不算删除成功
            if v < n.val:
                n.left = rec(n.left)
                return n
            if v > n.val:
                n.right = rec(n.right)
                return n
            removed = True
            if n.cnt > 1:
                n.cnt -= 1  # 还有同值的其它出现，只减少一次计数
                return n
            if n.left is None:
                return n.right
            if n.right is None:
                return n.left
            # 双孩子：中序后继没有左孩子，摘除它只需接上它的右孩子
            n.right, s = BST._pop_min(n.right)
            n.val = s.val
            n.cnt = s.cnt  # 后继可能自己也带重复计数，整份搬过来
            return n

        self.root = rec(self.root)
        return removed

    def height(self):
        def rec(n):
            if n is None:
                return 0
            l = rec(n.left)
            r = rec(n.right)
            return (l if l > r else r) + 1

        return rec(self.root)

    def size(self):
        def rec(n):
            if n is None:
                return 0
            return n.cnt + rec(n.left) + rec(n.right)

        return rec(self.root)

    def node_count(self):
        def rec(n):
            if n is None:
                return 0
            return 1 + rec(n.left) + rec(n.right)

        return rec(self.root)

    def inorder(self):
        """中序：每个值按 cnt 重复输出，所以整体非递减"""
        out = []

        def rec(n):
            if n is None:
                return
            rec(n.left)
            for _ in range(n.cnt):
                out.append(n.val)
            rec(n.right)

        rec(self.root)
        return out

    def check(self):
        """校验 BST 不变量：严格区间夹逼 + cnt >= 1"""
        def rec(n, lo, hi):
            if n is None:
                return True
            if n.cnt < 1:
                return False
            if not (lo < n.val < hi):
                return False
            return rec(n.left, lo, n.val) and rec(n.right, n.val, hi)

        return rec(self.root, -(1 << 62), 1 << 62)


def main():
    # 1. 正常用例：插入、查找、极值、高度、中序
    t = BST()
    for v in [50, 30, 70, 20, 40, 60, 80]:
        assert t.insert(v)
    assert t.size() == 7
    assert t.node_count() == 7
    assert t.height() == 3
    assert t.find(40)
    assert not t.find(41)
    assert t.min_val() == 20
    assert t.max_val() == 80
    assert t.check()
    assert t.inorder() == [20, 30, 40, 50, 60, 70, 80]
    print("inorder:", t.inorder())

    # 2. 重复值：只加计数，不新建节点
    t = BST()
    t.insert(40)
    assert not t.insert(40)
    assert not t.insert(40)
    assert t.count(40) == 3
    assert t.node_count() == 1
    assert t.size() == 3
    assert t.erase(40)
    assert t.count(40) == 2
    assert t.find(40)
    assert t.erase(40)
    assert t.erase(40)
    assert not t.find(40)
    assert not t.erase(40)  # 已经没了，删除失败
    assert t.node_count() == 0
    assert t.height() == 0

    # 3. 删除叶子 / 单孩子 / 根 / 双孩子
    t = BST()
    for v in [50, 30, 70, 20, 40, 60, 80]:
        t.insert(v)
    assert t.erase(20)  # 叶子
    assert not t.find(20)
    assert t.check()
    assert t.erase(60)
    assert t.erase(70)  # 70 此时只有右孩子 80
    assert t.find(80)
    assert t.check()
    assert t.erase(50)  # 双孩子，后继 80 顶上
    assert t.find(80)
    assert t.check()
    assert t.inorder() == [30, 40, 80]
    # 80 上位后左边还挂着 30 -> 40，所以高度是 3
    assert t.height() == 3

    # 4. 单孩子删根：根被唯一孩子顶替
    t = BST()
    t.insert(10)
    t.insert(5)
    t.insert(3)
    assert t.erase(10)
    assert t.inorder() == [3, 5]
    assert t.check()
    assert t.height() == 2
    assert t.min_val() == 3
    assert t.max_val() == 5

    # 5. 空树删除：返回 False，树保持为空
    t = BST()
    assert not t.erase(1)
    assert t.inorder() == []
    assert t.height() == 0
    assert t.min_val() is None
    assert t.max_val() is None
    assert t.size() == 0
    assert t.check()

    # 6. 双孩子 + 后继自身带重复计数：计数必须整份搬上来
    t = BST()
    t.insert(10)
    t.insert(5)
    t.insert(20)
    t.insert(20)
    assert t.count(20) == 2
    assert t.erase(10)  # 后继是 20(cnt=2)
    assert t.inorder() == [5, 20, 20]
    assert t.count(20) == 2
    assert t.node_count() == 2
    assert t.check()

    # 7. 中序严格递增（无重复时）
    t = BST()
    for i in range(64):
        t.insert(i * 3 - 50)
    seq = t.inorder()
    for i in range(1, len(seq)):
        assert seq[i - 1] < seq[i]
    assert t.check()

    # 8. 操作序列对拍：固定种子随机插删，与有序数组对照
    t = BST()
    ref = []
    seed = 12345
    for _ in range(400):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        v = ((seed >> 16) % 201) - 100
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        do_erase = (seed >> 16) % 2 == 0
        if do_erase:
            removed = t.erase(v)
            assert removed == (v in ref)
            if v in ref:
                ref.remove(v)  # 只删一个出现
        else:
            t.insert(v)
            at = 0
            while at < len(ref) and ref[at] < v:
                at += 1
            ref.insert(at, v)  # 手写插入，保持有序
        assert t.check()
        assert t.size() == len(ref)
        assert t.inorder() == ref
    print("random ops size:", t.size())

    print("all tests passed")


if __name__ == "__main__":
    main()
