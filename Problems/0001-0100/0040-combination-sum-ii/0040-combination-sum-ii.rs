/*
============================================================================
40. 组合总和 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/combination-sum-ii/
分类：回溯 · 组合树（有重复值，每数只用一次）

给定一个候选人编号的集合 candidates 和一个目标数 target ，找出
candidates 中所有可以使数字和为 target 的组合。

candidates 中的每个数字在每个组合中只能使用 一次 。

注意：解集不能包含重复的组合。

示例 1：
输入: candidates = [10,1,2,7,6,1,5], target = 8,
输出:
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]

示例 2：
输入: candidates = [2,5,2,1,2], target = 5,
输出:
[
[1,2,2],
[5]
]

提示:
- 1 <= candidates.length <= 100
- 1 <= candidates[i] <= 50
- 1 <= target <= 30
*/
/*
解法：排序 + 回溯，下一层从 i + 1 开始，并在同层跳过相同值【最优解】

为什么慢：candidates 里有重复值，直接照搬 39 的「下一层仍从 i 开始」会出
两类问题：一是同一个数会被它的多个副本反复使用，[1a,1b,6] 与 [1b,1a,6]
其实是同一个组合；二是若只用集合去重，重复候选仍然被完整枚举。反过来，若
不加区分地「值相同就跳过」，又会把 [1,1,6] 这种需要两个 1 的合法解误杀。

换个角度：先排序，让相同的值相邻；然后把两个约束拆开、各管一件事：
「每个下标最多用一次」交给 start = i + 1，「同一层不重复选相同值」交给一个
同层跳过条件。前者限制纵向深度，后者限制横向宽度。

关键点（与 39 的差别只有两处，逐条对照）：
  · 下一层下标：39 传 i（可以重复取自己），本题传 i + 1（每个数只用一次）
  · 同层去重：本题多一句 `i > start && cand[i] == cand[i-1] → continue`；
    39 的候选互不相同，所以完全没有这一句
  · 这里的下界是 `i > start`，不是 `i > 0`：start 是本层第一个可选下标，
    i == start 时 cand[i-1] 属于上一层已经选走的元素，拿它比较会把「两个
    相同值都要选」的合法分支误删 —— [1,1,6] 里第二层的那个 1 必须留下
  · 跳过只发生在同层：相同的值仍可以在更深层被选，因为那一层的 start 已经
    越过前一个副本
  · 排序同时服务两件事：break 剪枝需要升序，同层判断需要相同值相邻
  · remain == 0 在递归入口收集；cand[i] > remain 用 break 而不是 continue

手推一遍：
  以示例 1（candidates = [10,1,2,7,6,1,5]，target = 8）为例：
  排序后为 [1,1,2,5,6,7,10]
  第 0 层选 1a → 第 1 层从下标 1 开始，选 1b（i == start，不被跳过）
    → 第 2 层从下标 2 开始，选 6 → remain 0，收集 [1,1,6]
  第 0 层仍是 1a → 第 1 层选 2 → 选 5，收集 [1,2,5]
  第 0 层仍是 1a → 第 1 层选 7，收集 [1,7]
  第 0 层 i = 1：与 i = 0 同值且 i > start(=0) → 跳过，上面三组不再重来
  第 0 层选 2 → 选 6，收集 [2,6]
  共四组 ✔

复杂度分析：
  时间 O(n·2^n)：每个下标只有「选 / 不选」两种去向，剪枝后节点数不超过
    2^n；收集解时还要复制长度最多 n 的路径
  空间 O(n)：递归深度最多 n 层，path 同阶；返回结果不计入
*/

// 单文件 rustc 编译时平台不提供 Solution 类型，这里自行声明
pub struct Solution;

impl Solution {
    pub fn combination_sum2(
        mut candidates: Vec<i32>,
        target: i32,
    ) -> Vec<Vec<i32>> {
        // 升序：break 剪枝与「相同值相邻」都依赖它
        candidates.sort_unstable();
        let mut path = Vec::new();
        let mut ans = Vec::new();

        fn dfs(
            candidates: &[i32],
            remain: i32,
            start: usize,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if remain == 0 {
                ans.push(path.clone());
                return;
            }
            for i in start..candidates.len() {
                if candidates[i] > remain {
                    break;
                }
                // i > start 才说明 candidates[i-1] 属于本层的其他分支；
                // 若写成 i > 0，会把上一层用过的相同值也当成重复而误杀
                if i > start && candidates[i] == candidates[i - 1] {
                    continue;
                }
                path.push(candidates[i]);
                // i + 1：当前下标已经用掉，下一层不能再用它
                dfs(candidates, remain - candidates[i], i + 1, path, ans);
                path.pop();
            }
        }

        dfs(&candidates, target, 0, &mut path, &mut ans);
        ans
    }
}

// ===== 测试辅助 =====
fn normalize(v: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // 组合内部顺序无意义：先给每个组合排序，再给列表排序
    let mut out: Vec<Vec<i32>> = v
        .iter()
        .map(|comb| {
            let mut row = comb.clone();
            row.sort_unstable();
            row
        })
        .collect();
    out.sort();
    out
}

fn check(candidates: &[i32], target: i32, want: Vec<Vec<i32>>) {
    let got = normalize(&Solution::combination_sum2(
        candidates.to_vec(),
        target,
    ));
    assert_eq!(got, normalize(&want));
}

fn main() {
    // 示例 1：两个 1 必须能同时被选中，同时不能因重复副本产出重复组合
    check(
        &[10, 1, 2, 7, 6, 1, 5],
        8,
        vec![
            vec![1, 1, 6],
            vec![1, 2, 5],
            vec![1, 7],
            vec![2, 6],
        ],
    );

    // 示例 2：三个 2 里最多取两个，且只能组成一组
    check(&[2, 5, 2, 1, 2], 5, vec![vec![1, 2, 2], vec![5]]);

    // 边界：恰好需要两个相同的数 —— i > start 写成 i > 0 会返回空集
    check(&[1, 1], 2, vec![vec![1, 1]]);

    // 边界：三个相同的数全部用上
    check(&[1, 1, 1], 3, vec![vec![1, 1, 1]]);

    // 边界：四个 1 取两个，不同组合只有一种
    check(&[1, 1, 1, 1], 2, vec![vec![1, 1]]);

    // 边界：单元素直接命中，另一元素超出 target
    check(&[1, 2], 2, vec![vec![2]]);

    // 边界：重复值组成唯一解，且不能用第三个副本
    check(&[2, 2, 2], 4, vec![vec![2, 2]]);

    // 边界：单元素等于 target
    check(&[1], 1, vec![vec![1]]);

    // 边界：target 小于所有候选
    check(&[3, 4], 2, vec![]);

    // 边界：单个数成解与两数成解并存，等价于「选 / 不选」两种走向
    check(&[1, 2, 3], 3, vec![vec![1, 2], vec![3]]);

    println!("all tests passed");
}
