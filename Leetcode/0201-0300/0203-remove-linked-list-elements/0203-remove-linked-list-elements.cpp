/*
203. 移除链表元素

给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

示例1：
输入：head = [1,2,6,3,4,5,6], val = 6
输出：[1,2,3,4,5]

示例 2：
输入：head = [], val = 1
输出：[]

示例 3：
输入：head = [7,7,7,7], val = 7
输出：[]
 
提示：
列表中的节点数目在范围 [0, 10^4] 内
1 <= Node.val <= 50
0 <= val <= 50

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/remove-linked-list-elements
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
    ListNode *removeElements(ListNode *head, int val)
    {
        // 为了简化程序，创建一个空的头结点
        ListNode *dummyHead = new ListNode(-1, head);
        ListNode *temp = dummyHead;

        while (temp->next != nullptr)
        {
            if (temp->next->val == val)
            {
                free(temp->next); // 本机执行没问题，leetcode环境会报错
                temp->next = temp->next->next;
            }
            else
            {
                temp = temp->next;
            }
        }
        // 严格意义上说，此处需要添加临时变量用作返回值，并将空的头节点free
        return dummyHead->next;
    }
};

int main()
{
    ListNode *head = new ListNode(6);
    ListNode *temp = head;
    for (int i = 2; i < 9; i++)
    {
        ListNode *node = new ListNode(i);
        temp->next = node;
        temp = temp->next;
    }
    temp->next = nullptr;
    Solution s;
    s.removeElements(head, 6);
    // need free list nodes
    return 0;
}
