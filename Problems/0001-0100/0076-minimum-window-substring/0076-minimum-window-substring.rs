/*
============================================================================
76. 最小覆盖子串（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/minimum-window-substring/
分类：滑动窗口 · 欠账计数

给定两个字符串 s 和 t，长度分别是 m 和 n，返回 s 中的 最短窗口 子串，
使得该子串包含 t 中的每一个字符（包括重复字符）。如果没有这样的子串，
返回空字符串 ""。

测试用例保证答案唯一。

示例 1：
输入：s = "ADOBECODEBANC", t = "ABC"
输出："BANC"
解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。

示例 2：
输入：s = "a", t = "a"
输出："a"
解释：整个字符串 s 是最小覆盖子串。

示例 3:
输入: s = "a", t = "aa"
输出: ""
解释: t 中两个字符 'a' 均应包含在 s 的子串中，
因此没有符合条件的子字符串，返回空字符串。

提示：
- m == s.length
- n == t.length
- 1 <= m, n <= 10^5
- s 和 t 由英文字母组成

进阶：你能设计一个在 O(m + n) 时间内解决此问题的算法吗？
*/

/*
Rust 没有平台预置的 Solution 类型，单文件 rustc 编译时必须自己声明。
解法只有一条，所以直接用 Solution，与平台签名保持一致。
*/

/*
解法：滑动窗口 + 欠账计数（need / missing）【最优解】

为什么慢：枚举所有子串再逐个检查是否覆盖 t，窗口有 O(m^2) 个，每次检查
最坏又要 O(n)，合起来是 O(m^2·n)。把检查降到 O(1)（靠一张频次表）也只
省掉最后那个因子，O(m^2) 个窗口本身已经超出 10^5 的量级。逐字符为 t 的
每个位置去 s 里找最近匹配，同样会退化成平方级。

换个角度：右端右移时，"覆盖 t"这个性质只会从不满足变成满足，一旦满足，
再往右扩只会更长、绝不可能更优。所以每个右端点只值得做一件事：把左端
一路右移，直到窗口刚好不再覆盖 —— 这一路的每个位置都是候选答案。左端
与右端都单调右移，每个字符最多入窗一次、出窗一次。

关键点：
  · need[c] 记"还欠几个 c"：入窗时先看 need[c] > 0 再 missing--，
    只有真欠的字符才抵消欠账，多余字符只把 need 记成负数
  · 收缩时先 need[out] += 1，再看它是否 > 0：只有加回去之后重新变欠的
    字符才能使 missing += 1，窗口中多余的字符加回去仍 <= 0，不动 missing
  · 记录答案要写在 missing == 0 的循环体开头、左移之前，否则当前这个
    可行窗口就漏了
  · 收缩用一个 while 一路缩到 missing 被破坏为止，不能只缩一格
  · 用长度 128 的数组代替哈希表：字母表固定，省掉哈希开销与扩容
  · t 里重复字符按多重集计：need 的初值是每个字符的出现次数，missing
    是 t 的长度，所以 s = "a"、t = "aa" 时 missing 永远减不到 0
  · 用 as_bytes 按下标读写，避免在循环里反复取 str 切片

手推一遍：以示例 1（s = "ADOBECODEBANC"，t = "ABC"）为例：
  右端走到下标 5（'C'）时首次覆盖，窗口 "ADOBEC"，收缩到 "DOBEC" 之
  前记下长度 6
  右端继续走到 10、12，每次 missing 归零就收缩；右端到 12（'C'）时
  窗口是 "ODEBANC"，一路左移挤出 O、D、E，得到 "BANC"（长 4）
  再左移一格就会把 B 挤掉，missing 重新变成 1，收缩结束
  答案 "BANC" ✔

复杂度分析：
  时间 O(m + n)：建表 O(n)，right 走 m 步，left 累计也只走 m 步
  空间 O(Σ)：一张 128 长的数组，与 m、n 都无关
*/
pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let sb = s.as_bytes();
        // need[c] > 0 表示还欠几个 c；<= 0 表示窗口里已经多余
        let mut need = [0i32; 128];
        let mut missing = 0i32;
        for &c in t.as_bytes() {
            need[c as usize] += 1;
            missing += 1;
        }

        let mut best_left = 0usize;
        let mut best_len = usize::MAX;
        let mut left = 0usize;

        for right in 0..sb.len() {
            let c = sb[right] as usize;
            // 只有真欠的字符才抵消欠账，多余的字符只是记成负数
            if need[c] > 0 {
                missing -= 1;
            }
            need[c] -= 1;

            // 已经覆盖：继续扩右端只会更长，此刻只值得收缩左端
            while missing == 0 {
                if right - left + 1 < best_len {
                    best_len = right - left + 1;
                    best_left = left;
                }
                let out = sb[left] as usize;
                need[out] += 1;
                // 加回去之后重新变欠，说明这一步不能再缩
                if need[out] > 0 {
                    missing += 1;
                }
                left += 1;
            }
        }

        // best_len 没被更新过，说明从未覆盖过 t
        if best_len == usize::MAX {
            return String::new();
        }
        s[best_left..best_left + best_len].to_string()
    }
}

fn check(s: &str, t: &str, want: &str) {
    assert_eq!(
        Solution::min_window(s.to_string(), t.to_string()),
        want,
        "minWindow({:?}, {:?})",
        s,
        t
    );
}

fn main() {
    // 示例 1：答案不在最左边，也不在第一个可行窗口里
    check("ADOBECODEBANC", "ABC", "BANC");

    // 示例 2：整个串就是答案
    check("a", "a", "a");

    // 示例 3：t 里的字符有重复，s 里不够
    check("a", "aa", "");

    // 边界：t 里有 s 完全没有的字符
    check("abc", "d", "");

    // 边界：单字符重复，答案要覆盖全部重复次数
    check("aa", "aa", "aa");

    // 反直觉：最短窗口以 s 的最后一个字符结尾
    check("bba", "ab", "ba");

    // 边界：滑到末尾才凑齐，答案就是尾部那一段
    check("xyzabc", "abc", "abc");

    // 压力：10^5 个 a 对 "a"，每次覆盖后立刻收缩成单字符
    check(&"a".repeat(100000), "a", "a");

    println!("all tests passed");
}
