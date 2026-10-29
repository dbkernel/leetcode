/*
============================================================================
240. 搜索二维矩阵 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/search-a-2d-matrix-ii/
分类：矩阵 · 有序矩阵的 Z 字形搜索

编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target 。
该矩阵具有以下特性：

- 每行的元素从左到右升序排列。
- 每列的元素从上到下升序排列。

示例 1：
输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],
               [10,13,14,17,24],[18,21,23,26,30]], target = 5
输出：true

示例 2：
输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],
               [10,13,14,17,24],[18,21,23,26,30]], target = 20
输出：false

提示：
- m == matrix.length
- n == matrix[i].length
- 1 <= n, m <= 300
- -10^9 <= matrix[i][j] <= 10^9
- 每行的所有元素从左到右升序排列
- 每列的所有元素从上到下升序排列
- -10^9 <= target <= 10^9
*/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

/*
解法一：逐个比较【非最优】

为什么慢：有序性一点没用上，把 mn 个元素全部看一遍。对于 300 × 300
的矩阵就是 9 万次比较，而有序性允许我们一次排除一整行或一整列。

换个角度：先有最朴素的答案作为基线 —— 双层循环找 target，找不到返回
false。它的价值在于给出正确性参照和 O(mn) 的时间上界。

关键点：
  · 找到即返回 true，不要继续扫完
  · 行列都从 0 开始，矩阵非空由约束保证
  · 没有任何剪枝，因此对任意矩阵都成立（不依赖有序）

以示例 1 中 target = 20 为例：
  逐行扫完 25 个元素都没命中 → 返回 false ✔

复杂度分析：
  时间 O(mn)：每个元素比较一次
  空间 O(1)：只用循环变量
*/
class Solution1
{
public:
    bool searchMatrix(vector<vector<int>> &matrix, int target)
    {
        for (const vector<int> &row : matrix)
        {
            for (int value : row)
            {
                if (value == target) return true;
            }
        }
        return false;
    }
};

/*
解法二：每行做一次二分查找【非最优】

为什么慢：解法一在行内也是线性扫描，浪费了「行内升序」。对每一行二分，
时间从 O(mn) 降到 O(m log n)。

换个角度：把问题拆成 m 个独立的一维查找。行内有序正好可以用 lower_bound
定位第一个 >= target 的位置。

关键点：
  · 用 lower_bound 而不是自己写 while(low<=high)：语义就是「第一个
    >= target 的下标」，省掉边界判断
  · 命中条件是 it != row.end() && *it == target：lower_bound 可能返回
    end，解引用前必须先判
  · 每行独立判断，任意一行命中即可返回 true

以示例 1 中 target = 5 为例：
  第 0 行 [1,4,7,11,15] 二分得到 7，不是 5
  第 1 行 [2,5,8,12,19] 二分得到 5 → 返回 true ✔

复杂度分析：
  时间 O(m log n)：每行一次二分，共 m 行
  空间 O(1)：只用迭代器
*/
class Solution2
{
public:
    bool searchMatrix(vector<vector<int>> &matrix, int target)
    {
        for (const vector<int> &row : matrix)
        {
            auto it = lower_bound(row.begin(), row.end(), target);
            if (it != row.end() && *it == target) return true;
        }
        return false;
    }
};

/*
解法三：从右上角出发的 Z 字形搜索【最优解】

为什么慢：解法二每行都要二分，共 O(m log n)；但列的有序性完全没用上。
只要同时利用行、列两个方向的有序性，就能把两个维度的搜索合并成一条
单调路径。

换个角度：站在右上角 (0, n-1)，它是「本行最大、本列最小」。比较它和
target：
  · 相等 → 找到
  · 大于 target → 这一列从它往下都更大，整列可以排除，列下标左移
  · 小于 target → 这一行从它往左都更小，整行可以排除，行下标下移
每走一步都排除一整行或一整列，走到底最多 m + n 步。

关键点：
  · 起点必须是右上角或左下角：只有这两个角同时是「一行的一端」和
    「一列的一端」，才能从比较结果推出「整行/整列可弃」
  · 左上角不行：它同时是本行与本列的最小值，比 target 小只能说明
    「它自己不是」，推不出整行或整列可弃
  · 大于时减列、小于时加行，方向反了会走向已经排除的区域
  · 循环条件是 x < m && y >= 0，越界即说明 target 不存在

以示例 1 中 target = 5 为例：
  (0,4)=15 > 5 → 弃第 4 列，y = 3
  (0,3)=11 > 5 → 弃第 3 列，y = 2
  (0,2)=7 > 5 → 弃第 2 列，y = 1
  (0,1)=4 < 5 → 弃第 0 行，x = 1
  (1,1)=5 → 返回 true ✔

复杂度分析：
  时间 O(m+n)：x 只增、y 只减，各最多走 m / n 步
  空间 O(1)：只用两个下标
*/
class Solution3
{
public:
    bool searchMatrix(vector<vector<int>> &matrix, int target)
    {
        int m = static_cast<int>(matrix.size());
        int n = static_cast<int>(matrix[0].size());

        // 右上角：本行最大、本列最小，一次比较可排除一整行或一整列
        int row = 0, col = n - 1;
        while (row < m && col >= 0)
        {
            if (matrix[row][col] == target) return true;
            if (matrix[row][col] > target)
            {
                --col;
            }
            else
            {
                ++row;
            }
        }
        return false;
    }
};

// ===== 测试辅助 =====
namespace
{
using Matrix = vector<vector<int>>;

void check(Matrix matrix, int target, bool want)
{
    // 三个解法都只读矩阵，按值传参让每个用例互不影响
    Solution1 s1;
    Solution2 s2;
    Solution3 s3;
    assert(s1.searchMatrix(matrix, target) == want);
    assert(s2.searchMatrix(matrix, target) == want);
    assert(s3.searchMatrix(matrix, target) == want);
}

Matrix sample()
{
    return {{1, 4, 7, 11, 15},
            {2, 5, 8, 12, 19},
            {3, 6, 9, 16, 22},
            {10, 13, 14, 17, 24},
            {18, 21, 23, 26, 30}};
}
} // namespace

int main()
{
    // 示例 1：目标在第二行中间
    check(sample(), 5, true);

    // 示例 2：目标落在行、列的缝隙里
    check(sample(), 20, false);

    // 覆盖四个角：右上、左下、左上、右下
    check(sample(), 15, true);
    check(sample(), 18, true);
    check(sample(), 1, true);
    check(sample(), 30, true);

    // 边界：小于最小值、大于最大值
    check(sample(), 0, false);
    check(sample(), 31, false);

    // 边界：1 x 1
    check({{5}}, 5, true);
    check({{5}}, 4, false);

    // 边界：单行，Z 字形退化为一路左移
    check({{1, 3, 5, 7}}, 5, true);
    check({{1, 3, 5, 7}}, 6, false);

    // 边界：单列，Z 字形退化为一路下移
    check({{1}, {3}, {5}}, 5, true);
    check({{1}, {3}, {5}}, 4, false);

    // 反直觉：矩阵允许重复值，比较相等即返回，不必找唯一位置
    check({{1, 1, 1}, {1, 1, 1}, {1, 1, 1}}, 1, true);
    check({{1, 1, 1}, {1, 1, 1}, {1, 1, 1}}, 2, false);

    // 反直觉：负值矩阵，不能把 0 当作哨兵
    check({{-5, -3}, {-2, -1}}, -3, true);
    check({{-5, -3}, {-2, -1}}, 0, false);

    cout << "all tests passed" << endl;
    return 0;
}
