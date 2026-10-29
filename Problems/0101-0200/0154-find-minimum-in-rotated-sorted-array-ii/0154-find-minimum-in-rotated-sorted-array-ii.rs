/*
============================================================================
154. 寻找旋转排序数组中的最小值 II（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array-ii/
分类：二分查找 · 旋转数组 · 重复元素

已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到
输入数组。例如，原数组 nums = [0,1,4,4,5,6,7] 在变化后可能得到：
若旋转 4 次，则可以得到 [4,5,6,7,0,1,4]
若旋转 7 次，则可以得到 [0,1,4,4,5,6,7]
注意，数组 [a[0], a[1], a[2], ..., a[n-1]] 旋转一次 的结果为数组
[a[n-1], a[0], a[1], a[2], ..., a[n-2]] 。

给你一个可能存在 重复 元素值的数组 nums ，它原来是一个升序排列的数组，并按
上述情形进行了多次旋转。请你找出并返回数组中的 最小元素 。

你必须尽可能减少整个操作步骤。

示例 1：
输入：nums = [1,3,5]
输出：1

示例 2：
输入：nums = [2,2,2,0,1]
输出：0

提示：
- n == nums.length
- 1 <= n <= 5000
- -5000 <= nums[i] <= 5000
- nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转

进阶：
这道题是 寻找旋转排序数组中的最小值 的延伸题目。
允许重复会影响算法的时间复杂度吗？会如何影响，为什么？
*/
/*
解法：与右端点比较，相等时只收缩右端【最优解】

为什么慢：有重复元素后，nums[mid] 与 nums[right] 可能相等，此时无法判断
mid 在第一段还是第二段 —— [1,1,1,0,1] 和 [1,0,1,1,1] 都满足 mid 与
right 同值。照搬 153 的规则任选一边，都可能把最小值所在的一半扔掉。

换个角度：nums[mid] == nums[right] 时，唯一确定的是这两个位置同值。
由于 mid < right，即使 nums[right] 就是最小值，nums[mid] 也是同值的最小值，
所以丢掉最右端不会丢掉"最小值这个值"。于是三种情况分开处理：
  · nums[mid] > nums[right]：最小值在 mid 右侧，left = mid + 1
  · nums[mid] < nums[right]：最小值在 mid 或其左侧，right = mid
  · 相等：right = right - 1，一次只剥离一个元素

关键点：
  · 区间取左闭右闭 [left, right]，循环条件 left < right，答案始终留在区间内
  · 相等分支必须是 right = right - 1，不能写 right = mid
    （相等说明 [mid, right] 可能整段都是最小值，只能逐个削掉右端）
  · 大于分支写 left = mid + 1，因为 mid 的值严格大于右端，不可能是最小值
  · 小于分支写 right = mid，mid 可能就是最小值，必须保留
  · 退出时 left == right，返回 nums[left]

手推一遍：nums = [1,0,1,1,1,1,1]
  left=0,right=6,mid=3，nums[3]=1 == nums[6]=1 → right=5
  left=0,right=5,mid=2，nums[2]=1 == nums[5]=1 → right=4
  left=0,right=4,mid=2，nums[2]=1 == nums[4]=1 → right=3
  left=0,right=3,mid=1，nums[1]=0 < nums[3]=1 → right=1
  left=0,right=1,mid=0，nums[0]=1 > nums[1]=0 → left=1
  left==right==1 → 返回 nums[1]=0 ✔
  前四轮都在削重复值，正是最坏 O(n) 的来源

复杂度分析：
  时间 平均 O(log n)，最坏 O(n)：形如 [1,0,1,1,...,1] 时，
    每轮 right 只左移一格
  空间 O(1)：只用 left / right / mid 三个下标
*/

// 单文件 rustc 编译时没有平台预置的 Solution，这里显式定义
pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        // 不变式：最小值始终落在闭区间 [left, right] 内
        while left < right {
            let mid = left + (right - left) / 2;
            let m = nums[mid as usize];
            let r = nums[right as usize];
            if m > r {
                left = mid + 1;
            } else if m < r {
                right = mid;
            } else {
                right -= 1; // 相等时无法二分，只能削掉右端这一个元素
            }
        }
        nums[left as usize]
    }
}

// 单解法也要覆盖全部用例，失败时能看出是哪一组数据
fn check(nums: &[i32], want: i32) {
    assert_eq!(Solution::find_min(nums.to_vec()), want);
}

fn main() {
    // 示例 1：旋转点在首
    check(&[1, 3, 5], 1);

    // 示例 2：重复的最大值围住最小值
    check(&[2, 2, 2, 0, 1], 0);

    // 边界：单元素
    check(&[1], 1);

    // 边界：完全升序（旋转 n 次），最小值在下标 0
    check(&[1, 2, 3, 4, 5], 1);

    // 边界：只旋转一次，最小值在末尾（旋转点在尾）
    check(&[2, 3, 4, 5, 1], 1);

    // 边界：全相等，三个分支里只剩相等分支被走到
    check(&[2, 2, 2, 2], 2);
    check(&[0, 0, 0], 0);

    // 反直觉：最小值被重复值包住，必须连续削减右端才能露出 0
    check(&[1, 0, 1, 1, 1, 1, 1], 0);
    check(&[1, 1, 1, 0, 1], 0);
    check(&[10, 1, 10, 10, 10], 1);

    // 边界：两个元素
    check(&[2, 1], 1);
    check(&[1, 1], 1);

    // 边界：重复的最小值成段出现
    check(&[2, 2, 0, 0, 1, 2], 0);

    // 边界：负数值参与比较
    check(&[-1, -1, -2], -2);
    check(&[-3, -3, -3], -3);

    // 说明：整体降序（如 [5,4,3,2,1]）不是升序数组的旋转结果，
    // 不能作为合法输入；"局部降序"由只旋转一次的用例覆盖

    println!("all tests passed");
}
