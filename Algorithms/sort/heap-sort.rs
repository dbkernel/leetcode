/*
============================================================================
堆排序（Heap Sort）
分类：手写数据结构 · 排序算法
复杂度：时间 O(n log n)；空间 O(1)；稳定性：不稳定
============================================================================

用途：
  原地、最坏也能保证 O(n log n) 的排序；面试里常和「第 k 大」
  「数据流中位数」一起出现，考的是 heapify 和下沉写不写对。

核心思想：
  数组天然是一棵完全二叉树（下标 i 的孩子是 2i+1、2i+2）。
  先把数组调成大根堆，堆顶就是最大值；把它换到末尾，堆的规模减一，
  再让新堆顶下沉复位，重复 n-1 次就得到升序数组。

关键点（不变量）：
  · 下沉时只在「比最大的孩子还小」时才交换，否则立刻停；
  · 建堆从最后一个非叶结点 n/2 - 1 往前走，叶子不必下沉；
  · 交换后堆的有效长度变成 end，下沉的边界必须用 end 而不是 n；
  · 大根堆 + 堆顶换到末尾 = 升序；想降序就建小根堆。

手推一遍：
  以 [3, 1, 4, 1, 5] 为例（n = 5）：
  1. i = 1 下沉：1 的孩子是 a[3]=1、a[4]=5，与 5 交换，
     得 [3, 5, 4, 1, 1]；
  2. i = 0 下沉：3 的孩子是 5、4，与 5 交换得 [5, 3, 4, 1, 1]，
     3 落到下标 1 后孩子都是 1，停下，堆建成；
  3. 堆顶 5 换到末尾，前 4 个下沉复位得 [4, 3, 1, 1 | 5]；
  4. 再换 4、3，最后两个 1 交换，整体 [1, 1, 3, 4, 5]。

复杂度分析：
  时间 O(n log n)：建堆 O(n)，之后 n-1 次下沉，每次最多走树高
    O(log n)，合计 O(n log n)，与输入是否有序无关。
  空间 O(1)：原地交换，下沉写成循环，不占递归栈。
  稳定性：不稳定。堆顶换到末尾、孩子与父结点交换都是远距离搬运，
    相等元素的前后次序无法保持。
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

// 让 a[i] 在「有效长度 n」的堆里下沉到合适位置。
// 循环版：不递归，额外空间才是 O(1)。
fn sift_down(a: &mut [i32], mut i: usize, n: usize) {
    loop {
        let l = 2 * i + 1;
        if l >= n {
            break; // 没有孩子，已经落到叶子
        }
        let mut big = l;
        let r = l + 1;
        if r < n && a[r] > a[l] {
            big = r; // 挑较大的孩子
        }
        if a[i] >= a[big] {
            break; // 父结点已经不小于孩子
        }
        a.swap(i, big);
        i = big; // 继续检查被换下去的元素
    }
}

// 自底向上建大根堆：从最后一个非叶结点往前扫。
// 为什么是 O(n)：高度 h 的结点最多 n/2^(h+1) 个，下沉代价 h，
// 总和 sum(h / 2^h) 收敛到常数，故为 O(n) 而不是 O(n log n)。
fn build_max_heap(a: &mut [i32]) {
    let n = a.len();
    let mut i = n / 2;
    while i > 0 {
        i -= 1;
        sift_down(a, i, n);
    }
}

fn heap_sort(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 {
        return;
    }
    build_max_heap(a);
    let mut end = n - 1;
    while end > 0 {
        a.swap(0, end); // 当前最大值放到它最终的位置
        sift_down(a, 0, end); // 边界是 end：末尾已归位，不再参与
        end -= 1;
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

// 逐结点检查大根堆性质：每个父结点都不小于它的孩子。
fn is_max_heap(a: &[i32]) -> bool {
    let n = a.len();
    for i in 0..n {
        let l = 2 * i + 1;
        if l < n && a[i] < a[l] {
            return false;
        }
        let r = l + 1;
        if r < n && a[i] < a[r] {
            return false;
        }
    }
    true
}

fn check_sorted(src: &[i32]) {
    let mut want = src.to_vec();
    want.sort(); // 仅用于校验：实现部分没有使用内置排序
    let mut got = src.to_vec();
    heap_sort(&mut got);
    assert_eq!(got, want, "heap_sort 结果不对");
}

fn main() {
    // 1. 正常用例：与上面「手推一遍」的数组一致
    {
        let mut a = vec![3, 1, 4, 1, 5];
        heap_sort(&mut a);
        print!("sorted: ");
        print_vec(&a);
        println!();
        assert_eq!(a, vec![1, 1, 3, 4, 5]);
    }

    // 2. 边界：空数组、单元素
    {
        let mut empty: Vec<i32> = Vec::new();
        heap_sort(&mut empty);
        assert!(empty.is_empty());

        let mut one = vec![7];
        heap_sort(&mut one);
        assert_eq!(one, vec![7]);
    }

    // 3. 建堆后数组必须满足大根堆性质（关键点写错时这条会挂）
    {
        let mut rng = Rng::new();
        for _ in 0..100 {
            let n = 1 + (rng.next_u32() % 200) as usize;
            let mut a: Vec<i32> = (0..n)
                .map(|_| rng.next_int(-100, 100))
                .collect();
            build_max_heap(&mut a);
            assert!(is_max_heap(&a), "建堆后的堆性质: {:?}", a);
        }
        let mut ordered = vec![1, 2, 3, 4, 5];
        build_max_heap(&mut ordered); // 有序输入也要能建成大根堆
        assert!(is_max_heap(&ordered));
    }

    // 4. 已有序 / 逆序 / 全相同
    {
        let asc: Vec<i32> = (0..500).collect();
        let desc: Vec<i32> = (1..=500).rev().collect();
        check_sorted(&asc);
        check_sorted(&desc);
        check_sorted(&vec![9; 500]);
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
