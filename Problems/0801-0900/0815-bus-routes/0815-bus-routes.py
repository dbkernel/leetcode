"""
============================================================================
815. 公交路线（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/bus-routes/
分类：广度优先搜索 · 图建模 / 倒排索引

给你一个数组 routes ，表示一系列公交线路，其中每个 routes[i] 表示一条
公交线路，第 i 辆公交车将会在上面循环行驶。

- 例如，路线 routes[0] = [1, 5, 7] 表示第 0 辆公交车会一直按序列
  1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... 这样的车站路线行驶。

现在从 source 车站出发（初始时不在公交车上），要前往 target 车站。
期间仅可乘坐公交车。

求出 最少乘坐的公交车数量 。如果不可能到达终点车站，返回 -1 。

示例 1：
输入：routes = [[1,2,7],[3,6,7]], source = 1, target = 6
输出：2
解释：最优策略是先乘坐第一辆公交车到达车站 7 , 然后换乘第二辆公交车到
车站 6 。

示例 2：
输入：routes = [[7,12],[4,5,15],[6],[15,19],[9,12,13]],
      source = 15, target = 12
输出：-1

提示：
- 1 <= routes.length <= 500.
- 1 <= routes[i].length <= 10^5
- routes[i] 中的所有值 互不相同
- sum(routes[i].length) <= 10^5
- 0 <= routes[i][j] < 10^6
- 0 <= source, target < 10^6
"""
from collections import deque


"""
解法：以路线为节点的 BFS + 站点倒排索引【最优解】

为什么慢：若按"站点"建图，就要判断任意两个站点是否被同一条线路覆盖——枚举
所有线路对求交是 O(r^2·len) 量级；或者对每条线路把站点两两连边，一条长 10^5
的线路会造出 10^10 条边。可题目关心的只是"换乘几次"，不是站与站之间走了多远。

换个角度：把每条公交线路看成一个节点，两条线路有公共站点就在它们之间连一条
边，边权为 1（换乘一次）。"最少坐几辆车"于是变成：从包含 source 的线路出发，
走到包含 target 的线路的最短距离 + 1。再用倒排索引 站点 → 线路编号 集合，
"当前线路能换乘到哪些线路"只需扫描这条线路的站点，总代价正比于
Σ routes[i].length，彻底省掉了 O(r^2) 的建图。

为什么这里必须用 BFS 而不是 DFS：图中每条边的代价都是 1（换乘一次），要求的
是最少换乘次数，即无权图最短路。DFS 第一次摸到 target 的那条路径未必换乘最少，
要保证最少就得把所有线路都搜完，层数信息完全用不上。

关键点：
  · source == target 直接返回 0：题目允许一站都不坐，这是唯一"0 辆车"的情形
  · 标记的是"线路"而不是"站点"：同一辆环形车可以反复坐，但重复上同一辆车
    不会减少换乘次数，只会让队列膨胀
  · 层号就是"已经坐过的车数"：第 1 层是 source 能直接上的车，展开到 target
    时返回当前层号即可
  · 站点只在第一次遇到时展开它的线路列表，否则一个枢纽站会被反复扫描
  · Python 用 `deque` 逐层弹出，`station_seen` 与 `route_used` 是两张独立的表

手推一遍：
以示例 1（routes = [[1,2,7],[3,6,7]]，source = 1，target = 6）为例：
  倒排：1→{0}，2→{0}，7→{0,1}，3→{1}，6→{1}
  第 1 层：{线路 0}（已坐 1 辆）
  展开线路 0：站点 1、2、7 都不是 6；站点 7 的列表里有线路 1，入队
  第 2 层：{线路 1}，它的站点 3、6 里出现 6，返回 2 ✔

复杂度分析：
  时间 O(Σ routes[i].length)：建倒排索引一遍，每条线路只出队一次，每个站点的
    线路列表只在第一次遇到它时扫一遍
  空间 O(Σ routes[i].length)：倒排索引 + 线路访问标记 + 队列
"""


class Solution:
    def numBusesToDestination(self, routes: list, source: int,
                              target: int) -> int:
        # 不上车即可到达，坐车数自然是 0
        if source == target:
            return 0

        # 倒排索引：站点 → 经过它的所有线路编号
        at = {}
        for bus, line in enumerate(routes):
            for station in line:
                at.setdefault(station, []).append(bus)

        # 起点或终点根本不在这张路网里
        if source not in at or target not in at:
            return -1

        route_used = [False] * len(routes)  # 线路只上一次车
        station_seen = set()                # 站点的线路列表只扫一次
        queue = deque(at[source])
        for bus in at[source]:
            route_used[bus] = True

        buses = 1  # 队列里这一层的线路，都是"坐第 buses 辆车"能到的
        while queue:
            layer_size = len(queue)
            for _ in range(layer_size):
                bus = queue.popleft()

                for station in routes[bus]:
                    # 当前这辆车就能直达终点，不必再换乘
                    if station == target:
                        return buses

                    # 该站的线路早已全部入队，重复扫描只是白费时间
                    if station in station_seen:
                        continue
                    station_seen.add(station)

                    for next_bus in at[station]:
                        if route_used[next_bus]:
                            continue
                        route_used[next_bus] = True
                        queue.append(next_bus)

            buses += 1

        # 队列空了还没碰到终点，说明路网不连通
        return -1


def check(routes: list, source: int, target: int, want: int) -> None:
    got = Solution().numBusesToDestination(routes, source, target)
    assert got == want, (routes, source, target, got, want)


if __name__ == "__main__":
    # 示例 1：先坐 0 号线到 7，再换 1 号线到 6
    check([[1, 2, 7], [3, 6, 7]], 1, 6, 2)

    # 示例 2：1 不在任何线路上，15 也到不了 12
    check([[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]], 15, 12, -1)

    # 边界：起终点相同，不需要坐车
    check([[1, 2, 7], [3, 6, 7]], 1, 1, 0)

    # 边界：起终点相同，但该站根本不在路网里，仍然是 0
    check([[1, 2]], 99, 99, 0)

    # 边界：一次换乘
    check([[1, 5], [5, 9]], 1, 9, 2)

    # 边界：三辆车接力
    check([[1, 2], [2, 3], [3, 4]], 1, 4, 3)

    # 边界：一条线路直达
    check([[1, 2, 3]], 1, 3, 1)

    # 边界：起点不在任何线路上
    check([[1, 2]], 5, 1, -1)

    # 边界：终点不在任何线路上
    check([[1, 2]], 1, 9, -1)

    # 边界：路网分成互不相交的两团
    check([[1, 2], [8, 9]], 1, 9, -1)

    print("all tests passed")
