/*
============================================================================
543. 二叉树的直径（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/diameter-of-binary-tree/
分类：二叉树 · 后序递归

给你一棵二叉树的根节点，返回该树的 直径 。

二叉树的 直径 是指树中任意两个节点之间最长路径的 长度 。这条路径可能经过也可能不经过根节点 root 。

两节点之间路径的 长度 由它们之间边数表示。

示例 1：
输入：root = [1,2,3,4,5]
输出：3
解释：3 ，取路径 [4,2,1,3] 或 [5,2,1,3] 的长度。

示例 2：
输入：root = [1,2]
输出：1

提示：
- 树中节点数目在范围 [1, 10^4] 内
- -100 <= Node.val <= 100
*/
#include <algorithm>
#include <cassert>
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
解法一：后序递归，边求深度边更新直径【最优解】

为什么慢：最直接的想法是"枚举每个节点，把它当作路径的最高点（拐点），
用左右子树的深度算出经过它的最长路径，取全局最大"。但若每遇到一个节点都
重新递归求一次左右深度，同一条链会被反复数很多遍，最坏（斜链）退化成
O(n^2)。

换个角度：求某个节点的左右深度本身就是后序遍历的一部分。一次后序遍历
返回"以该节点为端点的最长向下路径（也就是深度-1，即边数）"，同时顺手用
`left + right` 更新全局答案 —— 这条和正是以当前节点为拐点的最长路径边数。
深度计算和答案更新共用同一次遍历，信息不浪费。

关键点：
  · 统一用"边数"计量：dfs 返回从该节点向下到叶子的最长**边数**，
    空树返回 -1，叶子返回 0，则 `1 + max(左, 右)`
  · 这样经过当前节点的最长路径边数恰好是 `左 + 右 + 2`
    （两条向下的边各自 +1 才接到当前节点）
  · 答案必须在后序位置（拿到左右结果之后）更新，不能先序
  · 直径不一定经过根：全局最大值要在每个节点处都比较，而不是只算根
  · 空树是最小单位，返回 -1 让叶子的 +2 归零，公式无需特判

以示例 1（root = [1,2,3,4,5]）为例：
  叶子 4、5：更新 -1 + -1 + 2 = 0，返回 0
  节点 2：更新 0 + 0 + 2 = 2（路径 4-2-5），返回 1
  节点 3：更新 0，返回 0
  根 1：更新 1 + 0 + 2 = 3（路径 4-2-1-3）→ 答案 3 ✔

复杂度分析：
  时间 O(n)：每个节点只访问一次
  空间 O(h)：递归栈深度等于树高，最坏（斜链）O(n)

解法二：枚举每个节点求左右深度（暴力）【非最优】

对每个节点都调用一次 `maxDepth` 求左右深度，取 `ld + rd` 的最大值，
再递归到左右孩子继续枚举。
  · 思路最直白，完全对应"直径 = max over x of 左深度 + 右深度"
  · 但每个节点被重复计算 O(h) 次，斜链上退化为 O(n^2)
  · 只适合用来对照验证解法一

复杂度分析：
  时间 O(n^2)：平均 O(n log n)，斜链最坏 O(n^2)
  空间 O(h)：递归栈；但枚举时嵌套递归，栈深度仍为 O(h)
*/
class Solution1
{
public:
    int diameterOfBinaryTree(TreeNode *root)
    {
        ans = 0;
        depth(root);
        return ans;
    }

private:
    int ans;

    // 返回以 node 为根的子树深度，空树约定为 -1（叶子为 0）
    int depth(TreeNode *node)
    {
        if (node == nullptr) return -1;

        int left = depth(node->left);
        int right = depth(node->right);
        // 经过 node 的最长路径边数
        ans = max(ans, left + right + 2);
        return 1 + max(left, right);
    }
};

class Solution2
{
public:
    int diameterOfBinaryTree(TreeNode *root)
    {
        if (root == nullptr) return 0;

        int best = maxDepth(root->left) + maxDepth(root->right) + 2;
        best = max(best, diameterOfBinaryTree(root->left));
        best = max(best, diameterOfBinaryTree(root->right));
        return best;
    }

private:
    // 空树 -1，叶子 0，与解法一保持同一约定
    int maxDepth(TreeNode *node)
    {
        if (node == nullptr) return -1;
        return 1 + max(maxDepth(node->left), maxDepth(node->right));
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = 1001; // 节点值范围 [-100,100]

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

void check(const vector<int> &levelOrder, int want)
{
    Solution1 s1;
    TreeNode *t1 = buildTree(levelOrder);
    assert(s1.diameterOfBinaryTree(t1) == want);
    freeTree(t1);

    Solution2 s2;
    TreeNode *t2 = buildTree(levelOrder);
    assert(s2.diameterOfBinaryTree(t2) == want);
    freeTree(t2);
}
} // namespace

int main()
{
    // 示例 1
    check({1, 2, 3, 4, 5}, 3);

    // 示例 2
    check({1, 2}, 1);

    // 边界：单节点，没有边
    check({1}, 0);

    // 边界：空树（力扣保证至少 1 个节点，这里只验证健壮性）
    check({}, 0);

    // 边界：右斜链
    check({1, NULL_NODE, 2, NULL_NODE, 3}, 2);

    // 边界：左斜链
    check({1, 2, NULL_NODE, 3}, 2);

    // 边界：直径不经过根 —— 长路径藏在左子树内部
    check({1, 2, 3, 4, 5, NULL_NODE, NULL_NODE, 6, NULL_NODE, 7, NULL_NODE, 8, NULL_NODE, 9, NULL_NODE}, 6);

    // 边界：满二叉树，直径穿过根
    check({1, 2, 3, 4, 5, 6, 7}, 4);

    // 边界：负数值不影响结构
    check({-100, -50, 100}, 2);

    // 边界：深链 200 层，答案 199
    {
        vector<int> levelOrder;
        for (int i = 1; i <= 200; ++i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
        }
        check(levelOrder, 199);
    }

    cout << "all tests passed" << endl;
    return 0;
}
