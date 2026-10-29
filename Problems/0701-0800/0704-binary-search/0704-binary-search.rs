/*
============================================================================
704. 二分查找（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/binary-search/
分类：二分查找

给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target ，写一个函数搜索
nums 中的 target，如果 target 存在返回下标，否则返回 -1。

你必须编写一个具有 O(log n) 时间复杂度的算法。

示例 1:
输入: nums = [-1,0,3,5,9,12], target = 9
输出: 4
解释: 9 出现在 nums 中并且下标为 4

示例 2:
输入: nums = [-1,0,3,5,9,12], target = 2
输出: -1
解释: 2 不存在 nums 中因此返回 -1

提示：
你可以假设 nums 中的所有元素是不重复的。
n 将在 [1, 10000] 之间。
nums 的每个元素都将在 [-9999, 9999] 之间。
*/

/*
解法一：闭区间二分查找【最优解】

官方题解只给出这一种方法，也就是本题的最优解：时间 O(log n)、空间 O(1)。

为什么慢：线性扫描要逐个比较，O(n)。但数组是升序的，这个「有序」信息可以让我们
一次比较就排除一半候选——这正是二分的前提。

换个角度：维护一个「目标可能存在」的闭区间 [left, right]。每次取中点 mid：
  · nums[mid] == target：直接命中；
  · nums[mid] > target：target 只可能在 mid 左边，令 right = mid - 1；
  · nums[mid] < target：target 只可能在 mid 右边，令 left = mid + 1。
当区间为空时说明不存在，返回 -1。

关键点：
  · 区间必须是「闭区间」写法（right 初值为 n - 1，循环条件 left <= right），
    这样每次收缩都是 mid ± 1，不会漏元素，也不会死循环
  · 用 usize 表示下标时，`right = mid - 1` 在 mid == 0 时会下溢；
    所以这里统一用 isize，下界为 -1 时循环条件自然为假
  · mid = left + (right - left) / 2 比 (left + right) / 2 更稳，是应该养成的习惯
  · 循环条件是 left <= right 而不是 left < right：漏掉等号会把「区间只剩一个元素」
    的情况跳过，从而漏答案
  · 每次比较要么命中、要么区间严格缩小，所以循环必然结束
  · 题目保证元素不重复，因此找到任意一个就是唯一答案

以 nums = [-1,0,3,5,9,12], target = 9 为例：
  left=0, right=5 → mid=2，nums[2]=3 < 9 → left=3
  left=3, right=5 → mid=4，nums[4]=9 == 9 → 返回 4

以 nums = [-1,0,3,5,9,12], target = 2 为例：
  left=0, right=5 → mid=2，nums[2]=3 > 2 → right=1
  left=0, right=1 → mid=0，nums[0]=-1 < 2 → left=1
  left=1, right=1 → mid=1，nums[1]=0 < 2 → left=2
  left=2 > right=1 → 返回 -1

复杂度分析：
  时间 O(log n)：每次迭代区间长度减半
  空间 O(1)：只用三个下标
*/
struct Solution;

impl Solution {
    /// 解法一：闭区间二分查找（最优解）
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: isize = 0;
        let mut right: isize = nums.len() as isize - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let v = nums[mid as usize];
            if v == target {
                return mid as i32;
            }
            if v > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        -1
    }
}

fn check(nums: &[i32], target: i32, want: i32) {
    let got = Solution::search(nums.to_vec(), target);
    assert_eq!(got, want, "search({:?}, {}) = {}, want {}", nums, target, got, want);
}

fn main() {
    // 示例 1
    check(&[-1, 0, 3, 5, 9, 12], 9, 4);

    // 示例 2
    check(&[-1, 0, 3, 5, 9, 12], 2, -1);

    // 边界：单元素命中 / 未命中
    check(&[5], 5, 0);
    check(&[5], -5, -1);

    // 边界：目标在两端
    check(&[-1, 0, 3, 5, 9, 12], -1, 0);
    check(&[-1, 0, 3, 5, 9, 12], 12, 5);

    // 边界：目标比最小值还小 / 比最大值还大
    check(&[-1, 0, 3, 5, 9, 12], -2, -1);
    check(&[-1, 0, 3, 5, 9, 12], 13, -1);

    // 边界：取值范围的极值
    check(&[-9999, 0, 9999], -9999, 0);
    check(&[-9999, 0, 9999], 9999, 2);

    // 边界：偶数长度，mid 落在偏左位置
    check(&[1, 2, 3, 4], 3, 2);
    check(&[1, 2, 3, 4], 4, 3);

    // 边界：上限规模 n = 10000，逐个元素都应该能查到
    {
        let nums: Vec<i32> = (-5000..5000).collect();
        for (i, &v) in nums.iter().enumerate() {
            check(&nums, v, i as i32);
        }
        check(&nums, 5001, -1);
    }

    println!("all tests passed");
}
