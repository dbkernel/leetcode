/*
590. N 叉树的后序遍历

给定一个 N 叉树，返回其节点值的 后序遍历 。
N 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔（请参见示例）。

进阶：
递归法很简单，你可以使用迭代法完成此题吗?

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[5,6,3,2,4,1]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
输出：[2,6,14,11,7,3,12,8,4,13,9,10,5,1]

提示：
N 叉树的高度小于或等于 1000
节点总数在范围 [0, 10^4] 内

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-postorder-traversal
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
    vector<int> postorder(Node *root)
    {
        vector<int> res;
        postorderRecursive(root, res);
        return res;
    }

    void postorderRecursive(Node *root, vector<int> &res)
    {
        if (root == nullptr)
            return;
        for (auto ch : root->children)
            postorderRecursive(ch, res);
        res.emplace_back(root->val);
    }
};

/*
方法二：迭代

当每次我们从栈顶取出一个节点 u 时，就把 u 的所有子节点顺序推入栈中。例如 u 的子节点从左到右为 v1, v2, v3，
那么推入栈的顺序应当为 v1, v2, v3，这样就保证了下一个遍历到的节点（即 u 的第一个子节点 v3）出现在栈顶的位置。
在遍历结束之后，我们把遍历结果反转，就可以得到后序遍历。

举例说明：

输入：root = [1,null,3,2,4,null,5,6]

程序逻辑：

1 入栈，1出栈，1 放入结果集； stk=[],res=[1]
3、2、4入栈；stk=[3,2,4],res=[1]

4出栈，放入结果集；stk=[3,2],res=[1,4]
4的孩子节点为空，跳过；

2出栈，放入结果集；stk=[3],res=[1,4,2]
2的孩子节点为空，跳过；

3出栈，放入结果集；stk=[],res=[1,4,2,3]
3的孩子节点为5，6，顺序入栈；stk=[5,6],res=[1,4,2,3]

6出栈，放入结果集：stk=[5],res=[1,4,2,3,6]
6的孩子节点为空，跳过；

5出栈，放入结果集；stk=[],res=[1,4,2,3,6,5]
5的孩子节点为空，跳过；

翻转结果集，最终结果集为[5,6,3,2,4,1]

复杂度分析：
- 时间复杂度：时间复杂度：O(M)，其中 MM 是 N 叉树中的节点个数。每个节点只会入栈和出栈各一次。
- 空间复杂度：O(M)。在最坏的情况下，这棵 N 叉树只有 2 层，所有第 2 层的节点都是根节点的孩子。
                  将根节点推出栈后，需要将这些节点都放入栈，共有 M−1 个节点，因此栈的大小为 O(M)。
*/
class SolutionIteration
{
public:
    vector<int> postorder(Node *root)
    {
        vector<int> res;
        if (root == nullptr)
            return res;

        stack<Node *> stk;
        stk.push(root); // 根节点入栈
        while (!stk.empty())
        {
            // 取出当前栈顶节点
            Node *node = stk.top();
            stk.pop();
            res.emplace_back(node->val);
            // 以顺序方式入栈
            for (auto child : node->children)
                stk.emplace(child);
        }
        reverse(res.begin(), res.end()); // 翻转结果集
        return res;
    }
};