/*
25. K 个一组翻转链表

给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
k 是一个正整数，它的值小于或等于链表的长度。
如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。

进阶：
你可以设计一个只使用常数额外空间的算法来解决此问题吗？
你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
空间复杂度：O(1)
时间复杂度：O(n)

示例 1：
输入：head = [1,2,3,4,5], k = 2
输出：[2,1,4,3,5]

示例 2：
输入：head = [1,2,3,4,5], k = 3
输出：[3,2,1,4,5]

示例 3：
输入：head = [1,2,3,4,5], k = 1
输出：[1,2,3,4,5]

示例 4：
输入：head = [1], k = 1
输出：[1]

提示：
列表中节点的数量在范围 sz 内
1 <= sz <= 5000
0 <= Node.val <= 1000
1 <= k <= sz

难点：
在于最后一组节点数小于k，就不反转，因此需要每次都取子链表做翻转

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-nodes-in-k-group

*/
#include <iostream>
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
    ListNode *reverseList(ListNode *head)
    {
        ListNode *dummy = new ListNode(0); // 构造空头节点
        ListNode *cur = head;
        while (cur)
        {
            ListNode *temp = cur->next;
            cur->next = dummy->next;
            dummy->next = cur;
            cur = temp;
        }

        // free dummy node
        ListNode *res = dummy->next;
        delete dummy;

        return res;
    }

    ListNode *reverseKGroup(ListNode *head, int k)
    {
        /*
         输入：head = [1,2,3,4,5], k = 3
         输出：[3,2,1,4,5]
         难点在于最后一组节点数小于k，就不反转，因此需要每次都取子链表做翻转
        */
        ListNode *dummy = new ListNode(0); // 构造空头节点
        ListNode *tail = dummy, *start = head, *end = head;
        int cnt = 1;
        while (end && end->next)
        {
            end = end->next;
            if (++cnt == k)
            {
                ListNode *temp = end->next;
                end->next = nullptr;
                tail->next = reverseList(start);
                start->next = temp;

                cnt = 1;
                tail = start;
                end = temp;
                start = temp;
            }
        }
        // 接上最后一组节点数小于k的子链表
        tail->next = start;

        // free dummy node
        ListNode *res = dummy->next;
        delete dummy;
        return res;
    }
};