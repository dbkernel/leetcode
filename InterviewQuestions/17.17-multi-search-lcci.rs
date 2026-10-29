/*
============================================================================
面试题 17.17. 多次搜索（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/multi-search-lcci/
分类：字符串 · 字典树（Trie）

给定一个较长字符串big和一个包含较短字符串的数组smalls，设计一个方法，根据
smalls中的每一个较短字符串，对big进行搜索。输出smalls中的字符串在big里出现
的所有位置positions，其中positions[i]为smalls[i]出现的所有位置。

示例：
输入：
big = "mississippi"
smalls = ["is","ppi","hi","sis","i","ssippi"]
输出： [[1,4],[8],[],[3],[1,4,7,10],[5]]

提示：
- 0 <= len(big) <= 1000
- 0 <= len(smalls[i]) <= 1000
- smalls 的总字符数不会超过 10^6
- 你可以认为 smalls 中没有重复字符串
- 所有出现的字符均为英文小写字母

说明：
- 本题为面试金典；官方题解的主线做法是把 smalls 建成 Trie，再从 big 的
  每个起点沿 Trie 匹配。下面先写暴力对照，再写 Trie。
- 空的 smalls[i] 按「不出现字符」处理，positions[i] 为空列表。
*/

// 本地单文件编译需要显式给出 Solution 类型
struct Solution1;
struct Solution2;

/*
解法一：逐词暴力匹配【非最优】

为什么慢：对 smalls 里的每个词，都要从 big 的每个起点重新比较一遍。设
smalls 有 k 个词、big 长 B、最长词长 L，最坏要 O(k * B * L) 次字符比较。
k 和 B 都不大时能过，但每个词都重复扫描同一段 big，信息没有复用。

换个角度：这题本质上是一次「多模式串匹配」。若把所有短词放进一棵字典树，
从 big 的每个起点出发只需要沿 Trie 往下走，多个词共享同一条前缀路径。

关键点（本解法）：
  · 空串不能作为模式匹配，否则每个起点都会命中；直接跳过
  · 短词比 big 还长时不可能出现，跳过以省掉无意义的比较
  · 用字节切片比较一次比 L 个字符，命中才记录起点
  · 结果 positions 的顺序必须与 smalls 的下标一一对应

以示例（big = "mississippi"）为例：
  "is" 在 1、4 命中；"ppi" 在 8 命中；"hi" 不命中；"sis" 在 3 命中；
  "i" 在 1、4、7、10 命中；"ssippi" 在 5 命中

复杂度分析：
  时间 O(k * B * L)：每个词最坏扫描 B 个起点、每次比较 L 个字符
  空间 O(1)：除返回值外只用常数个下标

解法二：字典树（Trie）【最优解】

为什么慢：解法一对每个词单独扫描 big，k 个词就有 k 次全串扫描；当短词共享
前缀（如 "is" 与 "iss"）时，这些前缀被反复比较。

换个角度：把所有短词插入 Trie，再从 big 的每个起点 i 出发沿 Trie 走。走到
某个终止节点，就说明以 i 开头的这段字符是某个短词，记下 i。一次扫描可以
同时发现所有以 i 开头的短词，共享前缀只走一次。

关键点：
  · 终止节点存 smalls 的下标而不是字符串，输出时直接按下标归位
  · 只有非空且长度不超过 big 的词才插入；否则永远不可能命中
  · 从每个起点出发，一旦 Trie 没有对应孩子就立刻停，不再看更长的后缀
  · 同一个起点可能连续命中多个词（如 "a" 与 "aa"），每个终止节点都要记录
  · 沿 big 走 Trie 时只前进不回溯，因此总时间是 O(B * L)，不是 O(B^2)

为什么不用 AC 自动机：
  AC 自动机把「每个起点重新走」变成「一次扫描 + fail 指针跳转」，一般情形
  是 O(S + B + 出现次数)。但本题 B <= 1000，Trie 的 O(B * L) 与 AC 的
  O(B + S) 只差一个不超过 1000 的常数；AC 还要建 fail 指针和输出链，
  四语言实现与调试成本明显更高。若 big 或 smalls 总长放大到 10^7 级别，
  AC 自动机的优势才会显现，那时应换用 AC。

以示例（big = "mississippi"）为例：
  把 6 个短词插入 Trie；从 i = 1 出发走 "is" 命中 "is" 与 "i"，
  从 i = 4 出发同样命中；"ppi" 只在 i = 8 命中，其余同理

复杂度分析：
  时间 O(S + B * L + occ)：S 为插入短词的总字符数，occ 为答案总数
  空间 O(S + occ)：Trie 节点数与短词总长同阶，occ 是输出本身
*/
impl Solution1 {
    pub fn multi_search(big: String, smalls: Vec<String>) -> Vec<Vec<i32>> {
        let bytes = big.as_bytes();
        let mut ans: Vec<Vec<i32>> = vec![Vec::new(); smalls.len()];

        for (idx, word) in smalls.iter().enumerate() {
            // 空串不产生位置；比 big 长的词不可能命中
            if word.is_empty() || word.len() > bytes.len() {
                continue;
            }
            let len = word.len();
            for start in 0..=bytes.len() - len {
                // 字节切片比较命中才记录，避免逐字符判断
                if &bytes[start..start + len] == word.as_bytes() {
                    ans[idx].push(start as i32);
                }
            }
        }
        ans
    }
}

// 字典树节点：next 存孩子下标，index 存对应 smalls 的下标（-1 表示非终止）
struct TrieNode {
    next: [i32; 26],
    index: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            next: [-1; 26],
            index: -1,
        }
    }
}

impl Solution2 {
    pub fn multi_search(big: String, smalls: Vec<String>) -> Vec<Vec<i32>> {
        let bytes = big.as_bytes();
        let mut ans: Vec<Vec<i32>> = vec![Vec::new(); smalls.len()];
        let mut nodes = vec![TrieNode::new()];

        for (idx, word) in smalls.iter().enumerate() {
            if word.is_empty() || word.len() > bytes.len() {
                continue;
            }
            let mut node = 0usize;
            for &ch in word.as_bytes() {
                let d = (ch - b'a') as usize;
                if nodes[node].next[d] == -1 {
                    nodes[node].next[d] = nodes.len() as i32;
                    nodes.push(TrieNode::new());
                }
                node = nodes[node].next[d] as usize;
            }
            nodes[node].index = idx as i32;
        }

        for start in 0..bytes.len() {
            let mut node = 0usize;
            for &ch in &bytes[start..] {
                let next = nodes[node].next[(ch - b'a') as usize];
                // 没有这个前缀，再往后走也不可能命中
                if next == -1 {
                    break;
                }
                node = next as usize;
                // 一个起点可能同时命中多个词，每个终止节点都要记录
                if nodes[node].index != -1 {
                    ans[nodes[node].index as usize].push(start as i32);
                }
            }
        }
        ans
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

fn check(big: &str, smalls: &[&str], want: &[Vec<i32>]) {
    let owned: Vec<String> = smalls.iter().map(|s| s.to_string()).collect();
    let got1 = Solution1::multi_search(big.to_string(), owned.clone());
    let got2 = Solution2::multi_search(big.to_string(), owned);
    assert_eq!(got1, want, "brute({big:?}, {smalls:?}) = {got1:?}");
    assert_eq!(got2, want, "trie({big:?}, {smalls:?}) = {got2:?}");
}

fn main() {
    // 示例
    check(
        "mississippi",
        &["is", "ppi", "hi", "sis", "i", "ssippi"],
        &[vec![1, 4], vec![8], vec![], vec![3], vec![1, 4, 7, 10], vec![5]],
    );

    // 边界：big 为空，任何非空短词都不出现
    check("", &["a"], &[vec![]]);

    // 边界：smalls 为空
    check("abc", &[], &[]);

    // 边界：短词里有空串，空串不算出现
    check("abc", &["", "b"], &[vec![], vec![1]]);

    // 边界：短词比 big 长
    check("ab", &["abc", "abcd"], &[vec![], vec![]]);

    // 边界：互相重叠的匹配，同一位置可命中多个词
    check(
        "aaaa",
        &["a", "aa", "aaa"],
        &[vec![0, 1, 2, 3], vec![0, 1, 2], vec![0, 1]],
    );

    // 边界：交替模式，验证起点不遗漏
    check(
        "ababab",
        &["ab", "ba", "aba"],
        &[vec![0, 2, 4], vec![1, 3], vec![0, 2]],
    );

    // 边界：完全无匹配
    check("mississippi", &["xyz"], &[vec![]]);

    // 边界：单字符 big 与单字符短词
    check("a", &["a", "b"], &[vec![0], vec![]]);

    println!("all tests passed");
}
