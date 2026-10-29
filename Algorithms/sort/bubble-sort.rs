#![allow(dead_code)]

/*
============================================================================
冒泡排序（Bubble Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 最好 O(n)、平均 O(n^2)、最坏 O(n^2)；空间 O(1)；稳定性：稳定
============================================================================

用途：
  把数组按升序排好。面试里常以「手写一个稳定排序」或「冒泡为什么能提前
  退出」的形式出现，也常用来热身，考察对不变量和边界下标的把控。

核心思想：
  每一趟只在相邻元素之间比较，逆序就交换，于是一个元素每趟最多向右挪
  一格，最大的那个必然被一路「推」到本趟未排序区的末尾。趟数增加时未
  排序区同步变短，问题规模一路缩到只剩一个元素。

关键点（不变量）：
  · 不变量：第 k 趟结束后，末尾 k 个元素已经就位（是全局最大的 k 个，
    且内部升序）。代码里外层变量 i 从 0 开始，表示「已跑完 i 趟」，
    于是本趟未排序区是 a[0..n-1-i]，内层上界取 n-1-i：最后一次比较
    发生在 a[n-2-i] 与 a[n-1-i] 之间，正好把最大值顶到 a[n-1-i]。
  · 一趟里一次交换都没发生，说明所有相邻对都满足 a[j] <= a[j+1]，
    整个数组已有序，可以直接跳出，这就是最好 O(n) 的来源。
  · 交换条件是严格大于 a[j] > a[j+1]，相等时不换，相等元素的相对次序
    因此不会改变，所以排序稳定。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例（n = 5）：
  1. 第 0 趟，j 走 0..3：(3,1) 换 → [1,3,4,1,5]；(3,4) 不换；
     (4,1) 换 → [1,3,1,4,5]；(4,5) 不换。末尾 1 个元素 5 就位。
  2. 第 1 趟，j 走 0..2：(1,3) 不换；(3,1) 换 → [1,1,3,4,5]；
     (3,4) 不换。末尾 2 个元素 4、5 就位。
  3. 第 2 趟，j 走 0..1，一次交换都没有，swapped 仍为 false，退出。

复杂度分析：
  时间 O(n^2)：最坏（逆序）与平均都是约 n 趟、每趟线性比较；最好 O(n)，
    已有序时只扫一趟就提前退出。
  空间 O(1)：只有 swapped、循环变量和交换用的临时量。
  稳定性：稳定，相等元素不交换。
*/

use std::sync::atomic::{AtomicUsize, Ordering};

// 仅用于自测：统计相邻比较次数，用来验证「已有序时只扫一趟」。
static CMP_COUNT: AtomicUsize = AtomicUsize::new(0);

fn bubble_sort(a: &mut Vec<i32>) {
    let n = a.len();
    // 最多 n-1 趟：每趟至少让一个元素就位，剩 1 个元素时自然有序。
    // saturating_sub 只是为了让 n == 0 时不会下溢，行为与空循环一致。
    for i in 0..n.saturating_sub(1) {
        let mut swapped = false;
        // 未排序区是 a[0..n-1-i]，相邻对的最大下标是 n-1-i，
        // 所以 j 上界取 n-1-i，再大就越过已就位区了。
        for j in 0..n - 1 - i {
            CMP_COUNT.fetch_add(1, Ordering::Relaxed);
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                swapped = true;
            }
        }
        // 没发生交换说明已经整体有序，继续跑纯属浪费。
        if !swapped {
            break;
        }
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
    bubble_sort(&mut a);
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

    // 3. 已有序：只允许扫一趟，也就是恰好 n-1 次相邻比较。
    //    如果去掉 swapped 提前退出，这里会变成 (n-1)+(n-2)+... 次。
    let mut sorted_input = vec![1, 2, 3, 4, 5];
    CMP_COUNT.store(0, Ordering::Relaxed);
    bubble_sort(&mut sorted_input);
    assert_eq!(sorted_input, vec![1, 2, 3, 4, 5]);
    let used = CMP_COUNT.load(Ordering::Relaxed);
    assert_eq!(used, 4, "已有序数组比较 {} 次，应为 4 次", used);

    // 4. 逆序：最坏情况，内层每趟都跑满。
    check_sorted(&[5, 4, 3, 2, 1], &[1, 2, 3, 4, 5], "reverse");

    // 5. 大量重复值：含全相同、以及交替重复。
    check_sorted(&[7, 7, 7, 7], &[7, 7, 7, 7], "all-same");
    check_sorted(&[2, 1, 2, 1, 2, 1], &[1, 1, 1, 2, 2, 2], "duplicates");

    // 6. 随机对拍：固定种子，长度覆盖 0/1/2/3/10/100，含负数与重复值。
    let mut rng = Lcg::new(20240501);
    for &len in &[0usize, 1, 2, 3, 10, 100] {
        for round in 0..20 {
            let mut a: Vec<i32> =
                (0..len).map(|_| rng.range(-50, 50)).collect();
            let mut expect = a.clone();
            // 仅用于校验：实现部分没有使用内置排序
            expect.sort();
            bubble_sort(&mut a);
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
