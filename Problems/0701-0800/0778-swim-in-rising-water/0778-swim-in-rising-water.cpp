/*
============================================================================
778. 水位上升的泳池中游泳（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/swim-in-rising-water/
分类：并查集 · 按高度加格

在一个 n x n 的整数矩阵 grid 中，每一个方格的值 grid[i][j] 表示位置
(i, j) 的平台高度。

当开始下雨时，在时间为 t 时，水池中的水位为 t 。你可以从一个平台游向
四周相邻的任意一个平台，但是前提是此时水位必须同时淹没这两个平台。假定
你可以瞬间移动无限距离，也就是默认在方格内部游动是不耗时的。当然，在你
游泳的时候你必须待在坐标方格里面。

你从坐标方格的左上平台 (0，0) 出发。返回 你到达坐标方格的右下平台
(n-1, n-1) 所需的最少时间 。

示例 1:
输入: grid = [[0,2],[1,3]]
输出: 3
解释:
时间为0时，你位于坐标方格的位置为 (0, 0)。
此时你不能游向任意方向，因为四个相邻方向平台的高度都大于当前时间为 0
时的水位。
等时间到达 3 时，你才可以游向平台 (1, 1). 因为此时的水位是 3，坐标
方格中的平台没有比水位 3 更高的，所以你可以游向坐标方格中的任意位置

示例 2:
输入: grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
输出: 16
解释: 最终的路线用加粗进行了标记。
我们必须等到时间为 16，此时才能保证平台 (0, 0) 和 (4, 4) 是连通的

提示:
- n == grid.length
- n == grid[i].length
- 1 <= n <= 50
- 0 <= grid[i][j] < n^2
- grid[i][j] 中每个值 均无重复
*/
#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <numeric>
#include <queue>
#include <tuple>
#include <utility>
#include <vector>
using namespace std;

namespace
{
// 并查集：路径压缩 + 按大小合并。
// 路径压缩把查找路径上的点直接挂到根，缩短后续查找；
// 按大小合并把元素少的树挂到元素多的树下，防止树退化成链。
// 两者合起来让单次操作的均摊代价接近 O(α(n))。
struct DSU
{
    vector<int> parent;
    vector<int> size; // 只对根有意义：该集合的元素个数

    explicit DSU(int n) : parent(n), size(n, 1)
    {
        for (int i = 0; i < n; ++i) parent[i] = i;
    }

    int find(int x)
    {
        if (parent[x] != x) parent[x] = find(parent[x]);
        return parent[x];
    }

    void unite(int x, int y)
    {
        int rx = find(x);
        int ry = find(y);
        if (rx == ry) return;
        if (size[rx] < size[ry]) swap(rx, ry);
        parent[ry] = rx;
        size[rx] += size[ry];
    }
};

const int DR[4] = {1, -1, 0, 0};
const int DC[4] = {0, 0, 1, -1};
} // namespace

/*
解法一：二分答案 + BFS 可达性判定【最优解】

为什么慢：直接枚举所有路径再取“路径最大高度”的最小值，路径数随 n 指数
增长，根本枚举不完；但答案并不依赖具体走哪条路。

换个角度：固定水位 t，只允许走高度不超过 t 的格子，问题就退化成“起点
能否到达终点”。这个判定对 t 单调——水位越高，可走的格子只增不减——于是
可以对答案二分。

关键点：
  · 二分区间取 [grid[0][0], n*n-1]：起点自己必须被淹，答案不会低于
    起点高度；上界取最大可能高度
  · 每轮判定 BFS/DFS 只走 grid 值 ≤ t 的格子，visited 必须每轮重置
  · 单调性成立才敢二分：t 可行则 t+1 必可行，用 `lo < hi` 的写法不会漏解
  · 单格棋盘 n = 1 时 lo 与 hi 都是 0，循环不执行，直接返回起点高度

手推一遍：以示例 1（grid = [[0,2],[1,3]]）为例：
  t=0：只能待在 (0,0)，不可达
  t=1：能到 (1,0)，仍不可达
  t=2：能到 (0,1)，仍不可达
  t=3：四个格子全通，起点终点连通 → 3 ✔

复杂度分析：
  时间 O(n^2 log(n^2))：二分 log(n^2) 轮，每轮 BFS 扫 n^2 个格子
  空间 O(n^2)：visited 数组 + 迭代栈

解法二：最小堆，按路径最大高度扩展【最优解，与解法一并列】

为什么慢：二分要对同一张图重复搜 log(n^2) 次，很多格子被反复访问。

换个角度：把路径代价定义成路径上格子的最大高度，这就是一个 minimax
最短路。用小顶堆按“当前路径最大高度”从小到大扩展，第一次弹出终点时的
值就是最小可行水位，整个图只走一遍。

关键点：
  · 堆里存 (路径最大高度, r, c)，按高度升序弹出
  · dist 记录“到达该格的最小路径最大值”，只有更小才入堆，
    旧条目弹出时用 `d != dist[id]` 跳过
  · 松弛用 max(当前代价, 邻格高度)，不是相加——代价取最大值
  · 起点入堆的代价是 grid[0][0]，终点一弹出即可返回

手推一遍：仍以示例 1 为例：
  起点代价 0；扩展 (1,0) 代价 max(0,1)=1，(0,1) 代价 2
  弹 1 扩展 (1,1) 代价 max(1,3)=3；弹 2 无法更新 (1,1)
  终点以 3 弹出 → 3 ✔

复杂度分析：
  时间 O(n^2 log(n^2))：每个格子至多入堆一次，堆操作 O(log(n^2))
  空间 O(n^2)：dist 数组 + 最坏 n^2 个堆元素

解法三：并查集按高度升序加格子【最优解，与解法一、二并列】

为什么慢：二分与堆都在反复“试探”连通性；并查集可以直接模拟水位上涨：
水位每涨一格，就把这一格激活，并与四周已激活的格子合并。

换个角度：把格子按高度升序处理，处理到高度 h 时，所有高度 ≤ h 的格子
都已激活。第一次发现起点与终点同集合时，当前的 h 就是答案。

关键点：
  · 只与“已激活”的邻居合并：按高度升序处理保证不会提前连上更高的格子
  · active 数组区分已激活/未激活，未激活的格子不能参与合并
  · 排序键用高度即可，编号 r*n+c 可反解坐标
  · 起终点可能到最后一格才连通，循环中不能提前退出；n = 1 时第一格即连通

手推一遍：仍以示例 1 为例（高度升序 0,1,2,3）：
  加 0：起点激活，无已激活邻居
  加 1：(1,0) 与 (0,0) 合并
  加 2：(0,1) 与 (0,0) 合并
  加 3：(1,1) 与 (1,0)、(0,1) 合并 → 起终点同集合，返回 3 ✔

复杂度分析：
  时间 O(n^2 log(n^2))：排序主导，n^2 次合并均摊接近常数
  空间 O(n^2)：parent、size、active 与排序数组
*/
class Solution1
{
public:
    int swimInWater(vector<vector<int>> &grid)
    {
        int n = static_cast<int>(grid.size());
        int lo = grid[0][0];
        int hi = n * n - 1;
        while (lo < hi)
        {
            int mid = lo + (hi - lo) / 2;
            if (canReach(grid, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

private:
    bool canReach(const vector<vector<int>> &grid, int water)
    {
        int n = static_cast<int>(grid.size());
        if (grid[0][0] > water) return false;

        vector<char> seen(n * n, 0);
        vector<int> stack = {0};
        seen[0] = 1;
        while (!stack.empty())
        {
            int id = stack.back();
            stack.pop_back();
            int r = id / n;
            int c = id % n;
            if (r == n - 1 && c == n - 1) return true;
            for (int k = 0; k < 4; ++k)
            {
                int nr = r + DR[k];
                int nc = c + DC[k];
                if (nr < 0 || nr >= n || nc < 0 || nc >= n) continue;
                int nid = nr * n + nc;
                if (seen[nid] || grid[nr][nc] > water) continue;
                seen[nid] = 1;
                stack.push_back(nid);
            }
        }
        return false;
    }
};

class Solution2
{
public:
    int swimInWater(vector<vector<int>> &grid)
    {
        int n = static_cast<int>(grid.size());
        vector<int> dist(n * n, INT_MAX);
        using State = tuple<int, int, int>; // (代价, 行, 列)
        priority_queue<State, vector<State>, greater<State>> pq;

        dist[0] = grid[0][0];
        pq.push({grid[0][0], 0, 0});
        while (!pq.empty())
        {
            auto [d, r, c] = pq.top();
            pq.pop();
            int id = r * n + c;
            // 旧条目：这个格子已经用更小的代价处理过
            if (d != dist[id]) continue;
            if (r == n - 1 && c == n - 1) return d;
            for (int k = 0; k < 4; ++k)
            {
                int nr = r + DR[k];
                int nc = c + DC[k];
                if (nr < 0 || nr >= n || nc < 0 || nc >= n) continue;
                int nid = nr * n + nc;
                // 代价取路径最大值，不是边权求和
                int nd = max(d, grid[nr][nc]);
                if (nd < dist[nid])
                {
                    dist[nid] = nd;
                    pq.push({nd, nr, nc});
                }
            }
        }
        return grid[0][0];
    }
};

class Solution3
{
public:
    int swimInWater(vector<vector<int>> &grid)
    {
        int n = static_cast<int>(grid.size());
        vector<int> order(n * n);
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&grid, n](int a, int b)
             { return grid[a / n][a % n] < grid[b / n][b % n]; });

        DSU dsu(n * n);
        vector<char> active(n * n, 0);
        int start = 0;
        int target = n * n - 1;

        for (int id : order)
        {
            int r = id / n;
            int c = id % n;
            active[id] = 1;
            // 只连已激活的邻居：未激活的格子高度更高，水位还没到
            for (int k = 0; k < 4; ++k)
            {
                int nr = r + DR[k];
                int nc = c + DC[k];
                if (nr < 0 || nr >= n || nc < 0 || nc >= n) continue;
                int nid = nr * n + nc;
                if (active[nid]) dsu.unite(id, nid);
            }
            // 第一次连通时，当前格子的高度就是最小可行水位
            if (active[start] && active[target] &&
                dsu.find(start) == dsu.find(target))
                return grid[r][c];
        }
        return grid[n - 1][n - 1];
    }
};

namespace
{
void check(vector<vector<int>> grid, int want)
{
    vector<vector<int>> g1 = grid;
    vector<vector<int>> g2 = grid;
    vector<vector<int>> g3 = grid;
    assert(Solution1().swimInWater(g1) == want);
    assert(Solution2().swimInWater(g2) == want);
    assert(Solution3().swimInWater(g3) == want);
}
} // namespace

int main()
{
    // 示例 1
    check({{0, 2}, {1, 3}}, 3);

    // 示例 2：绕一圈的 5x5，答案 16
    check({{0, 1, 2, 3, 4},
           {24, 23, 22, 21, 5},
           {12, 13, 14, 15, 16},
           {11, 17, 18, 19, 20},
           {10, 9, 8, 7, 6}},
          16);

    // 边界：单格棋盘，起点即终点
    check({{0}}, 0);

    // 边界：起点高度最大，答案由起点决定
    check({{3, 2}, {0, 1}}, 3);

    // 两条路都是最大值 3
    check({{0, 1}, {2, 3}}, 3);

    // 绕行的 3x3：沿 0→1→3→4→6 走，最大高度 6
    check({{0, 1, 5}, {2, 3, 4}, {8, 7, 6}}, 6);

    cout << "all tests passed" << endl;
    return 0;
}
