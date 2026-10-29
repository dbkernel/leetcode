/*
============================================================================
437. 路径总和 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/path-sum-iii/
分类：二叉树 · 前缀和哈希（路径计数）

给定一个二叉树的根节点 root ，和一个整数 targetSum ，求该二叉树里节点值之和
等于 targetSum 的 路径 的数目。

路径 不需要从根节点开始，也不需要在叶子节点结束，但是路径方向必须是向下的
（只能从父节点到子节点）。

示例 1：
输入：root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
输出：3
解释：和等于 8 的路径有 3 条，如图所示。

示例 2：
输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
输出：3

提示：
- 二叉树的节点个数的范围是 [0,1000]
- -10^9 <= Node.val <= 10^9
- -1000 <= targetSum <= 1000
*/
#include <cassert>
#include <climits>
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
解法一：以每个节点为起点向下枚举【非最优】

为什么慢：路径起点可以是任意节点。直接的做法是让每个节点都当一次起点，
沿子树向下累加并统计命中。思路没错，但同一条边会被它上方的每个祖先各扫一次：
链状树上总代价是 1 + 2 + ... + n = O(n^2)，而节点只有 n 个。

换个角度：把"从任意节点出发"改成"以每个节点为终点"，一段向下路径就可以
写成两个"根到某点前缀和"之差。若当前前缀和为 cur，祖先链上某个前缀和为
cur - targetSum，两者之间那段路径的和恰好是 targetSum。这样每个节点只需
查一次表，不必再向下枚举。

关键点（本解法是枚举，关键点在解法二）：
  · 递归参数带入"从起点累加到当前节点"的 cur，起点一旦固定就只往下走
  · 统计完以 root 为起点的路径后，还要加上完全落在左右子树里的路径，
    三类路径互不重叠
  · 值可达 1e9、节点数 1000，累加和最大 1e12，必须用 long long
  · 值可以为负，cur == targetSum 命中后仍要继续往下走
  · 空树返回 0，空指针是递归的唯一边界

手推一遍：以示例 1（root = [10,5,-3,3,2,null,11,3,-2,null,1]，
targetSum = 8）为例：
  起点 10：10、15、18、17、... 没有命中
  起点 5：5、8（5+3）命中、7、8（5+2+1）命中 → 2 条
  起点 -3：-3、8（-3+11）命中 → 1 条
  其余叶子单独都不等于 8 → 合计 3 条 ✔

复杂度分析：
  时间 O(n^2)：链状树最坏，每个起点都要扫完自己的整棵子树；
    平衡树上为 O(n log n)
  空间 O(h)：递归栈，另有一层按起点的递归嵌套，栈深仍是 O(h)
*/
class Solution1
{
public:
    int pathSum(TreeNode *root, int targetSum)
    {
        if (root == nullptr) return 0;

        // 三类路径互不重叠：以 root 为起点、完全在左子树、完全在右子树
        return countFrom(root, 0, targetSum) +
               pathSum(root->left, targetSum) +
               pathSum(root->right, targetSum);
    }

private:
    // 起点固定为 node，只向下延伸，统计和等于 targetSum 的路径条数
    int countFrom(TreeNode *node, long long cur, int targetSum)
    {
        if (node == nullptr) return 0;

        cur += node->val;
        int hit = (cur == targetSum) ? 1 : 0;

        // 值可正可负，命中之后仍可能接着往下凑出新的 targetSum
        return hit + countFrom(node->left, cur, targetSum) +
               countFrom(node->right, cur, targetSum);
    }
};

/*
解法二：前缀和 + 哈希表回溯【最优解】

为什么慢：解法一对每个起点都重新向下扫，祖先链上同一段和被反复累加。
子树遍历返回后，这些信息被整段丢弃，下一个起点又要从头算。

换个角度：一条向下路径的端点记为 u（上）和 v（下），它的和等于
pref[v] - pref[parent(u)]，其中 pref 是根到该节点的前缀和。DFS 到 v 时，
只要知道祖先链上有多少个节点的前缀和等于 pref[v] - targetSum，就得到
以 v 为终点、起点在 u 处的路径条数。祖先链的前缀和用哈希表边进边记。

关键点：
  · 必须先查再插：若先把当前前缀插入再查询，targetSum = 0 时会把
    "自己减自己"的空路径也算进去（如 [0] 会返回 2 而不是 1）
  · 查的键是 cur - targetSum，方向不能反；前缀和之差才有"区间和"的含义
  · 初始化 cnt[0] = 1，代表空前缀：从根开始的路径靠它才能被数到
  · 退出当前节点前必须把 cnt[cur] 减一（减到 0 就删除），
    否则兄弟分支会看到不属于自己祖先链的前缀和，凭空多算路径
  · 哈希表存的是出现次数而不是布尔值：前缀和可以重复（0 或正负抵消）
  · 值可以为负，cur 不单调，不能排序或用双指针；累加和必须用 long long

手推一遍：仍以示例 1（targetSum = 8）为例，cnt 初始 {0:1}：
  进入 10：cur = 10，查 2 无；插入 10 → {0:1, 10:1}
  进入 5：cur = 15，查 7 无；插入 15
  进入 3：cur = 18，查 10 有 1 条 → 路径 5→3 ✔
  进入 3 的左孩子 3：cur = 21，查 13 无
  进入 -2：cur = 16，查 8 无；回溯到 3
  进入 2：cur = 17，查 9 无
  进入 1：cur = 18，查 10 有 1 条 → 路径 5→2→1 ✔
  进入 -3：cur = 7，查 -1 无；插入 7
  进入 11：cur = 18，查 10 有 1 条 → 路径 -3→11 ✔
  其余分支无命中，合计 3 条 ✔

复杂度分析：
  时间 O(n)：每个节点进出一次，哈希表查询与更新均摊 O(1)
  空间 O(n)：哈希表最多同时保存一条根到叶子的前缀和，最坏（斜链）O(n)
*/
class Solution2
{
public:
    int pathSum(TreeNode *root, int targetSum)
    {
        cnt.clear();
        // 空前缀：让"从根开始"的路径也能被 cur - targetSum = 0 数到
        cnt[0] = 1;
        return dfs(root, 0, targetSum);
    }

private:
    unordered_map<long long, int> cnt;

    int dfs(TreeNode *node, long long cur, int targetSum)
    {
        if (node == nullptr) return 0;

        cur += node->val;

        // 先查：此刻 cnt 只含祖先链上的前缀和，不含 node 自己
        int res = 0;
        auto it = cnt.find(cur - targetSum);
        if (it != cnt.end()) res = it->second;

        // 再插：当前前缀供后代查询使用
        ++cnt[cur];

        res += dfs(node->left, cur, targetSum);
        res += dfs(node->right, cur, targetSum);

        // 回滚：离开 node 后，它不再是任何待访问节点的祖先
        if (--cnt[cur] == 0) cnt.erase(cur);
        return res;
    }
};

// ===== 测试辅助：按层序数组（NULL_NODE 表示空）建树、回收内存 =====
namespace
{
const int NULL_NODE = INT_MAX; // 节点值绝对值不超过 1e9

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

void check(const vector<int> &levelOrder, int targetSum, int want)
{
    Solution1 s1;
    TreeNode *t1 = buildTree(levelOrder);
    assert(s1.pathSum(t1, targetSum) == want);
    freeTree(t1);

    Solution2 s2;
    TreeNode *t2 = buildTree(levelOrder);
    assert(s2.pathSum(t2, targetSum) == want);
    freeTree(t2);
}
} // namespace

int main()
{
    // 示例 1
    check({10, 5, -3, 3, 2, NULL_NODE, 11, 3, -2, NULL_NODE, 1}, 8, 3);

    // 示例 2
    check({5, 4, 8, 11, NULL_NODE, 13, 4, 7, 2, NULL_NODE, NULL_NODE, 5, 1},
          22, 3);

    // 边界：空树
    check({}, 0, 0);

    // 边界：单节点不等于目标和
    check({1}, 0, 0);

    // 关键点：单个 0 只有一条路径；先插后查会错误地数出空路径
    check({0}, 0, 1);

    // 关键点：前缀和重复出现，哈希表必须记次数
    check({0, 0, 0}, 0, 5);

    // 反直觉：正负抵消，1 + (-1) = 0
    check({1, -1}, 0, 1);

    // 反直觉：目标和为负
    check({-1, -2, -3}, -3, 2);

    // 关键点：左分支的前缀和不能在右分支复用（漏回滚会多算）
    check({1, 5, 2}, -3, 0);

    // 边界：兄弟节点各自单独成路
    check({1, 2, 3}, 3, 2);

    // 边界：目标和为 0，只有根自己那条路径命中
    check({0, 1, -1}, 0, 1);

    // 边界：200 层斜链全为 1，目标和 50，共 200-50+1 条
    {
        vector<int> chain = {1};
        for (int i = 2; i <= 200; ++i)
        {
            chain.push_back(NULL_NODE);
            chain.push_back(1);
        }
        check(chain, 50, 151);
    }

    // 边界：值接近 1e9，累加超过 32 位，验证 long long 不溢出
    check({1000000000, 1000000000, NULL_NODE, 1000000000}, 1000000000, 3);

    cout << "all tests passed" << endl;
    return 0;
}
