/*
============================================================================
216. 组合总和 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/combination-sum-iii/
分类：回溯 · 定长组合 + 和与个数的双重剪枝

找出所有相加之和为 n 的 k 个数的组合，且满足下列条件：

- 只使用数字 1 到 9
- 每个数字 最多使用一次

返回 所有可能的有效组合的列表。该列表不能包含相同的组合两次，组合可以以
任何顺序返回。

示例 1:
输入: k = 3, n = 7
输出: [[1,2,4]]
解释:
1 + 2 + 4 = 7
没有其他可行的组合。

示例 2:
输入: k = 3, n = 9
输出: [[1,2,6], [1,3,5], [2,3,4]]
解释:
1 + 2 + 6 = 9
1 + 3 + 5 = 9
2 + 3 + 4 = 9
没有其他可行的组合。

示例 3:
输入: k = 2, n = 18
输出: []

提示：
- 2 <= k <= 9
- 1 <= n <= 60
*/

/*
解法一：回溯，只在凑满 k 个时校验和【非最优】

为什么慢：从 1 开始往后取数，保证递增以免重复，凑满 k 个之后才把和与 n 比
一次。这条递归树展开的是"所有长度不超过 k 的递增前缀"，数量是
Σ_{j<=k} C(9,j)；而其中大部分前缀既不满足和的要求、也确实还有富余位置可填，
只是代码从不提前判断，只能一路走到第 k 层才发现和不对。

换个角度：和与个数是两条只会单向变化的量 —— 每压入一个数，和只会变大、
剩余空位只会变少。既然"和已经超过 n"和"剩余数字不够凑满 k 个"都能在展开
之前算出来，就没必要把它们留到叶子才检查。

关键点：见解法二。

手推一遍：
以示例 1（k = 3, n = 7）为例：
  1 → 2 → 3，和 6，不到 7，且已满 3 个，丢弃
  1 → 2 → 4，和 7，收集 [1,2,4]
  1 → 2 → 5，和 8，丢弃；…… 1 → 3 → 4 和为 8，丢弃
  2 → 3 → 4 和为 9，丢弃；其余前缀和都超过 7
  只有 [1,2,4] ✔

复杂度分析：
  时间 O(2^9)：9 个数字各"选/不选"，前缀总数 Σ_{j<=k} C(9,j)，最多 512 个
  空间 O(k)：递归栈与 path 都不超过 k 层
*/

// rustc 单文件编译时平台的 Solution 不存在，这里显式声明，提交时删掉即可
pub struct Solution1;

impl Solution1 {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn dfs(
            start: i32,
            k: usize,
            n: i32,
            total: i32,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == k {
                // 只在凑满 k 个时比较和，前面的校验全部推迟到这里
                if total == n {
                    res.push(path.clone());
                }
                return;
            }
            for i in start..=9 {
                path.push(i);
                dfs(i + 1, k, n, total + i, path, res);
                path.pop();
            }
        }

        let mut res = Vec::new();
        let mut path = Vec::new();
        dfs(1, k as usize, n, 0, &mut path, &mut res);
        res
    }
}

/*
解法二：回溯 + 和与剩余个数的双重剪枝【最优解】

为什么慢：解法一进入了每一棵子树的根，才发现"再加下去也不可能凑成"。
两类失败在进入之前就已知：和已经超过 n（后面只会更大），或者剩下的数字
不够补满 k 个。

换个角度：把这两条限制看成对"本层还能取哪些数"的约束。当前已有 len 个、
和为 total：
  · 和已经超过 n（total > n），继续只增不减，整棵子树无解，直接返回
  · 还差 need = k - len 个，从 i 到 9 共 9 - i + 1 个数，必须够 need 个，
    于是 i <= 9 - need + 1
第二个条件正是原循环上界 9 变成一个更小的数。

关键点：
  · 剪枝一放在 dfs 入口：`if total > n { return; }`，它只依赖已经压入的数，
    和后续分支无关，所以整棵子树可以整体剪掉
  · 剪枝二落在循环上界 `i <= 9 - need + 1`：need = k - path.len()，
    末尾的 +1 是"刚好够"的那一个 i，漏掉就会少一批答案
  · 两个剪枝的顺序不能换：need == 0 时必须先比较和再谈能不能继续展开，
    否则凑满 k 个的解会在展开阶段被误判成"剩余不够"
  · 因为每层都从 i + 1 开始，组合天然递增，不需要额外的去重逻辑

手推一遍：
以示例 2（k = 3, n = 9）为例：
  len = 0，need = 3，上界 9 - 3 + 1 = 7，第 0 层只取 1..7
    取 1，total = 1；len = 1，need = 2，上界 8
      取 2，total = 3，need = 1 → 第 2 层从 3 起取到 6，得 [1,2,6]
      取 3，total = 4，need = 1 → 取 5 得 [1,3,5]，取 6 起和就超 9
      取 4，total = 5，need = 1 → 只能从 5 起，最小也到 10，入口剪掉
    取 2，total = 2；len = 1，need = 2，上界 8
      取 3，total = 5，need = 1 → 取 4 得 [2,3,4]
      取 4 起的分支和都超过 9，入口剪掉
  答案正好 3 个 ✔

复杂度分析：
  时间 O(2^9)：9 个数字固定，上界仍是 512；但解法二展开的节点集合是解法一
    的真子集（每个节点都满足 total <= n 且剩余数字够补满），k、n 取极端时
    差别最大 —— k = 9、n = 45 时解法一展开 512 个前缀，解法二只有 10 个
  空间 O(k)：递归栈深度不超过 k
*/

pub struct Solution2;

impl Solution2 {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn dfs(
            start: i32,
            k: usize,
            n: i32,
            total: i32,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            // 剪枝一：和只会越加越大，已经超过 n 就没有可行解
            if total > n {
                return;
            }

            let need = k - path.len();
            if need == 0 {
                if total == n {
                    res.push(path.clone());
                }
                return;
            }
            // 剪枝二：从 start 到 9 只剩 9 - start + 1 个数，不够 need 个就无解
            if 9 - start + 1 < need as i32 {
                return;
            }

            // 与上一句同源，把"剩余够不够"落到每个分支上，免得白开一层递归
            for i in start..=(9 - need as i32 + 1) {
                path.push(i);
                dfs(i + 1, k, n, total + i, path, res);
                path.pop();
            }
        }

        let mut res = Vec::new();
        let mut path = Vec::new();
        dfs(1, k as usize, n, 0, &mut path, &mut res);
        res
    }
}

// 组合集合按题面可以任意顺序返回，内外层都排序后整体比较
fn normalize(v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = v
        .into_iter()
        .map(|mut one| {
            one.sort_unstable();
            one
        })
        .collect();
    out.sort_unstable();
    out
}

// 独立参照：枚举 1..9 的全部子集，留下大小为 k 且和为 n 的
fn brute_force(k: usize, n: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for mask in 0..(1usize << 9) {
        let mut one = Vec::new();
        let mut sum = 0;
        for b in 0..9 {
            if mask & (1usize << b) != 0 {
                one.push(b as i32 + 1);
                sum += b as i32 + 1;
            }
        }
        if one.len() == k && sum == n {
            res.push(one);
        }
    }
    normalize(res)
}

fn check(k: i32, n: i32) {
    let want = brute_force(k as usize, n);
    assert_eq!(
        normalize(Solution1::combination_sum3(k, n)),
        want,
        "noPrune({k},{n})"
    );
    assert_eq!(
        normalize(Solution2::combination_sum3(k, n)),
        want,
        "prune({k},{n})"
    );
}

fn main() {
    // 示例 1：k = 3, n = 7，只有 [1,2,4]
    assert_eq!(
        normalize(Solution1::combination_sum3(3, 7)),
        vec![vec![1, 2, 4]]
    );
    assert_eq!(
        normalize(Solution2::combination_sum3(3, 7)),
        vec![vec![1, 2, 4]]
    );

    // 示例 2：k = 3, n = 9，三组解，验证不重不漏
    let got = normalize(Solution2::combination_sum3(3, 9));
    assert_eq!(
        got,
        normalize(vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]])
    );
    assert_eq!(got.len(), 3);

    // 示例 3：k = 2, n = 18，最小的两个数和为 3，最大的为 17，无解
    assert!(Solution2::combination_sum3(2, 18).is_empty());

    // 边界 k = 9：只有 1..9 全取，和为 45，为 45 时唯一解
    check(9, 45);
    assert_eq!(
        Solution2::combination_sum3(9, 45),
        vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]]
    );

    // 边界：k = 9 但和偏小，剪枝二应把整棵子树挡在入口
    check(9, 1);
    // 边界：k = 2、n = 3，只有 [1,2]
    check(2, 3);
    // 边界：n 取上限 60，超过 1..9 的最大和 45，必空
    check(3, 60);
    // 边界：k = 1（力扣约束 k >= 2，这里验证健壮性），只有 [9]
    check(1, 9);
    // 常规规模
    check(4, 20);
    check(5, 25);

    println!("all tests passed");
}
