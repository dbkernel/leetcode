/*
============================================================================
LeetCode 2. 两数相加（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/add-two-numbers/
分类：链表 · 竖式加法（一次遍历）
============================================================================

给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的
方式存储的，并且每个节点只能存储 一位 数字。

请你将两个数相加，并以相同形式返回一个表示和的链表。

你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例 1：
输入：l1 = [2,4,3], l2 = [5,6,4]
输出：[7,0,8]
解释：342 + 465 = 807.

示例 2：
输入：l1 = [0], l2 = [0]
输出：[0]

示例 3：
输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
输出：[8,9,9,9,0,0,0,1]

提示：
每个链表中的节点数在范围 [1, 100] 内
0 <= Node.val <= 9
题目数据保证列表表示的数字不含前导零
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
解法：模拟（竖式加法，一次遍历）

为什么慢：先把两条链表还原成整数、相加、再拆回链表，看起来只扫两遍，
但节点数最多 100 位，远超 64 位整数；要么改用大整数或字符串加法，
要么在转换中反复处理进位，代码和复杂度都不划算。真正需要的信息只有
「同一位置的两位数字 + 来自低位的进位」。

换个角度：链表按逆序存储，表头就是个位，两条链表同一下标天然对齐到
同一位。于是把竖式加法直接搬到链表上：边走边算 n1 + n2 + carry，
当前位留下 sum % 10，进位 sum / 10 留给下一位。

关键点：
  · 进位必须参与当前位求和 —— 漏掉它，5 + 5 会得到 0 而不是 0 进 1
  · 循环条件用 l1 || l2，短链表走完后按 0 参与 —— 否则长链表的高位会丢
  · 循环结束后 carry > 0 要补一个节点 —— 否则 [9,9] + [1] 会少最高位的 1

以示例 1（l1 = [2,4,3], l2 = [5,6,4]，即 342 + 465）为例：
  个位：3 + 4 + 0 = 7，carry = 0 → 答案第 1 位 7
  十位：4 + 6 + 0 = 10，carry = 1 → 答案第 2 位 0
  百位：2 + 5 + 1 = 8，carry = 0 → 答案第 3 位 8
  结果 7 → 0 → 8，即 807

复杂度分析：
  时间 O(max(m,n))：两条链表各走一遍，每位只做常数次运算
  空间 O(1)：除返回值外只用了 head / tail / carry 三个变量
             （返回值本身占 O(max(m,n))，按官方口径不计入）
*/
class Solution
{
public:
    // 返回一条新链表，表示 l1 与 l2 所代表整数之和，不修改输入链表
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2)
    {
        // head 固定答案表头，tail 始终指向答案末尾，省掉每次追加都从头找
        ListNode *head = nullptr;
        ListNode *tail = nullptr;
        int carry = 0;

        // 任一链表还有位就继续：短链表缺的位按 0 处理
        while (l1 != nullptr || l2 != nullptr)
        {
            int n1 = (l1 != nullptr) ? l1->val : 0;
            int n2 = (l2 != nullptr) ? l2->val : 0;

            // 进位来自低位，必须和当前两位一起相加
            int sum = n1 + n2 + carry;

            ListNode *node = new ListNode(sum % 10);
            if (head == nullptr)
            {
                head = tail = node;
            }
            else
            {
                tail->next = node;
                tail = node;
            }

            // 留给下一位；sum < 20，所以 carry 只可能是 0 或 1
            carry = sum / 10;

            // 先判空再前进，否则长度较短的链表会提前越界
            if (l1 != nullptr)
            {
                l1 = l1->next;
            }
            if (l2 != nullptr)
            {
                l2 = l2->next;
            }
        }

        // 最高位仍可能产生进位，例如 [9,9] + [1] = [0,0,1]
        if (carry > 0)
        {
            tail->next = new ListNode(carry);
        }

        return head;
    }
};

namespace
{
// 由数组构造链表，方便写断言；节点所有权交给调用方，测试结束统一释放
ListNode *build(const vector<int> &digits)
{
    ListNode *head = nullptr;
    ListNode *tail = nullptr;
    for (int d : digits)
    {
        ListNode *node = new ListNode(d);
        if (head == nullptr)
        {
            head = tail = node;
        }
        else
        {
            tail->next = node;
            tail = node;
        }
    }
    return head;
}

// 按题目约定的顺序展开链表：表头是个位，表尾是最高位
vector<int> toVector(ListNode *head)
{
    vector<int> digits;
    for (ListNode *p = head; p != nullptr; p = p->next)
    {
        digits.push_back(p->val);
    }
    return digits;
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

// 一个用例 = 两组输入 + 一组期望输出，顺带验证实现没有改写输入链表
void check(const vector<int> &a, const vector<int> &b,
           const vector<int> &want)
{
    Solution s;
    ListNode *l1 = build(a);
    ListNode *l2 = build(b);
    ListNode *sum = s.addTwoNumbers(l1, l2);

    // 官方解法只读输入；若实现原地改写，这两条断言会先挂
    assert(toVector(l1) == a);
    assert(toVector(l2) == b);
    assert(toVector(sum) == want);
    assert(sum != l1 && sum != l2);

    destroy(l1);
    destroy(l2);
    destroy(sum);
}
} // namespace

int main()
{
    // 示例 1：342 + 465 = 807，逐位无跨位进位
    check({2, 4, 3}, {5, 6, 4}, {7, 0, 8});

    // 示例 2：两个 0 相加，答案必须仍是单节点 [0]
    check({0}, {0}, {0});

    // 示例 3：答案比两个输入都长，专门验证末尾补进位
    check({9, 9, 9, 9, 9, 9, 9}, {9, 9, 9, 9},
          {8, 9, 9, 9, 0, 0, 0, 1});

    // 边界：5 + 5 = 10，最短的「进位导致多一位」
    check({5}, {5}, {0, 1});

    // 边界：短链表先走完，长链表剩余位按 0 继续加
    check({1}, {9, 9}, {0, 0, 1});

    cout << "all tests passed" << endl;
    return 0;
}
