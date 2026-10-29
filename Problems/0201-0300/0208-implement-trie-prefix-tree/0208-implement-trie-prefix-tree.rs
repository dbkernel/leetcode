/*
============================================================================
208. 实现 Trie（前缀树）（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/implement-trie-prefix-tree/
分类：设计 · 前缀树（26 叉孩子数组）

Trie（发音类似 "try"）或者说 前缀树 是一种树形数据结构，用于高效地存储
和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补全
和拼写检查。

请你实现 Trie 类：
- Trie() 初始化前缀树对象。
- void insert(String word) 向前缀树中插入字符串 word 。
- boolean search(String word) 如果字符串 word 在前缀树中，返回 true
  （即，在检索之前已经插入）；否则，返回 false 。
- boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀
  之一为 prefix ，返回 true ；否则，返回 false 。

示例：
输入
["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
[[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
输出
[null, null, true, false, true, null, true]

解释
Trie trie = new Trie();
trie.insert("apple");
trie.search("apple");   // 返回 True
trie.search("app");     // 返回 False
trie.startsWith("app"); // 返回 True
trie.insert("app");
trie.search("app");     // 返回 True

提示：
- 1 <= word.length, prefix.length <= 2000
- word 和 prefix 仅由小写英文字母组成
- insert、search 和 startsWith 调用次数 总计 不超过 3 * 10^4 次
*/

/*
解法：孩子数组建树（每条边对应一个小写字母）【最优解】

接口契约：
  · insert 不要求 word 原本不在树中，重复插入同一个词必须幂等
  · search 要求整条路径存在，且终点带「词尾」标记
  · startsWith 只要求整条路径存在，不关心终点是否收尾
  · 三个操作都只读 word / prefix，不改动调用方传入的字符串

数据结构选型：
  · 每个节点存 26 个孩子指针，用 b - b'a' 直接定位，单步 O(1)
  · 孩子数组的内存是「节点数 × 26 个指针」，稀疏时浪费明显；换成
    HashMap 能省内存，但每次查找多一次哈希计算，常数更大
  · 本题字符集固定为 26 个小写字母、调用次数不超过 3 × 10^4，
    数组的常数优势更重要，因此选数组

关键点：
  · 路径上的节点可能在多次 insert 中共享，缺孩子时才新建，已有就复用
    —— 覆盖掉旧孩子会丢掉之前插入的词
  · is_end 只在 insert 走完整个 word 后置 true，中途不停留
  · search 与 starts_with 的唯一区别就是最后是否检查 is_end
  · 空串不会出现（长度至少 1），根节点本身不承载任何词
  · Rust 单文件 rustc 编译时平台预置的 Trie 不存在，这里显式定义；
    [Option<Box<Trie>>; 26] 用 std::array::from_fn 生成全 None

手推一遍：
  以示例序列为例：
  insert("apple") 建出 a → p → p → l → e，只在 e 上置 is_end
  search("apple") 走完整条路径，终点 is_end 为 true → true
  search("app") 路径存在，但停在 p 上，is_end 为 false → false
  starts_with("app") 只查路径 → true
  insert("app") 复用 a → p → p，只把第二个 p 的 is_end 置 true
  search("app") 终点 is_end 已为 true → true

复杂度分析：
  时间 O(L)：每次操作只沿字符串长度 L 走一遍，每步 O(1)
  空间 O(总字符数 × 26)：每个新建节点固定 26 个指针，
    最坏是所有词两两没有公共前缀
*/

pub struct Trie {
    child: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            child: std::array::from_fn(|_| None),
            is_end: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            // 共享已有前缀：只有缺孩子时才新建
            node = node.child[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        // 只有完整插入过的词尾才置 true
        node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let node = self.find(&word);
        // 路径存在不代表是一个词，必须曾经在这里收尾
        node.map_or(false, |n| n.is_end)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        // 前缀只要求路径存在，不要求收尾标记
        self.find(&prefix).is_some()
    }

    // 沿 s 往下走，返回终点；中途缺孩子就说明路径不存在
    fn find(&self, s: &str) -> Option<&Trie> {
        let mut node = self;
        for b in s.bytes() {
            node = node.child[(b - b'a') as usize].as_deref()?;
        }
        Some(node)
    }
}

// 按题目的调用序列回放；序列开头的 "Trie" 对应这里的一次 new
fn replay(calls: &[(&str, &str, &str)]) {
    let mut trie = Trie::new();
    for &(op, arg, want) in calls {
        match op {
            "insert" => {
                trie.insert(arg.to_string());
                assert_eq!(want, "null");
            }
            "search" => {
                let got = if trie.search(arg.to_string()) {
                    "true"
                } else {
                    "false"
                };
                assert_eq!(got, want, "search({})", arg);
            }
            "startsWith" => {
                let got = if trie.starts_with(arg.to_string()) {
                    "true"
                } else {
                    "false"
                };
                assert_eq!(got, want, "startsWith({})", arg);
            }
            _ => panic!("未知操作 {}", op),
        }
    }
}

fn main() {
    // 示例：按题目的调用序列逐条回放
    replay(&[
        ("insert", "apple", "null"),
        ("search", "apple", "true"),
        ("search", "app", "false"),
        ("startsWith", "app", "true"),
        ("insert", "app", "null"),
        ("search", "app", "true"),
    ]);

    // 边界：重复插入同一个词不改变语义
    replay(&[
        ("insert", "abc", "null"),
        ("insert", "abc", "null"),
        ("search", "abc", "true"),
        ("search", "ab", "false"),
        ("startsWith", "ab", "true"),
    ]);

    // 边界：查询从未插入过的首字母，路径第一步就断
    replay(&[
        ("insert", "a", "null"),
        ("search", "b", "false"),
        ("startsWith", "b", "false"),
    ]);

    // 边界：前一个词是后一个词的前缀，词尾标记不能互相污染
    replay(&[
        ("insert", "app", "null"),
        ("insert", "apple", "null"),
        ("search", "app", "true"),
        ("search", "appl", "false"),
        ("startsWith", "appl", "true"),
        ("search", "apples", "false"),
    ]);

    // 边界：26 个首字母各插一个单字符词，验证下标换算覆盖整个字母表
    {
        let mut trie = Trie::new();
        for b in b'a'..=b'z' {
            trie.insert((b as char).to_string());
        }
        for b in b'a'..=b'z' {
            let s = (b as char).to_string();
            assert!(trie.search(s.clone()), "{}", s);
            assert!(trie.starts_with(s), "{}", b);
        }
        // 空串不是任何已插入的词（根节点没有词尾标记）
        assert!(!trie.search(String::new()));
    }

    // 边界：长度 2000 的词，验证长路径上的补建与查找都没有越界
    {
        let long_word = "x".repeat(2000);
        let prefix = "x".repeat(1999);
        let mut trie = Trie::new();
        trie.insert(long_word.clone());
        assert!(trie.search(long_word));
        assert!(trie.starts_with(prefix.clone()));
        assert!(!trie.search(prefix));
    }

    println!("all tests passed");
}
