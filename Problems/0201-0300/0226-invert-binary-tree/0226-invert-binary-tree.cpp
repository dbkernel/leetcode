/*
============================================================================
226. 翻转二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/invert-binary-tree/
分类：二叉树 · 递归 / 层序遍历

给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。

示例 1：
输入：root = [4,2,7,1,3,6,9]
输出：[4,7,2,9,6,3,1]

示例 2：
输入：root = [2,1,3]
输出：[2,3,1]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目范围在 [0, 100] 内
- -100 <= Node.val <= 100
*/
#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <queue>
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
解法一：递归（交换左右孩子 + 递归翻转）【最优解】

为什么慢：按"新建一棵镜像树"的思路写也能过，但要额外 new 出一整棵树，
空间翻倍；而且原树被丢弃，白白浪费。层序迭代也能做，但要显式维护队列，
代码并不比递归短。

换个角度：翻转是一棵子树一个动作 —— 对当前节点，先交换它的左右孩子，
再分别翻转这两棵子树即可。由于"翻转左右子树"与"交换左右孩子"可交换顺序，
先序、后序写都成立；中序则会因为交换后访问的子树已变而需要小心，一般不用。

关键点：
  · 每次递归只需做两件事：交换 left / right，然后递归两个孩子
  · 空节点直接返回，作为递归出口
  · 交换后递归时的 left / right 已是"翻转前"的另一侧，注意别重复交换
  · 递归深度 = 树高，本题节点数 ≤ 100，绝对安全
  · 若题目不允许修改原树，才需要新建节点；本题允许原地翻转

以示例 1（root = [4,2,7,1,3,6,9]）为例：
  节点 1、3、6、9 无孩子，返回自身；
  节点 2 交换左右（1、3）→ 变 (3,1)；节点 7 同理变 (9,6)；
  根 4 交换左右（2、7）→ (7,2)，结果 [4,7,2,9,6,3,1] ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：BFS 层序迭代【非最优】

官方方法二。用队列做层序遍历，每弹出一个节点就交换它的左右孩子，
再把非空孩子入队 —— 因为交换只依赖节点自身，与访问顺序无关。

关键点：
  · 交换发生在"弹出"时，孩子入队顺序无所谓，反正后面还会各自交换
  · 空树直接返回 root（nullptr），不要先入队
  · 不需要按层分批处理，本题没有"按层"语义，一层一层反而多余
  · 空间 O(w)：宽树时要存下整层节点

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/
class Solution1
{
public:
    TreeNode *invertTree(TreeNode *root)
    {
        if (root == nullptr) return nullptr;

        swap(root->left, root->right);
        invertTree(root->left);
        invertTree(root->right);
        return root;
    }
};

class Solution2
{
public:
    TreeNode *invertTree(TreeNode *root)
    {
        if (root == nullptr) return nullptr;

        queue<TreeNode *> q;
        q.push(root);
        while (!q.empty())
        {
            TreeNode *node = q.front();
            q.pop();

            swap(node->left, node->right);
            if (node->left != nullptr) q.push(node->left);
            if (node->right != nullptr) q.push(node->right);
        }
        return root;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = INT_MAX; // 节点值范围 [-100,100]

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

bool sameTree(TreeNode *a, TreeNode *b)
{
    if (a == nullptr || b == nullptr) return a == b;
    return a->val == b->val && sameTree(a->left, b->left) && sameTree(a->right, b->right);
}

void check(const vector<int> &input, const vector<int> &want)
{
    Solution1 s1;
    TreeNode *t1 = buildTree(input);
    TreeNode *w1 = buildTree(want);
    assert(sameTree(s1.invertTree(t1), w1));
    freeTree(t1);
    freeTree(w1);

    Solution2 s2;
    TreeNode *t2 = buildTree(input);
    TreeNode *w2 = buildTree(want);
    assert(sameTree(s2.invertTree(t2), w2));
    freeTree(t2);
    freeTree(w2);
}
} // namespace

int main()
{
    // 示例 1
    check({4, 2, 7, 1, 3, 6, 9}, {4, 7, 2, 9, 6, 3, 1});

    // 示例 2
    check({2, 1, 3}, {2, 3, 1});

    // 示例 3：空树
    check({}, {});

    // 边界：单节点
    check({1}, {1});

    // 边界：只有左孩子 → 翻转后变成右孩子
    check({1, 2}, {1, NULL_NODE, 2});

    // 边界：只有右孩子
    check({1, NULL_NODE, 2}, {1, 2});

    // 边界：叶子层的孩子也要换位
    check({1, 2, 3, 4}, {1, 3, 2, NULL_NODE, NULL_NODE, NULL_NODE, 4});

    // 边界：负数不影响结构
    check({-100, -50, 100}, {-100, 100, -50});

    // 边界：左斜链 100 层，翻转后变成同深度的右斜链
    {
        vector<int> chain = {1};
        for (int i = 2; i <= 100; ++i)
        {
            chain.push_back(i);
            chain.push_back(NULL_NODE);
        }
        vector<int> want;
        for (int i = 1; i <= 100; ++i)
        {
            want.push_back(i);
            want.push_back(NULL_NODE);
        }
        check(chain, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
