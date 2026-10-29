/*
============================================================================
590. N 叉树的后序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-postorder-traversal/
分类：N 叉树 · 遍历（递归 / 栈 / 前序反转）

给定一个 n 叉树的根节点 root ，返回 其节点值的 后序遍历 。

n 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔
（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[5,6,3,2,4,1]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[2,6,14,11,7,3,12,8,4,13,9,10,5,1]

提示：
- 节点总数在范围 [0, 10^4] 内
- 0 <= Node.val <= 10^4
- n 叉树的高度小于或等于 1000

进阶：递归法很简单，你可以使用迭代法完成此题吗?
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <stack>
#include <unordered_map>
#include <unordered_set>
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

官方方法一。后序遍历的定义是「所有孩子从左到右 → 根」：
  · 依次递归每个孩子；
  · 所有孩子处理完之后，再把当前节点值加入答案。

为什么它是最优解：
  · 时间 O(m)，每个节点只访问一次；
  · 辅助空间只有递归栈 O(h)，不需要哈希表、visited 集合或成批的兄弟节点缓存。
  · 对比之下，方法三要用 O(m) 的 visited 集合，方法四的栈最坏也是 O(m)。

关键点：
  · 访问语句必须在 for 循环之后，这是与前序唯一的区别
  · children 为空时 for 循环自然跳过，叶子直接输出自己
  · 递归深度 = 树高 h，本题 h ≤ 1000

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  先处理 1 的孩子：3 的孩子 5、6 → 输出 5、6，再输出 3；然后 2；然后 4；
  最后输出 1 → [5,6,3,2,4,1]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：迭代（显式栈 + 哈希表记录孩子下标）【非最优】

官方方法二。与 589 的迭代完全同构，只是「输出时机」从入栈时挪到了孩子全都
处理完之后：
  · 一路沿着第一个孩子向下，只压栈不输出，并在表中记 0；
  · 走到空后看栈顶 p，取下一个未访问孩子的下标 index；
      - index 有效 → 更新下标，转去处理该孩子；
      - index 无效 → 孩子全处理完了，输出 p、弹出、删表项。

关键点：
  · 输出必须发生在「index 无效」分支里；提前输出就变成前序了
  · 最后这个分支弹出的节点，其所有孩子一定已经输出过，这是后序的正确性来源
  · 表项在弹出时删除，保证再次入栈时从 0 重新开始
  · 哈希表让常数和空间都变差，所以不是最优

复杂度分析：
  时间 O(m)
  空间 O(h)：栈与哈希表都只保存当前路径上的节点

解法三：迭代 + visited 集合（每个节点进出栈两次）【非最优】

官方方法三。换个思路：把后序看成「先压孩子、等孩子处理完再处理自己」。
  · 取栈顶 p：
      - p 是叶子，或 p 已经展开过孩子（visited 里有 p）→ 输出 p 并弹出；
      - 否则把 p 的所有孩子「从右到左」压栈，并把 p 标记为已展开。
  · 因为孩子压在 p 上面，所以孩子一定先于 p 被处理；等 p 再次成为栈顶时，
    visited 告诉我们可以输出它了。

关键点：
  · visited 标记必须在「压完孩子之后」，否则 p 会被当成已完成直接输出
  · 孩子仍要逆序压栈：从右到左入栈，出栈才是从左到右
  · 每个节点会两次成为栈顶（第一次展开孩子、第二次输出），但只输出一次
  · visited 集合最多存 O(m) 个节点，空间不如递归

复杂度分析：
  时间 O(m)
  空间 O(m)：visited 集合 + 栈

解法四：利用前序遍历反转【非最优】

官方方法四。这是本题最巧的做法，观察两个序列的关系：
  · 后序 = [孩子1…, 孩子2…, …, 根]；
  · 把前序的孩子顺序改成「从右到左」，得到 [根, 孩子k…, …, 孩子1…]；
  · 两者恰好互为逆序。
于是用前序的逆序压栈写法得到「根-右-左」，最后整体反转即可。
  · 每次弹出节点就输出；
  · 把孩子「按从左到右」压栈（这样最右孩子最先出栈）。

关键点：
  · 这里的压栈顺序与方法三相反：方法三要逆序压（为了让孩子从左到右处理），
    方法四要正序压（为了让最右孩子先出栈）
  · 翻转必须在遍历结束后整体做一次，不能边遍历边插到答案头部（那样是 O(m²)）
  · 空间最坏 O(m)：星形树时栈里同时存下所有兄弟
  · 代码最短，但空间不如递归，标记为非最优

复杂度分析：
  时间 O(m)：遍历一次 + 反转一次
  空间 O(m)：最坏为星形树，栈同时保存 O(m) 个兄弟节点

-------- 四种写法对比 --------
  递归：O(m) 时间、O(h) 空间，最省空间，标记为【最优解】
  解法二：O(m) 时间、O(h) 空间 + 哈希表，常数最大，示范「显式模拟递归」
  解法三：O(m) 时间、O(m) 空间，思路是「孩子处理完再输出自己」
  解法四：O(m) 时间、O(m) 空间，代码最短，最值得记住的迭代写法
  本题 N 叉树没有 Morris 那样的 O(1) 空间遍历写法。
*/
class Solution
{
public:
    // ---------- 解法一：递归（最优解） ----------
    void postorderHelper(Node *node, vector<int> &res)
    {
        if (node == nullptr) return;
        for (Node *child : node->children) postorderHelper(child, res);
        res.push_back(node->val);
    }

    vector<int> postorder(Node *root)
    {
        vector<int> res;
        postorderHelper(root, res);
        return res;
    }

    // ---------- 解法二：迭代 + 哈希表记录孩子下标 ----------
    vector<int> postorderIterativeWithIndex(Node *root)
    {
        vector<int> res;
        if (root == nullptr) return res;

        unordered_map<Node *, int> nextIndex;
        stack<Node *> stk;
        Node *node = root;
        while (!stk.empty() || node != nullptr)
        {
            // 沿着第一个孩子一路向下，只压栈、不输出
            while (node != nullptr)
            {
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
            // 叶子没有表项，-1 + 1 = 0，恰好越界
            int index = (nextIndex.count(node) ? nextIndex[node] : -1) + 1;
            if (index < (int)node->children.size())
            {
                nextIndex[node] = index;
                node = node->children[index];
            }
            else
            {
                // 孩子全部输出完，才输出自己
                res.push_back(node->val);
                stk.pop();
                nextIndex.erase(node);
                node = nullptr;
            }
        }
        return res;
    }

    // ---------- 解法三：迭代 + visited 集合 ----------
    vector<int> postorderIterativeVisited(Node *root)
    {
        vector<int> res;
        if (root == nullptr) return res;

        stack<Node *> stk;
        unordered_set<Node *> visited;
        stk.push(root);
        while (!stk.empty())
        {
            Node *node = stk.top();
            // 叶子，或者孩子已经展开过：此时可以输出自己
            if (node->children.empty() || visited.count(node))
            {
                res.push_back(node->val);
                stk.pop();
                continue;
            }
            // 孩子从右到左入栈，出栈顺序才是从左到右
            for (int i = (int)node->children.size() - 1; i >= 0; --i)
            {
                stk.push(node->children[i]);
            }
            // 标记必须在压完孩子之后：再次成为栈顶时才会走到输出分支
            visited.insert(node);
        }
        return res;
    }

    // ---------- 解法四：前序遍历反转 ----------
    vector<int> postorderIterativeReverse(Node *root)
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
            // 正序压栈：最右孩子最先出栈，得到「根-右-左」
            for (Node *child : node->children) stk.push(child);
        }
        reverse(res.begin(), res.end());
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
    assert(solution.postorder(t1) == want);
    freeNaryTree(t1);

    Node *t2 = buildNaryTree(levelOrder);
    assert(solution.postorderIterativeWithIndex(t2) == want);
    freeNaryTree(t2);

    Node *t3 = buildNaryTree(levelOrder);
    assert(solution.postorderIterativeVisited(t3) == want);
    freeNaryTree(t3);

    Node *t4 = buildNaryTree(levelOrder);
    assert(solution.postorderIterativeReverse(t4) == want);
    freeNaryTree(t4);
}
} // namespace

int main()
{
    // 示例 1
    check({1, NULL_NODE, 3, 2, 4, NULL_NODE, 5, 6}, {5, 6, 3, 2, 4, 1});

    // 示例 2
    check({1, NULL_NODE, 2, 3, 4, 5, NULL_NODE, NULL_NODE, 6, 7, NULL_NODE,
           8, NULL_NODE, 9, 10, NULL_NODE, NULL_NODE, 11, NULL_NODE, 12,
           NULL_NODE, 13, NULL_NODE, NULL_NODE, 14},
          {2, 6, 14, 11, 7, 3, 12, 8, 4, 13, 9, 10, 5, 1});

    // 边界：空树
    check({}, {});

    // 边界：单节点
    check({1}, {1});

    // 边界：根节点没有孩子
    check({1, NULL_NODE}, {1});

    // 边界：根的孩子都是叶子，验证兄弟间保持从左到右
    check({1, NULL_NODE, 2, 3, 4}, {2, 3, 4, 1});

    // 边界：星形树
    check({1, NULL_NODE, 2, 3, 4, 5, 6}, {2, 3, 4, 5, 6, 1});

    // 边界：中间节点没有孩子（2 无孩子，3 有孩子 4）
    check({1, NULL_NODE, 2, 3, NULL_NODE, NULL_NODE, 4}, {2, 4, 3, 1});

    // 边界：深链（每个节点只有一个孩子），深度 200，后序为降序
    {
        vector<int> levelOrder;
        for (int i = 1; i <= 200; ++i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
        }
        vector<int> want;
        for (int i = 200; i >= 1; --i) want.push_back(i);
        check(levelOrder, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
