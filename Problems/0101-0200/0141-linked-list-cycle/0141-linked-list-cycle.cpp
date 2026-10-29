/*
============================================================================
141. 环形链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/linked-list-cycle/
分类：链表 · 判环（哈希表 / Floyd 快慢指针）

给你一个链表的头节点 head ，判断链表中是否有环。

如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。
为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中
的位置（索引从 0 开始）。注意：pos 不作为参数进行传递。仅仅是为了标识链表
的实际情况。

如果链表中存在环，则返回 true 。 否则，返回 false 。

示例 1：
输入：head = [3,2,0,-4], pos = 1
输出：true
解释：链表中有一个环，其尾部连接到第二个节点。

示例 2：
输入：head = [1,2], pos = 0
输出：true
解释：链表中有一个环，其尾部连接到第一个节点。

示例 3：
输入：head = [1], pos = -1
输出：false
解释：链表中没有环。

提示：
链表中节点的数目范围是 [0, 10^4]
-10^5 <= Node.val <= 10^5
pos 为 -1 或者链表中的一个 有效索引 。

进阶：你能用 O(1)（即，常量）内存解决此问题吗？
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
解法一：哈希表（记录访问过的节点，再次遇到即有环）【非最优】

为什么慢：时间 O(n) 没有问题，问题在空间 O(n) —— 要为每个节点存一份指针。
题目「进阶」明确要求 O(1) 内存，这一解不满足。

换个角度：有环等价于「沿着 next 一直走，永远走不到头」。想快速发现「又回到
老地方」，最直接的做法就是把走过的节点记下来，每次到达时查一下是否访问过。

关键点：
  · 存的是节点地址而不是 val —— 值相同的不同节点很常见，按值判断会误报
  · 判断「是否已存在」要在插入之前做；反过来写也能跑，但语义是
    「插入失败说明重复」，容易写错
  · 空链表直接不进循环，返回 false

以示例 1（head = [3,2,0,-4], pos = 1）为例：
  依次访问 3、2、0、-4，四点都进集合
  从 -4 回到下标 1 的节点 2，发现它已在集合中 → 返回 true

复杂度分析：
  时间 O(n)：每个节点最多访问一次
  空间 O(n)：哈希表存节点指针
*/
class Solution1
{
public:
    bool hasCycle(ListNode *head)
    {
        unordered_set<ListNode *> seen;

        for (ListNode *p = head; p != nullptr; p = p->next)
        {
            if (seen.count(p) != 0)
            {
                return true;
            }
            seen.insert(p);
        }

        return false;
    }
};

/*
解法二：快慢指针（Floyd 判圈）【最优解】

题目「进阶」要求 O(1) 内存，官方这一解正是为此设计，故标为【最优解】：
时间 O(n)，空间 O(1)。

为什么慢：解法一只需一次遍历，时间已经最优；但它必须把每个节点都记下来。
换个角度看，判断「有没有环」其实只需要知道「能不能一直走下去」，
而这个事实可以用两个速度不同的指针来探测。

换个角度：慢指针每步走 1 格，快指针每步走 2 格。
  · 无环：快指针一定先撞到尾节点之后（null），此时立刻返回 false
  · 有环：两者先后进入环，快指针每轮净追 1 格，必定追上慢指针
这个「净追 1 格」就是它能保证相遇的原因。

关键点：
  · 官方写法把 slow 放在 head、fast 放在 head->next，这样 while 循环条件
    「是否相遇」就有意义；若两者都放 head，循环体会一次都不执行
  · fast 的两步跳必须先后判空：先判 fast 为 null，再判 fast->next 为 null，
    顺序反了会解引用空指针
  · 单节点自环（pos = 0）时 head->next 指向自己，不能被开头的
    「head->next == nullptr」误判成无环
  · 空链表直接返回 false

以示例 2（head = [1,2], pos = 0）为例：
  slow = 节点 1，fast = 节点 2
  一轮后 slow = 节点 2，fast 走两步回到节点 2
  两者重合 → 返回 true

以示例 3（head = [1], pos = -1）为例：
  head->next 为 null，直接返回 false

复杂度分析：
  时间 O(n)：慢指针最多绕环一圈，快指针不超过两圈
  空间 O(1)：只用两个指针
*/
class Solution2
{
public:
    bool hasCycle(ListNode *head)
    {
        if (head == nullptr || head->next == nullptr)
        {
            return false;
        }

        // fast 起步领先一步，使得 while 的「是否相遇」条件有意义
        ListNode *slow = head;
        ListNode *fast = head->next;

        while (slow != fast)
        {
            // 先判 fast，再判 fast->next，顺序不能反
            if (fast == nullptr || fast->next == nullptr)
            {
                return false;
            }
            slow = slow->next;
            fast = fast->next->next;
        }

        return true;
    }
};

namespace
{
// 统一管理本用例分配出的节点；链表可能有环，不能靠遍历释放
struct Fixture
{
    vector<ListNode *> owned;

    ~Fixture()
    {
        for (ListNode *node : owned)
        {
            delete node;
        }
    }

    // 按 values 建链，再把尾节点接到下标 pos 的节点上；pos < 0 表示无环
    ListNode *build(const vector<int> &values, int pos)
    {
        ListNode *head = nullptr;
        ListNode *tail = nullptr;

        for (int value : values)
        {
            ListNode *node = new ListNode(value);
            owned.push_back(node);
            if (head == nullptr)
            {
                head = node;
            }
            else
            {
                tail->next = node;
            }
            tail = node;
        }

        if (head != nullptr && pos >= 0)
        {
            tail->next = owned[static_cast<size_t>(pos)];
        }

        return head;
    }
};

// 一个用例喂给两种解法
void check(const vector<int> &values, int pos, bool want)
{
    Fixture fixture;

    // 两个解法都会改动指针变量本身，但不会改链表，可以共用同一份结构
    ListNode *head = fixture.build(values, pos);

    Solution1 byHash;
    Solution2 byTwoPointers;

    assert(byHash.hasCycle(head) == want);
    assert(byTwoPointers.hasCycle(head) == want);
}
} // namespace

int main()
{
    // 示例 1：环的入口在中间
    check({3, 2, 0, -4}, 1, true);

    // 示例 2：环的入口是头节点（整条链是环）
    check({1, 2}, 0, true);

    // 示例 3：单节点无环
    check({1}, -1, false);

    // 边界：空链表，两解都必须直接返回 false
    check({}, -1, false);

    // 边界：单节点自环，验证不会被 head->next == nullptr 提前判死
    check({1}, 0, true);

    // 边界：尾节点自环，环长为 1
    check({1, 2, 3}, 2, true);

    // 边界：多节点无环，验证快指针会正常撞到尾部
    check({1, 2, 3, 4}, -1, false);

    cout << "all tests passed" << endl;
    return 0;
}
