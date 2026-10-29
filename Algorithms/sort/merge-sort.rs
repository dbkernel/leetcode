/*
============================================================================
归并排序（Merge Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n log n)；空间 O(n)；稳定性：稳定
============================================================================

用途：
  给整数数组做稳定排序，也是「分治 + 双指针合并」的入门题。
  面试里常以「排序链表」「求逆序对」的形式出现。

核心思想：
  把数组一分为二，两半各自有序后用两个指针从头各取较小者写进
  辅助数组，一趟线性扫描完成合并；长度 1 的区间天然有序。

关键点（不变量）：
  · 合并时相等取左半（用 <= 判断），稳定性就靠这一处；
  · 辅助数组只分配一次，递归各层复用同一块；
  · 子区间用左闭右开 [lo, hi)，hi - lo 即长度，空区间直接返回；
  · 自底向上版让 width 从 1 翻倍，按层合并，不需要递归栈。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：
  1. 拆成 [3, 1] 和 [4, 1, 5]；
  2. [3, 1] 拆成 [3]、[1]，合并成 [1, 3]；
  3. [4, 1, 5] 拆成 [4]、[1, 5]，[1, 5] 已有序；
  4. 合并 [4] 与 [1, 5]：先取 1，再取 4，再取 5，得 [1, 4, 5]；
  5. 合并 [1, 3] 与 [1, 4, 5]：两个 1 相等，先取左半的 1
     （稳定性），再取 3、4、5，得 [1, 1, 3, 4, 5]。

复杂度分析：
  时间 O(n log n)：递归树高 log n，每层合并共 O(n)，好坏平均一致。
  空间 O(n)：辅助数组；递归栈 O(log n) 被 O(n) 吸收。
  稳定性：稳定。相等时先取左半，而左半元素原本就在前面；
    自底向上版同样只在合并处比较，所以也稳定。
*/
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// 固定种子的 xorshift32：四个语言跑同一条序列，对拍结果可复现。
// ---------------------------------------------------------------------------
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

// 合并 a[lo, mid) 与 a[mid, hi)：先写进 buf，再整体搬回 a。
// 只有 a[i] <= a[j] 时才取左半，相等元素的先后次序因此不变。
fn merge_range(a: &mut [i32], buf: &mut [i32],
               lo: usize, mid: usize, hi: usize) {
    let (mut i, mut j, mut k) = (lo, mid, lo);
    while i < mid && j < hi {
        if a[i] <= a[j] {
            buf[k] = a[i];
            i += 1;
        } else {
            buf[k] = a[j];
            j += 1;
        }
        k += 1;
    }
    while i < mid {
        buf[k] = a[i];
        i += 1;
        k += 1;
    }
    while j < hi {
        buf[k] = a[j];
        j += 1;
        k += 1;
    }
    for t in lo..hi {
        a[t] = buf[t];
    }
}

// 自顶向下：先排两半再合并；buf 由调用方分配，各层共用。
fn msort(a: &mut [i32], buf: &mut [i32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return; // 空或单元素即有序，也是递归出口
    }
    let mid = lo + (hi - lo) / 2;
    msort(a, buf, lo, mid);
    msort(a, buf, mid, hi);
    merge_range(a, buf, lo, mid, hi);
}

fn merge_sort(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 {
        return;
    }
    let mut buf = vec![0i32; n]; // 只分配这一次，各层复用
    msort(a, &mut buf, 0, n);
}

// 自底向上：width = 1, 2, 4 ...，每轮合并相邻两段长度 width 的区间。
// 好处是没有递归栈；链表排序用同一套按层合并的思路，只改 next 指针。
fn merge_sort_bottom_up(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 {
        return;
    }
    let mut buf = vec![0i32; n];
    let mut width = 1;
    while width < n {
        let mut lo = 0;
        while lo < n {
            let mid = lo + width;
            if mid >= n {
                break; // 只剩一段，本轮无需合并
            }
            let hi = usize::min(lo + 2 * width, n);
            merge_range(a, &mut buf, lo, mid, hi);
            lo += 2 * width;
        }
        width *= 2;
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

// 同一组输入分别喂给两个版本，都与内置排序的结果比对。
fn check_both(src: &[i32]) {
    let mut want = src.to_vec();
    want.sort(); // 仅用于校验：实现部分没有使用内置排序

    let mut top = src.to_vec();
    merge_sort(&mut top);
    assert_eq!(top, want, "merge_sort 结果不对");

    let mut bottom = src.to_vec();
    merge_sort_bottom_up(&mut bottom);
    assert_eq!(bottom, want, "merge_sort_bottom_up 结果不对");
}

fn main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        let mut a = vec![3, 1, 4, 1, 5];
        merge_sort(&mut a);
        print!("sorted: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![1, 1, 3, 4, 5]);
    }

    // 2. 边界：空数组、单元素，两个版本都不能越界
    {
        let mut empty: Vec<i32> = Vec::new();
        merge_sort(&mut empty);
        merge_sort_bottom_up(&mut empty);
        assert!(empty.is_empty());

        let mut one = vec![7];
        merge_sort(&mut one);
        merge_sort_bottom_up(&mut one);
        assert_eq!(one, vec![7]);
    }

    // 3. 逆序：每层都要真的搬运，漏合并的写法会在这里挂
    {
        let rev: Vec<i32> = (1..=1000).rev().collect();
        check_both(&rev);
    }

    // 4. 大量重复：全相同 + 只有三种取值
    {
        check_both(&vec![42; 500]);
        let few: Vec<i32> = (0..300).map(|i| i % 3).collect();
        check_both(&few);
    }

    // 5. 随机对拍：长度 0/1/2/3/10/100/1000，含负数与重复
    {
        let mut rng = Rng::new();
        for &n in &[0usize, 1, 2, 3, 10, 100, 1000] {
            for _ in 0..20 {
                let a: Vec<i32> = (0..n)
                    .map(|_| rng.next_int(-50, 50))
                    .collect();
                check_both(&a);
            }
        }
    }

    println!("all tests passed");
}
