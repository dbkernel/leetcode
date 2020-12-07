/*
给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例：
输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807
*/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution
{
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2)
    {
        struct ListNode *l;
        struct ListNode *i, *i1 = l1, *i2 = l2, *n, *rl;
        int res = 0, flag = 0;

        // 设置链表头节点
        l = new ListNode;
        l->next = NULL;
        i = l;

        while (i1 != NULL || i2 != NULL)
        {
            // 决定当前位的和
            if (i1 != NULL && i2 != NULL)
            {
                res = i1->val + i2->val + flag;
            }
            else if (i1 != NULL)
            {
                res = i1->val + flag;
            }
            else if (i2 != NULL)
            {
                res = i2->val + flag;
            }
            flag = res / 10;

            // new新节点
            n = new ListNode(res % 10);
            n->next = NULL;

            // 放入新链表
            i->next = n;
            i = i->next;

            // 右移链表l1和l2
            if (i1 != NULL)
                i1 = i1->next;
            if (i2 != NULL)
                i2 = i2->next;
        }

        // 处理 [5] + [5] = [0, 1] 和 处理 [9,9] + [1] = [0,0,1] 这种情况
        if (flag == 1)
        {
            n = new ListNode(flag);
            n->next = NULL;
            i->next = n;
        }

        return l->next;
    }
};
