/*
============================================================================
LCR 134 / 剑指 Offer 16. Pow(x, n)（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/shu-zhi-de-zheng-shu-ci-fang-lcof/
分类：分治 · 快速幂

实现 pow(x, n) ，即计算 x 的 n 次幂函数（即，x^n）。

示例 1：
输入：x = 2.00000, n = 10
输出：1024.00000

示例 2：
输入：x = 2.10000, n = 3
输出：9.26100

示例 3：
输入：x = 2.00000, n = -2
输出：0.25000
解释：2^-2 = 1/2^2 = 1/4 = 0.25

提示：
- -100.0 < x < 100.0
- -2^31 <= n <= 2^31-1
- -10^4 <= x^n <= 10^4

注意：本题与主站 50 题相同：https://leetcode.cn/problems/powx-n/

说明：本题在 leetcode.cn 已改号为 LCR 134，题名由「数值的整数次方」
改为「Pow(x, n)」，函数签名仍为 `myPow(double, int)`。
*/

/*
解法一：快速幂 + 递归【非最优】

为什么慢：逐次相乘要做 |n| 次乘法，n 接近 2^31 时根本跑不完。指数每
减少一半，规模才会指数下降，所以要把 n 拆成 n/2 的子问题。

换个角度：分治。先递归算出 y = x^(n/2)，偶数时 x^n = y*y，奇数时
再补乘一个 x。递归层数是 log n，而不是 n。

关键点：
  · 负指数先转成 1/x 和 -n；n 必须用 i64 保存，否则 n = i32::MIN 时
    -n 会溢出，仍然停在负数
  · 递归基 n = 0 返回 1.0，这样 x = 0 的 0 次方也按约定得到 1
  · 奇偶判断看 n 的最低位，别把 n/2 和 n%2 的顺序写反

以 x = 2，n = 10 为例：
  n = 10 -> 2^5 -> 2^2 -> 2^1 -> 2^0
  回溯：1, 2, 4, 32, 1024  ✔

复杂度分析：
  时间 O(log n)：每层把指数减半
  空间 O(log n)：递归栈深度等于指数的二进制位数
*/
// rustc 单文件编译时需要显式的类型名，平台提交时合并进 Solution
pub struct Solution1;

impl Solution1 {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // 用 i64 接住 i32::MIN，-n 才不会溢出
        let mut x = x;
        let mut exponent = n as i64;
        if exponent < 0 {
            x = 1.0 / x;
            exponent = -exponent;
        }

        fn quick_mul(x: f64, exponent: i64) -> f64 {
            if exponent == 0 {
                return 1.0;
            }
            let half = quick_mul(x, exponent / 2);
            // 奇数时多乘一个 x，偶数时正好是两个 half 相乘
            if exponent % 2 == 0 {
                half * half
            } else {
                half * half * x
            }
        }

        quick_mul(x, exponent)
    }
}

/*
解法二：快速幂 + 迭代【最优解】

为什么慢：递归版本时间已经是 O(log n)，但每层调用都要压栈，空间是
O(log n)。把指数的二进制拆分自底向上做，可以省掉递归栈。

换个角度：把 n 写成二进制，x^n 等于所有为 1 的二进制位对应的
x^(2^k) 的乘积。从 x 开始不断平方得到 x^1、x^2、x^4……，遇到当前
最低位为 1 就把这份贡献乘进答案。

关键点：
  · 每轮先判断最低位，再平方贡献，最后右移 n；先右移会漏掉当前位
  · 负指数同样先转成 1/x 和 -n，指数用 i64 接住 i32::MIN
  · n 变成 0 时循环结束，答案已经包含所有为 1 的位

以 x = 2，n = 10（二进制 1010）为例：
  初始 ans = 1，contribute = 2，n = 10
  最低位 0：ans 不变，contribute = 4，n = 5
  最低位 1：ans = 4，contribute = 16，n = 2
  最低位 0：ans 不变，contribute = 256，n = 1
  最低位 1：ans = 1024，contribute 平方，n = 0
  返回 1024  ✔

复杂度分析：
  时间 O(log n)：循环次数等于指数的二进制位数
  空间 O(1)：只用了答案和当前贡献两个变量
*/
pub struct Solution2;

impl Solution2 {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut exponent = n as i64;
        if exponent < 0 {
            x = 1.0 / x;
            exponent = -exponent;
        }

        let mut ans = 1.0;
        let mut contribute = x;

        while exponent > 0 {
            // 最低位为 1，说明这一份 x^(2^k) 要计入答案
            if exponent & 1 == 1 {
                ans *= contribute;
            }

            // 下一轮的贡献是当前贡献的平方
            contribute *= contribute;
            exponent >>= 1;
        }

        ans
    }
}

// 浮点结果按相对误差比较，避免 9.261 这类值被二进制表示卡住
fn almost_equal(got: f64, want: f64) -> bool {
    let scale = 1.0f64.max(got.abs()).max(want.abs());
    (got - want).abs() <= 1e-9 * scale
}

fn check(x: f64, n: i32, want: f64) {
    assert!(
        almost_equal(Solution1::my_pow(x, n), want),
        "recursive({}, {}) = {}",
        x,
        n,
        want
    );
    assert!(
        almost_equal(Solution2::my_pow(x, n), want),
        "iterative({}, {}) = {}",
        x,
        n,
        want
    );
}

fn main() {
    // 示例 1
    check(2.0, 10, 1024.0);

    // 示例 2
    check(2.1, 3, 9.261);

    // 示例 3：负指数
    check(2.0, -2, 0.25);

    // 边界：任何非零数的 0 次方都是 1
    check(2.0, 0, 1.0);
    check(0.0, 0, 1.0);

    // 边界：0 的正数次方是 0
    check(0.0, 5, 0.0);

    // 边界：负底数，奇数指数保留负号
    check(-2.0, 3, -8.0);

    // 边界：负指数且底数为 2
    check(2.0, -1, 0.5);

    // 边界：i32::MIN 指数，验证 -n 不会溢出
    check(1.0, i32::MIN, 1.0);
    check(-1.0, i32::MIN, 1.0);

    println!("all tests passed");
}
