/*
============================================================================
559. N 叉树的最大深度（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-depth-of-n-ary-tree/
分类：N 叉树 · 深度（DFS / BFS）

给定一个 N 叉树，找到其最大深度。

最大深度是指从根节点到最远叶子节点的最长路径上的节点总数。

N 叉树输入按层序遍历序列化表示，每组子节点由空值分隔（请参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：3

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：5

提示：
- 树的深度不会超过 1000 。
- 树的节点数目位于 [0, 10^4] 之间。
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

// 力扣预置的节点定义（多叉树，注意孩子是一个数组）
class Node
{
public:
    int val;
    vector<Node *> children;

    Node() {}

    Node(int _val) { val = _val; }

    Node(int _val, vector<Node *> _children)
    {
        val = _val;
        children = _children;
    }
};

/*
解法一：DFS 后序递归【最优解】

为什么慢：BFS 按层数数也能出答案，但要维护一个队列，空间随最宽的那一层
增长；对 N 叉树来说，一个「星形」根节点就能把队列塞成 O(m)。
而且按层数数还要额外写「每层弹固定个数」的循环，容易写成每弹一个加一次。

换个角度：最大深度天然满足递归结构 —— 空树深度 0；非空树的深度是
`1 + max(每个孩子的深度)`。把「兄弟之间取最大」交给一次循环即可，
递归返回时顺手加一。

关键点：
  · 空节点返回 0，是递归边界，也是叶子返回 1 的来源
  · N 叉树没有「左右」之分，必须在 children 数组里遍历取 max，
    不能只取第一个孩子（那是求某条特定路径，不是最大深度）
  · 孩子在数组中的顺序不影响答案，只影响遍历顺序
  · 递归深度 = 树高 h，题目保证 h ≤ 1000，不会炸栈

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  根 1 的孩子是 3、2、4；
  3 的孩子 5、6 都返回 1 → 3 返回 1 + max(1,1) = 2；
  2、4 无孩子，返回 1；
  根 1 返回 1 + max(2,1,1) = 3 ✔

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点访问一次
  空间 O(h)：递归栈，h 为树高；最坏（一条链）O(m)

解法二：BFS 按层计数【非最优】

官方方法二。层序遍历：每轮先锁定本层节点数 size，弹出 size 个节点并把
它们的孩子入队，然后 depth++；队列空时 depth 就是最大深度。

关键点：
  · size 必须在 for 之前取，循环中队列还会增长
  · 空树直接返回 0，不要靠「入队后再判断」兜底
  · 每层结束才 depth++，不能每弹一个节点就加
  · 空间 O(w)：w 为最大层宽，星形树时约为 m，比 DFS 差

复杂度分析：
  时间 O(m)：每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽
*/
class Solution1
{
public:
    int maxDepth(Node *root)
    {
        if (root == nullptr) return 0;

        int best = 0;
        for (Node *child : root->children)
        {
            best = max(best, maxDepth(child));
        }
        return best + 1;
    }
};

class Solution2
{
public:
    int maxDepth(Node *root)
    {
        if (root == nullptr) return 0;

        int depth = 0;
        queue<Node *> q;
        q.push(root);
        while (!q.empty())
        {
            int size = (int)q.size();
            for (int i = 0; i < size; ++i)
            {
                Node *node = q.front();
                q.pop();
                for (Node *child : node->children) q.push(child);
            }
            ++depth;
        }
        return depth;
    }
};

// ===== 测试辅助：按层序数组建 N 叉树（NULL_NODE 表示空） =====
namespace
{
const int NULL_NODE = 10001; // 节点值范围 [0, 10^4]

// 层序序列的规则：根之后先来一个 null，然后是该节点的全部孩子；
// 每个孩子按 BFS 顺序继续展开，孩子列表以 null 结尾。
Node *buildNaryTree(const vector<int> &levelOrder)
{
    if (levelOrder.empty() || levelOrder[0] == NULL_NODE) return nullptr;

    Node *root = new Node(levelOrder[0]);
    vector<Node *> nodes = {root};
    size_t idx = 2; // 跳过根与它后面的 null
    size_t p = 0;
    while (idx < levelOrder.size() && p < nodes.size())
    {
        Node *parent = nodes[p++];
        while (idx < levelOrder.size() && levelOrder[idx] != NULL_NODE)
        {
            Node *child = new Node(levelOrder[idx]);
            parent->children.push_back(child);
            nodes.push_back(child);
            ++idx;
        }
        ++idx; // 跳过孩子列表的 null 结束符
    }
    return root;
}

void freeNaryTree(Node *root)
{
    if (root == nullptr) return;
    for (Node *child : root->children) freeNaryTree(child);
    delete root;
}

void check(const vector<int> &levelOrder, int want)
{
    Solution1 s1;
    Node *t1 = buildNaryTree(levelOrder);
    assert(s1.maxDepth(t1) == want);
    freeNaryTree(t1);

    Solution2 s2;
    Node *t2 = buildNaryTree(levelOrder);
    assert(s2.maxDepth(t2) == want);
    freeNaryTree(t2);
}
} // namespace

int main()
{
    // 示例 1
    check({1, NULL_NODE, 3, 2, 4, NULL_NODE, 5, 6}, 3);

    // 示例 2
    check({1, NULL_NODE, 2, 3, 4, 5, NULL_NODE, NULL_NODE, 6, 7, NULL_NODE,
           8, NULL_NODE, 9, 10, NULL_NODE, NULL_NODE, 11, NULL_NODE, 12,
           NULL_NODE, 13, NULL_NODE, NULL_NODE, 14},
          5);

    // 边界：空树
    check({}, 0);

    // 边界：单节点
    check({1}, 1);

    // 边界：根没有孩子
    check({1, NULL_NODE}, 1);

    // 边界：星形树，深度 2
    check({1, NULL_NODE, 2, 3, 4, 5, 6}, 2);

    // 边界：不能只取第一个孩子 —— 深度藏在第二个孩子下面
    check({1, NULL_NODE, 2, 3, NULL_NODE, NULL_NODE, 4, NULL_NODE, 5, NULL_NODE}, 4);

    // 边界：孩子顺序不影响答案
    check({1, NULL_NODE, 3, 2, 4, NULL_NODE, 5, 6}, 3);

    // 边界：深链（每个节点只有一个孩子），深度 200
    {
        vector<int> levelOrder;
        for (int i = 1; i <= 200; ++i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
        }
        check(levelOrder, 200);
    }

    cout << "all tests passed" << endl;
    return 0;
}
