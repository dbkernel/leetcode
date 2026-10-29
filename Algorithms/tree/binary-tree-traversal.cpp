/*
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
  · Morris：prev->right == cur 表示第二次到达 cur，必须把
    prev->right 还原成空，否则树里留下环；
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
*/
#include <cassert>
#include <iostream>
#include <vector>

struct Node {
    int val;
    Node* left;
    Node* right;
    explicit Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

// 自己写的循环队列：用 count 区分空与满，容量不够就翻倍。
// 只把 std::vector 当裸数组用，队列是自己写的。
struct NodeQueue {
    std::vector<Node*> buf;
    int head;
    int tail;
    int count;

    explicit NodeQueue(int cap) : buf(cap > 0 ? cap : 1),
                                  head(0), tail(0), count(0) {}

    bool empty() const { return count == 0; }

    void push(Node* x) {
        if (count == static_cast<int>(buf.size())) grow();
        buf[tail] = x;
        tail = (tail + 1) % static_cast<int>(buf.size());
        ++count;
    }

    Node* pop() {
        Node* x = buf[head];
        head = (head + 1) % static_cast<int>(buf.size());
        --count;
        return x;
    }

    // 扩容时按 head 起的逻辑顺序搬移，搬完让 head 回到 0
    void grow() {
        int cap = static_cast<int>(buf.size());
        std::vector<Node*> nb(cap * 2);
        for (int i = 0; i < count; ++i) nb[i] = buf[(head + i) % cap];
        buf.swap(nb);
        head = 0;
        tail = count;
    }
};

struct BinTree {
    Node* root;

    BinTree() : root(nullptr) {}
    ~BinTree() { destroy(root); }
    // 禁止拷贝，否则两个对象析构时会把同一批节点 delete 两次
    BinTree(const BinTree&) = delete;
    BinTree& operator=(const BinTree&) = delete;

    static void destroy(Node* t) {
        if (!t) return;
        destroy(t->left);
        destroy(t->right);
        delete t;
    }

    // 用层序数组建树，-1 表示空位。仅用于构造，遍历时不认 -1。
    void build(const std::vector<int>& a) {
        destroy(root);
        root = nullptr;
        if (a.empty() || a[0] == -1) return;
        root = new Node(a[0]);
        NodeQueue q(8);
        q.push(root);
        int i = 1;
        int n = static_cast<int>(a.size());
        while (!q.empty() && i < n) {
            Node* cur = q.pop();
            if (a[i] != -1) {
                cur->left = new Node(a[i]);
                q.push(cur->left);
            }
            ++i;
            if (i < n) {
                if (a[i] != -1) {
                    cur->right = new Node(a[i]);
                    q.push(cur->right);
                }
                ++i;
            }
        }
    }

    // 只有左孩子的链，值从根往下是 n, n-1, ..., 1
    void build_left_chain(int n) {
        destroy(root);
        root = nullptr;
        for (int v = 1; v <= n; ++v) {
            Node* x = new Node(v);
            x->left = root;
            root = x;
        }
    }

    // 只有右孩子的链，值从根往下同样是 n, n-1, ..., 1
    void build_right_chain(int n) {
        destroy(root);
        root = nullptr;
        for (int v = 1; v <= n; ++v) {
            Node* x = new Node(v);
            x->right = root;
            root = x;
        }
    }

    // 层序序列化，空位写 -1 并去掉尾部空位，用来确认遍历没改坏树
    std::vector<int> to_level_array() const {
        std::vector<int> out;
        NodeQueue q(8);
        q.push(root);
        while (!q.empty()) {
            Node* t = q.pop();
            if (!t) {
                out.push_back(-1);
                continue;
            }
            out.push_back(t->val);
            q.push(t->left);
            q.push(t->right);
        }
        while (!out.empty() && out.back() == -1) out.pop_back();
        return out;
    }

    // ---------- 递归版 ----------
    static void pre_rec(Node* t, std::vector<int>& out) {
        if (!t) return;
        out.push_back(t->val);
        pre_rec(t->left, out);
        pre_rec(t->right, out);
    }

    static void in_rec(Node* t, std::vector<int>& out) {
        if (!t) return;
        in_rec(t->left, out);
        out.push_back(t->val);
        in_rec(t->right, out);
    }

    static void post_rec(Node* t, std::vector<int>& out) {
        if (!t) return;
        post_rec(t->left, out);
        post_rec(t->right, out);
        out.push_back(t->val);
    }

    std::vector<int> preorder() const {
        std::vector<int> out;
        pre_rec(root, out);
        return out;
    }

    std::vector<int> inorder() const {
        std::vector<int> out;
        in_rec(root, out);
        return out;
    }

    std::vector<int> postorder() const {
        std::vector<int> out;
        post_rec(root, out);
        return out;
    }

    // ---------- 显式栈迭代版 ----------
    std::vector<int> preorder_iter() const {
        std::vector<int> out;
        if (!root) return out;
        std::vector<Node*> st;  // 用 vector 当栈
        st.push_back(root);
        while (!st.empty()) {
            Node* t = st.back();
            st.pop_back();
            out.push_back(t->val);
            // 先压右再压左：后进先出，左孩子先被处理
            if (t->right) st.push_back(t->right);
            if (t->left) st.push_back(t->left);
        }
        return out;
    }

    std::vector<int> inorder_iter() const {
        std::vector<int> out;
        std::vector<Node*> st;
        Node* cur = root;
        while (cur || !st.empty()) {
            while (cur) {
                st.push_back(cur);
                cur = cur->left;
            }
            cur = st.back();
            st.pop_back();
            out.push_back(cur->val);
            cur = cur->right;
        }
        return out;
    }

    std::vector<int> postorder_iter() const {
        std::vector<int> out;
        std::vector<Node*> st;
        Node* cur = root;
        Node* last = nullptr;  // 上一个真正输出的节点
        while (cur || !st.empty()) {
            while (cur) {
                st.push_back(cur);
                cur = cur->left;
            }
            Node* top = st.back();
            // 右子树存在且还没走完，先把 cur 指过去，top 留在栈上
            if (top->right && top->right != last) {
                cur = top->right;
            } else {
                out.push_back(top->val);
                last = top;
                st.pop_back();
            }
        }
        return out;
    }

    // ---------- 层序：自己写的循环队列 ----------
    std::vector<int> level_order() const {
        std::vector<int> out;
        if (!root) return out;
        NodeQueue q(8);
        q.push(root);
        while (!q.empty()) {
            Node* t = q.pop();
            out.push_back(t->val);
            if (t->left) q.push(t->left);
            if (t->right) q.push(t->right);
        }
        return out;
    }

    // ---------- Morris 中序：O(1) 额外空间 ----------
    // 借用空右指针穿线到中序后继。有左孩子时：
    //   prev->right 为空 -> 第一次到达，穿线，往左走；
    //   prev->right == cur -> 第二次到达，拆线，输出 cur，往右走。
    std::vector<int> morris_inorder() const {
        std::vector<int> out;
        Node* cur = root;
        while (cur) {
            if (!cur->left) {
                out.push_back(cur->val);
                cur = cur->right;
            } else {
                Node* prev = cur->left;
                while (prev->right && prev->right != cur) {
                    prev = prev->right;
                }
                if (!prev->right) {
                    prev->right = cur;  // 穿线
                    cur = cur->left;
                } else {
                    prev->right = nullptr;  // 拆线，树恢复原状
                    out.push_back(cur->val);
                    cur = cur->right;
                }
            }
        }
        return out;
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

int main() {
    // 1. 固定形状的树，验证四种遍历序列
    {
        BinTree t;
        t.build({1, 2, 3, 4, 5, -1, 6, -1, -1, 7});
        std::vector<int> pre = {1, 2, 4, 5, 7, 3, 6};
        std::vector<int> in = {4, 2, 7, 5, 1, 3, 6};
        std::vector<int> post = {4, 7, 5, 2, 6, 3, 1};
        std::vector<int> lvl = {1, 2, 3, 4, 5, 6, 7};
        assert(t.preorder() == pre);
        assert(t.inorder() == in);
        assert(t.postorder() == post);
        assert(t.preorder_iter() == pre);
        assert(t.inorder_iter() == in);
        assert(t.postorder_iter() == post);
        assert(t.level_order() == lvl);
        // Morris 必须与递归中序一致
        assert(t.morris_inorder() == in);
        // 遍历（含 Morris 穿线拆线）之后树结构不能变
        std::vector<int> shape = {1, 2, 3, 4, 5, -1, 6, -1, -1, 7};
        assert(t.to_level_array() == shape);
        assert(t.preorder() == pre);
        std::cout << "fixed tree preorder: ";
        print_vec(t.preorder());
        std::cout << "\n";
    }

    // 2. 单节点
    {
        BinTree t;
        t.build({5});
        std::vector<int> one = {5};
        assert(t.preorder() == one);
        assert(t.inorder() == one);
        assert(t.postorder() == one);
        assert(t.preorder_iter() == one);
        assert(t.inorder_iter() == one);
        assert(t.postorder_iter() == one);
        assert(t.level_order() == one);
        assert(t.morris_inorder() == one);
        assert(t.to_level_array() == one);
    }

    // 3. 空树
    {
        BinTree t;
        assert(t.preorder().empty());
        assert(t.postorder().empty());
        assert(t.preorder_iter().empty());
        assert(t.inorder_iter().empty());
        assert(t.postorder_iter().empty());
        assert(t.level_order().empty());
        assert(t.morris_inorder().empty());
        assert(t.to_level_array().empty());
        // 空树里 -1 只表示空位，不产生节点
        t.build({-1, 1, 2});
        assert(t.to_level_array().empty());
    }

    // 4. 只有左 / 右孩子的长链：验证显式栈不塌，迭代结果正确
    {
        const int n = 500;
        std::vector<int> asc, desc;
        for (int v = 1; v <= n; ++v) {
            asc.push_back(v);
            desc.push_back(n - v + 1);
        }
        BinTree lt;
        lt.build_left_chain(n);
        assert(lt.preorder_iter() == desc);   // 根在下、值最大
        assert(lt.inorder_iter() == asc);
        assert(lt.postorder_iter() == asc);
        assert(lt.level_order() == desc);
        assert(lt.morris_inorder() == asc);

        BinTree rt;
        rt.build_right_chain(n);
        assert(rt.preorder_iter() == desc);
        assert(rt.inorder_iter() == desc);
        assert(rt.postorder_iter() == asc);
        assert(rt.level_order() == desc);
        assert(rt.morris_inorder() == desc);
    }

    std::cout << "all tests passed\n";
    return 0;
}
