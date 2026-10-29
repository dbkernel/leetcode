/*
============================================================================
LCR 180 / 剑指 Offer 57 - II. 文件组合（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/he-wei-sde-lian-xu-zheng-shu-xu-lie-lcof/
分类：滑动窗口 · 双指针（连续正整数的和）
说明：本题即《剑指 Offer（第 2 版）》第 57 题第 II 问「和为 s 的连续正
      数序列」，原题面是「输入一个正整数 target，输出所有和为 target 的
      连续正整数序列，至少含有两个数」。力扣现行题面改名为「文件组合」，
      输入仍是 target，签名由 findContinuousSequence(target) 改为
      fileCombination(target)。

待传输文件被切分成多个部分，按照原排列顺序，每部分文件编号均为一个 正整
数（至少含有两个文件）。传输要求为：连续文件编号总和为接收方指定数字
target 的所有文件。请返回所有符合该要求的文件传输组合列表。

注意，返回时需遵循以下规则：
- 每种组合按照文件编号 升序 排列；
- 不同组合按照第一个文件编号 升序 排列。

示例 1：
输入：target = 12
输出：[[3, 4, 5]]
解释：在上述示例中，存在一个连续正整数序列的和为 12，为 [3, 4, 5]。

示例 2：
输入：target = 18
输出：[[3,4,5,6],[5,6,7]]
解释：在上述示例中，存在两个连续正整数序列的和分别为 18，分别为
[3, 4, 5, 6] 和 [5, 6, 7]。

提示：
- 1 <= target <= 10^5
*/

/*
Rust 没有平台预置的 Solution 类型，单文件 rustc 编译时必须自己声明。
解法只有一条，所以直接用 Solution，与平台签名保持一致。
*/

/*
解法：滑动窗口（左右指针 + 当前和）【最优解】

为什么慢：最直白的写法是枚举左端点，再枚举右端点并把区间里的数累加起
来，看和是否等于 target。每个左端点都要重新累加一遍，target 取到 10^5
时是 O(target^2) 级别；即便先用前缀和把每次求和降到 O(1)，枚举所有
(left, right) 对本身仍是 O(target^2)。

换个角度：连续正整数序列的和随右端点右移单调递增、随左端点右移单调递
减，这正是双指针能用的前提。固定左端点时，和一旦超过 target，再往右加
只会更大，这个左端点就废了；和不足时只需右移右端点。两个指针都只前进
不后退，每个数最多被加一次、减一次。

关键点：
  · 窗口取闭区间 [left, right]，sum 恒等于这段区间的和；三个分支
    各自只动一个指针，不要一边记录一边又收缩两次
  · sum == target 时先把 [left, right] 收进答案，再左移一格；序列按
    首元素升序输出，正是因为 left 单调递增
  · 循环条件 left <= target / 2：窗口至少两个数，最小的两数和为
    2*left + 1，left 再大它就已经超过 target，不可能再有解
  · target < 3 时自然得到空答案：1 与 2 都凑不出两个正整数之和，
    3 = 1 + 2 是第一个有解的输入
  · 窗口长度天然 >= 2：left 只在 sum >= target 后才前进，right 始终
    不会被推到 left 的位置
  · `(left..=right).collect()` 直接生成升序序列，省掉手写循环

手推一遍：以 target = 15 为例：
  [1,2] 和 3 < 15 → 右移到 [1,5] 和 15，收下 [1,2,3,4,5]
  左移到 [2,5] 和 14 < 15 → 右移到 [2,6] 和 20 → 左移到 [4,6] 和 15，
  收下 [4,5,6]
  左移到 [5,7] 和 18 > 15 → [6,7] 和 13 < 15 → 右移到 [6,8] 和 21 →
  左移到 [7,8] 和 15，收下 [7,8]
  左移到 [8,8]，left > 15 / 2，结束
  答案 [[1,2,3,4,5],[4,5,6],[7,8]] ✔

复杂度分析：
  时间 O(target)：left 最多走到 target/2，right 不超过 target，两者
    都单调右移，加减各摊还 O(target) 次
  空间 O(1)：除输出外只用两个游标和一个累加和
*/
pub struct Solution;

impl Solution {
    pub fn file_combination(target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        // 窗口 [left, right] 的和，初值 1 + 2 是长度 2 的最小和
        let (mut left, mut right) = (1, 2);
        let mut sum = left + right;

        while left <= target / 2 {
            if sum == target {
                ans.push((left..=right).collect());
                // 记录之后必须左移，否则同一个窗口会被反复收下
                sum -= left;
                left += 1;
            } else if sum < target {
                // 不足只能加数：右端点右移，和只会变大，不会漏解
                right += 1;
                sum += right;
            } else {
                // 超出只能减数：右端点已经是最小可行位置，换左端点
                sum -= left;
                left += 1;
            }
        }
        ans
    }
}

fn check(target: i32, want: &[&[i32]]) {
    let got = Solution::file_combination(target);
    assert_eq!(got.len(), want.len(), "target = {}", target);
    for (g, w) in got.iter().zip(want.iter()) {
        assert_eq!(g, w, "target = {}", target);
    }
}

fn main() {
    // 示例 1
    check(12, &[&[3, 4, 5]]);

    // 示例 2：两组解，按首元素升序
    check(18, &[&[3, 4, 5, 6], &[5, 6, 7]]);

    // 长度 3 的窗口与长度 2 的窗口各一组
    check(9, &[&[2, 3, 4], &[4, 5]]);

    // 三组解，覆盖窗口从长到短
    check(15, &[&[1, 2, 3, 4, 5], &[4, 5, 6], &[7, 8]]);

    // 边界：3 是最小的有解输入
    check(3, &[&[1, 2]]);

    // 边界：4 介于 1+2 与 2+3 之间，没有解
    check(4, &[]);

    // 边界：target < 3 无解，不能把 target 本身当成一个数返回
    check(1, &[]);
    check(2, &[]);

    println!("all tests passed");
}
