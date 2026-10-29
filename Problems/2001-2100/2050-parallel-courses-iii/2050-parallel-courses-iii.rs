/*
============================================================================
2050. 并行课程 III（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/parallel-courses-iii/
分类：图 · 拓扑序上求最长路（DP）

给你一个整数 n ，表示有 n 节课，课程编号从 1 到 n 。同时给你一个二维整数
数组 relations ，其中 relations[j] = [prevCourse_j, nextCourse_j] ，表示
课程 prevCourse_j 必须在课程 nextCourse_j 之前 完成（先修课的关系）。同时
给你一个下标从 0 开始的整数数组 time ，其中 time[i] 表示完成第 (i+1) 门
课程需要花费的 月份 数。

请你根据以下规则算出完成所有课程所需要的 最少 月份数：

· 如果一门课的所有先修课都已经完成，你可以在 任意 时间开始这门课程。
· 你可以 同时 上 任意门课程 。

请你返回完成所有课程所需要的 最少 月份数。

注意：测试数据保证一定可以完成所有课程（也就是先修课的关系构成一个有向
无环图）。

示例 1：
输入：n = 3, relations = [[1,3],[2,3]], time = [3,2,5]
输出：8
解释：上图展示了输入数据所表示的先修关系图，以及完成每门课程需要花费的
时间。你可以在月份 0 同时开始课程 1 和 2 。课程 1 花费 3 个月，课程 2 花费
2 个月。所以，最早开始课程 3 的时间是月份 3 ，完成所有课程所需时间为
3 + 5 = 8 个月。

示例 2：
输入：n = 5, relations = [[1,5],[2,5],[3,5],[3,4],[4,5]], time = [1,2,3,4,5]
输出：12
解释：上图展示了输入数据所表示的先修关系图，以及完成每门课程需要花费的
时间。你可以在月份 0 同时开始课程 1 ，2 和 3 。在月份 1，2 和 3 分别完成
这三门课程。课程 4 需在课程 3 之后开始，也就是 3 个月后。课程 4 在
3 + 4 = 7 月完成。课程 5 需在课程 1，2，3 和 4 之后开始，也就是在
max(1,2,3,7) = 7 月开始。所以完成所有课程所需的最少时间为 7 + 5 = 12 个月。

提示：
- 1 <= n <= 5 * 10^4
- 0 <= relations.length <= min(n * (n - 1) / 2, 5 * 10^4)
- relations[j].length == 2
- 1 <= prevCourse_j, nextCourse_j <= n
- prevCourse_j != nextCourse_j
- 所有的先修课程对 [prevCourse_j, nextCourse_j] 都是 互不相同 的。
- time.length == n
- 1 <= time[i] <= 10^4
- 先修课程图是一个有向无环图。
*/
use std::collections::VecDeque;

/*
解法一：记忆化搜索（DFS 求每门课的最早完成月份）【最优解】

为什么慢：如果按月份推演，每个月扫一遍所有课，时间是 O(n·总时长)，
而总时长可达 5·10^4 * 10^4 = 5·10^8。真正的问题在于「每门课什么时候
完成」只取决于它的先修课，与月份的具体数值无关。

换个角度：设 finish[v] 为课程 v 的完成月份，则
finish[v] = time[v] + max(finish[u])，u 取遍 v 的所有先修课。
这是一个在 DAG 上的最长路：求每条路径的耗时之和，答案取全局最大。

关键点：
  · 是「所有先修课完成时间的最大值 + 本课耗时」，不是求和 ——
    同层课程可以并行，求和等于把它们排成队
  · 叶子课（没有先修）的完成时间就是 time[v] 本身
  · 记忆化保证每门课只算一次；DAG 无环，递归不会回绕
  · 答案取所有课完成时间的最大值，而不是某一门课的完成时间
  · Rust 里把 memo 以 &mut [i64] 传进递归函数，避免为了共享状态写
    全局变量或 Rc<RefCell>，也避免 `!memo[v].is_empty()` 之类的哨兵

复杂度分析：
  时间 O(n + m)：每门课算一次，每条先修关系被看一次，m 为关系数
  空间 O(n + m)：邻接表 + 记忆化数组 + 递归栈（最坏是一条 n 长的链）

解法二：拓扑排序 + DP 顺手递推【最优解，与解法一并列】

换个角度：解法一的递归本质是在拓扑序上从后往前推。反过来按拓扑序从
前往后推，每门课一出队就把它已经定稿的完成时间传给所有后继。

关键点：
  · 后继 ne 的完成时间 = max(它的所有先修) + time[ne]，
    所以边 cur -> ne 出队时做 latest[ne] = max(latest[ne], latest[cur])
  · 入度减到 0 才入队，此时 ne 的所有先修都已经递推过，latest[ne] 定稿
  · 「加自身耗时」要放在出队时做，而不是在更新后继时做
  · 不用递归，链长 5·10^4 也不会爆栈

复杂度分析：
  时间 O(n + m)：每个点入队一次，每条边被看一次
  空间 O(n + m)：邻接表 + 入度数组 + 队列 + latest 数组

两者取舍：复杂度完全同阶，解法一代码更短；解法二省掉递归栈，链状输入
更稳。
*/

// 单文件 rustc 编译需要显式定义各解法，平台提交时只保留其中一个
pub struct Solution1;

impl Solution1 {
    // 契约：relations[j] = [先修, 后续]，课程编号 1..n，返回最少月份数
    pub fn minimum_time(
        n: i32,
        relations: Vec<Vec<i32>>,
        time: Vec<i32>,
    ) -> i32 {
        let n = n as usize;
        // prev[v]：v 的全部先修课
        let mut prev: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for r in &relations {
            prev[r[1] as usize].push(r[0] as usize);
        }

        let mut memo = vec![0i64; n + 1]; // 0 表示还没算过（耗时至少为 1）
        let mut answer = 0;
        for v in 1..=n {
            // 每门课都要算：孤立课程不进任何一条边，也要参与取最大
            answer = answer.max(Self::finish(v, &prev, &time, &mut memo));
        }
        answer as i32
    }

    // 课程 v 的最早完成月份
    fn finish(
        v: usize,
        prev: &[Vec<usize>],
        time: &[i32],
        memo: &mut [i64],
    ) -> i64 {
        if memo[v] != 0 {
            return memo[v];
        }

        // 先修可以并行，所以取最晚的那一门，而不是累加
        let mut latest = 0;
        for &p in &prev[v] {
            latest = latest.max(Self::finish(p, prev, time, memo));
        }
        memo[v] = latest + time[v - 1] as i64;
        memo[v]
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn minimum_time(
        n: i32,
        relations: Vec<Vec<i32>>,
        time: Vec<i32>,
    ) -> i32 {
        let n = n as usize;
        let mut next: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let mut indegree = vec![0usize; n + 1];
        for r in &relations {
            next[r[0] as usize].push(r[1] as usize);
            indegree[r[1] as usize] += 1;
        }

        // latest[v]：v 的所有先修里最晚的完成月份，出队时再加上自身耗时
        let mut latest = vec![0i64; n + 1];
        let mut ready: VecDeque<usize> =
            (1..=n).filter(|&v| indegree[v] == 0).collect();

        let mut answer = 0i64;
        while let Some(cur) = ready.pop_front() {
            // 此刻 cur 的先修全部递推过，latest[cur] 已是最终的最晚先修
            latest[cur] += time[cur - 1] as i64;
            answer = answer.max(latest[cur]);

            for &nx in &next[cur] {
                latest[nx] = latest[nx].max(latest[cur]);
                indegree[nx] -= 1;
                if indegree[nx] == 0 {
                    ready.push_back(nx);
                }
            }
        }
        answer as i32
    }
}

fn check(n: i32, relations: &[Vec<i32>], time: &[i32], want: i32) {
    let got1 = Solution1::minimum_time(n, relations.to_vec(), time.to_vec());
    let got2 = Solution2::minimum_time(n, relations.to_vec(), time.to_vec());
    assert_eq!(got1, want, "solution 1: {} != {}", got1, want);
    assert_eq!(got2, want, "solution 2: {} != {}", got2, want);
}

fn main() {
    // 示例 1：课程 1、2 并行，课程 3 取两者的最大完成时间
    check(3, &[vec![1, 3], vec![2, 3]], &[3, 2, 5], 8);

    // 示例 2：课程 5 有四个先修，最长的那条路是 3 -> 4 -> 5
    check(
        5,
        &[vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
        &[1, 2, 3, 4, 5],
        12,
    );

    // 边界：只有一门课，没有先修关系
    check(1, &[], &[7], 7);

    // 边界：没有任何依赖，答案是单门课耗时的最大值，不是总和
    check(3, &[], &[4, 9, 2], 9);

    // 反直觉：三门课共用同一个后继，若不取 max 而累加会得到 8
    check(4, &[vec![1, 4], vec![2, 4], vec![3, 4]], &[5, 1, 1, 1], 6);

    // 反直觉：一条链要一路加起来
    check(3, &[vec![1, 2], vec![2, 3]], &[1, 1, 5], 7);

    // 反直觉：宽而浅的图 —— 并行 4 门后收尾，最长的是耗时 6 的那门
    check(
        5,
        &[vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5]],
        &[2, 6, 3, 1, 4],
        10,
    );

    // 边界：2000 级链，两种解法都要能算（递归深度 = 链长）
    let chain_rel: Vec<Vec<i32>> = (1..2000).map(|v| vec![v, v + 1]).collect();
    check(2000, &chain_rel, &vec![1; 2000], 2000);

    // 边界：5 * 10^4 级链（题面上限），只跑不递归的解法二
    let big_rel: Vec<Vec<i32>> = (1..50000).map(|v| vec![v, v + 1]).collect();
    assert_eq!(
        Solution2::minimum_time(50000, big_rel, vec![1; 50000]),
        50000
    );

    println!("all tests passed");
}
