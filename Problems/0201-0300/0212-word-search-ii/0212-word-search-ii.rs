/*
============================================================================
212. 单词搜索 II（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/word-search-ii/
分类：字典树 · 回溯 / 剪枝

给定一个 m x n 二维字符网格 board 和一个单词（字符串）列表 words， 返回
所有二维网格上的单词 。

单词必须按照字母顺序，通过 相邻的单元格 内的字母构成，其中“相邻”单元格是
那些水平相邻或垂直相邻的单元格。同一个单元格内的字母在一个单词中不允许被
重复使用。

示例 1：
输入：board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],
["i","f","l","v"]], words = ["oath","pea","eat","rain"]
输出：["eat","oath"]

示例 2：
输入：board = [["a","b"],["c","d"]], words = ["abcb"]
输出：[]

提示：
- m == board.length
- n == board[i].length
- 1 <= m, n <= 12
- board[i][j] 是一个小写英文字母
- 1 <= words.length <= 3 * 10^4
- 1 <= words[i].length <= 10
- words[i] 由小写英文字母组成
- words 中的所有字符串互不相同
*/
use std::collections::BTreeSet;

// 本地单文件编译需要显式给出 Solution 类型
struct Solution1;
struct Solution2;

// 字典树节点：child 存 26 个孩子，word 非空表示这里是一个单词的结尾
struct TrieNode {
    child: [Option<Box<TrieNode>>; 26],
    word: String,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            child: Default::default(),
            word: String::new(),
        }
    }

    fn has_child(&self) -> bool {
        self.child.iter().any(|p| p.is_some())
    }
}

fn insert_word(root: &mut TrieNode, word: &str) {
    let mut node = root;
    for byte in word.bytes() {
        let d = (byte - b'a') as usize;
        if node.child[d].is_none() {
            node.child[d] = Some(Box::new(TrieNode::new()));
        }
        node = node.child[d].as_mut().unwrap();
    }
    node.word = word.to_string();
}

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

/*
解法一：字典树 + 回溯（结果用集合去重）【最优解】

为什么慢：如果对 words 里的每个词单独在网格里做一次 DFS，最坏要
O(k * m * n * 3^l)，k 是词数。大量词共享前缀时，这些前缀路径被反复搜索。

换个角度：把所有词插入 Trie，再从每个格子出发做一次 DFS。DFS 的每一步都
沿着 Trie 的孩子走，走不通就剪枝；一条路径同时覆盖所有以它为前缀的词，
前缀只需要展开一次。

关键点：
  · 进入格子前先看 Trie 有没有对应孩子，没有就直接返回，避免无谓递归
  · 同一个单词可能由多条不同路径拼出（如多个 "a"），所以结果用集合去重
  · 访问过的格子临时改成 '#'，回溯时恢复原字母，防止单词内重复用格
  · 网格里只有小写字母，'#' 不会和合法字符冲突
  · 递归边界靠坐标判断，进入 DFS 的格子保证在网格内

以示例 1 为例：
  从 (1,0) 的 'e' 出发，沿 Trie 走 e→a→t 命中 "eat"
  从 (0,0) 的 'o' 出发，走 o→a→t→h 命中 "oath"
  "pea" 虽在 Trie 里，但网格中从 p 出发走不出 "pea"，被剪掉

复杂度分析：
  时间 O(m * n * 3^(l-1))：每个格子起步，之后每步最多 3 个方向（不走回头
  路），l 为最长单词长度；Trie 查询是 O(1) 每步
  空间 O(k * l)：Trie 节点总数与所有单词总长同阶；递归栈 O(l)

解法二：字典树 + 回溯 + 匹配后删除单词【最优解，与解法一并列】

为什么慢：解法一在全是 'a' 的网格上会把 "a"、"aa"、"aaa" 等大量重复路径
搜很多遍，只靠集合去重并不能减少搜索量。

换个角度：一个单词一旦被找到，就从 Trie 里把它的终止标记清空；如果某个
节点既没有单词也没有孩子，就把它从父节点上摘掉。这样后续 DFS 走到这条
分支会立刻被剪掉，每个单词最多被完整匹配一次。

关键点：
  · 命中后用 mem::take 取走 word，既收集了结果又清空终止标记
  · 清空后如果节点还有孩子（如 "ab" 是 "abc" 的前缀），不能删节点
  · Rust 的 Box 所有权唯一：先把 child[d].take() 出来，递归结束后再决定
    挂回去还是直接 drop，避免「持有子节点可变借用时又改父指针」的冲突
  · 因为每个词只命中一次，结果不需要集合去重，顺序也不要求
  · 摘节点靠 drop 自动释放整棵子树，不需要手动 delete

以示例 1 为例：
  命中 "eat" 后清空终止标记；"oath" 同理
  若输入是 ["a","aa","aaa"] 且网格全为 'a'，命中 "a" 后不再重复收集，
  命中 "aaa" 后其叶节点被摘掉，下一条路径到 "aa" 就停

复杂度分析：
  时间 O(m * n * 3^(l-1))：上界与解法一相同，但已匹配的分支被剪掉，
  重复词多的数据上实际搜索量显著下降
  空间 O(k * l)：Trie 节点总数与所有单词总长同阶；递归栈 O(l)
*/
impl Solution1 {
    pub fn find_words(
        board: Vec<Vec<char>>,
        words: Vec<String>,
    ) -> Vec<String> {
        let mut root = TrieNode::new();
        for word in &words {
            insert_word(&mut root, word);
        }

        let mut board = board;
        let mut found = BTreeSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::dfs(&mut board, i, j, &mut root, &mut found);
            }
        }
        found.into_iter().collect()
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        node: &mut TrieNode,
        found: &mut BTreeSet<String>,
    ) {
        let d = (board[x][y] as u8 - b'a') as usize;
        let next = match node.child[d].as_mut() {
            Some(next) => next,
            // 当前路径不是任何单词的前缀，直接剪枝
            None => return,
        };

        if !next.word.is_empty() {
            found.insert(next.word.clone());
        }

        let ch = board[x][y];
        board[x][y] = '#';
        for (dx, dy) in DIRS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= board.len() as isize {
                continue;
            }
            if ny < 0 || ny >= board[0].len() as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if board[nx][ny] != '#' {
                Self::dfs(board, nx, ny, next, found);
            }
        }
        board[x][y] = ch;
    }
}

impl Solution2 {
    pub fn find_words(
        board: Vec<Vec<char>>,
        words: Vec<String>,
    ) -> Vec<String> {
        let mut root = TrieNode::new();
        for word in &words {
            insert_word(&mut root, word);
        }

        let mut board = board;
        let mut ans = Vec::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::dfs(&mut board, i, j, &mut root, &mut ans);
            }
        }
        ans
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        node: &mut TrieNode,
        ans: &mut Vec<String>,
    ) {
        let d = (board[x][y] as u8 - b'a') as usize;

        // 先把子树从父节点摘下，递归期间父指针为空，避免借用冲突
        let mut next = match node.child[d].take() {
            Some(next) => next,
            None => return,
        };

        if !next.word.is_empty() {
            // take 走 word：既收集了结果，又清空了终止标记
            ans.push(std::mem::take(&mut next.word));
        }

        if next.has_child() {
            let ch = board[x][y];
            board[x][y] = '#';
            for (dx, dy) in DIRS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || nx >= board.len() as isize {
                    continue;
                }
                if ny < 0 || ny >= board[0].len() as isize {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if board[nx][ny] != '#' {
                    Self::dfs(board, nx, ny, &mut next, ans);
                }
            }
            board[x][y] = ch;
        }

        // 递归后孩子可能被摘空：空了就让 next 在此 drop，否则挂回父节点
        if next.has_child() {
            node.child[d] = Some(next);
        }
    }
}

// ---- 以下为自测代码，不属于题解主体 ----

fn build_board(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn check(rows: &[&str], words: &[&str], want: &[&str]) {
    let owned: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    let mut got1 = Solution1::find_words(build_board(rows), owned.clone());
    let mut got2 = Solution2::find_words(build_board(rows), owned);
    got1.sort_unstable();
    got2.sort_unstable();

    let mut expected: Vec<String> =
        want.iter().map(|s| s.to_string()).collect();
    expected.sort_unstable();

    assert_eq!(got1, expected, "set({rows:?}, {words:?})");
    assert_eq!(got2, expected, "prune({rows:?}, {words:?})");
}

fn main() {
    // 示例 1
    check(
        &["oaan", "etae", "ihkr", "iflv"],
        &["oath", "pea", "eat", "rain"],
        &["eat", "oath"],
    );

    // 示例 2：同一个格子不能重复使用，"abcb" 不可能
    check(&["ab", "cd"], &["abcb"], &[]);

    // 边界：单格网格
    check(&["a"], &["a"], &["a"]);

    // 边界：单词比可走路径长
    check(&["ab", "cd"], &["abcde"], &[]);

    // 边界：短词是长词的前缀，三个都要输出（2x2 可以走出 3 格 L 形）
    check(&["aa", "aa"], &["a", "aa", "aaa"], &["a", "aa", "aaa"]);

    // 边界：多条路径拼出同一个词，结果只能出现一次
    check(&["aa", "aa"], &["aa"], &["aa"]);

    // 边界：拐弯路径，验证四个方向都走到
    check(
        &["ab", "cd"],
        &["ab", "ac", "abd", "dca"],
        &["ab", "ac", "abd", "dca"],
    );

    // 边界：单词在网格里完全不存在
    check(&["ab", "cd"], &["xy", "zz"], &[]);

    // 边界：3x3 全 'a'，验证前缀词与长词可以同时命中
    check(
        &["aaa", "aaa", "aaa"],
        &["a", "aa", "aaaa", "aaaaaaaaaa"],
        &["a", "aa", "aaaa"],
    );

    println!("all tests passed");
}
