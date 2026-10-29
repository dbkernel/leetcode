/*
============================================================================
744. 寻找比目标字母大的最小字母（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-smallest-letter-greater-than-target/
分类：二分查找 · 上界 / 循环回绕

给你一个字符数组 letters，该数组按非递减顺序排序，以及一个字符 target。
letters 里至少存在两个不同的字符。

返回 letters 中大于 target 的最小的字符。如果不存在这样的字符，
则返回 letters 的第一个字符。

示例 1：
输入: letters = ["c", "f", "j"]，target = "a"
输出: "c"
解释：letters 中字典上比 'a' 大的最小字符是 'c'。

示例 2:
输入: letters = ["c","f","j"], target = "c"
输出: "f"
解释：letters 中字典顺序上大于 'c' 的最小字符是 'f'。

示例 3:
输入: letters = ["x","x","y","y"], target = "z"
输出: "x"
解释：letters 中没有一个字符在字典上大于 'z'，所以我们返回 letters
的第一个字符。

提示：
- 2 <= letters.length <= 10^4
- letters[i] 是一个小写字母
- letters 按非递减顺序排序
- letters 最少包含两个不同的字母
- target 是一个小写字母
*/

/*
解法：二分找第一个大于 target 的位置，取模回绕【最优解】

为什么慢：从左往右扫描，遇到第一个大于 target 的字符就返回，最多要走
完整个数组，O(n)。可 letters 已经按非递减顺序排好，这份顺序信息完全
没有被利用。

换个角度：要找的是「第一个严格大于 target 的位置」，这就是上界
（upper_bound）。二分的判断条件是 `letters[mid] <= target`：成立说明
mid 以及它左边都不可能是答案，不成立说明 mid 本身可能就是答案。

关键点：
  · 维护左闭右开区间 [left, right)，right 初值取 n
  · 判断写 `letters[mid] <= target` 才把 left 推到 mid + 1，等号不能丢，
    题目要的是严格大于，等于 target 的字符不算
  · 循环结束时 left 就是第一个大于 target 的下标；left == n 表示全部
    字符都 <= target，按题意回绕到首字母
  · 回绕只写一次 `left % n`，不要用 if/else 特判，n >= 2 时取模即可
  · char 在 Rust 里按 Unicode 标量比较，小写字母的顺序与字典序一致

以示例 2（letters = ["c","f","j"]，target = "c"）为例：
  left=0, right=3 → mid=1，'f' > 'c' → right=1
  left=0, right=1 → mid=0，'c' <= 'c' → left=1
  left == right == 1 → 返回 letters[1] = 'f' ✔

以示例 3（letters = ["x","x","y","y"]，target = "z"）为例：
  所有字符都 <= 'z'，二分结束时 left = 4 = n，4 % 4 = 0 → 'x' ✔

复杂度分析：
  时间 O(log n)：每轮区间减半
  空间 O(1)：只用了两个下标
*/

// rustc 单文件编译时没有平台预置的 Solution，这里显式定义
pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let n = letters.len();
        let (mut left, mut right) = (0usize, n);

        while left < right {
            let mid = left + (right - left) / 2;
            // 小于等于 target 的字符不可能是答案，连同 mid 一起排除
            if letters[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // left == n 说明没有更大的字符，取模回绕到首字符
        letters[left % n]
    }
}

fn check(letters: &[char], target: char, want: char) {
    let got = Solution::next_greatest_letter(letters.to_vec(), target);
    assert_eq!(got, want, "letters = {:?}, target = {:?}", letters, target);
}

fn main() {
    // 示例 1
    check(&['c', 'f', 'j'], 'a', 'c');

    // 示例 2
    check(&['c', 'f', 'j'], 'c', 'f');

    // 示例 3：没有更大字符时回绕
    check(&['x', 'x', 'y', 'y'], 'z', 'x');

    // 边界：target 等于末字符，必须回绕而不是返回末字符
    check(&['c', 'f', 'j'], 'j', 'c');

    // 边界：target 比首字符还小
    check(&['c', 'f', 'j'], 'a', 'c');

    // 边界：只有两个字符
    check(&['a', 'b'], 'a', 'b');
    check(&['a', 'b'], 'z', 'a');

    // 边界：target 落在重复字符中间，跳过后返回更大的那个
    check(&['x', 'x', 'y', 'y'], 'x', 'y');

    // 边界：target 略小于首字符且只有两个候选
    check(&['a', 'z'], 'm', 'z');

    // 规模：n = 10^4，前 n-1 个都是 'a'，最后一个是 'z'
    let mut letters = vec!['a'; 9999];
    letters.push('z');
    check(&letters, 'a', 'z');
    check(&letters, 'z', 'a');

    println!("all tests passed");
}
