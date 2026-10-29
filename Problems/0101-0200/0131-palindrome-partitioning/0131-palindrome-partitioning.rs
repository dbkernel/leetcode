/*
============================================================================
131. 分割回文串（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/palindrome-partitioning/
分类：回溯 · 分割问题（回文判定预处理）

给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是 回文串 。
返回 s 所有可能的分割方案。

示例 1：
输入：s = "aab"
输出：[["a","a","b"],["aa","b"]]

示例 2：
输入：s = "a"
输出：[["a"]]

提示：
- 1 <= s.length <= 16
- s 仅由小写英文字母组成
*/
/*
解法一：回溯 + 双指针判回文【非最优】

为什么慢：先把所有切法枚举出来再逐段验证，共 2^(n-1) 种切法，很多切法在
第一段就不合法，后面的段白验证；更糟的是同一条子串 s[l..r] 会在不同切法的
前缀里被反复判定。

换个角度：把「切」和「判」合并成一次 DFS。每一层从 start 出发，枚举下一段
的右端点 end，只对 s[start..end] 判一次回文；合法才把它放进 path 并递归
end + 1，不合法就换下一个 end。非法前缀当场截断，不会进入更深的层。

关键点：
  · 每层枚举的是「下一段的右端点」，段为 s[start..end]，end 从 start 开始，
    因此不会出现空段
  · 只有回文段才能入栈并递归；回溯时 pop，path 复用
  · start == n 时收集，此时 path 恰好是一套完整切法
  · 双指针 l、r 向中间收，一旦 s[l] != s[r] 立即返回 false，不要先反转再比
  · 单字符一定是回文，递归至少推进一位，不会原地打转

手推一遍：
  以示例 1（s = "aab"）为例：
  start = 0，end = 0：段 "a" 回文 → 进入 start = 1
    end = 1：段 "a" 回文 → 进入 start = 2
      end = 2：段 "b" 回文 → start = 3，收集 ["a","a","b"]
    end = 2：段 "ab" 不回文，跳过
  回到 start = 0，end = 1：段 "aa" 回文 → 进入 start = 2
    end = 2：段 "b" 回文 → 收集 ["aa","b"]
  回到 start = 0，end = 2：段 "aab" 不回文
  结果 [["a","a","b"],["aa","b"]] ✔

复杂度分析：
  时间 O(n·2^n)：最坏（如 "aaaa..."）每个节点都要把整段比完，搜索树节点
    数为 O(2^n)
  空间 O(n)：递归深度最多 n 层，path 最多 n 段；返回结果不计入
  说明：若把「复制输出」也算进总成本，两解同为 O(n·2^n)；本解多花的
    是每个节点的 O(段长) 判定
*/

// 单文件 rustc 编译时平台不提供 Solution 类型，这里自行声明
pub struct Solution1;

impl Solution1 {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_pal(bytes: &[u8], mut l: usize, mut r: usize) -> bool {
            while l < r {
                if bytes[l] != bytes[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        fn dfs(
            s: &str,
            bytes: &[u8],
            start: usize,
            path: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if start == bytes.len() {
                ans.push(path.clone());
                return;
            }
            for end in start..bytes.len() {
                if !is_pal(bytes, start, end) {
                    continue;
                }
                path.push(s[start..=end].to_string());
                dfs(s, bytes, end + 1, path, ans);
                path.pop();
            }
        }

        let bytes = s.as_bytes();
        let mut path = Vec::new();
        let mut ans = Vec::new();
        dfs(&s, bytes, 0, &mut path, &mut ans);
        ans
    }
}

/*
解法二：回溯 + 预处理的回文 DP 表【最优解】

为什么慢：解法一的瓶颈在判定。同一条子串会出现在许多不同的切法前缀里，
每次都重新用双指针扫一遍，最坏每个节点 O(段长)，整棵树 O(n·2^n)。

换个角度：回文有最优子结构：s[l..r] 是回文，当且仅当 s[l] == s[r] 且
s[l+1..r-1] 也是回文。于是可以按区间长度从小到大，把所有 (l, r) 填进一张
n × n 的布尔表，之后任何一段都是 O(1) 查表。

关键点：
  · 必须按 len 递增填表：pal[l+1][r-1] 的区间长度是 len - 2，先算过才能用
  · len <= 2 时只看首尾是否相等，不能访问 pal[l+1][r-1]（会越界或读到
    未初始化的格子）
  · 搜索框架与解法一逐行相同，唯一改动是把 is_pal 换成查表，这样两解的
    搜索树与输出完全一致，差别只在判定成本
  · 预处理 O(n²) 时间、O(n²) 空间；搜索仍是 O(2^n) 个节点，但每个节点
    O(1) 判定
  · 取舍：解法一只需 O(n) 空间，n ≤ 16 时通常更快、常数更小；解法二用
    O(n²) 空间换掉每个节点的 O(n) 判定，在判定密集的输入上更稳。若把复制
    输出计入总时间，两者同为 O(n·2^n)，DP 省下的是判定开销

手推一遍：
  以示例 1（s = "aab"）为例，先填表：
  len = 1：pal[0][0]、pal[1][1]、pal[2][2] 全为 true
  len = 2：s[0]==s[1] → pal[0][1] = true；s[1]!=s[2] → pal[1][2] = false
  len = 3：s[0]!=s[2] → pal[0][2] = false
  搜索时：start = 0 的 "a"、"aa" 依次命中；到 start = 2 只剩 "b"，
  得到 ["a","a","b"] 与 ["aa","b"] ✔

复杂度分析：
  时间 O(n² + 2^n)：O(n²) 填表，搜索树 O(2^n) 个节点且每次判定 O(1)；
    把每个解复制进答案另需 O(n·2^n)，两解相同，不计入比较
  空间 O(n²)：回文表；递归栈与 path 合计 O(n)
*/
pub struct Solution2;

impl Solution2 {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        // pal[l][r]：子串 s[l..r] 是否回文
        let mut pal = vec![vec![false; n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                if bytes[l] != bytes[r] {
                    continue;
                }
                // 长度 1、2 时中间为空，直接成立；更长的才回查内层
                let inner = len <= 2 || pal[l + 1][r - 1];
                pal[l][r] = inner;
            }
        }

        fn dfs(
            s: &str,
            start: usize,
            pal: &Vec<Vec<bool>>,
            path: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if start == pal.len() {
                ans.push(path.clone());
                return;
            }
            for end in start..pal.len() {
                // 与解法一唯一的区别：判定从 O(段长) 降为 O(1)
                if !pal[start][end] {
                    continue;
                }
                path.push(s[start..=end].to_string());
                dfs(s, end + 1, pal, path, ans);
                path.pop();
            }
        }

        let mut path = Vec::new();
        let mut ans = Vec::new();
        dfs(&s, 0, &pal, &mut path, &mut ans);
        ans
    }
}

// ===== 测试辅助 =====
// 每个方案：每段都是回文，拼回去必须等于原串
fn verify_pieces(got: &[Vec<String>], s: &str) {
    for plan in got {
        let mut joined = String::new();
        for piece in plan {
            assert!(!piece.is_empty());
            let b = piece.as_bytes();
            let (mut l, mut r) = (0, b.len() - 1);
            while l < r {
                assert_eq!(b[l], b[r]);
                l += 1;
                r -= 1;
            }
            joined.push_str(piece);
        }
        assert_eq!(joined, s);
    }
}

fn check(s: &str, want: Vec<Vec<String>>) {
    // 分割方案是字符串序列，顺序有意义：只对「方案列表」排序
    let mut expected = want;
    expected.sort();

    let mut got = Solution1::partition(s.to_string());
    verify_pieces(&got, s);
    got.sort();
    assert_eq!(got, expected);

    let mut got = Solution2::partition(s.to_string());
    verify_pieces(&got, s);
    got.sort();
    assert_eq!(got, expected);
}

// 方案数很多时只校验数量与每段合法性
fn check_count(s: &str, want_size: usize) {
    let got = Solution1::partition(s.to_string());
    assert_eq!(got.len(), want_size);
    verify_pieces(&got, s);

    let got = Solution2::partition(s.to_string());
    assert_eq!(got.len(), want_size);
    verify_pieces(&got, s);
}

fn row(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|p| p.to_string()).collect()
}

fn main() {
    // 示例 1：单字符切法与 "aa" 切法都要在
    check("aab", vec![row(&["a", "a", "b"]), row(&["aa", "b"])]);

    // 示例 2：单字符
    check("a", vec![row(&["a"])]);

    // 边界：两个相同字符，可以是两段也可以是一段
    check("aa", vec![row(&["a", "a"]), row(&["aa"])]);

    // 边界：首尾相同但不是全同，"aba" 整段回文
    check("aba", vec![row(&["a", "b", "a"]), row(&["aba"])]);

    // 边界：三个相同字符，方案数 2^(3-1) = 4
    check(
        "aaa",
        vec![
            row(&["a", "a", "a"]),
            row(&["a", "aa"]),
            row(&["aa", "a"]),
            row(&["aaa"]),
        ],
    );

    // 边界：没有长度 > 1 的回文段，只有逐字符一种切法
    check("abcd", vec![row(&["a", "b", "c", "d"])]);

    // 边界：题目上限 n = 16，全相同，方案数 2^15 = 32768
    check_count(&"a".repeat(16), 32768);

    println!("all tests passed");
}
