/*
============================================================================
101. 对称二叉树（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/symmetric-tree/
分类：二叉树 · 递归 / 队列

给你一个二叉树的根节点 root ， 检查它是否轴对称。

示例 1：
输入：root = [1,2,2,3,4,4,3]
输出：true

示例 2：
输入：root = [1,2,2,null,3,null,3]
输出：false

提示：
- 树中节点数目在范围 [1, 1000] 内
- -100 <= Node.val <= 100

进阶：你可以运用递归和迭代两种方法解决这个问题吗？
*/
#include <cassert>
#include <climits>
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
解法一：递归（对左右子树做镜像比较）【最优解】

为什么慢：直接的想法是"把树翻转一份，再和原树逐节点比较相等"，能过，
但要多存一整棵树；或者按层序把每层取出来、判回文，也很容易在 null
位置上出错（空位也要参与比较）。

换个角度：轴对称的本质不是"左右子树相同"，而是"左右子树互为镜像"。
于是问题变成：写一个 `mirror(a, b)`，判断 a 与 b 是否镜像：
  · 都空 → true；
  · 一个空一个不空 → false；
  · 值不同 → false；
  · 否则 a 的左要和 b 的右镜像、a 的右要和 b 的左镜像。
  从 `mirror(root->left, root->right)` 出发即可，不需要额外的翻转。

关键点：
  · 比较对象是"左对右"，不是"左对左"，这是全题唯一容易写错的地方
  · 两个为空的检查必须放在取 val 之前，否则空指针解引用
  · 叶子节点：镜像比较返回 (左空、右空) 的与，结果为 true
  · 根节点自身不需要比较，只需要它的左右子树互为镜像
  · 递归深度 = 树高，节点数 ≤ 1000，安全

以示例 1（root = [1,2,2,3,4,4,3]）为例：
  mirror(2左, 2右)：值相等；比较 (3 与 右3)、(4 与 左4) 都成立；
  逐层向内全部满足 → true ✔
以示例 2（root = [1,2,2,null,3,null,3]）为例：
  mirror(2左, 2右)：值相等；2左的右是 3，而 2右的左是 null → false ✔

复杂度分析：
  时间 O(n)：每个节点只参与一次镜像比较
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：迭代（队列成对出队）【非最优】

官方方法二。用一个队列成对存放"应该镜像的两个节点"，初始放
`(root->left, root->right)`：
  · 弹出 (a, b)：都空则继续；一个空则 false；值不同则 false；
  · 依次压入镜像配对：(a->left, b->right)、(a->right, b->left)。
队列空时说明全部配对成功。

关键点：
  · 空节点也要入队（用 nullptr 表示），否则无法判断"一边有孩子一边没有"
  · 入队顺序必须是"左对右、右对左"，不能写成本身
  · `queue<TreeNode*>` 里允许存 nullptr，取出后再判空
  · 空间 O(w)：宽树时队列可能存下整层节点对

复杂度分析：
  时间 O(n)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/
class Solution1
{
public:
    bool isSymmetric(TreeNode *root)
    {
        if (root == nullptr) return true;
        return mirror(root->left, root->right);
    }

private:
    bool mirror(TreeNode *a, TreeNode *b)
    {
        if (a == nullptr && b == nullptr) return true;
        if (a == nullptr || b == nullptr) return false;
        if (a->val != b->val) return false;

        return mirror(a->left, b->right) && mirror(a->right, b->left);
    }
};

class Solution2
{
public:
    bool isSymmetric(TreeNode *root)
    {
        if (root == nullptr) return true;

        queue<TreeNode *> q;
        q.push(root->left);
        q.push(root->right);
        while (!q.empty())
        {
            TreeNode *a = q.front();
            q.pop();
            TreeNode *b = q.front();
            q.pop();

            if (a == nullptr && b == nullptr) continue;
            if (a == nullptr || b == nullptr) return false;
            if (a->val != b->val) return false;

            q.push(a->left);
            q.push(b->right);
            q.push(a->right);
            q.push(b->left);
        }
        return true;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = INT_MAX; // 节点值范围 [-100,100]

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

void check(const vector<int> &levelOrder, bool want)
{
    Solution1 s1;
    TreeNode *t1 = buildTree(levelOrder);
    assert(s1.isSymmetric(t1) == want);
    freeTree(t1);

    Solution2 s2;
    TreeNode *t2 = buildTree(levelOrder);
    assert(s2.isSymmetric(t2) == want);
    freeTree(t2);
}
} // namespace

int main()
{
    // 示例 1
    check({1, 2, 2, 3, 4, 4, 3}, true);

    // 示例 2
    check({1, 2, 2, NULL_NODE, 3, NULL_NODE, 3}, false);

    // 边界：单节点
    check({1}, true);

    // 边界：两个孩子的值不同
    check({1, 2, 3}, false);

    // 边界：两个孩子的值相同
    check({1, 2, 2}, true);

    // 边界：一边有内层孩子，另一边没有
    check({1, 2, 2, 3, NULL_NODE, NULL_NODE, NULL_NODE}, false);

    // 边界：一侧是右孩子、另一侧是左孩子，仍然镜像
    check({1, 2, 2, NULL_NODE, 3, 3, NULL_NODE}, true);

    // 边界：镜像层差 —— 左侧全是 3，右侧只有一个 3
    check({1, 2, 2, 3, 3, 3, NULL_NODE}, false);

    // 边界：完全镜像的四层满二叉树
    check({1, 2, 2, 3, 4, 4, 3, 5, 6, 7, 8, 8, 7, 6, 5}, true);

    // 边界：满二叉树里改坏一个叶子
    check({1, 2, 2, 3, 4, 4, 3, 5, 6, 7, 8, 8, 7, 6, 9}, false);

    // 边界：负数不影响结构
    check({-100, -50, -50, 0, 1, 1, 0}, true);

    cout << "all tests passed" << endl;
    return 0;
}
