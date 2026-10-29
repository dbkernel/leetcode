/*
2. 两数相加

给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例1：
输入：l1 = [2,4,3], l2 = [5,6,4]
输出：[7,0,8]
解释：342 + 465 = 807.

示例2：
输入：l1 = [0], l2 = [0]
输出：[0]

示例3：
输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
输出：[8,9,9,9,0,0,0,1]

提示：
每个链表中的节点数在范围 [1, 100] 内
0 <= Node.val <= 9
题目数据保证列表表示的数字不含前导零

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/add-two-numbers/
*/

#include <cassert>
using namespace std;

// Definition for singly-linked list.
struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution
{
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2)
    {
        ListNode *cur = nullptr, *head = nullptr;
        int carry = 0;
        while (l1 || l2)
        {
            int n1 = l1 ? l1->val : 0;
            int n2 = l2 ? l2->val : 0;
            int sum = n1 + n2 + carry; // 计算总和时必须要加进位
            ListNode *node = new ListNode(sum % 10);
            if (head == nullptr)
                head = cur = node;
            else
            {
                cur->next = node;
                cur = cur->next;
            }
            carry = sum / 10;
            if (l1)
                l1 = l1->next;
            if (l2)
                l2 = l2->next;
        }
        // 适配 [9,9,9,9] + [9,9] 最后的进位
        if (carry > 0)
            cur->next = new ListNode(carry);
        return head;
    }
};

int main()
{
    ListNode *l1 = new ListNode(1);
    ListNode *temp = l1;
    for (int i = 2; i < 9; i++)
    {
        ListNode *node = new ListNode((i % 2 == 0) ? i : i - 1);
        temp->next = node;
        temp = temp->next;
    }
    temp->next = nullptr;

    ListNode *l2 = new ListNode(1);
    temp = l2;
    for (int i = 2; i < 9; i++)
    {
        ListNode *node = new ListNode((i % 2 == 0) ? i : i - 1);
        temp->next = node;
        temp = temp->next;
    }
    temp->next = nullptr;

    Solution s;
    s.addTwoNumbers(l1, l2);
    // need free list nodes
    return 0;
}