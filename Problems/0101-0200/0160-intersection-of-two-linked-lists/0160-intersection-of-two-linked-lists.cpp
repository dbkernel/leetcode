/*
============================================================================
160. 相交链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/intersection-of-two-linked-lists/
分类：链表 · 双指针（哈希集合 / 走对方的路）

给你两个单链表的头节点 headA 和 headB ，请你找出并返回两个单链表相交的
起始节点。如果两个链表不存在相交节点，返回 null 。

图示两个链表在节点 c1 开始相交。

题目数据 保证 整个链式结构中不存在环。

注意，函数返回结果后，链表必须 保持其原始结构 。

自定义评测：
评测系统 的输入如下（你设计的程序 不适用 此输入）：
- intersectVal - 相交的起始节点的值。如果不存在相交节点，这一值为 0
- listA - 第一个链表
- listB - 第二个链表
- skipA - 在 listA 中（从头节点开始）跳到交叉节点的节点数
- skipB - 在 listB 中（从头节点开始）跳到交叉节点的节点数

评测系统将根据这些输入创建链式数据结构，并将两个头节点 headA 和 headB
传递给你的程序。如果程序能够正确返回相交节点，那么你的解决方案将被
视作正确答案。

示例 1：
输入：intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5],
      skipA = 2, skipB = 3
输出：Intersected at '8'
解释：相交节点的值为 8（注意，如果两个链表相交则不能为 0）。
从各自的表头开始算起，链表 A 为 [4,1,8,4,5]，链表 B 为 [5,6,1,8,4,5]。
在 A 中，相交节点前有 2 个节点；在 B 中，相交节点前有 3 个节点。
— 请注意相交节点的值不为 1，因为在链表 A 和链表 B 之中值为 1 的节点
（A 中第二个节点和 B 中第三个节点）是不同的节点。换句话说，它们在内存中
指向两个不同的位置，而链表 A 和链表 B 中值为 8 的节点（A 中第三个节点，
B 中第四个节点）在内存中指向相同的位置。

示例 2：
输入：intersectVal = 2, listA = [1,9,1,2,4], listB = [3,2,4],
      skipA = 3, skipB = 1
输出：Intersected at '2'

示例 3：
输入：intersectVal = 0, listA = [2,6,4], listB = [1,5],
      skipA = 3, skipB = 2
输出：No intersection

提示：
listA 中节点数目为 m
listB 中节点数目为 n
1 <= m, n <= 3 * 10^4
1 <= Node.val <= 10^5
0 <= skipA <= m
0 <= skipB <= n
如果 listA 和 listB 没有交点，intersectVal 为 0
如果 listA 和 listB 有交点，
intersectVal == listA[skipA] == listB[skipB]

进阶：你能否设计一个时间复杂度 O(m + n)、仅用 O(1) 内存的解决方案？
*/
#include <cassert>
#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

// 平台预置的链表节点定义；本地用 g++ 编译时按同样签名补上
struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

/*
解法一：哈希集合（把 A 的所有节点存起来，再扫 B）【非最优】

为什么慢：时间上已经是 O(m + n)，但额外用了 O(m) 的空间存 A 的全部节点指针。
对 3 万节点的问题不算大，可题目「进阶」明确要求 O(1) 内存，这一解不满足。

换个角度：判断「相交」的本质是判断两个链表有没有同一个节点（按地址比较，
不是按值比较）。既然要快速判断「B 的当前节点在不在 A 里」，最直接的就是
先把 A 的节点放进一个哈希集合，再逐个查 B。

关键点：
  · 比较的是节点地址而不是 val —— 示例 1 里 A、B 都有值为 1 的节点，
    但它们是两个不同对象，按值比较会返回错误的节点
  · 必须先遍历完 A 再遍历 B；边遍历边查会漏掉「B 的某个前驱恰好在 A 里」
    这种（实际上不会发生的）情形，逻辑上也不严谨
  · 不用访问 val，也不修改任何 next，天然满足「保持原始结构」

以示例 1（A = [4,1,8,4,5], B = [5,6,1,8,4,5]，公共部分 [8,4,5]）为例：
  集合里存入 A 的五个节点地址
  扫 B：5 不在 → 6 不在 → 1 不在（B 的这个 1 与 A 的 1 是不同对象）
        → 8 在 → 返回该节点，即公共尾部的第一个节点

复杂度分析：
  时间 O(m + n)：两条链表各扫一遍
  空间 O(m)：哈希集合存 A 的全部节点
*/
class Solution1
{
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB)
    {
        unordered_set<ListNode *> visited;

        for (ListNode *p = headA; p != nullptr; p = p->next)
        {
            visited.insert(p);
        }

        for (ListNode *p = headB; p != nullptr; p = p->next)
        {
            if (visited.count(p) != 0)
            {
                return p;
            }
        }

        return nullptr;
    }
};

/*
解法二：双指针（走完自己走对方的链）【最优解】

第 1 页 进阶要求 O(1) 内存，官方这一解正是为此设计，故标为【最优解】：
时间 O(m + n)，空间 O(1)。

为什么慢：解法一的空间开销可以用「抵消长度差」的思路省掉。麻烦之处在于
不知道两条链谁长、长多少，先求长度差再出发又要多写一层计数。

换个角度：设 A 的独有部分是 a 个节点，B 的独有部分是 b 个节点，公共尾部是
c 个节点。让指针 pA 走完 A 再接着走 B，pB 走完 B 再接着走 A：
  · pA 到达相交点共走 a + c + b 步
  · pB 到达相交点共走 b + c + a 步
两边步数相等，所以它们必然同时到达相交点；若无交点（c = 0），则两者同时
走成空指针。换句话说，「把对方的路也走一遍」自动消掉了长度差。

关键点：
  · 指针为空时才切换到另一条链的头，不能写成「走完本链就切换」以外的逻辑
  · 循环条件是 pA != pB（比较地址），而不是比较 val
  · 无交点时两者最后都变成 nullptr，循环自然结束并返回 nullptr，
    不需要额外判断
  · 题目要求不得修改链表结构，这里只移动指针，符合要求

以示例 2（A = [1,9,1,2,4], B = [3,2,4]，公共 [2,4]）为例：
  a = 3, b = 1, c = 2
  pA 走 A 的 5 个节点后接到 B 头，再走 1 步到相交点，共 6 步
  pB 走 B 的 3 个节点后接到 A 头，再走 3 步到相交点，共 6 步
  第 6 步时两者指向同一个节点 2

复杂度分析：
  时间 O(m + n)：每个指针最多走 m + n 步
  空间 O(1)：只用两个指针
*/
class Solution2
{
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB)
    {
        ListNode *pA = headA;
        ListNode *pB = headB;

        // 走到空就换到对方链表的头部，长度差被自动抵消
        while (pA != pB)
        {
            pA = (pA != nullptr) ? pA->next : headB;
            pB = (pB != nullptr) ? pB->next : headA;
        }

        // 有交点时返回相交节点；无交点时两者同为 nullptr
        return pA;
    }
};

namespace
{
// 统一管理本用例分配出的节点，避免交叉链表重复释放
struct Fixture
{
    vector<ListNode *> owned;

    ListNode *make(int value)
    {
        ListNode *node = new ListNode(value);
        owned.push_back(node);
        return node;
    }

    ~Fixture()
    {
        for (ListNode *node : owned)
        {
            delete node;
        }
    }
};

// 把 values 依次接在 tail 前面，返回新的头节点
ListNode *chain(Fixture &fixture, const vector<int> &values, ListNode *tail)
{
    ListNode *head = tail;
    for (int i = static_cast<int>(values.size()) - 1; i >= 0; --i)
    {
        ListNode *node = fixture.make(values[i]);
        node->next = head;
        head = node;
    }
    return head;
}

// 一个用例：两条链表共享 common 这段尾部，验证返回的必须正是那个共享节点
void check(const vector<int> &prefixA, const vector<int> &prefixB,
           const vector<int> &common)
{
    Fixture fixture;

    // 先建公共尾部，再由它往前接各自的独有前缀
    ListNode *tail = chain(fixture, common, nullptr);
    ListNode *headA = chain(fixture, prefixA, tail);
    ListNode *headB = chain(fixture, prefixB, tail);

    Solution1 byHash;
    Solution2 byTwoPointers;

    ListNode *got1 = byHash.getIntersectionNode(headA, headB);
    ListNode *got2 = byTwoPointers.getIntersectionNode(headA, headB);

    if (common.empty())
    {
        // 无交点必须返回空指针，而不是「值碰巧相等的某个节点」
        assert(got1 == nullptr);
        assert(got2 == nullptr);
    }
    else
    {
        // 必须按地址命中公共尾部的第一个节点
        assert(got1 == tail);
        assert(got2 == tail);
    }
}
} // namespace

int main()
{
    // 示例 1：两链都有值为 1 的节点，但不是同一个节点，只能命中 8
    check({4, 1}, {5, 6, 1}, {8, 4, 5});

    // 示例 2：两链长度差较大（3 对 1），验证长度差被抵消
    check({1, 9, 1}, {3}, {2, 4});

    // 示例 3：完全不相交，两解都必须返回 nullptr
    check({2, 6, 4}, {1, 5}, {});

    // 边界：公共部分只有一个节点，且它正好是两链的尾节点
    check({1}, {2, 3}, {9});

    // 边界：两条链表从头就是同一个节点（前缀都为空）
    check({}, {}, {7, 8});

    // 边界：一侧前缀为空，验证「走完即切换」不会跳过头
    check({}, {1, 4, 6}, {5});

    cout << "all tests passed" << endl;
    return 0;
}
