/*
============================================================================
LCR 142 / 剑指 Offer 25. 训练计划 IV（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/he-bing-liang-ge-pai-xu-de-lian-biao-lcof/
分类：链表 · 双路归并（递归 / 迭代）

说明：本题原名「合并两个排序的链表」，leetcode.cn 现已改写成场景题
「训练计划 IV」，函数名为 trainningPlan（官方拼写如此，少一个 i）。
本质与主站 21 题完全一致。
============================================================================

给定两个以 有序链表 形式记录的训练计划 l1、l2，分别记录了两套核心肌群
训练项目编号，请合并这两个训练计划，按训练项目编号 升序 记录于链表并返回。

注意：新链表是通过拼接给定的两个链表的所有节点组成的。

示例 1：
输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]

示例 2：
输入：l1 = [], l2 = []
输出：[]

示例 3：
输入：l1 = [], l2 = [0]
输出：[0]

提示：
0 <= 链表长度 <= 1000

注意：
本题与主站 21 题相同：
https://leetcode.cn/problems/merge-two-sorted-lists/
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
解法一：递归（每次摘掉较小的那个头节点）【非最优】

为什么慢：思路不慢，时间仍是 O(n + m)；慢在空间 —— 每合并一个节点就压一层
调用栈，最坏深度 n + m。本題链长上限 1000 + 1000，递归足够深时有爆栈风险。

换个角度：合并结果的头，一定是 l1 头与 l2 头里较小的那个。取走它之后，
剩下的部分仍然是「两条有序链表合并」—— 问题结构完全没变，只是规模小了一，
于是可以直接写成递归定义。

关键点：
  · 两个空判断必须写在最前面 —— 任一链表为空时，剩余部分整段接上即可，
    不能再比较大小
  · 递归调用前先把自己的 next 指过去，再返回自己 —— 顺序反了会丢掉
    子问题的结果
  · 比较用 < 还是 <= 都能得到合法答案；相等时取哪个由实现自定

以示例 1（l1 = [1,2,4], l2 = [1,3,4]）为例：
  l1 头 1 不小于 l2 头 1 → 取 l2 的 1，问题变成 merge([1,2,4], [3,4])
  再比较 1 与 3 → 取 1，问题变成 merge([2,4], [3,4])
  依此递归到某一侧为空，剩余链表整段接上

复杂度分析：
  时间 O(n + m)：每个节点恰好被取出一次
  空间 O(n + m)：递归深度等于合并后的节点数
*/
class Solution1
{
public:
    ListNode *trainningPlan(ListNode *l1, ListNode *l2)
    {
        // 任一侧走完，剩下那一段整体有序，直接接上
        if (l1 == nullptr)
        {
            return l2;
        }
        if (l2 == nullptr)
        {
            return l1;
        }

        if (l1->val <= l2->val)
        {
            // 先接好子问题的结果，再返回自己；顺序反了会丢掉后半段
            l1->next = trainningPlan(l1->next, l2);
            return l1;
        }

        l2->next = trainningPlan(l1, l2->next);
        return l2;
    }
};

/*
解法二：迭代（哨兵节点 + 尾指针）【最优解】

官方两解中空间最优的一个，也是本题的最优解：时间 O(n + m) 已是下界
（每个节点至少要被接一次），空间压到 O(1)，不占用调用栈。

为什么慢：解法一的额外开销全在调用栈上。只要把「谁接在谁后面」这件事
交给一个循环，栈就不需要了。

换个角度：维护一个尾指针，每轮挑出两个链表头中较小的那个接到尾部，然后把
对应链表往后拨一位。与递归相比，只是把「递归返回时再连」换成了「当场连好」。

关键点：
  · 必须用哨兵（哑节点）—— 否则「结果链表的第一个节点」和「后续节点」
    要写两套赋值逻辑，容易漏掉空结果的分支
  · 尾指针每轮都要前进 —— 漏掉这一步会反复覆盖同一个 next，链表被截断
  · 循环结束后把 tail->next 指向剩余非空链表，而不是再逐个搬运 ——
    两条链表都有序，剩下那段整体都大于已合并的部分

以示例 1（l1 = [1,2,4], l2 = [1,3,4]）为例：
  1 与 1 比较，取 l1 的 1；再取 l2 的 1
  接着 2 与 3 比较取 2，4 与 3 比较取 3，4 与 4 比较取 l1 的 4
  此时 l1 已空，把 l2 剩下的 4 整段接上 → [1,1,2,3,4,4]

复杂度分析：
  时间 O(n + m)：每轮至少推进一条链表，循环次数不超过节点总数
  空间 O(1)：只用哨兵和尾指针两个额外节点
*/
class Solution2
{
public:
    ListNode *trainningPlan(ListNode *l1, ListNode *l2)
    {
        // 哨兵让「第一个节点」和「后续节点」走同一套赋值逻辑
        ListNode dummy(0);
        ListNode *tail = &dummy;

        while (l1 != nullptr && l2 != nullptr)
        {
            // 相等时优先取 l1，只是为了让结果稳定，题目不要求这一点
            if (l1->val <= l2->val)
            {
                tail->next = l1;
                l1 = l1->next;
            }
            else
            {
                tail->next = l2;
                l2 = l2->next;
            }
            // 漏掉这句会反复覆盖 tail->next，链表被截断
            tail = tail->next;
        }

        // 剩下那段整体有序，直接接上即可，不必逐节点搬运
        tail->next = (l1 != nullptr) ? l1 : l2;

        return dummy.next;
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

// 两种解法都直接把输入节点接进结果，所以只能释放结果链表，否则会重复释放
void destroy(ListNode *head)
{
    while (head != nullptr)
    {
        ListNode *next = head->next;
        delete head;
        head = next;
    }
}

// 一个用例喂给两种解法：递归与迭代必须得到同一条链表
void check(const vector<int> &a, const vector<int> &b,
           const vector<int> &want)
{
    Solution1 recursive;
    Solution2 iterative;

    // 解法会消耗输入，所以每个解法各用一份新建的链表
    ListNode *merged1 = recursive.trainningPlan(build(a), build(b));
    ListNode *merged2 = iterative.trainningPlan(build(a), build(b));

    assert(toVector(merged1) == want);
    assert(toVector(merged2) == want);

    destroy(merged1);
    destroy(merged2);
}
} // namespace

int main()
{
    // 示例 1：两链表等长且互相交错
    check({1, 2, 4}, {1, 3, 4}, {1, 1, 2, 3, 4, 4});

    // 示例 2：两侧都是空链表，结果必须为空而不是崩溃
    check({}, {}, {});

    // 示例 3：只有一侧有节点，走「整段接上」分支
    check({}, {0}, {0});

    // 边界：两侧元素完全相等，验证不会漏节点也不会重复
    check({1, 1}, {1, 1}, {1, 1, 1, 1});

    // 边界：一侧整体小于另一侧，验证剩余段是整段拼接
    check({1, 2}, {3, 4, 5}, {1, 2, 3, 4, 5});

    cout << "all tests passed" << endl;
    return 0;
}
