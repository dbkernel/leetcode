/*
589. N 叉树的前序遍历

给定一个 N 叉树，返回其节点值的 前序遍历 。
N 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔（请参见示例）。

进阶：
递归法很简单，你可以使用迭代法完成此题吗?

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[1,3,5,6,2,4]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
输出：[1,2,3,6,7,11,14,4,8,12,5,9,13,10]
 
提示：
N 叉树的高度小于或等于 1000
节点总数在范围 [0, 10^4] 内

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-preorder-traversal
*/
#include <iostream>
#include <cassert>
#include <vector>
#include <stack>
using namespace std;

// Definition for a Node.
class Node
{
public:
    int val;
    vector<Node *> children;

    Node() {}

    Node(int _val)
    {
        val = _val;
    }

    Node(int _val, vector<Node *> _children)
    {
        val = _val;
        children = _children;
    }
};

/*
方法一：递归
*/
class Solution
{
public:
    vector<int> preorder(Node *root)
    {
        vector<int> res;
        preorderRecursive(root, res);
        return res;
    }

    void preorderRecursive(Node *root, vector<int> &res)
    {
        if (root == nullptr)
            return;
        res.emplace_back(root->val);
        for (auto ch : root->children)
            preorderRecursive(ch, res);
    }
};

/*
方法二：迭代

我们使用栈来帮助我们得到前序遍历，需要保证栈顶的节点就是我们当前遍历到的节点。

我们首先把根节点入栈，因为根节点是前序遍历中的第一个节点。随后每次我们从栈顶取出一个节点 u，它是我们当前遍历到的节点，并把 u 的所有子节点逆序推入栈中。
例如 u 的子节点从左到右为 v1, v2, v3，那么推入栈的顺序应当为 v3, v2, v1，这样就保证了下一个遍历到的节点（即 u 的第一个子节点 v1）出现在栈顶的位置。

复杂度分析：
- 时间复杂度：时间复杂度：O(M)，其中 M 是 N 叉树中的节点个数。每个节点只会入栈和出栈各一次。
- 空间复杂度：O(M)。在最坏的情况下，这棵 N 叉树只有 2 层，所有第 2 层的节点都是根节点的孩子。将根节点推出栈后，需要将这些节点都放入栈，共有 M−1 个节点，因此栈的大小为 O(M)。
*/
class SolutionIteration
{
public:
    vector<int> preorder(Node *root)
    {
        vector<int> res;
        if (root == nullptr)
            return res;

        stack<Node *> stk;
        stk.push(root); // 根节点入栈
        while (!stk.empty())
        {
            // 取出当前栈顶节点，并将其放入结果集
            Node *node = stk.top();
            stk.pop();
            res.emplace_back(node->val);
            // 以逆序方式将所有子节点入栈，这样后续出栈时刚好符合从左到右的顺序，而在下一次循环时会继续找左孩子的子节点。
            for (int i = node->children.size() - 1; i >= 0; i--)
                stk.emplace(node->children[i]);
        }
        return res;
    }
};
