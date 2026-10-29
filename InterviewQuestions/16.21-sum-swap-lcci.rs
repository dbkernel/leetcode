/*
============================================================================
面试题 16.21. 交换和（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sum-swap-lcci/
分类：哈希表 · 数学（差值分解与集合判定）

给定两个整数数组，请交换一对数值（每个数组中取一个数值），使得两个数组所有
元素的和相等。

返回一个数组，第一个元素是第一个数组中要交换的元素，第二个元素是第二个数组中
要交换的元素。若有多个答案，返回任意一个均可。若无满足条件的数值，返回空数组。

示例 1：
输入：array1 = [4, 1, 2, 1, 1, 2], array2 = [3, 6, 3, 3]
输出：[1, 3]

示例 2：
输入：array1 = [1, 2, 3], array2 = [4, 5, 6]
输出：[]

提示：
- 1 <= array1.length, array2.length <= 100000
*/
use std::collections::HashSet;

/*
解法一：排序 + 双指针【非最优】

为什么慢：把「存在一对 (a, b) 满足 a - b = target」交给排序后的双指针，需要先付
O(m log m + n log n) 的排序代价；好处是额外空间只有排序栈的 O(log)，比建集合的
O(n) 省。

换个角度：两个数组和之差 sum1 - sum2 是固定的。交换 a 与 b 之后，和的变化量是
(b - a) 与 (a - b) 各一次，抵消后得到 2(a - b) = sum1 - sum2。于是问题只剩一个
等式 a - b = (sum1 - sum2) / 2，与「哪两个下标」无关，只与取值有关。

关键点：
  · 先判 (sum1 - sum2) 的奇偶性：差是奇数时 target 不是整数，直接无解
  · target 必须用 i64 存：长度 10^5、取值到 i32 上限时和会溢出
  · 排序后 a 随 i 增大、b 随 j 增大；当前差 diff = a - b 偏小就增大 a，
    偏大就增大 b，两指针各自单向移动，不会退回去
  · 找到即返回；两指针有一个越界就说明无解

以示例 1（array1 = [4,1,2,1,1,2]，array2 = [3,6,3,3]）为例：
  sum1 = 11，sum2 = 15，target = (11 - 15) / 2 = -2
  排序后 array1 = [1,1,1,2,2,4]，array2 = [3,3,3,6]
  i=0,j=0：1 - 3 = -2 == target → 返回 [1,3] ✔

复杂度分析：
  时间 O(m log m + n log n)：两个数组排序，双指针部分只占 O(m+n)
  空间 O(log m + log n)：排序递归栈

解法二：哈希集合【最优解】

为什么慢：解法一要排序，时间被抬到 O(n log n)。本题只关心「某个值在不在另一个
数组里」，这类判定用集合就能做到 O(1)，于是总和降到线性；代价是 O(n) 的空间。

换个角度：由 a - b = target 得 b = a - target。枚举 array1 里的 a，只需要回答
「a - target 是否出现在 array2 中」，一次集合查询即可，数组顺序完全无关紧要。

关键点：
  · 和用 i64 累加；target 也为 i64，只在最后传回 i32 时才需要范围检查
  · 先判奇偶：差为奇数直接返回空数组
  · 只建 array2 的集合，然后遍历 array1 —— 方向反了（建 array1 找 b）会让返回
    的两个元素顺序颠倒
  · 多个答案任取一个：本题不要求字典序或最小对，直接返回第一个命中的即可

以示例 1（array1 = [4,1,2,1,1,2]，array2 = [3,6,3,3]）为例：
  target = -2，集合为 {3,6}
  依次检查 4 - (-2) = 6 命中 → 返回 [4,6]（[1,3] 同样是有效答案）✔

以示例 2（array1 = [1,2,3]，array2 = [4,5,6]）为例：
  sum1 = 6，sum2 = 15，差为 -9，是奇数 → 返回 [] ✔

复杂度分析：
  时间 O(m + n)：建集合 O(n)，遍历 array1 做 O(1) 查询
  空间 O(n)：集合最多存 array2 的全部不同值
*/

// 力扣提交时平台给的是 `impl Solution`，本地单文件 rustc 没有该类型，
// 所以下面显式定义；两个解法各用一个类型，避免同名方法冲突
pub struct Solution1;

impl Solution1 {
    pub fn find_swap_values(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
        let sum1: i64 = array1.iter().map(|&x| x as i64).sum();
        let sum2: i64 = array2.iter().map(|&x| x as i64).sum();

        // 交换后和相等 <=> 2(a - b) = sum1 - sum2，差为奇数即无解
        if (sum1 - sum2) % 2 != 0 {
            return Vec::new();
        }
        let target = (sum1 - sum2) / 2;

        let mut a1 = array1;
        let mut a2 = array2;
        // 原地排序：复制一份会把额外空间抬到 O(m+n)
        a1.sort_unstable();
        a2.sort_unstable();

        let (mut i, mut j) = (0, 0);
        while i < a1.len() && j < a2.len() {
            let diff = a1[i] as i64 - a2[j] as i64;
            if diff == target {
                return vec![a1[i], a2[j]];
            }
            // 差偏小说明 a 不够大，偏大说明 b 不够大；两者都只增不减
            if diff < target {
                i += 1;
            } else {
                j += 1;
            }
        }
        Vec::new()
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn find_swap_values(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
        let sum1: i64 = array1.iter().map(|&x| x as i64).sum();
        let sum2: i64 = array2.iter().map(|&x| x as i64).sum();

        // 交换后和相等 <=> 2(a - b) = sum1 - sum2，差为奇数即无解
        if (sum1 - sum2) % 2 != 0 {
            return Vec::new();
        }
        let target = (sum1 - sum2) / 2;

        let pool: HashSet<i32> = array2.into_iter().collect();
        for a in array1 {
            // 需要 b = a - target；超出 i32 范围的值不可能出现在集合里
            let b = a as i64 - target;
            if !(i32::MIN as i64..=i32::MAX as i64).contains(&b) {
                continue;
            }
            if pool.contains(&(b as i32)) {
                return vec![a, b as i32];
            }
        }
        Vec::new()
    }
}

// ===== 测试辅助 =====
// 暴力判定：真的存在一对 (a, b) 能让两数组和相等吗
fn solvable(array1: &[i32], array2: &[i32]) -> bool {
    let sum1: i64 = array1.iter().map(|&x| x as i64).sum();
    let sum2: i64 = array2.iter().map(|&x| x as i64).sum();
    for &a in array1 {
        for &b in array2 {
            if sum1 - a as i64 + b as i64 == sum2 - b as i64 + a as i64 {
                return true;
            }
        }
    }
    false
}

// 任一有效答案都算对：空答案必须确实无解，非空答案必须真的让和相等
fn validate(array1: &[i32], array2: &[i32], got: &[i32]) {
    if got.is_empty() {
        assert!(
            !solvable(array1, array2),
            "{:?} / {:?} 应当有解",
            array1,
            array2
        );
        return;
    }

    assert_eq!(got.len(), 2, "答案长度应为 2");
    let sum1: i64 = array1.iter().map(|&x| x as i64).sum();
    let sum2: i64 = array2.iter().map(|&x| x as i64).sum();
    assert_eq!(
        sum1 - got[0] as i64 + got[1] as i64,
        sum2 - got[1] as i64 + got[0] as i64,
        "交换 {got:?} 后和不等"
    );
    assert!(
        array1.contains(&got[0]),
        "第一个值不在 array1 中",
    );
    assert!(
        array2.contains(&got[1]),
        "第二个值不在 array2 中",
    );
}

fn check(array1: &[i32], array2: &[i32]) {
    let got = Solution1::find_swap_values(array1.to_vec(), array2.to_vec());
    validate(array1, array2, &got);

    let got = Solution2::find_swap_values(array1.to_vec(), array2.to_vec());
    validate(array1, array2, &got);
}

fn main() {
    // 示例 1：多个答案，[1,3] 与 [4,6] 都合法
    check(&[4, 1, 2, 1, 1, 2], &[3, 6, 3, 3]);

    // 示例 2：差为奇数，无解
    check(&[1, 2, 3], &[4, 5, 6]);

    // 边界：两个数组各一个相同元素，交换后和不变
    check(&[5], &[5]);

    // 边界：各一个不同元素，无论怎么换都不等
    check(&[1], &[2]);

    // 反直觉：两数组和已经相等，但没有公共值可换（target = 0）
    check(&[1, 3], &[2, 2]);

    // 边界：负数参与差值计算
    check(&[-1, 4], &[5]);

    // 边界：和超出 32 位，必须按 64 位累加
    check(&[1000000000, 1000000000], &[2, 2]);

    // 边界：重复值也能作为答案
    check(&[1, 1], &[2, 2]);

    println!("all tests passed");
}
