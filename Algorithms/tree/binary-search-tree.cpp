/*
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
*/
#include <cassert>
#include <climits>
#include <iostream>
#include <vector>

struct Node {
    int val;
    int cnt;  // 该值出现次数，重复值不新建节点
    Node* left;
    Node* right;
    explicit Node(int v) : val(v), cnt(1), left(nullptr), right(nullptr) {}
};

struct BST {
    Node* root;

    BST() : root(nullptr) {}
    ~BST() { destroy(root); }
    BST(const BST&) = delete;
    BST& operator=(const BST&) = delete;

    static void destroy(Node* t) {
        if (!t) return;
        destroy(t->left);
        destroy(t->right);
        delete t;
    }

    // 返回 true 表示新建了节点；false 表示只是 cnt 加一
    bool insert(int v) {
        bool created = false;
        root = insert_rec(root, v, created);
        return created;
    }

    static Node* insert_rec(Node* t, int v, bool& created) {
        if (!t) {
            created = true;
            return new Node(v);
        }
        if (v < t->val) {
            t->left = insert_rec(t->left, v, created);
        } else if (v > t->val) {
            t->right = insert_rec(t->right, v, created);
        } else {
            ++t->cnt;  // 重复值只计数，保持「一值一节点」
        }
        return t;
    }

    bool find(int v) const { return find_in(root, v); }

    static bool find_in(const Node* t, int v) {
        while (t) {
            if (v < t->val) t = t->left;
            else if (v > t->val) t = t->right;
            else return true;
        }
        return false;
    }

    // 返回该值出现次数，不存在返回 0
    int count(int v) const {
        const Node* t = root;
        while (t) {
            if (v < t->val) t = t->left;
            else if (v > t->val) t = t->right;
            else return t->cnt;
        }
        return 0;
    }

    bool min_val(int& out) const {
        if (!root) return false;
        const Node* t = root;
        while (t->left) t = t->left;
        out = t->val;
        return true;
    }

    bool max_val(int& out) const {
        if (!root) return false;
        const Node* t = root;
        while (t->right) t = t->right;
        out = t->val;
        return true;
    }

    // 摘掉子树里最小的节点：它一定没有左孩子，直接拿右孩子顶替
    static Node* pop_min(Node* t, Node** out) {
        if (!t->left) {
            *out = t;
            return t->right;
        }
        t->left = pop_min(t->left, out);
        return t;
    }

    // 返回 true 表示确实删掉了一次出现
    bool erase(int v) {
        bool removed = false;
        root = erase_rec(root, v, removed);
        return removed;
    }

    static Node* erase_rec(Node* t, int v, bool& removed) {
        if (!t) return nullptr;  // 空树删除：原样返回，不算删除成功
        if (v < t->val) {
            t->left = erase_rec(t->left, v, removed);
            return t;
        }
        if (v > t->val) {
            t->right = erase_rec(t->right, v, removed);
            return t;
        }
        removed = true;
        if (t->cnt > 1) {
            --t->cnt;  // 还有同值的其它出现，只减少一次计数
            return t;
        }
        if (!t->left) {
            Node* r = t->right;
            delete t;
            return r;
        }
        if (!t->right) {
            Node* l = t->left;
            delete t;
            return l;
        }
        // 双孩子：中序后继没有左孩子，摘除它只需把它的右孩子接上
        Node* s = nullptr;
        t->right = pop_min(t->right, &s);
        t->val = s->val;
        t->cnt = s->cnt;  // 后继可能自己也带着重复计数，整份搬过来
        delete s;
        return t;
    }

    int height() const { return height_rec(root); }

    static int height_rec(const Node* t) {
        if (!t) return 0;
        int l = height_rec(t->left);
        int r = height_rec(t->right);
        return (l > r ? l : r) + 1;
    }

    int size() const { return size_rec(root); }

    static int size_rec(const Node* t) {
        if (!t) return 0;
        return t->cnt + size_rec(t->left) + size_rec(t->right);
    }

    int node_count() const { return node_count_rec(root); }

    static int node_count_rec(const Node* t) {
        if (!t) return 0;
        return 1 + node_count_rec(t->left) + node_count_rec(t->right);
    }

    // 中序：每个值按 cnt 重复输出，所以整体非递减
    std::vector<int> inorder() const {
        std::vector<int> out;
        inorder_rec(root, out);
        return out;
    }

    static void inorder_rec(const Node* t, std::vector<int>& out) {
        if (!t) return;
        inorder_rec(t->left, out);
        for (int i = 0; i < t->cnt; ++i) out.push_back(t->val);
        inorder_rec(t->right, out);
    }

    // 校验 BST 不变量：严格区间夹逼 + cnt >= 1
    bool check() const { return check_rec(root, LLONG_MIN, LLONG_MAX); }

    static bool check_rec(const Node* t, long long lo, long long hi) {
        if (!t) return true;
        if (t->cnt < 1) return false;
        if (!(lo < t->val && t->val < hi)) return false;
        return check_rec(t->left, lo, t->val) &&
               check_rec(t->right, t->val, hi);
    }
};

static void print_vec(const std::vector<int>& v) {
    std::cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i) std::cout << " ";
        std::cout << v[i];
    }
    std::cout << "]";
}

static bool same(const std::vector<int>& a, const std::vector<int>& b) {
    if (a.size() != b.size()) return false;
    for (size_t i = 0; i < a.size(); ++i) {
        if (a[i] != b[i]) return false;
    }
    return true;
}

int main() {
    // 1. 正常用例：插入、查找、极值、高度、中序
    {
        BST t;
        int vals[] = {50, 30, 70, 20, 40, 60, 80};
        for (int v : vals) assert(t.insert(v));
        assert(t.size() == 7);
        assert(t.node_count() == 7);
        assert(t.height() == 3);
        assert(t.find(40));
        assert(!t.find(41));
        int mn = 0, mx = 0;
        assert(t.min_val(mn) && mn == 20);
        assert(t.max_val(mx) && mx == 80);
        assert(t.check());
        std::vector<int> expect = {20, 30, 40, 50, 60, 70, 80};
        assert(t.inorder() == expect);
        std::cout << "inorder: ";
        print_vec(t.inorder());
        std::cout << "\n";
    }

    // 2. 重复值：只加计数，不新建节点
    {
        BST t;
        t.insert(40);
        assert(!t.insert(40));
        assert(!t.insert(40));
        assert(t.count(40) == 3);
        assert(t.node_count() == 1);
        assert(t.size() == 3);
        assert(t.erase(40));
        assert(t.count(40) == 2);
        assert(t.find(40));
        assert(t.erase(40));
        assert(t.erase(40));
        assert(!t.find(40));
        assert(!t.erase(40));  // 已经没了，删除失败
        assert(t.node_count() == 0);
        assert(t.height() == 0);
    }

    // 3. 删除叶子 / 单孩子 / 根 / 双孩子
    {
        BST t;
        int vals[] = {50, 30, 70, 20, 40, 60, 80};
        for (int v : vals) t.insert(v);
        // 叶子
        assert(t.erase(20));
        assert(!t.find(20));
        assert(t.check());
        // 单孩子：70 只剩右孩子 80
        assert(t.erase(60));
        assert(t.erase(70));
        assert(t.find(80));
        assert(t.check());
        // 双孩子：删根 50，中序后继 80 顶上
        assert(t.erase(50));
        assert(t.find(80));
        assert(t.check());
        std::vector<int> expect = {30, 40, 80};
        assert(t.inorder() == expect);
        // 80 上位后左边还挂着 30 -> 40，所以高度是 3
        assert(t.height() == 3);
    }

    // 4. 单孩子删根：根被唯一孩子顶替
    {
        BST t;
        t.insert(10);
        t.insert(5);
        t.insert(3);
        assert(t.erase(10));
        // 10 只有左孩子 5，5 上位，3 仍是 5 的左孩子
        assert(t.inorder() == std::vector<int>({3, 5}));
        assert(t.check());
        assert(t.height() == 2);
        int mn = 0, mx = 0;
        assert(t.min_val(mn) && mn == 3);
        assert(t.max_val(mx) && mx == 5);
    }

    // 5. 空树删除：返回 false，树保持为空
    {
        BST t;
        assert(!t.erase(1));
        assert(t.inorder().empty());
        assert(t.height() == 0);
        int mn = 0, mx = 0;
        assert(!t.min_val(mn));
        assert(!t.max_val(mx));
        assert(t.size() == 0);
        assert(t.check());
    }

    // 6. 双孩子 + 后继自身带重复计数：计数必须整份搬上来
    {
        BST t;
        t.insert(10);
        t.insert(5);
        t.insert(20);
        t.insert(20);
        assert(t.count(20) == 2);
        assert(t.erase(10));  // 后继是 20(cnt=2)
        assert(t.inorder() == std::vector<int>({5, 20, 20}));
        assert(t.count(20) == 2);
        assert(t.node_count() == 2);
        assert(t.check());
    }

    // 7. 中序严格递增（无重复时）
    {
        BST t;
        for (int i = 0; i < 64; ++i) t.insert(i * 3 - 50);
        std::vector<int> seq = t.inorder();
        for (size_t i = 1; i < seq.size(); ++i) {
            assert(seq[i - 1] < seq[i]);
        }
        assert(t.check());
    }

    // 8. 操作序列对拍：固定种子随机插删，与有序数组对照
    {
        BST t;
        std::vector<int> ref;  // 有序参照，重复值重复存放
        unsigned int seed = 12345u;
        for (int step = 0; step < 400; ++step) {
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
                t.insert(v);
                size_t at = 0;
                while (at < ref.size() && ref[at] < v) ++at;
                ref.insert(ref.begin() + at, v);  // 手写插入，保持有序
            }
            assert(t.check());
            assert(t.size() == static_cast<int>(ref.size()));
            assert(same(t.inorder(), ref));
        }
        std::cout << "random ops size: " << t.size() << "\n";
    }

    std::cout << "all tests passed\n";
    return 0;
}
