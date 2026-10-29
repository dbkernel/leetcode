/*
============================================================================
桶排序（Bucket Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 平均 O(n)、最坏 O(n^2)；空间 O(n + k)；稳定性：稳定
        k = ceil(sqrt(n)) 为桶数
============================================================================

用途：
  数据在值域上大致均匀时，用「先分桶、桶内小规模排序」换掉全局比较；
  面试常问「k 怎么取」「为什么平均是线性」。

核心思想：
  按值域把 [min, max] 均分成 k 个区间，每个元素按值落进对应桶；
  桶内元素只可能落在本桶的值域里，所以桶内排好后按桶序拼接即可。

关键点（不变量）：
  · 桶下标 idx = (v - min) * k / (max - min + 1)，用 64 位算；
    分母是值域宽度，因此 v = max 时 idx 最大也只会到 k - 1；
  · 同值元素必然进同一个桶，桶内插入排序用严格大于（>）比较，
    相等元素不交换，稳定性的另一半靠这里；
  · 桶按值域区间从左到右拼接，桶间不需要再比较；
  · 全相同或值域极窄时所有元素挤进少数桶，会退化。

手推一遍：
  以 [7, 1, 5, 3] 为例：n = 4，k = 2，值域 [1, 7]，宽度 7。
  1. idx = (v - 1) * 2 / 7：7→1、1→0、5→1、3→0；
  2. 桶 0 = [1, 3]，桶 1 = [5, 7]（桶内插入排序）；
  3. 按桶序拼接得 [1, 3, 5, 7]。

复杂度分析：
  时间：分桶 O(n) + 桶内插入排序 sum(mi^2) + 拼接 O(n)。
    平均 O(n)：均匀输入下每个桶的期望规模 m = n / k；当 k 取到
      n 的量级时 m ≈ 1，桶内 O(1)，总计 O(n)，这是「平均线性」的
      准确前提。本例按题面取 k = ceil(sqrt(n))，m ≈ sqrt(n)，
      桶内约 O(n)、k 个桶合计 O(n sqrt(n))，常数很小、实测很快。
    最坏 O(n^2)：数据全挤进一个桶，退化成一次插入排序。
  空间 O(n + k)：桶里存全部元素共 n 个，桶对象 k 个（k <= n）。
  稳定性：稳定。同值元素同桶 + 桶内插入排序稳定 + 桶序即值域序。
*/
#![allow(dead_code)]

// 固定种子的 xorshift32：四个语言跑同一条序列，对拍结果可复现。
struct Rng {
    state: u32,
}

impl Rng {
    fn new() -> Rng {
        Rng { state: 20260101 }
    }

    fn next_u32(&mut self) -> u32 {
        let mut s = self.state;
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        self.state = s;
        s
    }

    // 返回闭区间 [lo, hi] 内的整数。
    fn next_int(&mut self, lo: i32, hi: i32) -> i32 {
        let span = (hi - lo + 1) as u32;
        lo + (self.next_u32() % span) as i32
    }
}

// 桶内排序用的插入排序。用严格大于比较：相等元素不交换 => 稳定。
fn insertion_sort(b: &mut [i32]) {
    let m = b.len();
    for i in 1..m {
        let key = b[i];
        let mut j = i;
        while j > 0 && b[j - 1] > key {
            // 注意是 >，不是 >=
            b[j] = b[j - 1];
            j -= 1;
        }
        b[j] = key;
    }
}

fn bucket_sort(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 {
        return;
    }

    let (mut lo, mut hi) = (a[0], a[0]);
    for &v in a.iter() {
        if v < lo {
            lo = v;
        }
        if v > hi {
            hi = v;
        }
    }
    // 值域宽度用 64 位：hi - lo + 1 在 i32 上可能溢出
    let span = hi as i64 - lo as i64 + 1;

    let mut k = 1usize;
    while (k * k) < n {
        k += 1; // ceil(sqrt(n))
    }

    // 桶 i 负责第 i 段值域；桶按值域升序排列，拼接后天然有序
    let mut buckets: Vec<Vec<i32>> = (0..k).map(|_| Vec::new()).collect();
    for &v in a.iter() {
        // 先乘后除：保证桶边界按值域均匀切分；大值域下必须 64 位
        let idx = ((v as i64 - lo as i64) * k as i64 / span) as usize;
        buckets[idx].push(v);
    }

    let mut w = 0;
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
        // 桶内元素都落在同一段值域，按桶序写回即全局有序
        for &v in bucket.iter() {
            a[w] = v;
            w += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// 自测
// ---------------------------------------------------------------------------
fn print_vec(a: &[i32]) {
    print!("[");
    for (i, v) in a.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", v);
    }
    print!("]");
}

fn check_sorted(src: &[i32]) {
    let mut want = src.to_vec();
    want.sort(); // 仅用于校验：实现部分没有使用内置排序
    let mut got = src.to_vec();
    bucket_sort(&mut got);
    assert_eq!(got, want, "bucket_sort 结果不对");
}

fn main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        let mut a = vec![7, 1, 5, 3];
        bucket_sort(&mut a);
        print!("sorted: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![1, 3, 5, 7]);
    }

    // 2. 边界：空数组、单元素
    {
        let mut empty: Vec<i32> = Vec::new();
        bucket_sort(&mut empty);
        assert!(empty.is_empty());

        let mut one = vec![7];
        bucket_sort(&mut one);
        assert_eq!(one, vec![7]);
    }

    // 3. 全相同：所有元素挤进一个桶，退化成一个桶的插入排序
    check_sorted(&vec![42; 500]);

    // 4. 已有序 / 逆序 / 含负数 / 值域极窄
    {
        let asc: Vec<i32> = (0..500).collect();
        let desc: Vec<i32> = (1..=500).rev().collect();
        check_sorted(&asc);
        check_sorted(&desc);
        check_sorted(&[-7, -100, 0, 5, -3]);
        check_sorted(&[-1, 0, 1, -1, 0, 1, 0]);
    }

    // 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数
    {
        let mut rng = Rng::new();
        for &n in &[0usize, 1, 2, 3, 10, 100, 1000] {
            for _ in 0..20 {
                let a: Vec<i32> = (0..n)
                    .map(|_| rng.next_int(-50, 50))
                    .collect();
                check_sorted(&a);
            }
        }
    }

    // 6. 值域偏斜：绝大多数元素集中在 [0, 5]，少数跑到 10^6
    {
        let mut rng = Rng::new();
        for _ in 0..20 {
            let mut a: Vec<i32> = (0..300)
                .map(|_| rng.next_int(0, 5))
                .collect();
            a.push(1000000);
            a.push(-1000000);
            check_sorted(&a);
        }
    }

    println!("all tests passed");
}
