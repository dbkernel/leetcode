/*
============================================================================
LCR 186 / 剑指 Offer 61. 文物朝代判断（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/bu-ke-pai-zhong-de-shun-zi-lcof/
分类：数组 · 去重与极差

展览馆展出来自 13 个朝代的文物，每排展柜展出 5 个文物。某排文物的摆放情况
记录于数组 places，其中 places[i] 表示处于第 i 位文物的所属朝代编号。其中，
编号为 0 的朝代表示未知朝代。请判断并返回这排文物的所属朝代编号是否能够视为
连续的五个朝代（如遇未知朝代可算作连续情况）。

示例 1：
输入：places = [0, 6, 9, 0, 7]
输出：True

示例 2：
输入：places = [7, 8, 9, 10, 11]
输出：True

提示：
- places.length = 5
- 0 <= places[i] <= 13

说明：
本题在力扣中文站已改号为 LCR 186，题名改为「文物朝代判断」，原题名为
「剑指 Offer 61. 扑克牌中的顺子」，函数签名由 isStraight(nums) 改为
checkDynasty(places)。下面的题面按当前页面逐字复制。
*/

/*
力扣预置的是 `impl Solution`，本地用 rustc 单文件编译时没有这个类型，
因此三个解法各自显式声明结构体，方法名与平台签名保持一致。
*/

/*
力扣官方没有发布本题题解，下面三种做法按时间、空间从劣到优排列，均按
输入长度 n 的一般情形分析复杂度；本题 n 固定为 5，实际都是常数级。

解法一：排序后跳过 0，检查重复与极差【非最优】

为什么慢：排序本身要 O(n log n)，虽然本题 n = 5 几乎感觉不到，但排序
把"是否连续"这个只依赖极差和重复的问题，做成了比线性扫描更重的操作。

换个角度：五张牌能组成顺子，等价于"非零牌互不重复，且最大值减最小值
小于 5"。极差不超过 4 时，区间里最多 5 个整数，缺的位置正好由 0 补上。

关键点：
  · 排序后 0 全在开头，先跳过再检查，否则 0 之间会被误判成重复
  · 重复检查只看非零牌，两个 0 不算重复
  · 全为 0 时没有非零牌，直接为真，不能让下标越界
  · 极差条件是 max - min < 5，不是 <= 5

手推一遍（places = [0,6,9,0,7]）：
  排序得 [0,0,6,7,9]，非零段 6、7、9
  无重复，极差 9 - 6 = 3 < 5 → true ✔

复杂度分析：
  时间 O(n log n)：排序占主导，后续扫描 O(n)
  空间 O(1)：原地排序，只用常数个下标
*/
pub struct Solution1;

impl Solution1 {
    pub fn check_dynasty(places: Vec<i32>) -> bool {
        let mut places = places;
        places.sort();

        // 0 已经排在前面，跳过它们再谈重复
        let mut i = 0usize;
        while i < places.len() && places[i] == 0 {
            i += 1;
        }

        // 全是未知朝代，可以补成任意连续五段
        if i == places.len() {
            return true;
        }

        for j in (i + 1)..places.len() {
            if places[j] == places[j - 1] {
                return false;
            }
        }

        places[places.len() - 1] - places[i] < 5
    }
}

/*
解法二：哈希集合记录出现过的朝代【非最优】

为什么慢：时间已经从 O(n log n) 降到 O(n)，但为了判重额外开了 O(n) 的
集合。朝代编号只有 0 到 13，这个集合完全可以用一个整数的二进制位代替。

换个角度：扫描时维护三件事——非零值是否重复、最小值、最大值。重复与
极差一起决定答案，不需要保留原始顺序。

关键点：
  · 0 不参与去重，也不参与极值的更新
  · 判重必须发生在更新极值之前，否则重复值会把极值再次写一遍，掩盖错误
  · 没有任何非零值时返回 true
  · 极差判断与解法一相同：max - min < 5

手推一遍（places = [7,8,9,10,11]）：
  依次记录 7、8、9、10、11，无重复
  min = 7，max = 11，极差 4 < 5 → true ✔

复杂度分析：
  时间 O(n)：一次线性扫描
  空间 O(n)：集合最多存 n 个非零值
*/
pub struct Solution2;

impl Solution2 {
    pub fn check_dynasty(places: Vec<i32>) -> bool {
        let mut seen = [false; 14];
        let (mut lo, mut hi) = (14, -1);

        for v in places {
            if v == 0 {
                continue;
            }
            let idx = v as usize;
            if seen[idx] {
                return false;
            }
            seen[idx] = true;
            lo = lo.min(v);
            hi = hi.max(v);
        }

        // 没有已知朝代时，五个位置都能由 0 补成连续段
        if hi == -1 {
            return true;
        }
        hi - lo < 5
    }
}

/*
解法三：位掩码同时完成判重与记录【最优解】

为什么慢：解法二的集合每个元素都要占空间，而朝代编号范围只有 14 个，
一个 32 位整数的每一位就能表示"这个朝代出现过没有"。判重也退化成一次
按位与。

换个角度：把"出现过哪些朝代"压进 mask 的二进制位，扫描时用最低位到
第 13 位记录；重复就是对应位已经为 1。极值仍然单独维护。

关键点：
  · 0 不设置任何位，否则所有未知朝代会被误判成重复
  · 判重写 `mask & (1 << v)`，置位写 `mask |= (1 << v)`，顺序不能反
  · 第 13 位在 i32 范围内，不会溢出
  · 全为 0 时 mask 仍为 0，此时返回 true

手推一遍（places = [0,6,9,0,7]）：
  6 → mask 置第 6 位，lo = hi = 6
  9 → 置第 9 位，hi = 9
  7 → 置第 7 位，hi 仍为 9
  极差 3 < 5 → true ✔

复杂度分析：
  时间 O(n)：一次线性扫描，位运算为常数时间
  空间 O(1)：只用一个掩码和两个极值变量
*/
pub struct Solution3;

impl Solution3 {
    pub fn check_dynasty(places: Vec<i32>) -> bool {
        let mut mask = 0i32;
        let (mut lo, mut hi) = (14, -1);

        for v in places {
            // 0 是未知朝代，不能占位，否则第二个 0 就会被当成重复
            if v == 0 {
                continue;
            }

            let bit = 1i32 << v;
            if mask & bit != 0 {
                return false;
            }
            mask |= bit;

            lo = lo.min(v);
            hi = hi.max(v);
        }

        if hi == -1 {
            return true;
        }
        hi - lo < 5
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 与三种解法独立的参照实现：枚举 1..13 中所有长度为 5 的连续区间
fn brute_dynasty(places: &[i32]) -> bool {
    // 非零值重复时，任何连续区间都无法同时容纳两个相同的编号
    let mut seen = [false; 14];
    for &v in places {
        if v == 0 {
            continue;
        }
        let idx = v as usize;
        if seen[idx] {
            return false;
        }
        seen[idx] = true;
    }

    // 连续五段只可能是 1..5、2..6、……、9..13
    for start in 1..=9 {
        let ok = places
            .iter()
            .all(|&v| v == 0 || (start..=start + 4).contains(&v));
        if ok {
            return true;
        }
    }
    false
}

fn check(places: &[i32], want: bool) {
    assert_eq!(Solution1::check_dynasty(places.to_vec()), want);
    assert_eq!(Solution2::check_dynasty(places.to_vec()), want);
    assert_eq!(Solution3::check_dynasty(places.to_vec()), want);
    assert_eq!(brute_dynasty(places), want);
}

fn main() {
    // 示例 1：两个未知朝代补上缺口
    check(&[0, 6, 9, 0, 7], true);

    // 示例 2：已经是连续五段
    check(&[7, 8, 9, 10, 11], true);

    // 边界：五个位置全未知，可以补成任意连续段
    check(&[0, 0, 0, 0, 0], true);

    // 边界：只有一个已知朝代，其余全未知
    check(&[13, 0, 0, 0, 0], true);
    check(&[0, 0, 0, 0, 1], true);

    // 边界：顺序打乱不影响判断
    check(&[11, 9, 13, 10, 12], true);

    // 反直觉：极差正好为 4 可以，极差为 5 不行
    check(&[8, 0, 5, 0, 9], true);
    check(&[1, 0, 0, 0, 6], false);

    // 反直觉：有重复的非零朝代，即使极差很小也不行
    check(&[1, 1, 2, 3, 4], false);
    check(&[0, 3, 3, 0, 5], false);

    // 反直觉：重复的 0 不算重复
    check(&[0, 0, 3, 4, 5], true);

    // 反直觉：连续段被未知朝代填满
    check(&[4, 7, 0, 0, 0], true);

    // 反直觉：跨度太大，0 不够填
    check(&[1, 2, 3, 4, 7], false);

    // 边界：连续段贴着上界 13
    check(&[13, 12, 11, 10, 9], true);
    check(&[10, 0, 12, 13, 0], true);

    // 穷举 14^5 个输入，与独立参照实现逐一比对
    for a in 0..=13 {
        for b in 0..=13 {
            for c in 0..=13 {
                for d in 0..=13 {
                    for e in 0..=13 {
                        let p = [a, b, c, d, e];
                        let want = brute_dynasty(&p);
                        assert_eq!(Solution1::check_dynasty(p.to_vec()), want);
                        assert_eq!(Solution2::check_dynasty(p.to_vec()), want);
                        assert_eq!(Solution3::check_dynasty(p.to_vec()), want);
                    }
                }
            }
        }
    }

    println!("all tests passed");
}
