/*
83. 删除排序链表中的重复元素

存在一个按升序排列的链表，给你这个链表的头节点 head ，请你删除所有重复的元素，使每个元素 只出现一次 。
返回同样按升序排列的结果链表。

示例1：
输入：head = [1,1,2]
输出：[1,2]

示例2：
输入：head = [1,1,2,3,3]
输出：[1,2,3]

提示：
链表中节点数目在范围 [0, 300] 内
-100 <= Node.val <= 100
题目数据保证链表已经按升序排列

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-list/
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
    ListNode *deleteDuplicates(ListNode *head)
    {
        ListNode *cur = head;
        while (cur != nullptr && cur->next != nullptr)
        {
            if (cur->val == cur->next->val)
                cur->next = cur->next->next;
            else
                cur = cur->next;
            // need free duplicate nodes
        }
        return head;
    }
};

int main()
{
    ListNode *head = new ListNode(1);
    ListNode *temp = head;
    for (int i = 2; i < 9; i++)
    {
        ListNode *node = new ListNode((i % 2 == 0) ? i : i - 1);
        temp->next = node;
        temp = temp->next;
    }
    temp->next = nullptr;
    Solution s;
    s.deleteDuplicates(head);
    // need free list nodes
    return 0;
}
