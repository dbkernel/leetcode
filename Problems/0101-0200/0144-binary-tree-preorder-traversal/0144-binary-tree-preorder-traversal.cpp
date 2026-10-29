/*
144. 二叉树的前序遍历

给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

示例 1：
输入：root = [1,null,2,3]
输出：[1,2,3]

示例 2：
输入：root = []
输出：[]

示例 3：
输入：root = [1]
输出：[1]

示例 4：
输入：root = [1,2]
输出：[1,2]

示例 5：
输入：root = [1,null,2]
输出：[1,2]

提示：
树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100

进阶：递归算法很简单，你可以通过迭代算法完成吗？

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-tree-preorder-traversal

*/
#include <iostream>
#include <cassert>
#include <vector>
#include <stack>
using namespace std;

// Definition for a binary tree node.
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
方法一：递归

复杂度分析：
- 时间复杂度：O(n)，其中 n 是二叉树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：O(n)，为递归过程中栈的开销，平均情况下为 O(logn)，最坏情况下树呈现链状，为 O(n)。
*/
class Solution
{
public:
    void preorder(TreeNode *root, vector<int> &res)
    {
        if (root == nullptr)
            return;
        res.push_back(root->val);
        preorder(root->left, res);
        preorder(root->right, res);
    }

    vector<int> preorderTraversal(TreeNode *root)
    {
        vector<int> res;
        preorder(root, res);
        return res;
    }
};

/*
方法二：迭代

我们也可以用迭代的方式实现方法一的递归函数，两种方式是等价的，区别在于递归的时候隐式地维护了一个栈，
而我们在迭代的时候需要显式地将这个栈模拟出来，其余的实现与细节都相同，具体可以参考下面的代码。

复杂度分析：
- 时间复杂度：O(n)，其中 n 是二叉树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：O(n)，为迭代过程中显式栈的开销，平均情况下为 O(logn)，最坏情况下树呈现链状，为 O(n)。
*/
class SolutionIteration
{
public:
    vector<int> preorderTraversal(TreeNode *root)
    {
        vector<int> res;
        if (root == nullptr)
            return res;
        stack<TreeNode *> stk;
        TreeNode *node = root;
        while (!stk.empty() || node != nullptr)
        {
            while (node != nullptr)
            {
                res.emplace_back(node->val); // 记录遍历结果
                stk.emplace(node);           // 将中间所用节点入栈，便于遍历右子节点
                node = node->left;
            }
            node = stk.top(); // 对于叶子节点，左子树为空，接着遍历右子树，也为空；之后pop出父节点，并开始遍历右节点
            stk.pop();
            node = node->right;
        }
        return res;
    }
};

/*
方法三：Morris 遍历 ----> 该答案是直接抄来的，没细看

思路与算法

有一种巧妙的方法可以在线性时间内，只占用常数空间来实现前序遍历。
这种方法由 J. H. Morris 在 1979 年的论文「Traversing Binary Trees Simply and Cheaply」中首次提出，因此被称为 Morris 遍历。

Morris 遍历的核心思想是利用树的大量空闲指针，实现空间开销的极限缩减。其前序遍历规则总结如下：
1. 新建临时节点，令该节点为 root；
2. 如果当前节点的左子节点为空，将当前节点加入答案，并遍历当前节点的右子节点；
3. 如果当前节点的左子节点不为空，在当前节点的左子树中找到当前节点在中序遍历下的前驱节点：
    1. 如果前驱节点的右子节点为空，将前驱节点的右子节点设置为当前节点。然后将当前节点加入答案，并将前驱节点的右子节点更新为当前节点。当前节点更新为当前节点的左子节点。
    2. 如果前驱节点的右子节点为当前节点，将它的右子节点重新设为空。当前节点更新为当前节点的右子节点。
4. 重复步骤 2 和步骤 3，直到遍历结束。

这样我们利用 Morris 遍历的方法，前序遍历该二叉树，即可实现线性时间与常数空间的遍历。
*/
class SolutionMorris
{
public:
    vector<int> preorderTraversal(TreeNode *root)
    {
        vector<int> res;
        if (root == nullptr)
        {
            return res;
        }

        TreeNode *p1 = root, *p2 = nullptr;

        while (p1 != nullptr)
        {
            p2 = p1->left;
            if (p2 != nullptr)
            {
                while (p2->right != nullptr && p2->right != p1)
                    p2 = p2->right;
                if (p2->right == nullptr)
                {
                    res.emplace_back(p1->val);
                    p2->right = p1;
                    p1 = p1->left;
                    continue;
                }
                else
                    p2->right = nullptr;
            }
            else
                res.emplace_back(p1->val);
            p1 = p1->right;
        }
        return res;
    }
};
