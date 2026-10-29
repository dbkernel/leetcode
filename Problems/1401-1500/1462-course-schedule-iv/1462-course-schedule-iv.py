"""
============================================================================
1462. 课程表 IV（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/course-schedule-iv/
分类：图 · 传递闭包（拓扑 / DFS）

你总共需要上 numCourses 门课，课程编号依次为 0 到 numCourses-1 。你会得到
一个数组 prerequisite ，其中 prerequisites[i] = [a_i, b_i] 表示如果你想选
b_i 课程，你 必须 先选 a_i 课程。

有的课会有直接的先修课程，比如如果想上课程 1 ，你必须先上课程 0 ，
  那么会以 [0,1] 数对的形式给出先修课程数对。

先决条件也可以是 间接 的。如果课程 a 是课程 b 的先决条件，课程 b 是课程 c
的先决条件，那么课程 a 就是课程 c 的先决条件。

你也得到一个数组 queries ，其中 queries[j] = [u_j, v_j]。对于第 j 个查询，
您应该回答课程 u_j 是否是课程 v_j 的先决条件。

返回一个布尔数组 answer ，其中 answer[j] 是第 j 个查询的答案。

示例 1：
输入：numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
输出：[false,true]
解释：[1, 0] 数对表示在你上课程 0 之前必须先上课程 1。
课程 0 不是课程 1 的先修课程，但课程 1 是课程 0 的先修课程。

示例 2：
输入：numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
输出：[false,false]
解释：没有先修课程对，所以每门课程之间是独立的。

示例 3：
输入：numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
输出：[true,true]

提示：
- 2 <= numCourses <= 100
- 0 <= prerequisites.length <= (numCourses * (numCourses - 1) / 2)
- prerequisites[i].length == 2
- 0 <= a_i, b_i <= numCourses - 1
- a_i != b_i
- 每一对 [a_i, b_i] 都 不同
- 先修课程图中没有环。
- 1 <= queries.length <= 10^4
- 0 <= u_i, v_i <= numCourses - 1
- u_i != v_i
"""
from collections import deque
from typing import List


"""
解法一：每个查询单独搜一次（暴力）【非最优】

为什么慢：查询之间互不相关，但它们问的是同一张图上的可达性。对每个查询
从 u 出发做一次 DFS/拓扑，同一段路径会被反复走：查询数最多 10^4，图只有
100 个点、边最多 4950 条，重复搜索是纯浪费。

换个角度：所有查询共用一张「谁是谁的先修」的全源表 is_pre[V][V]，离线算
一次，之后每个查询只查一格。

关键点：
  · 这是「离线」预处理：先把全源可达算出来，查询阶段只做 O(1) 查表
  · 单次搜索只需沿出边走，「谁能到达 v」与「v 能到达谁」是两个方向，
    这里问的是 u 能否到达 v，只能沿着 u 的出边搜
  · u == v 要先返回 false：课程不是自己的先修课（题面保证不会问，
    但语义上必须这样定，否则会把起点自己判成可达）

复杂度分析：
  时间 O(Q·(V+E))：每个查询一次遍历，V 为课程数，E 为先修关系数
  空间 O(V+E)：邻接表 + 每次查询的 visited 与栈
  和后面两个解法的分界：只有当 Q 远小于 V（查询极少）时逐查询才更省；
  题面允许 Q 达到 10^4 而 V 不超过 100，重复搜索的代价远超一次预处理

解法二：拓扑排序 + 传递矩阵【最优解】

为什么慢：解法一的瓶颈是「同一段可达信息被算了很多遍」。

换个角度：在拓扑序上，一个点的所有先修点一定先于它被处理完，于是可以把
「谁是 ne 的先修」整列从 cur 那里并过来，一次把 V 个格子全部定稿。

关键点：
  · cur 出队时遍历它的每条出边 cur -> ne，做两件事：is_pre[cur][ne] = True，
    再把 cur 那一列整列或进 ne 那一列（is_pre[i][ne] |= is_pre[i][cur]）
  · 必须按拓扑序处理 cur：出队时 cur 的入度已降为 0，它的整列已经完整，
    此时并过去才不会漏掉间接先修
  · 并入用的是「列」（第一个下标自由、第二个下标固定为 ne），
    写反成行就等于把方向搞反，[0,1] 会答成「1 是 0 的先修」

复杂度分析：
  时间 O(V·E + Q)：每条边处理一次，每次整列或运算 O(V)；查询 O(1)
  空间 O(V^2 + E)：传递矩阵 V^2，邻接表 E

解法三：DFS 求传递闭包【最优解，与解法二并列】

换个角度：和拓扑排序是同一件事的另一种实现——把「先递归完所有后继，再
把自己的行并上后继的行」当成后序动作，递归返回时该点的行就已经完整。

关键点：
  · 并按行（is_pre[cur][i] |= is_pre[ne][i]），与解法二的按列并入互为镜像
  · done 数组是记忆化：某个点的行只在第一次进入时计算，之后直接复用；
    由于图无环，「进入时置位」不会让行在半成品状态下被别人读到
  · 先 dfs(ne)，再 is_pre[cur][ne] = True，顺序反过来会读到 ne 的半成品行

复杂度分析：
  时间 O(V·E + Q)：每个点只算一次行，每算一行要做 outdeg 次长度 V 的或
  空间 O(V^2 + E)：传递矩阵 + 递归栈 O(V)

两种最优解的取舍：解法二不递归、不依赖栈深度；解法三少一个入度数组，
代码更短，但递归深度可达 V。V <= 100 时两者都远在限制内。
"""


class Solution1:
    def checkIfPrerequisite(
        self,
        numCourses: int,
        prerequisites: List[List[int]],
        queries: List[List[int]],
    ) -> List[bool]:
        graph: List[List[int]] = [[] for _ in range(numCourses)]
        for a, b in prerequisites:
            graph[a].append(b)

        # 沿出边搜索 frm 能否到达 to
        def reachable(frm: int, to: int) -> bool:
            # 课程不是自己的先修课，起点即终点也必须答 false
            if frm == to:
                return False
            seen = [False] * numCourses
            stack = [frm]
            seen[frm] = True
            while stack:
                cur = stack.pop()
                for nxt in graph[cur]:
                    if nxt == to:
                        return True
                    if seen[nxt]:
                        continue
                    seen[nxt] = True
                    stack.append(nxt)
            return False

        return [reachable(u, v) for u, v in queries]


class Solution2:
    def checkIfPrerequisite(
        self,
        numCourses: int,
        prerequisites: List[List[int]],
        queries: List[List[int]],
    ) -> List[bool]:
        graph: List[List[int]] = [[] for _ in range(numCourses)]
        indegree = [0] * numCourses
        for a, b in prerequisites:
            graph[a].append(b)
            indegree[b] += 1

        # is_pre[x][y]：x 是 y 的直接或间接先修课
        is_pre = [[False] * numCourses for _ in range(numCourses)]
        ready = deque(i for i in range(numCourses) if indegree[i] == 0)

        while ready:
            cur = ready.popleft()
            for nxt in graph[cur]:
                is_pre[cur][nxt] = True
                # cur 出队时其整列已完整，整列或进 nxt 不会漏间接先修
                for i in range(numCourses):
                    is_pre[i][nxt] = is_pre[i][nxt] or is_pre[i][cur]
                indegree[nxt] -= 1
                if indegree[nxt] == 0:
                    ready.append(nxt)

        return [is_pre[u][v] for u, v in queries]


class Solution3:
    def checkIfPrerequisite(
        self,
        numCourses: int,
        prerequisites: List[List[int]],
        queries: List[List[int]],
    ) -> List[bool]:
        graph: List[List[int]] = [[] for _ in range(numCourses)]
        for a, b in prerequisites:
            graph[a].append(b)

        is_pre = [[False] * numCourses for _ in range(numCourses)]
        done = [False] * numCourses

        # 后序求 cur 的整行；done 为真时该行已经完整
        def dfs(cur: int) -> None:
            if done[cur]:
                return
            done[cur] = True
            for nxt in graph[cur]:
                dfs(nxt)
                is_pre[cur][nxt] = True
                # 按行并入：cur 能到的点，以及 nxt 能到的点
                for i in range(numCourses):
                    is_pre[cur][i] = is_pre[cur][i] or is_pre[nxt][i]

        for i in range(numCourses):
            dfs(i)

        return [is_pre[u][v] for u, v in queries]


def check(
    numCourses: int,
    prerequisites: List[List[int]],
    queries: List[List[int]],
    want: List[bool],
) -> None:
    """同一组期望值给三个解法共用"""
    solvers = (Solution1(), Solution2(), Solution3())
    for index, solver in enumerate(solvers, start=1):
        got = solver.checkIfPrerequisite(numCourses, prerequisites, queries)
        assert got == want, f"solution {index}: {got} != {want}"


if __name__ == "__main__":
    # 示例 1
    check(2, [[1, 0]], [[0, 1], [1, 0]], [False, True])

    # 示例 2：没有任何先修关系
    check(2, [], [[1, 0], [0, 1]], [False, False])

    # 示例 3
    check(3, [[1, 2], [1, 0], [2, 0]], [[1, 0], [1, 2]], [True, True])

    # 边界：单点课程图（题面保证 >= 2，这里只验证健壮性）
    check(1, [], [], [])

    # 反直觉：问的是间接先修 —— 一条 4 级链上首尾必须可达
    check(4, [[0, 1], [1, 2], [2, 3]], [[0, 3], [3, 0], [2, 0]],
          [True, False, False])

    # 反直觉：拓扑序里 0 和 1 都是根，2 同时有两个先修，整列都要并上
    check(5, [[0, 2], [1, 2], [2, 3], [3, 4]], [[0, 4], [1, 4], [4, 0]],
          [True, True, False])

    # 反直觉：菱形两条路径，0 到 3 只能靠传递得到
    check(4, [[0, 1], [0, 2], [1, 3], [2, 3]], [[0, 3], [3, 0]], [True, False])

    # 反直觉：两条互不相连的边不能被串起来
    check(4, [[0, 1], [2, 3]], [[0, 3], [2, 1], [0, 1]], [False, False, True])

    print("all tests passed")
