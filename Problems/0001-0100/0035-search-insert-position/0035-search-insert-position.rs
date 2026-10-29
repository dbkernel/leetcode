/*
============================================================================
35. 搜索插入位置（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/search-insert-position/
分类：二分查找 · 下界

给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。
如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

请必须使用时间复杂度为 O(log n) 的算法。

示例 1:
输入: nums = [1,3,5,6], target = 5
输出: 2

示例 2:
输入: nums = [1,3,5,6], target = 2
输出: 1

示例 3:
输入: nums = [1,3,5,6], target = 7
输出: 4

提示:
- 1 <= nums.length <= 10^4
- -10^4 <= nums[i] <= 10^4
- nums 为 无重复元素 的 升序 排列数组
- -10^4 <= target <= 10^4
*/

/*
解法：左闭右开二分，返回插入点【最优解】

为什么慢：从左往右找第一个不小于 target 的元素，最坏要扫完整个数组，
O(n)。但数组升序，一次比较就能排除一半候选，这个信息线性扫描没用上。

换个角度：插入位置就是「第一个 >= target 的下标」，也就是下界
（lower_bound）。target 存在时它是命中位置，不存在时它正好是 target
该插进去的位置，两种情况的答案可以用同一套逻辑拿到。

关键点：
  · 用左闭右开区间 [left, right)，right 初值取 n 而不是 n - 1
  · 循环条件 left < right，结束时 left == right，区间收缩成唯一的插入点
  · 判断只写 `nums[mid] < target`：成立时 mid 及其左边都不可能是答案
  · 不必单独写 `nums[mid] == target` 提前返回，第一个 >= target 的位置
    对「存在」和「不存在」都是正确答案
  · target 大于所有元素时 left 会走到 n，这正好表示插到末尾，无需特判

以示例 2（nums = [1,3,5,6], target = 2）为例：
  left=0, right=4 → mid=2，nums[2]=5 >= 2 → right=2
  left=0, right=2 → mid=1，nums[1]=3 >= 2 → right=1
  left=0, right=1 → mid=0，nums[0]=1 < 2 → left=1
  left == right == 1 → 返回 1 ✔

以示例 3（nums = [1,3,5,6], target = 7）为例：
  每轮 mid 处的值都 < 7，left 一路推到 4 = n → 返回 4 ✔

复杂度分析：
  时间 O(log n)：每轮区间减半
  空间 O(1)：只用两个下标
*/

// rustc 单文件编译时没有平台预置的 Solution，这里显式定义
pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0usize, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;
            // nums[mid] 严格小于 target，它不可能是插入点
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // 循环结束时区间为空，left 就是第一个 >= target 的位置
        left as i32
    }
}

fn check(nums: &[i32], target: i32, want: i32) {
    let got = Solution::search_insert(nums.to_vec(), target);
    assert_eq!(got, want, "nums = {:?}, target = {}", nums, target);
}

fn main() {
    // 示例 1
    check(&[1, 3, 5, 6], 5, 2);

    // 示例 2
    check(&[1, 3, 5, 6], 2, 1);

    // 示例 3：插到末尾
    check(&[1, 3, 5, 6], 7, 4);

    // 边界：插到最前面
    check(&[1, 3, 5, 6], 0, 0);
    check(&[1, 3, 5, 6], -10000, 0);

    // 边界：单元素数组的三种情况
    check(&[1], 0, 0);
    check(&[1], 1, 0);
    check(&[1], 2, 1);

    // 边界：空数组（题面约束 n >= 1，这里只验证健壮性）
    check(&[], 5, 0);

    // 边界：命中首尾元素
    check(&[1, 3, 5, 6], 1, 0);
    check(&[1, 3, 5, 6], 6, 3);

    // 边界：取值范围两端
    check(&[-10000, 0, 10000], -10000, 0);
    check(&[-10000, 0, 10000], 10000, 2);
    check(&[-10000, 0, 10000], 9999, 2);

    // 边界：重复元素（题面保证无重复，这里验证取第一个 >= 的位置）
    check(&[1, 3, 3, 5], 3, 1);

    // 规模：n = 10^4 的连续整数
    let nums: Vec<i32> = (0..10000).collect();
    check(&nums, -1, 0);
    check(&nums, 5000, 5000);
    check(&nums, 10000, 10000);

    println!("all tests passed");
}
