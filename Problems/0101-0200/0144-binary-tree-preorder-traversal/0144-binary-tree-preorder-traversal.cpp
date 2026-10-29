/*
============================================================================
144. 二叉树的前序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-preorder-traversal/
分类：二叉树 · 遍历（递归 / 栈 / Morris）

给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

示例 1：
输入：root = [1,null,2,3]
输出：[1,2,3]

示例 2：
输入：root = [1,2,3,4,5,null,8,null,null,6,7,9]
输出：[1,2,4,5,6,7,3,8,9]

示例 3：
输入：root = []
输出：[]

示例 4：
输入：root = [1]
输出：[1]

提示：
树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100

进阶：递归算法很简单，你可以通过迭代算法完成吗？
*/
#include <cassert>
#include <iostream>
#include <stack>
#include <vector>
using namespace std;

// 力扣预置的节点定义
struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

/*
解法一：递归【非最优】

官方方法一。前序遍历的定义就是「根 → 左 → 右」，直接按定义写递归：
  · 空节点返回；
  · 先把当前节点值加入答案；
  · 再递归左子树、递归右子树。

关键点：
  · 三个步骤的顺序就是前/中/后序的唯一区别，前序是「访问」放在最前面
  · 需要用外层函数把结果容器传进去（避免每个递归层级都新建 vector）
  · 递归深度 = 树高；最坏（链状树）为 O(n)

复杂度分析：
  时间 O(n)：每个节点恰好访问一次
  空间 O(h)：递归栈，h 为树高，平均 O(log n)、最坏 O(n)

解法二：迭代（显式栈模拟）【非最优】

官方方法二。把递归里「隐式的栈」显式写出来：
  · 只要当前节点非空，就访问它、把右子树暂时记在栈上，然后一路向左走；
  · 走到空就把最近记录的右子树取出来继续。
这正好对应递归「先处理根、再处理左、最后处理右」的展开顺序。

关键点：
  · 循环条件是 `栈非空 或 当前节点非空`，两个条件缺一不可
  · 入栈的时机是「访问节点时」，出栈后立刻转向它的右子树
  · 与中序遍历的代码只差一行：前序在入栈前就记录答案，中序在出栈时记录
  · 栈的深度最多为树高，同样是 O(h)

复杂度分析：
  时间 O(n)
  空间 O(h)：显式栈

解法三：Morris 遍历【最优解】

官方方法三。前序、中序、后序都能用 Morris 做到 O(1) 额外空间（不计答案数组），
思路是利用叶子节点空着的 right 指针当「回退线索」：
  · 若当前节点没有左子节点：直接访问它，然后走向右子节点；
  · 若有左子节点：在左子树里找到当前节点在中序下的前驱（左子树的最右节点）。
      - 前驱的 right 为空 → 说明第一次来到这里：把 right 指向当前节点（建立线索），
        访问当前节点，然后走向左子节点；
      - 前驱的 right 指向当前节点 → 说明左子树已经遍历完：把线索拆掉（恢复原树），
        走向右子节点。
每个有左子树的节点会被访问两次，无左子树的节点只访问一次，总代价仍是 O(n)。

关键点：
  · 前序与中序的 Morris 差别：前序在「建立线索」时就访问当前节点，
    中序在「拆除线索」时才访问
  · 拆线索这一步必须做，否则会破坏原树结构（本题虽然不检查，但这是正确性的一部分）
  · 找前驱时循环条件是 `p2->right != nullptr && p2->right != p1`，
    两个条件都要有：既要走到最右，又要在已经建立线索时及时停下
  · 建立线索后要 `continue`，因为此时 p1 已经变成左子节点，不能再往下走 p1->right

以 root = [1,null,2,3] 为例（1 只有右子树 2，2 的左孩子是 3）：
  递归/迭代：1 → 右子树 2 → 2 的左孩子 3，得到 [1,2,3]
  Morris：1 没有左孩子 → 访问 1，走向 2；2 有左孩子 3，3 无右孩子 →
          建立线索 3->right=2，访问 2，走向 3；3 无左孩子 → 访问 3，走向线索 2；
          回到 2 时拆线索，走向 2 的右孩子（空）→ 结束。得到 [1,2,3]

复杂度分析：
  时间 O(n)：每个节点至多被访问两次
  空间 O(1)：只用了两个指针，不改动额外数据结构
*/
class Solution
{
public:
    // ---------- 解法一：递归 ----------
    void preorder(TreeNode *root, vector<int> &res)
    {
        if (root == nullptr) return;
        res.push_back(root->val);
        preorder(root->left, res);
        preorder(root->right, res);
    }

    vector<int> preorderTraversalRecursive(TreeNode *root)
    {
        vector<int> res;
        preorder(root, res);
        return res;
    }

    // ---------- 解法二：迭代（栈） ----------
    vector<int> preorderTraversalIterative(TreeNode *root)
    {
        vector<int> res;
        if (root == nullptr) return res;

        stack<TreeNode *> stk;
        TreeNode *node = root;
        while (!stk.empty() || node != nullptr)
        {
            while (node != nullptr)
            {
                res.push_back(node->val);
                stk.push(node);
                node = node->left;
            }
            node = stk.top();
            stk.pop();
            node = node->right;
        }
        return res;
    }

    // ---------- 解法三：Morris 遍历（最优解） ----------
    vector<int> preorderTraversal(TreeNode *root)
    {
        vector<int> res;
        if (root == nullptr) return res;

        TreeNode *p1 = root;
        while (p1 != nullptr)
        {
            TreeNode *p2 = p1->left;
            if (p2 != nullptr)
            {
                // 找左子树中最右节点（当前节点在中序下的前驱）
                while (p2->right != nullptr && p2->right != p1) p2 = p2->right;

                if (p2->right == nullptr)
                {
                    // 第一次到达：建立线索，访问当前节点，转向左子树
                    res.push_back(p1->val);
                    p2->right = p1;
                    p1 = p1->left;
                    continue;
                }
                // 第二次到达：拆除线索，转向右子树
                p2->right = nullptr;
            }
            else
            {
                res.push_back(p1->val);
            }
            p1 = p1->right;
        }
        return res;
    }
};

// ===== 测试辅助：按层序数组（nullptr 用特殊值表示）建树、回收内存 =====
namespace
{
const int NULL_NODE = 1000; // 节点值范围 [-100,100]，用 1000 表示空

TreeNode *buildTree(const vector<int> &levelOrder)
{
    if (levelOrder.empty() || levelOrder[0] == NULL_NODE) return nullptr;

    TreeNode *root = new TreeNode(levelOrder[0]);
    vector<TreeNode *> nodes = {root};
    size_t idx = 1;
    for (size_t i = 0; i < nodes.size() && idx < levelOrder.size(); ++i)
    {
        if (levelOrder[idx] != NULL_NODE)
        {
            nodes[i]->left = new TreeNode(levelOrder[idx]);
            nodes.push_back(nodes[i]->left);
        }
        ++idx;
        if (idx >= levelOrder.size()) break;
        if (levelOrder[idx] != NULL_NODE)
        {
            nodes[i]->right = new TreeNode(levelOrder[idx]);
            nodes.push_back(nodes[i]->right);
        }
        ++idx;
    }
    return root;
}

void freeTree(TreeNode *root)
{
    if (root == nullptr) return;
    freeTree(root->left);
    freeTree(root->right);
    delete root;
}

void check(const vector<int> &levelOrder, const vector<int> &want)
{
    Solution solution;

    // 每跑一种解法都重新建树：Morris 会临时改动指针，迭代/递归不该被影响
    TreeNode *t1 = buildTree(levelOrder);
    assert(solution.preorderTraversalRecursive(t1) == want);
    freeTree(t1);

    TreeNode *t2 = buildTree(levelOrder);
    assert(solution.preorderTraversalIterative(t2) == want);
    freeTree(t2);

    TreeNode *t3 = buildTree(levelOrder);
    assert(solution.preorderTraversal(t3) == want);
    freeTree(t3);
}

// 构造一条右斜链 / 左斜链，用于考察最坏深度
vector<int> rightChain(int n)
{
    vector<int> a;
    for (int i = 1; i <= n; ++i)
    {
        a.push_back(i);
        a.push_back(NULL_NODE); // 左孩子为空
    }
    return a;
}

vector<int> leftChain(int n)
{
    vector<int> a = {1};
    for (int i = 2; i <= n; ++i)
    {
        a.push_back(i);
        a.push_back(NULL_NODE); // 右孩子为空
    }
    return a;
}
} // namespace

int main()
{
    // 示例 1
    check({1, NULL_NODE, 2, 3}, {1, 2, 3});

    // 示例 2
    check({1, 2, 3, 4, 5, NULL_NODE, 8, NULL_NODE, NULL_NODE, 6, 7, 9},
          {1, 2, 4, 5, 6, 7, 3, 8, 9});

    // 示例 3：空树
    check({}, {});

    // 示例 4：单节点
    check({1}, {1});

    // 边界：只有左子树 / 只有右子树
    check({1, 2, NULL_NODE, 3}, {1, 2, 3});
    check({1, NULL_NODE, 2, NULL_NODE, 3}, {1, 2, 3});

    // 边界：完全二叉树
    check({1, 2, 3, 4, 5, 6, 7}, {1, 2, 4, 5, 3, 6, 7});

    // 边界：负数值
    check({-100, -50, 100, 0}, {-100, -50, 0, 100});

    // 边界：链状树（深度 100，考察递归与 Morris 在最坏形态下的正确性）
    {
        vector<int> want;
        for (int i = 1; i <= 100; ++i) want.push_back(i);
        check(rightChain(100), want);
        check(leftChain(100), want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
