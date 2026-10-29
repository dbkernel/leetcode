/*
============================================================================
234. 回文链表（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/palindrome-linked-list/
分类：链表 · 回文判断（复制到数组 / 递归 / 快慢指针 + 反转后半）

给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回
true ；否则，返回 false 。

示例 1：
输入：head = [1,2,2,1]
输出：true

示例 2：
输入：head = [1,2]
输出：false

提示：
链表中节点数目在范围 [1, 10^5] 内
0 <= Node.val <= 9

进阶：你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
*/
#include <cassert>
#include <iostream>
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
解法一：把值复制到数组，再用双指针夹逼【非最优】

为什么慢：链表只能单向访问，没法像数组那样从尾部回看，所以先把值抄进数组，
问题立刻退化成「数组是否回文」。代价是 O(n) 的额外空间，不满足题目「进阶」
要求的 O(1) 空间。

换个角度：判断回文需要「同时从两端向中间走」。数组给了随机访问，天然满足；
链表没这个能力，所以要么先把它「数组化」，要么把后半段物理翻转。

关键点：
  · 复制的是 val，不是节点 —— 比较节点地址永远不相等
  · 双指针用 i < j 而不是 i <= j，奇数长度时中间的节点不需要比较
  · 空链表或单节点直接为真（循环一次都不执行）

以示例 1（[1,2,2,1]）为例：
  数组为 [1,2,2,1]，i=0/j=3 相等，i=1/j=2 相等 → 返回 true

复杂度分析：
  时间 O(n)：复制一遍 + 比较半遍
  空间 O(n)：值数组
*/
class Solution1
{
public:
    bool isPalindrome(ListNode *head)
    {
        vector<int> vals;
        for (ListNode *p = head; p != nullptr; p = p->next)
        {
            vals.push_back(p->val);
        }

        for (int i = 0, j = static_cast<int>(vals.size()) - 1; i < j; ++i, --j)
        {
            if (vals[i] != vals[j])
            {
                return false;
            }
        }

        return true;
    }
};

/*
解法二：递归（借调用栈实现「反向遍历」）【非最优】

为什么慢：递归确实能让我们「先走到尾节点，再在回溯时从后往前比较」，看起来
很省事；但这份便利是用调用栈换来的 —— 空间仍是 O(n)，链长上限 10^5 时
还有爆栈风险。

换个角度：回文比较本质是「第 i 个与第 n-1-i 个相等」。递归正好提供了
「反向取节点」的能力：递归函数先处理 next（向尾部前进），回溯时再拿当前节点
和外部的前向指针 front 比对，然后让 front 前进一步。

关键点：
  · front 必须放在递归函数外面（成员或引用），否则每次回溯拿不到同一份进度
  · 比较要在递归调用「之后」做 —— 放在之前就变成正序和自己比了
  · 一旦子问题返回 false 要立刻短路，别继续推进 front

以示例 2（[1,2]）为例：
  递归到 2 的回溯，front 指向 1，2 != 1 → 返回 false

复杂度分析：
  时间 O(n)：每个节点参与一次比较
  空间 O(n)：递归栈深度等于链长
*/
class Solution2
{
public:
    bool isPalindrome(ListNode *head)
    {
        front = head;
        return check(head);
    }

private:
    ListNode *front = nullptr;

    // 递归到尾部再回溯，回溯时依次与 front 比对
    bool check(ListNode *cur)
    {
        if (cur == nullptr)
        {
            return true;
        }
        if (!check(cur->next))
        {
            return false;
        }
        if (cur->val != front->val)
        {
            return false;
        }
        front = front->next;
        return true;
    }
};

/*
解法三：快慢指针找中点 + 反转后半段【最优解】

满足题目「进阶」的 O(n) 时间、O(1) 空间，是三个官方解中最优的一个。

为什么慢：解法一把值抄进数组，空间 O(n)；解法二用调用栈反向遍历，空间也是
O(n)。想在 O(1) 里完成比较，就得让「从后往前」这件事不再依赖额外容器。

换个角度：回文的后半段读起来正是前半段的镜像。如果把后半段就地翻转，那么
从「头」和「中点之后」同时向后走，逐个比较就能判断回文 —— 空间只用了几个
指针。翻转完成后比较，比较完再把后半段翻回去，输入结构保持不变。

关键点：
  · 找前半段尾节点时，快指针循环条件是 fast->next && fast->next->next，
    这样偶数长度停在左中点、奇数长度停在正中间
  · 反转的是 slow->next 开始的半段，slow 本身留在前半段里
  · 比较循环用 p2 是否走完来控制次数 —— 后半段长度 <= 前半段，
    奇数长度时前半段会多一个正中间的节点，它不需要比较
  · 比较结束后把后半段再反转一次接回去，避免修改输入（官方同样复原）
  · 空链表与单节点要提前返回，否则 endOfFirstHalf 会解引用空 next

以示例 1（1 → 2 → 2 → 1）为例：
  快慢指针停下时 firstHalfEnd 是第 2 个节点
  反转后半段 2 → 1 得到 1 → 2
  p1 走 1、2；p2 走 1、2，全程相等 → true
  再把后半段翻回 2 → 1 并接回，链表恢复原样

复杂度分析：
  时间 O(n)：找中点 n/2、反转 n/2、比较 n/2、复原 n/2，仍是线性
  空间 O(1)：只用几个指针
*/
class Solution3
{
public:
    bool isPalindrome(ListNode *head)
    {
        if (head == nullptr || head->next == nullptr)
        {
            return true;
        }

        // 1. 快慢指针找到前半部分的尾节点
        ListNode *firstHalfEnd = endOfFirstHalf(head);
        // 2. 就地反转后半部分
        ListNode *secondHalfStart = reverseList(firstHalfEnd->next);

        // 3. 从两端向中间比较，p2 走完即结束
        ListNode *p1 = head;
        ListNode *p2 = secondHalfStart;
        bool result = true;
        while (result && p2 != nullptr)
        {
            if (p1->val != p2->val)
            {
                result = false;
            }
            p1 = p1->next;
            p2 = p2->next;
        }

        // 4. 复原后半部分，保持输入结构不变
        firstHalfEnd->next = reverseList(secondHalfStart);

        return result;
    }

private:
    ListNode *endOfFirstHalf(ListNode *head)
    {
        ListNode *fast = head;
        ListNode *slow = head;
        // 停在左中点（偶数）或正中间（奇数）
        while (fast->next != nullptr && fast->next->next != nullptr)
        {
            fast = fast->next->next;
            slow = slow->next;
        }
        return slow;
    }

    ListNode *reverseList(ListNode *head)
    {
        ListNode *prev = nullptr;
        ListNode *curr = head;
        while (curr != nullptr)
        {
            ListNode *next = curr->next;
            curr->next = prev;
            prev = curr;
            curr = next;
        }
        return prev;
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

// 按链表顺序展开，用于核对「复原」是否真的把链表还原了
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

// 一个用例喂给三种解法
void check(const vector<int> &values, bool want)
{
    Solution1 byArray;
    Solution2 byRecursion;
    Solution3 byTwoPointers;

    ListNode *head1 = build(values);
    ListNode *head2 = build(values);
    ListNode *head3 = build(values);

    assert(byArray.isPalindrome(head1) == want);
    assert(byRecursion.isPalindrome(head2) == want);
    assert(byTwoPointers.isPalindrome(head3) == want);
    // 解法三会翻转再复原，验证链表确实回到原样
    assert(toVector(head3) == values);

    destroy(head1);
    destroy(head2);
    destroy(head3);
}
} // namespace

int main()
{
    // 示例 1：偶数长度回文
    check({1, 2, 2, 1}, true);

    // 示例 2：两个不同值，必须在第一对就返回 false
    check({1, 2}, false);

    // 边界：单节点，天然回文（解法三需提前返回）
    check({7}, true);

    // 边界：奇数长度回文，正中间的节点不参与比较
    check({1, 2, 1}, true);

    // 边界：奇数长度非回文
    check({1, 2, 3}, false);

    // 边界：全部相同
    check({1, 1, 1, 1}, true);

    // 边界：只有最外层一对不同，验证比较从首尾开始而不是从中间开始
    check({9, 2, 3, 3, 2, 1}, false);

    // 边界：前半段相同而后半段相反，专防「只比了前半段」
    check({1, 2, 3, 2, 2}, false);

    cout << "all tests passed" << endl;
    return 0;
}
