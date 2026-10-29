/*
============================================================================
LCR 141 / 剑指 Offer 24. 训练计划 III（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/fan-zhuan-lian-biao-lcof/
分类：链表 · 三指针迭代 / 递归

给定一个头节点为 head 的单链表用于记录一系列核心肌群训练编号，请将该
系列训练编号 倒序 记录于链表并返回。

示例 1：
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]

示例 2：
输入：head = [1,2]
输出：[2,1]

示例 3：
输入：head = []
输出：[]

提示：
- 链表中节点的数目范围是 [0, 5000]
- -5000 <= Node.val <= 5000

注意：本题与主站 206 题相同：
https://leetcode.cn/problems/reverse-linked-list/

说明：本题在 leetcode.cn 已改号为 LCR 141，题名由「反转链表」改为
「训练计划 III」，函数签名由 `reverseList(ListNode*)` 改为
`trainningPlan(ListNode*)`（官方 slug 与函数名均保留 trainning 拼写）。
*/
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

// 力扣预置的节点定义
struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

/*
解法一：三指针原地迭代【最优解】

为什么慢：如果新建一条链表做头插，时间仍是 O(n)，但每个节点都要复制
一份，额外空间 O(n)。题目只要求把原来的 next 指针反过来，不需要动节点值。

换个角度：遍历时把当前节点的 next 指向前一个节点。为了断链后还能继续，
需要三个指针：prev（已反转部分的头）、curr（当前节点）、next（尚未处理
部分的头）。

关键点：
  · 必须先保存 curr->next，再改 curr->next；顺序反了，后半条链表就丢了
  · 循环结束时 curr 为空，prev 才是新链表的头，不能返回 head
  · 空链表或单节点时循环不执行/只执行一次，返回 prev 正好是原头

以 1 -> 2 -> 3 为例：
  初始 prev = null，curr = 1
  存 next = 2，1->next = null，prev = 1，curr = 2
  存 next = 3，2->next = 1，prev = 2，curr = 3
  存 next = null，3->next = 2，prev = 3，curr = null
  返回 prev = 3，链表 3 -> 2 -> 1  ✔

复杂度分析：
  时间 O(n)：每个节点改一次 next 指针
  空间 O(1)：只用了三个指针，没有新建节点
*/
class Solution1
{
public:
    ListNode *trainningPlan(ListNode *head)
    {
        ListNode *prev = nullptr;
        ListNode *curr = head;

        while (curr != nullptr)
        {
            // 先记住剩余部分，改完 next 后就找不到它了
            ListNode *next = curr->next;
            curr->next = prev;
            prev = curr;
            curr = next;
        }

        // curr 为空时，prev 指向原链表的尾，也就是新头
        return prev;
    }
};

/*
解法二：递归反转【非最优】

为什么慢：递归写法代码短，但每层调用都要保存当前节点和返回地址，
空间是 O(n) 的调用栈；链表很长时可能爆栈，做不到迭代版本的 O(1)。

换个角度：假设 head->next 之后的链表已经反转好，返回的新头是 newHead。
此时 head->next 是反转后链表的尾节点，让它指回 head，再把 head->next
置空，整条链表就接上了。

关键点：
  · 递归基是空链表或单节点，直接返回 head
  · `head->next->next = head` 必须在 `head->next = nullptr` 之前；
    顺序反了，尾节点就找不到了
  · 必须把 head->next 置空，否则原头和新尾会形成环

以 1 -> 2 -> 3 为例：
  递归到 3，返回 3
  回到 2：2->next->next = 2，即 3->2，2->next = null，返回 3
  回到 1：1->next->next = 1，即 2->1，1->next = null，返回 3
  链表 3 -> 2 -> 1  ✔

复杂度分析：
  时间 O(n)：每个节点做常数次指针操作
  空间 O(n)：递归栈深度等于链表长度
*/
class Solution2
{
public:
    ListNode *trainningPlan(ListNode *head)
    {
        if (head == nullptr || head->next == nullptr) return head;

        ListNode *newHead = trainningPlan(head->next);
        head->next->next = head;
        head->next = nullptr;
        return newHead;
    }
};

namespace
{
// 按数组顺序建链表，返回头节点；空数组返回 nullptr
ListNode *buildList(const vector<int> &values)
{
    ListNode dummy;
    ListNode *tail = &dummy;
    for (int value : values)
    {
        tail->next = new ListNode(value);
        tail = tail->next;
    }
    return dummy.next;
}

vector<int> toVector(ListNode *head)
{
    vector<int> values;
    for (ListNode *node = head; node != nullptr; node = node->next)
    {
        values.push_back(node->val);
    }
    return values;
}

void freeList(ListNode *head)
{
    while (head != nullptr)
    {
        ListNode *next = head->next;
        delete head;
        head = next;
    }
}

// 两个解法各自建一条新链表，反转后按顺序收集再比较
void check(const vector<int> &values, const vector<int> &want)
{
    ListNode *l1 = buildList(values);
    Solution1 s1;
    ListNode *r1 = s1.trainningPlan(l1);
    assert(toVector(r1) == want);
    freeList(r1);

    ListNode *l2 = buildList(values);
    Solution2 s2;
    ListNode *r2 = s2.trainningPlan(l2);
    assert(toVector(r2) == want);
    freeList(r2);
}
} // namespace

int main()
{
    // 示例 1
    check({1, 2, 3, 4, 5}, {5, 4, 3, 2, 1});

    // 示例 2
    check({1, 2}, {2, 1});

    // 示例 3：空链表
    check({}, {});

    // 边界：单节点，反转后还是自己
    check({7}, {7});

    // 边界：负数和零
    check({-5000, 0, 5000}, {5000, 0, -5000});

    // 反直觉：两个节点时，递归版若忘记把 head->next 置空会成环
    check({1, 2}, {2, 1});

    // 边界：最长 5000 个节点，验证递归深度
    {
        vector<int> values;
        vector<int> want;
        values.reserve(5000);
        want.reserve(5000);
        for (int i = 1; i <= 5000; ++i)
        {
            values.push_back(i);
            want.push_back(5001 - i);
        }
        check(values, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
