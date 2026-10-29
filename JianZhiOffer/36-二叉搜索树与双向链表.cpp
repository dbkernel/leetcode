/*
============================================================================
剑指 Offer 36. 二叉搜索树与双向链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/
分类：二叉搜索树 · 中序遍历 / 双向循环链表

将一个 二叉搜索树 就地转化为一个 已排序的双向循环链表 。

对于双向循环列表，你可以将左右孩子指针作为双向循环链表的前驱和后继指针，
第一个节点的前驱是最后一个节点，最后一个节点的后继是第一个节点。

特别地，我们希望可以 就地 完成转换操作。当转化完成以后，树中节点的左指针
需要指向前驱，树中节点的右指针需要指向后继。还需要返回链表中最小元素的
指针。

示例 1：
输入：root = [4,2,5,1,3]
输出：[1,2,3,4,5]
解释：下图显示了转化后的二叉搜索树，实线表示后继关系，虚线表示前驱关系。

示例 2：
输入：root = [2,1,3]
输出：[1,2,3]

示例 3：
输入：root = []
输出：[]
解释：输入是空树，所以输出也是空链表。

示例 4：
输入：root = [1]
输出：[1]

提示：
- -1000 <= Node.val <= 1000
- Node.left.val < Node.val < Node.right.val
- Node.val 的所有值都是独一无二的
- 0 <= Number of Nodes <= 2000

注意：本题与主站 426 题相同：
https://leetcode.cn/problems/convert-binary-search-tree-to-sorted-doubly-linked-list/
*/
#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <stack>
#include <vector>
using namespace std;

// 力扣预置的节点定义
class Node
{
public:
    int val;
    Node *left;
    Node *right;

    Node() : val(0), left(nullptr), right(nullptr) {}
    Node(int _val) : val(_val), left(nullptr), right(nullptr) {}
    Node(int _val, Node *_left, Node *_right)
        : val(_val), left(_left), right(_right) {}
};

/*
解法一：中序遍历 + 全局前驱指针【最优解】

为什么慢：BST 的中序序列就是升序，转换的难点不在"排序"，而在"改指针时不能
丢节点"。最直白的做法是先中序收集所有节点到数组，再按数组重连：正确但要多
一个 O(n) 的数组，而题目明确要求就地转换。

换个角度：中序遍历每次访问节点时，恰好知道它的前一个节点是谁。只要在访问
当前节点 node 时保存着 prev，就能当场把 `prev->right = node`、
`node->left = prev` 接好，指针改完再递归右子树，右子树根本没被碰过。

关键点：
  · 先递归左子树、再改指针、最后递归右子树 —— 顺序固定，提前改指针会丢掉
    还没访问的右子树或左子树
  · `prev` 为空时才赋值 head：中序第一个访问的节点就是最小值，即链表头
  · 递归结束后 prev 停在中序最后一个节点（最大值），此时才补首尾互指：
    `head->left = prev`、`prev->right = head`，少这一步得到的是单向链表
  · 空树必须提前返回 nullptr，否则 head 一直为空，补互指时会解引用空指针
  · 返回的是 head（最小元素），不是 root —— 根在中序中间，位置会变

以示例 1（root = [4,2,5,1,3]）为例：
  中序访问顺序 1、2、3、4、5；
  访问 1 时 prev 为空 → head = 1；访问 2 时 1->right = 2、2->left = 1；
  依此接到 5，结束时 prev = 5，补 5->right = 1、1->left = 5 ✔
  （若漏掉补互指，5 的 right 仍是原来的空指针，链表不成环。）

复杂度分析：
  时间 O(n)：每个节点访问一次，接线是常数次赋值
  空间 O(h)：递归栈深度等于树高，最坏（斜树）O(n)

解法二：显式栈中序【非最优】

为什么慢：思路与解法一完全相同，只是把系统递归栈换成手写栈。时间、空间
同为 O(n)/O(h)，与解法一同为最优；标为非最优只是避免重复计最优。它的实际
好处是递归深度不再受语言默认栈限制，2000 层的斜树也不会栈溢出。

换个角度：中序的"下一个访问节点"就是栈顶。一路向左压栈到底，弹栈即访问，
访问完把游标移到右孩子再重复压栈动作。

关键点：
  · 压栈用 while 走完整条左链，不能只压一层 —— 弹出时才保证是当前最小
  · 弹栈后立刻接 `prev->right` 和 `cur->left`，此时 cur 的右孩子还没被动过，
    所以 `cur = cur->right` 拿到的仍是原右子树
  · head 同样只在 prev 为空时赋值，即中序第一个节点
  · 循环条件是 `cur != nullptr || !stk.empty()`：游标空了还要继续弹栈，
    写成 `cur != nullptr` 会漏掉整条右链
  · 结束后与解法一一样补首尾互指

以示例 2（root = [2,1,3]）为例：
  压 2、1 → 弹 1（head = 1）→ cur 为 1 的右孩子（空）；
  弹 2，接 1->right = 2、2->left = 1 → cur 为 3；
  压 3、弹 3，接 2->right = 3、3->left = 2；
  补 3->right = 1、1->left = 3 ✔

复杂度分析：
  时间 O(n)：每个节点入栈、出栈、接线各一次
  空间 O(h)：栈最多保存一条根到叶的路径，最坏（斜树）O(n)
*/
class Solution1
{
public:
    Node *treeToDoublyList(Node *root)
    {
        if (root == nullptr) return nullptr;

        head = nullptr;
        prev = nullptr;
        inorder(root);

        // 首尾互指：prev 停在中序最后一个节点（最大值）
        head->left = prev;
        prev->right = head;
        return head;
    }

private:
    Node *head;
    Node *prev;

    void inorder(Node *node)
    {
        if (node == nullptr) return;

        inorder(node->left);

        // 中序第一个节点没有前驱，它的位置就是链表头
        if (prev != nullptr)
        {
            prev->right = node;
            node->left = prev;
        }
        else
        {
            head = node;
        }
        prev = node;

        // 右子树尚未被改动，递归进去时指针还是原样
        inorder(node->right);
    }
};

class Solution2
{
public:
    Node *treeToDoublyList(Node *root)
    {
        if (root == nullptr) return nullptr;

        Node *head = nullptr;
        Node *prev = nullptr;
        stack<Node *> stk;
        Node *cur = root;

        while (cur != nullptr || !stk.empty())
        {
            // 一路向左到底，栈顶就是当前最小的未访问节点
            while (cur != nullptr)
            {
                stk.push(cur);
                cur = cur->left;
            }

            cur = stk.top();
            stk.pop();

            if (prev != nullptr)
            {
                prev->right = cur;
                cur->left = prev;
            }
            else
            {
                head = cur;
            }
            prev = cur;

            // cur 的右孩子没被改过，这里拿到的是原右子树
            cur = cur->right;
        }

        head->left = prev;
        prev->right = head;
        return head;
    }
};

// ===== 测试辅助：按层序数组建树、校验双向循环链表并释放节点 =====
namespace
{
const long long NULL_NODE = LLONG_MIN; // 节点值为 int，不会等于这个哨兵
const int MAX_NODES = 2000;            // 题目上限，用来兜住成环的错误

Node *buildTree(const vector<long long> &levelOrder)
{
    if (levelOrder.empty() || levelOrder[0] == NULL_NODE) return nullptr;

    Node *root = new Node((int)levelOrder[0]);
    vector<Node *> nodes = {root};
    size_t idx = 1;
    for (size_t i = 0; i < nodes.size() && idx < levelOrder.size(); ++i)
    {
        if (levelOrder[idx] != NULL_NODE)
        {
            nodes[i]->left = new Node((int)levelOrder[idx]);
            nodes.push_back(nodes[i]->left);
        }
        ++idx;
        if (idx >= levelOrder.size()) break;
        if (levelOrder[idx] != NULL_NODE)
        {
            nodes[i]->right = new Node((int)levelOrder[idx]);
            nodes.push_back(nodes[i]->right);
        }
        ++idx;
    }
    return root;
}

// BST 的中序就是升序：期望值 = 输入里的非空值排序
vector<int> sortedValues(const vector<long long> &levelOrder)
{
    vector<int> values;
    for (long long v : levelOrder)
    {
        if (v != NULL_NODE) values.push_back((int)v);
    }
    sort(values.begin(), values.end());
    return values;
}

// 从 head 顺 right 走 n 步，校验升序、双向性、首尾互指，然后释放全部节点
void verifyAndFree(Node *head, const vector<int> &want)
{
    if (want.empty())
    {
        assert(head == nullptr);
        return;
    }

    int n = static_cast<int>(want.size());
    assert(n <= MAX_NODES);

    vector<Node *> nodes;
    Node *cur = head;
    for (int i = 0; i < n; ++i)
    {
        // 走到空说明链表长度不足，接错指针时会在这里暴露
        assert(cur != nullptr);
        nodes.push_back(cur);
        cur = cur->right;
    }
    // 走满 n 步必须回到 head，多出节点或断链都不满足循环链表定义
    assert(cur == head);

    for (int i = 0; i < n; ++i)
    {
        assert(nodes[i]->val == want[i]);
        // 头的前驱是尾、尾的后继是头，正好由下标取模表达
        assert(nodes[i]->left == nodes[(i + n - 1) % n]);
        assert(nodes[i]->right == nodes[(i + 1) % n]);
    }

    // 转换后是一整个环，不能再按左右孩子递归释放，逐个 delete
    for (Node *p : nodes) delete p;
}

void check(const vector<long long> &levelOrder)
{
    vector<int> want = sortedValues(levelOrder);

    Node *t1 = buildTree(levelOrder);
    verifyAndFree(Solution1().treeToDoublyList(t1), want);

    Node *t2 = buildTree(levelOrder);
    verifyAndFree(Solution2().treeToDoublyList(t2), want);
}
} // namespace

int main()
{
    // 示例 1：左右子树都有节点，最小值在最左端
    check({4, 2, 5, 1, 3});

    // 示例 2：只有三个节点
    check({2, 1, 3});

    // 示例 3：空树，必须返回 nullptr 而不是崩溃
    check({});

    // 示例 4：单节点，首尾都是它自己
    check({1});

    // 反直觉：最小值不在根的左孩子上，而在左孩子的左孩子上
    check({10, 5, 15, 2, 7, 12, 20, 1, 3, 6, 8, 11, 13});

    // 边界：只有左孩子的斜链，头是最深的那个、尾是根
    check({3, 2, NULL_NODE, 1});

    // 边界：只有右孩子的斜链，头是根、尾是最深的那个
    check({1, NULL_NODE, 2, NULL_NODE, 3});

    // 边界：2000 层左斜链，压一压递归与栈的深度
    {
        vector<long long> chain = {2000};
        for (int i = 1999; i >= 1; --i)
        {
            chain.push_back(i);
            chain.push_back(NULL_NODE);
        }
        check(chain);
    }

    // 边界：2000 层右斜链
    {
        vector<long long> chain = {1};
        for (int i = 2; i <= 2000; ++i)
        {
            chain.push_back(NULL_NODE);
            chain.push_back(i);
        }
        check(chain);
    }

    cout << "all tests passed" << endl;
    return 0;
}
