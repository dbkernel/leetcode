/*
============================================================================
LeetCode 1. 两数之和（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/two-sum/
分类：哈希表 · 两数之和查找（暴力枚举 / 一次遍历）
============================================================================

给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出
和为目标值 target 的那 两个 整数，并返回它们的数组下标。

你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。

你可以按任意顺序返回答案。

示例 1：
输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。

示例 2：
输入：nums = [3,2,4], target = 6
输出：[1,2]

示例 3：
输入：nums = [3,3], target = 6
输出：[0,1]

提示：
2 <= nums.length <= 10^4
-10^9 <= nums[i] <= 10^9
-10^9 <= target <= 10^9
只会存在一个有效答案

进阶：你可以想出一个时间复杂度小于 O(n^2) 的算法吗？
*/

use std::collections::HashMap;

// 平台上由判题系统预置 Solution 类型，本地用 rustc 单文件编译时需要显式声明。
// 本题有两种官方解法，按「一解一个结构体」拆成 Solution1 / Solution2；
// 提交时把选用的那个改名为 Solution 即可。
struct Solution1;
struct Solution2;

/*
解法一：暴力枚举（固定左端，只向右找）

为什么慢：对每个 x 都要在数组里找 target - x。若让内层循环从头扫到尾，
每个 x 都要扫 n 个位置，其中一半是和之前已经匹配过的元素重复比较，
总计 O(n^2) 对。

换个角度：(i, j) 与 (j, i) 是同一次配对，只要限定 i < j，每对就只枚举
一次，既不会漏解也不会重复比较。

关键点：
  · 内层从 i + 1 起步 —— 它同时排除了「同一个元素用两次」和重复配对
  · 命中立刻返回 —— 题目保证只有一个答案，继续扫也不会改变结果

以示例 1（nums = [2,7,11,15], target = 9）为例：
  i = 0, x = 2：j = 1 查 2 + 7 = 9 → 命中，返回 [0, 1]

Rust 的两处写法差异：
  · 下标遍历用 0..n 与 (i + 1)..n，取值用 nums[i]（i32 是 Copy，不移动）
  · nums[i] + nums[j] 最大 2*10^9，未超过 i32 上限 2^31-1 ≈ 2.147*10^9，
    所以这里不会溢出

复杂度分析：
  时间 O(n^2)：最坏比较 n(n-1)/2 对
  空间 O(1)：除返回值外只用两个下标
*/
impl Solution1 {
    // 返回一组合法下标 (i, j)，且保证 i < j
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();

        for i in 0..n {
            // 内层从 i + 1 起步：同一个元素不会被用两次
            for j in (i + 1)..n {
                if nums[i] + nums[j] == target {
                    // 题目保证只有一个答案，此处可以安全地直接返回
                    return vec![i as i32, j as i32];
                }
            }
        }

        // 题目保证有解，走不到这里；空 vec 是为了满足返回类型
        vec![]
    }
}

/*
解法二：哈希表（一次遍历）

为什么慢：解法一每次查找 target - x 都要重新扫一遍数组，单次 O(n)。

换个角度：对每个新元素 x，只需在「已遍历的前缀」里反向查找 target - x。
于是把前缀的「值 → 下标」记入 HashMap，查找就从 O(n) 降到 O(1)。

关键点：先查后插，两句话顺序不能反。
  · 先查 —— 表里只有下标 < i 的元素，天然排除「拿自己配自己」
  · 后插 —— nums[i] 要到第 i 步之后才对后续元素可见，保证 i != j
所以 [3, 3], target = 6 也能得到 [0, 1]：第二个 3 查到的是第一个 3。

Rust 的两个类型细节：
  · HashMap::get 返回 Option<&V>，`if let Some(&j)` 一次解引用拿到下标本身
  · 下标是 usize，而返回值要求 i32，需要 `as i32` 转换

同一数值只需留一个下标：题目保证答案唯一，同值元素互换不影响合法性，
因此重复写入覆盖旧下标不会丢失答案（HashMap::insert 同样是覆盖语义）。

以示例 1（nums = [2,7,11,15], target = 9）为例：
  i = 0, x = 2：查 9 - 2 = 7 → 未命中；写入 {2: 0}
  i = 1, x = 7：查 9 - 7 = 2 → 命中下标 0；返回 [0, 1]

复杂度分析：
  时间 O(n)：每个元素进出表各一次，哈希表操作摊还 O(1)
  空间 O(n)：最坏情况下前 n-1 个数都留在表里
*/
impl Solution2 {
    // 返回一组合法下标 (i, j)，满足 nums[i] + nums[j] == target 且 i != j
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 值 → 下标，只保存「已经扫过」的那部分元素；预分配容量省掉扩容 rehash
        let mut seen: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        // 用 iter().enumerate() 同时拿下标和值的引用，避免按索引访问的边界检查
        for (i, &x) in nums.iter().enumerate() {
            // 只在前缀里找：命中时表中存的下标必然 < i，不会是自己
            if let Some(&j) = seen.get(&(target - x)) {
                return vec![j as i32, i as i32];
            }

            // 未命中则登记当前元素，供后续元素匹配；同值重复出现时覆盖旧下标
            seen.insert(x, i);
        }

        // 题目保证有解，走不到这里；空 vec 是为了满足返回类型
        vec![]
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 题目允许任意顺序返回，先排序再比较，断言就不依赖实现的返回顺序
fn same_pair(mut a: Vec<i32>, mut b: Vec<i32>) -> bool {
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

// 同一个用例喂给两种解法：官方两解都必须得到同一组下标
fn check(name: &str, nums: &[i32], target: i32, want: &[i32]) {
    let got1 = Solution1::two_sum(nums.to_vec(), target);
    let got2 = Solution2::two_sum(nums.to_vec(), target);
    assert!(
        same_pair(got1.clone(), want.to_vec()),
        "{}: two_sum1 got {:?}, want {:?}",
        name,
        got1,
        want
    );
    assert!(
        same_pair(got2.clone(), want.to_vec()),
        "{}: two_sum2 got {:?}, want {:?}",
        name,
        got2,
        want
    );
}

fn main() {
    // 示例 1：2 + 7 == 9
    check("example 1", &[2, 7, 11, 15], 9, &[0, 1]);

    // 示例 2：答案不落在开头
    check("example 2", &[3, 2, 4], 6, &[1, 2]);

    // 示例 3：重复值 —— 解法二「先查后插」的关键用例
    check("example 3", &[3, 3], 6, &[0, 1]);

    // 边界：数组含 0，且答案用到下标 0，专防「按下标真假判断」的写法
    check("zero and index 0", &[0, 4, 3, 0], 0, &[0, 3]);

    // 边界：负数参与求和，且答案不与 0 号下标相邻
    check("negative values", &[-3, 4, 3, 90], 0, &[0, 2]);

    // 边界：取到约束上限，验证两数相加不会溢出
    check(
        "upper bound",
        &[1000000000, 1000000000],
        2000000000,
        &[0, 1],
    );

    println!("all tests passed");
}
