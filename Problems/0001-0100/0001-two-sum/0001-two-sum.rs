/*
============================================================================
LeetCode 1. 两数之和（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/two-sum/
分类：哈希表 · 一次遍历
============================================================================

给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那 两个 整数，并返回它们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
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

// 平台上由判题系统预置该类型，本地用 rustc 单文件编译时需要显式声明
struct Solution;

/*
解法：哈希表（一次遍历）

朴素做法：对每个 x 再向后扫一遍找 target - x，单次查找 O(n)，总计 O(n^2)。
换个角度看：对每个新元素 x，只需在「已遍历的前缀」里反向查找 target - x。
于是把前缀的「值 → 下标」记入 HashMap，查找就从 O(n) 降到 O(1)。

关键：先查后插，两句话顺序不能反。
  · 先查 —— 表里只有下标 < i 的元素，天然排除「拿自己配自己」
  · 后插 —— nums[i] 要到第 i 步之后才对后续元素可见，保证 i != j
所以 [3, 3], target = 6 也能得到 [0, 1]：第二个 3 查到的是第一个 3。

Rust 的两个类型细节：
  · HashMap::get 返回 Option<&V>，`if let Some(&j)` 一次解引用拿到下标本身
  · 下标是 usize，而返回值要求 i32，需要 `as i32` 转换；
    题目约束 |nums[i]|, |target| <= 10^9，target - x 的绝对值不超过 2*10^9，
    未超出 i32 上限 2^31-1 ≈ 2.15*10^9，所以这里不会溢出

同一数值只需留一个下标：题目只要求任意一组解，同值元素互换不影响合法性，
因此重复写入覆盖旧下标不会丢失答案（HashMap::insert 同样是覆盖语义）。

以示例 1（nums = [2,7,11,15], target = 9）为例：
  i = 0, x = 2：查 9 - 2 = 7 → 未命中；写入 {2: 0}
  i = 1, x = 7：查 9 - 7 = 2 → 命中下标 0；返回 [0, 1]

复杂度分析：
  时间 O(n)：每个元素进出表各一次，哈希表操作摊还 O(1)
  空间 O(n)：最坏情况下前 n-1 个数都留在表里
*/
impl Solution {
    // 返回任意一组两个下标 (i, j)，满足 nums[i] + nums[j] == target 且 i != j
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 值 → 下标，只保存「已经扫过」的那部分元素；预分配容量省掉扩容时的 rehash
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

        // 题目保证有解，走不到这里；空 vec 是为了满足返回类型，避免 unwrap 或 panic
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

fn main() {
    // 示例 1：2 + 7 == 9
    assert!(same_pair(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]));

    // 示例 2：答案不落在开头
    assert!(same_pair(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]));

    // 示例 3：重复值 —— 「先查后插」的关键用例
    assert!(same_pair(Solution::two_sum(vec![3, 3], 6), vec![0, 1]));

    // 重复值本身不是答案：重复写入覆盖旧下标，但不得丢失解
    assert!(same_pair(Solution::two_sum(vec![3, 3, 5], 8), vec![1, 2]));

    println!("all tests passed");
}
