/*
21. 合并两个有序链表

将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

示例 1：
输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]

示例 2：
输入：l1 = [], l2 = []
输出：[]

示例 3：
输入：l1 = [], l2 = [0]
输出：[0]
 
提示：
两个链表的节点数目范围是 [0, 50]
-100 <= Node.val <= 100
l1 和 l2 均按 非递减顺序 排列

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/merge-two-sorted-lists
*/
#include <cassert>
using namespace std;

// Definition for singly-linked list.
struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution
{
public:
    ListNode *mergeTwoLists(ListNode *l1, ListNode *l2)
    {
        // 带头的链表
        ListNode *head = new ListNode(0);
        ListNode *cur = head;
        while (l1 && l2)
        {
            if (l1->val < l2->val)
            {
                cur->next = l1;
                l1 = l1->next;
            }
            else
            {
                cur->next = l2;
                l2 = l2->next;
            }
            cur = cur->next;
            cur->next = nullptr;
        }
        if (l1)
            cur->next = l1;
        if (l2)
            cur->next = l2;
        // need free head node
        return head->next;
    }
};

// only for test
ListNode *createList(int start, int end)
{
    ListNode *l = new ListNode(start);
    ListNode *temp = l;
    for (int i = start + 1; i < end; i++)
    {
        ListNode *node = new ListNode((i % 2 == 0) ? i : i - 1);
        temp->next = node;
        temp = temp->next;
    }
    temp->next = nullptr;
    return l;
}

int main()
{
    ListNode *l1 = createList(1, 8);
    ListNode *l2 = createList(3, 12);
    Solution s;
    s.mergeTwoLists(l1, l2);
    // need free list nodes
    return 0;
}
