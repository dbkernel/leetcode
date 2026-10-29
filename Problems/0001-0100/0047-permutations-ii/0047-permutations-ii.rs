/*
============================================================================
47. 全排列 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/permutations-ii/
分类：回溯 · 排列树（同层去重）

给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。

示例 1：
输入：nums = [1,1,2]
输出：
[[1,1,2],
 [1,2,1],
 [2,1,1]]

示例 2：
输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

提示：
- 1 <= nums.length <= 8
- -10 <= nums[i] <= 10
*/
use std::collections::BTreeSet;

/*
解法一：排序 + used 标记 + 同层去重【最优解】

为什么慢：直接沿用 46 的 used 回溯，[1,1,2] 会输出 6 个排列，而不同的只有
3 个。重复来自「两个相同的 1 交换身份」—— 它们在数组里是两个不同的下标，
在任一条路径上互换两者，得到的排列完全相同。最后用集合去重能通过，但重复
的整棵子树已经被枚举过了，丢掉的是实打实的计算量。

换个角度：先排序，让相同的值挨在一起；再规定「同一层里，相同值只允许第一
个被选中」。这样每个值在每个位置只开出一条分支，重复子树在进入之前就被剪掉。

关键点（`!used[i-1]` 与 `used[i-1]` 的含义完全不同）：
  · 本题采用 `i > 0 && nums[i] == nums[i-1] && !used[i-1]`：
    used[i-1] 为 false 说明前一个相同值「不在当前路径上」，它属于同一层，
    在本层循环里已经被选过又回溯了；再选 nums[i] 开出的分支与它完全重复
  · 若把条件改成 `used[i-1]`（为 true），判的是「前一个相同值仍在当前路径
    上」，即同一树枝上的重复。它同样能得到正确答案，但规则变成「相同值只
    允许按下标从大到小的顺序进入路径」；例如 [1,2] 之后剩下的那个 1 会先被
    放过，绕到另一条分支才补回来，多出一些走到叶子前才被剪掉的死枝。
    实测递归节点数：[1,1,2] 是 9 对 12，[1,1,1] 是 4 对 9。两者同阶，
    但 `!used[i-1]` 在选元素的当下就剪掉，不留死枝
  · 去重必须配合排序：不排序时相同值不相邻，`nums[i] == nums[i-1]` 判不出
    任何东西，重复照样产生
  · 这里写 `i > 0` 而不是 `i > start`：本解法每层都从 0 扫、靠 used 过滤，
    不存在 start，i = 0 时由短路求值跳过前一项比较
  · used 的置位与回溯顺序不能反：先置 true 再递归，回来先 pop 再置 false

手推一遍：
  以示例 1（排序后 nums = [1,1,2]）为例：
  第 0 层选 1a → 第 1 层选 1b（used[0] 为 true，去重条件不触发）
    → 第 2 层选 2，得到 [1,1,2]
  第 0 层选 1a → 第 1 层选 2 → 第 2 层选 1b，得到 [1,2,1]
  第 0 层 i = 1：used[0] 为 false 且两值相同 → 跳过，重复分支不进入
  第 0 层选 2 → 第 1 层选 1a → 第 2 层选 1b，得到 [2,1,1]
  第 1 层再试 i = 1 时，同层去重同样跳过，不会产出第二个 [2,1,1]
  最终恰好 3 个 ✔

复杂度分析：
  时间 O(n·n!)：最坏（元素互不相同）与 46 同阶；有重复时叶子数降到
    n! / ∏(c_i!)，每个叶子复制长度 n 的路径
  空间 O(n)：递归深度 n，used 与 path 各 O(n)；返回结果不计入
*/

// 单文件 rustc 编译时平台不提供 Solution 类型，这里自行声明
pub struct Solution1;

impl Solution1 {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 相同的值相邻，后面的同层去重才判得出来
        nums.sort_unstable();
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
                if used[i] {
                    continue;
                }
                // 前一个相同值不在路径上 = 它同层已试过并回溯，跳过
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
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
解法二：交换法 + 集合去重【非最优】

为什么慢：46 的交换法靠「元素互不相同，所以不同 i 落位的值不同」保证分支
互斥。有重复值后这个前提没了：交换两个相同的 1，数组没有变化，同一条排列
会被不同分支重复生成。最省事的补救是把结果放进集合，但这只是「先生成再丢
弃」，重复枚举的代价一点没少。

换个角度：保留交换法的框架（第一段代码不动），把结果容器换成集合，让集合
按整个数组的值判重。正确性直接，但完全没有利用「排序后相同值相邻」的结构。

关键点：
  · 集合按元素值判重，与生成顺序无关，所以这里排不排序都不影响正确性；
    代码里仍排一次，让两条解法的输出顺序一致，便于对照
  · 交换后递归、递归后必须换回，否则后续分支会基于被改乱的数组继续枚举
  · 集合的键是整个 Vec<i32>，BTreeSet 按字典序比较；若换成 HashSet，
    需要给 Vec<i32> 计算哈希，量级不变
  · 非最优的根因在空间：集合要持有所有 |ans| 个排列，量级 O(n·n!)；
    解法一只有 O(n) 的递归栈与路径

手推一遍：
  以示例 1（nums = [1,1,2]）为例：
  first = 0：i = 0 不换，[1,1,2] 产出 112、121
             i = 1 交换两个 1，数组仍是 [1,1,2]，再产出 112、121（重复）
             i = 2 换成 [2,1,1]，产出 211、211（重复）
  共生成 6 个候选，集合去重后剩 3 个 ✔

复杂度分析：
  时间 O(n·n!·log(n!))：枚举与拷贝 O(n·n!)，另有 n! 次集合插入，
    每次比较 O(n)、树高 O(log(n!))
  空间 O(n·n!)：集合保存全部不同排列；递归栈 O(n) 相比之下可忽略
*/
pub struct Solution2;

impl Solution2 {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        fn dfs(
            nums: &mut Vec<i32>,
            first: usize,
            seen: &mut BTreeSet<Vec<i32>>,
        ) {
            if first == nums.len() {
                seen.insert(nums.clone());
                return;
            }
            for i in first..nums.len() {
                nums.swap(first, i);
                dfs(nums, first + 1, seen);
                // 换回原状，父层的下一个 i 才能从同一份数组出发
                nums.swap(first, i);
            }
        }

        let mut seen = BTreeSet::new();
        dfs(&mut nums, 0, &mut seen);
        seen.into_iter().collect()
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
        Solution1::permute_unique(nums.to_vec()),
        nums,
        expected.len(),
    );
    assert_eq!(got, expected);

    let got = verify(
        Solution2::permute_unique(nums.to_vec()),
        nums,
        expected.len(),
    );
    assert_eq!(got, expected);
}

fn main() {
    // 示例 1：两个 1 只能换身份，不能多出排列
    check(
        &[1, 1, 2],
        vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
    );

    // 示例 2：没有重复值，退化成 46，6 个排列
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

    // 边界：全部相同，只有 1 个排列（专门验证去重不会把唯一解也剪掉）
    check(&[1, 1, 1], vec![vec![1, 1, 1]]);

    // 边界：单元素
    check(&[1], vec![vec![1]]);

    // 边界：负数与 0，排序后 0 夹在中间
    check(
        &[0, -1, -1, 0],
        vec![
            vec![-1, -1, 0, 0],
            vec![-1, 0, -1, 0],
            vec![-1, 0, 0, -1],
            vec![0, -1, -1, 0],
            vec![0, -1, 0, -1],
            vec![0, 0, -1, -1],
        ],
    );

    // 边界：两组重复值，排列数 4! / (2!·2!) = 6
    check(
        &[2, 2, 1, 1],
        vec![
            vec![1, 1, 2, 2],
            vec![1, 2, 1, 2],
            vec![1, 2, 2, 1],
            vec![2, 1, 1, 2],
            vec![2, 1, 2, 1],
            vec![2, 2, 1, 1],
        ],
    );

    // 边界：题目上限 n = 8，全相同，仍只有 1 个
    let all_same = vec![-10; 8];
    check(&all_same, vec![vec![-10; 8]]);

    println!("all tests passed");
}
