#![allow(dead_code)]

/*
============================================================================
插入排序（Insertion Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 最好 O(n)、平均 O(n^2)、最坏 O(n^2)；空间 O(1)；稳定性：稳定
============================================================================

用途：
  小数组或「基本有序」数据的首选排序。面试里常问「为什么它不靠交换」、
  「元素移动多少次」，它也常被当作快排小区间的收尾排序、希尔排序的内层。

核心思想：
  维护已排序前缀 a[0..i-1]，把 a[i] 取出来暂存为 key，然后在前缀里从右
  往左找 key 的落点：比 key 大的元素直接往后挪一格，空出来的位置最后放
  key。前缀逐步扩张，直到整个数组有序。

关键点（不变量）：
  · 进入第 i 轮时 a[0..i-1] 已升序；本轮结束后 a[0..i] 升序。
  · 先暂存 key、再整体后移：a[i] 被 a[i-1] 覆盖之前 key 已经存下来了，
    所以不会丢数据。每轮只做「一次比较 + 一次赋值」，比「比较 + 交换」
    少两次写内存，这是它比冒泡快的原因。
  · 内层条件是 a[j] > key（严格大于）：遇到相等就停下并插在它后面，
    相等元素的次序不变，所以稳定；写成 a[j] >= key 就会变成不稳定。
  · j 是有符号类型且可能减到 -1，所以必须先判 j >= 0，再去取 a[j]；
    若用 usize 做下标，j 减到 -1 会直接下溢 panic。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例：
  1. i = 1，key = 1：a[0] = 3 > 1 后移 → [3, 3, 4, 1, 5]，j 减到 -1，
     落点 0 → [1, 3, 4, 1, 5]。
  2. i = 2，key = 4：a[1] = 3 不大于 4，立即停，落点 2。
  3. i = 3，key = 1：4、3 依次后移 → [1, 3, 3, 4, 5]，j 减到 -1，
     落点 0 → [1, 1, 3, 4, 5]。
  4. i = 4，key = 5：a[3] = 4 不大于 5，立即停。结束。

复杂度分析：
  时间：最好 O(n)（已有序时每轮只比较一次就停），最坏 O(n^2)（逆序时
    每轮都要挪到最前面），平均 O(n^2)（随机数据平均移动 n^2/4 次）。
  空间 O(1)：只有 key、i、j 三个变量，移动都发生在原 Vec 上。
  稳定性：稳定，见关键点第三条。
*/

use std::sync::atomic::{AtomicUsize, Ordering};

// 仅用于自测：统计比较次数，验证「已有序时每轮只比较一次」。
static CMP_COUNT: AtomicUsize = AtomicUsize::new(0);

// 仅用于自测：统计元素后移次数，逆序时应当等于 n(n-1)/2。
static MOVE_COUNT: AtomicUsize = AtomicUsize::new(0);

fn insertion_sort(a: &mut Vec<i32>) {
    let n = a.len();
    // a[0] 单独一个元素天然有序，所以从下标 1 开始扩张前缀。
    for i in 1..n {
        let key = a[i]; // 先暂存，后面 a[i] 会被覆盖
        let mut j = i as isize - 1; // 有符号，才能安全地减到 -1
        // 先判 j >= 0 再取 a[j]，否则 as usize 会把 -1 变成天文数字。
        while j >= 0 {
            CMP_COUNT.fetch_add(1, Ordering::Relaxed); // 比较真实发生
            // 小于等于就停：相等元素原地不动，保证稳定。
            if a[j as usize] <= key {
                break;
            }
            a[(j + 1) as usize] = a[j as usize]; // 整体后移一格，不交换
            MOVE_COUNT.fetch_add(1, Ordering::Relaxed);
            j -= 1;
        }
        // 此时 a[j] <= key（或 j == -1），key 落在 j+1。
        a[(j + 1) as usize] = key;
    }
}

fn show(a: &[i32]) -> String {
    let mut s = String::from("[");
    for (i, v) in a.iter().enumerate() {
        if i != 0 {
            s.push_str(", ");
        }
        s.push_str(&v.to_string());
    }
    s.push(']');
    s
}

fn check_sorted(input: &[i32], expect: &[i32], name: &str) {
    let mut a = input.to_vec();
    insertion_sort(&mut a);
    assert!(
        a.as_slice() == expect,
        "case {} failed: got {} expect {}",
        name,
        show(&a),
        show(expect)
    );
}

// 固定种子的线性同余发生器，故意不用时间种子，保证失败可复现。
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Lcg {
        Lcg { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 33) as u32
    }

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        let span = (hi - lo + 1) as u32;
        lo + (self.next_u32() % span) as i32
    }
}

fn main() {
    // 1. 正常用例。
    check_sorted(&[5, 2, 4, 1, 3], &[1, 2, 3, 4, 5], "normal");

    // 2. 边界：空数组、单元素。
    check_sorted(&[], &[], "empty");
    check_sorted(&[7], &[7], "single");

    // 3. 已有序：每轮只比较一次就停，总比较次数恰好 n-1，一次移动都没有。
    CMP_COUNT.store(0, Ordering::Relaxed);
    MOVE_COUNT.store(0, Ordering::Relaxed);
    check_sorted(&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5], "already-sorted");
    let cmp = CMP_COUNT.load(Ordering::Relaxed);
    let mv = MOVE_COUNT.load(Ordering::Relaxed);
    assert!(
        cmp == 4 && mv == 0,
        "已有序：比较 {} 次、移动 {} 次，应为 4 和 0",
        cmp,
        mv
    );

    // 4. 逆序：最坏情况，移动次数 = 0+1+...+(n-1) = n(n-1)/2。
    CMP_COUNT.store(0, Ordering::Relaxed);
    MOVE_COUNT.store(0, Ordering::Relaxed);
    check_sorted(&[5, 4, 3, 2, 1], &[1, 2, 3, 4, 5], "reverse");
    let cmp = CMP_COUNT.load(Ordering::Relaxed);
    let mv = MOVE_COUNT.load(Ordering::Relaxed);
    assert_eq!(mv, 10, "逆序移动 {} 次，应为 10 次", mv);
    // 每轮都一路比到 j == -1，比较次数与移动次数相同。
    assert_eq!(cmp, 10, "逆序比较 {} 次，应为 10 次", cmp);

    // 5. 重复值：全相同 + 交替重复。全相同时每轮只比较一次，
    //    说明相等元素不会触发后移，稳定性靠的正是这一点。
    check_sorted(&[7, 7, 7, 7], &[7, 7, 7, 7], "all-same");
    CMP_COUNT.store(0, Ordering::Relaxed);
    check_sorted(&[7, 7, 7, 7], &[7, 7, 7, 7], "all-same-cmp");
    let cmp = CMP_COUNT.load(Ordering::Relaxed);
    assert_eq!(cmp, 3, "全相同比较 {} 次，应为 3 次", cmp);
    check_sorted(&[2, 1, 2, 1, 2, 1], &[1, 1, 1, 2, 2, 2], "duplicates");
    check_sorted(&[0, 0, -1, -1, 0], &[-1, -1, 0, 0, 0], "dups-negative");

    // 6. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
    let mut rng = Lcg::new(20240501);
    for &len in &[0usize, 1, 2, 3, 10, 100] {
        for round in 0..20 {
            let mut a: Vec<i32> =
                (0..len).map(|_| rng.range(-50, 50)).collect();
            let mut expect = a.clone();
            // 仅用于校验：实现部分没有使用内置排序
            expect.sort();
            insertion_sort(&mut a);
            assert!(
                a == expect,
                "len={} round={}: got {} expect {}",
                len,
                round,
                show(&a),
                show(&expect)
            );
        }
    }

    println!("all tests passed");
}
