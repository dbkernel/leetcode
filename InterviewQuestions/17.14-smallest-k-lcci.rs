/*
============================================================================
面试题 17.14. 最小 K 个数（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/smallest-k-lcci/
分类：排序 / 堆 / 快速选择（快排思想）

设计一个算法，找出数组中最小的 k 个数。以任意顺序返回这 k 个数均可。

示例：
输入： arr = [1,3,5,7,2,4,6,8], k = 4
输出： [1,2,3,4]

提示：
0 <= len(arr) <= 100000
0 <= k <= min(100000, len(arr))
*/

/*
解法一：直接排序后取前 k 个【非最优】

官方方法一。对原数组从小到大排序，前 k 个就是答案。

关键点：
  · 题目允许任意顺序返回，因此排序结果直接截断即可
  · 排序会把整个数组都处理好，但我们只关心最小的 k 个，做了多余的工作
  · k == 0 时要返回空数组，别越界

复杂度分析：
  时间 O(n log n)：排序
  空间 O(n)：复制出的排序副本

解法二：大小为 k 的大根堆【非最优】

官方方法二。用一个大根堆实时维护「当前最小的 k 个数」：
  · 先把前 k 个数入堆；
  · 之后每遇到一个比堆顶更小的数，就弹出堆顶、把新数入堆；
  · 扫描结束后，堆里的 k 个数就是答案。

关键点：
  · 必须用大根堆（堆顶是当前 k 小里的最大值），这样才方便「淘汰最大的那个」；
    Rust 标准库没有优先队列，这里手写一个只支持 push/pop/peek 的二叉堆
  · 只有比堆顶更小才替换；相等时替换与不替换结果都对，不必处理
  · k == 0 时堆为空，必须提前返回，否则堆顶无定义
  · 适合 n 很大、k 很小（或数据流无法整体载入内存）的场景

复杂度分析：
  时间 O(n log k)：每个元素至多一次 O(log k) 的堆操作
  空间 O(k)：堆的容量

解法三：快排思想（随机化快速选择）【最优解】

官方方法三。快排的划分（partition）每做一次，就能确定主元的最终位置 pos，
并且 pos 左边都不大于它、右边都不小于它。因此：
  · pos 左边的元素个数（含 pos）恰好是 pos - l + 1 个「当前区间最小」的数；
  · 若这个数量正好等于 k，说明左边这段就是要找的 k 个数，直接结束；
  · 若 k 更小，只需递归左半边；
  · 若 k 更大，说明左边这段都要，还需在右半边再找 k - num 个。
每次只递归一侧，总代价期望 O(n)。

关键点：
  · 划分以区间末尾 nums[r] 为主元；随机化是把「随机选中的下标」先换到末尾，
    避免在有序/近似有序数据上退化成 O(n^2)。这里用固定的线性同余发生器代替
    rand crate，既保持「随机化」的行为，又让用例可复现
  · partition 用「小于等于主元就往前换」的单向写法，最后把主元换到 i + 1
  · 递归返回后，数组前 k 个位置就是答案（内部无序，但集合正确）
  · k == 0 要提前返回空数组；l >= r 是递归出口
  · 判方向时用的是「左边个数 num = pos - l + 1」，不是绝对下标 pos

以 arr = [1,3,5,7,2,4,6,8]，k = 4 为例：
  随机化划分一次，可能得到 pos 落在 3 附近，左边 {1,3,2,4} 共 4 个 → num == k，结束
  返回前 4 个（某种排列，排序后是 [1,2,3,4]）
  若某次 num > k，则只在左半边继续找 k 个；若 num < k，则右半边再找 k - num 个

复杂度分析：
  时间 期望 O(n)，最坏 O(n^2)（划分点极端时）
  空间 期望 O(log n)，最坏 O(n)（递归栈）

三解对比：
  · 排序：实现最简单，O(n log n)，n = 1e5 完全够用
  · 大根堆：O(n log k)，适合 k 远小于 n 或流式数据
  · 快速选择：期望 O(n)，是本题的渐进最优解
*/
struct Solution;

/// 手写大根堆（仅保留本题需要的操作）
struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    fn new(capacity: usize) -> Self {
        MaxHeap {
            data: Vec::with_capacity(capacity),
        }
    }

    fn peek(&self) -> i32 {
        self.data[0]
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
        let mut i = self.data.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[parent] >= self.data[i] {
                break;
            }
            self.data.swap(parent, i);
            i = parent;
        }
    }

    fn pop(&mut self) -> i32 {
        let top = self.data[0];
        let last = self.data.pop().unwrap();
        if !self.data.is_empty() {
            self.data[0] = last;
            let mut i = 0;
            loop {
                let (l, r) = (i * 2 + 1, i * 2 + 2);
                let mut largest = i;
                if l < self.data.len() && self.data[l] > self.data[largest] {
                    largest = l;
                }
                if r < self.data.len() && self.data[r] > self.data[largest] {
                    largest = r;
                }
                if largest == i {
                    break;
                }
                self.data.swap(i, largest);
                i = largest;
            }
        }
        top
    }
}

impl Solution {
    // ---------- 解法一：排序 ----------
    pub fn smallest_k_sort(arr: &[i32], k: usize) -> Vec<i32> {
        if k == 0 {
            return Vec::new();
        }
        let mut sorted = arr.to_vec();
        sorted.sort_unstable();
        sorted.truncate(k);
        sorted
    }

    // ---------- 解法二：大小为 k 的大根堆 ----------
    pub fn smallest_k_heap(arr: &[i32], k: usize) -> Vec<i32> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap = MaxHeap::new(k);
        for (i, &v) in arr.iter().enumerate() {
            if i < k {
                heap.push(v);
            } else if v < heap.peek() {
                heap.pop();
                heap.push(v);
            }
        }

        let mut res = Vec::with_capacity(heap.len());
        while heap.len() > 0 {
            res.push(heap.pop());
        }
        res
    }

    // ---------- 解法三：随机化快速选择（最优解） ----------
    /// 以 nums[r] 为主元的划分（Lomuto 分区），返回主元最终下标
    fn partition(nums: &mut [i32], l: usize, r: usize) -> usize {
        let pivot = nums[r];
        let mut i = l as isize - 1;
        for j in l..r {
            if nums[j] <= pivot {
                i += 1;
                nums.swap(i as usize, j);
            }
        }
        nums.swap((i + 1) as usize, r);
        (i + 1) as usize
    }

    /// 固定种子的线性同余发生器，模拟「随机选主元」且保证可复现
    fn next_rand(state: &mut u64) -> u64 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *state >> 33
    }

    fn randomized_selected(arr: &mut [i32], l: usize, r: usize, k: usize, state: &mut u64) {
        if l >= r {
            return;
        }

        // 随机选一个下标换到末尾，再划分
        let span = (r - l + 1) as u64;
        let pick = l + (Self::next_rand(state) % span) as usize;
        arr.swap(r, pick);

        let pos = Self::partition(arr, l, r);
        let num = pos - l + 1; // 左段（含 pos）的元素个数
        if k == num {
            return;
        }
        if k < num {
            Self::randomized_selected(arr, l, pos - 1, k, state);
        } else {
            Self::randomized_selected(arr, pos + 1, r, k - num, state);
        }
    }

    pub fn smallest_k(arr: &mut [i32], k: usize) -> Vec<i32> {
        if k == 0 {
            return Vec::new();
        }
        if k >= arr.len() {
            return arr.to_vec();
        }
        let mut state: u64 = 20261029;
        Self::randomized_selected(arr, 0, arr.len() - 1, k, &mut state);
        arr[..k].to_vec()
    }
}

fn check(arr: &[i32], k: usize) {
    let mut want = arr.to_vec();
    want.sort_unstable();
    want.truncate(k);

    let mut got_sort = Solution::smallest_k_sort(arr, k);
    let mut got_heap = Solution::smallest_k_heap(arr, k);
    let mut input = arr.to_vec();
    let mut got_quick = Solution::smallest_k(&mut input, k);

    got_sort.sort_unstable();
    got_heap.sort_unstable();
    got_quick.sort_unstable();

    assert_eq!(got_sort, want, "排序解({:?}, {})", arr, k);
    assert_eq!(got_heap, want, "堆解({:?}, {})", arr, k);
    assert_eq!(got_quick, want, "快速选择({:?}, {})", arr, k);
}

fn main() {
    // 示例
    check(&[1, 3, 5, 7, 2, 4, 6, 8], 4);

    // 边界：k = 0 返回空数组
    check(&[1, 2, 3], 0);

    // 边界：k = n 返回全部元素
    check(&[5, 1, 3], 3);

    // 边界：空数组
    check(&[], 0);

    // 边界：单元素
    check(&[7], 1);

    // 边界：全部相同
    check(&[2, 2, 2, 2, 2], 3);

    // 边界：负数
    check(&[-1, -5, 3, 0, -2], 2);

    // 边界：已经有序 / 逆序
    check(&[1, 2, 3, 4, 5, 6], 3);
    check(&[6, 5, 4, 3, 2, 1], 3);

    // 数据规模上限 n = 100000，k = 100000 与 k = 1000
    let big: Vec<i32> = (0..100000).map(|i| (i * 7919) % 100003).collect();
    check(&big, 100000);
    check(&big, 1000);

    println!("all tests passed");
}
