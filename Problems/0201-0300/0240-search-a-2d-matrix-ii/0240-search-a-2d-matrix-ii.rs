/*
============================================================================
240. 搜索二维矩阵 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/search-a-2d-matrix-ii/
分类：矩阵 · 有序矩阵的 Z 字形搜索

编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target 。
该矩阵具有以下特性：

- 每行的元素从左到右升序排列。
- 每列的元素从上到下升序排列。

示例 1：
输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],
               [10,13,14,17,24],[18,21,23,26,30]], target = 5
输出：true

示例 2：
输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],
               [10,13,14,17,24],[18,21,23,26,30]], target = 20
输出：false

提示：
- m == matrix.length
- n == matrix[i].length
- 1 <= n, m <= 300
- -10^9 <= matrix[i][j] <= 10^9
- 每行的所有元素从左到右升序排列
- 每列的所有元素从上到下升序排列
- -10^9 <= target <= 10^9
*/

// 本地 rustc 单文件编译时力扣不会注入 Solution，故显式定义
pub struct Solution1;
pub struct Solution2;
pub struct Solution3;

/*
解法一：逐个比较【非最优】

为什么慢：有序性一点没用上，把 mn 个元素全部看一遍。对于 300 × 300
的矩阵就是 9 万次比较，而有序性允许我们一次排除一整行或一整列。

换个角度：先有最朴素的答案作为基线 —— 双层循环找 target，找不到返回
false。它的价值在于给出正确性参照和 O(mn) 的时间上界。

关键点：
  · 找到即返回 true，不要继续扫完
  · 行列都从 0 开始，矩阵非空由约束保证
  · 没有任何剪枝，因此对任意矩阵都成立（不依赖有序）

以示例 1 中 target = 20 为例：
  逐行扫完 25 个元素都没命中 → 返回 false ✔

复杂度分析：
  时间 O(mn)：每个元素比较一次
  空间 O(1)：只用循环变量
*/

impl Solution1 {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in &matrix {
            for &value in row {
                if value == target {
                    return true;
                }
            }
        }
        false
    }
}

/*
解法二：每行做一次二分查找【非最优】

为什么慢：解法一在行内也是线性扫描，浪费了「行内升序」。对每一行二分，
时间从 O(mn) 降到 O(m log n)。

换个角度：把问题拆成 m 个独立的一维查找。行内有序正好可以用
binary_search 判断 target 是否存在。

关键点：
  · binary_search 返回 Result，用 is_ok() 取布尔值，不关心具体下标
  · 行内允许重复值，binary_search 命中任意一个即可，结果只看真假
  · 每行独立判断，任意一行命中即可返回 true

以示例 1 中 target = 5 为例：
  第 0 行 [1,4,7,11,15] 二分找不到 5
  第 1 行 [2,5,8,12,19] 二分命中 → 返回 true ✔

复杂度分析：
  时间 O(m log n)：每行一次二分，共 m 行
  空间 O(1)：只用切片借用
*/

impl Solution2 {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in &matrix {
            if row.binary_search(&target).is_ok() {
                return true;
            }
        }
        false
    }
}

/*
解法三：从右上角出发的 Z 字形搜索【最优解】

为什么慢：解法二每行都要二分，共 O(m log n)；但列的有序性完全没用上。
只要同时利用行、列两个方向的有序性，就能把两个维度的搜索合并成一条
单调路径。

换个角度：站在右上角 (0, n-1)，它是「本行最大、本列最小」。比较它和
target：
  · 相等 → 找到
  · 大于 target → 这一列从它往下都更大，整列可以排除，列下标左移
  · 小于 target → 这一行从它往左都更小，整行可以排除，行下标下移
每走一步都排除一整行或一整列，走到底最多 m + n 步。

关键点：
  · 起点必须是右上角或左下角：只有这两个角同时是「一行的一端」和
    「一列的一端」，才能从比较结果推出「整行/整列可弃」
  · 左上角不行：它同时是本行与本列的最小值，比 target 小只能说明
    「它自己不是」，推不出整行或整列可弃
  · 大于时减列、小于时加行，方向反了会走向已经排除的区域
  · 列下标会走到 -1，所以用 isize；循环条件是 row < m && col >= 0

以示例 1 中 target = 5 为例：
  (0,4)=15 > 5 → 弃第 4 列，col = 3
  (0,3)=11 > 5 → 弃第 3 列，col = 2
  (0,2)=7 > 5 → 弃第 2 列，col = 1
  (0,1)=4 < 5 → 弃第 0 行，row = 1
  (1,1)=5 → 返回 true ✔

复杂度分析：
  时间 O(m+n)：row 只增、col 只减，各最多走 m / n 步
  空间 O(1)：只用两个下标
*/

impl Solution3 {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len() as isize;
        let n = matrix[0].len() as isize;

        // 右上角：本行最大、本列最小，一次比较可排除一整行或一整列
        let mut row = 0isize;
        let mut col = n - 1;
        while row < m && col >= 0 {
            let value = matrix[row as usize][col as usize];
            if value == target {
                return true;
            }
            if value > target {
                col -= 1;
            } else {
                row += 1;
            }
        }
        false
    }
}

// ===== 测试辅助 =====
fn sample() -> Vec<Vec<i32>> {
    vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ]
}

fn check(matrix: &[Vec<i32>], target: i32, want: bool) {
    // 三个解法都只读矩阵，可以共用同一份数据
    assert_eq!(Solution1::search_matrix(matrix.to_vec(), target), want);
    assert_eq!(Solution2::search_matrix(matrix.to_vec(), target), want);
    assert_eq!(Solution3::search_matrix(matrix.to_vec(), target), want);
}

fn main() {
    // 示例 1：目标在第二行中间
    check(&sample(), 5, true);

    // 示例 2：目标落在行、列的缝隙里
    check(&sample(), 20, false);

    // 覆盖四个角：右上、左下、左上、右下
    check(&sample(), 15, true);
    check(&sample(), 18, true);
    check(&sample(), 1, true);
    check(&sample(), 30, true);

    // 边界：小于最小值、大于最大值
    check(&sample(), 0, false);
    check(&sample(), 31, false);

    // 边界：1 x 1
    check(&[vec![5]], 5, true);
    check(&[vec![5]], 4, false);

    // 边界：单行，Z 字形退化为一路左移
    check(&[vec![1, 3, 5, 7]], 5, true);
    check(&[vec![1, 3, 5, 7]], 6, false);

    // 边界：单列，Z 字形退化为一路下移
    check(&[vec![1], vec![3], vec![5]], 5, true);
    check(&[vec![1], vec![3], vec![5]], 4, false);

    // 反直觉：矩阵允许重复值，比较相等即返回，不必找唯一位置
    check(
        &[vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
        1,
        true,
    );
    check(
        &[vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
        2,
        false,
    );

    // 反直觉：负值矩阵，不能把 0 当作哨兵
    check(&[vec![-5, -3], vec![-2, -1]], -3, true);
    check(&[vec![-5, -3], vec![-2, -1]], 0, false);

    println!("all tests passed");
}
