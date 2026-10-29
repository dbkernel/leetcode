/*
============================================================================
105. 从前序与中序遍历序列构造二叉树（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
分类：二叉树 · 分治构造（先序 + 中序）

给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历，
inorder 是同一棵树的中序遍历，请构造二叉树并返回其根节点。

示例 1：
输入: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
输出: [3,9,20,null,null,15,7]

示例 2：
输入: preorder = [-1], inorder = [-1]
输出: [-1]

提示：
- 1 <= preorder.length <= 3000
- inorder.length == preorder.length
- -3000 <= preorder[i], inorder[i] <= 3000
- preorder 和 inorder 均 无重复 元素
- inorder 均出现在 preorder
- preorder 保证 为二叉树的前序遍历序列
- inorder 保证 为二叉树的中序遍历序列
*/
#include <cassert>
#include <climits>
#include <iostream>
#include <stack>
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
解法一：递归分治 + 中序下标哈希表【最优解】

为什么慢：先序的第一个元素就是根，这一步是 O(1)；真正的开销在于必须知道
"根落在中序的哪个位置"，才能把左右两段切出来。若每层递归都线性扫描中序区间
找根，每层花 O(区间长)，链状树有 O(n) 层，累计退化成 O(n^2)。

换个角度：定位根需要的唯一信息是"值 → 中序下标"，而题目保证所有值互不重复。
先用一次遍历把这张表建好，之后每次定位都是 O(1)，整棵树只需 O(n) 次访问。

关键点：
  · 左子树节点数 `leftLen = 中序根下标 - 中序区间左端`，先序与中序两侧必须
    用同一个 leftLen 切分；两处各写一套偏移，错一个就整棵树错位
  · 先序右半段的起点是 `pl + leftLen + 1`，多出的 1 是被根占掉的那一位
  · 递归出口是区间为空（`pl > pr`），不能写成 `pl == pr` 时返回
  · 值域含负数，不能拿值当下标开数组，只能用哈希表
  · 中序区间与先序区间长度始终相等，可用它自检切分有没有写错

以示例 1（preorder = [3,9,20,15,7]，inorder = [9,3,15,20,7]）为例：
  根 3，中序下标 1 → leftLen = 1 - 0 = 1
  左：先序 [9]、中序 [9]，leftLen = 1 - 0 = 0 → 叶子 9
  右：先序 [20,15,7]、中序 [15,20,7]，根 20，leftLen = 2 - 2 = 0
      右半段：先序 [15,7]、中序 [15,7]，根 15，leftLen = 0 → 15 带右孩子 7
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：建表 O(n)，之后每个节点恰被创建一次
  空间 O(n)：哈希表 n 项；递归栈 O(h)，斜链时也是 O(n)

解法二：迭代栈，按先序顺序逐个挂节点【最优解】

为什么慢：解法一每次递归都要压一个栈帧，树退化成链时递归深度等于 n，而
哈希表本身又占 O(n) 空间。时间上并没有更省，只是换取少一张哈希表。

换个角度：先序序列本身就是"一路向左下走"的记录。用栈维护当前这条未完成的
左链，栈顶就是"最深的一个还没在中序里出现过的祖先"。于是只需比较栈顶与中序
当前指针：
  · 不相等：新节点是栈顶的左孩子，压栈继续往左下走
  · 相等：左链到底了，连续弹栈跳过所有已在中序出现过的祖先，新节点挂在最后
    弹出的那个节点的右边

关键点：
  · 比较的是「栈顶的值」与 `inorder[inIdx]`，inIdx 从 0 起只增不减
  · 相等时必须连续弹栈，弹到「不相等或栈空」为止再挂右孩子；只弹一个会把
    剩下的祖先错当父节点
  · 挂右孩子用的是最后弹出的节点 `node`，不是弹完之后的栈顶
  · 先序首元素先建根、先入栈，循环从 i = 1 开始
  · 与解法一并列最优：时间同为 O(n)，但不再需要哈希表，空间从 O(n) 降到
    O(h)；代价是状态推进更绕，建议先读懂解法一再回来看这一版

以示例 1（preorder = [3,9,20,15,7]，inorder = [9,3,15,20,7]）为例：
  i=1，值 9：栈顶 3 ≠ in[0]=9 → 3.left = 9，压栈
  i=2，值 20：栈顶 9 = in[0] → 弹 9、弹 3，inIdx 走到 2；20 挂 3 右边
  i=3，值 15：栈顶 20 ≠ in[2]=15 → 20.left = 15
  i=4，值 7：栈顶 15 = in[2] → 弹 15、弹 20，inIdx 走到 4；7 挂 20 右边
  得到 [3,9,20,null,null,15,7] ✔

复杂度分析：
  时间 O(n)：每个节点进出栈各一次
  空间 O(h)：栈里只保存当前左链，不再需要哈希表
*/
class Solution1
{
public:
    TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder)
    {
        // pos：值 → 中序下标，题目保证无重复值，所以一个值只对应一个位置
        pos.clear();
        for (int i = 0; i < static_cast<int>(inorder.size()); ++i)
        {
            pos[inorder[i]] = i;
        }
        return build(preorder, 0, static_cast<int>(preorder.size()) - 1,
                     0, static_cast<int>(inorder.size()) - 1);
    }

private:
    unordered_map<int, int> pos;

    TreeNode *build(const vector<int> &pre, int pl, int pr, int il, int ir)
    {
        // 空区间才是出口：长度为 1 时仍需建出那个叶子
        if (pl > pr) return nullptr;

        int rootVal = pre[pl];
        int k = pos[rootVal];
        int leftLen = k - il;

        TreeNode *root = new TreeNode(rootVal);
        // 先序左半段是 [pl+1, pl+leftLen]，右半段从 pl+leftLen+1 起
        root->left = build(pre, pl + 1, pl + leftLen, il, k - 1);
        root->right = build(pre, pl + leftLen + 1, pr, k + 1, ir);
        return root;
    }
};

class Solution2
{
public:
    TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder)
    {
        int n = static_cast<int>(preorder.size());
        TreeNode *root = new TreeNode(preorder[0]);

        stack<TreeNode *> stk;
        stk.push(root);
        // inIdx 之后的元素都还没被确认，等于栈顶时说明左链到底
        int inIdx = 0;

        for (int i = 1; i < n; ++i)
        {
            TreeNode *node = stk.top();
            if (node->val != inorder[inIdx])
            {
                // 栈顶尚未在中序出现，先序的这一项只能是它的左孩子
                node->left = new TreeNode(preorder[i]);
                stk.push(node->left);
            }
            else
            {
                // 弹到不相等为止：最后弹出的节点正是新节点的父节点
                while (!stk.empty() && stk.top()->val == inorder[inIdx])
                {
                    node = stk.top();
                    stk.pop();
                    ++inIdx;
                }
                node->right = new TreeNode(preorder[i]);
                stk.push(node->right);
            }
        }
        return root;
    }
};

// ===== 测试辅助：层序序列化（NULL_NODE 表示空）、回收内存 =====
namespace
{
const int NULL_NODE = INT_MIN; // 节点值在 [-3000, 3000]，不会撞上哨兵

vector<int> toLevelOrder(TreeNode *root)
{
    vector<int> out;
    if (root == nullptr) return out;

    // 用队列逐个展开：空节点也入队一次，保证位置不塌陷
    vector<TreeNode *> q = {root};
    for (size_t i = 0; i < q.size(); ++i)
    {
        if (q[i] == nullptr)
        {
            out.push_back(NULL_NODE);
            continue;
        }
        out.push_back(q[i]->val);
        q.push_back(q[i]->left);
        q.push_back(q[i]->right);
    }

    // 末尾的空孩子不改变结构，裁掉才能和题面输出逐项比较
    while (!out.empty() && out.back() == NULL_NODE) out.pop_back();
    return out;
}

void freeTree(TreeNode *root)
{
    if (root == nullptr) return;
    freeTree(root->left);
    freeTree(root->right);
    delete root;
}

// 平台签名收的是非 const 引用，这里按值收再各拷一份，两个解法互不污染
void check(vector<int> preorder, vector<int> inorder,
           const vector<int> &want)
{
    {
        Solution1 s;
        vector<int> pre = preorder;
        vector<int> in = inorder;
        TreeNode *t = s.buildTree(pre, in);
        assert(toLevelOrder(t) == want);
        freeTree(t);
    }
    {
        Solution2 s;
        vector<int> pre = preorder;
        vector<int> in = inorder;
        TreeNode *t = s.buildTree(pre, in);
        assert(toLevelOrder(t) == want);
        freeTree(t);
    }
}
} // namespace

int main()
{
    // 示例 1
    check({3, 9, 20, 15, 7}, {9, 3, 15, 20, 7},
          {3, 9, 20, NULL_NODE, NULL_NODE, 15, 7});

    // 示例 2：单节点，递归与迭代都不该进循环体
    check({-1}, {-1}, {-1});

    // 边界：左斜链，左子树长度取到最大
    check({3, 2, 1}, {1, 2, 3}, {3, 2, NULL_NODE, 1});

    // 边界：右斜链，每层 leftLen 都是 0
    check({1, 2, 3}, {1, 2, 3}, {1, NULL_NODE, 2, NULL_NODE, 3});

    // 关键点用例：左右子树长度不同，切分偏移写错时这里会错位
    check({1, 2, 4, 3}, {4, 2, 3, 1}, {1, 2, NULL_NODE, 4, 3});

    // 满二叉树：每层两侧都非空
    check({1, 2, 4, 5, 3, 6, 7}, {4, 2, 5, 1, 6, 3, 7},
          {1, 2, 3, 4, 5, 6, 7});

    // 边界：含 0 与负数，验证按值定位而不是按下标推算
    check({0, -3000, 3000}, {-3000, 0, 3000}, {0, -3000, 3000});

    // 边界：1000 层左斜链，先序降序而中序升序，验证两侧切分方向没写反
    {
        vector<int> pre;
        vector<int> in;
        for (int v = 1000; v >= 1; --v) pre.push_back(v);
        for (int v = 1; v <= 1000; ++v) in.push_back(v);
        vector<int> want = {1000};
        for (int v = 999; v >= 1; --v)
        {
            want.push_back(v);
            if (v > 1) want.push_back(NULL_NODE);
        }
        check(pre, in, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
