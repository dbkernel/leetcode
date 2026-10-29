/*
============================================================================
238. 除自身以外数组的乘积（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/product-of-array-except-self/
分类：前缀后缀统计 · 左右乘积

给你一个整数数组 nums，返回 数组 answer ，其中 answer[i] 等于 nums 中
除了 nums[i] 之外其余各元素的乘积 。

题目数据 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在
32 位 整数范围内。

请 不要使用除法，且在 O(n) 时间复杂度内完成此题。

示例 1：
输入: nums = [1,2,3,4]
输出: [24,12,8,6]

示例 2：
输入: nums = [-1,1,0,-3,3]
输出: [0,0,9,0,0]

提示：
- 2 <= nums.length <= 10^5
- -30 <= nums[i] <= 30
- 输入 保证 数组 answer[i] 在 32 位 整数范围内

进阶：你可以在 O(1) 的额外空间复杂度内完成这个题目吗？
（ 出于对空间复杂度分析的目的，输出数组 不被视为 额外空间。）
*/

/*
解法一：左右乘积数组【非最优】

为什么慢：按定义，answer[i] 是「左边所有数之积」再乘「右边所有数之积」。
若对每个 i 都把两侧各扫一遍，就是 O(n^2)。改用前缀积、后缀积把它降到 O(n)
之后，时间已经到底，剩下能省的是空间 —— 这里多开了两个长度 n 的数组。

换个角度：把「除自己以外的乘积」拆成两段必有交集的段：左边 nums[0..i-1]
和右边 nums[i+1..n-1]。两段各自都能用一次累乘递推出来，互不依赖。

关键点：
  · 空段乘积约定为 1：left[0] = 1、right[n-1] = 1，否则长度 2 的数组会全变 0
  · 不能用除法绕开：数组含一个 0 时整段乘积归零，含两个 0 时还要处理 0/0
  · 题设保证每个前缀、后缀乘积都在 32 位整数范围内，用 i32 累乘不会溢出
  · 递推方向要相反：left 从左往右，right 从右往左

手推一遍：
以 nums = [1,2,3,4] 为例：
  left  = [1, 1, 2, 6]（left[i] 是 i 左边所有数的积）
  right = [24, 12, 4, 1]（right[i] 是 i 右边所有数的积）
  逐项相乘 → [24, 12, 8, 6] ✔

复杂度分析：
  时间 O(n)：三次线性扫描
  空间 O(n)：left 与 right 各占 n 个 i32，是本题可省掉的那部分

解法二：结果数组存左乘积，一个变量滚动右乘积【最优解】

为什么慢：解法一的两个辅助数组其实有一个是多余的 —— answer 本身就要返回，
第一趟把左乘积直接写进 answer，就省掉了 left。

换个角度：正序一趟让 answer[i] 变成「左边所有数的积」，再倒序一趟用一个
变量 right 携带「右边所有数的积」，边走边乘进 answer。

关键点：
  · 倒序时先乘后更新：先 answer[i] *= right，再把 nums[i] 并进 right；
    顺序一颠倒，nums[i] 就会被算进它自己的答案
  · right 初值取 1，对应「最右元素右边是空段」
  · 全程不用除法，含 0、含两个 0 的数组都不需要特判
  · 额外空间只有 right 一个变量，输出数组按题面约定不计入

手推一遍：
以 nums = [1,2,3,4] 为例：
  正序 → answer = [1, 1, 2, 6]（此时 answer[i] 是左乘积）
  倒序：i=3 right=1  → answer[3] = 6，right 更新为 4
        i=2 right=4  → answer[2] = 8，right 更新为 12
        i=1 right=12 → answer[1] = 12，right 更新为 24
        i=0 right=24 → answer[0] = 24 ✔

复杂度分析：
  时间 O(n)：两趟线性扫描
  空间 O(1)：除返回数组外只用 right 一个变量
*/

// 本地 rustc 单文件编译时平台类型不存在，需显式声明
pub struct Solution1;

impl Solution1 {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        // 空段乘积为 1，天然覆盖 i == 0 与 i == n-1
        let mut left = vec![1; n];
        let mut right = vec![1; n];

        for i in 1..n {
            left[i] = left[i - 1] * nums[i - 1];
        }
        for i in (0..n - 1).rev() {
            right[i] = right[i + 1] * nums[i + 1];
        }

        (0..n).map(|i| left[i] * right[i]).collect()
    }
}

// ---------- 解法二：结果数组存左乘积，一个变量滚动右乘积 ----------
pub struct Solution2;

impl Solution2 {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];

        // 第一趟：ans[i] 变成 nums[0..i-1] 的积
        for i in 1..n {
            ans[i] = ans[i - 1] * nums[i - 1];
        }

        let mut right = 1; // 当前元素右侧所有数的积，最右元素右侧是空段
        for i in (0..n).rev() {
            // 先乘右侧积，再把 nums[i] 并入 right，否则会算进自己的答案
            ans[i] *= right;
            right *= nums[i];
        }
        ans
    }
}

// ===== 测试辅助：两个解法共用一份输入，逐个断言 =====
fn check(nums: &[i32], want: &[i32]) {
    assert_eq!(Solution1::product_except_self(nums.to_vec()), want);
    assert_eq!(Solution2::product_except_self(nums.to_vec()), want);
}

fn main() {
    // 示例 1：全正数，验证左右两段的组合
    check(&[1, 2, 3, 4], &[24, 12, 8, 6]);

    // 示例 2：含一个 0，只有 0 所在位置非零
    check(&[-1, 1, 0, -3, 3], &[0, 0, 9, 0, 0]);

    // 边界：单个 0 且 0 在头部
    check(&[0, 1, 2], &[2, 0, 0]);

    // 边界：两个 0，所有位置的答案都是 0（除法解法会在这里崩掉）
    check(&[0, 0, 1], &[0, 0, 0]);

    // 边界：最小长度 2
    check(&[1, 2], &[2, 1]);
    check(&[0, 5], &[5, 0]);

    // 边界：负数个数影响符号，三个负数结果全正
    check(&[-2, -3, -4], &[12, 8, 6]);

    // 边界：取值上下限附近，乘积仍在 32 位内
    check(&[-30, 30, -1], &[-30, 30, -900]);

    println!("all tests passed");
}
