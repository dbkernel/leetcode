/*
============================================================================
46. 全排列（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/permutations/
分类：回溯 · 排列树（每个元素只用一次）

给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。
你可以 按任意顺序 返回答案。

示例 1：
输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

示例 2：
输入：nums = [0,1]
输出：[[0,1],[1,0]]

示例 3：
输入：nums = [1]
输出：[[1]]

提示：
- 1 <= nums.length <= 6
- -10 <= nums[i] <= 10
- nums 中的所有整数 互不相同
*/
/*
解法一：回溯 + used 标记数组【最优解】

为什么慢：最直白的做法是「每个位置都从 n 个数里挑一个」，能枚举出 n^n 个
长度为 n 的序列，最后再去重。但合法的排列只有 n! 个，绝大多数枚举结果都
含重复元素，在去重之前就白算了。

换个角度：把答案看成一棵排列树。第 0 层决定第 0 位放谁，第 1 层在剩下的数
里决定第 1 位放谁……走到第 n 层正好凑成一个排列。每一层只从「当前路径上
还没出现过」的数里挑，天然不重不漏。

关键点：
  · used[i] 在选 nums[i] 前置 true、回溯后置 false —— 它表达的是
    「nums[i] 是否在当前路径上」，而不是「是否被访问过」
  · 结果在 path.len() == n 时收集；若写在 for 循环里，会漏掉最后一位
    已经被占满、循环体一次都不执行的情况
  · path 用 push/pop 复用同一块内存，只有收集结果时才复制

手推一遍：
  以示例 1（nums = [1,2,3]）为例：
  第 0 层选 1 → 第 1 层选 2 → 第 2 层选 3，得到 [1,2,3]
  回到第 1 层改选 3 → 第 2 层只剩 2，得到 [1,3,2]
  回到第 0 层选 2、选 3，同法各得两个，共 3 × 2 = 6 个 ✔

复杂度分析：
  时间 O(n·n!)：叶子 n! 个，每个叶子复制长度 n 的路径；内部节点总数
    也是 O(n!) 量级，不改变阶
  空间 O(n)：递归深度 n，used 与 path 各 O(n)；返回结果不计入
*/

// 单文件 rustc 编译时平台不提供 Solution 类型，这里自行声明
pub struct Solution1;

impl Solution1 {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(
            nums: &[i32],
            used: &mut [bool],
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                ans.push(path.clone());
                return;
            }
            for i in 0..nums.len() {
                // 已在当前路径上的数不能再选
                if used[i] {
                    continue;
                }
                used[i] = true;
                path.push(nums[i]);
                dfs(nums, used, path, ans);
                path.pop();
                used[i] = false;
            }
        }

        let n = nums.len();
        let mut used = vec![false; n];
        let mut path = Vec::with_capacity(n);
        let mut ans = Vec::new();
        dfs(&nums, &mut used, &mut path, &mut ans);
        ans
    }
}

/*
解法二：交换元素，原地生成【最优解，与解法一并列】

为什么慢：解法一需要 used 与 path 两块额外数组，每个节点还要扫完整个
[0, n) 才能知道谁没用过。当递归已经走到第 first 位时，剩下可选的元素其实
就集中在后缀 [first, n) 里，扫描范围可以随之缩小。

换个角度：不再另开 path，而是把 nums 本身当成状态：前缀 [0, first) 是已经
定好的位，后缀 [first, n) 是还没用的元素。枚举 i ∈ [first, n)，交换
nums[first] 与 nums[i]，就等于「把第 first 位定成 nums[i]」；递归处理
first + 1，返回后再交换回来。

关键点（交换法为什么不会重复）：
  · 不同 i 的分支按「落在第 first 位的值」划分；元素互不相同，所以 i 不同
    则第 first 位的值不同，两个分支产出的排列必然不同
  · 每个分支内部只对后缀递归，前缀不再参与交换，已经定好的位不会被改动
  · 于是「第 0 位选谁、第 1 位选谁……」这串选择唯一确定一个排列，反过来
    任一排列也唯一对应一串选择，路径与结果一一对应
  · 递归返回后必须换回；否则父层下一个 i 会在被改乱的数组上继续枚举
  · 叶子处直接复制当前 nums，它已经是完整排列

手推一遍：
  以示例 1（nums = [1,2,3]）为例：
  first = 0：i = 0 不换，后缀 [2,3] 产出 123、132
             i = 1 换成 [2,1,3]，产出 213、231
             i = 2 换成 [3,2,1]，产出 321、312
  共 6 个，且没有重复 ✔

复杂度分析：
  时间 O(n·n!)：与解法一同阶；每个叶子仍要复制长度 n 的数组
  空间 O(n)：只有递归栈，省掉了 used 与 path；返回结果不计入
  取舍：解法一的枚举顺序更直观、便于加剪枝；解法二少两块数组、常数更小，
    但会改动入参，且不排序时输出顺序与解法一不同
*/
pub struct Solution2;

impl Solution2 {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &mut Vec<i32>, first: usize, ans: &mut Vec<Vec<i32>>) {
            if first == nums.len() {
                ans.push(nums.clone());
                return;
            }
            for i in first..nums.len() {
                // 第 first 位换成 nums[i]，前缀不动，后续只在后缀里继续选
                nums.swap(first, i);
                dfs(nums, first + 1, ans);
                // 换回原状，父层的下一个 i 才能从同一份数组出发
                nums.swap(first, i);
            }
        }

        let mut ans = Vec::new();
        dfs(&mut nums, 0, &mut ans);
        ans
    }
}

// ===== 测试辅助 =====
fn verify(
    mut got: Vec<Vec<i32>>,
    nums: &[i32],
    want_size: usize,
) -> Vec<Vec<i32>> {
    assert_eq!(got.len(), want_size);
    let mut want_sorted = nums.to_vec();
    want_sorted.sort_unstable();
    for p in &got {
        // 每个结果必须恰好用掉 nums 的每个元素一次
        let mut q = p.clone();
        q.sort_unstable();
        assert_eq!(q, want_sorted);
    }
    got.sort();
    // 排序后相邻相等说明同一排列生成两次
    for w in got.windows(2) {
        assert!(w[0] < w[1]);
    }
    got
}

fn check(nums: &[i32], want: Vec<Vec<i32>>) {
    let mut expected = want;
    expected.sort();

    let got = verify(
        Solution1::permute(nums.to_vec()),
        nums,
        expected.len(),
    );
    assert_eq!(got, expected);

    let got = verify(
        Solution2::permute(nums.to_vec()),
        nums,
        expected.len(),
    );
    assert_eq!(got, expected);
}

fn check_count(nums: &[i32], want_size: usize) {
    verify(Solution1::permute(nums.to_vec()), nums, want_size);
    verify(Solution2::permute(nums.to_vec()), nums, want_size);
}

fn main() {
    // 示例 1：3 个数，6 个互不相同的排列
    check(
        &[1, 2, 3],
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ],
    );

    // 示例 2：含 0，验证 0 不会被当成「空位」跳过
    check(&[0, 1], vec![vec![0, 1], vec![1, 0]]);

    // 示例 3：单元素，直接返回自身
    check(&[1], vec![vec![1]]);

    // 边界：负数，验证比较与下标逻辑与数值无关
    check(
        &[-1, 0, 1],
        vec![
            vec![-1, 0, 1],
            vec![-1, 1, 0],
            vec![0, -1, 1],
            vec![0, 1, -1],
            vec![1, -1, 0],
            vec![1, 0, -1],
        ],
    );

    // 边界：n = 4，24 个排列
    check_count(&[1, 2, 3, 4], 24);

    // 边界：题目上限 n = 6，720 个排列，顺带压一下拷贝逻辑
    check_count(&[1, 2, 3, 4, 5, 6], 720);

    // 边界：乱序且含负数，结果数量只与 n 有关
    check_count(&[5, -10, 0, 10], 24);

    println!("all tests passed");
}
