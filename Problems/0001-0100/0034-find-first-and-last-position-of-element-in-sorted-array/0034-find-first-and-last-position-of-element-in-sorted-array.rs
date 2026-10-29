/*
============================================================================
34. 在排序数组中查找元素的第一个和最后一个位置（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/
分类：二分查找 · 左右边界

给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。
请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 target，返回 [-1, -1]。

你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。

示例 1：
输入：nums = [5,7,7,8,8,10], target = 8
输出：[3,4]

示例 2：
输入：nums = [5,7,7,8,8,10], target = 6
输出：[-1,-1]

示例 3：
输入：nums = [], target = 0
输出：[-1,-1]

提示：
- 0 <= nums.length <= 10^5
- -10^9 <= nums[i] <= 10^9
- nums 是一个非递减数组
- -10^9 <= target <= 10^9
*/

/*
解法一：一次二分命中后向两侧线性扩展【非最优】

为什么慢：二分 O(log n) 就能命中任意一个 target，但左右边界还要从命中点
向两边逐个走。数组里全是 target 时（如 n 个 8），这一步要扫完整个数组，
整体退化成 O(n)，不满足题目要求的 O(log n)。

换个角度：左右边界本质上各自是一次「找第一个满足条件的位置」：
左边界是第一个 >= target 的位置，右边界是第一个 > target 的位置减一。
把「逐个走」换成「再二分一次」，线性项就消失了。

关键点：
  · 先二分找到任意一个 target 作为起点，没找到直接返回 [-1, -1]
  · 向左扩展前先判 lo > 0，再取 nums[lo - 1]，否则会越界
  · 向右扩展同理，先判 hi + 1 < n
  · 扩展用 while 而不是 if，因为连续 target 段可能很长
  · 这段思路只用来对照，规模大且 target 密集时会超时

以示例 1（nums = [5,7,7,8,8,10], target = 8）为例：
  二分命中下标 4（值 8）
  向左：nums[3] = 8 → lo = 3；nums[2] = 7 停
  向右：nums[5] = 10 停 → hi = 4
  返回 [3,4] ✔

复杂度分析：
  时间 O(log n + k)：k 为 target 的出现次数，全相同时退化为 O(n)
  空间 O(1)：只用了常数个下标
*/

// rustc 单文件编译时没有平台预置的 Solution，这里显式定义
pub struct Solution1;

impl Solution1 {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len() as i64;
        let (mut left, mut right) = (0i64, n - 1);
        let mut hit: i64 = -1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                hit = mid;
                break;
            }
            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        if hit == -1 {
            return vec![-1, -1];
        }

        let (mut lo, mut hi) = (hit, hit);
        // 先判下标再取值，避免 hit 正好在端点时越界
        while lo > 0 && nums[(lo - 1) as usize] == target {
            lo -= 1;
        }
        while hi + 1 < n && nums[(hi + 1) as usize] == target {
            hi += 1;
        }

        vec![lo as i32, hi as i32]
    }
}

// 第一个 >= target 的下标；全部小于 target 时返回 n
fn lower_bound(nums: &[i32], target: i32) -> usize {
    let (mut left, mut right) = (0usize, nums.len());
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

// 第一个 > target 的下标；全部不大于 target 时返回 n
fn upper_bound(nums: &[i32], target: i32) -> usize {
    let (mut left, mut right) = (0usize, nums.len());
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

/*
解法二：两次二分分别找左右边界【最优解】

为什么慢：解法一在命中的那一段上仍然线性扫描。target 越密集，这段扫描
越长，最坏 O(n)，没有把「有序」这个前提用满。

换个角度：两个边界都可以用同一个下界函数表达：
  · 左边界 = 第一个 >= target 的位置；
  · 右边界 = 第一个 > target 的位置减一。
所以只要写一个能接受「比较条件」的下界二分，调用两次即可。

关键点：
  · lower_bound 用 `nums[mid] < target` 收缩左界
  · upper_bound 用 `nums[mid] <= target` 收缩左界，差别只在等号
  · 先拿 lower_bound 判断存在性：越界或值不等就返回 [-1, -1]，
    这样后面求右边界时不会出现空区间
  · 右边界用 upper_bound - 1，而不是再写一个「最后一个 == target」的二分，
    避免两套边界条件记混
  · target 等于 10^9 时也不要写 target + 1 来找上界，直接比较 <= 更安全

以示例 1（nums = [5,7,7,8,8,10], target = 8）为例：
  lower_bound：第一个 >= 8 是下标 3
  upper_bound：第一个 > 8 是下标 5，减一得 4
  返回 [3,4] ✔

以示例 2（nums = [5,7,7,8,8,10], target = 6）为例：
  lower_bound 返回 3，nums[3] = 8 != 6 → 返回 [-1,-1] ✔

复杂度分析：
  时间 O(log n)：两次二分，每次区间减半
  空间 O(1)：只用了常数个下标
*/
pub struct Solution2;

impl Solution2 {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lo = lower_bound(&nums, target);
        // 越界说明 target 比所有元素都大；值不等说明不存在
        if lo == nums.len() || nums[lo] != target {
            return vec![-1, -1];
        }

        // upper_bound 指向第一个大于 target 的位置，减一即最后一个 target
        let hi = upper_bound(&nums, target) - 1;
        vec![lo as i32, hi as i32]
    }
}

fn check(nums: &[i32], target: i32, want: &[i32]) {
    let got1 = Solution1::search_range(nums.to_vec(), target);
    assert_eq!(got1, want, "brute nums = {:?}, target = {}", nums, target);
    let got2 = Solution2::search_range(nums.to_vec(), target);
    assert_eq!(got2, want, "optimal nums = {:?}, target = {}", nums, target);
}

fn main() {
    // 示例 1
    check(&[5, 7, 7, 8, 8, 10], 8, &[3, 4]);

    // 示例 2
    check(&[5, 7, 7, 8, 8, 10], 6, &[-1, -1]);

    // 示例 3：空数组
    check(&[], 0, &[-1, -1]);

    // 边界：单元素命中 / 未命中
    check(&[1], 1, &[0, 0]);
    check(&[1], 0, &[-1, -1]);

    // 边界：target 在数组两端
    check(&[1, 2, 3], 1, &[0, 0]);
    check(&[1, 2, 3], 3, &[2, 2]);

    // 边界：target 小于最小值 / 大于最大值
    check(&[1, 2, 3], 0, &[-1, -1]);
    check(&[1, 2, 3], 4, &[-1, -1]);

    // 边界：全部元素相同，解法一的线性扩展会走满整个数组
    check(&[2, 2, 2, 2], 2, &[0, 3]);
    check(&[2, 2, 2, 2], 3, &[-1, -1]);

    // 边界：取值范围的极值
    check(
        &[-1000000000, -1000000000, 1000000000],
        -1000000000,
        &[0, 1],
    );
    check(&[-1000000000, 0, 1000000000], 1000000000, &[2, 2]);

    // 规模：10^5 个相同元素，验证两次二分不随重复次数退化
    let nums = vec![5; 100000];
    check(&nums, 5, &[0, 99999]);
    check(&nums, 4, &[-1, -1]);
    check(&nums, 6, &[-1, -1]);

    println!("all tests passed");
}
