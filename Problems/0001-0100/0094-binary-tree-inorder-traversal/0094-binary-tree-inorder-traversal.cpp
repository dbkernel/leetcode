/*
============================================================================
94. 二叉树的中序遍历（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-inorder-traversal/
分类：二叉树 · 遍历（递归 / 栈 / Morris）

给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。

示例 1：
输入：root = [1,null,2,3]
输出：[1,3,2]

示例 2：
输入：root = []
输出：[]

示例 3：
输入：root = [1]
输出：[1]

提示：
树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100

进阶: 递归算法很简单，你可以通过迭代算法完成吗？
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

官方方法一。中序遍历的定义是「左 → 根 → 右」，直接按定义写递归：
  · 空节点返回；
  · 先递归左子树，再把当前节点值加入答案，最后递归右子树。

关键点：
  · 与 144 前序遍历只差「访问语句的位置」：中序夹在两个递归之间
  · 中序遍历对二叉搜索树有特殊意义——结果一定是升序序列（后续 98 题会用到）
  · 递归深度 = 树高，最坏（链状树）为 O(n)

复杂度分析：
  时间 O(n)：每个节点恰好访问一次
  空间 O(h)：递归栈

解法二：迭代（显式栈模拟）【非最优】

官方方法二。递归的展开顺序是「一路向左、到底后访问、再转向右子树」，
用显式栈照着写即可：
  · 只要当前节点非空，就压栈并一路向左（先不访问，因为左子树还没处理完）；
  · 走到空就弹栈——此时该节点是「左子树已处理完」的节点，访问它，然后转向它的右子树。

关键点：
  · 与前序遍历的唯一差别：前序在入栈前记录答案，中序在出栈时记录
  · 循环条件仍是「栈非空 或 当前节点非空」
  · 内层循环只压栈不访问，这点最容易和先序写混
  · 栈深度最多为树高 O(h)

复杂度分析：
  时间 O(n)
  空间 O(h)：显式栈

解法三：Morris 中序遍历【最优解】

官方方法三。用叶子节点空着的 right 指针做「回退线索」，把空间降到 O(1)：
  · 若当前节点 x 没有左孩子：访问 x，然后走向右孩子；
  · 若 x 有左孩子：找到 x 左子树的最右节点（即 x 在中序下的前驱 predecessor）。
      - predecessor 的 right 为空 → 说明第一次到达：把 right 指向 x（建立线索），
        然后走向 x 的左孩子（**此时不访问 x**，因为左子树还没遍历）；
      - predecessor 的 right 指向 x → 说明左子树已遍历完：拆掉线索，
        访问 x，然后走向 x 的右孩子。
  · 重复以上步骤直到 x 为空。

关键点：
  · 与 Morris 前序的关键差别：前序在「建立线索」时访问，中序在「拆除线索」时才访问
  · 拆线索必须做，否则破坏原树结构
  · 找前驱的循环条件要同时判断 `right != nullptr` 和 `right != x`
  · 每个节点至多被访问两次，总代价仍是 O(n)

以 root = [1,null,2,3] 为例（1 只有右子树 2，2 的左孩子是 3）：
  中序应为 3 → 2 → 1？注意：1 的右孩子是 2，2 的左孩子是 3，
  中序 = 左(空) → 1 → 右子树的中序 = 3 → 2，结果为 [1,3,2]，与示例一致。
  Morris：1 无左孩子 → 访问 1，走向 2；2 有左孩子 3，3 无右孩子 →
          建立线索 3.right = 2，走向 3；3 无左孩子 → 访问 3，走向线索 2；
          回到 2，发现线索指向自己 → 拆线索，访问 2，走向 2 的右孩子（空）→ 结束。
          得到 [1,3,2]

复杂度分析：
  时间 O(n)
  空间 O(1)：只用两个指针
*/
class Solution
{
public:
    // ---------- 解法一：递归 ----------
    void inorder(TreeNode *root, vector<int> &res)
    {
        if (root == nullptr) return;
        inorder(root->left, res);
        res.push_back(root->val);
        inorder(root->right, res);
    }

    vector<int> inorderTraversalRecursive(TreeNode *root)
    {
        vector<int> res;
        inorder(root, res);
        return res;
    }

    // ---------- 解法二：迭代（栈） ----------
    vector<int> inorderTraversalIterative(TreeNode *root)
    {
        vector<int> res;
        stack<TreeNode *> stk;
        TreeNode *node = root;
        while (!stk.empty() || node != nullptr)
        {
            // 一路向左，只压栈不访问
            while (node != nullptr)
            {
                stk.push(node);
                node = node->left;
            }
            // 弹出时左子树已处理完，此时才访问
            node = stk.top();
            stk.pop();
            res.push_back(node->val);
            node = node->right;
        }
        return res;
    }

    // ---------- 解法三：Morris 中序遍历（最优解） ----------
    vector<int> inorderTraversal(TreeNode *root)
    {
        vector<int> res;
        TreeNode *x = root;
        while (x != nullptr)
        {
            if (x->left != nullptr)
            {
                // 找 x 在中序下的前驱：左子树的最右节点
                TreeNode *predecessor = x->left;
                while (predecessor->right != nullptr && predecessor->right != x)
                    predecessor = predecessor->right;

                if (predecessor->right == nullptr)
                {
                    // 第一次到达：建立线索，转向左子树（此刻不访问 x）
                    predecessor->right = x;
                    x = x->left;
                }
                else
                {
                    // 左子树已遍历完：拆线索，访问 x，转向右子树
                    res.push_back(x->val);
                    predecessor->right = nullptr;
                    x = x->right;
                }
            }
            else
            {
                // 没有左孩子，直接访问并转向右孩子
                res.push_back(x->val);
                x = x->right;
            }
        }
        return res;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = 1000; // 节点值范围 [-100,100]

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

    TreeNode *t1 = buildTree(levelOrder);
    assert(solution.inorderTraversalRecursive(t1) == want);
    freeTree(t1);

    TreeNode *t2 = buildTree(levelOrder);
    assert(solution.inorderTraversalIterative(t2) == want);
    freeTree(t2);

    TreeNode *t3 = buildTree(levelOrder);
    assert(solution.inorderTraversal(t3) == want);
    freeTree(t3);
}
} // namespace

int main()
{
    // 示例 1
    check({1, NULL_NODE, 2, 3}, {1, 3, 2});

    // 示例 2：空树
    check({}, {});

    // 示例 3：单节点
    check({1}, {1});

    // 边界：完全二叉树
    check({1, 2, 3, 4, 5, 6, 7}, {4, 2, 5, 1, 6, 3, 7});

    // 边界：左斜链（中序是降序）
    check({3, 2, NULL_NODE, 1}, {1, 2, 3});

    // 边界：右斜链（中序是升序）
    check({1, NULL_NODE, 2, NULL_NODE, 3}, {1, 2, 3});

    // 边界：负数值
    check({-100, -50, 100, 0}, {0, -50, -100, 100});

    // 边界：二叉搜索树，中序应为升序
    check({5, 3, 8, 1, 4, 7, 9}, {1, 3, 4, 5, 7, 8, 9});

    // 边界：深层右斜链（深度 100）
    {
        vector<int> levelOrder;
        for (int i = 1; i <= 100; ++i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
        }
        vector<int> want;
        for (int i = 1; i <= 100; ++i) want.push_back(i);
        check(levelOrder, want);
    }

    // 边界：深层左斜链（深度 100，中序为降序）
    {
        vector<int> levelOrder = {100};
        for (int i = 99; i >= 1; --i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
        }
        vector<int> want;
        for (int i = 1; i <= 100; ++i) want.push_back(i);
        check(levelOrder, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
