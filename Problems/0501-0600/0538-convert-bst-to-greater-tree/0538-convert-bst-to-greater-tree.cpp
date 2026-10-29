/*
============================================================================
538. 把二叉搜索树转换为累加树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/convert-bst-to-greater-tree/
分类：二叉搜索树 · 反序中序遍历 / Morris

给出二叉 搜索 树的根节点，该树的节点值各不相同，请你将其转换为累加树
（Greater Sum Tree），使每个节点 node 的新值等于原树中大于或等于
node.val 的值之和。

提醒一下，二叉搜索树满足下列约束条件：
- 节点的左子树仅包含键 小于 节点键的节点。
- 节点的右子树仅包含键 大于 节点键的节点。
- 左右子树也必须是二叉搜索树。

示例 1：
输入：[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
输出：[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]

示例 2：
输入：root = [0,null,1]
输出：[1,null,1]

示例 3：
输入：root = [1,0,2]
输出：[3,3,2]

示例 4：
输入：root = [3,2,4,1]
输出：[7,9,4,10]

提示：
- 树中的节点数介于 0 和 10^4 之间。
- 每个节点的值介于 -10^4 和 10^4 之间。
- 树中的所有值 互不相同 。
- 给定的树为二叉搜索树。

注意：本题和 1038 题相同。
*/
#include <cassert>
#include <climits>
#include <iostream>
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
解法一：反序中序遍历（递归）【非最优】

为什么慢：如果按"每个节点都重新遍历一遍整棵树，把大于等于它的值加起来"
来写，是 O(n^2)。即使用中序遍历拿到升序数组再算后缀和，也要 O(n) 额外数组。

换个角度：BST 的中序是升序，那么"右 → 根 → 左"就是降序。降序遍历时，
当前节点需要的"所有大于等于它的值之和"恰好就是**已经累加过的部分**：
  · 维护一个累加和 sum；
  · 每访问一个节点，先 `sum += node->val`，再把 `node->val = sum`。
  因为降序保证先处理的一定更大，累加和天然满足"大于等于自己"。

关键点：
  · 顺序必须是右 → 根 → 左，写反了累加方向就变成"小于等于"了
  · 必须先累加再赋值，否则当前值会被覆盖两次
  · 不需要额外数组，也不需要知道整棵树的总和
  · 递归深度 = 树高，节点数 ≤ 10^4，斜链下递归栈可能到 10^4 层，
    在 C++ 里通常可行但偏紧，因此官方还给了 Morris 版本

以示例 4（root = [3,2,4,1]）为例：
  访问 4：sum = 4，节点变 4；
  访问 3：sum = 7，节点变 7；
  访问 2：sum = 9，节点变 9；
  访问 1：sum = 10，节点变 10；
  结果 [7,9,4,10] ✔

复杂度分析：
  时间 O(n)：每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：Morris 反序中序遍历【最优解】

官方方法二。把解法一搬到 Morris 上，就能去掉递归栈，做到 O(1) 辅助空间。
思路是普通 Morris 的镜像版本：
  · 若当前节点没有右孩子，访问它，转向左孩子；
  · 否则找右子树的最左节点 p（反序中序的前驱）：
      - p->left 为空 → 指向当前节点（建立线索），当前节点向右走；
      - p->left 就是当前节点 → 说明右子树已遍历完，断开线索（还原树），
        访问当前节点，再向左走。
  "访问"做的事和递归版一样：`sum += val; val = sum`。

关键点：
  · 这是 Morris 的镜像版本：普通中序用 right 指针当线索、向左找最右；
    反序中序用 left 指针当线索、向右找最左
  · 线索的建立与断开必须成对，否则会把输入树改坏
  · 找最左节点时 `p->left != currentNode` 防止绕圈
  · 不需要提前退出，所以天然会把所有线索还原干净
  · 辅助空间只有几个指针，是最优解

以示例 4（root = [3,2,4,1]）为例：
  3 有右孩子：最左节点是 4，4->left 空 → 建线索 4->left=3，向右到 4；
  4 无右孩子 → 访问 4（sum=4），向左回 3；
  3 的最左节点 4 的 left 指向自己 → 断线索，访问 3（sum=7），向左到 2；
  2 无右孩子 → 访问 2（sum=9），向左到 1；
  1 无右孩子 → 访问 1（sum=10）；
  结果 [7,9,4,10] ✔

复杂度分析：
  时间 O(n)：每条边最多走两遍（建线索、断线索）
  空间 O(1)：只用了几个指针

-------- 两种写法对比 --------
  递归反序中序：O(n) 时间、O(h) 空间，写法最短，优先记忆
  Morris 反序中序：O(n) 时间、O(1) 空间，标记为【最优解】，代价是代码更绕
*/
class Solution1
{
public:
    TreeNode *convertBST(TreeNode *root)
    {
        sum = 0;
        dfs(root);
        return root;
    }

private:
    int sum;

    void dfs(TreeNode *node)
    {
        if (node == nullptr) return;

        dfs(node->right);
        sum += node->val;
        node->val = sum;
        dfs(node->left);
    }
};

class Solution2
{
public:
    TreeNode *convertBST(TreeNode *root)
    {
        int sum = 0;
        TreeNode *node = root;

        while (node != nullptr)
        {
            if (node->right == nullptr)
            {
                // 访问 node
                sum += node->val;
                node->val = sum;
                node = node->left;
            }
            else
            {
                // 找右子树的最左节点（反序中序的前驱）
                TreeNode *p = node->right;
                while (p->left != nullptr && p->left != node) p = p->left;

                if (p->left == nullptr)
                {
                    // 建立线索，向右
                    p->left = node;
                    node = node->right;
                }
                else
                {
                    // 右子树走完，断开线索，访问 node，向左
                    p->left = nullptr;
                    sum += node->val;
                    node->val = sum;
                    node = node->left;
                }
            }
        }
        return root;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = INT_MAX; // 节点值范围 [-10^4, 10^4]

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
    assert(sameTree(s1.convertBST(t1), w1));
    freeTree(t1);
    freeTree(w1);

    Solution2 s2;
    TreeNode *t2 = buildTree(input);
    TreeNode *w2 = buildTree(want);
    assert(sameTree(s2.convertBST(t2), w2));
    freeTree(t2);
    freeTree(w2);
}
} // namespace

int main()
{
    // 示例 1
    check({4, 1, 6, 0, 2, 5, 7, NULL_NODE, NULL_NODE, NULL_NODE, 3, NULL_NODE, NULL_NODE, NULL_NODE, 8},
          {30, 36, 21, 36, 35, 26, 15, NULL_NODE, NULL_NODE, NULL_NODE, 33, NULL_NODE, NULL_NODE, NULL_NODE, 8});

    // 示例 2
    check({0, NULL_NODE, 1}, {1, NULL_NODE, 1});

    // 示例 3
    check({1, 0, 2}, {3, 3, 2});

    // 示例 4
    check({3, 2, 4, 1}, {7, 9, 4, 10});

    // 边界：空树
    check({}, {});

    // 边界：单节点，新值等于自身
    check({5}, {5});

    // 边界：只有右孩子的链，最右节点值不变，越往左累加越多
    check({1, NULL_NODE, 2, NULL_NODE, 3}, {6, NULL_NODE, 5, NULL_NODE, 3});

    // 边界：只有左孩子的链
    check({3, 2, NULL_NODE, 1}, {3, 5, NULL_NODE, 6});

    // 边界：负数
    check({-1, -2, 0}, {-1, -3, 0});

    // 边界：100 层右斜链，值 1..100，节点 v 的新值是 v..100 之和
    {
        vector<int> chain = {1};
        vector<int> want = {100 * 101 / 2};
        for (int i = 2; i <= 100; ++i)
        {
            chain.push_back(NULL_NODE);
            chain.push_back(i);
            want.push_back(NULL_NODE);
            want.push_back((100 + i) * (101 - i) / 2);
        }
        check(chain, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
