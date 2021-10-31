/*
145. 二叉树的后序遍历

给定一个二叉树，返回它的 后序 遍历。

示例:
输入: [1,null,2,3]
   1
    \
     2
    /
   3

输出: [3,2,1]

进阶: 递归算法很简单，你可以通过迭代算法完成吗？
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
- 时间复杂度：O(n)，其中 n 是二叉搜索树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：O(n)，为递归过程中栈的开销，平均情况下为 O(logn)，最坏情况下树呈现链状，为 O(n)。
*/
class Solution
{
public:
    void postorder(TreeNode *root, vector<int> &res)
    {
        if (root == nullptr)
            return;
        postorder(root->left, res);
        postorder(root->right, res);
        res.emplace_back(root->val);
    }

    vector<int> postorderTraversal(TreeNode *root)
    {
        vector<int> res;
        postorder(root, res);
        return res;
    }
};

/*
方法二：迭代

我们也可以用迭代的方式实现方法一的递归函数，两种方式是等价的，区别在于递归的时候隐式地维护了一个栈，
而我们在迭代的时候需要显式地将这个栈模拟出来，其余的实现与细节都相同。

复杂度分析：
- 时间复杂度：O(n)，其中 n 是二叉搜索树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：O(n)，为递归过程中栈的开销，平均情况下为 O(logn)，最坏情况下树呈现链状，为 O(n)。
*/
class SolutionIteration
{
public:
    vector<int> postorderTraversal(TreeNode *root)
    {
        vector<int> res;
        if (root == nullptr)
            return res;
        TreeNode *node = root, *prev = nullptr;
        stack<TreeNode *> stk;
        while (!stk.empty() || node)
        {
            // 遍历左子树，并将中间节点入栈；循环结束时，node等于node->left，即 nullptr
            while (node)
            {
                stk.emplace(node);
                node = node->left;
            }

            node = stk.top();
            stk.pop();

            if (node->right == nullptr || node->right == prev)
            {
                res.emplace_back(node->val);
                prev = node;    // 如果node->right等于prev，表示右子树扫描完成，可以输出node了
                node = nullptr; // 为了跳过下一轮对左子树的遍历
            }
            else // 右子树不为空，需要扫描右子树，因此将父节点入栈，开始扫描右子树
            {
                stk.emplace(node);
                node = node->right;
            }
            return res;
        }
    }
};

/*
方法三：Morris 遍历 ----> 该答案是直接抄来的，没细看

有一种巧妙的方法可以在线性时间内，只占用常数空间来实现后序遍历。
这种方法由 J. H. Morris 在 1979 年的论文「Traversing Binary Trees Simply and Cheaply」中首次提出，因此被称为 Morris 遍历。

Morris 遍历的核心思想是利用树的大量空闲指针，实现空间开销的极限缩减。其后序遍历规则总结如下：
1. 新建临时节点，令该节点为 root；
2. 如果当前节点的左子节点为空，则遍历当前节点的右子节点；
3. 如果当前节点的左子节点不为空，在当前节点的左子树中找到当前节点在中序遍历下的前驱节点；
   3.1. 如果前驱节点的右子节点为空，将前驱节点的右子节点设置为当前节点，当前节点更新为当前节点的左子节点。
   3.2. 如果前驱节点的右子节点为当前节点，将它的右子节点重新设为空。倒序输出从当前节点的左子节点到该前驱节点这条路径上的所有节点。
        当前节点更新为当前节点的右子节点。
4. 重复步骤 2 和步骤 3，直到遍历结束。

这样我们利用 Morris 遍历的方法，后序遍历该二叉搜索树，即可实现线性时间与常数空间的遍历。
*/
class SolutionMorris
{
public:
    void addPath(vector<int> &vec, TreeNode *node)
    {
        int count = 0;
        while (node != nullptr)
        {
            ++count;
            vec.emplace_back(node->val);
            node = node->right;
        }
        reverse(vec.end() - count, vec.end());
    }

    vector<int> postorderTraversal(TreeNode *root)
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
                {
                    p2 = p2->right;
                }
                if (p2->right == nullptr)
                {
                    p2->right = p1;
                    p1 = p1->left;
                    continue;
                }
                else
                {
                    p2->right = nullptr;
                    addPath(res, p1->left);
                }
            }
            p1 = p1->right;
        }
        addPath(res, root);
        return res;
    }
};
