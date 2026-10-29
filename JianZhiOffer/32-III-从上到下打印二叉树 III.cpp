/*
============================================================================
LCR 151 / 剑指 Offer 32 - III. 从上到下打印二叉树 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/
分类：二叉树 · 层序遍历（BFS / 双端队列）

一棵圣诞树记作根节点为 root 的二叉树，节点值为该位置装饰彩灯的颜色编号。
请按照如下规则记录彩灯装饰结果：

- 第一层按照从左到右的顺序记录
- 除第一层外每一层的记录顺序均与上一层相反。即第一层为从左到右，
  第二层为从右到左。

说明：本题原名「从上到下打印二叉树 III」，leetcode.cn 现已改用
decorateRecord 作为函数名，题面意思不变。

示例 1：
输入：root = [8,17,21,18,null,null,6]
输出：[[8],[21,17],[18,6]]

提示：
- 节点总数 <= 1000
*/
#include <cassert>
#include <deque>
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
解法一：BFS 按层收集 + 奇数层反转【非最优】

为什么慢：直接的 BFS 只能保证「从左右到」的顺序；要得到之字形，最顺手的
补丁是每层收完再 `reverse` 一次。代码最短，但奇数层的元素被多写了一遍，
每一层都是一次额外的 O(层宽) 操作，总多出 O(n) 的搬运。

换个角度：与其收完再整体翻转，不如「收的时候就放到正确的一端」。
一层要么全从头插，要么全从尾插，用一个双端队列就能一步到位。

关键点：
  · 判断方向只能看层号，不能看节点值或队列长度
  · reverse 只作用于奇数层，层号从 0 开始数
  · 保证本题正确性的核心仍是先左后右入队

复杂度分析：
  时间 O(n)：遍历 O(n)，最坏反转总量也不超过 O(n)
  空间 O(w)：队列与当前层

解法二：BFS + 双端队列按方向插入【最优解】

按层处理，每层开始前记录 `size`，然后：
  · 偶数层（0、2、4…）：从队尾 push_back，得到从左到右；
  · 奇数层（1、3、5…）：从队首 push_front，得到从右到左。
注意孩子入队的方向（左、右）始终不变 —— 只翻转当前层的输出，不翻转遍历顺序。

关键点：
  · 需要翻转的是「当前层往 deque 里放的位置」，而不是「孩子入队的顺序」
  · 遍历节点的顺序永远是左孩子在前，否则下一层的父子关系都会乱
  · size 必须在 for 循环之前锁定，循环中队列长度会增长
  · deque 两端插入都是 O(1)，所以没有解法一那样的额外搬运

以示例 1（root = [8,17,21,18,null,null,6]）为例：
  第 0 层 size=1，从左到右 → [8]
  第 1 层 size=2，从右到左：先取 17 插到队首 [17]，再取 21 插到队首
  → [21,17]
  第 2 层 size=2，从左到右：18 插尾 [18]，6 插尾 [18,6]
  → [[8],[21,17],[18,6]]

复杂度分析：
  时间 O(n)：每个节点入队、出队、写入当前层各一次
  空间 O(w)：BFS 队列与当前层的 deque 各占 O(w)
*/
class Solution1
{
public:
    vector<vector<int>> decorateRecord(TreeNode *root)
    {
        vector<vector<int>> res;
        if (root == nullptr) return res;

        queue<TreeNode *> q;
        q.push(root);
        bool leftToRight = true;
        while (!q.empty())
        {
            int size = (int)q.size();
            vector<int> level;
            level.reserve(size);
            for (int i = 0; i < size; ++i)
            {
                TreeNode *node = q.front();
                q.pop();
                level.push_back(node->val);
                if (node->left != nullptr) q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }
            // 奇数层整体翻转
            if (!leftToRight) reverse(level.begin(), level.end());
            res.push_back(std::move(level));
            leftToRight = !leftToRight;
        }
        return res;
    }
};

class Solution2
{
public:
    vector<vector<int>> decorateRecord(TreeNode *root)
    {
        vector<vector<int>> res;
        if (root == nullptr) return res;

        queue<TreeNode *> q;
        q.push(root);
        bool leftToRight = true;
        while (!q.empty())
        {
            int size = (int)q.size();
            deque<int> level;
            for (int i = 0; i < size; ++i)
            {
                TreeNode *node = q.front();
                q.pop();

                // 只改变写入当前层的位置，孩子入队顺序始终先左后右
                if (leftToRight)
                {
                    level.push_back(node->val);
                }
                else
                {
                    level.push_front(node->val);
                }

                if (node->left != nullptr) q.push(node->left);
                if (node->right != nullptr) q.push(node->right);
            }
            res.emplace_back(level.begin(), level.end());
            leftToRight = !leftToRight;
        }
        return res;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = 1001; // 节点值范围未限定负数，用 1001 作哨兵

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
    Solution1 s1;
    TreeNode *t1 = buildTree(levelOrder);
    assert(s1.decorateRecord(t1) == want);
    freeTree(t1);

    Solution2 s2;
    TreeNode *t2 = buildTree(levelOrder);
    assert(s2.decorateRecord(t2) == want);
    freeTree(t2);
}
} // namespace

int main()
{
    // 示例 1
    check({8, 17, 21, 18, NULL_NODE, NULL_NODE, 6}, {{8}, {21, 17}, {18, 6}});

    // 边界：空树
    check({}, {});

    // 边界：单节点
    check({1}, {{1}});

    // 边界：只有一层两个节点
    check({1, 2, 3}, {{1}, {3, 2}});

    // 边界：满二叉树三层
    check({1, 2, 3, 4, 5, 6, 7}, {{1}, {3, 2}, {4, 5, 6, 7}});

    // 边界：满二叉树四层（第 4 层从右到左）
    check({1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15},
          {{1}, {3, 2}, {4, 5, 6, 7}, {15, 14, 13, 12, 11, 10, 9, 8}});

    // 边界：左斜链 / 右斜链，每层只有一个节点，方向翻转不影响结果
    check({1, 2, NULL_NODE, 3}, {{1}, {2}, {3}});
    check({1, NULL_NODE, 2, NULL_NODE, 3}, {{1}, {2}, {3}});

    // 边界：缺左孩子，奇数层只有单个节点，验证不会重复写入
    check({1, 2, 3, NULL_NODE, 4, 5}, {{1}, {3, 2}, {4, 5}});

    // 边界：负数值
    check({-100, -50, 100, 0}, {{-100}, {100, -50}, {0}});

    cout << "all tests passed" << endl;
    return 0;
}
