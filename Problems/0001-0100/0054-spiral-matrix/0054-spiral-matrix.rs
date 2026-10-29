/*
============================================================================
54. 螺旋矩阵（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/spiral-matrix/
分类：矩阵 · 按层遍历（边界收缩）

给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的
所有元素。

示例 1：
输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
输出：[1,2,3,6,9,8,7,4,5]

示例 2：
输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
输出：[1,2,3,4,8,12,11,10,9,5,6,7]

提示：
- m == matrix.length
- n == matrix[i].length
- 1 <= m, n <= 10
- -100 <= matrix[i][j] <= 100
*/

// 本地 rustc 单文件编译时力扣不会注入 Solution，故显式定义
pub struct Solution1;
pub struct Solution2;

/*
解法一：模拟行走（方向数组 + 访问标记）【非最优】

为什么慢：螺旋路径本身要访问 mn 个格子，时间不可能低于 O(mn)。这一版
并不慢在时间，而是慢在空间：为了知道「下一步是不是已经走过」，额外开了
一个 m × n 的 visited 矩阵，空间从 O(1) 涨到 O(mn)。

换个角度：把螺旋看成一个「一直往前走」的过程 —— 沿当前方向前进，只有
当下一步会出界、或者会踩到已访问的格子时才右转。于是路径只需要一个
方向下标和一张已访问表。

关键点：
  · 方向数组必须按顺时针排列：(0,1)、(1,0)、(0,-1)、(-1,0)
  · 先算出下一步再判断，越界或已访问才右转；不要把「当前格」判成非法
  · 起点先标记已访问，否则第一圈绕回来会把自己当成可走的格子
  · 总共只取 mn 个元素，所以循环次数固定，最后一格之后不需要再验证
    转向后的落点
  · 边界：单行或单列时，转向规则自然让路径直线走完，无需特判
  · Rust 的 usize 不能表示 -1，方向与坐标统一用 isize，最后再转回 usize

以示例 1（3 × 3）为例：
  (0,0)(0,1)(0,2) → 右侧出界，右转
  (1,2)(2,2) → 下方出界，右转
  (2,1)(2,0) → 左侧出界，右转
  (1,0) → 上方 (0,0) 已访问，右转，接着 (1,1)
  得到 1,2,3,6,9,8,7,4,5 ✔

复杂度分析：
  时间 O(mn)：每个格子进出各一次
  空间 O(mn)：visited 与原矩阵等大
*/

impl Solution1 {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let total = rows * cols;

        let mut visited = vec![vec![false; cols]; rows];
        let mut order = Vec::with_capacity(total);

        // 顺时针：右、下、左、上
        let dr: [isize; 4] = [0, 1, 0, -1];
        let dc: [isize; 4] = [1, 0, -1, 0];

        let (mut row, mut col) = (0isize, 0isize);
        let mut dir = 0usize;

        for _ in 0..total {
            order.push(matrix[row as usize][col as usize]);
            visited[row as usize][col as usize] = true;

            let mut nr = row + dr[dir];
            let mut nc = col + dc[dir];
            // 只有下一步不可走时才右转；只转一次就够，螺旋没有连续两次转弯
            if nr < 0
                || nr >= rows as isize
                || nc < 0
                || nc >= cols as isize
                || visited[nr as usize][nc as usize]
            {
                dir = (dir + 1) % 4;
                nr = row + dr[dir];
                nc = col + dc[dir];
            }
            row = nr;
            col = nc;
        }
        order
    }
}

/*
解法二：按层模拟，四条边界收缩【最优解】

为什么慢：解法一的 visited 是「事后防重」——先走过去，再用额外空间记住
走过哪里。但螺旋的形状完全由四条边界决定，边界一旦给出，就不需要再记住
走过的格子，visited 是多余的信息。

换个角度：把矩阵看成一层套一层的环。每一轮先输出上边（top 行）、右边
（right 列）、下边（bottom 行）、左边（left 列），然后把四条边界各内缩
一格，进入下一层。

关键点：
  · 上边与右边无条件输出：只要 top <= bottom 且 left <= right，这两条边
    一定还有没输出的格子
  · 下边与左边必须同时满足 left < right 且 top < bottom 才输出：
    只剩一行时（top == bottom）下边与上边重合，只剩一列时（left == right）
    左边与右边重合，不判就会重复输出
  · 两条边要一起判，不能只判其中一个：单行时 top == bottom、单列时
    left == right，各自会让对应的一对边重复
  · 收缩放在一轮末尾统一做，避免边写边改导致循环条件提前失效
  · 下边从 right-1 走到 left+1、左边从 bottom 走到 top+1：四个角已经在
    上边与右边输出过，不能重复
  · 四条边界用 isize，避免只剩一行时 right - 1 在 usize 下回绕

以示例 1（3 × 3）为例：
  第一层 top=0,bottom=2,left=0,right=2：输出 1,2,3（上）、6,9（右）、
  8,7（下）、4（左），四条边都成立，边界缩成 top=1,bottom=1,left=1,right=1
  第二层只剩中心：输出 5，随后 left > right，循环结束 ✔

以 3 行 4 列（示例 2）的最后一行为例：
  第一层输出 1,2,3,4、8,12、11,10,9、5 之后，剩中间一行 [6,7]，
  此时 top == bottom，后两条边被跳过，只输出 6,7，不会把 6,7 再走一遍 ✔

复杂度分析：
  时间 O(mn)：每个元素恰好输出一次
  空间 O(1)：除返回数组外只用四条边界
*/

impl Solution2 {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }

        let rows = matrix.len() as isize;
        let cols = matrix[0].len() as isize;

        let (mut left, mut right) = (0isize, cols - 1);
        let (mut top, mut bottom) = (0isize, rows - 1);
        let mut order = Vec::with_capacity((rows * cols) as usize);

        while left <= right && top <= bottom {
            // 上边：从左到右，整行都要
            for col in left..=right {
                order.push(matrix[top as usize][col as usize]);
            }
            // 右边：从上到下，跳过已在角上输出的 (top, right)
            for row in top + 1..=bottom {
                order.push(matrix[row as usize][right as usize]);
            }
            // 只剩一行或一列时，下边与左边会和上边、右边重合
            if left < right && top < bottom {
                for col in (left + 1..right).rev() {
                    order.push(matrix[bottom as usize][col as usize]);
                }
                for row in (top + 1..=bottom).rev() {
                    order.push(matrix[row as usize][left as usize]);
                }
            }
            left += 1;
            right -= 1;
            top += 1;
            bottom -= 1;
        }
        order
    }
}

// ===== 测试辅助 =====
fn check(matrix: &[Vec<i32>], want: &[i32]) {
    // 两个解法都只读矩阵，可以共用同一份数据
    let by_sim = Solution1::spiral_order(matrix.to_vec());
    let by_layer = Solution2::spiral_order(matrix.to_vec());
    assert_eq!(by_sim, want);
    assert_eq!(by_layer, want);
}

fn main() {
    // 示例 1：方形矩阵，最后一层只剩中心一格
    check(
        &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        &[1, 2, 3, 6, 9, 8, 7, 4, 5],
    );

    // 示例 2：3 行 4 列，最后一层只剩一行
    check(
        &[
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ],
        &[1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
    );

    // 边界：1 x 1
    check(&[vec![42]], &[42]);

    // 边界：单行，不应重复输出
    check(&[vec![1, 2, 3, 4, 5]], &[1, 2, 3, 4, 5]);

    // 边界：单列，不应重复输出
    check(
        &[vec![1], vec![2], vec![3], vec![4], vec![5]],
        &[1, 2, 3, 4, 5],
    );

    // 边界：2 x 2
    check(&[vec![1, 2], vec![3, 4]], &[1, 2, 4, 3]);

    // 反直觉：2 行 3 列，第二行要从右往左读
    check(&[vec![1, 2, 3], vec![4, 5, 6]], &[1, 2, 3, 6, 5, 4]);

    // 反直觉：3 行 2 列，最后只剩一列，容易上下重复
    check(
        &[vec![1, 2], vec![3, 4], vec![5, 6]],
        &[1, 2, 4, 6, 5, 3],
    );

    // 边界：3 行 5 列
    check(
        &[
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ],
        &[1, 2, 3, 4, 5, 10, 15, 14, 13, 12, 11, 6, 7, 8, 9],
    );

    // 边界：5 行 3 列
    check(
        &[
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
            vec![13, 14, 15],
        ],
        &[1, 2, 3, 6, 9, 12, 15, 14, 13, 10, 7, 4, 5, 8, 11],
    );

    println!("all tests passed");
}
