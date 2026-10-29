/*
============================================================================
面试题 17.22. 单词转换（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/word-transformer-lcci/
分类：广度优先搜索 · 隐式图路径

给定字典中的两个词，长度相等。写一个方法，把一个词转换成另一个词，
但是一次只能改变一个字符。每一步得到的新词都必须能在字典中找到。

编写一个程序，返回一个可能的转换序列。如有多个可能的转换序列，
你可以返回任何一个。

示例 1：
输入：
beginWord = "hit",
endWord = "cog",
wordList = ["hot","dot","dog","lot","log","cog"]
输出：
["hit","hot","dot","lot","log","cog"]

示例 2：
输入：
beginWord = "hit"
endWord = "cog"
wordList = ["hot","dot","dog","lot","log"]
输出：
[]
解释：
endWord "cog" 不在字典中，所以不存在符合要求的转换序列。

提示：
原题未给出额外约束。
*/
use std::collections::{HashMap, HashSet, VecDeque};

// 沿 parent 表从 begin 走到 end，再反转成起点在前的序列
fn build_path(
    parent: &HashMap<String, String>,
    begin: &str,
    end: &str,
) -> Vec<String> {
    let mut path = Vec::new();
    let mut cur = end.to_string();
    while cur != begin {
        path.push(cur.clone());
        cur = parent.get(&cur).unwrap().clone();
    }
    path.push(begin.to_string());
    path.reverse();
    path
}

/*
解法一：单向广度优先搜索【最优解】

为什么慢：如果从起点做深度优先搜索，一条分支走不通就要回溯换一条，
最坏会枚举出指数级的路径；即便加 visited 剪枝，先撞到的终点也不保证
序列短，重新搜索的代价仍高。

换个角度：把每个单词看成图上的点，相差一个字母的两个词之间连一条边。
问题变成"从起点走到终点，输出一条路径"。BFS 按步数一层层向外扩展，
第一次到达终点时，沿 parent 表倒推就是一条合法序列。

关键点：
  · endWord 不在字典里直接返回空数组，这是示例 2 的判定入口
  · 只有字典中的词可以入队；beginWord 是起点，不必在字典里
  · 逐位替换成 26 个字母再查哈希表，比两两比较所有词快一个量级
  · 入队时就写 parent，同一层里多个词指向它也不会重复入队
  · 到达 endWord 立即返回，此时路径已经最短

手推一遍（示例 1）：
  hit -> hot -> dot -> dog -> cog 是一条 5 个词的路径；
  官方示例给的是 hit-hot-dot-lot-log-cog（6 个词），说明题目只要求
  序列合法，不要求最短。BFS 顺带返回了最短的那条，同样被接受。
  parent 记录 hot<-hit、dot<-hot、dog<-dot、cog<-dog，
  从 cog 倒推到 hit 再反转即得 ["hit","hot","dot","dog","cog"] ✔

复杂度分析：
  时间 O(V·L²)：V 为字典词数、L 为词长；每个词出队一次，每次枚举
    L·26 个候选，每个候选要花 O(L) 构造并哈希，26 视为常数
  空间 O(V·L)：字典与 parent 表各存 V 个长度为 L 的字符串

解法二：双向广度优先搜索【最优解，与解法一并列】

为什么慢：单向 BFS 从起点向外铺，分支因子约为 25L；终点在 d 层时，
要先把前 d-1 层几乎铺满。两个方向各走一半，访问量会小得多。

换个角度：同时维护起点侧与终点侧两张 parent 表，每轮扩展较小的那一侧。
新词一旦出现在另一侧的表里，就找到了相遇点，把两段路径拼起来即可。

关键点：
  · 每轮比较两侧 frontier 大小，只扩展较小的一侧，避免一边跑太深
  · 命中条件是"新词已在另一侧 parent 中"，而不是等到出队
  · 拼接时起点侧要反转（parent 指向起点），终点侧顺着 parent 走向终点
  · 两侧的树在相遇前互不相交，拼出的序列不会出现重复词
  · 最坏仍与解法一同阶，优势在实际访问量；因此两条并列最优

手推一遍（示例 1）：
  起点侧 {hit}，终点侧 {cog}，先扩较小的一侧
  hit 的邻居有 hot -> 记 parentA[hot] = hit
  之后从 hot 扩出 dot、lot；终点侧从 cog 扩出 dog、log
  某一轮 next 同时出现在两张表里，例如 dot，拼接得
  hit-hot-dot-dog-cog ✔

复杂度分析：
  时间 O(V·L²)：两侧合计至多访问每个词一次，单次扩展代价同上
  空间 O(V·L)：两张 parent 表与两个 frontier

rustc 单文件编译时没有平台预置的 Solution，故显式写出这两个结构体。
*/

// ---------- 解法一：单向广度优先搜索 ----------
pub struct Solution1;

impl Solution1 {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<String> {
        let dict: HashSet<String> = word_list.into_iter().collect();

        // 示例 2：终点不在字典里，任何序列都不成立
        if !dict.contains(&end_word) {
            return vec![];
        }
        // 起点即终点时，零步序列本身就是合法答案
        if begin_word == end_word {
            return vec![begin_word];
        }

        let mut parent: HashMap<String, String> = HashMap::new();
        parent.insert(begin_word.clone(), String::new());
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(begin_word.clone());

        while let Some(word) = queue.pop_front() {
            for i in 0..word.len() {
                let old = word.as_bytes()[i];
                for c in b'a'..=b'z' {
                    if c == old {
                        continue;
                    }
                    let mut cand = word.clone().into_bytes();
                    cand[i] = c;
                    let next = String::from_utf8(cand).unwrap();

                    // 不在字典里的词不能作为中间步骤
                    if !dict.contains(&next) {
                        continue;
                    }
                    // 已经入队过的词不再改写 parent，保证同一层不重复
                    if parent.contains_key(&next) {
                        continue;
                    }

                    parent.insert(next.clone(), word.clone());
                    if next == end_word {
                        return build_path(&parent, &begin_word, &end_word);
                    }
                    queue.push_back(next);
                }
            }
        }

        vec![]
    }
}

// ---------- 解法二：双向广度优先搜索 ----------
pub struct Solution2;

impl Solution2 {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<String> {
        let dict: HashSet<String> = word_list.into_iter().collect();

        if !dict.contains(&end_word) {
            return vec![];
        }
        if begin_word == end_word {
            return vec![begin_word];
        }

        let mut parent_a: HashMap<String, String> = HashMap::new();
        let mut parent_b: HashMap<String, String> = HashMap::new();
        parent_a.insert(begin_word.clone(), String::new());
        parent_b.insert(end_word.clone(), String::new());

        let mut front_a: HashSet<String> = HashSet::new();
        let mut front_b: HashSet<String> = HashSet::new();
        front_a.insert(begin_word.clone());
        front_b.insert(end_word.clone());

        while !front_a.is_empty() && !front_b.is_empty() {
            // 扩较小的一侧，让两边的深度尽量同步
            let expand_a = front_a.len() <= front_b.len();
            let (next_front, meet) = if expand_a {
                expand(&front_a, &mut parent_a, &parent_b, &dict)
            } else {
                expand(&front_b, &mut parent_b, &parent_a, &dict)
            };

            if let Some(meet) = meet {
                return join_path(
                    &parent_a, &parent_b, &begin_word, &end_word, &meet,
                );
            }
            if expand_a {
                front_a = next_front;
            } else {
                front_b = next_front;
            }
        }

        vec![]
    }
}

// 从一侧 frontier 扩一层；返回下一层与相遇词（若撞上另一侧）
fn expand(
    front: &HashSet<String>,
    parent: &mut HashMap<String, String>,
    other: &HashMap<String, String>,
    dict: &HashSet<String>,
) -> (HashSet<String>, Option<String>) {
    let mut next_front = HashSet::new();
    for word in front {
        for i in 0..word.len() {
            let old = word.as_bytes()[i];
            for c in b'a'..=b'z' {
                if c == old {
                    continue;
                }
                let mut cand = word.clone().into_bytes();
                cand[i] = c;
                let next = String::from_utf8(cand).unwrap();

                if !dict.contains(&next) {
                    continue;
                }
                if parent.contains_key(&next) {
                    continue;
                }

                parent.insert(next.clone(), word.clone());
                if other.contains_key(&next) {
                    return (next_front, Some(next));
                }
                next_front.insert(next);
            }
        }
    }
    (next_front, None)
}

fn join_path(
    parent_a: &HashMap<String, String>,
    parent_b: &HashMap<String, String>,
    begin: &str,
    end: &str,
    meet: &str,
) -> Vec<String> {
    // 起点侧：从相遇点沿 parent 走到 begin，再反转
    let mut path = Vec::new();
    let mut cur = meet.to_string();
    while cur != begin {
        path.push(cur.clone());
        cur = parent_a.get(&cur).unwrap().clone();
    }
    path.push(begin.to_string());
    path.reverse();

    // 终点侧：相遇点已经写入，从它的下一个词接到 end
    let mut cur = meet.to_string();
    while cur != end {
        cur = parent_b.get(&cur).unwrap().clone();
        path.push(cur.clone());
    }
    path
}

// ===== 测试辅助：校验序列是否合法，而不是比对某一条固定路径 =====
fn one_char_apart(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut diff = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            diff += 1;
            // 第二个不同字符出现即可判定失败
            if diff > 1 {
                return false;
            }
        }
    }
    diff == 1
}

fn valid_path(
    path: &[String],
    begin: &str,
    end: &str,
    word_list: &[String],
) -> bool {
    if path.is_empty() {
        return false;
    }
    if path[0] != begin || path[path.len() - 1] != end {
        return false;
    }

    let dict: HashSet<&str> = word_list.iter().map(|s| s.as_str()).collect();
    // 起点不需要在字典里，之后每一步都必须是字典中的词
    for i in 1..path.len() {
        if !one_char_apart(&path[i - 1], &path[i])
            || !dict.contains(path[i].as_str())
        {
            return false;
        }
    }
    true
}

fn check(begin: &str, end: &str, word_list: &[&str], has_path: bool) {
    let list: Vec<String> = word_list.iter().map(|s| s.to_string()).collect();

    let got_bfs = Solution1::find_ladders(
        begin.to_string(),
        end.to_string(),
        list.clone(),
    );
    if has_path {
        assert!(
            valid_path(&got_bfs, begin, end, &list),
            "bfs({}, {}) = {:?}, invalid path",
            begin,
            end,
            got_bfs
        );
    } else {
        assert!(
            got_bfs.is_empty(),
            "bfs({}, {}) = {:?}, want empty",
            begin,
            end,
            got_bfs
        );
    }

    let got_bi = Solution2::find_ladders(
        begin.to_string(),
        end.to_string(),
        list.clone(),
    );
    if has_path {
        assert!(
            valid_path(&got_bi, begin, end, &list),
            "bidirectional({}, {}) = {:?}, invalid path",
            begin,
            end,
            got_bi
        );
    } else {
        assert!(
            got_bi.is_empty(),
            "bidirectional({}, {}) = {:?}, want empty",
            begin,
            end,
            got_bi
        );
    }
}

fn main() {
    // 示例 1：多条合法路径，只校验合法性
    check(
        "hit",
        "cog",
        &["hot", "dot", "dog", "lot", "log", "cog"],
        true,
    );

    // 示例 2：endWord 不在字典中
    check("hit", "cog", &["hot", "dot", "dog", "lot", "log"], false);

    // 边界：起点即终点，零步序列
    check("hot", "hot", &["hot"], true);

    // 边界：字典里只有终点，和起点差多个字符
    check("hit", "cog", &["cog"], false);

    // 边界：单字符词，a -> b -> c
    check("a", "c", &["b", "c"], true);

    // 边界：需要连续改四个位置
    check("aaaa", "dddd", &["aaad", "aadd", "addd", "dddd"], true);

    // 反直觉：存在多条等长路径，不能假定某一条固定顺序
    check("ab", "cd", &["ac", "ad", "cc", "cd"], true);

    // 反直觉：终点可达但必须经过中间词，不能跳到 "abcd"
    check("ab", "cd", &["abcd", "cd"], false);

    // 边界：词表中混入与起点相同的词，不应影响 parent
    check("hit", "cog", &["hit", "hot", "dot", "dog", "cog"], true);

    // 边界：26 个单字母词构成的完全图，任意两点一步可达
    let letters: Vec<String> = (b'a'..=b'z')
        .map(|c| (c as char).to_string())
        .collect();
    let got1 = Solution1::find_ladders(
        "a".to_string(),
        "z".to_string(),
        letters.clone(),
    );
    assert!(valid_path(&got1, "a", "z", &letters));
    let got2 = Solution2::find_ladders(
        "a".to_string(),
        "z".to_string(),
        letters.clone(),
    );
    assert!(valid_path(&got2, "a", "z", &letters));

    println!("all tests passed");
}
