/*
============================================================================
162. 寻找峰值（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-peak-element/
分类：二分查找 · 峰值 · 单峰收缩

峰值元素是指其值严格大于左右相邻值的元素。

给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，
在这种情况下，返回 任何一个峰值 所在位置即可。

你可以假设 nums[-1] = nums[n] = -∞ 。

你必须实现时间复杂度为 O(log n) 的算法来解决此问题。

示例 1：
输入：nums = [1,2,3,1]
输出：2
解释：3 是峰值元素，你的函数应该返回其索引 2。

示例 2：
输入：nums = [1,2,1,3,5,6,4]
输出：1 或 5
解释：你的函数可以返回索引 1，其峰值元素为 2；或者返回索引 5，
其峰值元素为 6。

提示：
- 1 <= nums.length <= 1000
- -2^31 <= nums[i] <= 2^31 - 1
- 对于所有有效的 i 都有 nums[i] != nums[i + 1]
*/
/*
解法：二分向更高的一侧收缩【最优解】

为什么慢：逐个比较 nums[i-1] < nums[i] > nums[i+1] 是 O(n)，题目要求
O(log n)。而且峰值可能不止一个，线性扫描只能给出"第一个"找到的答案，
没有利用数组的局部单调性。

换个角度：把 nums[-1] 和 nums[n] 视作 -∞ 后，"峰值一定存在"可以加强成：
沿任意方向一直往更高的邻元素走，一定会在边界前停下。于是只比较
nums[mid] 与 nums[mid+1]：
  · nums[mid] < nums[mid+1]：右侧更高，峰一定在 [mid+1, right]
  · nums[mid] > nums[mid+1]：左侧不低于右侧，峰一定在 [left, mid]
每轮砍掉一半区间，走到长度为 1 时，那个位置就是峰。

关键点：
  · 区间取左闭右闭 [left, right]，初值 [0, n-1]
  · 循环条件 left < right，mid 取下整，因此 mid < right，nums[mid+1] 不越界
  · nums[mid] < nums[mid+1] 时 left = mid + 1；否则 right = mid
    （mid 自己可能就是峰，必须保留）
  · 题面保证 nums[i] != nums[i+1]，所以只有"上升/下降"两种，不存在相等分支
  · 下标 0 与 n-1 也可以当峰（区间外是 -∞），因此不必特判"峰在内部"

手推一遍：nums = [1,2,1,3,5,6,4]
  left=0,right=6,mid=3，nums[3]=3 < nums[4]=5 → left=4
  left=4,right=6,mid=5，nums[5]=6 > nums[6]=4 → right=5
  left=4,right=5,mid=4，nums[4]=5 < nums[5]=6 → left=5
  left==right==5 → 返回 5，nums[5]=6 是峰 ✔

另一个方向：nums = [5,4,3,2,1] 完全降序
  每轮 nums[mid] > nums[mid+1] 都成立，right 一路收到 0，返回下标 0 ✔

复杂度分析：
  时间 O(log n)：每轮区间长度减半
  空间 O(1)：只用 left / right / mid 三个下标
*/

// 单文件 rustc 编译时没有平台预置的 Solution，这里显式定义
pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        // 不变式：闭区间 [left, right] 内一定存在峰值
        while left < right {
            let mid = left + (right - left) / 2;
            // 右侧更高，峰值不可能停在 mid 或它左边
            if nums[mid] < nums[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

// 题面允许返回任意一个峰，用邻接关系判定，不绑死具体下标
fn is_peak(nums: &[i32], idx: usize) -> bool {
    let left_ok = idx == 0 || nums[idx] > nums[idx - 1];
    let right_ok = idx == nums.len() - 1 || nums[idx] > nums[idx + 1];
    left_ok && right_ok
}

// 唯一峰的用例：答案确定，直接比较下标
fn check_at(nums: &[i32], want: usize) {
    let got = Solution::find_peak_element(nums.to_vec()) as usize;
    assert_eq!(got, want);
    assert!(is_peak(nums, want));
}

// 多峰的用例：只要求返回的确实是某个峰
fn check_any(nums: &[i32]) {
    let got = Solution::find_peak_element(nums.to_vec()) as usize;
    assert!(is_peak(nums, got));
}

fn main() {
    // 示例 1：唯一峰在下标 2
    check_at(&[1, 2, 3, 1], 2);

    // 示例 2：两个峰，两个答案都接受
    check_any(&[1, 2, 1, 3, 5, 6, 4]);

    // 边界：单元素，两侧都是 -∞，它自己就是峰
    check_at(&[1], 0);

    // 边界：完全升序，峰只能在最后一个元素
    check_at(&[1, 2, 3, 4, 5], 4);

    // 边界：完全降序，峰只能在第一个元素
    check_at(&[5, 4, 3, 2, 1], 0);

    // 边界：两个元素，一端必为峰
    check_at(&[1, 2], 1);
    check_at(&[2, 1], 0);

    // 边界：三个元素且峰在正中
    check_at(&[1, 3, 1], 1);
    check_at(&[-5, -3, -7], 1);

    // 边界：峰紧挨着另一个峰，只能返回其中之一
    check_any(&[2, 1, 2]);
    check_any(&[1, 3, 2, 4, 3]);

    // 边界：谷在两端，峰在中间偏右
    check_at(&[3, 2, 1, 2, 3], 4);

    // 边界：长升链，答案确定为末尾
    let nums: Vec<i32> = (0..1000).collect();
    check_at(&nums, 999);

    // 边界：取到 int 上限与下限，比较时不能溢出
    check_at(&[0, i32::MAX, 0], 1);
    // 最小值在正中时它不是峰，二分会被两侧更高的 0 带走，落到下标 2
    check_at(&[0, i32::MIN, 0], 2);

    println!("all tests passed");
}
