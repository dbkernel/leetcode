/*
============================================================================
529. 扫雷游戏（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/minesweeper/
分类：深度优先搜索 · 网格模拟

让我们一起来玩扫雷游戏！

给你一个大小为 m x n 二维字符矩阵 board ，表示扫雷游戏的盘面，其中：

- 'M' 代表一个 未挖出的 地雷，
- 'E' 代表一个 未挖出的 空方块，
- 'B' 代表没有相邻（上，下，左，右，和所有4个对角线）地雷的 已挖出的
  空白方块，
- 数字（'1' 到 '8'）表示有多少地雷与这块 已挖出的 方块相邻，
- 'X' 则表示一个 已挖出的 地雷。

给你一个整数数组 click ，其中 click = [clickr, clickc] 表示在所有 未挖出的
方块（'M' 或者 'E'）中的下一个点击位置（clickr 是行下标，clickc 是列下标）。

根据以下规则，返回相应位置被点击后对应的盘面：

1. 如果一个地雷（'M'）被挖出，游戏就结束了- 把它改为 'X' 。
2. 如果一个 没有相邻地雷 的空方块（'E'）被挖出，修改它为（'B'），并且所有
   和其相邻的 未挖出 方块都应该被递归地揭露。
3. 如果一个 至少与一个地雷相邻 的空方块（'E'）被挖出，修改它为数字
   （'1' 到 '8' ），表示相邻地雷的数量。
4. 如果在此次点击中，若无更多方块可被揭露，则返回盘面。

示例 1：
输入：board = [["E","E","E","E","E"],["E","E","M","E","E"],
["E","E","E","E","E"],["E","E","E","E","E"]], click = [3,0]
输出：[["B","1","E","1","B"],["B","1","M","1","B"],
["B","1","1","1","B"],["B","B","B","B","B"]]

示例 2：
输入：board = [["B","1","E","1","B"],["B","1","M","1","B"],
["B","1","1","1","B"],["B","B","B","B","B"]], click = [1,2]
输出：[["B","1","E","1","B"],["B","1","X","1","B"],
["B","1","1","1","B"],["B","B","B","B","B"]]

提示：
- m == board.length
- n == board[i].length
- 1 <= m, n <= 50
- board[i][j] 为 'M'、'E'、'B' 或数字 '1' 到 '8' 中的一个
- click.length == 2
- 0 <= clickr < m
- 0 <= clickc < n
- board[clickr][clickc] 为 'M' 或 'E'
*/
#include <cassert>
#include <iostream>
#include <queue>
#include <string>
#include <utility>
#include <vector>
using namespace std;

namespace
{
// 八个方向：上、下、左、右与四条对角线
const int DX[8] = {0, 1, 0, -1, 1, 1, -1, -1};
const int DY[8] = {1, 0, -1, 0, 1, -1, 1, -1};

bool inBoard(const vector<vector<char>> &board, int x, int y)
{
    int m = static_cast<int>(board.size());
    int n = static_cast<int>(board[0].size());
    return x >= 0 && x < m && y >= 0 && y < n;
}

// 统计 (x, y) 周围八个格子里的地雷数
int countMines(const vector<vector<char>> &board, int x, int y)
{
    int cnt = 0;
    for (int d = 0; d < 8; ++d)
    {
        int nx = x + DX[d];
        int ny = y + DY[d];
        // 越界的邻居直接跳过，不能把下标算负
        if (inBoard(board, nx, ny) && board[nx][ny] == 'M') ++cnt;
    }
    return cnt;
}
} // namespace

/*
解法一：深度优先搜索 + 模拟【最优解】

为什么慢：最容易想到的是对每个格子反复扫描周围的雷数，再按规则逐个改值。
但揭露具有连锁性：一个 'B' 会带出相邻的 'E'，这些新格子又可能带出更多格子。
若没有"已访问"的约束，同一片空白会被多条路径重复进入，甚至无限递归。

换个角度：把"点击"看成从起点出发的一次泛洪。每个格子只处理一次，
处理时先数雷：有雷就写成数字并停下；没雷就写 'B' 并继续向八个方向扩散。
visited 的职责由盘面本身承担 —— 处理过的格子不再是 'E'，天然不会被二次进入。

关键点：
  · 先判断点击格是不是 'M'，是则只改 'X' 并立即返回，绝不扩散
  · 数字格只写数字、不递归；只有 'B' 的邻居才继续扩散
  · 写 'B' 必须发生在递归之前，否则两个邻居会同时把同一格当成 'E'
  · 递归时只进入 'E'，'B'、数字、'M' 都跳过
  · 方向数组覆盖八个方向，对角线也要算进雷数

以示例 1（4x5，地雷在 (1,2)，点击 (3,0)）为例：
  (3,0) 周围无雷 -> 'B'，向八个方向扩散
    (2,0)、(1,0)、(0,0) 仍无雷 -> 继续为 'B'
    (2,1)、(2,2)、(1,1) 邻雷 -> 依次写 '1'，不再扩散
  (0,2) 四周全是数字或地雷，没有 'B' 邻居可达 -> 保持 'E' ✔
  最终盘面与示例输出一致

复杂度分析：
  时间 O(mn)：每个格子最多被访问一次，每次检查固定的八个方向
  空间 O(mn)：最坏（全为空地）递归栈深度可达格子总数
*/
class Solution1
{
public:
    vector<vector<char>> updateBoard(vector<vector<char>> &board,
                                     vector<int> &click)
    {
        int x = click[0];
        int y = click[1];

        // 规则 1：踩雷即结束，只翻这一格
        if (board[x][y] == 'M')
        {
            board[x][y] = 'X';
            return board;
        }

        dfs(board, x, y);
        return board;
    }

private:
    void dfs(vector<vector<char>> &board, int x, int y)
    {
        int cnt = countMines(board, x, y);

        // 规则 3：邻雷数大于零，写数字并终止这一条分支
        if (cnt > 0)
        {
            board[x][y] = static_cast<char>('0' + cnt);
            return;
        }

        // 规则 2：先落 'B' 再扩散，避免邻居重复进入
        board[x][y] = 'B';
        for (int d = 0; d < 8; ++d)
        {
            int nx = x + DX[d];
            int ny = y + DY[d];
            // 只有未挖出的空格需要继续揭露
            if (inBoard(board, nx, ny) && board[nx][ny] == 'E')
            {
                dfs(board, nx, ny);
            }
        }
    }
};

/*
解法二：广度优先搜索 + 模拟【最优解，与解法一并列】

为什么慢：解法一的递归深度最坏是 O(mn)。50x50 全为空地时，调用栈会被
压到两千多层；语言栈空间紧张时存在爆栈风险。算法本身没问题，是"递归"
这一实现方式带来的额外风险。

换个角度：泛洪不要求深度优先，把待处理的格子放进显式队列逐层处理即可。
队列里的每个格子入队时标记 visited，出队时再数雷、写值、决定是否扩散。

关键点：
  · 入队时就置 visited，而不是出队时；否则同一格会被多个邻居重复入队
  · 数雷在出队后进行，此时盘面里所有 'M' 都还没被改动
  · 邻雷数为零才把周围的 'E' 入队；数字格不扩散
  · 起点只入队一次；起点是 'M' 的情况在函数入口单独处理
  · 队列与 visited 都是 O(mn)，空间并没有比 DFS 更省，只是不占调用栈

以示例 2（点击 (1,2) 的 'M'）为例：
  入口发现 board[1][2] == 'M' -> 改为 'X' 并返回
  周围已翻开的 'B'、'1' 与未挖出的 'E' 都保持原样 ✔

复杂度分析：
  时间 O(mn)：每格至多入队、出队一次
  空间 O(mn)：visited 标记与队列最坏都装下整个盘面
*/

class Solution2
{
public:
    vector<vector<char>> updateBoard(vector<vector<char>> &board,
                                     vector<int> &click)
    {
        int sx = click[0];
        int sy = click[1];

        // 规则 1：踩雷只翻当前格，不触发任何扩散
        if (board[sx][sy] == 'M')
        {
            board[sx][sy] = 'X';
            return board;
        }

        int m = static_cast<int>(board.size());
        int n = static_cast<int>(board[0].size());
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        queue<pair<int, int>> q;

        q.push({sx, sy});
        visited[sx][sy] = true;

        while (!q.empty())
        {
            auto [x, y] = q.front();
            q.pop();

            int cnt = countMines(board, x, y);
            if (cnt > 0)
            {
                // 规则 3：数字格不扩散
                board[x][y] = static_cast<char>('0' + cnt);
                continue;
            }

            // 规则 2：空白格向八个方向继续揭露
            board[x][y] = 'B';
            for (int d = 0; d < 8; ++d)
            {
                int nx = x + DX[d];
                int ny = y + DY[d];
                if (inBoard(board, nx, ny) && board[nx][ny] == 'E' &&
                    !visited[nx][ny])
                {
                    visited[nx][ny] = true;
                    q.push({nx, ny});
                }
            }
        }

        return board;
    }
};

// ===== 测试辅助：用字符串行构造盘面、逐格比较 =====
namespace
{
using Board = vector<vector<char>>;

Board makeBoard(const vector<string> &rows)
{
    Board board;
    board.reserve(rows.size());
    for (const string &row : rows)
    {
        board.push_back(vector<char>(row.begin(), row.end()));
    }
    return board;
}

void check(const vector<string> &rows, const vector<int> &click,
           const vector<string> &wantRows)
{
    Board want = makeBoard(wantRows);

    Solution1 s1;
    Board b1 = makeBoard(rows);
    vector<int> c1 = click;
    assert(s1.updateBoard(b1, c1) == want);

    Solution2 s2;
    Board b2 = makeBoard(rows);
    vector<int> c2 = click;
    assert(s2.updateBoard(b2, c2) == want);
}

} // namespace

int main()
{
    // 示例 1：从角落扩散，被数字包围的 (0,2) 仍保持 'E'
    check({"EEEEE", "EEMEE", "EEEEE", "EEEEE"}, {3, 0},
          {"B1E1B", "B1M1B", "B111B", "BBBBB"});

    // 示例 2：点击地雷，只把该格改为 'X'
    check({"B1E1B", "B1M1B", "B111B", "BBBBB"}, {1, 2},
          {"B1E1B", "B1X1B", "B111B", "BBBBB"});

    // 边界：单格地雷
    check({"M"}, {0, 0}, {"X"});

    // 边界：单格空地
    check({"E"}, {0, 0}, {"B"});

    // 反直觉：点击格紧邻地雷，只写数字、不扩散
    check({"EM"}, {0, 0}, {"1M"});

    // 反直觉：点击右下角，左上角的地雷只影响它周围三格
    check({"MEE", "EEE", "EEE"}, {2, 2}, {"M1B", "11B", "BBB"});

    // 反直觉：点击中心地雷的斜上方，该格是数字，盘面其余不动
    check({"EEE", "EME", "EEE"}, {0, 0}, {"1EE", "EME", "EEE"});

    // 边界：一整行空地，整行都会被揭露
    check({"EEEEE"}, {0, 2}, {"BBBBB"});

    // 边界：已翻开的 'B' 与数字不应被重算，只揭露剩余的 'E'
    check({"B1E", "11E", "EEE"}, {2, 2}, {"B1B", "11B", "BBB"});

    // 边界：点击地雷时，周围未挖出的 'E' 也不会被带出
    check({"EME"}, {0, 1}, {"EXE"});

    // 边界：50x50 全空地，验证泛洪能覆盖整个盘面
    {
        vector<string> rows(50, string(50, 'E'));
        vector<string> wantRows(50, string(50, 'B'));
        check(rows, {25, 25}, wantRows);
    }

    cout << "all tests passed" << endl;
    return 0;
}
