/*
============================================================================
快速排序（Quick Sort，三路划分）
分类：手写数据结构 · 排序算法
复杂度：时间 平均 O(n log n)、最坏 O(n^2)；空间 O(log n)；稳定性：不稳定
============================================================================

用途：
  给整数数组原地排序，是面试最常要求手写的第一题；变体是
  「第 k 大」「按颜色/奇偶划分」（荷兰国旗问题）。

核心思想：
  随机取一个元素当枢轴，一趟扫描把区间分成「小于 / 等于 / 大于」
  三段；等值段已经落在最终位置，再对两侧各自递归即可。

关键点（不变量）：
  · 扫描中 [lo, lt) 全小于 pivot，[lt, i) 全等于 pivot，
    [gt, hi) 全大于 pivot，i 之前的元素都已归位；
  · a[i] > pivot 时换进来的元素还没看过，所以 i 不能右移；
  · 枢轴随机取（固定种子），全相同或已有序都不会退化成 O(n^2)；
  · 三路划分让「全相同」一趟结束，等于段不再进入递归。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例，假设随机到 pivot = 4（下标 2）：
  1. lt = 0、i = 0、gt = 5；
  2. a[0]=3 < 4：与 a[lt] 交换（自换），lt=1、i=1；
  3. a[1]=1 < 4：同上，lt=2、i=2；
  4. a[2]=4 == 4：只右移 i，i=3；
  5. a[3]=1 < 4：与 a[2] 交换得 [3, 1, 1, 4, 5]，lt=3、i=4；
  6. a[4]=5 > 4：gt 先左移到 4 再交换（自换），此时 i == gt 结束。
  等于段是 [3, 4)，左段 [3, 1, 1] 再排得 [1, 1, 3]，右段为空，
  整体 [1, 1, 3, 4, 5]。

复杂度分析：
  时间：平均 O(n log n)，每层划分共 O(n)，随机化后树高期望 O(log n)。
    最坏 O(n^2)：每次枢轴都取到极值（靠随机化兜底，本例测不到）。
    全相同数组因三路划分只扫一趟，是 O(n) 而不是 O(n^2)。
    小区间（长度 <= 16）改用插入排序能削掉大部分递归与常数开销，
    本例为了只留一份实现没有加，结果不变、只是常数略大。
  空间：原地，只有递归栈。最坏 O(n)（每次都只切掉一个元素），
    随机化后期望 O(log n)。若想严格 O(log n)，可只递归较短的一侧、
    较长的一侧用循环续做，本例保持一份最朴素实现。
  稳定性：不稳定。划分时会把远处的元素直接换到前面，
    相等元素的相对次序被打乱。
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

// 三路划分的结果：a[lo, lt) < pivot，a[lt, gt) == pivot，
// a[gt, hi) > pivot。
struct Split {
    lt: usize,
    gt: usize,
}

// 仅用于自测：划分次数与递归最大深度。
#[derive(Default)]
struct QuickStats {
    calls: usize,
    depth: usize,
}

// 三路划分（荷兰国旗）。等于段已经就位，不再进入递归。
fn partition3(a: &mut [i32], lo: usize, hi: usize,
              rng: &mut Rng, calls: &mut usize) -> Split {
    *calls += 1;
    // 随机枢轴：下标落在 [lo, hi) 内，先取出值再扫描
    let p = lo + (rng.next_u32() as usize) % (hi - lo);
    let pivot = a[p];

    let (mut lt, mut i, mut gt) = (lo, lo, hi);
    while i < gt {
        if a[i] < pivot {
            // [lt, i) 全是等值元素，换到 lt 只是把等值段往前挪
            a.swap(lt, i);
            lt += 1;
            i += 1;
        } else if a[i] > pivot {
            gt -= 1;
            // 换进来的元素还没看过，所以 i 原地不动
            a.swap(i, gt);
        } else {
            i += 1;
        }
    }
    Split { lt, gt }
}

fn qs(a: &mut [i32], lo: usize, hi: usize, rng: &mut Rng,
      st: &mut QuickStats, depth: usize) {
    if hi - lo < 2 {
        return; // 空或单元素：已有序，递归出口
    }
    if depth > st.depth {
        st.depth = depth;
    }
    let s = partition3(a, lo, hi, rng, &mut st.calls);
    qs(a, lo, s.lt, rng, st, depth + 1); // 严格小于段
    qs(a, s.gt, hi, rng, st, depth + 1); // 严格大于段
}

// 返回 (划分次数, 递归最大深度)，仅用于自测观察复杂度特征。
fn quick_sort_with_stats(a: &mut Vec<i32>) -> (usize, usize) {
    let n = a.len();
    if n < 2 {
        return (0, 0);
    }
    let mut rng = Rng::new();
    let mut st = QuickStats::default();
    qs(a, 0, n, &mut rng, &mut st, 1);
    (st.calls, st.depth)
}

fn quick_sort(a: &mut Vec<i32>) {
    let _ = quick_sort_with_stats(a);
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
    quick_sort(&mut got);
    assert_eq!(got, want, "quick_sort 结果不对");
}

fn main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        let mut a = vec![3, 1, 4, 1, 5];
        quick_sort(&mut a);
        print!("sorted: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![1, 1, 3, 4, 5]);
    }

    // 2. 边界：空数组、单元素
    {
        let mut empty: Vec<i32> = Vec::new();
        quick_sort(&mut empty);
        assert!(empty.is_empty());

        let mut one = vec![7];
        quick_sort(&mut one);
        assert_eq!(one, vec![7]);
    }

    // 3. 已有序：随机枢轴让它不退化，深度应远小于 n
    {
        let mut asc: Vec<i32> = (0..1000).collect();
        let (_, depth) = quick_sort_with_stats(&mut asc);
        assert!(asc.windows(2).all(|w| w[0] <= w[1]));
        assert!(depth <= 64, "升序递归深度");
        println!("ascending n=1000: depth={}", depth);
    }

    // 4. 全相同：三路划分必须一趟结束（划分 1 次、深度 1）
    {
        let mut same = vec![7; 1000];
        let (calls, depth) = quick_sort_with_stats(&mut same);
        assert_eq!(calls, 1);
        assert_eq!(depth, 1);
        assert!(same.iter().all(|&v| v == 7));
    }

    // 5. 随机对拍：含负数与大量重复，长度覆盖 0/1/2/3/10/100/1000
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
