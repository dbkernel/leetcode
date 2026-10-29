/*
============================================================================
LCR 181 / 剑指 Offer 58 - I. 字符串中的单词反转（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/fan-zhuan-dan-ci-shun-xu-lcof/
分类：字符串 · 单词级反转（双指针 / 分割后倒序）

说明：本题原名「翻转单词顺序」，leetcode.cn 现已更名并改用
reverseMessage 作为函数名，题面意思不变。
============================================================================

你在与一位习惯从右往左阅读的朋友发消息，他发出的文字顺序都与正常相反但
单词内容正确，为了和他顺利交流你决定写一个转换程序，把他所发的消息
message 转换为正常语序。

注意：输入字符串 message 中可能会存在前导空格、尾随空格或者单词间的多个
空格。返回的结果字符串中，单词间应当仅用单个空格分隔，且不包含任何额外的
空格。

示例 1：
输入: message = "the sky is blue"
输出: "blue is sky the"

示例 2：
输入: message = "  hello world!  "
输出: "world! hello"
解释：输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能
包括。

示例 3：
输入: message = "a good   example"
输出: "example good a"
解释：如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。

提示：
0 <= message.length <= 10^4
message 中包含英文大小写字母、空格和数字

注意：
本题与主站 151 题相同：
https://leetcode.cn/problems/reverse-words-in-a-string/
*/

// 平台预置 Solution 类型；本题有两种官方解法，拆成 Solution1 / Solution2，
// 提交时把选用的那个改名为 Solution 即可。
struct Solution1;
struct Solution2;

/*
解法一：双指针（倒序遍历，逐个框出单词）【最优解】

官方两解中更推荐的一个（另一解依赖内置分割函数，官方明确注明「面试时不
建议使用」），因此本题把这一种标为最优解；两者复杂度同为 O(N)。

为什么慢：想「按空格切开再倒序」就得先把整串切成若干子串，除了结果字符串
之外还要额外装一份单词表；而且切分对连续空格、首尾空格都很敏感，稍不小心
就把空串当成单词。

换个角度：其实不需要先切分。从右往左扫，用两个下标框住一个单词：
i 向左找到该单词左边界的前一格，j 始终停在该单词的最后一个字符上，
于是 message[i + 1, j] 就是一个完整单词。收完这个单词后继续向左跳过
空格，再把 j 挪到下一个单词的末尾。

关键点：
  · 必须先把首尾空格去掉 —— 否则尾部空格会让第一轮就切出空单词
  · 收单词的区间是 [i + 1, j]，长度为 j - i
  · 两个内层循环都要带 i >= 0 守卫 —— 本题用 i64 维护下标，
    这样「走到 -1」能被真实表达，不会像 usize 那样回绕
  · 结果里的空格由「添加前先补一个分隔空格」控制，避免末尾多空格

Rust 的一处写法细节：
  · 逐字节切片用 as_bytes()，收下的每个单词都是 &[u8]，最后用
    std::str::from_utf8 还原（本题限定字母、数字、空格，不会失败）

以示例 3（message = "a good   example"）为例：
  i 从末尾左移，停在 'example' 前的空格上 → 收下 "example"
  跳过三个空格，j 移到 'good' 的 'd'
  i 继续左移，停在 'good' 前的空格 → 收下 "good"
  跳过空格，j 移到 'a'
  i 走到 -1 → 收下 "a"，循环结束 → "example good a"

复杂度分析：
  时间 O(N)：每个字符最多被访问两次（一次定位、一次拷贝）
  空间 O(N)：单词表 + 结果字符串
*/
impl Solution1 {
    pub fn reverse_message(message: String) -> String {
        let bytes = message.as_bytes();

        // 去掉首尾空格；全是空格时直接返回空串
        let mut begin = 0;
        while begin < bytes.len() && bytes[begin] == b' ' {
            begin += 1;
        }
        if begin == bytes.len() {
            return String::new();
        }
        let mut end = bytes.len() - 1;
        while end > begin && bytes[end] == b' ' {
            end -= 1;
        }
        let s = &bytes[begin..=end];

        let mut words: Vec<&[u8]> = Vec::new();
        let mut i = s.len() as i64 - 1;
        let mut j = i;

        while i >= 0 {
            // 向左找到本单词左边界的前一格
            while i >= 0 && s[i as usize] != b' ' {
                i -= 1;
            }
            // 单词位于 [i + 1, j]，右边界要写 j + 1
            words.push(&s[(i + 1) as usize..=(j as usize)]);
            // 跳过单词之间的空格，j 落到下一个单词的末尾
            while i >= 0 && s[i as usize] == b' ' {
                i -= 1;
            }
            j = i;
        }

        words
            .iter()
            .map(|w| std::str::from_utf8(w).expect("ascii only"))
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

/*
解法二：分割 + 倒序（借助内置切分函数）【非最优】

为什么慢：思路最短，但要把整串先切成单词表，额外占一份 O(N) 空间；而且
切分规则因语言而异 —— 有的把连续空格当一处分隔，有的会切出一串空串，
必须额外过滤。官方也注明这种写法「面试时不建议使用」。

换个角度：既然内部已经把单词分好了，只剩「倒着拼起来」这一步。
Rust 的 split_whitespace() 天然按任意长度空白切分且不产生空串，
正好省掉过滤。

关键点：
  · 用 split_whitespace() 而不是 split(' ') —— 后者会把连续空格切成空串
  · collect 到 Vec<&str> 后 reverse()，再 join(" ")
  · 不要在每个单词后手动补空格再 trim，join 已经处理好分隔

以示例 2（message = "  hello world!  "）为例：
  split_whitespace 得到 ["hello", "world!"]
  倒序拼成 "world! hello"

复杂度分析：
  时间 O(N)：切分一遍，拼接一遍
  空间 O(N)：单词表 + 结果字符串
*/
impl Solution2 {
    pub fn reverse_message(message: String) -> String {
        let mut words: Vec<&str> = message.split_whitespace().collect();
        words.reverse();
        words.join(" ")
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

// 一个用例喂给两种解法：双指针与分割倒序必须给出相同结果
fn check(message: &str, want: &str) {
    let got1 = Solution1::reverse_message(message.to_string());
    let got2 = Solution2::reverse_message(message.to_string());

    assert_eq!(got1, want, "reverse_message1({:?}) = {:?}", message, got1);
    assert_eq!(got2, want, "reverse_message2({:?}) = {:?}", message, got2);
}

fn main() {
    // 示例 1：单词之间只有一个空格
    check("the sky is blue", "blue is sky the");

    // 示例 2：首尾都有空格，验证先去掉再切
    check("  hello world!  ", "world! hello");

    // 示例 3：单词间有多个空格，结果只能保留单个空格
    check("a good   example", "example good a");

    // 边界：空串，必须提前返回，不能读 bytes[0]
    check("", "");

    // 边界：全是空格，去掉后为空，不能输出任何单词
    check("     ", "");

    // 边界：只有一个单词，i 会一路退到 -1
    check("  solo  ", "solo");

    println!("all tests passed");
}
