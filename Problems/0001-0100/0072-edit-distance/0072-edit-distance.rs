/*
============================================================================
72. 编辑距离（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/edit-distance/
分类：动态规划 · 二维字符串 DP（插入 / 删除 / 替换）

给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少
操作数  。

你可以对一个单词进行如下三种操作：

- 插入一个字符
- 删除一个字符
- 替换一个字符

示例 1：
输入：word1 = "horse", word2 = "ros"
输出：3
解释：
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')

示例 2：
输入：word1 = "intention", word2 = "execution"
输出：5
解释：
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')

提示：
0 <= word1.length, word2.length <= 500
word1 和 word2 由小写英文字母组成
*/

// 平台上由判题系统预置 Solution 类型，本地用 rustc 单文件编译时需要显式声明
pub struct Solution1;
pub struct Solution2;
pub struct Solution3;

/*
解法一：记忆化搜索【非最优】

为什么慢：把「改哪些字符、按什么顺序改」直接枚举，每一步都有 3 种操作可
选，路径数指数级增长。真正的瓶颈是重复子问题：从 "hor" 到 "ro" 的最少次
数一旦确定，无论前面怎么走过来的都一样，但暴力枚举会把同一个前缀对反复
求很多遍。

换个角度：唯一决定后续代价的是「word1 已经处理到第 i 个字符、word2 已经
处理到第 j 个字符」。把 (i, j) 当作状态，答案只由它和三个更小的状态决定。

关键点：
  · 状态 (i, j) 的含义：把 word1 前 i 个字符变成 word2 前 j 个字符
    所需的最少操作数
  · 三个转移各自对应一种操作，方向不能写反：
    (i-1, j) + 1 —— 删掉 word1[i-1]，word1 少一个字符
    (i, j-1) + 1 —— 在 word1 里插入 word2[j-1]，word2 这一位被补齐
    (i-1, j-1) + (word1[i-1] != word2[j-1]) —— 替换；字符相同时不花代价，
    直接继承对角
  · 边界只有两行：i = 0 时 word1 空了，只能插入，答案是 j；j = 0 时
    word2 空了，只能删除，答案是 i
  · 边界必须先于记忆化查表返回：memo[0][j]、memo[i][0] 从未被填过，
    不先返回会拿到「未知」标记
  · 用 -1 表示未知即可：真实的距离都 >= 0

以示例 1（word1 = "horse"，word2 = "ros"）为例：
  dfs(5, 3) 先试删末尾 'e'：dfs(4, 3) + 1
  dfs(4, 3) 发现 word1[3] 与 word2[2] 都是 's'，走对角 dfs(3, 2)
  dfs(3, 2) 一路收敛到 dfs(0, 1) = 1，回溯得到 2，最后答案 3 ✔

复杂度分析：
  时间 O(mn)：状态总数 m·n，每个状态只算一次
  空间 O(mn)：备忘录存满整张表，另有 O(m+n) 的递归栈
*/

impl Solution1 {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let a = word1.as_bytes();
        let b = word2.as_bytes();
        let mut memo = vec![vec![-1i32; b.len() + 1]; a.len() + 1];
        Self::dfs(a, b, a.len(), b.len(), &mut memo)
    }

    fn dfs(
        a: &[u8],
        b: &[u8],
        i: usize,
        j: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        // 空串边界：一方为空时只剩纯插入或纯删除
        if i == 0 {
            return j as i32;
        }
        if j == 0 {
            return i as i32;
        }
        if memo[i][j] != -1 {
            return memo[i][j];
        }

        let del = Self::dfs(a, b, i - 1, j, memo);
        let ins = Self::dfs(a, b, i, j - 1, memo);
        let mut best = del.min(ins) + 1;

        let differ = i32::from(a[i - 1] != b[j - 1]);
        let sub = Self::dfs(a, b, i - 1, j - 1, memo) + differ;
        best = best.min(sub);

        memo[i][j] = best;
        best
    }
}

/*
解法二：二维动态规划【非最优】

把解法一的递归改成自底向上填表：dp[i][j] 就是「word1 前 i 个字符变成
word2 前 j 个字符的最少操作数」，转移与解法一逐字对应。
  · 第 0 行、第 0 列先填好，相当于递归里的空串边界
  · 每个格子只看左边、上边、左上三个已算好的格子，按行推进即可
  · 与解法一时间同为 O(mn)，但要多开一整张 m·n 的表

复杂度分析：
  时间 O(mn)：双层循环各遍历一次
  空间 O(mn)：整张表都要留着，滚动到哪一行都不需要旧行以外的数据
*/

impl Solution2 {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let a = word1.as_bytes();
        let b = word2.as_bytes();
        let (m, n) = (a.len(), b.len());

        let mut dp = vec![vec![0i32; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if a[i - 1] == b[j - 1] {
                    // 末尾相同，白拿一次对齐，不必付操作代价
                    dp[i - 1][j - 1]
                } else {
                    1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
                };
            }
        }
        dp[m][n]
    }
}

/*
解法三：一维滚动数组（短串放内层）【最优解】

为什么慢：解法二每个格子只用到「上一行」和「本行左边一格」，整张表里
除了这两样，其余都是历史包袱，却一直占着 m·n 的空间。

换个角度：把 dp 压成一行，第 i 行覆盖第 i-1 行。覆盖时唯一会丢失的是
左上角 dp[i-1][j-1]，用一个变量 diag 提前存下来即可。

关键点：
  · 每轮进 j 循环前 diag 装的是 dp[i-1][0]，循环里每算完一格就把
    「本格被覆盖前的旧值」（即 dp[i-1][j]）交给 diag，供下一格使用
  · 删、插、替三条转移分别取 上（dp[j]，尚未覆盖）、左（dp[j-1]）、
    左上（diag），顺序写错就会把本行刚算出的值当成上一行
  · dp[0] 每行都要重置成 i：它是「word1 前 i 个字符清空」的代价
  · 编辑距离对两个参数对称，可以先把长串放外层、短串放内层，滚动数组的
    长度取 min(m, n)；不先交换，空间上界仍是较长那一侧的 O(m)
  · 边界仍然是 dp[0] = 0、dp[j] = j（第 0 行的初值）

以示例 1（word1 = "horse"，word2 = "ros"）为例：
  初值 dp = [0, 1, 2, 3]
  i=1 后 dp = [1, 1, 2, 3]（"h" 对齐 "r" 要替换一次）
  i=3 后 dp = [3, 2, 2, 2]（'r' 对上 'r'，第一格变成继承对角）
  i=5 后 dp[3] = 1 + min(2, 4, 3) = 3 ✔

复杂度分析：
  时间 O(mn)：两重循环走满短串长度
  空间 O(min(m, n))：只保留一行，长度是较短串加一
*/

impl Solution3 {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // 对称：让短的做内层，滚动数组才不会按长串开
        let (long, short) = if word1.len() >= word2.len() {
            (word1, word2)
        } else {
            (word2, word1)
        };
        let a = long.as_bytes();
        let b = short.as_bytes();
        let (m, n) = (a.len(), b.len());

        let mut dp: Vec<i32> = (0..=n as i32).collect();

        for i in 1..=m {
            let mut diag = dp[0]; // dp[i-1][0]，覆盖前先取走
            dp[0] = i as i32;     // dp[i][0]：删掉 word1 的前 i 个字符
            for j in 1..=n {
                let up = dp[j]; // 覆盖前仍是 dp[i-1][j]
                dp[j] = if a[i - 1] == b[j - 1] {
                    diag
                } else {
                    1 + diag.min(up).min(dp[j - 1])
                };
                diag = up;
            }
        }
        dp[n]
    }
}

// 三个解法的期望值一致，统一在这里断言
fn check(word1: &str, word2: &str, want: i32) {
    assert_eq!(
        Solution1::min_distance(word1.to_string(), word2.to_string()),
        want,
        "memo({word1},{word2})"
    );
    assert_eq!(
        Solution2::min_distance(word1.to_string(), word2.to_string()),
        want,
        "2d({word1},{word2})"
    );
    assert_eq!(
        Solution3::min_distance(word1.to_string(), word2.to_string()),
        want,
        "rolling({word1},{word2})"
    );
}

fn main() {
    // 示例 1
    check("horse", "ros", 3);

    // 示例 2
    check("intention", "execution", 5);

    // 边界：word1 为空，只能一路插入
    check("", "abc", 3);

    // 边界：word2 为空，只能一路删除
    check("abc", "", 3);

    // 边界：两边都为空，距离为 0
    check("", "", 0);

    // 边界：完全相同，一次操作都不用
    check("abc", "abc", 0);

    // 反直觉：一个字符全换成另一个，距离是 1 而不是长度
    check("a", "b", 1);

    // 反直觉：短串是长串的子序列，只靠删除 / 插入就能对齐
    check("abc", "abcde", 2);

    // 关键点用例：参数交换后结果必须一样（滚动数组的对称性依赖它）
    check("ros", "horse", 3);
    check("execution", "intention", 5);

    // 规模上界：两条 500 字符的串毫无公共字符，答案是 500
    check(&"a".repeat(500), &"b".repeat(500), 500);

    println!("all tests passed");
}
