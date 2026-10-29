"""
============================================================================
207. 课程表（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/course-schedule/
分类：图 · 拓扑排序（判环）

你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。

在选修某些课程之前需要一些先修课程。先修课程按数组 prerequisites 给出，
其中 prerequisites[i] = [ai, bi] ，表示如果要学习课程 ai 则 必须 先学习
课程 bi 。

- 例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。

请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回
false 。

示例 1：
输入：numCourses = 2, prerequisites = [[1,0]]
输出：true
解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。

示例 2：
输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
输出：false
解释：总共有 2 门课程。学习课程 1 之前，你需要先完成课程 0 ；并且学习
课程 0 之前，你还应先完成课程 1 。这是不可能的。

提示：
- 1 <= numCourses <= 10^5
- 0 <= prerequisites.length <= 5000
- prerequisites[i].length == 2
- 0 <= ai, bi < numCourses
- prerequisites 中的所有课程对 互不相同
"""
import sys
from collections import deque

"""
解法一：BFS 拓扑排序（入度数组）【最优解】

为什么慢：判断“能不能学完”最直接的做法是反复扫描整张先修表，每轮挑出
“前置都已完成”的课删掉，删到删不动为止。每轮都要重扫一遍边表，最坏
O(V·E)，而且“删不动”只告诉你结果，不告诉你卡在哪里。

换个角度：一门课此刻能不能学，只取决于“它还有几门前置没完成”，也就是
入度是否为 0。把入度当成就绪信号：学完一门就给它指向的课入度减一，减到
0 就立刻进就绪队列。这就是拓扑排序，每条边只处理一次。

关键点：
  · 边按「先修课 -> 后续课」定向：prerequisites[i] = [a, b] 是 b -> a，
    方向反了会把入度加到错误的节点上
  · 先把所有入度为 0 的课入队，这是拓扑排序唯一的入口
  · 入度只能在出队时减，不能在入队前预减 —— 减到 0 才说明前置全学完
  · 判环不看队列空不空，看“出队计数 == numCourses”：剩下的课互相
    等待，永远进不了队列
  · Python 用 deque 做队列，list.pop(0) 是 O(n) 会拖垮整体复杂度

以示例 2（numCourses = 2，prerequisites = [[1,0],[0,1]]）为例：
  0、1 的入度都是 1，队列一开始就是空的
  studied = 0，与 2 不等 → 返回 false ✔

复杂度分析：
  时间 O(V+E)：每门课入队出队各一次，每条先修边被访问一次
  空间 O(V+E)：邻接表存 E 条边，另有入度数组与队列各 O(V)

解法二：DFS 三色标记判环【最优解，与解法一并列】

为什么慢：反复删点同样是 O(V·E)。而“有环”等价于“DFS 时走到一个还挂在
当前递归链上的节点”，这份信息一次 DFS 就能拿到，不必反复扫描。

换个角度：给每个节点三种状态 —— 0 未访问、1 在递归栈上、2 已完成。
只有回到状态 1 的节点才构成环；回到状态 2 的节点只是走进了一条已经
验证过无环的分支，直接跳过。

关键点：
  · 状态 1 在递归进入时置上、退出时改成 2，两者顺序不能颠倒
  · 遇到状态 1 立刻返回 false：这条边指回了当前递归链
  · 遇到状态 2 直接返回 true：这个分支已证明无环，重复走只是浪费
  · 每个未访问节点都要起一次 DFS：先修图不一定连通
  · 递归深度可达 O(V)，默认 1000 层限制会先报错，需放宽上限

以示例 2（numCourses = 2，prerequisites = [[1,0],[0,1]]）为例：
  从 0 进入：state[0] = 1，边 0 -> 1 递归进去
  从 1 进入：state[1] = 1，边 1 -> 0 发现 state[0] == 1 → false ✔

复杂度分析：
  时间 O(V+E)：每个节点只进入一次，每条边只检查一次
  空间 O(V+E)：邻接表 O(V+E)，状态数组与递归栈各 O(V)

两解法的复杂度同为 O(V+E)，并列最优。取舍：解法一不用递归，长链输入
不会爆栈，常数也更小，工程上优先；解法二省掉入度数组，但递归深度
最坏 O(V)。两者对“有向图是否存在环”判定等价。
"""


class Solution1:
    def canFinish(self, numCourses: int, prerequisites: list) -> bool:
        nxt = [[] for _ in range(numCourses)]
        indegree = [0] * numCourses
        for course, pre in prerequisites:
            # 边指向“学完之后才能学的课”，BFS 才能顺着学习方向推进
            nxt[pre].append(course)
            indegree[course] += 1

        ready = deque(i for i in range(numCourses) if indegree[i] == 0)
        studied = 0
        while ready:
            cur = ready.popleft()
            studied += 1
            for child in nxt[cur]:
                # 前置课此时已完成，才允许把它的后继入度减一
                indegree[child] -= 1
                if indegree[child] == 0:
                    ready.append(child)

        # 有课始终进不了队列，说明它们互相等待，构成环
        return studied == numCourses


class Solution2:
    def canFinish(self, numCourses: int, prerequisites: list) -> bool:
        # 递归深度最坏等于课程数，默认上限 1000 不够用
        sys.setrecursionlimit(max(1000, numCourses * 2 + 100))
        nxt = [[] for _ in range(numCourses)]
        for course, pre in prerequisites:
            nxt[pre].append(course)

        # 0 未访问，1 在当前递归链上，2 已完成
        state = [0] * numCourses

        def no_cycle(cur: int) -> bool:
            if state[cur] == 1:
                return False  # 回到链上节点，成环
            if state[cur] == 2:
                return True  # 该分支已确认无环
            state[cur] = 1
            for child in nxt[cur]:
                # 沿学习方向深入，任一条分支成环就整体失败
                if not no_cycle(child):
                    return False
            state[cur] = 2
            return True

        return all(no_cycle(i) for i in range(numCourses))


# ===== 自测辅助：两个解法必须给出同一答案 =====
def can_finish_both(n: int, pre: list) -> bool:
    by_bfs = Solution1().canFinish(n, pre)
    by_dfs = Solution2().canFinish(n, pre)
    assert by_bfs == by_dfs, f"n={n}: bfs={by_bfs}, dfs={by_dfs}"
    return by_bfs


def check(n: int, pre: list, want: bool) -> None:
    assert can_finish_both(n, pre) == want, f"n={n}, pre={pre}"


if __name__ == "__main__":
    # 示例 1：只有一条先修边，可以学完
    check(2, [[1, 0]], True)

    # 示例 2：0 与 1 互为前置，构成环
    check(2, [[1, 0], [0, 1]], False)

    # 边界：没有先修课，按任意顺序都能学完
    check(3, [], True)

    # 边界：只有一门课，不必等任何人
    check(1, [], True)

    # 反直觉：课程依赖自己也算环，入度永远不会归零
    check(1, [[0, 0]], False)

    # 边界：链式依赖 0 -> 1 -> 2
    check(3, [[1, 0], [2, 1]], True)

    # 边界：三节点环 0 -> 1 -> 2 -> 0
    check(3, [[0, 1], [1, 2], [2, 0]], False)

    # 边界：环只占图的一部分，仍判定为不可能学完
    check(4, [[1, 0], [2, 1], [1, 2]], False)

    # 边界：菱形依赖，两条分支都要修完才轮到 3
    check(4, [[1, 0], [2, 0], [3, 1], [3, 2]], True)

    print("all tests passed")
