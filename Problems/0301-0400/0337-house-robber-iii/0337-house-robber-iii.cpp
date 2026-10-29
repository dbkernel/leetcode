/*
============================================================================
337. 打家劫舍 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/house-robber-iii/
分类：动态规划 · 树形 DP

小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为 root 。

除了 root 之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果 两个直接相连的房子在同一天晚上被打劫 ，房屋将自动报警。

给定二叉树的 root 。返回 在不触动警报的情况下 ，小偷能够盗取的最高金额 。

示例 1:
输入: root = [3,2,3,null,3,null,1]
输出: 7 
解释: 小偷一晚能够盗取的最高金额 3 + 3 + 1 = 7

示例 2:
输入: root = [3,4,5,1,3,null,1]
输出: 9
解释: 小偷一晚能够盗取的最高金额 4 + 5 = 9

提示：
- 树的节点数在 [1, 10^4] 范围内
- 0 <= Node.val <= 10^4
*/
#include <algorithm>
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
解法一：树上 DP，后序一次返回 (选, 不选)【最优解】

为什么慢：把数组上的递推照搬到树上，最直白的式子是对每个节点比较
"选它 + 孙子们的最优" 和 "两个孩子的最优"。但"两个孩子的最优"本身又要
递归求孙子，同一个子树会在两条分支里各展开一次，节点被重复进入的次数
随深度指数增长。

换个角度：每个节点只需要向父节点汇报两个数——以它为根的子树里，
"选它"的最大值与"不选它"的最大值。父节点拿到孩子的这两个数就能自己
做决定，完全不需要知道子树内部怎么选的。于是后序一次遍历即可。

关键点：
  · 选 x 时孩子只能"不选"：take(x) = x.val + skip(左) + skip(右)
  · 不选 x 时孩子选不选都行：skip(x) = max(take(左), skip(左))
    + max(take(右), skip(右))
  · 返回二元组的两个分量顺序必须在整棵树里统一，父子两侧写反是这类题
    最常见的错，而且小用例未必立刻暴露
  · 空节点返回 (0, 0)：缺失的孩子既不能被选，也不会贡献金额
  · 答案是根节点两个值的 max，不能只看"选根"那一个

以示例 1（root = [3,2,3,null,3,null,1]）为例，从叶子往上：
  val=3 的叶子： (3, 0)
  左孩子 2（右孩子是上面那个 3）：take = 2 + 0 + 0 = 2，
    skip = max(3,0) + 0 = 3 → (2, 3)
  右孩子 3（右孩子是 val=1 的叶子）：take = 3 + 0 + 0 = 3，
    skip = 0 + max(1,0) = 1 → (3, 1)
  根 3：take = 3 + skip(左)=3 + skip(右)=1 = 7；
    skip = max(2,3) + max(3,1) = 6 → max(7,6) = 7 ✔

复杂度分析：
  时间 O(n)：每个节点只被后序访问一次，汇报两个整数
  空间 O(h)：递归栈等于树高，斜树最坏 O(n)，平衡树 O(log n)
*/
class Solution1
{
public:
    int rob(TreeNode *root)
    {
        pair<int, int> best = dfs(root);
        return max(best.first, best.second);
    }

private:
    // 返回 {选当前节点的最大值, 不选当前节点的最大值}
    pair<int, int> dfs(TreeNode *node)
    {
        if (node == nullptr) return {0, 0};

        pair<int, int> left = dfs(node->left);
        pair<int, int> right = dfs(node->right);

        // 选当前节点：两个孩子都只能跳过
        int take = node->val + left.second + right.second;
        // 不选当前节点：每个孩子都可以在"选"与"不选"之间取较优
        int skip = max(left.first, left.second) +
                   max(right.first, right.second);
        return {take, skip};
    }
};

/*
解法二：记忆化递归，按节点缓存它的最优值【非最优】

为什么慢：解法一的式子在"不选 x"时要取孩子的 max，"选 x"时要取孙子的值，
如果直接递归而不缓存，同一个节点的最优值会被它的父、祖父、曾祖父反复
求解，深度为 h 的链上退化成指数级。

换个角度：把"以 x 为根的子树的最优值"当成唯一状态，算过就存。于是与解法
一一样每节点只算一次，但缓存用的是节点地址而不是返回值结构。

关键点：
  · 缓存键必须是节点本身（C++ 里用 TreeNode*），不能用 val——
    值相同的节点是不同子树，混用会算错
  · best(x) 内部必须先查表再展开，查表放在两个递归调用之后等于没缓存
  · 取孙子时要判空：node->left 本身可能为空，再取 ->left 会解引用空指针

复杂度分析：
  时间 O(n)：每个节点的 best 只计算一次
  空间 O(n)：哈希表要给每个节点存一个整数，比解法一多一个数量级的常数；
        加上递归栈 O(h)，整体仍以哈希表为主
*/
class Solution2
{
public:
    int rob(TreeNode *root)
    {
        return best(root);
    }

private:
    // 键是节点地址：同一棵树里地址唯一，值相同也不冲突
    unordered_map<TreeNode *, int> memo;

    int best(TreeNode *node)
    {
        if (node == nullptr) return 0;

        auto it = memo.find(node);
        if (it != memo.end()) return it->second;

        // 不选当前节点
        int skip = best(node->left) + best(node->right);
        // 选当前节点：只能再从孙子们身上取
        int take = node->val;
        if (node->left != nullptr)
            take += best(node->left->left) + best(node->left->right);
        if (node->right != nullptr)
            take += best(node->right->left) + best(node->right->right);

        int ans = max(take, skip);
        memo[node] = ans;
        return ans;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = -1; // 节点值非负，用 -1 表示空位

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

void check(const vector<int> &levelOrder, int want)
{
    Solution1 s1;
    TreeNode *t1 = buildTree(levelOrder);
    assert(s1.rob(t1) == want);
    freeTree(t1);

    Solution2 s2;
    TreeNode *t2 = buildTree(levelOrder);
    assert(s2.rob(t2) == want);
    freeTree(t2);
}
} // namespace

int main()
{
    // 示例 1：3 + 3 + 1，根与右孩子、右孩子的右孩子互不相邻
    check({3, 2, 3, NULL_NODE, 3, NULL_NODE, 1}, 7);

    // 示例 2：4 + 5 优于 3 + 1 + 3 + 1
    check({3, 4, 5, 1, 3, NULL_NODE, 1}, 9);

    // 边界：单节点，必选
    check({1}, 1);

    // 边界：值为 0 的节点，答案是 0 而不是 -1
    check({0}, 0);

    // 关键点用例：隔代能同选，根 5 与两个孙子 4、6 一起拿共 15
    check({5, 1, NULL_NODE, 4, 6}, 15);

    // 反直觉：3 与 4 分属不同分支，中间隔着一层，3+4=7 比 2+4=6 更好
    check({2, 1, 3, NULL_NODE, 4}, 7);

    // 关键点用例：全零树，任何选法都是 0
    check({0, 0, 0, 0, 0}, 0);

    // 边界：右斜链 200 个节点，等价于 198 处理 [1..200]，偶数位之和更大
    {
        vector<int> levelOrder;
        for (int i = 1; i <= 200; ++i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
        }
        check(levelOrder, 10100);
    }

    cout << "all tests passed" << endl;
    return 0;
}
