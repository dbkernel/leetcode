/*
============================================================================
102. 二叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-level-order-traversal/
分类：二叉树 · 层序遍历（BFS / 队列）

给你二叉树的根节点 root ，返回其节点值的 层序遍历 。
（即逐层地，从左到右访问所有节点）。

示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：[[3],[9,20],[15,7]]

示例 2：
输入：root = [1]
输出：[[1]]

示例 3：
输入：root = []
输出：[]

提示：
- 树中节点数目在范围 [0, 2000] 内
- -1000 <= Node.val <= 1000
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
解法：BFS + 按层定长出队【最优解】

为什么慢：最朴素的 BFS 只知道「下一个是谁」，并不知道「这一层到哪儿结束」。
如果给每个节点额外挂一个层号，再用哈希表按层号分组，就要多存 O(n) 个层号；
最后还要按下标顺序遍历哈希表，空间和实现都变重。

换个角度：BFS 的队列有一个几乎白送的性质 —— 某一时刻队列里的元素，恰好就是
「当前这一层剩下的节点 + 下一层已经入队的节点」。因此只要在每轮开始时先记下
`s = q.size()`，就锁定了本层的边界：接下来连续出队 s 个，得到的就是完整的一层，
而这 s 个节点新入队的恰好全是下一层。

关键点：
  · `s` 必须在 for 循环之前取，循环中队列长度会增长，不能写成 `i < q.size()`
  · 每轮先 `res.push_back({})` 占好一层的位置，再往这一层里追加
  · 入队仍要「先左后右」，并且跳过空孩子，才能保证同层从左到右
  · 根为空直接返回空数组：不要入队再靠循环兜底
  · 空树不会进入循环，所以不会多出一个空层

以示例 1（root = [3,9,20,null,null,15,7]）为例：
  第 1 轮 s=1 → 出 3，入 9、20 → [[3]]
  第 2 轮 s=2 → 出 9，入空；出 20，入 15、7 → [[3],[9,20]]
  第 3 轮 s=2 → 出 15、7 → [[3],[9,20],[15,7]]

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案数组外，队列最多同时存两层节点
*/
class Solution
{
public:
    vector<vector<int>> levelOrder(TreeNode *root)
    {
        vector<vector<int>> res;
        if (root == nullptr) return res;

        queue<TreeNode *> q;
        q.push(root);
        while (!q.empty())
        {
            // 本层节点个数必须在循环前锁定，循环中队列还会增长
            int levelSize = (int)q.size();
            vector<int> level;
            level.reserve(levelSize);
            for (int i = 0; i < levelSize; ++i)
            {
                TreeNode *node = q.front();
                q.pop();
                level.push_back(node->val);
                if (node->left != nullptr) q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }
            res.push_back(std::move(level));
        }
        return res;
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

void check(const vector<int> &levelOrder, const vector<vector<int>> &want)
{
    Solution solution;
    TreeNode *root = buildTree(levelOrder);
    assert(solution.levelOrder(root) == want);
    freeTree(root);
}
} // namespace

int main()
{
    // 示例 1
    check({3, 9, 20, NULL_NODE, NULL_NODE, 15, 7}, {{3}, {9, 20}, {15, 7}});

    // 示例 2：单节点
    check({1}, {{1}});

    // 示例 3：空树
    check({}, {});

    // 边界：完全二叉树，层宽递增
    check({1, 2, 3, 4, 5, 6, 7}, {{1}, {2, 3}, {4, 5, 6, 7}});

    // 边界：左斜链 / 右斜链，每层只有一个节点
    check({1, 2, NULL_NODE, 3}, {{1}, {2}, {3}});
    check({1, NULL_NODE, 2, NULL_NODE, 3}, {{1}, {2}, {3}});

    // 边界：缺左孩子，层内顺序不能乱
    check({1, 2, 3, NULL_NODE, 4, 5}, {{1}, {2, 3}, {4, 5}});

    // 边界：负数值
    check({-1000, -500, 1000, 0}, {{-1000}, {-500, 1000}, {0}});

    // 边界：满二叉树四层
    {
        vector<int> levelOrder;
        for (int i = 1; i <= 15; ++i) levelOrder.push_back(i);
        vector<vector<int>> want;
        int start = 1;
        for (int width = 1; width <= 8; width <<= 1)
        {
            vector<int> level;
            for (int j = 0; j < width; ++j) level.push_back(start + j);
            want.push_back(level);
            start += width;
        }
        check(levelOrder, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
