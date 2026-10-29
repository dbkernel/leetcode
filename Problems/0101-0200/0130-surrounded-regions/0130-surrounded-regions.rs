/*
============================================================================
130. 被围绕的区域（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/surrounded-regions/
分类：并查集 · 边界洪水填充

给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' 组成，捕获 所有
被围绕的区域：

- 连接：一个单元格与水平或垂直方向上相邻的单元格连接。
- 区域：连接所有 'O' 的单元格来形成一个区域。
- 围绕：如果一个区域中的所有 'O' 单元格都不在棋盘的边缘，则该区域被
  包围。这样的区域 完全 被 'X' 单元格包围。

通过 原地 将输入矩阵中的所有 'O' 替换为 'X' 来 捕获被围绕的区域。你
不需要返回任何值。

示例 1：
输入：board = [['X','X','X','X'],['X','O','O','X'],['X','X','O','X'],['X','O','X','X']]
输出：[['X','X','X','X'],['X','X','X','X'],['X','X','X','X'],['X','O','X','X']]
解释：在上图中，底部的区域没有被捕获，因为它在 board 的边缘并且不能被围绕。

示例 2：
输入：board = [['X']]
输出：[['X']]

提示：
- m == board.length
- n == board[i].length
- 1 <= m, n <= 200
- board[i][j] 为 'X' 或 'O'
*/

// 并查集：路径压缩 + 按大小合并。
// 路径压缩把查找路径上的点直接挂到根，缩短后续查找；
// 按大小合并把元素少的树挂到元素多的树下，防止树退化成链。
// 两者合起来让单次操作的均摊代价接近 O(α(n))。
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let (mut rx, mut ry) = (self.find(x), self.find(y));
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        self.parent[ry] = rx;
        self.size[rx] += self.size[ry];
    }
}

/*
解法一：从边界多源 BFS，标记安全格后回写【最优解】

为什么慢：直接对每个 'O' 判断“它所在区域是否被围”，要么反复扫描同一块
区域，要么从内部出发根本无从知道它有没有连到边界。

换个角度：逆向思考 —— 不被围的反面是“与边界连通”。从四条边上的每个
'O' 同时出发做洪水填充，凡是能淹到的 'O' 都不会被围，剩下的才是被围的。

关键点：
  · 必须从边界出发：边界上的 'O' 天然安全，从它扩散到的整块 'O' 都安全
  · 标记用第三个字符 '#'，先标记后回写，避免边标记边改写导致漏填或死循环
  · 四条边都要扫，角落会被重复入队，靠 board 上的 '#' 判重
  · 最后一律把剩余 'O' 改 'X'、把 '#' 改回 'O'，全 'X' 或全 'O' 都成立

手推一遍：以示例 1 为例（4x4，底部 (3,1) 是 'O'）：
  (3,1) 在边界，入队并标 '#'
  它四周都是 'X'，无法继续扩散
  回写时 (1,1)(1,2)(2,2) 仍是 'O' → 全部改 'X'；(3,1) 的 '#' 还原成 'O' ✔

复杂度分析：
  时间 O(mn)：每个格子至多入队一次，最后回写再扫一遍
  空间 O(mn)：队列最坏装下所有 'O'，原地 '#' 标记省掉了 visited

解法二：并查集 + 虚拟边界节点【最优解，与解法一并列】

为什么慢：洪水填充要真的走到每个格子；若只想回答“这块 'O' 是否触边”，
并查集把问题变成一次集合查找。

换个角度：多设一个虚拟节点代表“棋盘外”。把所有边界 'O' 与它合并，再把
相邻的 'O' 两两合并；最后不在虚拟节点集合里的 'O' 就是被围的。

关键点：
  · 格子编号 r*n+c，虚拟节点编号 m*n，并查集开 m*n+1 个位置
  · 四条边上的 'O' 都要与虚拟节点合并，漏掉一条边就会把贴边的 'O' 误判
  · 相邻关系只需向右、向下合并，这两条方向已覆盖全部无向相邻边
  · 回写时比较 find(格子) 与 find(虚拟节点)，不要把虚拟节点写进棋盘

手推一遍：仍以示例 1 为例：
  边界 'O' 只有 (3,1)，与虚拟节点合并成 {虚拟, (3,1)}
  (1,1)-(1,2)-(2,2) 相邻合并成另一集合，与虚拟节点不相通
  回写：(1,1)(1,2)(2,2) 改 'X'，(3,1) 保留 'O' ✔

复杂度分析：
  时间 O(mn α(mn))：每个格子常数次查找与合并
  空间 O(mn)：parent 与 size 各一个长度 m*n+1 的数组
*/

// 本地 rustc 单文件编译需要显式写出类型，平台上的 Solution 由系统提供
pub struct Solution1;

impl Solution1 {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut queue: Vec<(usize, usize)> = Vec::new();

        for r in 0..m {
            mark(board, &mut queue, r, 0);
            mark(board, &mut queue, r, n - 1);
        }
        for c in 0..n {
            mark(board, &mut queue, 0, c);
            mark(board, &mut queue, m - 1, c);
        }

        let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
        let mut head = 0;
        while head < queue.len() {
            let (r, c) = queue[head];
            head += 1;
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                    continue;
                }
                mark(board, &mut queue, nr as usize, nc as usize);
            }
        }

        // 先标记后回写：此刻 '#' 是安全的 O，剩余的 'O' 才是被围的
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 'O' {
                    *cell = 'X';
                } else if *cell == '#' {
                    *cell = 'O';
                }
            }
        }
    }
}

// 入队即标记：'#' 表示“与边界连通的 O”，同时充当判重
fn mark(board: &mut [Vec<char>], queue: &mut Vec<(usize, usize)>,
        r: usize, c: usize) {
    if board[r][c] != 'O' {
        return;
    }
    board[r][c] = '#';
    queue.push((r, c));
}

pub struct Solution2;

impl Solution2 {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let root = m * n; // 虚拟节点代表“棋盘外”
        let mut dsu = Dsu::new(m * n + 1);

        for r in 0..m {
            for c in 0..n {
                if board[r][c] != 'O' {
                    continue;
                }
                let cur = r * n + c;
                // 贴边的 O 与棋盘外连通，永远不会被围
                if r == 0 || r == m - 1 || c == 0 || c == n - 1 {
                    dsu.unite(cur, root);
                }
                // 只看右、下：每条无向边恰好被一个格子负责一次
                if r + 1 < m && board[r + 1][c] == 'O' {
                    dsu.unite(cur, (r + 1) * n + c);
                }
                if c + 1 < n && board[r][c + 1] == 'O' {
                    dsu.unite(cur, r * n + c + 1);
                }
            }
        }

        for r in 0..m {
            for c in 0..n {
                if board[r][c] == 'O' && dsu.find(r * n + c) != dsu.find(root) {
                    board[r][c] = 'X';
                }
            }
        }
    }
}

fn check(board: Vec<Vec<char>>, want: Vec<Vec<char>>) {
    let mut b1 = board.clone();
    Solution1::solve(&mut b1);
    assert_eq!(b1, want);

    let mut b2 = board;
    Solution2::solve(&mut b2);
    assert_eq!(b2, want);
}

fn main() {
    // 示例 1：逐一比对整个矩阵
    check(
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ],
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ],
    );

    // 示例 2：单格 X
    check(vec![vec!['X']], vec![vec!['X']]);

    // 边界：单格 O 在边界上，保留
    check(vec![vec!['O']], vec![vec!['O']]);

    // 边界：全 X，原样返回
    check(
        vec![
            vec!['X', 'X', 'X'],
            vec!['X', 'X', 'X'],
            vec!['X', 'X', 'X'],
        ],
        vec![
            vec!['X', 'X', 'X'],
            vec!['X', 'X', 'X'],
            vec!['X', 'X', 'X'],
        ],
    );

    // 边界：全 O，全部贴边，原样返回
    check(
        vec![
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
        ],
        vec![
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
        ],
    );

    // 反直觉：内圈 O 被 X 包围，整块都要变成 X
    check(
        vec![
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'O', 'X'],
            vec!['X', 'X', 'X', 'X', 'X'],
        ],
        vec![
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X'],
        ],
    );

    // 反直觉：只在对角相邻不算连通，中心 O 会被改写
    check(
        vec![
            vec!['O', 'X', 'O'],
            vec!['X', 'O', 'X'],
            vec!['O', 'X', 'O'],
        ],
        vec![
            vec!['O', 'X', 'O'],
            vec!['X', 'X', 'X'],
            vec!['O', 'X', 'O'],
        ],
    );

    // 有通道连到边界：整块 O 都保留
    check(
        vec![
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'X', 'X'],
        ],
        vec![
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'X', 'X'],
        ],
    );

    // 单行棋盘：两端都是边界，全部保留
    check(
        vec![vec!['O', 'X', 'O']],
        vec![vec!['O', 'X', 'O']],
    );

    println!("all tests passed");
}
