/*
============================================================================
计数排序（Counting Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n + K)；空间 O(n + K)；稳定性：稳定
        K 为值域宽度 max - min + 1，不是元素个数
============================================================================

用途：
  值域不大（K 与 n 同量级）时能突破比较排序的 O(n log n) 下界；
  也是基数排序每一位的稳定子过程。面试常问「为什么不用比较」。

核心思想：
  数出每个值出现几次，再做前缀和得到「该值应该放在哪一段」，
  最后按原数组顺序把元素填进输出数组：值本身决定位置，不用比较。

关键点（不变量）：
  · 统一减去 min 做偏移，负数也能当数组下标用；
  · 前缀和之后 cnt[i] 表示「值 <= i + min 的元素个数」，也是右边界；
  · 回填必须倒序遍历原数组：这样同值元素按原次序落在连续区间的
    后半段，前一个同值元素再填到它前面，稳定性由此保证；
  · K 是值域宽度：值域极宽时计数数组比数据还大，会慢且费内存。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：min = 1，K = 5。
  1. 计数：值 1、2、3、4、5 依次出现 2、0、1、1、1 次，
     cnt = [2, 0, 1, 1, 1]；
  2. 前缀和：cnt = [2, 2, 3, 4, 5]；
  3. 倒序回填：a[4] = 5 放到 out[4]；a[3] = 1 放到 out[1]；
     a[2] = 4 放到 out[3]；a[1] = 1 放到 out[0]；a[0] = 3 放到
     out[2]；
  4. out = [1, 1, 3, 4, 5]，搬回 a。

复杂度分析：
  时间 O(n + K)：计数一趟、前缀和 K 趟、回填一趟，与比较无关。
    K 远大于 n 时（比如 n = 100、值域跨 10^9），光清零计数数组就要
    10^9 步，比 O(n log n) 的比较排序慢得多，也更费内存。
  空间 O(n + K)：输出数组 n、计数数组 K。
  稳定性：稳定。倒序回填让同值元素保持原有的相对次序。
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

fn counting_sort(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 {
        return;
    }

    let (mut lo, mut hi) = (a[0], a[0]);
    for &v in a.iter() {
        // 先求值域，负数靠偏移 v - lo 变成合法下标
        if v < lo {
            lo = v;
        }
        if v > hi {
            hi = v;
        }
    }
    let k = (hi - lo + 1) as usize; // 值域宽度
    let mut cnt = vec![0usize; k];
    for &v in a.iter() {
        cnt[(v - lo) as usize] += 1;
    }

    // 前缀和：cnt[i] 变成「值 <= i + lo 的元素个数」，即该值右边界
    for i in 1..k {
        cnt[i] += cnt[i - 1];
    }

    let mut out = vec![0i32; n];
    // 倒序回填：同值元素里靠后的先落位，靠前的再填到它前面，
    // 于是同值元素保持了原数组中的先后次序（稳定）
    for i in (0..n).rev() {
        let v = a[i];
        let idx = (v - lo) as usize;
        cnt[idx] -= 1;
        out[cnt[idx]] = v;
    }
    *a = out;
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
    counting_sort(&mut got);
    assert_eq!(got, want, "counting_sort 结果不对");
}

fn main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        let mut a = vec![3, 1, 4, 1, 5];
        counting_sort(&mut a);
        print!("sorted: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![1, 1, 3, 4, 5]);
    }

    // 2. 含负数：偏移写错（漏减 min）就会越界
    {
        let mut a = vec![-7, 3, -1, 0, -7, 2];
        counting_sort(&mut a);
        print!("with negatives: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![-7, -7, -1, 0, 2, 3]);
        check_sorted(&[-1000000, -999999, -1000001]);
    }

    // 3. 边界：空数组、单元素、全相同
    {
        let mut empty: Vec<i32> = Vec::new();
        counting_sort(&mut empty);
        assert!(empty.is_empty());

        let mut one = vec![-42];
        counting_sort(&mut one);
        assert_eq!(one, vec![-42]);

        check_sorted(&vec![5; 300]);
    }

    // 4. 值域极窄：只有两个取值，计数数组长度为 2
    {
        let a: Vec<i32> = (0..400).map(|i| i % 2).collect();
        check_sorted(&a);
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

    println!("all tests passed");
}
