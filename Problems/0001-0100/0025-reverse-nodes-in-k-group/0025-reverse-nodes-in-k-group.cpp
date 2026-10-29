/*
============================================================================
25. K 个一组翻转链表（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-nodes-in-k-group/
分类：链表 · 分组反转（模拟 + 哑节点）

给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。

k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，
那么请将最后剩余的节点保持原有顺序。

你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。

示例 1：
输入：head = [1,2,3,4,5], k = 2
输出：[2,1,4,3,5]

示例 2：
输入：head = [1,2,3,4,5], k = 3
输出：[3,2,1,4,5]

提示：
链表中的节点数目为 n
1 <= k <= n <= 5000
0 <= Node.val <= 1000

进阶：你可以设计一个只用 O(1) 额外内存空间的算法解决这个问题吗？
*/
#include <cassert>
#include <iostream>
#include <utility>
#include <vector>
using namespace std;

// 平台预置的链表节点定义；本地用 g++ 编译时按同样签名补上
struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

/*
解法：模拟（哑节点 + 按组反转再拼接）【最优解】

官方题解只给出这一种方法，且它满足「进阶」要求的 O(1) 额外空间，因此是本题
的最优解：时间 O(n)（每个节点恰好被翻转一次），空间 O(1)。

为什么慢：最直观的想法是「先求出长度，把节点按组塞进数组，反转每组再重新
连线」——要多开 O(n) 空间。而直接硬写又会掉进两个坑：第一组前面没有前驱节点；
翻转后整个链表的头节点变了，返回值不好确定。

换个角度：那就「创造」一个前驱 —— 在头节点前接一个哑节点 hair。
  · 有了 hair，第一组也有 pre 可以挂接，不必特判；
  · 无论有没有翻转，hair->next 始终是最终结果的头节点，直接返回它即可。

关键点：
  · 每轮先探路：从 pre 出发走 k 步得到 tail，若中途走空说明剩余不足 k 个，
    直接返回 hair->next（剩下的节点保持原序）
  · 反转 [head, tail] 这一段时，prev 的初值要设成 tail->next ——
    这样反转完成后尾部自然接上了后面未处理的部分
  · 反转后原来的 head 变成尾部、tail 变成头部，把 pre->next 指向新头，
    再把新尾的 next 指向刚记录的 nex
  · 收尾更新 pre = 新尾，head = 新尾的 next 进入下一轮
  · k == 1 时每组反转等于不变，逻辑仍然正确（不需要特判）

以示例 1（1 → 2 → 3 → 4 → 5, k = 2）为例：
  第一组探路得 tail = 2，nex = 3；反转 1、2 得 2 → 1，接回得 hair → 2 → 1 → 3...
  第二组探路得 tail = 4，nex = 5；反转 3、4 得 4 → 3，接回 → 2 → 1 → 4 → 3 → 5
  第三组探路时走不到 2 步 → 剩余 5 保持原序
  最终 2 → 1 → 4 → 3 → 5

以示例 2（1 → 2 → 3 → 4 → 5, k = 3）为例：
  第一组 tail = 3，反转得 3 → 2 → 1，接回 → 3 → 2 → 1 → 4 → 5
  第二组只剩 2 个节点，不足 3 个 → 直接返回，结果 3 → 2 → 1 → 4 → 5

复杂度分析：
  时间 O(n)：探路与反转各扫一遍，合计不超过 2n
  空间 O(1)：只用 hair 和几个指针
*/
class Solution
{
public:
    ListNode *reverseKGroup(ListNode *head, int k)
    {
        // 哑节点：既充当第一组的前驱，也保证返回值稳定
        ListNode *hair = new ListNode(0, head);
        ListNode *pre = hair;

        while (head != nullptr)
        {
            // 探路：从 pre 出发走 k 步定为本组的尾节点
            ListNode *tail = pre;
            for (int i = 0; i < k; ++i)
            {
                tail = tail->next;
                if (tail == nullptr)
                {
                    // 剩余不足 k 个，保持原序
                    ListNode *result = hair->next;
                    delete hair;
                    return result;
                }
            }

            // 记下本组之后的第一个节点，反转后要接回去
            ListNode *nex = tail->next;

            // 反转 [head, tail]；prev 从 tail->next 起步，尾部会自动接上 nex
            ListNode *prev = nex;
            ListNode *p = head;
            while (prev != tail)
            {
                ListNode *nxt = p->next;
                p->next = prev;
                prev = p;
                p = nxt;
            }

            // 反转后：tail 是新的头，head 是新的尾
            pre->next = tail;
            pre = head;
            head = nex;
        }

        ListNode *result = hair->next;
        delete hair;
        return result;
    }
};

namespace
{
// 由数组构造链表，方便写断言
ListNode *build(const vector<int> &values)
{
    ListNode dummy(0);
    ListNode *tail = &dummy;
    for (int v : values)
    {
        tail->next = new ListNode(v);
        tail = tail->next;
    }
    return dummy.next;
}

// 按链表顺序展开，用于和期望序列比较
vector<int> toVector(ListNode *head)
{
    vector<int> values;
    for (ListNode *p = head; p != nullptr; p = p->next)
    {
        values.push_back(p->val);
    }
    return values;
}

void destroy(ListNode *head)
{
    while (head != nullptr)
    {
        ListNode *next = head->next;
        delete head;
        head = next;
    }
}

// 一个用例 = 输入序列 + k + 期望序列
void check(const vector<int> &values, int k, const vector<int> &want)
{
    Solution solution;

    ListNode *head = build(values);
    ListNode *got = solution.reverseKGroup(head, k);

    assert(toVector(got) == want);
    destroy(got);
}
} // namespace

int main()
{
    // 示例 1：k = 2，最后剩一个节点保持原序
    check({1, 2, 3, 4, 5}, 2, {2, 1, 4, 3, 5});

    // 示例 2：k = 3，最后剩两个节点保持原序
    check({1, 2, 3, 4, 5}, 3, {3, 2, 1, 4, 5});

    // 边界：k == 1，每组一个节点，结果应与输入完全相同
    check({1, 2, 3}, 1, {1, 2, 3});

    // 边界：k == n，整条链翻转
    check({1, 2, 3, 4, 5}, 5, {5, 4, 3, 2, 1});

    // 边界：单节点，k 只能是 1
    check({7}, 1, {7});

    // 边界：长度恰好整除 k，最后一组也要被翻转
    check({1, 2, 3, 4}, 2, {2, 1, 4, 3});

    // 边界：长度不足 k，整条链保持原序
    check({1, 2}, 3, {1, 2});

    cout << "all tests passed" << endl;
    return 0;
}
