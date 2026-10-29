/*
============================================================================
LCR 170 / 剑指 Offer 51. 交易逆序对的总数（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/shu-zu-zhong-de-ni-xu-dui-lcof/
分类：分治 · 归并排序 / 树状数组

在股票交易中，如果前一天的股价高于后一天的股价，则可以认为存在一个「交易
逆序对」。请设计一个程序，输入一段时间内的股票交易记录 record，返回其中
存在的「交易逆序对」总数。

示例 1：
输入：record = [9, 7, 5, 4, 6]
输出：8
解释：交易中的逆序对为 (9, 7), (9, 5), (9, 4), (9, 6), (7, 5), (7, 4),
(7, 6), (5, 4)。

提示：
- 0 <= record.length <= 50000

说明：
- leetcode.cn 已将本题改号为 LCR 170，题名改为「交易逆序对的总数」；
  提交接口的函数名仍是 reverse_pairs，参数名现为 record。
- 原题名「数组中的逆序对」保留在文件名中，便于按剑指题号检索。
*/

// 本地单文件编译需要显式给出 Solution 类型
struct Solution1;
struct Solution2;

/*
解法一：归并排序，合并时统计跨段逆序对【最优解】

为什么慢：直接枚举所有 (i, j) 判断 record[i] > record[j] 是 O(n^2)。
n = 50000 时约 1.25e9 次比较，超时。

换个角度：逆序对只可能在「左半段取一个、右半段取一个」时跨段产生。归并
排序合并两段有序序列时，右段某个元素被取出，说明它小于左段尚未取出的所有
元素；这些元素的下标都在它前面，正好每个都贡献一个逆序对。

关键点：
  · 比较用 record[i] <= record[j] 时取左边：相等不算逆序对，题目要求严格
    大于；若写成 < 会把相等元素也计进去
  · 只有「取右段元素」时才累加 m - i：i 是左段当前未取出的第一个位置，
    左段 [i, m) 全都大于 record[j]
  · 最坏 1.25e9，仍在 i32 范围内；内部仍用 i64 计数，避免后续改题面
    扩大数据范围时溢出
  · 合并完必须把 tmp 写回原数组，否则上一层合并拿到的是旧数据
  · 用半开区间 [l, r)：r - l <= 1 就是长度为 0 或 1，无需特判

以示例 1（record = [9, 7, 5, 4, 6]）为例：
  [9,7,5] 内部：合并 [9] 与 [7] 得 +1；再用 [7,9] 与 [5] 合并，取 5 时
  左段还剩 2 个，+2，内部共 3
  [4,6] 内部：已经有序，0
  最后合并 [5,7,9] 与 [4,6]：取 4 时左段剩 3 个（+3），取 6 时剩 2 个
  （+2），跨段共 5
  总计 3 + 0 + 5 = 8

复杂度分析：
  时间 O(n log n)：归并排序的层数与每层线性合并
  空间 O(n)：tmp 数组与 record 等长；递归栈 O(log n)

解法二：离散化 + 树状数组【最优解，与解法一并列】

为什么慢：值域可能很大，直接开桶统计「比当前值大的数有多少个」会爆内存；
即使值域不大，逐个前缀求和也是 O(n) 一次。

换个角度：只关心相对大小，先把 record 排序，用 partition_point 把每个数
映射成 1..n 的秩。从右往左扫描：已经插入树状数组的就是「下标更大」的元素，
查询秩比当前小的前缀和，就是能与当前元素组成逆序对的数量。

关键点：
  · 从右往左扫描：先查询再插入，避免把自己算进去，也保证只统计 j > i
  · 查询 rank - 1 而不是 rank：相等元素不能算逆序对，必须严格小于
  · 可以不去重：partition_point 让相等元素落到同一个秩，空秩不影响前缀和
  · 树状数组下标从 1 开始，tree 开 n + 1
  · 最坏 n(n-1)/2 = 1.25e9，内部用 i64 计数

以示例 1（record = [9, 7, 5, 4, 6]）为例：
  排序后 [4,5,6,7,9]，秩为 9→5, 7→4, 5→2, 4→1, 6→3
  从右往左：6 插入（0）；4 查询 <1 得 0，插入；5 查询 <2 得 1（4），
  插入；7 查询 <4 得 3（4,5,6），插入；9 查询 <5 得 4（4,5,6,7）
  合计 0 + 0 + 1 + 3 + 4 = 8

复杂度分析：
  时间 O(n log n)：排序 O(n log n) + n 次查询/更新各 O(log n)
  空间 O(n)：排序副本与树状数组各 O(n)
*/
impl Solution1 {
    pub fn reverse_pairs(record: Vec<i32>) -> i32 {
        let mut a = record;
        let n = a.len();
        let mut tmp = vec![0; n];
        Solution1::merge_sort(&mut a, &mut tmp, 0, n) as i32
    }

    // 半开区间 [l, r)；返回区间内的逆序对数，并顺手把区间排好序
    fn merge_sort(a: &mut [i32], tmp: &mut [i32], l: usize, r: usize) -> i64 {
        if r - l <= 1 {
            return 0;
        }

        let m = l + (r - l) / 2;
        let mut count = Solution1::merge_sort(a, tmp, l, m);
        count += Solution1::merge_sort(a, tmp, m, r);

        let (mut i, mut j, mut k) = (l, m, l);
        while i < m && j < r {
            if a[i] <= a[j] {
                // 相等时取左边，保证相等元素不计入逆序对
                tmp[k] = a[i];
                i += 1;
            } else {
                // 右段 a[j] 小于左段 [i, m)，这些元素都与它构成逆序对
                tmp[k] = a[j];
                j += 1;
                count += (m - i) as i64;
            }
            k += 1;
        }
        while i < m {
            tmp[k] = a[i];
            i += 1;
            k += 1;
        }
        while j < r {
            tmp[k] = a[j];
            j += 1;
            k += 1;
        }

        // 写回后才能供上一层继续合并
        a[l..r].copy_from_slice(&tmp[l..r]);
        count
    }
}

struct BIT {
    n: usize,
    tree: Vec<i32>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT {
            n,
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut index: usize) {
        while index <= self.n {
            self.tree[index] += 1;
            index += index & index.wrapping_neg();
        }
    }

    fn prefix_sum(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & index.wrapping_neg();
        }
        sum
    }
}

impl Solution2 {
    pub fn reverse_pairs(record: Vec<i32>) -> i32 {
        let n = record.len();
        if n < 2 {
            return 0;
        }

        // 离散化：相等元素得到同一个秩，不去重也不影响前缀和
        let mut sorted = record.clone();
        sorted.sort_unstable();

        let mut bit = BIT::new(n);
        let mut count = 0i64;
        for &value in record.iter().rev() {
            let rank = sorted.partition_point(|&x| x < value) + 1;
            // 只查严格小于当前秩的元素：相等元素不构成逆序对
            count += bit.prefix_sum(rank - 1) as i64;
            bit.add(rank);
        }
        count as i32
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 暴力枚举，用作小规模用例的独立期望值来源
fn brute_force(record: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..record.len() {
        for j in i + 1..record.len() {
            if record[i] > record[j] {
                count += 1;
            }
        }
    }
    count
}

fn check(input: &[i32], want: i32) {
    let got1 = Solution1::reverse_pairs(input.to_vec());
    let got2 = Solution2::reverse_pairs(input.to_vec());
    assert_eq!(got1, want, "merge({input:?}) = {got1}, want {want}");
    assert_eq!(got2, want, "bit({input:?}) = {got2}, want {want}");
}

fn main() {
    // 示例 1
    check(&[9, 7, 5, 4, 6], 8);

    // 边界：空数组
    check(&[], 0);

    // 边界：单元素
    check(&[7], 0);

    // 边界：已经有序，没有逆序对
    check(&[1, 2, 3, 4, 5], 0);

    // 边界：完全逆序，共 5 * 4 / 2 = 10 个
    check(&[5, 4, 3, 2, 1], 10);

    // 边界：全部相同，严格大于才算逆序对，答案为 0
    check(&[3, 3, 3, 3], 0);

    // 边界：重复值夹杂在乱序里，(2,1)、(2,1)、(2,1) 共 3 个
    check(&[2, 1, 2, 1, 2], 3);

    // 边界：负数与正数混合
    check(&[0, -5, 5, -1, 1], 4);

    // 边界：小规模随机数据，与暴力枚举对照
    let mut seed: u32 = 12345;
    let random: Vec<i32> = (0..200)
        .map(|_| {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            (seed % 401) as i32 - 200
        })
        .collect();
    check(&random, brute_force(&random));

    // 规模用例：50000 个逆序元素，答案 50000 * 49999 / 2
    let large: Vec<i32> = (1..=50000).rev().collect();
    check(&large, 1249975000);

    println!("all tests passed");
}
