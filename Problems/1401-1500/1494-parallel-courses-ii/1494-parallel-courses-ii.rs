/*
============================================================================
1494. 并行课程 II（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/parallel-courses-ii/
分类：状态压缩 DP · 子集枚举

给你一个整数 n 表示某所大学里课程的数目，编号为 1 到 n ，数组 relations
中， relations[i] = [x_i, y_i]  表示一个先修课的关系，也就是课程 x_i 必须
在课程 y_i 之前上。同时你还有一个整数 k 。

在一个学期中，你 最多 可以同时上 k 门课，前提是这些课的先修课在之前的
学期里已经上过了。

请你返回上完所有课最少需要多少个学期。题目保证一定存在一种上完所有课的
方式。

示例 1：
输入：n = 4, relations = [[2,1],[3,1],[1,4]], k = 2
输出：3
解释：上图展示了题目输入的图。在第一个学期中，我们可以上课程 2 和课程 3 。
然后第二个学期上课程 1 ，第三个学期上课程 4 。

示例 2：
输入：n = 5, relations = [[2,1],[3,1],[4,1],[1,5]], k = 2
输出：4
解释：上图展示了题目输入的图。一个最优方案是：第一学期上课程 2 和 3，
第二学期上课程 4 ，第三学期上课程 1 ，第四学期上课程 5 。

示例 3：
输入：n = 11, relations = [], k = 2
输出：6

提示：
- 1 <= n <= 15
- 1 <= k <= n
- 0 <= relations.length <= n * (n-1) / 2
- relations[i].length == 2
- 1 <= x_i, y_i <= n
- x_i != y_i
- 所有先修关系都是不同的，也就是说 relations[i] != relations[j] 。
- 题目输入的图是个有向无环图。
*/

/*
解法：状态压缩 DP + 子集枚举【最优解】

为什么慢：按学期做 BFS/DFS，状态天然是「已修完的课程集合」，可一旦真的
按学期展开，同一集合会被不同学期划分重复到达，且每层要试探的组合是指数级。
贪心也不行：每学期从可选课里挑 k 门「看起来最该上」的，选错就得回退。

换个角度：学期数只和「已修完哪些课」有关，与它们分在哪几学期无关，所以
把集合本身当状态。dp[mask] = 修完 mask 的最少学期数，转移时回头考虑
「最后一学期上了哪一批」。

关键点：
  · sub 要当最后一学期，必须满足三条：sub ⊆ mask、popcount(sub) <= k、
    sub 里每门课的先修都在 mask ^ sub 中（先修必须在之前的学期修完）
  · need[mask] 记 mask 内所有课程的先修并集，用最低位 low = mask & -mask
    递推 need[mask] = need[mask ^ low] | need[low]，一次 O(2^n)
  · 最后一学期只能从 valid = mask ^ need[mask] 里挑：need[mask] 里的课
    总还要给别的课当先修，不可能与本学期同时被修完
  · |valid| <= k 时直接整批全上，不必枚举 —— 一学期装得下的课留到下学期
    只会让学期数变多
  · 否则降序枚举 valid 的非空子集 sub = (sub - 1) & valid，只留
    popcount(sub) <= k 的；这种写法不漏不重，且每次严格变小，必然终止
  · 转移写成 dp[mask ^ sub] + 1，从 mask 递增计算，子状态一定已经算过
  · (need[mask] | mask) != mask 的状态直接跳过：它把「先修在集合外」的
    课也算进了 mask，不可能是任何真实进度，dp 保持 INF
  · Rust 的 usize 没有一元负号，取最低位要写 mask & mask.wrapping_neg()；
    popcount 用 count_ones()

手推一遍（示例 1，n = 4，k = 2，边 2->1、3->1、1->4）：
  位 0..3 依次是课程 1..4，need[0b0001] = {2,3} = 0b0110，
  need[0b1000] = {1} = 0b0001，need[0b1111] = 0b0111
  mask = 0b0110（课程 2、3）：need = 0，valid = 0b0110，只有 2 门，
    整批上，dp = dp[0] + 1 = 1
  mask = 0b0111（课程 1、2、3）：need = 0b0110，valid = 0b0001（只剩课程 1），
    dp = dp[0b0110] + 1 = 2
  mask = 0b1111：valid = 0b1000（只剩课程 4），dp = dp[0b0111] + 1 = 3 ✔
  学期划分是 {2,3} / {1} / {4}，正是示例给出的方案

复杂度分析：
  时间 O(3^n)：2^n 个状态，状态 mask 的转移数是 valid 的子集个数
    2^|valid|；按「sub ⊆ valid ⊆ mask」把每个 (mask, sub) 计一次，
    总数不超过 3^n。n = 15 时约 1.4 * 10^7，实际远小于此
  空间 O(2^n)：need 与 dp 各开 2^n 个元素
*/

// 单文件 rustc 编译需要显式定义 Solution，平台提交时使用同名结构体
pub struct Solution;

impl Solution {
    // 契约：relations[i] = [先修, 后续]，课程编号 1..n，返回最少学期数
    pub fn min_number_of_semesters(
        n: i32,
        relations: Vec<Vec<i32>>,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        let limit = k as u32;
        let inf = (n + 1) as i32; // 最坏也只是每学期一门，n 个学期一定够
        let total = 1usize << n;

        // need[mask]：mask 里所有课程的先修课并集
        let mut need = vec![0usize; total];
        for r in &relations {
            need[1 << (r[1] as usize - 1)] |= 1 << (r[0] as usize - 1);
        }

        let mut dp = vec![inf; total];
        dp[0] = 0;

        for mask in 1..total {
            // 按最低位拆分，复用已经算好的两半
            let low = mask & mask.wrapping_neg();
            need[mask] = need[mask ^ low] | need[low];

            // 先修课不在集合里，这个 mask 不是可达进度，dp 只能是 INF
            if need[mask] | mask != mask {
                continue;
            }

            // 还差 need[mask] 这批课没修，它们不可能出现在最后一学期
            let valid = mask ^ need[mask];
            if valid.count_ones() <= limit {
                // 一学期装得下就整批上，拆成两学期只会更差
                dp[mask] = dp[mask].min(dp[mask ^ valid] + 1);
                continue;
            }

            // 降序枚举 valid 的非空子集，作为最后一批
            let mut sub = valid;
            while sub > 0 {
                if sub.count_ones() <= limit {
                    dp[mask] = dp[mask].min(dp[mask ^ sub] + 1);
                }
                sub = (sub - 1) & valid;
            }
        }

        dp[total - 1]
    }
}

fn check(n: i32, k: i32, relations: &[Vec<i32>], want: i32) {
    let got = Solution::min_number_of_semesters(n, relations.to_vec(), k);
    assert_eq!(
        got, want,
        "minNumberOfSemesters({}, {:?}, {}) = {}, want {}",
        n, relations, k, got, want
    );
}

fn main() {
    // 示例 1：{2,3} / {1} / {4}
    check(4, 2, &[vec![2, 1], vec![3, 1], vec![1, 4]], 3);

    // 示例 2：一学期两门也只能先修完 4 才能修 1
    check(5, 2, &[vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]], 4);

    // 示例 3：没有依赖，纯粹是每学期两门的上取整
    check(11, 2, &[], 6);

    // 边界：只有一门课
    check(1, 1, &[], 1);

    // 边界：k 足够大，所有课一个学期上完
    check(5, 5, &[], 1);

    // 边界：k = 2 且无依赖，5 门课要 3 个学期（不是 2.5 的截断）
    check(3, 2, &[], 2);

    // 反直觉：4 有三个先修，第 3 学期才能轮到它 —— 看着像 2 学期，实际 3 学期
    check(4, 2, &[vec![1, 4], vec![2, 4], vec![3, 4]], 3);

    // 反直觉：层级宽度恰好卡在 k 上，一层一批
    check(
        6,
        2,
        &[
            vec![1, 2],
            vec![1, 3],
            vec![2, 4],
            vec![3, 4],
            vec![4, 5],
            vec![4, 6],
        ],
        4,
    );

    // 反直觉：先修只有一门，但整层两门课 + 收尾不能并行塞进 2 学期
    check(5, 2, &[vec![1, 3], vec![2, 3], vec![3, 4], vec![3, 5]], 3);

    // 边界：n = 15、k = 7、无依赖，最坏规模的子集枚举；15 门要 3 个学期
    check(15, 7, &[], 3);

    println!("all tests passed");
}
