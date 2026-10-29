/*
============================================================================
面试题 01.08. 零矩阵（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zero-matrix-lcci/
分类：数组 · 原地标记

编写一种算法，若M × N矩阵中某个元素为0，则将其所在的行与列清零。

示例 1：
输入：
[
  [1,1,1],
  [1,0,1],
  [1,1,1]
]
输出：
[
  [1,0,1],
  [0,0,0],
  [1,0,1]
]

示例 2：
输入：
[
  [0,1,2,0],
  [3,4,5,2],
  [1,3,1,5]
]
输出：
[
  [0,0,0,0],
  [0,4,5,0],
  [0,3,1,0]
]
*/
#include <cassert>
#include <iostream>
#include <utility>
#include <vector>
using namespace std;

/*
解法一：两个标记数组（先记录，再统一清零）【非最优】

为什么慢：如果边扫描边把行列清零，刚写下的 0 会在后续扫描中被当成
"原本就是 0"，把原本不该清零的行列也牵连进去，答案会被污染。要避免
这个顺序问题，就得把"哪些行、哪些列要清零"先完整记录下来。

换个角度：真正需要传递的信息只有 m 个行标记和 n 个列标记，和矩阵里
零的个数、位置都无关。

关键点：
  · 第一遍只读不写，把 row[i] / col[j] 置位
  · 第二遍才写，顺序反过来就会把新写的 0 当成原始零
  · 标记数组初值必须是 false，否则没零的行列也会被清零

以示例 2 为例：
  零出现在 (0,0)、(0,3)，row[0]、col[0]、col[3] 置位
  第二遍按标记写，得到 [[0,0,0,0],[0,4,5,0],[0,3,1,0]] ✔

复杂度分析：
  时间 O(mn)：两遍完整扫描
  空间 O(m+n)：两排标记数组，m、n 为行列数
*/
class Solution1
{
public:
    void setZeroes(vector<vector<int>> &matrix)
    {
        int m = static_cast<int>(matrix.size());
        int n = static_cast<int>(matrix[0].size());

        // 只记录，不改矩阵：避免新写入的 0 影响后续判断
        vector<bool> row(m, false);
        vector<bool> col(n, false);
        for (int i = 0; i < m; ++i)
        {
            for (int j = 0; j < n; ++j)
            {
                if (matrix[i][j] == 0)
                {
                    row[i] = true;
                    col[j] = true;
                }
            }
        }

        // 标记已经完整，第二遍可以放心写
        for (int i = 0; i < m; ++i)
        {
            for (int j = 0; j < n; ++j)
            {
                if (row[i] || col[j]) matrix[i][j] = 0;
            }
        }
    }
};

/*
解法二：首行首列当标记，另用两个变量记录它们自身【最优解】

为什么慢：解法一的标记数组本身要 O(m+n) 空间，可矩阵的第一行和第一列
天然就能充当这两排标记，额外空间只剩"第一行、第一列原本是否有零"。

换个角度：把 matrix[i][0] 当作第 i 行的标记，把 matrix[0][j] 当作第 j 列
的标记；首行首列自己是否要清零，则用两个布尔变量单独保存。

关键点：
  · 两个 flag 必须在改写首行首列之前采集，晚一步就被覆盖
  · 标记阶段只处理 i>=1、j>=1 的内部区域，不能碰首行首列标记
  · 写内部区域时用 matrix[i][0] / matrix[0][j] 判断
  · 最后才按 flag 清零首行、首列，顺序颠倒会破坏标记

以示例 2 为例：
  内部零只有 (0,0)、(0,3)，都在首行，flag_row0 = true
  内部区域无需标记，按首行标记写内部 → 第 0、3 列全零
  最后按 flag_row0 清首行 → 整行零 ✔

复杂度分析：
  时间 O(mn)：仍是最多两遍扫描
  空间 O(1)：只多用两个布尔变量
*/
class Solution2
{
public:
    void setZeroes(vector<vector<int>> &matrix)
    {
        int m = static_cast<int>(matrix.size());
        int n = static_cast<int>(matrix[0].size());

        // 先采集首行首列自身的状态，之后它们会被当作标记改写
        bool firstRowZero = false;
        bool firstColZero = false;
        for (int j = 0; j < n; ++j)
        {
            if (matrix[0][j] == 0) firstRowZero = true;
        }
        for (int i = 0; i < m; ++i)
        {
            if (matrix[i][0] == 0) firstColZero = true;
        }

        // 内部区域的零，投影到首行首列上
        for (int i = 1; i < m; ++i)
        {
            for (int j = 1; j < n; ++j)
            {
                if (matrix[i][j] == 0)
                {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // 标记齐了，再写内部区域
        for (int i = 1; i < m; ++i)
        {
            for (int j = 1; j < n; ++j)
            {
                if (matrix[i][0] == 0 || matrix[0][j] == 0)
                {
                    matrix[i][j] = 0;
                }
            }
        }

        // 首行首列最后处理，避免提前破坏内部区域还要用的标记
        if (firstColZero)
        {
            for (int i = 0; i < m; ++i) matrix[i][0] = 0;
        }
        if (firstRowZero)
        {
            for (int j = 0; j < n; ++j) matrix[0][j] = 0;
        }
    }
};

/*
解法三：只留一个标记变量，matrix[0][0] 兼任首行标记【最优解，与解法二并列】
【最优解，与解法二并列】

为什么慢：解法二用了两个布尔变量，其中一个其实可以省掉：matrix[0][0]
既在第一列、又在第一行，可以让它专门记录"首行是否要清零"，首列则只用
一个 flag。

换个角度：把首列状态放进 flag_col0，首行状态放进 matrix[0][0]。代价是
matrix[0][0] 在清零首列时可能被提前改写，所以处理行的顺序要改。

关键点：
  · 第一遍先看 matrix[i][0] 更新 flag_col0，再处理 j>=1 的标记
  · 第二遍必须从最后一行往上走：matrix[0][j] 是列标记，若先处理第 0 行
    就会把列标记全部写成 0，后面每行都会被误清零
  · 每行先按标记写 j>=1 的部分，再按 flag_col0 写 matrix[i][0]
  · n == 1 时内层循环不执行，首列完全由 flag_col0 决定

以示例 2 为例：
  flag_col0 = true（matrix[0][0] 原本为 0）
  首行标记由 matrix[0][0] 承担，列 0、3 被标记
  从第 2 行倒序写，最后处理第 0 行，首行整行清零 ✔

复杂度分析：
  时间 O(mn)：最多两遍扫描
  空间 O(1)：只多一个布尔变量
*/
class Solution3
{
public:
    void setZeroes(vector<vector<int>> &matrix)
    {
        int m = static_cast<int>(matrix.size());
        int n = static_cast<int>(matrix[0].size());

        bool firstColZero = false;
        for (int i = 0; i < m; ++i)
        {
            // 必须在写 matrix[i][0] 之前判断它原本是否为零
            if (matrix[i][0] == 0) firstColZero = true;

            for (int j = 1; j < n; ++j)
            {
                if (matrix[i][j] == 0)
                {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // 倒序：第 0 行的 matrix[0][j] 是列标记，必须留到最后再用
        for (int i = m - 1; i >= 0; --i)
        {
            for (int j = 1; j < n; ++j)
            {
                if (matrix[i][0] == 0 || matrix[0][j] == 0)
                {
                    matrix[i][j] = 0;
                }
            }
            if (firstColZero) matrix[i][0] = 0;
        }
    }
};

namespace
{
// 与三种解法独立的参照实现：先收集坐标，再统一清零
void bruteZero(vector<vector<int>> &matrix)
{
    vector<pair<int, int>> zeros;
    for (int i = 0; i < static_cast<int>(matrix.size()); ++i)
    {
        for (int j = 0; j < static_cast<int>(matrix[0].size()); ++j)
        {
            if (matrix[i][j] == 0) zeros.push_back({i, j});
        }
    }
    for (const auto &[r, c] : zeros)
    {
        for (int j = 0; j < static_cast<int>(matrix[0].size()); ++j)
        {
            matrix[r][j] = 0;
        }
        for (int i = 0; i < static_cast<int>(matrix.size()); ++i)
        {
            matrix[i][c] = 0;
        }
    }
}

void check(const vector<vector<int>> &input,
           const vector<vector<int>> &want)
{
    Solution1 s1;
    vector<vector<int>> a = input;
    s1.setZeroes(a);
    assert(a == want);

    Solution2 s2;
    vector<vector<int>> b = input;
    s2.setZeroes(b);
    assert(b == want);

    Solution3 s3;
    vector<vector<int>> c = input;
    s3.setZeroes(c);
    assert(c == want);

    vector<vector<int>> d = input;
    bruteZero(d);
    assert(d == want);
}
} // namespace

int main()
{
    // 示例 1：零在正中间，整行整列清零
    check({{1, 1, 1}, {1, 0, 1}, {1, 1, 1}},
          {{1, 0, 1}, {0, 0, 0}, {1, 0, 1}});

    // 示例 2：首行有两个零，列标记来自首行
    check({{0, 1, 2, 0}, {3, 4, 5, 2}, {1, 3, 1, 5}},
          {{0, 0, 0, 0}, {0, 4, 5, 0}, {0, 3, 1, 0}});

    // 边界：1x1 且为零，唯一元素清零
    check({{0}}, {{0}});

    // 边界：1x1 且非零，不能凭空清零
    check({{7}}, {{7}});

    // 边界：只有一行，零会清掉整行
    check({{1, 0, 3}}, {{0, 0, 0}});

    // 边界：只有一列，零会清掉整列
    check({{1}, {0}, {3}}, {{0}, {0}, {0}});

    // 边界：零在左上角，首行首列同时被清
    check({{0, 2}, {3, 4}}, {{0, 0}, {0, 4}});

    // 边界：零只在首行，首行 flag 与列标记同时生效
    check({{1, 0, 3}, {4, 5, 6}}, {{0, 0, 0}, {4, 0, 6}});

    // 边界：零只在首列，首列 flag 单独生效
    check({{1, 2, 3}, {0, 5, 6}, {7, 8, 9}},
          {{0, 2, 3}, {0, 0, 0}, {0, 8, 9}});

    // 边界：全零矩阵
    check({{0, 0}, {0, 0}}, {{0, 0}, {0, 0}});

    // 反直觉：没有零时，任何解法都不得改动矩阵
    check({{1, 2, 3}, {4, 5, 6}}, {{1, 2, 3}, {4, 5, 6}});

    // 反直觉：零在内部非首行非首列，检验内部标记投影
    check({{1, 2, 3, 4}, {5, 0, 7, 8}, {9, 10, 11, 12}},
          {{1, 0, 3, 4}, {0, 0, 0, 0}, {9, 0, 11, 12}});

    cout << "all tests passed" << endl;
    return 0;
}
