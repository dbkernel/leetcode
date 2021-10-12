/*
234. 回文链表

请判断一个链表是否为回文链表。

示例 1:
输入: 1->2
输出: false

示例 2:
输入: 1->2->2->1
输出: true

进阶：
你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/palindrome-linked-list

思路：
可以分为以下五个步骤：
- 找到前半部分链表的尾节点。
- 反转后半部分链表。
- 判断是否回文。
- 恢复链表。
- 返回结果。

- 执行步骤一，我们可以计算链表节点的数量，然后遍历链表找到前半部分的尾节点。
    - 我们也可以使用快慢指针在一次遍历中找到：慢指针一次走一步，快指针一次走两步，快慢指针同时出发。
当快指针移动到链表的末尾时，慢指针恰好到链表的中间。通过慢指针将链表分为两部分。
    - 若链表有奇数个节点，则中间的节点应该看作是前半部分。
- 步骤二可以使用「206. 反转链表」问题中的解决方法来反转链表的后半部分。
- 步骤三比较两个部分的值，当后半部分到达末尾则比较完成，可以忽略计数情况中的中间节点。
- 步骤四与步骤二使用的函数相同，再反转一次恢复链表本身。
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
    bool isPalindrome(ListNode *head)
    {
        if (head == nullptr)
            return true;

        // 找到前半部分的尾节点并翻转后半部分，以 1->2->3->3->2->1 说明
        ListNode *firstHalfEnd = findEndOfFirstHalf(head); // 找到第一个3的位置
        reverseList(firstHalfEnd);                         // 把 firstHalfEnd 当成后半部分链表的空头节点

        // 判断是否是回文链表
        ListNode *p1 = head, *p2 = firstHalfEnd->next;
        bool res = true;
        while (p2)
        {
            if (p1->val != p2->val)
            {
                res = false;
                break;
            }
            p1 = p1->next;
            p2 = p2->next;
        }

        // 将后半部分链表还原
        reverseList(firstHalfEnd);
        return res;
    }

    /*
    查找 1->2->3->3->2->1 中第一个3的位置
    查找 1->2->3->2->1 中3的位置
    */
    ListNode *findEndOfFirstHalf(ListNode *head)
    {
        if (head == nullptr)
            return head;

        // 使用快慢指针
        ListNode *slow = head, *fast = head;
        while (fast->next && fast->next->next)
        {
            slow = slow->next;
            fast = fast->next->next;
        }

        return slow;
    }

    ListNode *reverseList(ListNode *head)
    {
        ListNode *prev = head->next, *cur;
        head->next = nullptr;
        while (prev)
        {
            cur = prev->next;
            prev->next = head->next;
            head->next = prev;
            prev = cur;
        }
        return head;
    }
};

// only for test
ListNode *createList(int start, int end)
{
    ListNode *l = new ListNode(start);
    ListNode *temp = l;
    for (int i = start + 1; i < end; i++)
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
    assert(s.isPalindrome(l) == false);
    // need free list nodes
    return 0;
}
