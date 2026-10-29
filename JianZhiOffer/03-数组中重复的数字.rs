/*
============================================================================
LCR 120 / 剑指 Offer 03. 数组中重复的数字（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof/
分类：数组 · 原地置换（下标即哈希）

说明：本题原名「数组中重复的数字」：长度为 n 的数组 nums，所有数字都在
0 ~ n-1 的范围内，请找出任意一个重复的数字。leetcode.cn 现改写成
「寻找文件副本」，函数名 findRepeatDocument、参数改称 documents，
判定动作完全相同。下面按当前平台题面与签名实现，并保留
findRepeatNumber 别名。

设备中存有 n 个文件，文件 id 记于数组 documents。若文件 id 相同，则
定义为该文件存在副本。请返回任一存在副本的文件 id。

示例 1：
输入：documents = [2, 5, 3, 0, 5, 0]
输出：0 或 5

提示：
- 0 <= documents[i] <= n-1
- 2 <= n <= 100000
*/
use std::collections::HashSet;

/*
解法一：哈希集合记录出现过的 id【非最优】

为什么慢：一边遍历一边用集合记下出现过的 id，第一次撞到重复就返回，
时间已经是 O(n)。真正的代价在空间：额外开一个最多装 n 个 id 的集合，
而题目已经把取值限制在 0 ~ n-1，这份信息其实可以还给数组本身。

换个角度：既然每个 id 都落在一个确定的下标范围内，数组下标就可以当作
哈希桶；哈希集合只是把这个映射又抄了一份。

关键点：
  · 先查集合再插入，或用 insert 的返回值判断，顺序反了会把第一次出现
    也当成重复
  · 集合里存 id 而不是下标，题目要找的是重复的 id
  · 只读 documents，不修改输入
  · 没找到时返回 -1 只是自测用的哨兵；题目保证至少有一个重复

手推一遍：
  以示例（documents = [2,5,3,0,5,0]）为例：
  依次加入 2、5、3、0；再遇到 5 时集合里已有 5 → 返回 5

复杂度分析：
  时间 O(n)：每个元素最多查一次、插一次
  空间 O(n)：最坏前 n-1 个 id 互不相同，全留在集合里
*/

// Rust 单文件 rustc 编译时平台预置的 Solution 不存在，这里显式定义
pub struct Solution1;

impl Solution1 {
    pub fn find_repeat_document(documents: Vec<i32>) -> i32 {
        let mut seen = HashSet::with_capacity(documents.len());
        for id in documents {
            // insert 返回 false 表示这个 id 之前已经出现过
            if !seen.insert(id) {
                return id;
            }
        }
        -1
    }

    // 原「剑指 Offer 03」接口名，语义完全一致
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        Self::find_repeat_document(nums)
    }
}

/*
解法二：排序后比较相邻元素【非最优】

为什么慢：排序把 O(n) 的扫描变成 O(n log n)；而且题目要求不能修改输入，
还得先把整个数组复制一份，空间也没省下来。

换个角度：重复的元素在有序数组里一定相邻。排序后只需看哪一对相邻元素
相等，第一个这样的元素就是答案之一。

关键点：
  · 必须排序副本，原地排序会改掉调用方的数组
  · 从下标 1 开始比较 documents[i] 与 documents[i-1]，不要漏掉第 0 个
  · 返回的是重复的值，不是下标
  · 排序不稳定不影响正确性：相等元素必然被排到一起

手推一遍：
  示例 [2,5,3,0,5,0] 排序后 [0,0,2,3,5,5]
  下标 1：0 == 0 → 返回 0

复杂度分析：
  时间 O(n log n)：排序占主导，扫描 O(n)
  空间 O(n)：排序用的副本；不计副本则为排序栈 O(log n)
*/
pub struct Solution2;

impl Solution2 {
    pub fn find_repeat_document(documents: Vec<i32>) -> i32 {
        let mut sorted = documents;
        sorted.sort_unstable();
        for i in 1..sorted.len() {
            // 有序后相等的值必然相邻
            if sorted[i] == sorted[i - 1] {
                return sorted[i];
            }
        }
        -1
    }

    // 原「剑指 Offer 03」接口名，语义完全一致
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        Self::find_repeat_document(nums)
    }
}

/*
解法三：原地置换（把 id 放回它对应的下标）【最优解】

为什么慢：解法一用集合额外记「谁出现过」，解法二用排序把相等元素凑到
一起；两者都没有利用「id ∈ [0, n-1]」这个强约束。

换个角度：把 documents 本身看成一个哈希表：下标 i 的「家」就是值为 i
的元素。反复把当前位置的元素换到它该去的下标，直到当前位置正好放着
自己的值。若某次要换过去的位置已经放着相同的值，就找到了重复。

关键点：
  · 交换而不是覆盖：覆盖会丢掉被挤走的值，后续就查不到它了
  · 每次都把 documents[i] 送回它的家，所以每个元素最多被换一次，
    总交换次数不超过 n，整体仍是 O(n)
  · 判定重复用 documents[documents[i]] == documents[i]；相等时目标位置
    已经被同一个 id 占住，这个 id 至少出现两次
  · 题目保证 0 <= id <= n-1，下标可以直接用，不需要越界判断
  · 这个版本会修改传入的数组；自测里必须传副本，否则会影响其他解法

手推一遍：
  以示例（documents = [2,5,3,0,5,0]）为例：
  i = 0：documents[0] = 2，家在下标 2，那里是 3 → 交换
        得 [3,5,2,0,5,0]
  i = 0：documents[0] = 3，家在下标 3，那里是 0 → 交换
        得 [0,5,2,3,5,0]
  i = 0：documents[0] = 0，归位
  i = 1：documents[1] = 5，家在下标 5，那里是 0 → 交换
        得 [0,0,2,3,5,5]
  i = 1：documents[1] = 0，家在下标 0，那里已经是 0
        → 0 重复，返回 0

复杂度分析：
  时间 O(n)：每个元素至多被换回家一次，外层 i 也只前进 n 步
  空间 O(1)：只在原数组上交换，没有额外容器
*/
pub struct Solution3;

impl Solution3 {
    pub fn find_repeat_document(documents: Vec<i32>) -> i32 {
        let mut documents = documents;
        let n = documents.len();
        for i in 0..n {
            // 一直换到 documents[i] 归位为止
            while documents[i] != i as i32 {
                let j = documents[i] as usize;
                // 目标下标 j 已经放着同一个 id，说明它出现了至少两次
                if documents[j] == documents[i] {
                    return documents[i];
                }
                documents.swap(i, j);
            }
        }
        -1
    }

    // 原「剑指 Offer 03」接口名，语义完全一致
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        Self::find_repeat_document(nums)
    }
}

// 每个解法各拿一份副本；原地置换版本改的是副本，不会影响其他解法
fn check(documents: &[i32], allowed: &[i32]) {
    let got = Solution1::find_repeat_document(documents.to_vec());
    assert!(allowed.contains(&got));

    let got = Solution2::find_repeat_document(documents.to_vec());
    assert!(allowed.contains(&got));

    // 原地置换会改数组，必须传副本
    let got = Solution3::find_repeat_document(documents.to_vec());
    assert!(allowed.contains(&got));

    // 原题接口名与当前接口名等价
    let got = Solution3::find_repeat_number(documents.to_vec());
    assert!(allowed.contains(&got));
}

fn main() {
    // 示例：0 和 5 都是合法答案
    check(&[2, 5, 3, 0, 5, 0], &[0, 5]);

    // 原「剑指 Offer 03」示例：2 和 3 都是合法答案
    check(&[2, 3, 1, 0, 2, 5, 3], &[2, 3]);

    // 边界：长度 2，唯一的一对重复
    check(&[1, 1], &[1]);
    check(&[0, 0], &[0]);

    // 边界：重复值恰好等于下标 0
    check(&[0, 1, 1], &[1]);
    check(&[0, 0, 0], &[0]);

    // 边界：所有元素都相同
    check(&[2, 2, 2], &[2]);

    // 边界：重复值在末尾
    check(&[0, 1, 2, 3, 4, 4], &[4]);

    // 边界：无重复（题目保证至少一个，这里只验证 -1 哨兵）
    check(&[0, 1, 2], &[-1]);

    // 边界：空数组（题目保证 n >= 2，这里只验证健壮性）
    check(&[], &[-1]);

    // 边界：规模 100000，重复的 0 藏在末尾
    let mut big: Vec<i32> = (0..99999).collect();
    big.push(0);
    check(&big, &[0]);

    println!("all tests passed");
}
