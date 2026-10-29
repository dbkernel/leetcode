/*
============================================================================
349. 两个数组的交集（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/intersection-of-two-arrays/
分类：哈希表 · 去重与集合判定

给定两个数组 nums1 和 nums2 ，返回 它们的 交集 。输出结果中的每个元素一定是
唯一 的。我们可以 不考虑输出结果的顺序 。

示例 1：
输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]

示例 2：
输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
解释：[4,9] 也是可通过的

提示：
- 1 <= nums1.length, nums2.length <= 1000
- 0 <= nums1[i], nums2[i] <= 1000
*/
use std::collections::HashSet;

/*
解法一：两个集合【最优解】

为什么慢：最直白的做法是拿 nums1 的每个元素去 nums2 里线性查找，单次判定 O(n)，
总共 O(m·n)。瓶颈很明确：同一个值被反复比较，而题目关心的只是「它到底在不在」。

换个角度：交集只取决于「哪些值同时出现过」，与出现次数无关。把两个数组各自压成
集合，重复元素自动消失，答案的唯一性也就不用额外判断了。

关键点：
  · 两个数组都要先去重，否则 nums1 里的重复值会让答案出现多次
  · 遍历较小的集合：交集大小不超过它，查找次数因此最少
  · 结果顺序任意，不需要排序；题面明确「不考虑输出结果的顺序」

以示例 2（nums1 = [4,9,5]，nums2 = [9,4,9,8,4]）为例：
  集合为 {4,9,5} 与 {4,8,9}
  遍历较小的 {4,9,5}：4 命中、9 命中、5 未命中 → [4,9] ✔

复杂度分析：
  时间 O(m+n)：建两个集合各扫一遍，再遍历较小的集合做 O(1) 判定
  空间 O(m+n)：两个集合最多各存一个数组的全部不同值

解法二：排序 + 双指针【非最优】

为什么慢：时间 O(m log m + n log n)，比哈希解法的线性慢一个 log。换来的是额外
空间只有排序栈的 O(log m + log n)，比两个集合的 O(m+n) 小。按渐近时间判定，它是
次优的那一个。

换个角度：两边都排好序后，交集元素在各自数组里都是递增出现的，于是各拿一个指针
同步前进即可，不需要任何查找结构。

关键点：
  · 相等时两个指针同时右移，否则只移动指向较小值的那一个
  · 收进答案前与上一个收下的值比较，跳过重复 —— 排序后重复元素必然相邻
  · 比较分「小于 / 大于 / 等于」三支，只写相等分支会让指针卡住

以示例 2（nums1 = [4,9,5]，nums2 = [9,4,9,8,4]）为例：
  排序后 [4,5,9] 与 [4,4,8,9,9]
  i=0,j=0：4 == 4，收 4，双双右移
  5 > 4：j 右移两次到 8
  5 < 8：i 右移到 9
  9 == 9：与已收的 4 不同，收 9，双双右移
  i 越界，结束 → [4,9] ✔

复杂度分析：
  时间 O(m log m + n log n)：两个数组的排序是瓶颈，双指针部分只占 O(m+n)
  空间 O(log m + log n)：排序递归栈；原地排序，不用哈希表
*/

// 力扣提交时平台给的是 `impl Solution`，本地单文件 rustc 没有该类型，
// 所以下面显式定义；两个解法各用一个类型，避免同名方法冲突
pub struct Solution1;

impl Solution1 {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();

        // 只遍历较小的集合：交集不可能比它更大
        let (small, big) = if set1.len() <= set2.len() {
            (&set1, &set2)
        } else {
            (&set2, &set1)
        };

        // 两边都已去重，命中即说明该值只该出现一次
        small.iter().filter(|x| big.contains(x)).copied().collect()
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        // 原地排序：复制一份会把额外空间抬到 O(m+n)
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut ans: Vec<i32> = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                i += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else {
                // 与上一个收下的值不同才收，跳过同一值的重复出现
                if ans.last() != Some(&nums1[i]) {
                    ans.push(nums1[i]);
                }
                i += 1;
                j += 1;
            }
        }
        ans
    }
}

// ===== 测试辅助 =====
// 题面说明输出顺序任意，比较前统一排序
fn check(nums1: &[i32], nums2: &[i32], want: &[i32]) {
    let mut got = Solution1::intersection(nums1.to_vec(), nums2.to_vec());
    got.sort_unstable();
    let mut want_sorted = want.to_vec();
    want_sorted.sort_unstable();
    assert_eq!(got, want_sorted, "set({nums1:?}, {nums2:?})");

    // 解法二会原地排序，传入的是新拷贝，不影响上面的调用
    let mut got = Solution2::intersection(nums1.to_vec(), nums2.to_vec());
    got.sort_unstable();
    assert_eq!(got, want_sorted, "sort({nums1:?}, {nums2:?})");
}

fn main() {
    // 示例 1：nums1 里重复的 1、2 不应让答案重复
    check(&[1, 2, 2, 1], &[2, 2], &[2]);

    // 示例 2：输出顺序任意，这里按升序核对
    check(&[4, 9, 5], &[9, 4, 9, 8, 4], &[4, 9]);

    // 边界：没有任何公共元素
    check(&[1, 2], &[3, 4], &[]);

    // 边界：两个数组完全相同且都有重复
    check(&[1, 1, 1], &[1, 1], &[1]);

    // 边界：值域下限 0 也是合法元素
    check(&[0, 1], &[0], &[0]);

    // 边界：单元素相等
    check(&[5], &[5], &[5]);

    // 边界：长度悬殊，交集只剩一个值
    check(&[1, 2, 3, 4, 5], &[3], &[3]);

    // 反直觉：值域上限 1000 与 0 同时出现
    check(&[1000, 0], &[0, 1000], &[0, 1000]);

    println!("all tests passed");
}
