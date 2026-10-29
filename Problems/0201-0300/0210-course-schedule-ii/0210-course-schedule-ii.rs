/*
============================================================================
210. 课程表 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/course-schedule-ii/
分类：图 · 拓扑排序（输出顺序）

现在你总共有 numCourses 门课需要选，记为 0 到 numCourses - 1。给你一个
数组 prerequisites ，其中 prerequisites[i] = [ai, bi] ，表示在选修课程
ai 前 必须 先选修 bi 。

- 例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示：[0,1] 。

返回你为了学完所有课程所安排的学习顺序。可能会有多个正确的顺序，你只要
返回 任意一种 就可以了。如果不可能完成所有课程，返回 一个空数组 。

示例 1:
输入：numCourses = 2, prerequisites = [[1,0]]
输出：[0,1]
解释：总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的
课程顺序为 [0,1] 。

示例 2:
输入：numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
输出：[0,2,1,3]
解释：总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且
课程 1 和课程 2 都应该排在课程 0 之后。
因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。

示例 3:
输入：numCourses = 1, prerequisites = []
输出：[0]

提示：
- 1 <= numCourses <= 2000
- 0 <= prerequisites.length <= numCourses * (numCourses - 1)
- prerequisites[i].length == 2
- 0 <= ai, bi < numCourses
- ai != bi
- prerequisites 中的所有课程对 互不相同
*/
use std::collections::VecDeque;

/*
解法一：BFS 拓扑排序，出队即入答案【最优解】

为什么慢：想拿到“一个合法顺序”可以反复扫描边表，每轮找出一门前置都
已完成的课追加到答案里，扫到没有新课可加为止。每轮 O(E)，最多 V 轮，
最坏 O(V·E)；而且一旦剩下的课互相等待，还得再判断一次是“加完了”还是
“卡住了”。

换个角度：复用课程表 I 的入度思路，唯一的区别是把“计数”换成“记录
顺序”。入度归零的那一刻，这门课的所有前置都已排进答案，它可以安全地
追加到队尾 —— 出队顺序本身就是拓扑序。

关键点：
  · 出队时写入答案，不要入队时写：入队只代表“前置刚够”，出队才代表
    它被正式安排
  · 结尾必须检查答案长度是否等于 numCourses：有环时 BFS 只排出环外的
    部分课，题目要求这种情况返回空数组，不能交半截答案
  · 边方向仍是「先修课 -> 后续课」

以示例 2（numCourses = 4，prerequisites = [[1,0],[2,0],[3,1],[3,2]]）
为例：
  入度 0:0、1:1、2:1、3:2，初始队列 [0]
  出队 0 → 答案 [0]，1、2 的入度归零入队
  出队 1 → 答案 [0,1]，3 的入度降到 1，暂不入队
  出队 2 → 答案 [0,1,2]，3 的入度归零入队
  出队 3 → 答案 [0,1,2,3]，长度 4 == numCourses ✔

复杂度分析：
  时间 O(V+E)：每门课出队一次，每条先修边被访问一次
  空间 O(V+E)：邻接表 O(V+E)，入度数组、队列、答案各 O(V)

解法二：DFS 后序逆序【最优解，与解法一并列】

为什么慢：与解法一同样可以边删边排，代价 O(V·E)。而拓扑序天然对应
“所有后继都处理完之后才记录自己”的顺序，这正是后序遍历。

换个角度：DFS 时把节点在“后序位置”（它的所有后继都已处理完）追加到
列表，得到的顺序保证每条边 a -> b 中 b 先于 a 出现，也就是逆拓扑序；
整体翻转一次就是拓扑序。

关键点：
  · 必须在后序位置追加，先序追加会让后继排到前面，顺序反了
  · 最后整体 reverse 一次，不要试图在递归里插到头部（那是 O(n^2)）
  · 三色标记同样用于判环：命中状态 1 立刻整体返回空数组
  · 判环失败后答案已收集一半，必须丢弃，不能 return 现有列表

以示例 2（numCourses = 4，prerequisites = [[1,0],[2,0],[3,1],[3,2]]）
为例：
  dfs(0) → 进 1 → 进 3（3 无后继，追加 3）→ 追加 1 → 进 2（3 已完成，
  跳过）→ 追加 2 → 追加 0
  后序列表 [3,1,2,0]，翻转得到 [0,2,1,3] ✔（与示例给出的另一种答案一致）

复杂度分析：
  时间 O(V+E)：每个节点进入一次，每条边检查一次，翻转 O(V)
  空间 O(V+E)：邻接表 O(V+E)，状态数组、后序列表、递归栈各 O(V)

两解法时间同为 O(V+E)，并列最优。取舍：解法一不需要递归，长链输入
不会爆栈；解法二少维护一个入度数组，但递归深度最坏 O(V)，且多一次
翻转。答案不唯一，两者的输出不同是正常现象。
*/

// rustc 单文件编译时平台上的 Solution 不存在，这里显式定义结构体
pub struct Solution1;

impl Solution1 {
    pub fn find_order(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = num_courses as usize;
        let mut next = vec![Vec::new(); n];
        let mut indegree = vec![0usize; n];
        for p in &prerequisites {
            let (course, pre) = (p[0] as usize, p[1] as usize);
            next[pre].push(course);
            indegree[course] += 1;
        }

        let mut ready = VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                ready.push_back(i);
            }
        }

        let mut order = Vec::with_capacity(n);
        while let Some(cur) = ready.pop_front() {
            // 出队时刻前置全部就位，此时追加才是合法顺序
            order.push(cur as i32);
            for &child in &next[cur] {
                indegree[child] -= 1;
                if indegree[child] == 0 {
                    ready.push_back(child);
                }
            }
        }

        // 长度不足说明剩下的课互相等待，题目要求整题返回空数组
        if order.len() != n {
            return Vec::new();
        }
        order
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn find_order(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = num_courses as usize;
        let mut next = vec![Vec::new(); n];
        for p in &prerequisites {
            next[p[1] as usize].push(p[0] as usize);
        }

        let mut state = vec![0u8; n];
        let mut post = Vec::with_capacity(n);
        for i in 0..n {
            if !visit(&next, &mut state, i, &mut post) {
                // 判环失败后已收集的节点作废，直接交空数组
                return Vec::new();
            }
        }
        post.reverse();
        post.into_iter().map(|v| v as i32).collect()
    }
}

fn visit(
    next: &[Vec<usize>],
    state: &mut [u8],
    cur: usize,
    post: &mut Vec<usize>,
) -> bool {
    if state[cur] == 1 {
        return false; // 成环，整题无解
    }
    if state[cur] == 2 {
        return true;
    }
    state[cur] = 1;
    for &child in &next[cur] {
        if !visit(next, state, child, post) {
            return false;
        }
    }
    state[cur] = 2;
    // 后继全部落位后才记录自己，翻转后它就是逆序的拓扑序
    post.push(cur);
    true
}

// ===== 自测辅助：题意只要求任意合法顺序，故只校验合法性 =====
fn valid_order(n: usize, pre: &[Vec<i32>], order: &[i32]) -> bool {
    if order.len() != n {
        return false;
    }
    let mut pos = vec![-1i32; n];
    for (i, &course) in order.iter().enumerate() {
        // 越界或重复都说明不是 0..n-1 的一个排列
        if course < 0 || course as usize >= n || pos[course as usize] != -1 {
            return false;
        }
        pos[course as usize] = i as i32;
    }
    for p in pre {
        // 先修课必须严格排在依赖它的课之前
        if pos[p[1] as usize] >= pos[p[0] as usize] {
            return false;
        }
    }
    true
}

fn check(n: i32, pre: &[Vec<i32>], solvable: bool) {
    let by_bfs = Solution1::find_order(n, pre.to_vec());
    let by_dfs = Solution2::find_order(n, pre.to_vec());
    if !solvable {
        // 有环时两个解法都必须交空数组，不能交半截序列
        assert!(by_bfs.is_empty() && by_dfs.is_empty(), "n={} 有环却非空", n);
        return;
    }
    assert!(
        valid_order(n as usize, pre, &by_bfs),
        "n={} pre={:?} bfs={:?}",
        n,
        pre,
        by_bfs
    );
    assert!(
        valid_order(n as usize, pre, &by_dfs),
        "n={} pre={:?} dfs={:?}",
        n,
        pre,
        by_dfs
    );
}

fn main() {
    // 示例 1：唯一解 [0,1]
    check(2, &[vec![1, 0]], true);

    // 示例 2：答案不唯一，只校验先修关系
    check(4, &[vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], true);

    // 示例 3：只有一门课，答案必须是 [0]
    check(1, &[], true);

    // 边界：没有先修课，任意排列都合法
    check(3, &[], true);

    // 边界：两门课互为前置，无解
    check(2, &[vec![1, 0], vec![0, 1]], false);

    // 边界：三节点环
    check(3, &[vec![0, 1], vec![1, 2], vec![2, 0]], false);

    // 边界：环外还有可排的课，仍必须整体返回空数组
    check(4, &[vec![1, 0], vec![2, 1], vec![1, 2]], false);

    // 边界：一门课有两个前置，必须都排在它前面
    check(3, &[vec![2, 0], vec![2, 1]], true);

    // 边界：长链 0 -> 1 -> 2 -> 3 -> 4，答案是唯一排列
    check(5, &[vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3]], true);

    println!("all tests passed");
}
