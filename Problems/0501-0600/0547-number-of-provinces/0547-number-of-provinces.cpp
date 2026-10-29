/*
============================================================================
547. 省份数量（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/number-of-provinces/
分类：并查集 · 连通块计数

有 n 个城市，其中一些彼此相连，另一些没有相连。如果城市 a 与城市 b
直接相连，且城市 b 与城市 c 直接相连，那么城市 a 与城市 c 间接相连。

省份 是一组直接或间接相连的城市，组内不含其他没有相连的城市。

给你一个 n x n 的矩阵 isConnected ，其中 isConnected[i][j] = 1 表示
第 i 个城市和第 j 个城市直接相连，而 isConnected[i][j] = 0 表示二者
不直接相连。

返回矩阵中 省份 的数量。

示例 1：
输入：isConnected = [[1,1,0],[1,1,0],[0,0,1]]
输出：2

示例 2：
输入：isConnected = [[1,0,0],[0,1,0],[0,0,1]]
输出：3

提示：
- 1 <= n <= 200
- n == isConnected.length
- n == isConnected[i].length
- isConnected[i][j] 为 1 或 0
- isConnected[i][i] == 1
- isConnected[i][j] == isConnected[j][i]
*/
#include <algorithm>
#include <cassert>
#include <iostream>
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

    // 返回 false 表示 x、y 本就在同一集合，合并次数不应增加
    bool unite(int x, int y)
    {
        int rx = find(x);
        int ry = find(y);
        if (rx == ry) return false;
        if (size[rx] < size[ry]) swap(rx, ry);
        parent[ry] = rx;
        size[rx] += size[ry];
        return true;
    }
};
} // namespace

/*
解法一：并查集，连通块数 = n - 成功合并次数【最优解】

为什么慢：若把省份当成集合，最直观的做法是反复扫描所有城市对，一发现
相连就把一个集合里的元素逐个搬进另一个集合；每次搬运 O(n)，最坏 O(n^3)。
判断与合并混在一起，同一批城市会被反复处理。

换个角度：省份数就是无向图的连通块数。初始时 n 个城市各自成一个省，
并查集把“把两个城市并进同一个省”压缩成一次合并：每成功合并一次，省数
就减一，扫完所有相连关系后剩下的就是答案。

关键点：
  · 初始 ans = n，只有 unite 返回 true（两个不同集合真的合并）才 ans--，
    两端已在同一集合时返回 false，不能计数
  · 只扫上三角 j > i：矩阵对称，扫全阵会重复合并，答案虽对但白做功
  · isConnected[i][i] == 1 不必特判，i 与自身同集合，unite 返回 false
  · 记牢递推：连通块数 = n - 成功合并次数，合并 n-k 次后恰剩 k 块

手推一遍：以示例 1（isConnected = [[1,1,0],[1,1,0],[0,0,1]]）为例：
  初始 3 个省：{0}、{1}、{2}
  (0,1)=1：合并 → {0,1}、{2}，ans = 2
  (0,2)=0、(1,2)=0：跳过
  扫完剩余 2 个省 ✔

复杂度分析：
  时间 O(n^2 α(n))：上三角共约 n^2/2 次判断，每次合并均摊接近常数
  空间 O(n)：parent 与 size 各一个长度 n 的数组

解法二：DFS 洪水填充【最优解，与解法一并列】

为什么慢：并查集要维护 parent 并做合并，适合动态加边；本题的图一次性
给全，只需要数连通块，为每个点跑一遍 DFS 就够了，不必维护集合结构。

换个角度：从每个未访问的城市出发做一次 DFS，能走到的城市都与它同省；
每启动一次新的 DFS，省份数就加一。

关键点：
  · 外层遍历所有城市，只有遇到未访问的才 ans++ 并从它开始 DFS
  · 直接用 isConnected 矩阵当邻接表，不必另建图
  · 进入 DFS 立刻标记 visited，保证每个城市只被扩展一次
  · 递归深度最坏为 n（链状连通），本题 n ≤ 200 不会爆栈

手推一遍：仍以示例 1 为例：
  城市 0 未访问 → ans=1，DFS 走到 1，标记 {0,1}
  城市 1 已访问，跳过
  城市 2 未访问 → ans=2，DFS 无邻居
  结果 2 ✔

复杂度分析：
  时间 O(n^2)：每个城市扩展时都要扫自己那一行 n 个格子
  空间 O(n)：visited 数组 + 递归栈（最坏链状 n 层）

解法三：BFS 洪水填充【最优解，与解法一、二并列】

为什么慢：与 DFS 同理，本题只需数连通块；DFS 的递归栈在极深连通块上
有栈溢出风险，BFS 用显式队列把空间放到堆上。

换个角度：从每个未访问的城市出发做一次 BFS，队列里能扩散到的城市同属
一个省；每次出队后把邻接且未访问的城市入队。

关键点：
  · 起点入队时就标记 visited，避免同一城市被多个邻居重复入队
  · 外层仍是“遇到未访问才 ans++”，与解法二一致
  · 队列最坏装下整个连通块，空间仍是 O(n)

手推一遍：仍以示例 1 为例：
  城市 0 入队 → ans=1，弹出 0，把 1 入队；弹出 1，无新邻居
  城市 2 未访问 → ans=2
  结果 2 ✔

复杂度分析：
  时间 O(n^2)：每个城市出队一次，出队时扫一行 n 个格子
  空间 O(n)：visited 与队列
*/
class Solution1
{
public:
    int findCircleNum(vector<vector<int>> &isConnected)
    {
        int n = static_cast<int>(isConnected.size());
        DSU dsu(n);
        int ans = n; // 每个城市先各自成一个省
        for (int i = 0; i < n; ++i)
        {
            // 只扫上三角：对称位置会重复合并
            for (int j = i + 1; j < n; ++j)
            {
                if (isConnected[i][j] == 1 && dsu.unite(i, j)) --ans;
            }
        }
        return ans;
    }
};

class Solution2
{
public:
    int findCircleNum(vector<vector<int>> &isConnected)
    {
        int n = static_cast<int>(isConnected.size());
        vector<char> visited(n, 0);
        int ans = 0;
        for (int i = 0; i < n; ++i)
        {
            // 每启动一次新的 DFS，就多一个连通块
            if (visited[i]) continue;
            ++ans;
            dfs(isConnected, visited, i);
        }
        return ans;
    }

private:
    void dfs(const vector<vector<int>> &g, vector<char> &visited, int x)
    {
        visited[x] = 1;
        for (int y = 0; y < static_cast<int>(g.size()); ++y)
        {
            if (g[x][y] == 1 && !visited[y]) dfs(g, visited, y);
        }
    }
};

class Solution3
{
public:
    int findCircleNum(vector<vector<int>> &isConnected)
    {
        int n = static_cast<int>(isConnected.size());
        vector<char> visited(n, 0);
        vector<int> queue;
        int ans = 0;
        for (int i = 0; i < n; ++i)
        {
            if (visited[i]) continue;
            ++ans;
            // 入队即标记，避免同一城市被多个邻居重复入队
            visited[i] = 1;
            queue.push_back(i);
            size_t head = 0;
            while (head < queue.size())
            {
                int x = queue[head++];
                for (int y = 0; y < n; ++y)
                {
                    if (isConnected[x][y] == 1 && !visited[y])
                    {
                        visited[y] = 1;
                        queue.push_back(y);
                    }
                }
            }
            queue.clear();
        }
        return ans;
    }
};

namespace
{
void check(const vector<vector<int>> &isConnected, int want)
{
    vector<vector<int>> g1 = isConnected;
    vector<vector<int>> g2 = isConnected;
    vector<vector<int>> g3 = isConnected;
    assert(Solution1().findCircleNum(g1) == want);
    assert(Solution2().findCircleNum(g2) == want);
    assert(Solution3().findCircleNum(g3) == want);
}
} // namespace

int main()
{
    // 示例 1
    check({{1, 1, 0}, {1, 1, 0}, {0, 0, 1}}, 2);

    // 示例 2
    check({{1, 0, 0}, {0, 1, 0}, {0, 0, 1}}, 3);

    // 边界：只有一个城市
    check({{1}}, 1);

    // 边界：全连通，只有一个省
    check({{1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}}, 1);

    // 间接相连：0-1-2 同属一省，3 单独
    check({{1, 1, 0, 0}, {1, 1, 1, 0}, {0, 1, 1, 0}, {0, 0, 0, 1}}, 2);

    // 压力用例：200 个城市两两独立
    {
        vector<vector<int>> g(200, vector<int>(200, 0));
        for (int i = 0; i < 200; ++i) g[i][i] = 1;
        check(g, 200);
    }

    // 压力用例：200 个城市串成一条链，只有一个省
    {
        vector<vector<int>> g(200, vector<int>(200, 0));
        for (int i = 0; i < 200; ++i)
        {
            g[i][i] = 1;
            if (i + 1 < 200)
            {
                g[i][i + 1] = 1;
                g[i + 1][i] = 1;
            }
        }
        check(g, 1);
    }

    cout << "all tests passed" << endl;
    return 0;
}
