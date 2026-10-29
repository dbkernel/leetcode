/*
============================================================================
基数排序（Radix Sort，LSD 十进制）
分类：手写数据结构 · 排序算法
复杂度：时间 O(d(n + 10))；空间 O(n + 10)；稳定性：稳定
        d 为最大值的十进制位数
============================================================================

用途：
  值域很大但位数不多（32 位整数最多 10 位）时，用「按位分配」
  代替「按值比较」；面试常追问「为什么要求稳定」与「负数怎么办」。

核心思想：
  从最低位到最高位，每一位用一次稳定的计数排序把数组按该位排好；
  低位排好的相对次序会被高位排序保留，d 趟之后整体有序。

关键点（不变量）：
  · 子过程必须稳定：同一位数字相同的元素要保持上一趟的相对次序；
  · 负数单独处理：取绝对值排好再反转负数段，不能和非负数混排；
  · exp 每次乘 10，到最高位就停，多乘一次只会空跑还可能溢出；
  · 十进制每趟只有 10 个桶，所以每趟 O(n + 10)，不是 O(n + K)。

手推一遍：
  以 [13, 21, 4] 为例：
  1. 个位：13→3、21→1、4→4，按个位稳定排序得 [21, 13, 4]；
  2. 十位：21→2、13→1、4→0，按十位稳定排序得 [4, 13, 21]；
  3. 十位已是最高位，结束。
  负数同理：[-5, 21, -13] 取绝对值 [5, 13] 排好再反转成
  [-13, -5]，拼上 [21] 得 [-13, -5, 21]。

复杂度分析：
  时间 O(d(n + 10))：共 d 趟，每趟计数 + 前缀和 + 回填都是 O(n + 10)。
    d = 10 时相当于 10 倍常数；数据量小时不一定比快排快。
  空间 O(n + 10)：每趟的输出缓冲 n，计数桶 10 个。
    负数分段还额外占 O(n) 的临时数组，这是「按符号分段」的代价。
  稳定性：稳定，前提是每趟的计数排序都稳定（倒序回填保证）。
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

// 一趟：按 exp 位（1, 10, 100 ...）对非负数组做稳定计数排序。
fn counting_by_digit(a: &mut [i64], buf: &mut [i64], exp: i64) {
    let mut cnt = [0usize; 10];
    let n = a.len();
    for &v in a.iter() {
        cnt[((v / exp) % 10) as usize] += 1;
    }
    for i in 1..10 {
        cnt[i] += cnt[i - 1];
    }
    // 倒序回填，同一位数字的元素保持原次序 => 这一趟稳定
    for i in (0..n).rev() {
        let d = ((a[i] / exp) % 10) as usize;
        cnt[d] -= 1;
        buf[cnt[d]] = a[i];
    }
    a.copy_from_slice(buf);
}

// 从个位到最高位逐位稳定排序。要求元素全为非负。
fn radix_lsd(a: &mut [i64]) {
    let n = a.len();
    if n < 2 {
        return;
    }
    let maxv = *a.iter().max().unwrap();
    let mut buf = vec![0i64; n];
    let mut exp = 1i64;
    while exp <= maxv {
        counting_by_digit(a, &mut buf, exp);
        // maxv / 10 之后再乘 10 就越过最高位了，提前收工避免溢出
        if exp > maxv / 10 {
            break;
        }
        exp *= 10;
    }
}

fn radix_sort(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 {
        return;
    }

    // 负数单独处理：按绝对值排好，再把负数段反转（绝对值升序 =>
    // 数值降序）。代价是多一趟分组和 O(n) 临时空间，好处是不必
    // 整体偏移到非负，min 很负、max 很大时也不会因偏移而溢出。
    let mut neg_mag: Vec<i64> = Vec::new(); // 负数的绝对值
    let mut pos: Vec<i64> = Vec::new(); // 非负数
    for &v in a.iter() {
        if v < 0 {
            neg_mag.push(-(v as i64)); // 先转 64 位，i32::MIN 也不溢出
        } else {
            pos.push(v as i64);
        }
    }
    radix_lsd(&mut pos);
    radix_lsd(&mut neg_mag);

    let mut k = 0;
    for &m in neg_mag.iter().rev() {
        a[k] = -m as i32; // 绝对值大的负数更小
        k += 1;
    }
    for &v in pos.iter() {
        a[k] = v as i32;
        k += 1;
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
    radix_sort(&mut got);
    assert_eq!(got, want, "radix_sort 结果不对");
}

fn main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        let mut a = vec![13, 21, 4];
        radix_sort(&mut a);
        print!("sorted: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![4, 13, 21]);
    }

    // 2. 含负数、含 0：负数段反转与 0 的归属都在这里验
    {
        let mut a = vec![-5, 21, -13, 0, 0, -5];
        radix_sort(&mut a);
        print!("with negatives: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![-13, -5, -5, 0, 0, 21]);
    }

    // 3. 边界：空数组、单元素、全为 0
    {
        let mut empty: Vec<i32> = Vec::new();
        radix_sort(&mut empty);
        assert!(empty.is_empty());

        let mut one = vec![-9];
        radix_sort(&mut one);
        assert_eq!(one, vec![-9]);

        check_sorted(&vec![0; 200]);
    }

    // 4. 大数值：10^9 级别，验证位数循环
    {
        check_sorted(&[1000000000, -1000000000, 999999999, 1, -1, 0,
                       100000000]);
    }

    // 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
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

    // 6. 大值域随机对拍：[-10^9, 10^9]，含负数
    {
        let mut rng = Rng::new();
        for _ in 0..20 {
            let a: Vec<i32> = (0..500)
                .map(|_| rng.next_int(-1000000000, 1000000000))
                .collect();
            check_sorted(&a);
        }
    }

    println!("all tests passed");
}
