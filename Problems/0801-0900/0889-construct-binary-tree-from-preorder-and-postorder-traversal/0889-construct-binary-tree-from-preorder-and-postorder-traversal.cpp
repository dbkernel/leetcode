/*
============================================================================
889. 根据前序和后序遍历构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
分类：二叉树 · 分治构造（前序 + 后序，答案不唯一）

给定两个整数数组，preorder 和 postorder ，其中 preorder 是一个具有 无重复
值的二叉树的前序遍历，postorder 是同一棵树的后序遍历，重构并返回二叉树。

如果存在多个答案，您可以返回其中 任何 一个。

示例 1：
输入：preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
输出：[1,2,3,4,5,6,7]

示例 2：
输入: preorder = [1], postorder = [1]
输出: [1]

提示：
- 1 <= preorder.length <= 30
- 1 <= preorder[i] <= preorder.length
- preorder 中所有值都 不同
- postorder.length == preorder.length
- 1 <= postorder[i] <= postorder.length
- postorder 中所有值都 不同
- 保证 preorder 和 postorder 是同一棵二叉树的前序遍历和后序遍历
*/
#include <cassert>
#include <iostream>
#include <unordered_map>
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
解法：递归 + 后序下标哈希表【最优解】

官方题解只给出这一种方法，它同时也就是本题的最优解：每层都必须确定"左子树
占了多长"，而这一步靠查表降到了 O(1)。

为什么慢：前序第一个元素是根、后序最后一个元素也是根，根本身不费事，难在切
分点。左子树的根必然是 `preorder[preL + 1]`，它在后序中的位置决定了左子树的
长度；若每层线性扫描后序区间去找这个位置，链状树上有 O(n) 层，退化成
O(n^2)。

换个角度：`preorder[preL + 1]` 在后序中的下标 p 就是左子树在后序里的终点。
于是左子树节点数 = `p - postL + 1`，一次查表就把前序、后序四个端点全部切出来。

关键点：
  · 前序的根在左端 `preL`，后序的根在右端 `postR`，两侧同时收缩
  · 左子树长度必须用**后序下标**算：`pos[pre[preL + 1]] - postL + 1`，
    用前序下标算会得到同一段长度却对不上边界
  · 递归出口有两个：`preL > preR` 返回空；`preL == preR` 时直接返回叶子，
    否则下一步取 `pre[preL + 1]` 会越界
  · 答案不唯一：某节点只有一个孩子时，前序和后序都无法区分这孩子挂左还是挂
    右，本实现一律当左孩子
  · 正因为不唯一，自测不能断言层序，只能把结果重新前序/后序遍历后与原序列
    逐项比对（两者相等当且仅当它是同一棵树的合法还原）

以示例 1（preorder = [1,2,4,5,3,6,7]，postorder = [4,5,2,6,7,3,1]）为例：
  根 1，左孩子 2 在后序下标 2 → leftSize = 2 - 0 + 1 = 3
  左：前序 [2,4,5]、后序 [4,5,2] → 根 2，左孩子 4 在后序下标 0
      leftSize = 1 → 左 [4]、右 [5]
  右：前序 [3,6,7]、后序 [6,7,3] → 根 3，左孩子 6 在后序下标 3
      leftSize = 1 → 左 [6]、右 [7]
  得到 [1,2,3,4,5,6,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，每个节点只被创建一次
  空间 O(n)：哈希表 n 项；递归栈 O(h)，链状时也是 O(n)
*/
class Solution
{
public:
    TreeNode *constructFromPrePost(vector<int> &preorder,
                                   vector<int> &postorder)
    {
        // pos：值 → 后序下标，用来把「左孩子」翻译成「左子树的长度」
        pos.clear();
        for (int i = 0; i < static_cast<int>(postorder.size()); ++i)
        {
            pos[postorder[i]] = i;
        }
        return build(preorder, postorder,
                     0, static_cast<int>(preorder.size()) - 1,
                     0, static_cast<int>(postorder.size()) - 1);
    }

private:
    unordered_map<int, int> pos;

    TreeNode *build(const vector<int> &pre, const vector<int> &post,
                    int preL, int preR, int postL, int postR)
    {
        if (preL > preR) return nullptr;

        TreeNode *root = new TreeNode(pre[preL]);
        // 只剩一个节点时没有孩子，继续取 pre[preL + 1] 就越界了
        if (preL == preR) return root;

        int leftSize = pos[pre[preL + 1]] - postL + 1;

        root->left = build(pre, post, preL + 1, preL + leftSize,
                           postL, postL + leftSize - 1);
        // 右子树的根在后序的 postR - 1 处，那一位是根，不再划给别人
        root->right = build(pre, post, preL + leftSize + 1, preR,
                            postL + leftSize, postR - 1);
        return root;
    }
};

// ===== 测试辅助：前序/后序遍历、节点计数、回收内存 =====
namespace
{
void preOrder(TreeNode *node, vector<int> &out)
{
    if (node == nullptr) return;
    out.push_back(node->val);
    preOrder(node->left, out);
    preOrder(node->right, out);
}

void postOrder(TreeNode *node, vector<int> &out)
{
    if (node == nullptr) return;
    postOrder(node->left, out);
    postOrder(node->right, out);
    out.push_back(node->val);
}

int countNodes(TreeNode *node)
{
    if (node == nullptr) return 0;
    return 1 + countNodes(node->left) + countNodes(node->right);
}

void freeTree(TreeNode *root)
{
    if (root == nullptr) return;
    freeTree(root->left);
    freeTree(root->right);
    delete root;
}

// 答案不唯一，所以不比对层序：只要求还原出的树能重现原始的两条序列
void check(vector<int> preorder, vector<int> postorder)
{
    Solution s;
    vector<int> pre = preorder;
    vector<int> post = postorder;
    TreeNode *t = s.constructFromPrePost(pre, post);

    vector<int> gotPre;
    vector<int> gotPost;
    preOrder(t, gotPre);
    postOrder(t, gotPost);

    assert(gotPre == preorder);
    assert(gotPost == postorder);
    // 节点数相同才说明没有多建或漏建节点
    assert(countNodes(t) == static_cast<int>(preorder.size()));

    freeTree(t);
}
} // namespace

int main()
{
    // 示例 1
    check({1, 2, 4, 5, 3, 6, 7}, {4, 5, 2, 6, 7, 3, 1});

    // 示例 2：单节点，验证 preL == preR 的出口
    check({1}, {1});

    // 关键点用例：根只有一个左孩子，取 pre[preL + 1] 后必须及时停
    check({1, 2}, {2, 1});

    // 反直觉用例：链 [1,2,3] 与 [3,2,1] 同时对应「全左链」和「全右链」，
    // 这里不校验形状，只要能重现两条序列即可
    check({1, 2, 3}, {3, 2, 1});

    // 不平衡树：3 只有左孩子 6，左子树长度取到最大
    check({1, 2, 4, 5, 3, 6}, {4, 5, 2, 6, 3, 1});

    // 满二叉树 15 个节点，两侧子树长度都不为 0
    check({1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15},
          {8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1});

    // 边界：题面上限 30 个节点的单链
    {
        vector<int> pre;
        vector<int> post;
        for (int v = 1; v <= 30; ++v) pre.push_back(v);
        for (int v = 30; v >= 1; --v) post.push_back(v);
        check(pre, post);
    }

    cout << "all tests passed" << endl;
    return 0;
}
