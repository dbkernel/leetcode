/*
============================================================================
51. N 皇后（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/n-queens/
分类：回溯 · 逐行放置与冲突集合

按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。

n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间
不能相互攻击。

给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。

每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.'
分别代表了皇后和空位。

示例 1：
输入：n = 4
输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
解释：如上图所示，4 皇后问题存在两个不同的解法。

示例 2：
输入：n = 1
输出：[["Q"]]

提示：
- 1 <= n <= 9
*/
#include <cassert>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

/*
解法：逐行放置 + 列/主对角/副对角三个冲突集合【最优解】

为什么慢：最直白的写法是把 n 个皇后往 n×n 个格子里硬塞，再逐一检查两两
是否同行、同列、同对角线，枚举量约 C(n^2, n)，n = 8 时已经上亿；而且大量
分支在放下第二个皇后时就注定失败，全都白枚举了。

换个角度：每行必须恰好一个皇后，于是"放哪个格子"退化成"第 row 行的皇后
放在哪一列"，搜索树变成 n 叉、深度 n。冲突判断也不必两两比较，只要让列、
主对角线、副对角线各用一个集合记账，放置前 O(1) 查一次即可。

关键点：
  · 主对角线（左上到右下）上 col - row 恒定，加 n - 1 偏移后落在 [0, 2n-2]
  · 副对角线（右上到左下）上 col + row 恒定，直接落在 [0, 2n-2]
  · 先查三个集合再落子，递归返回时三个集合必须一起撤销，
    漏撤任何一个都会让后续分支误判为冲突
  · 递归到 row == n 才收集棋盘：此时每行一个皇后且无冲突，必然合法
  · 输出顺序不固定，测试必须按集合比较，不能按列表下标比较

以 n = 4 为例：
  row 0 放 col 1，占用列 1、主对角 1-0 = 1、副对角 1+0 = 1
  row 1 只能放 col 3（col 0、col 2 分别撞列与对角线）
  row 2 放 col 0，row 3 放 col 2，得到 ".Q.."/"...Q"/"Q..."/"..Q."
  回溯到 row 0 改放 col 2，对称地得到另一个解 ✔

复杂度分析：
  时间 O(n!)：第 row 行最多 n - row 个可选列（实际更少），搜索树规模不超过
    排列数 n!，每个叶子再花 O(n^2) 拼棋盘；n <= 9 时完全够用
  空间 O(n)：三个集合各 O(n)，递归栈 O(n)；结果集按题目要求不计入
*/
class Solution
{
public:
    // 契约：返回全部合法摆法，每个摆法含 n 行、每行 n 个字符
    vector<vector<string>> solveNQueens(int n)
    {
        cols.assign(n, false);
        diag1.assign(2 * n - 1, false); // col - row + n - 1
        diag2.assign(2 * n - 1, false); // col + row
        board.assign(n, string(n, '.'));
        answer.clear();

        dfs(0, n);
        return answer;
    }

private:
    vector<bool> cols, diag1, diag2;
    vector<string> board;
    vector<vector<string>> answer;

    void dfs(int row, int n)
    {
        // 每行恰好一个皇后，走满 n 行就一定是一个完整合法解
        if (row == n)
        {
            answer.push_back(board);
            return;
        }

        for (int col = 0; col < n; ++col)
        {
            int d1 = col - row + n - 1;
            int d2 = col + row;
            // 三个集合任一命中都能直接剪掉整棵子树
            if (cols[col] || diag1[d1] || diag2[d2]) continue;

            cols[col] = diag1[d1] = diag2[d2] = true;
            board[row][col] = 'Q';

            dfs(row + 1, n);

            // 撤销必须与置位一一对应，否则后续分支会继承脏状态
            board[row][col] = '.';
            cols[col] = diag1[d1] = diag2[d2] = false;
        }
    }
};

namespace
{
// 校验一份棋盘：形状正确、恰好 n 个皇后、且任意两皇后不共线
bool validBoard(const vector<string> &b)
{
    int n = static_cast<int>(b.size());
    int count = 0;
    vector<int> col(n, 0), d1(2 * n - 1, 0), d2(2 * n - 1, 0);

    for (int r = 0; r < n; ++r)
    {
        if (static_cast<int>(b[r].size()) != n) return false;
        for (int c = 0; c < n; ++c)
        {
            if (b[r][c] == '.') continue;
            if (b[r][c] != 'Q') return false;

            ++count;
            // 同行由「每行只写一个 Q」的构造保证，这里只查剩余三类
            if (++col[c] > 1) return false;
            if (++d1[c - r + n - 1] > 1) return false;
            if (++d2[c + r] > 1) return false;
        }
    }
    return count == n;
}

void check(int n, int want)
{
    Solution s;
    vector<vector<string>> got = s.solveNQueens(n);

    assert(static_cast<int>(got.size()) == want);
    for (const vector<string> &b : got) assert(validBoard(b));

    // 方案不能重复：数量对但内容重复说明回溯路径有 bug
    for (size_t i = 0; i < got.size(); ++i)
        for (size_t j = i + 1; j < got.size(); ++j)
            assert(got[i] != got[j]);
}
} // namespace

int main()
{
    // 示例 1：n = 4 有两个解
    check(4, 2);

    // 示例 2：n = 1 只有一个解
    check(1, 1);

    // 边界：n = 2、n = 3 无解
    check(2, 0);
    check(3, 0);

    // 经典规模
    check(8, 92);
    check(5, 10);
    check(6, 4);
    check(7, 40);

    // 约束上界：n = 9，解数 352
    check(9, 352);

    cout << "all tests passed" << endl;
    return 0;
}
