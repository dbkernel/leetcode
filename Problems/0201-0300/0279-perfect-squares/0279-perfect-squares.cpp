/*
============================================================================
279. 完全平方数（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/perfect-squares/
分类：动态规划 · 完全背包 / 四平方和定理
============================================================================

给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。

完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个
整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。

示例 1：
输入：n = 12
输出：3
解释：12 = 4 + 4 + 4

示例 2：
输入：n = 13
输出：2
解释：13 = 4 + 9

提示：
- 1 <= n <= 10^4
*/
#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：动态规划（完全背包）【非最优】

为什么慢：这是把 n 当成背包容量、把 1、4、9、16… 当成物品的完全背包。
为了求 n 一个值，必须先把 1 到 n 每个容量的最优答案都算出来，状态数是
n 个；每个状态还要枚举不超过它的平方数，共 √n 个，于是 O(n·√n)。n 最大
10^4 时够用，但答案本身只可能是 1 到 4，把这些状态全建出来是浪费。

换个角度：把「和」当成容量递增地推，dp[i] 只依赖比 i 小的 dp 值；因为
平方数可以反复取，内层正序即可，不必考虑「物品只能用一次」。

关键点：
  · dp[0] = 0 是唯一初值，含义是「和为 0 不需要任何平方数」；它是所有
    转移的落点，缺了它连 dp[1] 都推不出来
  · 初值取 n + 1 而不是 INT_MAX：dp[i - j²] + 1 会做加法，INT_MAX + 1
    会溢出成负数；n + 1 一定比任何合法答案（最多 n 个 1）都大
  · 内层只枚举 j² ≤ i，保证 i - j² 不越界
  · 外层必须按 i 递增，内层平方数怎么枚举都行，这是「容量在外」的完全
    背包写法

手推一遍（n = 12）：
  dp[1] = 1，dp[4] = 1，dp[9] = 1（本身是平方数）
  dp[8] = dp[4] + 1 = 2
  dp[11] = dp[9] + 1 + 1 = 3
  dp[12] = min(dp[11], dp[8], dp[3]) + 1 = min(3, 2, 3) + 1 = 3 ✔
  对应 12 = 4 + 4 + 4

复杂度分析：
  时间 O(n·√n)：n 个容量，每个枚举到 √n 个平方数
  空间 O(n)：一维 dp 表
*/
class Solution1
{
public:
    int numSquares(int n)
    {
        // dp[i]：凑出 i 的最少个数；最多用 i 个 1，故 n + 1 是不可能值
        vector<int> dp(n + 1, n + 1);
        dp[0] = 0;

        for (int i = 1; i <= n; ++i)
        {
            // 枚举最后一个平方数 j²，剩下的 i - j² 已经算好
            for (int j = 1; j * j <= i; ++j)
            {
                dp[i] = min(dp[i], dp[i - j * j] + 1);
            }
        }
        return dp[n];
    }
};

/*
解法二：BFS 按层扩展（把和看成图上的点）【非最优】

为什么慢：把 0 到 n 每个整数看成图上的点，两个点差一个平方数就连一条边，
题目问的就是 0 到 n 的最短路。BFS 按步数一层层扩散，第一次碰到 n 时的
层数就是答案。

但 BFS 的规模并没有比解法一小：最坏要把 n 个点全访问一遍，每个点展开
√n 条边，仍是 O(n·√n)，而且多了一个队列和一份访问标记，常数更大。
真正能把复杂度压下来的只有解法三的数论结论。

换个角度：与其问「凑出 i 最少要几个」，不如问「用 k 个平方数能凑出哪些
和」，k 从 1 往上加，加到 n 出现为止。

关键点：
  · 每层开始前先快照队列长度：循环里新入队的属于下一层，混在本层里数
    层数会数错
  · visited 必须在入队时标记，不能出队时标记，否则同一个和会被重复入队
  · 从 0 出发、每次加平方数，或从 n 出发、每次减平方数都可以；前者更好
    写，因为答案就是层数
  · 层数从 1 开始计：第一层放进去的点都只用了一个平方数
  · 因为一定有 1 这个平方数，n 必然可达，循环不会空转结束

手推一遍（n = 12）：
  第 1 层：1、4、9（不含 12）
  第 2 层：由 1 得 2、5、10；由 4 得 8、13…；由 9 得 10、18…
  第 3 层：由 8 加 4 得 12 → 返回 3 ✔（12 = 4 + 4 + 4）

复杂度分析：
  时间 O(n·√n)：最坏访问 n 个和，每个和展开 √n 条边
  空间 O(n)：队列与访问标记各占一份
*/
class Solution2
{
public:
    int numSquares(int n)
    {
        if (n <= 0) return 0;

        vector<char> visited(n + 1, 0);
        vector<int> queue;
        queue.reserve(n + 1);
        queue.push_back(0);
        visited[0] = 1;

        int level = 0;
        size_t head = 0;
        while (head < queue.size())
        {
            // 本层结点数此刻固定，后面 push 的都是下一层
            size_t tail = queue.size();
            ++level;

            while (head < tail)
            {
                int cur = queue[head++];
                for (int j = 1; cur + j * j <= n; ++j)
                {
                    int next = cur + j * j;
                    if (next == n) return level;
                    if (visited[next] == 0)
                    {
                        visited[next] = 1;
                        queue.push_back(next);
                    }
                }
            }
        }
        // 1 是平方数，n 一定能被凑出，走不到这里
        return level;
    }
};

/*
解法三：四平方和定理（按 n 的形态直接分类）【最优解】

为什么慢：解法一、二都在算「每个和的最少个数」，而题目只要 n 这一个值。
拉格朗日四平方和定理给出：每个正整数都能写成不超过 4 个平方数之和，
所以答案只可能是 1、2、3、4。再配上「n = 4^a(8b+7) 时恰好需要 4 个」
这条判定，就能 O(√n) 直接说出答案。

换个角度：别做加法式搜索，改成对 n 的分类判断，每一类一次判定就够。

关键点：
  · 判 1：n 本身是完全平方数
  · 判 2：枚举 a ≤ √n，只要 n - a² 也是完全平方数，答案就是 2
    （把 n 拆成 a² + b²，两个平方数）
  · 判 4 前必须先把因子 4 除干净，再看剩余部分是否 ≡ 7 (mod 8)；
    直接对原数取模会漏掉 28 这类（28 = 4 × 7，需要 4 个）
  · 以上都不成立就是 3，这是定理保证的，不需要再搜
  · 判平方时浮点 sqrt 可能有 1 的误差，要回退/前进一格再验证

手推一遍：
  n = 12：不是平方；12-1=11、12-4=8、12-9=3 都不是平方；除 4 得 3，
  3 % 8 = 3 ≠ 7 → 3 ✔
  n = 28：不是平方；28-1=27、28-4=24、28-9=19、28-16=12、28-25=3
  都不是平方；除 4 得 7，7 % 8 = 7 → 4 ✔（28 = 25 + 1 + 1 + 1）

复杂度分析：
  时间 O(√n)：判 2 要枚举到 √n，其余都是 O(1)
  空间 O(1)：只用几个整数变量
*/
class Solution3
{
public:
    int numSquares(int n)
    {
        if (isSquare(n)) return 1;

        // 两个平方数：枚举较小的一项，另一项必须是平方数
        for (int a = 1; a * a <= n; ++a)
        {
            if (isSquare(n - a * a)) return 2;
        }

        // 定理：n = 4^a(8b+7) ⇔ 恰好要 4 个；先除净因子 4 再判模 8
        int m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;

        return 3;
    }

private:
    static bool isSquare(int x)
    {
        int r = static_cast<int>(sqrt(static_cast<double>(x)));
        // 浮点开方可能差一格，向下、向上各校正一次再下结论
        while (r > 0 && r * r > x) --r;
        while ((r + 1) * (r + 1) <= x) ++r;
        return r * r == x;
    }
};

// ===== 测试辅助：小规模穷举，作为对拍的独立真值来源 =====
namespace
{
// 逐个试平方数往下减，used 是已用个数；best 作为剪枝上界
void dfs(int remain, int used, int &best)
{
    if (remain == 0)
    {
        if (used < best) best = used;
        return;
    }
    if (used + 1 >= best) return;

    for (int j = 1; j * j <= remain; ++j)
    {
        dfs(remain - j * j, used + 1, best);
    }
}

int bruteForce(int n)
{
    // 上界：全部用 1
    int best = n;
    dfs(n, 0, best);
    return best;
}

void check(int n, int want)
{
    Solution1 s1;
    Solution2 s2;
    Solution3 s3;
    assert(s1.numSquares(n) == want);
    assert(s2.numSquares(n) == want);
    assert(s3.numSquares(n) == want);
}
} // namespace

int main()
{
    // 示例 1 / 示例 2
    check(12, 3);
    check(13, 2);

    // 1 到 20 的全部答案，逐个钉死
    const int expected[20] = {1, 2, 3, 1, 2, 3, 4, 2, 1, 2,
                              3, 3, 2, 3, 4, 1, 2, 2, 3, 2};
    for (int n = 1; n <= 20; ++n)
    {
        check(n, expected[n - 1]);
    }

    // 边界：本身就是平方数，答案为 1
    check(1, 1);
    check(4, 1);
    check(10000, 1);

    // 边界：4^a(8b+7) 形态，答案为 4
    check(7, 4);
    check(28, 4);
    check(9999, 4);

    // 边界：需要 3 个的典型值
    check(3, 3);
    check(11, 3);

    // 对拍：1..300 逐个与穷举比对
    for (int n = 1; n <= 300; ++n)
    {
        int want = bruteForce(n);
        check(n, want);
    }

    cout << "all tests passed" << endl;
    return 0;
}
