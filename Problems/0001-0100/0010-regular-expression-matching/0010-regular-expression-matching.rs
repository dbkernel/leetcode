/*
============================================================================
10. 正则表达式匹配（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/regular-expression-matching/
分类：动态规划 · 字符串匹配（状态转移 / 记忆化）

给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则
表达式匹配。

- '.' 匹配任意单个字符
- '*' 匹配零个或多个前面的那一个元素

返回一个布尔值，表示匹配是否覆盖整个输入字符串（而非部分）。

示例 1：
输入：s = "aa", p = "a"
输出：false
解释："a" 无法匹配 "aa" 整个字符串。

示例 2:
输入：s = "aa", p = "a*"
输出：true
解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素
就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。

示例 3：
输入：s = "ab", p = ".*"
输出：true
解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。

提示：
1 <= s.length <= 20
1 <= p.length <= 20
s 只包含从 a-z 的小写字母。
p 只包含从 a-z 的小写字母，以及字符 . 和 *。
保证每次出现字符 * 时，前面都匹配到有效的字符
*/

// 平台上由判题系统预置 Solution 类型，本地用 rustc 单文件编译时需要显式声明
pub struct Solution1;
pub struct Solution2;

/*
解法一：记忆化递归【最优解】

为什么慢：最直白的写法是「遇到 '*' 就枚举它吃掉几个字符」，每个 '*' 都
开出一串分支。不同分支会走到同一个 (s 的下标, p 的下标) 组合，后面的匹配
却完全一样，于是同一对下标被反复求解，分支数随 '*' 的个数指数增长。

换个角度：匹配就是「左边已经对齐了，接下来只看 s 剩下多少、p 剩下多少」。
前面怎么对齐的不影响后面，所以 (i, j) 两个下标就是全部状态。

关键点：
  · 判断 '*' 要看 p[j+1]，也就是「下一个字符」；递归的 j 停在当前待匹配
    的字符上，与 DP 里看 p[j-1] 的写法差一位，这里最容易写混
  · '*' 有两种转移，缺一不可：
    匹配 0 次 —— 跳过「字符 + *」这一对，转 (i, j+2)
    再匹配 1 次 —— 当前字符对得上时，s 前进一格、p 原地不动，转 (i+1, j)；
    「原地不动」是能吃下多个相同字符的唯一写法
  · 两条分支用 || 连接：任意一条成立整体就成立，先短路的那条不浪费
  · 边界：j == m 时模式用完了，只有 i == n 才算匹配成功；反过来 p 还剩
    字符而 s 用完，靠 0 次分支继续消掉 a*b* 这类尾巴
  · a* 必须能吃掉空串：i == n 时也要先试 (i, j+2)，否则 "ab" 对 "a*b"
    会误判
  · 备忘录用 i8 存 -1 / 0 / 1，-1 表示未知：false 本身也是合法结果

以示例 2（s = "aa"，p = "a*"）为例：
  (0,0)：p[1] 是 '*' → 先试 0 次 (0,2)：模式用完了但 s 没走完，假
         再试 1 次：'a' 对上 s[0]，转 (1,0)
  (1,0)：同样先试 0 次 (1,2)：假；再试 1 次，转 (2,0)
  (2,0)：0 次分支到 (2,2)：j == m 且 i == n → 真 ✔

复杂度分析：
  时间 O(mn)：状态 (i,j) 共 (n+1)(m+1) 个，每个只求一次
  空间 O(mn)：备忘录整张表，另加 O(n+m) 的递归栈
*/

impl Solution1 {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut memo = vec![vec![-1i8; p.len() + 1]; s.len() + 1];
        Self::dfs(s, p, 0, 0, &mut memo)
    }

    fn dfs(
        s: &[u8],
        p: &[u8],
        i: usize,
        j: usize,
        memo: &mut Vec<Vec<i8>>,
    ) -> bool {
        if memo[i][j] != -1 {
            return memo[i][j] == 1;
        }

        let ok = if j == p.len() {
            // 模式耗尽：s 也必须耗尽
            i == s.len()
        } else {
            let first = i < s.len() && (p[j] == b'.' || p[j] == s[i]);
            if j + 1 < p.len() && p[j + 1] == b'*' {
                // 0 次跳过这一对；或吃掉一个字符后仍停在这里继续吃
                let zero = Self::dfs(s, p, i, j + 2, memo);
                zero || (first && Self::dfs(s, p, i + 1, j, memo))
            } else {
                first && Self::dfs(s, p, i + 1, j + 1, memo)
            }
        };

        memo[i][j] = if ok { 1 } else { 0 };
        ok
    }
}

/*
解法二：二维动态规划【最优解，与解法一并列】

把解法一的递归改成自底向上：dp[i][j] 表示 s 的前 i 个字符与 p 的前 j 个
字符是否匹配。转移就是解法一那两条分支的镜像。
  · 第 0 行单独处理：a*b*c* 这类模式能让空串与它匹配，dp[0][j] 取
    dp[0][j-2]
  · p[j-1] 是 '*' 时看 dp[i][j-2]（匹配 0 次）与 dp[i-1][j]（再匹配 1 次）
  · p[j-1] 不是 '*' 时看 dp[i-1][j-1] 且当前字符对得上

关键点：
  · 「再匹配 1 次」落在 dp[i-1][j] 而不是 dp[i-1][j-2]：p 的指针不后退，
    这样同一个 a* 才能连续吃掉 aaa
  · 「匹配 0 次」落在 dp[i][j-2]：连同星号与被它修饰的字符一起跳过
  · j == 1 时 dp[i][j-2] 会越界，但题目保证 '*' 前面必有有效字符，
    所以 p[0] 不可能是 '*'，这条分支只在 j >= 2 时才可能被走到
  · 第 0 行必须从 j = 2 开始按 dp[0][j-2] 递推，不能只看 p[j-1]
  · . 与字符的等价判断抽成一个函数，两处转移都调它，避免一处漏掉 '.'

以示例 3（s = "ab"，p = ".*"）为例：
  dp[0][2]：p[1] 是 '*'，取 dp[0][0] = true —— ".*" 能吃空串
  dp[1][1]：'.' 能匹配 'a'，取 dp[0][0] = true
  dp[1][2]：'*' 的 0 次分支 dp[1][0] = false；1 次分支 '.' 对 'a'
            成立，取 dp[0][2] = true → 真
  dp[2][2]：1 次分支取 dp[1][2] = true → 真 ✔

复杂度分析：
  时间 O(mn)：两层循环填满整张表
  空间 O(mn)：布尔表 (n+1)×(m+1)

两条解法时间同为 O(mn)、空间同为 O(mn)，并列最优。取舍：记忆化只展开
真正被访问到的状态，p 里 '*' 多时实际计算量明显更小，但递归深度是 O(n+m)；
DP 迭代不占调用栈，状态数固定，代价是即使无关状态也要填。题目规模只有
20，两者实测量级一致，按渐近复杂度判为并列。
*/

impl Solution2 {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let (n, m) = (s.len(), p.len());

        // dp[i][j]：s 的前 i 个字符与 p 的前 j 个字符是否匹配
        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;

        // 空串对 a*b*c* 这类「匹配 0 次」的模式仍可能成立
        for j in 2..=m {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=n {
            for j in 1..=m {
                if p[j - 1] == b'*' {
                    // 0 次：连同被修饰的字符一起跳过
                    let zero = dp[i][j - 2];
                    // 再匹配 1 次：p 停在这一对字符上，s 前进一格
                    let more = single_match(p[j - 2], s[i - 1]) && dp[i - 1][j];
                    dp[i][j] = zero || more;
                } else if single_match(p[j - 1], s[i - 1]) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
        dp[n][m]
    }
}

// '.' 是通配，其余必须逐字相等
fn single_match(pc: u8, sc: u8) -> bool {
    pc == b'.' || pc == sc
}

// 两个解法的期望值一致，统一在这里断言
fn check(s: &str, p: &str, want: bool) {
    assert_eq!(
        Solution1::is_match(s.to_string(), p.to_string()),
        want,
        "memo({s},{p})"
    );
    assert_eq!(
        Solution2::is_match(s.to_string(), p.to_string()),
        want,
        "dp({s},{p})"
    );
}

fn main() {
    // 示例 1
    check("aa", "a", false);

    // 示例 2：a* 吃掉两个 a
    check("aa", "a*", true);

    // 示例 3：.* 吃掉任意个字符
    check("ab", ".*", true);

    // 关键点用例：模式里夹着不能匹配的字符
    check("mississippi", "mis*is*p*.", false);

    // 关键点用例：a* 必须能匹配 0 次（b* 空吃）
    check("a", "ab*", true);

    // 关键点用例：连续两个 a* 前缀，s 用完但模式还剩
    check("aab", "c*a*b", true);

    // 边界：空串对能空吃的模式
    check("", ".*", true);
    check("", "a*", true);
    check("", "a", false);

    // 反直觉：模式比串短，但多出来的 a* 归零后恰好对上
    check("aaa", "a*a", true);

    // 反直觉：.* 贪婪会吃掉 ab，剩下的 c 对不上
    check("ab", ".*c", false);

    // 规模上界：20 个 a 对 10 段 a*
    check("aaaaaaaaaaaaaaaaaaaa", "a*a*a*a*a*a*a*a*a*a*", true);

    println!("all tests passed");
}
