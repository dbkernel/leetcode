/*
============================================================================
215. 数组中的第 K 个最大元素（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/kth-largest-element-in-an-array/
分类：分治 · 快速选择（quickselect） / 堆

给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。

示例 1:
输入: [3,2,1,5,6,4], k = 2
输出: 5

示例 2:
输入: [3,2,3,1,2,4,5,5,6], k = 4
输出: 4

提示：
1 <= k <= nums.length <= 10^5
-10^4 <= nums[i] <= 10^4
*/

/*
解法一：快速选择（基于快速排序的划分）【最优解】

官方方法一，也是题目要求的 O(n) 解法。

为什么慢：先把整个数组排序再取倒数第 k 个，时间是 O(n log n)，且把「不必要的信息」
也算了出来——我们只要一个元素，不需要关心左右两边是否有序。

换个角度：快速排序的划分（partition）每做一次，就能把主元放到它的最终位置上，
并保证左边都不大于它、右边都不小于它。也就是说：一次划分确定一个元素的最终下标 q。
如果 q 正好是目标下标，就直接返回；如果目标在左边，就只递归左边；否则只递归右边。
每次问题规模期望减半，总代价 n + n/2 + n/4 + … = O(n)。

关键点：
  · 目标下标不是 k - 1，而是 n - k：第 k 大 = 升序数组里下标 n - k 的位置
  · 官方用的是「双指针 + 前后各扩一格」的划分写法（类似 Hoare 划分），
    不在循环内做随机化，实测对各类数据都比较稳
  · 划分循环 while i < j：i 从左跳过小于主元的，j 从右跳过大于主元的，
    都停下就交换；循环结束后 j 是左半段的右端点
  · 判断方向：k <= j 说明目标在左段及 j，下一步收缩到 (l, j)；否则收缩到 (j + 1, r)
  · 这里用循环替代官方的递归写法，逻辑完全等价，且不怕递归深度
  · 注意数组元素可能为负数，比较直接用 < / > 即可

以 nums = [3,2,1,5,6,4]，k = 2 为例：
  目标下标 = 6 - 2 = 4（升序 [1,2,3,4,5,6] 的下标 4 是 5）
  第一次划分取主元 3：左边得到 {2,1}，右侧留下 {5,6,4}
  目标下标 4 落在右段，只需继续处理右段
  右段继续划分最终把 5 放到下标 4，返回 5

复杂度分析：
  时间 O(n)（期望）：每次只处理一侧，规模期望减半
  空间 O(1)：原地划分（递归版为 O(log n) 栈空间）

解法二：建大根堆 + 依次删除堆顶【非最优】

官方方法二。先把数组建成大根堆（O(n)），然后做 k - 1 次「堆顶与末尾交换 + 堆大小减一
+ 下滤」，最后堆顶就是第 k 大。

关键点：
  · 建堆从 heapSize / 2 - 1 往前下滤（叶子节点天然是堆）
  · 删除 k - 1 次就够了：每删掉一个当前最大值，堆顶就变成下一个最大值
  · 循环上界 n - k + 1 正好做 k - 1 次；用 isize 避免下标下溢
  · 下滤时要先在左右孩子里挑较大的那个再交换

复杂度分析：
  时间 O(n + k log n)，最坏 O(n log n)
  空间 O(log n)：下滤递归栈

对比：堆解法稳定 O(n log n) 但不受「最坏划分」影响；
快速选择期望 O(n) 更快，是本题的最优解。
*/
struct Solution;

impl Solution {
    /// 解法一：快速选择（最优解）；官方为递归写法，这里改成等价循环
    pub fn find_kth_largest_quick_select(nums: &mut Vec<i32>, k: i32) -> i32 {
        let mut l: isize = 0;
        let mut r: isize = nums.len() as isize - 1;
        let k = (nums.len() as i32 - k) as isize; // 转成升序下的目标下标

        while l < r {
            let pivot = nums[l as usize];
            let mut i = l - 1;
            let mut j = r + 1;
            while i < j {
                // i 从左跳过小于主元的元素
                loop {
                    i += 1;
                    if nums[i as usize] >= pivot {
                        break;
                    }
                }
                // j 从右跳过大于主元的元素
                loop {
                    j -= 1;
                    if nums[j as usize] <= pivot {
                        break;
                    }
                }
                if i < j {
                    nums.swap(i as usize, j as usize);
                }
            }

            // 目标在左段（含 j）就只处理左边，否则只处理右边
            if k <= j {
                r = j;
            } else {
                l = j + 1;
            }
        }
        nums[k as usize]
    }

    // ---------- 解法二：大根堆 ----------
    fn max_heapify(a: &mut [i32], i: usize, heap_size: usize) {
        let (l, r) = (i * 2 + 1, i * 2 + 2);
        let mut largest = i;
        if l < heap_size && a[l] > a[largest] {
            largest = l;
        }
        if r < heap_size && a[r] > a[largest] {
            largest = r;
        }
        if largest != i {
            a.swap(i, largest);
            Solution::max_heapify(a, largest, heap_size);
        }
    }

    fn build_max_heap(a: &mut [i32], heap_size: usize) {
        if heap_size < 2 {
            return;
        }
        for i in (0..heap_size / 2).rev() {
            Solution::max_heapify(a, i, heap_size);
        }
    }

    /// 解法二：手写大根堆
    pub fn find_kth_largest_heap(nums: &mut Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut heap_size = n;
        Solution::build_max_heap(nums, heap_size);
        // 删除 k - 1 次堆顶，剩下的堆顶就是第 k 大
        for i in (n - k as usize + 1..n).rev() {
            nums.swap(0, i);
            heap_size -= 1;
            Solution::max_heapify(nums, 0, heap_size);
        }
        nums[0]
    }
}

fn check(nums: &[i32], k: i32, want: i32) {
    let mut a = nums.to_vec();
    let mut b = nums.to_vec();
    assert_eq!(
        Solution::find_kth_largest_quick_select(&mut a, k),
        want,
        "快速选择({:?}, {}) = {}, want {}",
        nums, k, want, want
    );
    assert_eq!(
        Solution::find_kth_largest_heap(&mut b, k),
        want,
        "堆({:?}, {}) = {}, want {}",
        nums, k, want, want
    );
}

fn main() {
    // 示例 1
    check(&[3, 2, 1, 5, 6, 4], 2, 5);

    // 示例 2：存在重复元素
    check(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4);

    // 边界：只有一个元素
    check(&[1], 1, 1);

    // 边界：k = 1（最大值）/ k = n（最小值）
    check(&[3, 2, 1, 5, 6, 4], 1, 6);
    check(&[3, 2, 1, 5, 6, 4], 6, 1);

    // 边界：全部相同
    check(&[2, 2, 2, 2], 3, 2);

    // 边界：负数与极值
    check(&[-1, -1], 2, -1);
    check(&[-10000, 10000, 0], 2, 0);
    check(&[10000, -10000], 1, 10000);

    // 边界：已经升序 / 降序
    check(&[1, 2, 3, 4, 5], 2, 4);
    check(&[5, 4, 3, 2, 1], 2, 4);

    // 边界：数据规模上限 10^5
    {
        let nums: Vec<i32> = (0..100000).map(|i| (i * 37) % 20001 - 10000).collect();
        let mut reference = nums.clone();
        reference.sort_unstable();
        for k in [1usize, 2, 500, 99999, 100000] {
            let want = reference[reference.len() - k];
            check(&nums, k as i32, want);
        }
    }

    // 交叉验证：小数组上对所有 k 逐一核对
    {
        let nums = vec![7, 1, 5, 3, 6, 4, 2, 8, 0, 9, 5];
        let mut reference = nums.clone();
        reference.sort_unstable();
        for k in 1..=nums.len() {
            let want = reference[reference.len() - k];
            check(&nums, k as i32, want);
        }
    }

    println!("all tests passed");
}
