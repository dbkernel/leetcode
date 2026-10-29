/*
============================================================================
LCR 166 / 剑指 Offer 47. 珠宝的最高价值（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/li-wu-de-zui-da-jie-zhi-lcof/
分类：动态规划 · 网格路径累加
说明：本题即《剑指 Offer（第 2 版）》第 47 题「礼物的最大价值」。力扣
      现行题面改名为「珠宝的最高价值」，矩阵名由 grid 改为 frame，
      签名相应改为 jewelleryValue(frame)。

现有一个记作二维矩阵 frame 的珠宝架，其中 frame[i][j] 为该位置珠宝的价值。拿取珠宝的规则为：

- 只能从架子的左上角开始拿珠宝
- 每次可以移动到右侧或下侧的相邻位置
- 到达珠宝架子的右下角时，停止拿取

注意：珠宝的价值都是大于 0 的。除非这个架子上没有任何珠宝，比如 frame = [[0]]。

示例 1：
输入：frame = [[1,3,1],[1,5,1],[4,2,1]]
输出：12
解释：路径 1→3→5→2→1 可以拿到最高价值的珠宝

提示：
- 0 < frame.length <= 200
- 0 < frame[0].length <= 200
*/
/*
Rust 没有平台预置的 Solution 类型，单文件 rustc 编译时必须自己声明。
解法只有一条，所以直接用 Solution，与平台签名保持一致。
*/

/*
解法：二维 DP 行内原地累加【最优解】

为什么慢：把每条路线都走一遍，路线数与 C(m+n-2, m-1) 同阶，200x200 时
是天文数字；就算另开一张 dp 表逐格递推，也白搭了 O(m*n) 的额外空间。

换个角度：走进 (i,j) 的最后一步只能来自左邻 (i,j-1) 或上邻 (i-1,j)，
所以累计价值 f(i,j) = max(f(i,j-1), f(i-1,j)) + frame[i][j]。而 f(i,j)
只读本行左侧与上一行同列，把结果直接写回 frame[i][j]，旧值在被读完后
就不再需要，一张表就够。

关键点：
  · 扫描顺序必须是"从上到下、从左到右"，这样左邻与上邻都已被更新
  · 第一行只有左邻、第一列只有上邻，要单独处理，否则会越界
  · 覆盖 frame 会改写输入；本题按平台签名按值接管 frame，改写不会外溢
  · 珠宝价值都大于 0，所以不需要哨兵行/列来兜底；[[0]] 表示空架子，
    累加后仍是 0
  · 答案在 frame[m-1][n-1]

手推一遍：以示例 1（frame = [[1,3,1],[1,5,1],[4,2,1]]）为例：
  第一行累加 → [1,4,5]
  第二行：frame[1][0] = 1+1 = 2
          frame[1][1] = max(4,2)+5 = 9
          frame[1][2] = max(5,9)+1 = 10
  第三行：frame[2][0] = 2+4 = 6
          frame[2][1] = max(9,6)+2 = 11
          frame[2][2] = max(10,11)+1 = 12 → 答案 12 ✔

复杂度分析：
  时间 O(m*n)：每格一次 max 加一次加法
  空间 O(1)：除输入矩阵本身外没有额外分配
*/
pub struct Solution;

impl Solution {
    pub fn jewellery_value(mut frame: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (frame.len(), frame[0].len());

        // 第一行只能一直向右，就地累加成前缀和
        for j in 1..n {
            frame[0][j] += frame[0][j - 1];
        }

        for i in 1..m {
            // 第一列只能一直向下
            let up = frame[i - 1][0];
            frame[i][0] += up;

            for j in 1..n {
                // 左邻与上邻都已是累加值，取较大者再加当前珠宝
                let best = frame[i - 1][j].max(frame[i][j - 1]);
                frame[i][j] += best;
            }
        }
        frame[m - 1][n - 1]
    }
}

// 暴力参考解：把所有路线走一遍取最大价值，只适合小矩阵
fn brute_max(frame: &[Vec<i32>], i: usize, j: usize) -> i32 {
    let (m, n) = (frame.len(), frame[0].len());
    if i == m - 1 && j == n - 1 {
        return frame[i][j];
    }

    let mut best = i32::MIN;
    if i + 1 < m {
        best = best.max(brute_max(frame, i + 1, j));
    }
    if j + 1 < n {
        best = best.max(brute_max(frame, i, j + 1));
    }
    best + frame[i][j]
}

// 就地累加会改写传入的矩阵，所以每个用例只跑一次
fn check(frame: Vec<Vec<i32>>, want: i32) {
    assert_eq!(Solution::jewellery_value(frame), want);
}

// 小矩阵额外走一遍暴力枚举；大矩阵的路线数太多，只验证 DP 结果
fn check_brute(frame: Vec<Vec<i32>>) {
    let want = brute_max(&frame, 0, 0);
    check(frame, want);
}

// 与其它三种语言共用的线性同余发生器，保证四份自测数据一致
fn next_int(seed: &mut u32, modulus: u32) -> i32 {
    *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
    ((*seed >> 16) % modulus) as i32
}

fn main() {
    // 示例 1
    check(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]], 12);

    // 边界：空架子，题面明确给出的唯一 0 值场景
    check(vec![vec![0]], 0);

    // 边界：单行，只能一路向右，答案是整行之和
    check(vec![vec![1, 2, 5]], 8);

    // 边界：单列，只能一路向下
    check(vec![vec![1], vec![2], vec![5]], 8);

    // 边界：单格非 0
    check(vec![vec![7]], 7);

    // 边界：1x2 与 2x1，只有一条路线
    check(vec![vec![3, 4]], 7);
    check(vec![vec![3], vec![4]], 7);

    // 边界：局部最优不等于全局最优 —— 中间的大珠宝只有绕路才拿得到
    check(
        vec![vec![1, 100, 1], vec![1, 1, 1], vec![1, 1, 100]],
        203,
    );

    // 边界：全 1 的 200x200，答案等于路径格子数
    check(vec![vec![1i32; 200]; 200], 200 + 200 - 1);

    // 随机对拍：1..5 的小矩阵、取值 0..9，逐组与暴力枚举比对
    let mut seed = 20260811u32;
    for _ in 0..60 {
        let m = 1 + next_int(&mut seed, 5) as usize;
        let n = 1 + next_int(&mut seed, 5) as usize;
        let mut frame = vec![vec![0i32; n]; m];
        for row in frame.iter_mut() {
            for cell in row.iter_mut() {
                *cell = next_int(&mut seed, 10);
            }
        }
        check_brute(frame);
    }

    println!("all tests passed");
}
