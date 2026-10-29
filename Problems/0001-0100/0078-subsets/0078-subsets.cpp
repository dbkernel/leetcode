/*
============================================================================
78. 子集（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/subsets/
分类：回溯 · 幂集枚举（回溯 / 位运算）

给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集
（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。

示例 1：
输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

示例 2：
输入：nums = [0]
输出：[[],[0]]

提示：
- 1 <= nums.length <= 10
- -10 <= nums[i] <= 10
- nums 中的所有元素 互不相同
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：回溯，树的每个节点都收集【最优解】

为什么慢：幂集有 2^n 个子集，答案本身平均每个长度 n/2，光把它们写出去就要
Ω(2^n·n)，所以这里不存在"更快的算法"，只存在"每个子集构造得够不够省"。
朴素写法常见的翻车点是"只在叶子处收集"—— 那样只能拿到一种长度的子集。

换个角度：把"取不取 nums[i]"看成一条按下标递增的决策链。链上的每一个时刻
（包括一个都没取的时候）都对应一个合法子集，所以不必等到走完 n 层才收，
每进入一个节点就收一次。

关键点：
  · `res.push_back(path)` 写在 dfs 的第一行，不是写在循环之后也不是写在
    叶子判断里 —— 根节点对应空集，只收叶子会丢掉所有真子集
  · 循环从 start 开始、递归传 i + 1，按下标递增取数，保证每个元素最多出现
    一次，也就保证 2^n 个子集互不相同
  · 不需要额外的去重，因为题面保证 nums 中元素互不相同

手推一遍：
以示例 1（nums = [1,2,3]）为例：
  进入根节点，收集 []
  取 1 → 收集 [1]；再取 2 → 收集 [1,2]；再取 3 → 收集 [1,2,3]
  回到 [1]，取 3 → 收集 [1,3]
  回到根，取 2 → 收集 [2]；再取 3 → [2,3]
  回到根，取 3 → 收集 [3]
  共 8 个 = 2^3 ✔

复杂度分析：
  时间 O(2^n · n)：2^n 个节点各收集一次，每次拷贝当前 path，长度最多 n
  空间 O(n)：递归栈与 path 都不超过 n 层；答案数组不计入
*/
class Solution1
{
public:
    vector<vector<int>> subsets(vector<int> &nums)
    {
        vector<vector<int>> res;
        vector<int> path;
        backtrace(nums, 0, path, res);
        return res;
    }

private:
    void backtrace(const vector<int> &nums, size_t start, vector<int> &path,
                   vector<vector<int>> &res)
    {
        // 进入节点就先收：当前 path 本身已经是一个合法子集
        res.push_back(path);

        for (size_t i = start; i < nums.size(); ++i)
        {
            path.push_back(nums[i]);
            // 传 i + 1：同一层之后的数只能取更大的下标，避免重复与漏解
            backtrace(nums, i + 1, path, res);
            path.pop_back();
        }
    }
};

/*
解法二：位运算枚举掩码 0..2^n-1【最优解，与解法一并列】

为什么慢：解法一靠递归"试探"出每个下标取或不取；而"取或不取"本身就是一位
二进制，n 个下标的状态天然可以用一个 n 位整数编码，递归带来的栈和回溯开销
在这个视角下是多余的。

换个角度：第 i 位为 1 就表示 nums[i] 在子集里。于是 [0, 2^n) 里的每个整数
唯一对应一个子集，直接从小到大枚举整数即可，既不会漏也不会重。

关键点：
  · 掩码取遍 [0, 2^n)，左端 0 对应空集，右端 2^n - 1 对应全集；
    写成 `mask <= (1 << n)` 就会多枚举一个越界掩码
  · 判断第 i 位用 `mask & (1 << i)`，命中的下标顺序天然递增，
    与解法一产生的子集完全相同（顺序也一致）
  · n = 0 时 1 << 0 = 1，循环只跑 mask = 0 一次，返回一个空集，
    与"幂集至少含空集"一致

手推一遍：
以示例 1（nums = [1,2,3]）为例：
  mask = 0 → []        mask = 4 → [3]
  mask = 1 → [1]       mask = 5 → [1,3]
  mask = 2 → [2]       mask = 6 → [2,3]
  mask = 3 → [1,2]     mask = 7 → [1,2,3]
  共 8 个 = 2^3 ✔

复杂度分析：
  时间 O(2^n · n)：外层 2^n 个掩码，内层扫 n 位决定每一位取不取
  空间 O(n)：只有当前子集这一个缓冲数组，递归栈为空；答案数组不计入

取舍：两条解法时间同为 O(2^n·n)、空间同为 O(n)，并列最优。解法二的 O(n)
只是一个输出缓冲、没有递归栈，深度增长时不会栈溢出；解法一不用想位运算，
但要记牢 start 下标与"进节点就收集"这两处。n <= 10 时两者实测量级一致。
*/
class Solution2
{
public:
    vector<vector<int>> subsets(vector<int> &nums)
    {
        int n = (int)nums.size();
        vector<vector<int>> res;
        res.reserve(1u << n);

        for (int mask = 0; mask < (1 << n); ++mask)
        {
            vector<int> one;
            // 位为 1 表示取该下标，低位到高位对应 nums 从前到后
            for (int i = 0; i < n; ++i)
                if (mask & (1 << i)) one.push_back(nums[i]);
            res.push_back(one);
        }
        return res;
    }
};

namespace
{
// 子集集合按题面可以任意顺序返回，内外层都排序后整体比较
vector<vector<int>> normalize(vector<vector<int>> v)
{
    for (vector<int> &inner : v) sort(inner.begin(), inner.end());
    sort(v.begin(), v.end());
    return v;
}

// 校验幂集的三个不变量：个数 2^n、互不相同、元素总数 n·2^(n-1)
void checkInvariants(const vector<vector<int>> &subs, int n)
{
    size_t want = (size_t)1 << n;
    assert(subs.size() == want);

    // 去重后个数不变，说明没有重复子集
    vector<vector<int>> sorted = normalize(subs);
    assert(unique(sorted.begin(), sorted.end()) == sorted.end());

    // 每个元素在各子集中出现的总次数：一半子集含它，即 2^(n-1) 次
    long long total = 0;
    for (const vector<int> &one : subs) total += (long long)one.size();
    if (n == 0)
        assert(total == 0);
    else
        assert(total == (long long)n * (1LL << (n - 1)));
}

void check(const vector<int> &nums)
{
    Solution1 s1;
    Solution2 s2;
    vector<int> copy1 = nums;
    vector<int> copy2 = nums;

    vector<vector<int>> a = s1.subsets(copy1);
    vector<vector<int>> b = s2.subsets(copy2);

    // 两个解法必须给出同一组子集，顺便交叉验证
    assert(normalize(a) == normalize(b));
    checkInvariants(a, (int)nums.size());
    checkInvariants(b, (int)nums.size());
}
} // namespace

int main()
{
    // 示例 1：n = 3，共 8 个子集
    {
        vector<int> nums = {1, 2, 3};
        vector<vector<int>> want = {{},    {1},    {2},    {1, 2},
                                    {3},   {1, 3}, {2, 3}, {1, 2, 3}};
        Solution1 s1;
        Solution2 s2;
        assert(normalize(s1.subsets(nums)) == normalize(want));
        assert(normalize(s2.subsets(nums)) == normalize(want));
    }

    // 示例 2：单元素，只有空集与它自己
    {
        vector<int> nums = {0};
        vector<vector<int>> want = {{}, {0}};
        Solution1 s1;
        Solution2 s2;
        assert(normalize(s1.subsets(nums)) == normalize(want));
        assert(normalize(s2.subsets(nums)) == normalize(want));
    }

    // 边界：空数组，幂集只有一个空集（力扣约束 n >= 1，这里验证健壮性）
    {
        vector<int> nums;
        vector<vector<int>> want = {{}};
        Solution1 s1;
        Solution2 s2;
        assert(normalize(s1.subsets(nums)) == normalize(want));
        assert(normalize(s2.subsets(nums)) == normalize(want));
    }

    // 元素互不相同但不是 1..n：验证实现不依赖元素具体取值
    check({4, 7});
    check({-10, 0, 10});

    // 上限规模 n = 10：2^10 = 1024 个子集，逐一去重后仍应是 1024 个
    {
        vector<int> nums;
        for (int i = 0; i < 10; ++i) nums.push_back(i - 5);
        check(nums);
    }

    cout << "all tests passed" << endl;
    return 0;
}
