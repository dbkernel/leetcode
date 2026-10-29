/*
============================================================================
LCR 174 / 剑指 Offer 54. 寻找二叉搜索树中的目标节点（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/
分类：二叉搜索树 · 反序中序遍历

某公司组织架构以二叉搜索树形式记录，节点值为处于该职位的员工编号。
请返回第 cnt 大的员工编号。

示例 1：
输入：root = [7, 3, 9, 1, 5], cnt = 2
       7
      / \
     3   9
    / \
   1   5
输出：7

示例 2：
输入: root = [10, 5, 15, 2, 7, null, 20, 1, null, 6, 8], cnt = 4
       10
      /  \
     5    15
    / \     \
   2   7     20
  /   / \
 1   6   8
输出：8

提示：
- 1 ≤ cnt ≤ 二叉搜索树元素个数
*/
#include <cassert>
#include <climits>
#include <iostream>
#include <stack>
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
解法一：反序中序遍历（右 → 根 → 左）【最优解】

为什么慢：最容易想到的是"中序遍历把整棵树存成升序数组，再取倒数第 cnt 个"，
时间是 O(n) 但要多开 O(n) 的数组；如果先求"总节点数 n，再找第 n-cnt+1 小"，
还得额外遍历一次。

换个角度：BST 的中序是升序，"右 → 根 → 左"的遍历就是降序。按降序数到
第 cnt 个就是答案，数到就可以停，不必访问剩余节点。
  · 递归时先走右子树，再处理当前节点，最后走左子树；
  · 每处理一个节点就 `cnt--`，减到 0 时记下答案并停止。

关键点：
  · 顺序是"右、根、左"，写成"左、根、右"就变成第 cnt 小了
  · 计数必须在访问当前节点时做，不能在进入函数时做
  · 找到答案后要能及时收手：`cnt == 0` 时直接 return，
    否则会继续遍历把 cnt 减成负数
  · 用成员变量/闭包保存答案和计数，函数签名只返回 int
  · 题目保证 1 ≤ cnt ≤ 节点数，因此一定能找到答案

以示例 1（root = [7,3,9,1,5]，cnt = 2）为例：
  右子树 9 → 访问 9，cnt 变 1；
  回到根 7 → 访问 7，cnt 变 0 → 答案是 7 ✔

复杂度分析：
  时间 O(n)：最坏要走到最小节点；平均只需访问前 cnt 大的一小部分
  空间 O(h)：递归栈，h 为树高；最坏（斜链）O(n)

解法二：迭代中序（右根左）【非最优】

把解法一改成显式栈：先一路向右压栈，弹出时计数，再转向左子树。
  · 这样能避免递归，思路与解法一完全一致；
  · 代码稍长，好处是不受递归深度限制（虽然本题数据量也用不上）。

关键点：
  · 压栈方向是"向右"，弹栈时才是"访问"，别把方向写反
  · 计数到 0 即刻 break，栈里剩下的节点不用再管
  · 与解法一相同，顺序必须是右 → 根 → 左

复杂度分析：
  时间 O(n)
  空间 O(h)：栈保存当前路径
*/
class Solution1
{
public:
    int findTargetNode(TreeNode *root, int cnt)
    {
        ans = 0;
        remain = cnt;
        dfs(root);
        return ans;
    }

private:
    int ans;
    int remain;

    void dfs(TreeNode *node)
    {
        if (node == nullptr || remain == 0) return;

        dfs(node->right);

        --remain;
        if (remain == 0)
        {
            ans = node->val;
            return;
        }

        dfs(node->left);
    }
};

class Solution2
{
public:
    int findTargetNode(TreeNode *root, int cnt)
    {
        stack<TreeNode *> stk;
        TreeNode *node = root;

        while (node != nullptr || !stk.empty())
        {
            while (node != nullptr)
            {
                stk.push(node);
                node = node->right;
            }
            node = stk.top();
            stk.pop();

            --cnt;
            if (cnt == 0) return node->val;

            node = node->left;
        }
        return -1; // 题目保证不会走到这里
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = INT_MAX; // 节点值均非负且远小于 INT_MAX

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

void check(const vector<int> &levelOrder, int cnt, int want)
{
    Solution1 s1;
    TreeNode *t1 = buildTree(levelOrder);
    assert(s1.findTargetNode(t1, cnt) == want);
    freeTree(t1);

    Solution2 s2;
    TreeNode *t2 = buildTree(levelOrder);
    assert(s2.findTargetNode(t2, cnt) == want);
    freeTree(t2);
}
} // namespace

int main()
{
    // 示例 1
    check({7, 3, 9, 1, 5}, 2, 7);

    // 示例 2
    check({10, 5, 15, 2, 7, NULL_NODE, 20, 1, NULL_NODE, 6, 8}, 4, 8);

    // 边界：单节点
    check({1}, 1, 1);

    // 剑指 Offer 原示例 1：root = [3,1,4,null,2], k = 1
    check({3, 1, 4, NULL_NODE, 2}, 1, 4);

    // 剑指 Offer 原示例 2：root = [5,3,6,2,4,null,null,1], k = 3
    check({5, 3, 6, 2, 4, NULL_NODE, NULL_NODE, 1}, 3, 4);

    // 边界：取最小值（cnt = 节点总数）
    check({5, 3, 6, 2, 4, NULL_NODE, NULL_NODE, 1}, 6, 1);

    // 边界：cnt = 1 就是最大值
    check({5, 3, 6, 2, 4, NULL_NODE, NULL_NODE, 1}, 1, 6);

    // 边界：只有右孩子的 BST，降序就是 3、2、1
    check({1, NULL_NODE, 2, NULL_NODE, 3}, 2, 2);

    // 边界：只有左孩子的 BST，降序就是 1、2、3... 的最大者
    check({3, 2, NULL_NODE, 1}, 3, 1);

    // 边界：100 层右斜链，值 1..100，第 1 大是 100
    {
        vector<int> chain = {1};
        for (int i = 2; i <= 100; ++i)
        {
            chain.push_back(NULL_NODE);
            chain.push_back(i);
        }
        check(chain, 1, 100);
        check(chain, 100, 1);
        check(chain, 50, 51);
    }

    cout << "all tests passed" << endl;
    return 0;
}
