/*
============================================================================
LCR 149 / 剑指 Offer 32 - I. 从上到下打印二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/
分类：二叉树 · 层序遍历（BFS / 队列）

一棵圣诞树记作根节点为 root 的二叉树，节点值为该位置装饰彩灯的颜色编号。
请按照从 左 到 右 的顺序返回每一层彩灯编号。

说明：本题原名「从上到下打印二叉树」，leetcode.cn 现已改用
decorateRecord 作为函数名，题面意思不变；本题与主站 102 题的
「按层收集」版本不同，输出是一维数组。

示例 1：
输入：root = [8,17,21,18,null,null,6]
输出：[8,17,21,18,6]

提示：
- 节点总数 <= 1000
*/
#include <cassert>
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
解法：BFS + 队列【最优解】

为什么慢：如果按「深度优先」去搜，虽然每个节点只访问一次，但得到的顺序是
一条路走到黑（根-左子树整条-右子树整条），要还原成层序还得额外记层号再排序，
反而把简单问题复杂化。

换个角度：题目要的正是「先来先访问」的顺序 —— 先遇到第 1 层，再遇到第 2 层。
这正是先进先出队列的语义：把根入队，每次从队头取出一个节点输出，并把它
非空的左右孩子依次接到队尾。

关键点：
  · 入队顺序必须是「先左后右」，否则同一层内的顺序会反
  · 孩子为空时不要入队，否则会把空指针带进队列，后续取值崩溃
  · 根为空要单独处理：直接返回空数组，不要入队
  · 队列里同时最多存两层的节点（当前层剩余 + 下一层），空间 O(w)，
    w 为最大层宽；本题不要求按层分组，所以不需要记录层大小

以示例 1（root = [8,17,21,18,null,null,6]）为例：
  队列 [8] → 输出 8，入队 17、21
  输出 17，入队 18 → 输出 21，入队 6
  输出 18、6 → [8,17,21,18,6]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为树的最大层宽，最坏（完全二叉树最后一层）O(n)
*/
class Solution
{
public:
    vector<int> decorateRecord(TreeNode *root)
    {
        vector<int> res;
        if (root == nullptr) return res;

        queue<TreeNode *> q;
        q.push(root);
        while (!q.empty())
        {
            TreeNode *node = q.front();
            q.pop();
            res.push_back(node->val);

            // 先左后右，保证同层内从左到右
            if (node->left != nullptr) q.push(node->left);
            if (node->right != nullptr) q.push(node->right);
        }
        return res;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = 1001; // 节点值范围未限定负数，这里用 1001 作哨兵

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
    TreeNode *root = buildTree(levelOrder);
    assert(solution.decorateRecord(root) == want);
    freeTree(root);
}
} // namespace

int main()
{
    // 示例 1
    check({8, 17, 21, 18, NULL_NODE, NULL_NODE, 6}, {8, 17, 21, 18, 6});

    // 边界：空树
    check({}, {});

    // 边界：单节点
    check({1}, {1});

    // 边界：完全二叉树，验证同层从左到右
    check({1, 2, 3, 4, 5, 6, 7}, {1, 2, 3, 4, 5, 6, 7});

    // 边界：左斜链 / 右斜链
    check({1, 2, NULL_NODE, 3}, {1, 2, 3});
    check({1, NULL_NODE, 2, NULL_NODE, 3}, {1, 2, 3});

    // 边界：右孩子缺失、左孩子存在（不能因为右空就提前结束本层）
    check({1, 2, 3, 4, NULL_NODE, NULL_NODE, 5}, {1, 2, 3, 4, 5});

    // 边界：负数值
    check({-100, -50, 100, 0}, {-100, -50, 100, 0});

    // 边界：满二叉树三层，层序就是 1..15
    {
        vector<int> levelOrder;
        for (int i = 1; i <= 15; ++i) levelOrder.push_back(i);
        vector<int> want;
        for (int i = 1; i <= 15; ++i) want.push_back(i);
        check(levelOrder, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
