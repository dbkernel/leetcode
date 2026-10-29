/*
============================================================================
994. 腐烂的橘子（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/rotting-oranges/
分类：广度优先搜索 · 多源最短路

在给定的 m x n 网格 grid 中，每个单元格可以有以下三个值之一：

- 值 0 代表空单元格；
- 值 1 代表新鲜橘子；
- 值 2 代表腐烂的橘子。

每分钟，腐烂的橘子 周围 4 个方向上相邻 的新鲜橘子都会腐烂。

返回 直到单元格中没有新鲜橘子为止所必须经过的最小分钟数。如果不可能，
返回 -1 。

示例 1：
输入：grid = [[2,1,1],[1,1,0],[0,1,1]]
输出：4

示例 2：
输入：grid = [[2,1,1],[0,1,1],[1,0,1]]
输出：-1
解释：左下角的橘子（第 2 行， 第 0 列）永远不会腐烂，因为腐烂只会发生在
4 个方向上。

示例 3：
输入：grid = [[0,2]]
输出：0
解释：因为 0 分钟时已经没有新鲜橘子了，所以答案就是 0 。

提示：
- m == grid.length
- n == grid[i].length
- 1 <= m, n <= 10
- grid[i][j] 仅为 0 、 1 或 2
*/
use std::collections::VecDeque;

/*
解法：多源 BFS（按层推进）【最优解】

为什么慢：最容易想到的是"每分钟扫一遍整个网格，把 2 周围的新鲜橘子改成 2"，
每轮 O(mn)，最坏要扫 O(mn) 轮（一条蛇形走廊），总计 O(m^2 n^2)。

换 DFS 也不行：DFS 顺着一条路走到底，第一次摸到某个新鲜橘子时的路径长度
未必是它最早被感染的时间，要拿到最小值就得枚举所有路径，重复计算更多。

为什么这里只能用 BFS：某个橘子变腐烂的时刻，等于它到"最近的那个初始腐烂
橘子"的距离。边权全是 1（每分钟一步），这是无权图上的多源最短路，BFS 按距离
从小到大出队，第一次到达就是最短。

换个角度：既然所有腐烂橘子都是源点、距离都是 0，那就把它们在第 0 分钟一起
入队。此后队列的第 k 层恰好是"第 k 分钟被感染的橘子"，层号直接就是答案，
不必再逐分钟扫全网格。

关键点：
  · 初始就把所有腐烂橘子入队（多源），并顺手数清新鲜橘子的总数
  · 只在新橘子被感染时 `fresh -= 1`，于是 `fresh == 0` 正是"全部感染完毕"
  · 进入新一层时才 `minutes += 1`：一层的扩散发生在同一分钟内，不能一格一加
  · 循环条件带 `fresh > 0`：全部感染后不再空转，避免多算层数
  · 一开始 `fresh == 0` 立即返回 0，否则会先 minutes += 1 而多算一分钟
  · 结束时仍有 `fresh > 0`，说明这些橘子一开始就没有腐烂邻居，返回 -1
  · Rust 的行列是 `usize`，加方向前先转 `i32` 判越界，再转回下标，
    否则 `r - 1` 在 usize 下会回绕成巨大的值

手推一遍：
以示例 1（[[2,1,1],[1,1,0],[0,1,1]]，初始 6 个新鲜橘子）为例：
  第 0 层：腐烂橘子 (0,0)
  第 1 分钟：(0,1)、(1,0) 被感染，fresh = 4
  第 2 分钟：(0,2)、(1,1) 被感染，fresh = 2
  第 3 分钟：(2,1) 被感染，fresh = 1
  第 4 分钟：(2,2) 被感染，fresh = 0，退出，答案 4 ✔

复杂度分析：
  时间 O(mn)：每个格子最多入队一次，出队时只看 4 个邻居
  空间 O(mn)：队列最坏要同时放下所有腐烂橘子
*/

// 单文件 rustc 编译时不存在平台的 Solution，这里显式声明为单元结构体
pub struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh = 0;
        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    // 多源：所有腐烂橘子同时是起点
                    2 => queue.push_back((i, j)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }

        // 一开始就没有新鲜橘子，0 分钟即可，不能进循环去 minutes += 1
        if fresh == 0 {
            return 0;
        }

        let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut minutes = 0;

        // fresh > 0 保证最后一层感染完就停，不会把空转的层也数进去
        while !queue.is_empty() && fresh > 0 {
            minutes += 1; // 整层扩散发生在同一分钟内
            let layer_size = queue.len();

            for _ in 0..layer_size {
                let (r, c) = queue.pop_front().unwrap();

                for (dr, dc) in dirs.iter() {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    // 越界、空格子、已腐烂的橘子都不需要处理
                    if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] != 1 {
                        continue;
                    }

                    grid[nr][nc] = 2; // 立刻标记，防止同一分钟内被重复入队
                    fresh -= 1;
                    queue.push_back((nr, nc));
                }
            }
        }

        // 还有新鲜的说明它们从未与腐烂橘子连通
        if fresh == 0 {
            minutes
        } else {
            -1
        }
    }
}

fn check(grid: Vec<Vec<i32>>, want: i32) {
    assert_eq!(
        Solution::oranges_rotting(grid.clone()),
        want,
        "{:?}",
        grid
    );
}

fn main() {
    // 示例 1：需要 4 分钟才能感染全部
    check(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4);

    // 示例 2：左下角被 0 隔开，永远感染不到
    check(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1);

    // 示例 3：没有新鲜橘子，0 分钟
    check(vec![vec![0, 2]], 0);

    // 边界：只有一个新鲜橘子且没有腐烂橘子
    check(vec![vec![1]], -1);

    // 边界：只有一个腐烂橘子
    check(vec![vec![2]], 0);

    // 边界：一维两格，相邻扩散 1 分钟
    check(vec![vec![1, 2]], 1);

    // 边界：一行里新鲜橘子排成链
    check(vec![vec![2, 1, 1]], 2);

    // 边界：全是空格子
    check(vec![vec![0, 0], vec![0, 0]], 0);

    println!("all tests passed");
}
