/*
19. 删除链表的倒数第 N 个结点

给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
进阶：你能尝试使用一趟扫描实现吗？

示例 1：
输入：head = [1,2,3,4,5], n = 2
输出：[1,2,3,5]

示例 2：
输入：head = [1], n = 1
输出：[]

示例 3：
输入：head = [1,2], n = 1
输出：[1]
 
提示：
链表中结点的数目为 sz
1 <= sz <= 30
0 <= Node.val <= 100
1 <= n <= sz

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-nth-node-from-end-of-list

思路：
借鉴 [剑指 Offer 22. 链表中倒数第k个节点] 中的思路，使用快慢双指针只需遍历一次。
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
    ListNode *removeNthFromEnd(ListNode *head, int n)
    {
        ListNode *dummy = new ListNode(0, head); // 创建辅助头节点，目的是为了处理链表只有1个或2个节点的情况
        ListNode *slow = dummy, *fast = dummy;
        int i = 0;
        while (fast->next)
        {
            // 保持 slow 与 fast 之间的距离为 n
            if (i++ >= n)
                slow = slow->next;
            fast = fast->next;
        }
        if (slow->next)
            slow->next = slow->next->next;
        ListNode *res = dummy->next;
        delete dummy;
        return res;
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
    s.removeNthFromEnd(l, 3);
    // need free list nodes
    return 0;
}
