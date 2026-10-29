r"""
============================================================================
AVL 树（AVL Tree）
分类：手写数据结构 · 树
操作复杂度：
  height   O(1)     每个节点缓存子树高度
  find     O(log n)
  insert   O(log n) 回溯更新高度，至多一次旋转（单旋或双旋）
  erase    O(log n) 双孩子先搬后继，再从后继父节点回溯
  rotate   O(1)     单次旋转只改三条指针、修正两个高度
  空间 O(n)；树高恒为 O(log n)，最坏也是，不会退化成链
============================================================================

用途：
  需要「有序 + 稳定 O(log n)」时的首选手写结构；面试里常用来考
  「你知不知道旋转之后要回溯修高度」「删除为什么比重插入难」。

核心思想：
  AVL 在 BST 的有序性之上再加一条：每个节点左右子树高度差不超过 1。
  插入 / 删除后只有从改动点到根这一条路径上的节点可能失衡，于是回溯
  时自底向上修高度，一旦某节点 bf 变成 +2 或 -2，就按失衡形态做
  一次旋转把高度压回去。旋转不改变中序序列，所以有序性自动保持。

关键点（不变量）：
  · 先修自己的高度，再看 bf；旋转返回新的子树根，调用方必须接住；
  · 单旋后两个节点的高度都要重算：先算子节点，再算新根，顺序反了
    就会把旧高度写回去；
  · 删除双孩子节点后，必须从「后继的父节点」开始回溯，不能只修到
    被删节点为止。这一段是 AVL 删除最容易漏的地方：后继在右子树
    深处，它的祖先高度全变了，漏掉就会出现 bf 已经越界却没人旋转；
  · 重复值直接返回、不计入、不旋转，保证树里每个值唯一。

手推一遍：
  依次插入 30 20 10：
  1. 插 30 后 h=1；插 20 挂在左边，30 的 bf=1；
  2. 插 10 仍往左，30 的 bf=2，且左孩子 20 的 bf=1，属于 LL：
     对 30 右旋，20 升为根，30 成为 20 的右孩子，20 原来的右子树
     （空）挂到 30 左边；先修 30 的高度为 1，再修 20 为 2；
  3. 若插入序列是 30 10 20，则 30 的 bf=2 而左孩子 10 的 bf=-1，
     属于 LR：先对 10 左旋得到 30 -> 20 -> 10 的形状，再对 30
     右旋，最终 20 为根，10、30 为两个孩子。

复杂度分析：
  时间 O(log n)：树高被 bf <= 1 卡在 1.44 log2(n+2) 附近；
        insert / erase 各走一条根到叶的路径，旋转 O(1)。
  空间 O(n) 存节点，O(log n) 递归栈。
"""


class Node:
    __slots__ = ("val", "height", "left", "right")

    def __init__(self, val):
        self.val = val
        # 以该节点为根的子树高度，空树记 0，叶子记 1
        self.height = 1
        self.left = None
        self.right = None


def h(n):
    return 0 if n is None else n.height


def bf(n):
    return 0 if n is None else h(n.left) - h(n.right)


def fix(n):
    l = h(n.left)
    r = h(n.right)
    n.height = (l if l > r else r) + 1


# LL 失衡：左孩子的左子树过高，对 t 右旋
def rotate_ll(t):
    x = t.left
    t.left = x.right  # x 的右子树夹在 t 和 x 之间，仍然有序
    x.right = t
    fix(t)  # 先算子节点，再算新根
    fix(x)
    return x


# RR 失衡：右孩子的右子树过高，对 t 左旋
def rotate_rr(t):
    x = t.right
    t.right = x.left
    x.left = t
    fix(t)
    fix(x)
    return x


# LR 失衡：先对左孩子左旋，把它变成 LL，再对自己右旋
def rotate_lr(t):
    t.left = rotate_rr(t.left)
    return rotate_ll(t)


# RL 失衡：先对右孩子右旋，把它变成 RR，再对自己左旋
def rotate_rl(t):
    t.right = rotate_ll(t.right)
    return rotate_rr(t)


def rebalance(t):
    fix(t)
    b = bf(t)
    if b > 1:
        # 左孩子 bf >= 0 是 LL；< 0 说明新节点插在左孩子的右边
        if bf(t.left) >= 0:
            return rotate_ll(t)
        return rotate_lr(t)
    if b < -1:
        if bf(t.right) <= 0:
            return rotate_rr(t)
        return rotate_rl(t)
    return t


def insert_rec(t, v, state):
    if t is None:
        state[0] = True
        return Node(v)
    if v < t.val:
        t.left = insert_rec(t.left, v, state)
    elif v > t.val:
        t.right = insert_rec(t.right, v, state)
    else:
        return t  # 重复值不插入，也不必修高度
    return rebalance(t)


def erase_rec(t, v, state):
    if t is None:
        return None
    if v < t.val:
        t.left = erase_rec(t.left, v, state)
    elif v > t.val:
        t.right = erase_rec(t.right, v, state)
    else:
        state[0] = True
        if t.left is None or t.right is None:
            # 叶子或单孩子：孩子顶上，高度由调用方 rebalance 修
            return t.left if t.left is not None else t.right
        # 双孩子：中序后继（右子树最小值）的值搬上来
        s = t.right
        while s.left is not None:
            s = s.left
        t.val = s.val
        successor = s.val
        # 关键：真正删的是右子树里的后继，回溯必须从右子树内部
        # 一路 rebalance 上来，否则后继的祖先高度会停在旧值
        t.right = erase_rec(t.right, successor, state)
    return rebalance(t)


class AVL:
    def __init__(self):
        self.root = None

    def insert(self, v):
        state = [False]
        self.root = insert_rec(self.root, v, state)
        return state[0]

    def erase(self, v):
        state = [False]
        self.root = erase_rec(self.root, v, state)
        return state[0]

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

    def height(self):
        return h(self.root)

    def root_val(self):
        return self.root.val  # 调用前保证非空

    def size(self):
        def rec(n):
            if n is None:
                return 0
            return 1 + rec(n.left) + rec(n.right)

        return rec(self.root)

    def inorder(self):
        out = []

        def rec(n):
            if n is None:
                return
            rec(n.left)
            out.append(n.val)
            rec(n.right)

        rec(self.root)
        return out

    def check_balanced(self):
        """同时验证 BST 严格递增、|bf| <= 1、缓存高度没写错"""
        state = [False, 0]  # [has_prev, prev]

        def rec(n):
            if n is None:
                return True
            if not rec(n.left):
                return False
            if state[0] and not (state[1] < n.val):
                return False  # BST 必须严格递增
            state[0] = True
            state[1] = n.val
            if not rec(n.right):
                return False
            diff = h(n.left) - h(n.right)
            if diff > 1 or diff < -1:
                return False
            l = h(n.left)
            r = h(n.right)
            return n.height == (l if l > r else r) + 1

        return rec(self.root)


def main():
    # 1. 顺序插入 1..100，高度不能超过理论上界 9
    t = AVL()
    for v in range(1, 101):
        assert t.insert(v)
        assert t.check_balanced()
    assert t.size() == 100
    # 高度为 h 的 AVL 最少节点数 N(h)=N(h-1)+N(h-2)+1，
    # N(9)=88 <= 100 < N(10)=143，所以高度最多 9
    assert t.height() <= 9
    assert t.height() >= 7
    assert t.inorder() == list(range(1, 101))
    print("1..100 height:", t.height())

    # 2. 四种旋转各一个最小用例，最终都应是 20 为根、高度 2
    ll = AVL()
    ll.insert(30)
    ll.insert(20)
    ll.insert(10)
    assert ll.root_val() == 20 and ll.height() == 2
    assert ll.inorder() == [10, 20, 30]
    assert ll.check_balanced()

    rr = AVL()
    rr.insert(10)
    rr.insert(20)
    rr.insert(30)
    assert rr.root_val() == 20 and rr.height() == 2
    assert rr.inorder() == [10, 20, 30]
    assert rr.check_balanced()

    lr = AVL()
    lr.insert(30)
    lr.insert(10)
    lr.insert(20)
    assert lr.root_val() == 20 and lr.height() == 2
    assert lr.inorder() == [10, 20, 30]
    assert lr.check_balanced()

    rl = AVL()
    rl.insert(10)
    rl.insert(30)
    rl.insert(20)
    assert rl.root_val() == 20 and rl.height() == 2
    assert rl.inorder() == [10, 20, 30]
    assert rl.check_balanced()

    # 3. 重复值不插入
    t = AVL()
    assert t.insert(5)
    assert not t.insert(5)
    assert not t.insert(5)
    assert t.size() == 1
    assert t.height() == 1
    assert t.inorder() == [5]
    assert t.check_balanced()
    assert not t.erase(4)  # 不存在，不算删除
    assert t.size() == 1

    # 4. 删除：叶子 / 单孩子 / 根 / 空树
    t = AVL()
    for v in range(1, 21):
        t.insert(v)
    assert t.erase(1)  # 叶子
    assert not t.find(1)
    assert t.check_balanced()
    while t.size() > 1:  # 逐个删到只剩根
        v = t.inorder()[0]
        assert t.erase(v)
        assert t.check_balanced()
    assert t.erase(t.root_val())  # 删根
    assert t.size() == 0 and t.height() == 0
    assert t.check_balanced()
    assert not t.erase(42)  # 空树删除
    assert t.size() == 0

    # 5. 双孩子删除：后继在右子树深处，回溯必须覆盖后继的祖先
    t = AVL()
    for v in [50, 25, 75, 10, 30, 60, 90, 5, 15, 27, 35, 55, 65,
              80, 95, 1, 7, 12, 18]:
        t.insert(v)
    assert t.check_balanced()
    # 50 有双孩子，后继是 55，删完从 55 的父节点一路修回根
    assert t.erase(50)
    assert not t.find(50)
    assert t.find(55)
    assert t.check_balanced()
    assert t.size() == 18

    # 6. 随机插入 / 删除对拍：固定种子，与有序数组对照
    t = AVL()
    ref = []
    seed = 20240601
    for _ in range(1600):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        v = ((seed >> 16) % 201) - 100
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        do_erase = (seed >> 16) % 2 == 0
        if do_erase:
            removed = t.erase(v)
            assert removed == (v in ref)
            if v in ref:
                ref.remove(v)  # ref 中的值互不重复
        else:
            inserted = t.insert(v)
            assert inserted == (v not in ref)
            if v not in ref:
                at = 0
                while at < len(ref) and ref[at] < v:
                    at += 1
                ref.insert(at, v)  # 手写插入，保持有序
        # 每一步都必须满足：BST 有序 + |bf| <= 1 + 高度缓存正确
        assert t.check_balanced()
        assert t.size() == len(ref)
        assert t.inorder() == ref
    print("random ops size:", t.size(), "height:", t.height())
    # 再把剩下的按升序删空，验证连续的删除路径也能保持平衡
    while ref:
        v = ref.pop(0)
        assert t.erase(v)
        assert t.check_balanced()
        assert t.inorder() == ref
    assert t.size() == 0 and t.height() == 0

    print("all tests passed")


if __name__ == "__main__":
    main()
