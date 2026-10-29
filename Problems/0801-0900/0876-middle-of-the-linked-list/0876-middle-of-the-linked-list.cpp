/*
876. 链表的中间结点

给定一个头结点为 head 的非空单链表，返回链表的中间结点。
如果有两个中间结点，则返回第二个中间结点。

示例 1：
输入：[1,2,3,4,5]
输出：此列表中的结点 3 (序列化形式：[3,4,5])
返回的结点值为 3 。 (测评系统对该结点序列化表述是 [3,4,5])。
注意，我们返回了一个 ListNode 类型的对象 ans，这样：
ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, 以及 ans.next.next.next = NULL.

示例 2：
输入：[1,2,3,4,5,6]
输出：此列表中的结点 4 (序列化形式：[4,5,6])
由于该列表有两个中间结点，值分别为 3 和 4，我们返回第二个结点。

提示：
给定链表的结点数介于 1 和 100 之间。

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/middle-of-the-linked-list

思路：
- 方法一：
遍历一遍链表，将 ListNode （包括val和next）存放到数组里，再借助数组的下表返回中间节点。
时间复杂度：O(N)，其中 N 是给定链表中的结点数目。
空间复杂度：O(N)，即数组 A 用去的空间。

- 方法二：
两次遍历。第一次遍历时，我们统计链表中的元素个数 N；第二次遍历时，我们遍历到第 N/2 个元素（链表的首节点为第 0 个元素）时，将该元素返回即可。
时间复杂度：O(N)，其中 N 是给定链表的结点数目。
空间复杂度：O(1)，只需要常数空间存放变量和指针。

- 方法三：快满指针法，推荐
用两个指针 slow 与 fast 一起遍历链表。slow 一次走一步，fast 一次走两步。那么当 fast 到达链表的末尾时，slow 必然位于中间。
时间复杂度：O(N)，其中 N 是给定链表的结点数目。
空间复杂度：O(1)，只需要常数空间存放 slow 和 fast 两个指针。
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
    ListNode *middleNode(ListNode *head)
    {
        ListNode *slow = head, *fast = head;
        while (fast != nullptr && fast->next != nullptr)
        {
            slow = slow->next;
            fast = fast->next->next;
        }
        return slow;
    }
};

int main()
{
    ListNode *head = new ListNode(1);
    ListNode *temp = head;
    for (int i = 2; i < 9; i++)
    {
        ListNode *node = new ListNode(i);
        temp->next = node;
        temp = temp->next;
    }
    temp->next = nullptr;
    Solution s;
    assert(s.middleNode(head)->val == 5);
    // need free list nodes
    return 0;
}