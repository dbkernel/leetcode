/*
============================================================================
124. 二叉树中的最大路径和（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-maximum-path-sum/
分类：二叉树 · 后序递归（单边贡献）

二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。
同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定
经过根节点。

路径和 是路径中各节点值的总和。

给你一个二叉树的根节点 root ，返回其 最大路径和 。

示例 1：
输入：root = [1,2,3]
输出：6
解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6

示例 2：
输入：root = [-10,9,20,null,null,15,7]
输出：42
解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42

提示：
- 树中节点数目范围是 [1, 3 * 10^4]
- -1000 <= Node.val <= 1000
*/
#include <algorithm>
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
解法：后序递归，返回单边最大贡献【最优解】

为什么慢：路径可以在任意节点拐弯，最直白的做法是枚举每个节点当拐点，
再分别求左右子树的最大下行和。若求下行和时每次都重新往下扫一遍，
同一条链会被它的每个祖先各扫一次，斜链上退化成 O(n^2)，而树本身只有 n 个点。

换个角度：把问题拆成"贡献值"——以某节点为起点、只朝一个方向往下走能取得的
最大和。它天然可以后序返回：左、右贡献各自与 0 取 max，挑较大的一边接上
自己。而"以当前节点为拐点"的完整路径和是 左 + 当前 + 右，它只用来更新全局
答案，不参与返回。这样每个节点只算一次。

关键点：
  · 返回给父节点的只能是一条腿：路径一旦向上回到父节点，就不能再分叉，
    同时带两条腿会和父节点组成环
  · 单边贡献与 0 取 max：负数贡献宁可不要，取 0 等价于"从当前节点开始"
  · 全局答案用 左 + 右 + 当前值 更新，并且必须在后序位置（拿到左右结果后）
  · 答案初值取 INT_MIN 而不是 0：全负树必须返回最大的那个负数，不能返回 0
  · 更新答案和返回值是两个不同的量，不要用返回值去更新答案

手推一遍：以示例 2（root = [-10,9,20,null,null,15,7]）为例：
  叶子 15、7：更新 15、7，各自返回 15、7
  叶子 9：更新 9，返回 9
  节点 20：15 + 20 + 7 = 42 更新答案；返回 20 + max(15,7) = 35
  节点 -10：9 + 35 - 10 = 34，不及 42；返回 -10 + 35 = 25
  答案 42 ✔

复杂度分析：
  时间 O(n)：每个节点只进出一次，贡献值一次算清，没有重复扫描
  空间 O(h)：递归栈深度等于树高，最坏（斜链）O(n)
*/
class Solution
{
public:
    int maxPathSum(TreeNode *root)
    {
        best = INT_MIN;
        gain(root);
        return best;
    }

private:
    int best;

    // 返回以 node 为起点、只朝一个方向向下的最大路径和
    int gain(TreeNode *node)
    {
        if (node == nullptr) return 0;

        // 负贡献不如不取，取 0 等价于路径从当前节点开始
        int left = max(gain(node->left), 0);
        int right = max(gain(node->right), 0);

        // 拐点在当前节点：两条腿可以同时要，但只用于更新答案
        best = max(best, node->val + left + right);

        // 交给父节点只能带一条腿，否则路径在父节点处分叉成环
        return node->val + max(left, right);
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = 1001; // 节点值范围 [-1000,1000]

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
    Solution s;
    TreeNode *root = buildTree(levelOrder);
    int got = s.maxPathSum(root);
    freeTree(root);
    assert(got == want);
}
} // namespace

int main()
{
    // 示例 1
    check({1, 2, 3}, 6);

    // 示例 2
    check({-10, 9, 20, NULL_NODE, NULL_NODE, 15, 7}, 42);

    // 边界：单节点，答案就是节点值本身
    check({-3}, -3);

    // 反直觉：全负树不能返回 0，应取最大的单个节点
    check({-5, -2, -3}, -2);

    // 反直觉：负孩子贡献为负，必须丢掉，答案仍是根
    check({2, -1}, 2);

    // 反直觉：根为负，最优路径完全在右子树
    check({-1, 2}, 2);

    // 关键点：路径可以只走一侧，不能强求左右都取
    check({1, -2, 3}, 4);

    // 关键点：最长路径不经过根，藏在左子树内部
    check({1, 2, 3, 4, 5}, 11);

    // 边界：全 0，答案为 0
    check({0, 0, 0}, 0);

    // 边界：负根 + 长负链，答案在链中间取到
    check({-10, -20, -30, NULL_NODE, 5}, 5);

    // 边界：1000 层右斜链全为 1，答案 1000，用来压递归深度
    {
        vector<int> chain = {1};
        for (int i = 2; i <= 1000; ++i)
        {
            chain.push_back(NULL_NODE);
            chain.push_back(1);
        }
        check(chain, 1000);
    }

    cout << "all tests passed" << endl;
    return 0;
}
