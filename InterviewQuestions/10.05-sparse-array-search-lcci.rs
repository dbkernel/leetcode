/*
============================================================================
面试题 10.05. 稀疏数组搜索（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sparse-array-search-lcci/
分类：二分查找 · 稀疏数组 / 空串跳过

稀疏数组搜索。有个排好序的字符串数组，其中散布着一些空字符串，
编写一种方法，找出给定字符串的位置。

示例1:
 输入: words = ["at", "", "", "", "ball", "", "", "car", "", "", "dad", "", ""], s = "ta"
 输出：-1
 说明: 不存在返回-1。

示例2:
 输入：words = ["at", "", "", "", "ball", "", "", "car", "", "", "dad", "", ""], s = "ball"
 输出：4

提示:
- words的长度在[1, 1000000]之间
*/

/*
解法：二分查找，中点为空时向右探测或收缩边界【最优解】

为什么慢：因为空串散布在数组里，「取中点直接比较」经常拿到空串，空串不
携带任何大小信息，无法判断该往哪一半收缩；退回从头线性扫描又丢掉了有序
这个前提，O(n)。

换个角度：空串虽然不能比较，但它占着位置。中点为空时就向右找最近的实词
来充当比较对象；如果一路到右边界都是空串，说明 [mid, right] 内根本没有
候选，答案只可能在左半边。

关键点：
  · 先令 probe = mid，向右推进到第一个非空位置或越过 right
  · probe > right 时不能判 -1，只能令 right = mid - 1 继续找左半边
  · 比较和返回都用 words[probe]，不要误用 words[mid]
  · words[probe] < s 时 left = probe + 1，probe 及其左边都不可能命中
  · words[probe] > s 时 right = mid - 1，mid 及其右侧被排除
  · 字符串按字典序比较，与题目「排好序」的定义一致

以示例 2（s = "ball"）为例：
  left=0, right=12 → mid=6 为空 → probe 推到 7，words[7]="car"
  "car" > "ball" → right = 6
  left=0, right=6 → mid=3 为空 → probe 推到 4，words[4]="ball"
  相等 → 返回 4 ✔

以示例 1（s = "ta"）为例：
  最终 left 越过 right，循环结束返回 -1 ✔

复杂度分析：
  时间 O(n)：最坏情况下每层探测都扫过大量空串；空串很少时接近 O(log n)
  空间 O(1)：只用了 left / right / probe 三个下标
*/

// rustc 单文件编译时没有平台预置的 Solution，这里显式定义
pub struct Solution;

impl Solution {
    pub fn find_string(words: Vec<String>, s: String) -> i32 {
        let mut left: i64 = 0;
        let mut right: i64 = words.len() as i64 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            // 中点为空串时无法比较，向右找最近的实词
            let mut probe = mid;
            while probe <= right && words[probe as usize].is_empty() {
                probe += 1;
            }

            // [mid, right] 全是空串，候选只在左半边
            if probe > right {
                right = mid - 1;
                continue;
            }

            let word = words[probe as usize].as_str();
            if word == s.as_str() {
                return probe as i32;
            }
            // probe 左侧不含候选，连 probe 一起跳过
            if word < s.as_str() {
                left = probe + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

fn check(words: &[&str], s: &str, want: i32) {
    let owned: Vec<String> = words.iter().map(|w| w.to_string()).collect();
    let got = Solution::find_string(owned, s.to_string());
    assert_eq!(got, want, "words = {:?}, s = {:?}", words, s);
}

fn main() {
    let sparse = [
        "at", "", "", "", "ball", "", "", "car", "", "", "dad", "", "",
    ];

    // 示例 1：不存在
    check(&sparse, "ta", -1);

    // 示例 2：命中
    check(&sparse, "ball", 4);

    // 边界：命中首尾两个实词
    check(&sparse, "at", 0);
    check(&sparse, "dad", 10);

    // 边界：命中中间的实词
    check(&sparse, "car", 7);

    // 边界：比所有单词都小 / 都大
    check(&sparse, "aa", -1);
    check(&sparse, "zzz", -1);

    // 边界：单元素
    check(&["a"], "a", 0);
    check(&["a"], "b", -1);

    // 边界：全是空串
    check(&["", "", ""], "a", -1);
    check(&["", "", ""], "", -1);

    // 边界：中点连续为空，需要向右探测到实词
    check(&["a", "", "b"], "b", 2);
    check(&["", "a"], "a", 1);
    check(&["a", "", ""], "a", 0);

    // 边界：实词全在右侧，中点探测必须能越过大量空串
    check(&["", "", "", "", "z"], "z", 4);

    // 规模：10^5 个位置，只有首尾是实词
    let mut words = vec![String::new(); 100000];
    words[0] = "a".to_string();
    words[99999] = "z".to_string();
    let words: Vec<&str> = words.iter().map(|w| w.as_str()).collect();
    check(&words, "a", 0);
    check(&words, "z", 99999);
    check(&words, "m", -1);

    println!("all tests passed");
}
