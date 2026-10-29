/*
============================================================================
461. 汉明距离（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/hamming-distance/
分类：位运算 · 异或 + 汉明重量

两个整数之间的 汉明距离 指的是这两个数字对应二进制位不同的位置的数目。

给你两个整数 x 和 y，计算并返回它们之间的汉明距离。

示例 1：
输入：x = 1, y = 4
输出：2
解释：
1   (0 0 0 1)
4   (0 1 0 0)
       ↑   ↑
上面的箭头指出了对应二进制位不同的位置。

示例 2：
输入：x = 3, y = 1
输出：1

提示：
- 0 <= x, y <= 2^31 - 1

注意：本题与 2220. 转换数字的最少位翻转次数 相同。
*/

// 平台上由判题系统预置 Solution 类型，本地用 rustc 单文件编译时需要显式声明
struct Solution1;
struct Solution2;
struct Solution3;

/*
解法一：异或后逐位检查（固定 32 轮）【非最优】

为什么慢：异或已经把「哪些位不同」压进一个数，可逐位检查仍要走满 32 轮 ——
x 与 y 完全相同时答案是 0，也要白跑 32 次。代价与答案无关。

换个角度：答案就是 diff = x ^ y 的汉明重量，于是问题直接退回 191：只有
diff 里那些为 1 的位值得处理。

关键点：
  · 必须先异或再数 1：分别数两边的 1 再相减是错的，1 的个数之差并不等于
    不同位的个数
  · 异或的两个操作数先 `as u32`：x、y 非负，转换不丢位，之后移位语义
    才与「32 位位图」一致
  · 掩码用 1u32 << bit，字面量显式带类型，天然不会溢出到符号位

手推一遍（x = 1，y = 4）：
  0001 ^ 0100 = 0101
  第 0 位为 1 → count = 1；第 2 位为 1 → count = 2
  其余位为 0，跑满 32 轮 → 2 ✔

复杂度分析：
  时间 O(w)：w 固定为 32，与不同的位数无关
  空间 O(1)：只用一个计数器

解法二：异或 + n & (n - 1)【最优解】

为什么慢：解法一慢在轮数与答案脱钩。差异位往往很少，却照样转满 32 轮。

换个角度：既然答案就是 diff 里 1 的个数，就让每一轮抹掉一个 1，
diff &= diff - 1。轮数等于答案，输入越「像」就越快。

关键点：
  · 终止条件是 diff == 0，不能留在 i32 里判 diff > 0 再移位 —— 补码下
    负数永远不为 0，算术右移就是死循环
  · diff 声明为 u32，让清位与计数都落在 32 位域内
  · 计数在清位之后，顺序反了会少算最后一次

手推一遍（x = 1，y = 4）：
  diff = 0101
  0101 & 0100 = 0100 → count = 1
  0100 & 0011 = 0000 → count = 2
  diff == 0 结束 → 2 ✔（只转了 2 轮）

复杂度分析：
  时间 O(k)：k 为不同位的个数，最坏 32
  空间 O(1)：只用一个计数器

解法三：异或 + 内置 popcount【最优解，与解法二并列】

这一条不是「更慢」，而是把数 1 交给标准库：u32::count_ones 在支持 POPCNT
的机器上编译成一条指令，否则也是常数条位运算。

换个角度：汉明距离就是 popcount(x ^ y)，处理器直接支持这个度量。

关键点：
  · count_ones 挂在 u32 上，所以 `(x as u32) ^ (y as u32)` 之后才能调用
  · 返回值是 u32，题目要 i32，再用 `as i32` 收口（至多 32，不会截断）
  · 面试通常要求手写解法二，内置函数只作为工程上的捷径

手推一遍（x = 1，y = 4）：
  (1u32 ^ 4u32).count_ones() → 2 ✔

复杂度分析：
  时间 O(1)：单条指令，或常数条位运算
  空间 O(1)
*/
impl Solution1 {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let diff = (x as u32) ^ (y as u32);
        let mut count = 0;

        for bit in 0..32 {
            if diff & (1u32 << bit) != 0 {
                count += 1;
            }
        }
        count
    }
}

impl Solution2 {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut diff = (x as u32) ^ (y as u32);
        let mut count = 0;

        // diff == 0 是唯一可靠的终止条件：清位一定会把 32 位域清空
        while diff != 0 {
            diff &= diff - 1;
            count += 1;
        }
        count
    }
}

impl Solution3 {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        ((x as u32) ^ (y as u32)).count_ones() as i32
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

fn check(x: i32, y: i32, want: i32) {
    assert_eq!(Solution1::hamming_distance(x, y), want, "shift x={}", x);
    assert_eq!(
        Solution2::hamming_distance(x, y),
        want,
        "clearLowest x={}",
        x
    );
    assert_eq!(Solution3::hamming_distance(x, y), want, "builtin x={}", x);
}

fn main() {
    // 示例 1：差异位是第 0 位与第 2 位
    check(1, 4, 2);

    // 示例 2：只有最低位不同
    check(3, 1, 1);

    // 边界：完全相同，答案是 0，任何一轮都不该被计数
    check(0, 0, 0);
    check(12345, 12345, 0);

    // 边界：顺序无所谓，异或是对称的
    check(4, 1, 2);

    // 边界：第 30 位参与比较（2^30 与 0）
    check(1 << 30, 0, 1);

    // 边界：一方为 0，答案退化为另一方的汉明重量
    check(0, 2147483645, 30);

    println!("all tests passed");
}
