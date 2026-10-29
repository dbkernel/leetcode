/*
============================================================================
147. 对链表进行插入排序（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/insertion-sort-list/
分类：链表 · 插入排序 / 归并排序

给定单个链表的头 head ，使用 插入排序 对链表进行排序，并返回 排序后链表
的头 。

插入排序 算法的步骤:

1. 插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的
   输出列表。
2. 每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列
   中适当的位置，并将其插入。
3. 重复直到所有输入数据插入完为止。

下面是插入排序算法的一个图形示例。部分排序的列表(黑色)最初只包含列表中的
第一个元素。每次迭代时，从输入数据中删除一个元素(红色)，并就地插入已排序
的列表中。

对链表进行插入排序。

示例 1：
输入: head = [4,2,1,3]
输出: [1,2,3,4]

示例 2：
输入: head = [-1,5,3,4,0]
输出: [-1,0,3,4,5]

提示：
- 列表中的节点数在 [1, 5000] 范围内
- -5000 <= Node.val <= 5000
*/
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

// 力扣预置的节点定义
struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

/*
解法一：链表插入排序（官方方法，题面指定算法）【最优解】

为什么慢：数组插入排序的代价在「搬移元素」，链表插入只需改指针，插入本身
是 O(1)；但单链表没有前驱指针，每次都要从表头走一遍才能找到插入点，单次
定位 O(n)，n 次插入累计 O(n^2)。输入已经有序时，`lastSorted` 一次比较就能
跳过，退化成 O(n)，但逆序输入每次都要从头扫。

换个角度：把链表分成「已排序段」和「待插入段」。已排序段用 `lastSorted`
标记尾节点，待插入节点是 `lastSorted->next`。如果它不小于尾节点，直接接到
尾部；否则从 dummy 开始找第一个严格大于它的位置插进去。

关键点：
  · 先比较 `lastSorted->val <= curr->val`：相等时留在后面，插入排序才是
    稳定的；写成 `<` 会让相等元素逆序
  · 插入前必须先把 `lastSorted->next` 接到 `curr->next`，否则 curr 原来的
    后继会丢
  · curr 插到 lastSorted 前面时，lastSorted 不动；所以循环末尾统一用
    `curr = lastSorted->next` 取下一个待排节点，而不是 `curr = curr->next`
  · dummy 节点让 curr 可以插到头节点之前，返回 `dummy->next`

以示例 1（head = [4,2,1,3]）为例：
  已排序段 [4]，curr = 2：2 < 4，插到 4 前面 → [2,4]
  curr = 1：1 < 4，插到最前 → [1,2,4]
  curr = 3：3 < 4，插到 4 前面 → [1,2,3,4]

复杂度分析：
  时间 O(n^2)：每插入一个节点最多从头扫描 O(n)
  空间 O(1)：只用了 dummy、lastSorted、curr、prev 几个指针

解法二：自顶向下归并排序【非最优】

为什么慢：解法一的瓶颈是「每次插入都要重新定位」。链表不能随机访问，插入
排序无法用二分把定位降到 O(log n)。

换个角度：既然最终只要求升序，可以放弃插入排序，改用分治：快慢指针把链表
切成两半，递归排好左右两半，再合并两个有序链表。合并只改指针，不需要额外
节点。

关键点：
  · 用 `fast = head->next` 起步，偶数长度时 slow 停在左半段的最后一个节点，
    两半长度差不超过 1
  · 切分时必须令 `slow->next = nullptr`，否则递归不会终止
  · 合并时用 dummy 节点承接，比较用 `<=` 保持稳定
  · 渐近上比插入排序更优，但题面明确要求「使用插入排序」，所以它不算本题
    的解；此处保留只为对照，本题以解法一为准；
    若严格按题意提交，应使用解法一

以示例 1（head = [4,2,1,3]）为例：
  切分 [4,2] 与 [1,3]；各自再切成单节点
  合并 [4] 与 [2] 得 [2,4]；合并 [1] 与 [3] 得 [1,3]
  合并 [2,4] 与 [1,3] 得 [1,2,3,4]

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归栈深度为 log n；节点本身复用，没有新建链表
*/
class Solution1
{
public:
    ListNode *insertionSortList(ListNode *head)
    {
        if (head == nullptr) return nullptr;

        ListNode dummy(0);
        dummy.next = head;
        ListNode *lastSorted = head;
        ListNode *curr = head->next;

        while (curr != nullptr)
        {
            if (lastSorted->val <= curr->val)
            {
                // 已排序段整体不大于 curr，直接把它并入尾部
                lastSorted = lastSorted->next;
            }
            else
            {
                // 从 dummy 起找第一个严格大于 curr 的节点，插在它前面
                ListNode *prev = &dummy;
                while (prev->next->val <= curr->val) prev = prev->next;

                // 先摘下 curr，再接到 prev 后面，顺序反了会丢后继
                lastSorted->next = curr->next;
                curr->next = prev->next;
                prev->next = curr;
            }
            curr = lastSorted->next;
        }
        return dummy.next;
    }
};

class Solution2
{
public:
    ListNode *sortList(ListNode *head)
    {
        if (head == nullptr || head->next == nullptr) return head;

        // fast 从 head->next 起步，偶数长度时 slow 落在左半段末尾
        ListNode *slow = head;
        ListNode *fast = head->next;
        while (fast != nullptr && fast->next != nullptr)
        {
            slow = slow->next;
            fast = fast->next->next;
        }

        ListNode *mid = slow->next;
        slow->next = nullptr;

        ListNode *left = sortList(head);
        ListNode *right = sortList(mid);
        return merge(left, right);
    }

private:
    // 合并两个有序链表，返回新表头；节点全部复用
    ListNode *merge(ListNode *a, ListNode *b)
    {
        ListNode dummy(0);
        ListNode *tail = &dummy;
        while (a != nullptr && b != nullptr)
        {
            if (a->val <= b->val)
            {
                tail->next = a;
                a = a->next;
            }
            else
            {
                tail->next = b;
                b = b->next;
            }
            tail = tail->next;
        }
        tail->next = (a != nullptr) ? a : b;
        return dummy.next;
    }
};

// ===== 测试辅助：数组建表、链表转数组、释放内存 =====
namespace
{
ListNode *buildList(const vector<int> &values)
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

vector<int> toVector(ListNode *head)
{
    vector<int> values;
    for (ListNode *p = head; p != nullptr; p = p->next)
        values.push_back(p->val);
    return values;
}

void freeList(ListNode *head)
{
    while (head != nullptr)
    {
        ListNode *next = head->next;
        delete head;
        head = next;
    }
}

void check(const vector<int> &input, const vector<int> &want)
{
    ListNode *a = buildList(input);
    a = Solution1().insertionSortList(a);
    assert(toVector(a) == want);
    freeList(a);

    ListNode *b = buildList(input);
    b = Solution2().sortList(b);
    assert(toVector(b) == want);
    freeList(b);
}
} // namespace

int main()
{
    // 示例 1
    check({4, 2, 1, 3}, {1, 2, 3, 4});

    // 示例 2
    check({-1, 5, 3, 4, 0}, {-1, 0, 3, 4, 5});

    // 边界：空链表（题面保证至少 1 个节点，这里验证健壮性）
    check({}, {});

    // 边界：单节点
    check({7}, {7});

    // 边界：已经有序，插入排序应只比较尾节点
    check({1, 2, 3, 4, 5}, {1, 2, 3, 4, 5});

    // 边界：完全逆序，每次都要插到最前面
    check({5, 4, 3, 2, 1}, {1, 2, 3, 4, 5});

    // 边界：全部相同，验证稳定性与不会死循环
    check({3, 3, 3, 3}, {3, 3, 3, 3});

    // 边界：重复值夹杂在乱序里
    check({2, 1, 2, 1, 2}, {1, 1, 2, 2, 2});

    // 边界：负数与正数混合
    check({0, -5000, 5000, -1, 1}, {-5000, -1, 0, 1, 5000});

    cout << "all tests passed" << endl;
    return 0;
}
