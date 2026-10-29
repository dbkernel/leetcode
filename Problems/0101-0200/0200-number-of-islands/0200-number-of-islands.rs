/*
============================================================================
200. 岛屿数量（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/number-of-islands/
分类：网格 · 深度优先搜索（淹岛）

给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中
岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的
陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。

示例 1：
输入：grid = [
  ['1','1','1','1','0'],
  ['1','1','0','1','0'],
  ['1','1','0','0','0'],
  ['0','0','0','0','0']
]
输出：1

示例 2：
输入：grid = [
  ['1','1','0','0','0'],
  ['1','1','0','0','0'],
  ['0','0','1','0','0'],
  ['0','0','0','1','1']
]
输出：3

提示：
- m == grid.length
- n == grid[i].length
- 1 <= m, n <= 300
- grid[i][j] 的值为 '0' 或 '1'
*/
use std::collections::VecDeque;

/*
解法一：DFS 淹岛【最优解】

为什么慢：把"岛屿"翻译成"任意两块陆地是否连通"，就要对每一对陆地
判断一次，O(mn) 个格子两两比较是 O((mn)^2)；即便想到从一个陆地出发
扩散，如果只做标记而不敢改图，扩散结束后仍要再扫一遍找下一块起点，
多出一整轮全图扫描。

换个角度：既然一整块岛屿最终只记 1，它的形状就不必保留。每发现一个
还没被淹掉的 '1'，就地把它和它能到达的所有 '1' 全改成 '0'。此后网格里
剩下的 '1' 只可能是尚未计数的岛，扫到的每个 '1' 都是一块新岛。

关键点：
  · 计数和淹没必须绑死：扫到 '1' 先 count += 1，紧接着从这格淹完整块
  · 淹的动作放在进入递归之前（先置 '0' 再扩散），否则同一格会被多个
    邻居重复进入，复杂度退化
  · 相邻只看上下左右四向，对角相邻不算连通
  · 用改 grid 代替 visited 表：题面允许原地修改，省掉 O(mn) 的标记数组
  · Rust 下标是无符号的，越界判断要在减法之前做：先 `r > 0` 再 `r - 1`，
    写成 `r - 1 >= 0` 会直接下溢 panic

手推一遍：以示例 2 为例：
  (0,0) 是 '1' → count = 1，淹没左上角 2×2 的陆地
  继续扫，到 (2,2) 是 '1' → count = 2，淹掉它自己
  到 (3,3) 是 '1' → count = 3，顺手淹掉 (3,4)
  扫完，答案 3 ✔

复杂度分析：
  时间 O(mn)：每格最多被访问一次，淹过就变 '0'
  空间 O(mn)：递归栈最深 mn，最坏是整张图都是陆地

解法二：BFS 淹岛【最优解，与解法一并列】

把解法一的递归换成显式队列：扫到 '1' 时 count += 1，该格入队，然后
反复弹出队首、把四邻中仍是 '1' 的格子置 '0' 后入队，直到队列为空。
  · 时间同为 O(mn)：每格入队至多一次
  · 空间同为 O(mn)：队列最坏同时装下整块陆地的格子
  · 取舍：BFS 不占递归栈，深度只受堆内存限制，长条形网格里比 DFS 稳；
    DFS 不用队列、少一层循环。同量级，两者并列最优
  · 入队时就要置 '0'，不能等出队再置：否则同一格会被多个邻居重复入队，
    队列膨胀到 O(4mn)

复杂度分析：
  时间 O(mn)
  空间 O(mn)：队列最多同时装 O(mn) 个格子
*/

// ---------- 解法一：DFS 淹岛 ----------
pub struct Solution1;

impl Solution1 {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut count = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '1' {
                    continue;
                }
                // 扫到 '1'：它一定属于尚未计数的岛，先计数再淹
                count += 1;
                sink(&mut grid, rows, cols, r, c);
            }
        }
        count
    }
}

// 调用前已确认 grid[r][c] == '1'；入口先置 '0' 再递归，避免重复进入
fn sink(grid: &mut [Vec<char>], rows: usize, cols: usize, r: usize, c: usize) {
    grid[r][c] = '0';
    if r > 0 && grid[r - 1][c] == '1' {
        sink(grid, rows, cols, r - 1, c);
    }
    if r + 1 < rows && grid[r + 1][c] == '1' {
        sink(grid, rows, cols, r + 1, c);
    }
    if c > 0 && grid[r][c - 1] == '1' {
        sink(grid, rows, cols, r, c - 1);
    }
    if c + 1 < cols && grid[r][c + 1] == '1' {
        sink(grid, rows, cols, r, c + 1);
    }
}

// ---------- 解法二：BFS 淹岛 ----------
pub struct Solution2;

impl Solution2 {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut count = 0;
        let mut pending: VecDeque<(usize, usize)> = VecDeque::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '1' {
                    continue;
                }
                count += 1;
                // 入队前就淹掉：否则会被相邻格子重复入队
                grid[r][c] = '0';
                pending.push_back((r, c));

                while let Some((cr, cc)) = pending.pop_front() {
                    // 先判"下标存在"再访问，无符号减法才不会下溢
                    if cr > 0 && grid[cr - 1][cc] == '1' {
                        grid[cr - 1][cc] = '0';
                        pending.push_back((cr - 1, cc));
                    }
                    if cr + 1 < rows && grid[cr + 1][cc] == '1' {
                        grid[cr + 1][cc] = '0';
                        pending.push_back((cr + 1, cc));
                    }
                    if cc > 0 && grid[cr][cc - 1] == '1' {
                        grid[cr][cc - 1] = '0';
                        pending.push_back((cr, cc - 1));
                    }
                    if cc + 1 < cols && grid[cr][cc + 1] == '1' {
                        grid[cr][cc + 1] = '0';
                        pending.push_back((cr, cc + 1));
                    }
                }
            }
        }
        count
    }
}

// ===== 测试辅助：字符串行转网格、双解法对照 =====
// 解法会原地淹岛，所以每条用例都要现造一份新网格
fn make_grid(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn check(rows: &[&str], want: i32) {
    assert_eq!(Solution1::num_islands(make_grid(rows)), want);
    assert_eq!(Solution2::num_islands(make_grid(rows)), want);
}

fn main() {
    // 示例 1：一整块大陆，中间有凹陷的水
    check(&["11110", "11010", "11000", "00000"], 1);

    // 示例 2：三座岛，(2,2) 与右下角互不相连
    check(&["11000", "11000", "00100", "00011"], 3);

    // 边界：全是水，一座岛也没有
    check(&["000", "000", "000"], 0);

    // 边界：全是陆地，只有一座岛
    check(&["111", "111", "111"], 1);

    // 边界：单格网格
    check(&["1"], 1);
    check(&["0"], 0);

    // 对角相邻不算连通：一行里两块陆地隔着水
    check(&["101"], 2);

    // 环形岛：中间的水不参与计数，但整圈仍是一座岛
    check(&["111", "101", "111"], 1);

    // 边界：竖条与横条各成一体，只在角上相遇也不算连通
    check(&["10", "10", "01", "01"], 2);

    // 边界：100×100 全陆，逼递归深度到 10000 层
    let big: Vec<Vec<char>> = (0..100).map(|_| vec!['1'; 100]).collect();
    assert_eq!(Solution1::num_islands(big.clone()), 1);
    assert_eq!(Solution2::num_islands(big), 1);

    println!("all tests passed");
}
