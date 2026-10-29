/*
剑指 Offer 22. 链表中倒数第k个节点

输入一个链表，输出该链表中倒数第k个节点。为了符合大多数人的习惯，本题从1开始计数，即链表的尾节点是倒数第1个节点。
例如，一个链表有 6 个节点，从头节点开始，它们的值依次是 1、2、3、4、5、6。这个链表的倒数第 3 个节点是值为 4 的节点。

示例：
给定一个链表: 1->2->3->4->5, 和 k = 2.
返回链表 4->5.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof

思路：
与 [ 876. 链表的中间结点（简单）] 思路类似，使用快、慢双指针。
*/
#include <cassert>
using namespace std;

// Definition for singly-linked list.
struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution
{
public:
    ListNode *getKthFromEnd(ListNode *head, int k)
    {
        ListNode *slow = head, *fast = head;
        int i = 0;
        while (fast)
        {
            if (i++ >= k)
                slow = slow->next;
            fast = fast->next;
        }
        return slow;
    }
};

// only for test
ListNode *createList(int start, int end)
{
    ListNode *l = new ListNode(start);
    ListNode *temp = l;
    for (int i = start + 1; i <= end; i++)
    {
        ListNode *node = new ListNode(i);
        temp->next = node;
        temp = temp->next;
    }
    temp->next = nullptr;
    return l;
}

int main()
{
    ListNode *l = createList(1, 8);
    Solution s;
    assert(s.getKthFromEnd(l, 3)->val == 6);
    // need free list nodes
    return 0;
}