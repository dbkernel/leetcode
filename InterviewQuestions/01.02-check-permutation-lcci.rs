/*
============================================================================
面试题 01.02. 判定是否互为字符重排（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/check-permutation-lcci/
分类：哈希表 · 字符频次统计

给定两个由小写字母组成的字符串 s1 和 s2，请编写一个程序，确定其中一个
字符串的字符重新排列后，能否变成另一个字符串。

示例 1：
输入: s1 = "abc", s2 = "bca"
输出: true

示例 2：
输入: s1 = "abc", s2 = "bad"
输出: false

说明：
- 0 <= len(s1) <= 100
- 0 <= len(s2) <= 100
*/

/*
解法一：排序后比较【非最优】

为什么慢：互为字符重排等价于"两个字符串排序后相等"，思路最直白。
但排序每个字符串要 O(n log n)，而判断重排本质上只需要知道每种字符
出现了几次，不需要知道它们的顺序。

换个角度：把"顺序"这个无关信息直接丢掉，只比较字符频次。排序是丢掉
顺序的一种方式，但代价偏高；频次数组可以线性完成。

关键点：
  · 先比较长度，长度不同一定不是重排，直接返回 false
  · 转成 Vec<u8> 后原地排序，不修改调用方传入的 String
  · 排序后逐字符相等即可，不要只比较字符集合
  · 长度相同且频次相同才是充要条件

以示例 1（s1 = "abc", s2 = "bca"）为例：
  排序后都是 "abc"，相等 → true ✔
以示例 2（s1 = "abc", s2 = "bad"）为例：
  排序后 "abc" 与 "abd"，不相等 → false ✔

复杂度分析：
  时间 O(n log n)：两次排序
  空间 O(n)：两个字节向量副本

解法二：频次数组（一次遍历）【最优解】

用长度为 256 的数组记录频次：先遍历 s1 累加，再遍历 s2 递减。
一旦某个字符的频次变成负数，说明 s2 多用了这个字符，立即返回 false。
长度相等时，总和为 0，因此不需要在最后再扫一遍数组确认全零。

关键点：
  · 先比较长度，避免 s2 比 s1 长时出现"多出的字符恰好抵消"的误判
  · Rust 的 u8 范围是 0..255，数组开 256 就不会越界
  · 递减后立刻检查 < 0，能提前发现不匹配
  · 长度相等时"没有负数"就等价于所有频次恰好为 0

以示例 1（s1 = "abc", s2 = "bca"）为例：
  table[a]=1,b=1,c=1；s2 依次减到全 0，没有负数 → true ✔
以示例 2（s1 = "abc", s2 = "bad"）为例：
  s2 处理到 'd' 时 table['d'] 从 0 减到 -1 → false ✔

复杂度分析：
  时间 O(n)：两次线性遍历
  空间 O(1)：字符集大小固定为 256
*/

// 单文件 rustc 编译时平台预置的 Solution 不存在，以下类型为本地编译所需
// ---------- 解法一：排序 ----------
pub struct Solution1;

impl Solution1
{
    pub fn check_permutation(s1: String, s2: String) -> bool
    {
        if s1.len() != s2.len() {
            return false;
        }
        let mut a = s1.into_bytes();
        let mut b = s2.into_bytes();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    }
}

// ---------- 解法二：频次数组（最优解） ----------
pub struct Solution2;

impl Solution2
{
    pub fn check_permutation(s1: String, s2: String) -> bool
    {
        if s1.len() != s2.len() {
            return false;
        }

        let mut table = [0i32; 256];
        for b in s1.bytes() {
            table[b as usize] += 1;
        }
        for b in s2.bytes() {
            // 先减再判负：s2 用到了 s1 没有的字符时立刻失败
            let idx = b as usize;
            table[idx] -= 1;
            if table[idx] < 0 {
                return false;
            }
        }
        // 长度相等时频次总和为 0，没有负数就意味着全为 0
        true
    }
}

fn check(s1: &str, s2: &str, want: bool)
{
    let got1 = Solution1::check_permutation(s1.to_string(), s2.to_string());
    assert_eq!(got1, want, "sort({:?},{:?})", s1, s2);

    let got2 = Solution2::check_permutation(s1.to_string(), s2.to_string());
    assert_eq!(got2, want, "count({:?},{:?})", s1, s2);
}

fn main()
{
    // 示例 1
    check("abc", "bca", true);

    // 示例 2
    check("abc", "bad", false);

    // 边界：两个空串互为重排
    check("", "", true);

    // 边界：长度不同，即使前缀相同也不是重排
    check("a", "", false);
    check("abc", "ab", false);

    // 边界：单字符
    check("a", "a", true);
    check("a", "b", false);

    // 反直觉：字符集合相同但频次不同
    check("aab", "abb", false);

    // 反直觉：顺序完全不同，但频次一致
    check("aabbcc", "ccbbaa", true);
    check("abcdefghij", "jihgfedcba", true);

    // 边界：100 个字符的极限长度
    let same = "a".repeat(100);
    check(&same, &same, true);
    let almost = format!("{}b", "a".repeat(99));
    check(&almost, &same, false);

    println!("all tests passed");
}
