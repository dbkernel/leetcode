/*
============================================================================
589. N 叉树的前序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-preorder-traversal/
分类：N 叉树 · 遍历（递归 / 栈）

给定一个 n 叉树的根节点 root ，返回 其节点值的 前序遍历 。

n 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔
（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[1,3,5,6,2,4]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[1,2,3,6,7,11,14,4,8,12,5,9,13,10]

提示：
- 节点总数在范围 [0, 10^4]内
- 0 <= Node.val <= 10^4
- n 叉树的高度小于或等于 1000

进阶：递归法很简单，你可以使用迭代法完成此题吗?
*/
#include <cassert>
#include <iostream>
#include <stack>
#include <unordered_map>
#include <vector>
using namespace std;

// 力扣预置的节点定义（多叉树，注意孩子是一个数组）
class Node
{
public:
    int val;
    vector<Node *> children;

    Node() {}

    Node(int _val) { val = _val; }

    Node(int _val, vector<Node *> _children)
    {
        val = _val;
        children = _children;
    }
};

/*
解法一：递归【最优解】

官方方法一。N 叉树的前序遍历与二叉树完全一致，只是「左右两个孩子」换成了
「children 数组」：
  · 先访问当前节点；
  · 再按从左到右的顺序依次递归每个孩子。

为什么它是最优解：
  · 时间是 O(m)，每个节点只访问一次，不可能更快；
  · 辅助空间只有递归栈 O(h)，并且不需要额外哈希表或「待处理兄弟」的缓存。
  · 因此它在空间上优于下面两种迭代写法。

关键点：
  · 访问语句必须在 for 循环之前，否则就变成后序了
  · children 为空时 for 循环自然跳过，不需要单独判空
  · 递归深度 = 树高 h，本题 h ≤ 1000，不会爆栈

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  访问 1 → 依次进孩子 3、2、4：
  访问 3 → 孩子 5、6：访问 5，访问 6；回到 2：访问 2；回到 4：访问 4
  得到 [1,3,5,6,2,4]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：迭代（显式栈 + 哈希表记录孩子下标）【非最优】

官方方法二。直接模拟递归：递归时「返回到父节点后该处理第几个孩子」这件事
由调用栈自动记住，手写栈就必须自己记，于是用哈希表存每个节点已处理到的下标。
  · 一路沿着第一个孩子向下，边下边访问、边把节点压栈，并在表中记 0；
  · 走到空后看栈顶 p：取它下一个未访问孩子的下标 index；
      - index 有效 → 更新表中下标，转到该孩子继续向下；
      - index 无效 → 说明 p 的孩子都处理完了，弹出 p、删掉表项，回到 p 的父亲。

关键点：
  · 「先访问再往下」决定了这是前序；如果改成出栈时才访问就变成后序
  · 表项要在节点弹出时删除，否则同一节点再次入栈会读到旧下标
  · `cnt.count(node) ? cnt[node] : -1` 里的 -1 是为叶子准备的：
    叶子没有表项，加一后下标为 0，恰好判定为「已无孩子」
  · 额外的哈希表让常数和空间都变差，所以不是最优

复杂度分析：
  时间 O(m)
  空间 O(h)：栈与哈希表都只保存当前路径上的节点

解法三：迭代（逆序压栈，最简写法）【非最优】

官方方法三。前序要求「根 → 孩子从左到右」，而栈是后进先出：
  · 先弹出并访问当前节点；
  · 再把孩子「从右到左」压栈，这样左孩子最后入栈、最先出栈。

关键点：
  · 压栈必须逆序，正序压栈会让最右孩子先被访问，顺序就反了
  · 不需要哈希表，也不需要「当前节点」指针，循环体只有三步
  · 空间最坏是 O(m)：星形树（根有 m-1 个孩子）时栈里同时存下所有兄弟节点
  · 与解法二同为 O(m) 时间，但写法短得多，是迭代法里的首选

复杂度分析：
  时间 O(m)
  空间 O(m)：最坏为星形树，栈同时保存 O(m) 个兄弟节点

-------- 三种写法对比 --------
  递归：时间 O(m)、空间 O(h)，最省空间，标记为【最优解】
  解法二：时间 O(m)、空间 O(h) + 哈希表，常数最大，仅作为「模拟递归」的示范
  解法三：时间 O(m)、空间 O(m)，代码最短，迭代时优先选它
  本题 N 叉树没有 Morris 那样的 O(1) 空间遍历写法。
*/
class Solution
{
public:
    // ---------- 解法一：递归（最优解） ----------
    void preorderHelper(Node *node, vector<int> &res)
    {
        if (node == nullptr) return;
        res.push_back(node->val);
        for (Node *child : node->children) preorderHelper(child, res);
    }

    vector<int> preorder(Node *root)
    {
        vector<int> res;
        preorderHelper(root, res);
        return res;
    }

    // ---------- 解法二：迭代 + 哈希表记录孩子下标 ----------
    vector<int> preorderIterativeWithIndex(Node *root)
    {
        vector<int> res;
        if (root == nullptr) return res;

        unordered_map<Node *, int> nextIndex;
        stack<Node *> stk;
        Node *node = root;
        while (!stk.empty() || node != nullptr)
        {
            // 沿着第一个孩子一路向下，同时访问沿途节点
            while (node != nullptr)
            {
                res.push_back(node->val);
                stk.push(node);
                if (!node->children.empty())
                {
                    nextIndex[node] = 0;
                    node = node->children[0];
                }
                else
                {
                    node = nullptr;
                }
            }
            node = stk.top();
            // 没有表项说明是叶子，-1 + 1 = 0，正好越界
            int index = (nextIndex.count(node) ? nextIndex[node] : -1) + 1;
            if (index < (int)node->children.size())
            {
                nextIndex[node] = index;
                node = node->children[index];
            }
            else
            {
                stk.pop();
                nextIndex.erase(node);
                node = nullptr;
            }
        }
        return res;
    }

    // ---------- 解法三：迭代 + 逆序压栈 ----------
    vector<int> preorderIterativeReverse(Node *root)
    {
        vector<int> res;
        if (root == nullptr) return res;

        stack<Node *> stk;
        stk.push(root);
        while (!stk.empty())
        {
            Node *node = stk.top();
            stk.pop();
            res.push_back(node->val);
            // 逆序压栈：左孩子最后入栈，从而最先出栈
            for (int i = (int)node->children.size() - 1; i >= 0; --i)
            {
                stk.push(node->children[i]);
            }
        }
        return res;
    }
};

// ===== 测试辅助：按层序数组建 N 叉树（NULL_NODE 表示空） =====
namespace
{
const int NULL_NODE = 10001; // 节点值范围 [0, 10^4]

// 层序序列的规则：根之后先来一个 null，然后是该节点的全部孩子；
// 每个孩子按 BFS 顺序继续展开，孩子列表以 null 结尾。
Node *buildNaryTree(const vector<int> &levelOrder)
{
    if (levelOrder.empty() || levelOrder[0] == NULL_NODE) return nullptr;

    Node *root = new Node(levelOrder[0]);
    vector<Node *> nodes = {root};
    size_t idx = 2; // 跳过根与它后面的 null
    size_t p = 0;
    while (idx < levelOrder.size() && p < nodes.size())
    {
        Node *parent = nodes[p++];
        while (idx < levelOrder.size() && levelOrder[idx] != NULL_NODE)
        {
            Node *child = new Node(levelOrder[idx]);
            parent->children.push_back(child);
            nodes.push_back(child);
            ++idx;
        }
        ++idx; // 跳过孩子列表的 null 结束符
    }
    return root;
}

void freeNaryTree(Node *root)
{
    if (root == nullptr) return;
    for (Node *child : root->children) freeNaryTree(child);
    delete root;
}

void check(const vector<int> &levelOrder, const vector<int> &want)
{
    Solution solution;

    Node *t1 = buildNaryTree(levelOrder);
    assert(solution.preorder(t1) == want);
    freeNaryTree(t1);

    Node *t2 = buildNaryTree(levelOrder);
    assert(solution.preorderIterativeWithIndex(t2) == want);
    freeNaryTree(t2);

    Node *t3 = buildNaryTree(levelOrder);
    assert(solution.preorderIterativeReverse(t3) == want);
    freeNaryTree(t3);
}
} // namespace

int main()
{
    // 示例 1
    check({1, NULL_NODE, 3, 2, 4, NULL_NODE, 5, 6}, {1, 3, 5, 6, 2, 4});

    // 示例 2
    check({1, NULL_NODE, 2, 3, 4, 5, NULL_NODE, NULL_NODE, 6, 7, NULL_NODE,
           8, NULL_NODE, 9, 10, NULL_NODE, NULL_NODE, 11, NULL_NODE, 12,
           NULL_NODE, 13, NULL_NODE, NULL_NODE, 14},
          {1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10});

    // 边界：空树
    check({}, {});

    // 边界：单节点
    check({1}, {1});

    // 边界：根节点没有孩子
    check({1, NULL_NODE}, {1});

    // 边界：叶子节点顺序 —— 根的 3 个孩子都没有孩子
    check({1, NULL_NODE, 2, 3, 4}, {1, 2, 3, 4});

    // 边界：星形树，验证逆序压栈有没有把兄弟顺序写反
    check({1, NULL_NODE, 2, 3, 4, 5, 6}, {1, 2, 3, 4, 5, 6});

    // 边界：孩子为空要用 null 占位（2 没有孩子，3 有孩子 4）
    check({1, NULL_NODE, 2, 3, NULL_NODE, NULL_NODE, 4}, {1, 2, 3, 4});

    // 边界：深链（每个节点只有一个孩子），深度 200
    {
        vector<int> levelOrder;
        vector<int> want;
        for (int i = 1; i <= 200; ++i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
            want.push_back(i);
        }
        check(levelOrder, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
