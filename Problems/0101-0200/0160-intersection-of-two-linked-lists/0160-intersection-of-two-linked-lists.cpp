/*
160. 相交链表

给你两个单链表的头节点 headA 和 headB ，请你找出并返回两个单链表相交的起始节点。如果两个链表没有交点，返回 null 。
题目数据 保证 整个链式结构中不存在环。

注意，函数返回结果后，链表必须 保持其原始结构 。

示例 1：
输入：intersectVal = 8, listA = [4,1,8,4,5], listB = [5,0,1,8,4,5], skipA = 2, skipB = 3
输出：Intersected at '8'
解释：相交节点的值为 8 （注意，如果两个链表相交则不能为 0）。
从各自的表头开始算起，链表 A 为 [4,1,8,4,5]，链表 B 为 [5,0,1,8,4,5]。
在 A 中，相交节点前有 2 个节点；在 B 中，相交节点前有 3 个节点。

示例 2：
输入：intersectVal = 2, listA = [0,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1
输出：Intersected at '2'
解释：相交节点的值为 2 （注意，如果两个链表相交则不能为 0）。
从各自的表头开始算起，链表 A 为 [0,9,1,2,4]，链表 B 为 [3,2,4]。
在 A 中，相交节点前有 3 个节点；在 B 中，相交节点前有 1 个节点。

示例 3：
输入：intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2
输出：null
解释：从各自的表头开始算起，链表 A 为 [2,6,4]，链表 B 为 [1,5]。
由于这两个链表不相交，所以 intersectVal 必须为 0，而 skipA 和 skipB 可以是任意值。
这两个链表不相交，因此返回 null 。

提示：
listA 中节点数目为 m
listB 中节点数目为 n
0 <= m, n <= 3 * 10^4
1 <= Node.val <= 10^5
0 <= skipA <= m
0 <= skipB <= n
如果 listA 和 listB 没有交点，intersectVal 为 0
如果 listA 和 listB 有交点，intersectVal == listA[skipA + 1] == listB[skipB + 1]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/intersection-of-two-linked-lists
*/
#include <cassert>
#include <unordered_set>
using namespace std;

// Definition for singly-linked list.
struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

/*
方法一：双指针法

以 listA = [0,9,1,2,4], listB = [3,2,4] 来说明：
由于两个链表在 相交节点 前的节点数量不相同，因此，采用 双指针 法，
分别使这两个指针先遍历自身，再遍历另一个链表，以达到这两个指针遍历相等节点数的目的（m + n），
在遍历到第一个相等节点后，则肯定是 相交节点。

复杂度分析：
- 时间复杂度：O(m+n)，其中 m 和 n 是分别是链表 headA 和 headB 的长度。两个指针同时遍历两个链表，每个指针遍历两个链表各一次。
- 空间复杂度：O(1)。
*/
class Solution
{
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB)
    {
        if (headA == nullptr || headB == nullptr)
            return nullptr;
        ListNode *pA = headA, *pB = headB;
        // 在双指针都遍历 len(A) + len(B) 个节点之后，分别达到两个链表的 nullptr，则可退出循环
        while (pA != pB)
        {
            pA = (pA == nullptr ? headB : pA->next);
            pB = (pB == nullptr ? headA : pB->next);
        }
        return pA;
    }
};

/*
方法二：哈希集合

思路和算法

判断两个链表是否相交，可以使用哈希集合存储链表节点。
首先遍历链表 headA，并将链表 headA 中的每个节点加入哈希集合中。然后遍历链表 headB，对于遍历到的每个节点，判断该节点是否在哈希集合中：
- 如果当前节点不在哈希集合中，则继续遍历下一个节点；
- 如果当前节点在哈希集合中，则后面的节点都在哈希集合中，即从当前节点开始的所有节点都在两个链表的相交部分，
因此在链表 headB 中遍历到的第一个在哈希集合中的节点就是两个链表相交的节点，返回该节点。
如果链表 headB 中的所有节点都不在哈希集合中，则两个链表不相交，返回 null。

复杂度分析：
- 时间复杂度：O(m+n)，其中 m 和 n 是分别是链表 headA 和 headB 的长度。需要遍历两个链表各一次。
- 空间复杂度：O(m)，其中 m 是链表 headA 的长度。需要使用哈希集合存储链表 headA 中的全部节点。
*/
class SolutionHashSet
{
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB)
    {
        unordered_set<ListNode *> visited;
        ListNode *temp = headA;
        while (temp)
        {
            visited.insert(temp);
            temp = temp->next;
        }
        temp = headB;
        while (temp)
        {
            if (visited.count(temp))
                return temp;
            temp = temp->next;
        }
        return nullptr;
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
    ListNode *l2 = createList(5, 12);
    Solution s;
    s.getIntersectionNode(l1, l2);
    // need free list nodes
    return 0;
}