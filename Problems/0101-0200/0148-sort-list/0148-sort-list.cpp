/*
============================================================================
148. 排序链表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-list/
分类：链表 · 归并排序 / 堆排序

给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。

示例 1：
输入：head = [4,2,1,3]
输出：[1,2,3,4]

示例 2：
输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]

示例 3：
输入：head = []
输出：[]

提示：
- 链表中节点的数目在范围 [0, 5 * 10^4] 内
- -10^5 <= Node.val <= 10^5

进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序
吗？
*/
#include <algorithm>
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
解法一：自顶向下归并排序（官方方法一）【非最优】

为什么慢：插入排序在链表上定位插入点要 O(n)，总共 O(n^2)。要达到
O(n log n)，适合链表的是归并排序：合并只改指针，不需要随机访问。

换个角度：快慢指针找中点，把链表切成两半，递归排好左右两半，再合并两个
有序链表。递归深度是 O(log n)，这部分栈空间就是它比解法二多出来的开销。

关键点：
  · fast 从 head->next 起步，偶数长度时 slow 落在左半段末尾，两半长度差
    不超过 1
  · 切分时必须令 slow->next = nullptr，否则递归不终止
  · 递归终止条件是空表或单节点
  · 合并用 dummy 承接，比较用 <= 保持稳定

以示例 1（head = [4,2,1,3]）为例：
  切成 [4,2] 与 [1,3]；再切成 [4]、[2] 与 [1]、[3]
  合并得 [2,4] 与 [1,3]，再合并得 [1,2,3,4]

复杂度分析：
  时间 O(n log n)：每层合并 O(n)，共 O(log n) 层
  空间 O(log n)：递归调用栈；节点本身复用，没有新建链表

解法二：自底向上归并排序（官方方法二）【最优解】

为什么慢：解法一时间已经最优，但递归栈还要 O(log n)。进阶要求常数级空间，
就差在这层栈上。

换个角度：把「递归切半」换成「按固定步长切段」。subLength 从 1 开始，每次
把链表切成若干长度不超过 subLength 的段，两两合并，再让 subLength 翻倍，
直到超过链表长度。整个过程只用了几个游标指针。

关键点：
  · 先把链表长度数出来，否则不知道 subLength 何时停止
  · 每轮用 prev 记住「已合并部分的尾节点」，合并完接回去
  · 切出 head2 后必须把 head1 段尾的 next 置空，否则 merge 会越过段边界
  · 右半段可能不足 subLength，甚至为空；所有推进都要判空
  · 与解法一同为 O(n log n)，但辅助空间从 O(log n) 降到 O(1)，因此是最优解

以示例 1（head = [4,2,1,3]）为例：
  subLength = 1：[4] 与 [2] 合并得 [2,4]，[1] 与 [3] 合并得 [1,3]
  subLength = 2：[2,4] 与 [1,3] 合并得 [1,2,3,4]
  subLength = 4 不小于长度 4，结束

复杂度分析：
  时间 O(n log n)：共 O(log n) 轮，每轮合并 O(n)
  空间 O(1)：只用游标指针，节点原地重连

解法三：堆排序【非最优】

为什么慢：堆排序时间 O(n log n)，与两种归并同阶，本身不慢；慢在空间。
  · 数组上的堆排序可以做到 O(1) 辅助空间，因为下标能直接定位父子节点
  · 链表没有随机访问，必须先遍历一次把节点值收集到数组，才能建堆和下沉；
    这一步就是 O(n) 额外空间
  · 排完后还要把值写回原节点，节点身份不变，只有 val 被改写

换个角度：把问题降维成「对一个长为 n 的数组做堆排序」，再写回链表。

关键点：
  · 建堆从最后一个非叶节点 n/2 - 1 开始向前下沉，才是 O(n) 建堆
  · 每轮把堆顶换到末尾，堆大小减一后再从根下沉
  · 下沉时先比较左右孩子取较大者，再和父节点比较；相等不交换
  · 与解法二同为 O(n log n) 时间，但多出 O(n) 数组，故不是最优解

复杂度分析：
  时间 O(n log n)：建堆 O(n)，n 次取堆顶并下沉共 O(n log n)
  空间 O(n)：values 数组保存全部节点值；链表本身没有额外分配
*/
class Solution1
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

        return merge(sortList(head), sortList(mid));
    }

private:
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

class Solution2
{
public:
    ListNode *sortList(ListNode *head)
    {
        if (head == nullptr) return nullptr;

        int length = 0;
        for (ListNode *p = head; p != nullptr; p = p->next) ++length;

        ListNode dummy(0);
        dummy.next = head;
        for (int subLength = 1; subLength < length; subLength <<= 1)
        {
            ListNode *prev = &dummy;
            ListNode *curr = dummy.next;
            while (curr != nullptr)
            {
                // 切出长度不超过 subLength 的左半段
                ListNode *head1 = curr;
                for (int i = 1; i < subLength && curr->next != nullptr; ++i)
                    curr = curr->next;

                ListNode *head2 = curr->next;
                curr->next = nullptr;
                curr = head2;

                // 再切出长度不超过 subLength 的右半段
                for (int i = 1;
                     i < subLength && curr != nullptr && curr->next != nullptr;
                     ++i)
                    curr = curr->next;

                ListNode *next = nullptr;
                if (curr != nullptr)
                {
                    next = curr->next;
                    curr->next = nullptr;
                }

                prev->next = merge(head1, head2);
                while (prev->next != nullptr) prev = prev->next;
                curr = next;
            }
        }
        return dummy.next;
    }

private:
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

class Solution3
{
public:
    ListNode *sortList(ListNode *head)
    {
        vector<int> values;
        for (ListNode *p = head; p != nullptr; p = p->next)
            values.push_back(p->val);

        int n = static_cast<int>(values.size());
        // 从最后一个非叶节点开始下沉，建大根堆是 O(n)
        for (int i = n / 2 - 1; i >= 0; --i) siftDown(values, i, n);

        // 每轮把堆顶换到末尾，缩小堆后重新下沉
        for (int end = n - 1; end > 0; --end)
        {
            swap(values[0], values[end]);
            siftDown(values, 0, end);
        }

        // 节点顺序不变，只把排好的值写回
        ListNode *p = head;
        for (int v : values)
        {
            p->val = v;
            p = p->next;
        }
        return head;
    }

private:
    static void siftDown(vector<int> &a, int i, int n)
    {
        while (true)
        {
            int left = 2 * i + 1;
            int right = left + 1;
            int largest = i;
            if (left < n && a[left] > a[largest]) largest = left;
            if (right < n && a[right] > a[largest]) largest = right;
            if (largest == i) break;
            swap(a[i], a[largest]);
            i = largest;
        }
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
    a = Solution1().sortList(a);
    assert(toVector(a) == want);
    freeList(a);

    ListNode *b = buildList(input);
    b = Solution2().sortList(b);
    assert(toVector(b) == want);
    freeList(b);

    ListNode *c = buildList(input);
    c = Solution3().sortList(c);
    assert(toVector(c) == want);
    freeList(c);
}
} // namespace

int main()
{
    // 示例 1
    check({4, 2, 1, 3}, {1, 2, 3, 4});

    // 示例 2
    check({-1, 5, 3, 4, 0}, {-1, 0, 3, 4, 5});

    // 示例 3：空链表
    check({}, {});

    // 边界：单节点
    check({7}, {7});

    // 边界：已经有序
    check({1, 2, 3, 4, 5}, {1, 2, 3, 4, 5});

    // 边界：完全逆序
    check({5, 4, 3, 2, 1}, {1, 2, 3, 4, 5});

    // 边界：全部相同，验证合并不会丢掉重复节点
    check({3, 3, 3, 3}, {3, 3, 3, 3});

    // 边界：重复值夹杂在乱序里
    check({2, 1, 2, 1, 2}, {1, 1, 2, 2, 2});

    // 边界：两个节点，验证自底向上时右半段可能为空
    check({2, 1}, {1, 2});

    // 边界：负数与正数混合
    check({0, -100000, 100000, -1, 1}, {-100000, -1, 0, 1, 100000});

    // 边界：奇数长度，验证按长度切分的最后一段不足 subLength
    check({5, 1, 4, 2, 3}, {1, 2, 3, 4, 5});

    // 规模用例：1000 个乱序节点，与 std::sort 的结果对照
    {
        vector<int> input;
        for (int i = 0; i < 1000; ++i) input.push_back((i * 37) % 1000 - 500);
        vector<int> want = input;
        sort(want.begin(), want.end());
        check(input, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
