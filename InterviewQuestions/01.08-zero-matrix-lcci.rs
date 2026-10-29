/*
============================================================================
面试题 01.08. 零矩阵（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zero-matrix-lcci/
分类：数组 · 原地标记

编写一种算法，若M × N矩阵中某个元素为0，则将其所在的行与列清零。

示例 1：
输入：
[
  [1,1,1],
  [1,0,1],
  [1,1,1]
]
输出：
[
  [1,0,1],
  [0,0,0],
  [1,0,1]
]

示例 2：
输入：
[
  [0,1,2,0],
  [3,4,5,2],
  [1,3,1,5]
]
输出：
[
  [0,0,0,0],
  [0,4,5,0],
  [0,3,1,0]
]
*/

/*
力扣预置的是 `impl Solution`，本地用 rustc 单文件编译时没有这个类型，
因此三个解法各自显式声明结构体，方法名与平台签名保持一致。
*/

/*
解法一：两个标记数组（先记录，再统一清零）【非最优】

为什么慢：如果边扫描边把行列清零，刚写下的 0 会在后续扫描中被当成
"原本就是 0"，把原本不该清零的行列也牵连进去，答案会被污染。要避免
这个顺序问题，就得把"哪些行、哪些列要清零"先完整记录下来。

换个角度：真正需要传递的信息只有 m 个行标记和 n 个列标记，和矩阵里
零的个数、位置都无关。

关键点：
  · 第一遍只读不写，把 row[i] / col[j] 置位
  · 第二遍才写，顺序反过来就会把新写的 0 当成原始零
  · 标记数组初值必须是 false，否则没零的行列也会被清零

以示例 2 为例：
  零出现在 (0,0)、(0,3)，row[0]、col[0]、col[3] 置位
  第二遍按标记写，得到 [[0,0,0,0],[0,4,5,0],[0,3,1,0]] ✔

复杂度分析：
  时间 O(mn)：两遍完整扫描
  空间 O(m+n)：两排标记数组，m、n 为行列数
*/
pub struct Solution1;

impl Solution1 {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        // 只记录，不改矩阵：避免新写入的 0 影响后续判断
        let mut row = vec![false; m];
        let mut col = vec![false; n];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    row[i] = true;
                    col[j] = true;
                }
            }
        }

        // 标记已经完整，第二遍可以放心写
        for i in 0..m {
            for j in 0..n {
                if row[i] || col[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

/*
解法二：首行首列当标记，另用两个变量记录它们自身【最优解】

为什么慢：解法一的标记数组本身要 O(m+n) 空间，可矩阵的第一行和第一列
天然就能充当这两排标记，额外空间只剩"第一行、第一列原本是否有零"。

换个角度：把 matrix[i][0] 当作第 i 行的标记，把 matrix[0][j] 当作第 j 列
的标记；首行首列自己是否要清零，则用两个布尔变量单独保存。

关键点：
  · 两个 flag 必须在改写首行首列之前采集，晚一步就被覆盖
  · 标记阶段只处理 i>=1、j>=1 的内部区域，不能碰首行首列标记
  · 写内部区域时用 matrix[i][0] / matrix[0][j] 判断
  · 最后才按 flag 清零首行、首列，顺序颠倒会破坏标记

以示例 2 为例：
  内部零只有 (0,0)、(0,3)，都在首行，flag_row0 = true
  内部区域无需标记，按首行标记写内部 → 第 0、3 列全零
  最后按 flag_row0 清首行 → 整行零 ✔

复杂度分析：
  时间 O(mn)：仍是最多两遍扫描
  空间 O(1)：只多用两个布尔变量
*/
pub struct Solution2;

impl Solution2 {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        // 先采集首行首列自身的状态，之后它们会被当作标记改写
        let first_row_zero = (0..n).any(|j| matrix[0][j] == 0);
        let first_col_zero = (0..m).any(|i| matrix[i][0] == 0);

        // 内部区域的零，投影到首行首列上
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // 标记齐了，再写内部区域
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // 首行首列最后处理，避免提前破坏内部区域还要用的标记
        if first_col_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
        if first_row_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}

/*
解法三：只留一个标记变量，matrix[0][0] 兼任首行标记【最优解，与解法二并列】
【最优解，与解法二并列】

为什么慢：解法二用了两个布尔变量，其中一个其实可以省掉：matrix[0][0]
既在第一列、又在第一行，可以让它专门记录"首行是否要清零"，首列则只用
一个 flag。

换个角度：把首列状态放进 flag_col0，首行状态放进 matrix[0][0]。代价是
matrix[0][0] 在清零首列时可能被提前改写，所以处理行的顺序要改。

关键点：
  · 第一遍先看 matrix[i][0] 更新 flag_col0，再处理 j>=1 的标记
  · 第二遍必须从最后一行往上走：matrix[0][j] 是列标记，若先处理第 0 行
    就会把列标记全部写成 0，后面每行都会被误清零
  · 每行先按标记写 j>=1 的部分，再按 flag_col0 写 matrix[i][0]
  · n == 1 时内层循环不执行，首列完全由 flag_col0 决定

以示例 2 为例：
  flag_col0 = true（matrix[0][0] 原本为 0）
  首行标记由 matrix[0][0] 承担，列 0、3 被标记
  从第 2 行倒序写，最后处理第 0 行，首行整行清零 ✔

复杂度分析：
  时间 O(mn)：最多两遍扫描
  空间 O(1)：只多一个布尔变量
*/
pub struct Solution3;

impl Solution3 {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut first_col_zero = false;
        for i in 0..m {
            // 必须在写 matrix[i][0] 之前判断它原本是否为零
            if matrix[i][0] == 0 {
                first_col_zero = true;
            }

            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // 倒序：第 0 行的 matrix[0][j] 是列标记，必须留到最后再用
        for i in (0..m).rev() {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
            if first_col_zero {
                matrix[i][0] = 0;
            }
        }
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 与三种解法独立的参照实现：先收集坐标，再统一清零
fn brute_zero(matrix: &mut Vec<Vec<i32>>) {
    let mut zeros = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if v == 0 {
                zeros.push((i, j));
            }
        }
    }
    for (r, c) in zeros {
        for j in 0..matrix[0].len() {
            matrix[r][j] = 0;
        }
        for i in 0..matrix.len() {
            matrix[i][c] = 0;
        }
    }
}

fn check(input: &[Vec<i32>], want: &[Vec<i32>]) {
    let mut a = input.to_vec();
    Solution1::set_zeroes(&mut a);
    assert_eq!(a, want);

    let mut b = input.to_vec();
    Solution2::set_zeroes(&mut b);
    assert_eq!(b, want);

    let mut c = input.to_vec();
    Solution3::set_zeroes(&mut c);
    assert_eq!(c, want);

    let mut d = input.to_vec();
    brute_zero(&mut d);
    assert_eq!(d, want);
}

fn main() {
    // 示例 1：零在正中间，整行整列清零
    check(
        &[vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
        &[vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
    );

    // 示例 2：首行有两个零，列标记来自首行
    check(
        &[vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
        &[vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
    );

    // 边界：1x1 且为零，唯一元素清零
    check(&[vec![0]], &[vec![0]]);

    // 边界：1x1 且非零，不能凭空清零
    check(&[vec![7]], &[vec![7]]);

    // 边界：只有一行，零会清掉整行
    check(&[vec![1, 0, 3]], &[vec![0, 0, 0]]);

    // 边界：只有一列，零会清掉整列
    check(&[vec![1], vec![0], vec![3]], &[vec![0], vec![0], vec![0]]);

    // 边界：零在左上角，首行首列同时被清
    check(&[vec![0, 2], vec![3, 4]], &[vec![0, 0], vec![0, 4]]);

    // 边界：零只在首行，首行 flag 与列标记同时生效
    check(
        &[vec![1, 0, 3], vec![4, 5, 6]],
        &[vec![0, 0, 0], vec![4, 0, 6]],
    );

    // 边界：零只在首列，首列 flag 单独生效
    check(
        &[vec![1, 2, 3], vec![0, 5, 6], vec![7, 8, 9]],
        &[vec![0, 2, 3], vec![0, 0, 0], vec![0, 8, 9]],
    );

    // 边界：全零矩阵
    check(&[vec![0, 0], vec![0, 0]], &[vec![0, 0], vec![0, 0]]);

    // 反直觉：没有零时，任何解法都不得改动矩阵
    check(
        &[vec![1, 2, 3], vec![4, 5, 6]],
        &[vec![1, 2, 3], vec![4, 5, 6]],
    );

    // 反直觉：零在内部非首行非首列，检验内部标记投影
    check(
        &[vec![1, 2, 3, 4], vec![5, 0, 7, 8], vec![9, 10, 11, 12]],
        &[vec![1, 0, 3, 4], vec![0, 0, 0, 0], vec![9, 0, 11, 12]],
    );

    println!("all tests passed");
}
