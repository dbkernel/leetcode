/*
============================================================================
279. 完全平方数（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/perfect-squares/
分类：动态规划 · 完全背包 / 四平方和定理
============================================================================

给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。

完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个
整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。

示例 1：
输入：n = 12
输出：3
解释：12 = 4 + 4 + 4

示例 2：
输入：n = 13
输出：2
解释：13 = 4 + 9

提示：
- 1 <= n <= 10^4
*/

/*
解法一：动态规划（完全背包）【非最优】

为什么慢：这是把 n 当成背包容量、把 1、4、9、16… 当成物品的完全背包。
为了求 n 一个值，必须先把 1 到 n 每个容量的最优答案都算出来，状态数是
n 个；每个状态还要枚举不超过它的平方数，共 √n 个，于是 O(n·√n)。n 最大
10^4 时够用，但答案本身只可能是 1 到 4，把这些状态全建出来是浪费。

换个角度：把「和」当成容量递增地推，dp[i] 只依赖比 i 小的 dp 值；因为
平方数可以反复取，内层正序即可，不必考虑「物品只能用一次」。

关键点：
  · dp[0] = 0 是唯一初值，含义是「和为 0 不需要任何平方数」；它是所有
    转移的落点，缺了它连 dp[1] 都推不出来
  · 初值取 n + 1 而不是 i32::MAX：dp[i - j²] + 1 会做加法，MAX + 1 在
    debug 下直接 panic，release 下静默回绕；n + 1 一定比任何合法答案大
  · 内层只枚举 j² ≤ i，保证 i - j² 不越界
  · 外层必须按 i 递增，内层平方数怎么枚举都行，这是「容量在外」的完全
    背包写法

手推一遍（n = 12）：
  dp[1] = 1，dp[4] = 1，dp[9] = 1（本身是平方数）
  dp[8] = dp[4] + 1 = 2
  dp[11] = dp[9] + 1 + 1 = 3
  dp[12] = min(dp[11], dp[8], dp[3]) + 1 = min(3, 2, 3) + 1 = 3 ✔
  对应 12 = 4 + 4 + 4

复杂度分析：
  时间 O(n·√n)：n 个容量，每个枚举到 √n 个平方数
  空间 O(n)：一维 dp 表
*/

// rustc 单文件编译时没有平台的 Solution，这里显式定义（结构体方案）
pub struct Solution1;

impl Solution1 {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        // dp[i]：凑出 i 的最少个数；最多用 i 个 1，故 n + 1 是不可能值
        let mut dp = vec![n + 1; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            // 枚举最后一个平方数 j²，剩下的 i - j² 已经算好
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
                j += 1;
            }
        }
        dp[n] as i32
    }
}

/*
解法二：BFS 按层扩展（把和看成图上的点）【非最优】

为什么慢：把 0 到 n 每个整数看成图上的点，两个点差一个平方数就连一条边，
题目问的就是 0 到 n 的最短路。BFS 按步数一层层扩散，第一次碰到 n 时的
层数就是答案。

但 BFS 的规模并没有比解法一小：最坏要把 n 个点全访问一遍，每个点展开
√n 条边，仍是 O(n·√n)，而且多了一个队列和一份访问标记，常数更大。
真正能把复杂度压下来的只有解法三的数论结论。

换个角度：与其问「凑出 i 最少要几个」，不如问「用 k 个平方数能凑出哪些
和」，k 从 1 往上加，加到 n 出现为止。

关键点：
  · 每层开始前先快照队列长度：循环里新入队的属于下一层，混在本层里数
    层数会数错
  · visited 必须在入队时标记，不能出队时标记，否则同一个和会被重复入队
  · 从 0 出发、每次加平方数，或从 n 出发、每次减平方数都可以；前者更好
    写，因为答案就是层数
  · 层数从 1 开始计：第一层放进去的点都只用了一个平方数
  · 因为一定有 1 这个平方数，n 必然可达，循环不会空转结束

手推一遍（n = 12）：
  第 1 层：1、4、9（不含 12）
  第 2 层：由 1 得 2、5、10；由 4 得 8、13…；由 9 得 10、18…
  第 3 层：由 8 加 4 得 12 → 返回 3 ✔（12 = 4 + 4 + 4）

复杂度分析：
  时间 O(n·√n)：最坏访问 n 个和，每个和展开 √n 条边
  空间 O(n)：队列与访问标记各占一份
*/

pub struct Solution2;

impl Solution2 {
    pub fn num_squares(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let n = n as usize;

        let mut visited = vec![false; n + 1];
        let mut queue = vec![0usize];
        visited[0] = true;

        let mut level = 0;
        let mut head = 0;
        while head < queue.len() {
            // 本层结点数此刻固定，后面 push 的都是下一层
            let tail = queue.len();
            level += 1;

            while head < tail {
                let cur = queue[head];
                head += 1;
                let mut j = 1;
                while cur + j * j <= n {
                    let next = cur + j * j;
                    if next == n {
                        return level;
                    }
                    if !visited[next] {
                        visited[next] = true;
                        queue.push(next);
                    }
                    j += 1;
                }
            }
        }
        // 1 是平方数，n 一定能被凑出，走不到这里
        level
    }
}

/*
解法三：四平方和定理（按 n 的形态直接分类）【最优解】

为什么慢：解法一、二都在算「每个和的最少个数」，而题目只要 n 这一个值。
拉格朗日四平方和定理给出：每个正整数都能写成不超过 4 个平方数之和，
所以答案只可能是 1、2、3、4。再配上「n = 4^a(8b+7) 时恰好需要 4 个」
这条判定，就能 O(√n) 直接说出答案。

换个角度：别做加法式搜索，改成对 n 的分类判断，每一类一次判定就够。

关键点：
  · 判 1：n 本身是完全平方数
  · 判 2：枚举 a ≤ √n，只要 n - a² 也是完全平方数，答案就是 2
    （把 n 拆成 a² + b²，两个平方数）
  · 判 4 前必须先把因子 4 除干净，再看剩余部分是否 ≡ 7 (mod 8)；
    直接对原数取模会漏掉 28 这类（28 = 4 × 7，需要 4 个）
  · 以上都不成立就是 3，这是定理保证的，不需要再搜
  · 判平方时浮点 sqrt 可能有误差，向上校正一格再验证

手推一遍：
  n = 12：不是平方；12-1=11、12-4=8、12-9=3 都不是平方；除 4 得 3，
  3 % 8 = 3 ≠ 7 → 3 ✔
  n = 28：不是平方；28-1=27、28-4=24、28-9=19、28-16=12、28-25=3
  都不是平方；除 4 得 7，7 % 8 = 7 → 4 ✔（28 = 25 + 1 + 1 + 1）

复杂度分析：
  时间 O(√n)：判 2 要枚举到 √n，其余都是 O(1)
  空间 O(1)：只用几个整数变量
*/

pub struct Solution3;

impl Solution3 {
    pub fn num_squares(n: i32) -> i32 {
        if is_square(n) {
            return 1;
        }

        // 两个平方数：枚举较小的一项，另一项必须是平方数
        let mut a = 1;
        while a * a <= n {
            if is_square(n - a * a) {
                return 2;
            }
            a += 1;
        }

        // 定理：n = 4^a(8b+7) ⇔ 恰好要 4 个；先除净因子 4 再判模 8
        let mut m = n;
        while m % 4 == 0 {
            m /= 4;
        }
        if m % 8 == 7 {
            return 4;
        }

        3
    }
}

// 浮点开方可能差一格，向上校正后再验证
fn is_square(x: i32) -> bool {
    let mut r = (x as f64).sqrt() as i32;
    while (r + 1) * (r + 1) <= x {
        r += 1;
    }
    while r * r > x {
        r -= 1;
    }
    r * r == x
}

// ===== 测试辅助：小规模穷举，作为对拍的独立真值来源 =====
// 逐个试平方数往下减，used 是已用个数；best 作为剪枝上界
fn dfs_squares(remain: i32, used: i32, best: &mut i32) {
    if remain == 0 {
        *best = (*best).min(used);
        return;
    }
    if used + 1 >= *best {
        return;
    }

    let mut j = 1;
    while j * j <= remain {
        dfs_squares(remain - j * j, used + 1, best);
        j += 1;
    }
}

fn brute_force(n: i32) -> i32 {
    // 上界：全部用 1
    let mut best = n;
    dfs_squares(n, 0, &mut best);
    best
}

fn check(n: i32, want: i32) {
    assert_eq!(Solution1::num_squares(n), want, "dp {}", n);
    assert_eq!(Solution2::num_squares(n), want, "bfs {}", n);
    assert_eq!(Solution3::num_squares(n), want, "math {}", n);
}

fn main() {
    // 示例 1 / 示例 2
    check(12, 3);
    check(13, 2);

    // 1 到 20 的全部答案，逐个钉死
    let expected = [
        1, 2, 3, 1, 2, 3, 4, 2, 1, 2, 3, 3, 2, 3, 4, 1, 2, 2, 3, 2,
    ];
    for (i, &want) in expected.iter().enumerate() {
        check(i as i32 + 1, want);
    }

    // 边界：本身就是平方数，答案为 1
    check(1, 1);
    check(4, 1);
    check(10000, 1);

    // 边界：4^a(8b+7) 形态，答案为 4
    check(7, 4);
    check(28, 4);
    check(9999, 4);

    // 边界：需要 3 个的典型值
    check(3, 3);
    check(11, 3);

    // 对拍：1..300 逐个与穷举比对
    for n in 1..=300 {
        check(n, brute_force(n));
    }

    println!("all tests passed");
}
