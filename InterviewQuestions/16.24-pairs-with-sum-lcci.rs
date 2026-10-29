/*
============================================================================
面试题 16.24. 数对和（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/pairs-with-sum-lcci/
分类：双指针 · 排序后两端收束（对照：哈希计数）

设计一个算法，找出数组中两数之和为指定值的所有整数对。一个数只能属于一个数对。

示例 1：
输入：nums = [5,6,5], target = 11
输出：[[5,6]]

示例 2：
输入：nums = [5,6,5,6], target = 11
输出：[[5,6],[5,6]]

提示：
- nums.length <= 100000
- -10^5 <= nums[i], target <= 10^5
*/
use std::collections::HashMap;

/*
解法一：排序 + 两端双指针【非最优】

为什么慢：先想最笨的办法 —— 每个数都去问"表里有没有 target - x"，若用两次
循环枚举数对则是 O(n^2)。排序本身要 O(n log n)，比线性慢，所以在时间是更
贵的那一维时，它不如解法二。它的价值在别的维度：只借用了排序的栈空间，
不需要额外的哈希表。

换个角度：数组有序之后，"和太小"与"和太大"各自只有一个调整方向。让
l 指向最小、r 指向最大：和偏小就把 l 右移（换一个更大的左端），和偏大就把
r 左移（换一个更小的右端）。两端夹逼的过程中，每个数只被判定一次。

关键点：
  · 命中时 l 和 r 必须同时移动：题目规定"一个数只能属于一个数对"，
    只挪一侧会让同一个下标被反复配给不同的数
  · 不能先对数组去重再配对：示例 2 的 [5,6,5,6] 要输出两对数对，
    去重后只剩 (5,6) 一对，对数就丢了
  · 重复值本身不需要特殊处理，排序后它们挨在一起，恰好被双指针一对一消耗
  · 提示保证 |nums[i]|、|target| <= 10^5，两个元素相加不会溢出 i32；
    若取值范围再大 10 倍就要换成 i64 比较
  · 参数按值传入，排序只会改动局部副本，与力扣的签名一致

以示例 1（nums = [5,6,5]，target = 11）为例：
  排序 → [5,5,6]，l=0，r=2
  5 + 6 = 11 → 收下 (5,6)，l=1，r=1，l < r 不成立，结束 → [[5,6]] ✔
以示例 2（nums = [5,6,5,6]）为例：
  排序 → [5,5,6,6]，l=0，r=3：5 + 6 = 11 → 收下，l=1，r=2
  仍是 5 + 6 = 11 → 收下，l=2，r=1，结束 → 两对 ✔

复杂度分析：
  时间 O(n log n)：排序是主项，双指针扫描 O(n)
  空间 O(log n)：排序栈开销，除此之外只用了两个下标

解法二：哈希计数，一次遍历配对【最优解】

官方解法。维护"还没被配走的数"的计数表：对当前数 x，先查 target - x 的
计数是否大于 0；有就配成一对并把计数减一，没有就把 x 自己也记一笔。

关键点：
  · 顺序必须是"先查后插"：若先把 x 插入再查，2*x == target 时 x 会和自己
    配成一对，凭空多出一对
  · 计数减一而不是删除，是把"还剩几份"表达清楚；同一份数被配走后不能再
    被别的数用，这正是"一个数只能属于一个数对"
  · 配对成功后当前 x 不入表，因为它的这次出现已经用掉了
  · 表的规模只与"配不上对的元素个数"有关，最坏仍是 O(n)

以示例 1（nums = [5,6,5]，target = 11）为例：
  x=5：查 6，表空 → 记 waiting[5]=1
  x=6：查 5，waiting[5]=1 → 收下 (5,6)，waiting[5]=0
  x=5：查 6，表空 → 记 waiting[5]=1
  结果 [[5,6]] ✔
以示例 2（nums = [5,6,5,6]）为例：
  前两步同上收下第一对；第三个 5 记 waiting[5]=1；第四个 6 命中 → 第二对 ✔

复杂度分析：
  时间 O(n)：每个元素只做常数次哈希操作
  空间 O(n)：最坏情况没有任何数能配对，所有元素都留在表里
*/

// ---------- 解法一：排序 + 两端双指针（非最优） ----------
// 力扣提交时平台给的是 `impl Solution`，本地单文件 rustc 没有类型，
// 所以这里显式定义；两个解法各用一个类型，避免同名方法冲突
pub struct Solution1;

impl Solution1 {
    pub fn pair_sums(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut res: Vec<Vec<i32>> = Vec::new();
        let (mut l, mut r) = (0i32, nums.len() as i32 - 1);
        while l < r {
            let sum = nums[l as usize] + nums[r as usize];
            if sum == target {
                res.push(vec![nums[l as usize], nums[r as usize]]);
                // 同时收两侧：一个下标只能用一次
                l += 1;
                r -= 1;
            } else if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        res
    }
}

// ---------- 解法二：哈希计数（最优解） ----------
pub struct Solution2;

impl Solution2 {
    pub fn pair_sums(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut waiting: HashMap<i32, i32> = HashMap::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for x in nums {
            let need = target - x;
            let left = waiting.entry(need).or_insert(0);
            if *left > 0 {
                *left -= 1;
                res.push(vec![need, x]);
                continue;
            }
            // 配对成功的 x 已经用掉了，不能再入表
            *waiting.entry(x).or_insert(0) += 1;
        }
        res
    }
}

// ===== 测试辅助：题面没规定输出顺序，先规范化再比 =====
fn normalize(pairs: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = pairs
        .iter()
        .map(|p| {
            let mut q = p.clone();
            q.sort_unstable();
            q
        })
        .collect();
    out.sort_unstable();
    out
}

fn check(nums: &[i32], target: i32, want: &[Vec<i32>]) {
    let got = Solution1::pair_sums(nums.to_vec(), target);
    assert_eq!(normalize(&got), normalize(want), "sort({nums:?},{target})");

    let got = Solution2::pair_sums(nums.to_vec(), target);
    assert_eq!(normalize(&got), normalize(want), "hash({nums:?},{target})");
}

fn main() {
    // 示例 1：只有一个 6，所以 5 再多也只能配出一对
    check(&[5, 6, 5], 11, &[vec![5, 6]]);

    // 示例 2：两个 5 配两个 6，对数由较少的重复次数决定
    check(&[5, 6, 5, 6], 11, &[vec![5, 6], vec![5, 6]]);

    // 边界：四个 0 两两配对，用同一个下标两次会得到 4 对而不是 2 对
    check(&[0, 0, 0, 0], 0, &[vec![0, 0], vec![0, 0]]);

    // 边界：奇数个相同元素，落单的那份不能再被使用
    check(&[5, 5, 5], 10, &[vec![5, 5]]);

    // 边界：两数之和由同一个值的两份组成，先插后查会多算出一对
    check(&[5, 5, 1], 10, &[vec![5, 5]]);

    // 边界：正负混合，答案不能把同一个 -1 用两次
    check(&[-1, 0, 1, 2, -1, -4], 0, &[vec![-1, 1]]);

    // 边界：没有配对时返回空数组
    check(&[1], 2, &[]);

    // 常规：多对互不相干
    check(&[1, 2, 3, 4], 5, &[vec![1, 4], vec![2, 3]]);

    println!("all tests passed");
}
