#![allow(dead_code)]
/*
============================================================================
字典树（Trie / 前缀树）
分类：手写数据结构 · 树
操作复杂度：
  insert       O(L)   L 为单词长度，沿字符一路向下建节点
  search       O(L)   整词：走完 L 步后看 end > 0
  starts_with  O(L)   前缀：走完 L 步只要节点存在即可
  count_prefix O(L)   返回该节点被多少单词经过
  count_word   O(L)
  erase        O(L)   沿路径 pass 减一，可选自底向上剪枝
  空间 O(总字符数 * 26)；共享前缀的单词越多越省
============================================================================

用途：
  前缀匹配、自动补全、敏感词过滤、单词搜索题的加速结构。
  面试常考：实现 Trie、前缀计数、支持重复插入与删除。

核心思想：
  把字符串按字符拆成一条路径，根到某节点的路径就是一个前缀。每个
  节点用 26 个槽位指向子节点，节点上记两个计数：pass 表示有多少个
  单词经过这里，end 表示有多少个单词在这里结束。查询时沿路径走，
  走不动就说明没这个前缀。

关键点（不变量）：
  · 用计数而不是 bool：同一个单词插入两次，end 就是 2；删除一次只
    减一，还能查到。用 bool 就无法区分「插入过一次」和「插入过三次」，
    删一次就没了，重复插入的信息直接丢失；
  · pass 与 end 的含义不能混：search 看 end，starts_with 看 pass
    （节点存在即 pass > 0），否则 "app" 会被误判成 "apple" 的整词；
  · 删除必须先确认 count_word > 0，否则会把不存在的路径减成负数；
  · 剪枝只能自底向上，且一旦某层 pass > 0 就必须停：上层还有别的
    单词经过，继续剪会把它们一起删掉。

手推一遍：
  依次插入 app(1 次)、apple(2 次)：
  1. 路径 a-p-p 上每个节点 pass 都加 1，最后把 p 的 end 加 1；
  2. 再插两次 apple：a、p、p、l、e 的 pass 各加 2，e 的 end 变 2；
     此时 root.pass=3、app 节点 pass=3/end=1、e 节点 pass=2/end=2；
  3. count_prefix("app") 返回 3，count_prefix("appl") 返回 2，
     search("app") 为 true，search("appl") 为 false；
  4. erase("app") 只把路径上 pass 各减一、end 减到 0，因为两个
     apple 还经过这层，pass 仍是 2，不触发剪枝。

复杂度分析：
  时间 O(L)：每次操作只走单词长度条边，与树里已有多少单词无关。
  空间 O(总字符数 * 26)：26 叉数组是定长开销，前缀共享能显著省空间。

Rust 实现选择：
  索引数组 + free 列表：子节点存 i32 下标（-1 表示空），剪枝时把
  节点下标压回 free 复用。26 叉数组放进数组里不需要 Box，也没有
  递归 Drop 的问题，写起来比树状 Box 干净。
*/

const NONE: i32 = -1;

struct TrieNode {
    child: [i32; 26], // -1 表示没有这条边
    pass: i32,        // 有多少个单词经过这个节点
    end: i32,         // 有多少个单词在这个节点结束
}

struct Trie {
    nodes: Vec<TrieNode>,
    free: Vec<usize>,
    root: usize,
    words: i32, // 单词总数，含重复
}

impl Trie {
    fn new() -> Self {
        let root = TrieNode { child: [NONE; 26], pass: 0, end: 0 };
        Trie { nodes: vec![root], free: Vec::new(), root: 0, words: 0 }
    }

    fn new_node(&mut self) -> usize {
        let n = TrieNode { child: [NONE; 26], pass: 0, end: 0 };
        if let Some(id) = self.free.pop() {
            self.nodes[id] = n;
            id
        } else {
            self.nodes.push(n);
            self.nodes.len() - 1
        }
    }

    fn insert(&mut self, w: &str) -> bool {
        if !valid(w) {
            return false;
        }
        let mut cur = self.root;
        self.nodes[cur].pass += 1; // 根节点也要记：空前缀的计数靠它
        for &ch in w.as_bytes() {
            let c = (ch - b'a') as usize;
            let nxt = self.nodes[cur].child[c];
            let id = if nxt == NONE {
                let id = self.new_node();
                self.nodes[cur].child[c] = id as i32;
                id
            } else {
                nxt as usize
            };
            cur = id;
            self.nodes[cur].pass += 1;
        }
        self.nodes[cur].end += 1;
        self.words += 1;
        true
    }

    // 整词查询：必须走完且该节点有单词结束
    fn search(&self, w: &str) -> bool {
        if !valid(w) {
            return false;
        }
        match self.walk(w) {
            Some(t) => self.nodes[t].end > 0,
            None => false,
        }
    }

    // 前缀查询：只要路径走得通就算存在
    fn starts_with(&self, p: &str) -> bool {
        if p.is_empty() {
            return self.words > 0;
        }
        if !valid(p) {
            return false;
        }
        self.walk(p).is_some()
    }

    // 有多少个单词以 p 为前缀；空前缀返回单词总数
    fn count_prefix(&self, p: &str) -> i32 {
        if p.is_empty() {
            return self.words;
        }
        if !valid(p) {
            return 0;
        }
        match self.walk(p) {
            Some(t) => self.nodes[t].pass,
            None => 0,
        }
    }

    // 该词被插入了几次
    fn count_word(&self, w: &str) -> i32 {
        if !valid(w) {
            return 0;
        }
        match self.walk(w) {
            Some(t) => self.nodes[t].end,
            None => 0,
        }
    }

    // 删除一次出现；没插过则返回 false
    fn erase(&mut self, w: &str) -> bool {
        if !valid(w) || self.count_word(w) == 0 {
            return false;
        }
        let b = w.as_bytes();
        // 记下整条路径，方便自底向上剪枝
        let mut path = vec![self.root];
        let mut letters = Vec::new();
        let mut cur = self.root;
        self.nodes[cur].pass -= 1;
        for &ch in b {
            let c = (ch - b'a') as usize;
            let nxt = self.nodes[cur].child[c] as usize;
            self.nodes[nxt].pass -= 1;
            path.push(nxt);
            letters.push(c);
            cur = nxt;
        }
        self.nodes[cur].end -= 1;
        self.words -= 1;
        for i in (0..b.len()).rev() {
            if self.nodes[path[i + 1]].pass == 0 {
                // 没有单词再经过这个节点了，剪掉它并复用槽位
                let dead = path[i + 1];
                let parent = path[i];
                self.nodes[parent].child[letters[i]] = NONE;
                self.free.push(dead);
            } else {
                break; // 上层还有别的单词，继续剪会误伤
            }
        }
        true
    }

    fn walk(&self, w: &str) -> Option<usize> {
        let mut cur = self.root;
        for &ch in w.as_bytes() {
            let c = (ch - b'a') as usize;
            let nxt = self.nodes[cur].child[c];
            if nxt == NONE {
                return None;
            }
            cur = nxt as usize;
        }
        Some(cur)
    }
}

// 只接受非空的全小写单词
fn valid(w: &str) -> bool {
    if w.is_empty() {
        return false;
    }
    for &ch in w.as_bytes() {
        if ch < b'a' || ch > b'z' {
            return false;
        }
    }
    true
}

fn main() {
    // 1. 插入 / 整词查询 / 前缀查询 / 前缀计数
    {
        let mut t = Trie::new();
        assert!(t.insert("apple"));
        assert!(t.insert("app"));
        assert!(t.insert("apply"));
        assert!(t.search("apple"));
        assert!(t.search("app"));
        assert!(t.starts_with("app"));
        assert!(t.starts_with("appl"));
        assert_eq!(t.count_prefix("app"), 3);
        assert_eq!(t.count_prefix("appl"), 2);
        assert_eq!(t.count_prefix("a"), 3);
        assert_eq!(t.count_word("app"), 1);
        // 前缀存在但整个词不存在
        assert!(!t.search("appl"));
        assert!(!t.search("ap"));
        assert!(!t.starts_with("b"));
        println!("prefix app count: {}", t.count_prefix("app"));
    }

    // 2. 重复插入：计数累加，search 只看 end > 0
    {
        let mut t = Trie::new();
        assert!(t.insert("abc"));
        assert!(t.insert("abc"));
        assert!(t.insert("abc"));
        assert_eq!(t.count_word("abc"), 3);
        assert_eq!(t.count_prefix("ab"), 3);
        assert_eq!(t.count_prefix("abc"), 3);
        assert!(t.search("abc"));
        // 删一次还有两次
        assert!(t.erase("abc"));
        assert_eq!(t.count_word("abc"), 2);
        assert!(t.search("abc"));
        assert_eq!(t.count_prefix("ab"), 2);
        assert!(t.erase("abc"));
        assert!(t.erase("abc"));
        assert!(!t.search("abc"));
        assert!(!t.starts_with("ab")); // 剪枝后路径不存在
        assert_eq!(t.count_prefix("ab"), 0);
        assert_eq!(t.count_word("abc"), 0);
        // 删光了再删一次必须失败
        assert!(!t.erase("abc"));
    }

    // 3. 空前缀：存在的口径是「树里还有单词」
    {
        let mut t = Trie::new();
        assert!(!t.starts_with(""));
        assert_eq!(t.count_prefix(""), 0);
        assert!(!t.search(""));
        t.insert("x");
        assert!(t.starts_with(""));
        assert_eq!(t.count_prefix(""), 1);
        assert!(!t.search("")); // 没插入过空串
        t.erase("x");
        assert!(!t.starts_with(""));
        assert_eq!(t.count_prefix(""), 0);
    }

    // 4. 非小写字母：一律拒绝
    {
        let mut t = Trie::new();
        assert!(!t.insert("Abc"));
        assert!(!t.insert("aBc"));
        assert!(!t.insert("ab1"));
        assert!(!t.insert(""));
        assert!(!t.search("Abc"));
        assert!(!t.starts_with("aB"));
        assert_eq!(t.count_prefix("a1"), 0);
        assert!(!t.erase("a b"));
        assert_eq!(t.count_prefix(""), 0);
    }

    // 5. 空树删除 / 删除不存在的词
    {
        let mut t = Trie::new();
        assert!(!t.erase("a"));
        assert!(!t.search("a"));
        assert!(t.insert("ab"));
        assert!(!t.erase("a")); // 只是前缀，不是单词
        assert!(t.search("ab")); // 不能误删
        assert!(!t.erase("abc"));
        assert_eq!(t.count_word("ab"), 1);
    }

    // 6. 剪枝后仍可复用同一层
    {
        let mut t = Trie::new();
        assert!(t.insert("ab"));
        assert!(t.erase("ab"));
        assert!(!t.starts_with("a"));
        assert!(t.insert("ac")); // 重新建 a 这一层
        assert!(t.search("ac"));
        assert!(!t.search("ab"));
        assert_eq!(t.count_prefix("a"), 1);
    }

    println!("all tests passed");
}
