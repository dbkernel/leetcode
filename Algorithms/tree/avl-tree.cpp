/*
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
*/
#include <cassert>
#include <iostream>
#include <vector>

struct Node {
    int val;
    int height;  // 以该节点为根的子树高度，空树记 0，叶子记 1
    Node* left;
    Node* right;
    explicit Node(int v) : val(v), height(1), left(nullptr), right(nullptr) {}
};

struct AVL {
    Node* root;

    AVL() : root(nullptr) {}
    ~AVL() { destroy(root); }
    AVL(const AVL&) = delete;
    AVL& operator=(const AVL&) = delete;

    static void destroy(Node* t) {
        if (!t) return;
        destroy(t->left);
        destroy(t->right);
        delete t;
    }

    int height() const { return root ? root->height : 0; }
    int root_val() const { return root->val; }  // 调用前保证非空

    bool insert(int v) {
        bool inserted = false;
        root = insert_rec(root, v, inserted);
        return inserted;
    }

    bool erase(int v) {
        bool removed = false;
        root = erase_rec(root, v, removed);
        return removed;
    }

    bool find(int v) const {
        const Node* t = root;
        while (t) {
            if (v < t->val) t = t->left;
            else if (v > t->val) t = t->right;
            else return true;
        }
        return false;
    }

    int size() const { return size_rec(root); }

    static int size_rec(const Node* t) {
        if (!t) return 0;
        return 1 + size_rec(t->left) + size_rec(t->right);
    }

    std::vector<int> inorder() const {
        std::vector<int> out;
        inorder_rec(root, out);
        return out;
    }

    static void inorder_rec(const Node* t, std::vector<int>& out) {
        if (!t) return;
        inorder_rec(t->left, out);
        out.push_back(t->val);
        inorder_rec(t->right, out);
    }

    // 校验函数：同时验证 BST 严格递增、|bf| <= 1、缓存高度没写错
    bool check_balanced() const {
        bool has_prev = false;
        int prev = 0;
        return check_rec(root, has_prev, prev);
    }

private:
    static int h(const Node* t) { return t ? t->height : 0; }

    static int bf(const Node* t) { return t ? h(t->left) - h(t->right) : 0; }

    static void fix(Node* t) {
        int l = h(t->left);
        int r = h(t->right);
        t->height = (l > r ? l : r) + 1;
    }

    // LL 失衡：左孩子的左子树过高，对 t 右旋
    static Node* rotate_ll(Node* t) {
        Node* x = t->left;
        t->left = x->right;  // x 的右子树夹在 t 和 x 之间，仍然有序
        x->right = t;
        fix(t);  // 先算子节点，再算新根
        fix(x);
        return x;
    }

    // RR 失衡：右孩子的右子树过高，对 t 左旋
    static Node* rotate_rr(Node* t) {
        Node* x = t->right;
        t->right = x->left;
        x->left = t;
        fix(t);
        fix(x);
        return x;
    }

    // LR 失衡：先对左孩子左旋，把它变成 LL，再对自己右旋
    static Node* rotate_lr(Node* t) {
        t->left = rotate_rr(t->left);
        return rotate_ll(t);
    }

    // RL 失衡：先对右孩子右旋，把它变成 RR，再对自己左旋
    static Node* rotate_rl(Node* t) {
        t->right = rotate_ll(t->right);
        return rotate_rr(t);
    }

    static Node* rebalance(Node* t) {
        fix(t);
        int b = bf(t);
        if (b > 1) {
            // 左孩子 bf >= 0 是 LL；< 0 说明新节点插在左孩子的右边
            if (bf(t->left) >= 0) return rotate_ll(t);
            return rotate_lr(t);
        }
        if (b < -1) {
            if (bf(t->right) <= 0) return rotate_rr(t);
            return rotate_rl(t);
        }
        return t;
    }

    static Node* insert_rec(Node* t, int v, bool& inserted) {
        if (!t) {
            inserted = true;
            return new Node(v);
        }
        if (v < t->val) {
            t->left = insert_rec(t->left, v, inserted);
        } else if (v > t->val) {
            t->right = insert_rec(t->right, v, inserted);
        } else {
            return t;  // 重复值不插入，也不必修高度
        }
        return rebalance(t);
    }

    static Node* erase_rec(Node* t, int v, bool& removed) {
        if (!t) return nullptr;
        if (v < t->val) {
            t->left = erase_rec(t->left, v, removed);
        } else if (v > t->val) {
            t->right = erase_rec(t->right, v, removed);
        } else {
            removed = true;
            if (!t->left || !t->right) {
                // 叶子或单孩子：孩子顶上，高度由调用方 rebalance 修
                Node* c = t->left ? t->left : t->right;
                delete t;
                return c;
            }
            // 双孩子：中序后继（右子树最小值）的值搬上来
            Node* s = t->right;
            while (s->left) s = s->left;
            t->val = s->val;
            int successor = s->val;
            // 关键：真正删的是右子树里的后继，回溯必须从右子树内部
            // 一路 rebalance 上来，否则后继的祖先高度会停在旧值
            t->right = erase_rec(t->right, successor, removed);
        }
        return rebalance(t);
    }

    static bool check_rec(const Node* t, bool& has_prev, int& prev) {
        if (!t) return true;
        if (!check_rec(t->left, has_prev, prev)) return false;
        if (has_prev && !(prev < t->val)) return false;  // BST 严格递增
        has_prev = true;
        prev = t->val;
        if (!check_rec(t->right, has_prev, prev)) return false;
        int diff = h(t->left) - h(t->right);
        if (diff > 1 || diff < -1) return false;
        int l = h(t->left);
        int r = h(t->right);
        if (t->height != (l > r ? l : r) + 1) return false;  // 缓存高度
        return true;
    }
};

static bool same(const std::vector<int>& a, const std::vector<int>& b) {
    if (a.size() != b.size()) return false;
    for (size_t i = 0; i < a.size(); ++i) {
        if (a[i] != b[i]) return false;
    }
    return true;
}

int main() {
    // 1. 顺序插入 1..100，高度不能超过理论上界 9
    {
        AVL t;
        for (int v = 1; v <= 100; ++v) {
            assert(t.insert(v));
            assert(t.check_balanced());
        }
        assert(t.size() == 100);
        // 高度为 h 的 AVL 最少节点数 N(h)=N(h-1)+N(h-2)+1，
        // N(9)=88 <= 100 < N(10)=143，所以高度最多 9
        assert(t.height() <= 9);
        assert(t.height() >= 7);
        std::vector<int> expect;
        for (int v = 1; v <= 100; ++v) expect.push_back(v);
        assert(t.inorder() == expect);
        std::cout << "1..100 height: " << t.height() << "\n";
    }

    // 2. 四种旋转各一个最小用例，最终都应是 20 为根、高度 2
    {
        AVL ll;
        ll.insert(30);
        ll.insert(20);
        ll.insert(10);
        assert(ll.root_val() == 20 && ll.height() == 2);
        assert(ll.inorder() == std::vector<int>({10, 20, 30}));
        assert(ll.check_balanced());

        AVL rr;
        rr.insert(10);
        rr.insert(20);
        rr.insert(30);
        assert(rr.root_val() == 20 && rr.height() == 2);
        assert(rr.inorder() == std::vector<int>({10, 20, 30}));
        assert(rr.check_balanced());

        AVL lr;
        lr.insert(30);
        lr.insert(10);
        lr.insert(20);
        assert(lr.root_val() == 20 && lr.height() == 2);
        assert(lr.inorder() == std::vector<int>({10, 20, 30}));
        assert(lr.check_balanced());

        AVL rl;
        rl.insert(10);
        rl.insert(30);
        rl.insert(20);
        assert(rl.root_val() == 20 && rl.height() == 2);
        assert(rl.inorder() == std::vector<int>({10, 20, 30}));
        assert(rl.check_balanced());
    }

    // 3. 重复值不插入
    {
        AVL t;
        assert(t.insert(5));
        assert(!t.insert(5));
        assert(!t.insert(5));
        assert(t.size() == 1);
        assert(t.height() == 1);
        assert(t.inorder() == std::vector<int>({5}));
        assert(t.check_balanced());
        assert(!t.erase(4));  // 不存在，不算删除
        assert(t.size() == 1);
    }

    // 4. 删除：叶子 / 单孩子 / 根 / 空树
    {
        AVL t;
        for (int v = 1; v <= 20; ++v) t.insert(v);
        assert(t.erase(1));   // 叶子
        assert(!t.find(1));
        assert(t.check_balanced());
        while (t.size() > 1) {  // 逐个删到只剩根
            int v = t.inorder().front();
            assert(t.erase(v));
            assert(t.check_balanced());
        }
        assert(t.erase(t.root_val()));  // 删根
        assert(t.size() == 0 && t.height() == 0);
        assert(t.check_balanced());
        assert(!t.erase(42));  // 空树删除
        assert(t.size() == 0);
    }

    // 5. 双孩子删除：后继在右子树深处，回溯必须覆盖后继的祖先
    {
        AVL t;
        int vals[] = {50, 25, 75, 10, 30, 60, 90, 5, 15, 27, 35, 55, 65,
                      80, 95, 1, 7, 12, 18};
        for (int v : vals) t.insert(v);
        assert(t.check_balanced());
        // 50 有双孩子，后继是 55，删完从 55 的父节点一路修回根
        assert(t.erase(50));
        assert(!t.find(50));
        assert(t.find(55));
        assert(t.check_balanced());
        assert(t.size() == 18);
    }

    // 6. 随机插入 / 删除对拍：固定种子，与有序数组对照
    {
        AVL t;
        std::vector<int> ref;
        unsigned int seed = 20240601u;
        for (int step = 0; step < 1600; ++step) {
            seed = seed * 1103515245u + 12345u;
            int v = static_cast<int>((seed >> 16) % 201) - 100;
            seed = seed * 1103515245u + 12345u;
            bool do_erase = (seed >> 16) % 2 == 0;
            if (do_erase) {
                bool removed = t.erase(v);
                size_t at = ref.size();
                for (size_t i = 0; i < ref.size(); ++i) {
                    if (ref[i] == v) {
                        at = i;
                        break;
                    }
                }
                bool ref_has = at != ref.size();
                assert(removed == ref_has);
                if (ref_has) ref.erase(ref.begin() + at);
            } else {
                bool inserted = t.insert(v);
                size_t at = 0;
                while (at < ref.size() && ref[at] < v) ++at;
                bool ref_has = at < ref.size() && ref[at] == v;
                assert(inserted == !ref_has);
                if (!ref_has) ref.insert(ref.begin() + at, v);
            }
            // 每一步都必须满足：BST 有序 + |bf| <= 1 + 高度缓存正确
            assert(t.check_balanced());
            assert(t.size() == static_cast<int>(ref.size()));
            assert(same(t.inorder(), ref));
        }
        std::cout << "random ops size: " << t.size()
                  << " height: " << t.height() << "\n";
        // 再把剩下的按升序删空，验证连续的删除路径也能保持平衡
        while (!ref.empty()) {
            int v = ref.front();
            ref.erase(ref.begin());
            assert(t.erase(v));
            assert(t.check_balanced());
            assert(same(t.inorder(), ref));
        }
        assert(t.size() == 0 && t.height() == 0);
    }

    std::cout << "all tests passed\n";
    return 0;
}
