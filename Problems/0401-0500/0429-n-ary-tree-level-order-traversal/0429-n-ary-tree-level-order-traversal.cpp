/*
============================================================================
429. N 叉树的层序遍历（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-ary-tree-level-order-traversal/
分类：N 叉树 · 层序遍历（BFS / 队列）

给定一个 N 叉树，返回其节点值的层序遍历。（即从左到右，逐层遍历）。

树的序列化输入是用层序遍历，每组子节点都由 null 值分隔（参见示例）。

示例 1：
输入：root = [1,null,3,2,4,null,5,6]
输出：[[1],[3,2,4],[5,6]]

示例 2：
输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,
             null,null,11,null,12,null,13,null,null,14]
输出：[[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]

提示：
- 树的高度不会超过 1000
- 树的节点总数在 [0, 10^4] 之间
*/
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
解法：BFS + 按层定长出队【最优解】

为什么慢：层序遍历要求「同层从左到右、不同层分开」。如果只用一个普通队列
逐个弹出，虽然节点顺序是对的，但不知道哪几个属于同一层；若给每个节点挂
层号再按层号分组，就要多存 O(n) 个层号。

换个角度：BFS 队列在某一时刻恰好是「当前层剩余节点 + 下一层已入队节点」。
每轮开始先记下 `cnt = q.size()`，再连续弹出 cnt 个节点，这批就是完整的一层；
而这 cnt 个节点入队的孩子，恰好全部是下一层，并且按父节点的顺序排好。

关键点：
  · cnt 必须在 for 循环之前取，循环中队列还会增长，不能写成 `i < q.size()`
  · 每轮新建 level 并 `ans.push_back(move(level))`，避免各层共用同一个数组
  · N 叉树没有「先左后右」，只要按 children 的下标顺序入队，层内自然有序
  · 空树要单独返回：不要入队再靠循环兜底，否则会多出一个空层
  · 树高 1000 也不影响 BFS 的栈空间（队列宽度与深度无关）

以示例 1（root = [1,null,3,2,4,null,5,6]）为例：
  第 1 轮 cnt=1 → 出 1，入孩子 3、2、4 → [[1]]
  第 2 轮 cnt=3 → 出 3（入 5、6）、2、4 → [[1],[3,2,4]]
  第 3 轮 cnt=2 → 出 5、6 → [[1],[3,2,4],[5,6]]

复杂度分析：
  时间 O(m)：m 为节点总数，每个节点入队、出队各一次
  空间 O(w)：w 为最大层宽；除答案外，队列最多同时存两层节点
*/
class Solution
{
public:
    vector<vector<int>> levelOrder(Node *root)
    {
        vector<vector<int>> ans;
        if (root == nullptr) return ans;

        queue<Node *> q;
        q.push(root);
        while (!q.empty())
        {
            // 本层节点个数必须在循环前锁定
            int cnt = (int)q.size();
            vector<int> level;
            level.reserve(cnt);
            for (int i = 0; i < cnt; ++i)
            {
                Node *cur = q.front();
                q.pop();
                level.push_back(cur->val);
                for (Node *child : cur->children) q.push(child);
            }
            ans.push_back(std::move(level));
        }
        return ans;
    }
};

// ===== 测试辅助：按层序数组建 N 叉树（NULL_NODE 表示空） =====
namespace
{
const int NULL_NODE = 10001; // 节点值范围未限定，用足够大的哨兵

// 层序序列规则：根之后先来一个 null，然后是该节点的全部孩子；
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

void check(const vector<int> &levelOrder, const vector<vector<int>> &want)
{
    Solution solution;
    Node *root = buildNaryTree(levelOrder);
    assert(solution.levelOrder(root) == want);
    freeNaryTree(root);
}
} // namespace

int main()
{
    // 示例 1
    check({1, NULL_NODE, 3, 2, 4, NULL_NODE, 5, 6}, {{1}, {3, 2, 4}, {5, 6}});

    // 示例 2
    check({1, NULL_NODE, 2, 3, 4, 5, NULL_NODE, NULL_NODE, 6, 7, NULL_NODE,
           8, NULL_NODE, 9, 10, NULL_NODE, NULL_NODE, 11, NULL_NODE, 12,
           NULL_NODE, 13, NULL_NODE, NULL_NODE, 14},
          {{1}, {2, 3, 4, 5}, {6, 7, 8, 9, 10}, {11, 12, 13}, {14}});

    // 边界：空树
    check({}, {});

    // 边界：单节点
    check({1}, {{1}});

    // 边界：根节点没有孩子
    check({1, NULL_NODE}, {{1}});

    // 边界：星形树，第二层顺序就是孩子数组顺序
    check({1, NULL_NODE, 2, 3, 4, 5, 6}, {{1}, {2, 3, 4, 5, 6}});

    // 边界：某层节点有的有孩子、有的没有（null 占位）
    check({1, NULL_NODE, 2, 3, NULL_NODE, NULL_NODE, 4}, {{1}, {2, 3}, {4}});

    // 边界：深链（每层一个节点），深度 200
    {
        vector<int> levelOrder;
        vector<vector<int>> want;
        for (int i = 1; i <= 200; ++i)
        {
            levelOrder.push_back(i);
            levelOrder.push_back(NULL_NODE);
            want.push_back({i});
        }
        check(levelOrder, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
