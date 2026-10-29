/*
============================================================================
127. 单词接龙（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/word-ladder/
分类：广度优先搜索 · 无权图最短路

字典 wordList 中从单词 beginWord 到 endWord 的 转换序列 是一个按下述
规格形成的序列 beginWord -> s1 -> s2 -> ... -> sk：

- 每一对相邻的单词只差一个字母。
- 对于 1 <= i <= k 时，每个 si 都在 wordList 中。注意，beginWord
  不需要在 wordList 中。
- sk == endWord

给你两个单词 beginWord 和 endWord 和一个字典 wordList ，返回 从
beginWord 到 endWord 的 最短转换序列 中的 单词数目 。如果不存在这样的
转换序列，返回 0 。

示例 1：
输入：beginWord = "hit", endWord = "cog",
      wordList = ["hot","dot","dog","lot","log","cog"]
输出：5
解释：一个最短转换序列是 "hit" -> "hot" -> "dot" -> "dog" -> "cog"，
返回它的长度 5。

示例 2：
输入：beginWord = "hit", endWord = "cog",
      wordList = ["hot","dot","dog","lot","log"]
输出：0
解释：endWord "cog" 不在字典中，所以无法进行转换。

提示：
- 1 <= beginWord.length <= 10
- endWord.length == beginWord.length
- 1 <= wordList.length <= 5000
- wordList[i].length == beginWord.length
- beginWord、endWord 和 wordList[i] 由小写英文字母组成
- beginWord != endWord
- wordList 中的所有字符串 互不相同
*/
use std::collections::{HashSet, VecDeque};

/*
解法一：逐层 BFS（隐式图，枚举替换字母）【最优解】

为什么慢：若把"一步可达"全部显式建图，就要把字典里任意两个单词都比一遍，
O(N^2) 次比较、每次 O(L) 字符（N = 5000、L = 10 时约 2.5 亿次字符比较），
建图本身就超时，而真正被用到的边只是其中极小一部分。

换个角度：这张图是隐式的，一个单词的邻居不必预先枚举——把 L 个位置分别
换成 26 个字母，就现场列出了它的全部邻居，再用哈希表 O(1) 判断邻居是否在
字典里。总枚举量 N·L·26 约 130 万，比 N^2·L 小两个量级。

为什么这里只能用 BFS 而不是 DFS：每条边的代价都是 1（一次字母替换），求的是
无权图最短路。DFS 顺着一支走到底，第一次摸到 endWord 时的路径未必最短，
要保证最短就得把所有分支都走完，层数信息完全丢失。

关键点：
  · endWord 不在字典里直接返回 0，这种输入不必开搜
  · 队列按层推进，`steps` 只在一层全部出队后 +1，于是它天然等于"当前层
    单词在序列中的序号"，第一次命中 endWord 时的序号就是最短单词数
  · 入队即标记：单词一入队就从字典里删掉，保证每个单词只展开一次
  · beginWord 本身不必在字典里，但要单独入队；若它恰好在字典里，先删掉，
    否则会被当成邻居再次入队
  · 题面保证 beginWord != endWord；若两者相等，最短序列只含起点本身，
    按单词数计为 1
  · Rust 里 `HashSet::remove` 返回 bool，正好同时完成"判断存在"与"标记
    已用"，不必先 `contains` 再 `remove`

手推一遍：
以示例 1（hit → cog，字典含 cog）为例：
  第 1 层 {hit}：换字母只命中 hot，steps = 1
  第 2 层 {hot}：命中 dot、lot
  第 3 层 {dot, lot}：命中 dog、log
  第 4 层 {dog, log}：命中 cog，steps = 4，返回 5 ✔

复杂度分析：
  时间 O(N·L·26)：每个单词最多出队一次，展开时枚举 L 个位置 × 26 个字母
  空间 O(N·L)：字典哈希表存下全部单词，队列最多同时放一层
*/

// 单文件 rustc 编译时不存在平台的 Solution，这里显式声明为单元结构体
pub struct Solution1;

impl Solution1 {
    pub fn ladder_length(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> i32 {
        let mut dict: HashSet<String> = word_list.into_iter().collect();
        // 终点不在字典里，任何序列都接不上
        if !dict.contains(&end_word) {
            return 0;
        }
        // 题面保证两者不同；相等时序列只含起点本身，单词数计为 1
        if begin_word == end_word {
            return 1;
        }

        // begin_word 可能不在字典里，删除是空操作；在字典里则顺便标记为已用
        dict.remove(&begin_word);

        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(begin_word);
        let mut steps = 1; // 队首层的单词在序列里的序号

        while let Some(_) = queue.front() {
            let layer_size = queue.len();
            for _ in 0..layer_size {
                let word = queue.pop_front().unwrap();
                let bytes = word.as_bytes().to_vec();

                for pos in 0..bytes.len() {
                    for c in b'a'..=b'z' {
                        // 位置不变不算一次转换
                        if c == bytes[pos] {
                            continue;
                        }
                        let mut cand = bytes.clone();
                        cand[pos] = c;
                        let s = String::from_utf8(cand).unwrap();

                        // 先判终点：endWord 可能已被删出字典，但命中就该收工
                        if s == end_word {
                            return steps + 1;
                        }
                        // remove 返回 true 说明此前未被用过，入队后即算标记
                        if dict.remove(&s) {
                            queue.push_back(s);
                        }
                    }
                }
            }
            steps += 1;
        }

        0
    }
}

/*
解法二：双向 BFS【最优解，与解法一并列】

为什么慢：单向 BFS 从 beginWord 出发，展开的单词数随层数指数增长，量级是
b^d（b 是平均出度，d 是最短序列的边数）。中间层离两端都很远，绝大多数
展开都花在这一层上，而它只贡献"再走一步"。

换个角度：从 beginWord 和 endWord 各起一个 BFS，每轮只扩展当前更小的那一层，
两个搜索圈迟早会碰面。碰面处的路径由两段拼成，展开量从 b^d 降到
b^(d/2) + b^(d/2)，实际运行时间常常相差几个数量级。

为什么仍然是 BFS：两端都在按层推进，遇到交集时，交集中的单词到两端的层数都
已经是最小的，拼出来的序列必然最短；这也是双向 BFS 只在无权图里成立的原因。

关键点：
  · 每轮只展开较小的一侧（`front.len() > back.len()` 时交换），否则常数
    上的优势就消失了
  · 先判"邻居是否落在对面层"再判"是否在字典里"：对面的词可能已被删出字典，
    顺序颠倒就会漏掉相遇点
  · 相遇时返回 `steps + 1`：`steps` 表示当前最短序列已经确定的单词数，
    再跨一条边接到对面
  · 每层无论朝哪一侧扩展，`steps` 都只 +1，所以两侧步数之和自然累计
  · 相邻的已用词直接跳过（字典里已删），避免在两圈之间来回横跳
  · 交换用 `std::mem::swap`，不能写成 `front = back` 再重新赋值

手推一遍：
以示例 1 为例，记 steps 为"已确定的单词数"：
  front = {hit}，back = {cog}，steps = 1
  展开 hit：hot 不在 back 里，front = {hot}，steps = 2
  展开 hot：dot、lot 都不在 back 里，front = {dot, lot}，steps = 3
  back 更小，展开 cog：dog、log 不在 front 里，back = {dog, log}，steps = 4
  展开 dot：邻居 dog 落在 back 里，返回 4 + 1 = 5 ✔

复杂度分析：
  时间 O(N·L·26)：两侧合起来每个单词仍只展开一次，只是实际展开的单词数远
    少于单向 BFS
  空间 O(N·L)：两侧的层加字典
*/

pub struct Solution2;

impl Solution2 {
    pub fn ladder_length(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> i32 {
        let mut dict: HashSet<String> = word_list.into_iter().collect();
        // 两端相同：不需要任何转换，序列就是起点这一个词
        if begin_word == end_word {
            return 1;
        }
        if !dict.contains(&end_word) {
            return 0;
        }

        // 两端都在字典里删掉：它们已属于各自的搜索圈，不该再被当成邻居
        dict.remove(&begin_word);
        dict.remove(&end_word);

        let mut front: HashSet<String> = HashSet::new();
        front.insert(begin_word);
        let mut back: HashSet<String> = HashSet::new();
        back.insert(end_word);
        let mut steps = 1; // 已确定的单词数，等于两侧层数之和 + 1

        while !front.is_empty() && !back.is_empty() {
            // 只扩展较小的一侧，让每一层的枚举量都是两侧中较小者
            if front.len() > back.len() {
                std::mem::swap(&mut front, &mut back);
            }

            let mut next: HashSet<String> = HashSet::new();
            for word in front.iter() {
                let bytes = word.as_bytes().to_vec();
                for pos in 0..bytes.len() {
                    for c in b'a'..=b'z' {
                        if c == bytes[pos] {
                            continue;
                        }
                        let mut cand = bytes.clone();
                        cand[pos] = c;
                        let s = String::from_utf8(cand).unwrap();

                        // 落在对面层即为相遇：对面那个词可能已不在字典里
                        if back.contains(&s) {
                            return steps + 1;
                        }
                        if dict.remove(&s) {
                            next.insert(s);
                        }
                    }
                }
            }
            front = next;
            steps += 1;
        }

        0
    }
}

fn check(begin: &str, end: &str, words: &[&str], want: i32) {
    let list: Vec<String> = words.iter().map(|w| w.to_string()).collect();
    assert_eq!(
        Solution1::ladder_length(
            begin.to_string(),
            end.to_string(),
            list.clone()
        ),
        want
    );
    assert_eq!(
        Solution2::ladder_length(begin.to_string(), end.to_string(), list),
        want
    );
}

fn main() {
    // 示例 1：最短序列 5 个单词
    check(
        "hit",
        "cog",
        &["hot", "dot", "dog", "lot", "log", "cog"],
        5,
    );

    // 示例 2：endWord 不在字典里，不开搜直接 0
    check("hit", "cog", &["hot", "dot", "dog", "lot", "log"], 0);

    // 边界：beginWord == endWord。题面保证不会出现，此时序列只含起点，答案 1；
    // 字典里故意放上 beginWord，确保走的是相等分支而不是"缺 endWord"分支
    check("hit", "hit", &["hit", "hot"], 1);

    // 边界：一步直达，序列是 2 个单词
    check("hot", "dot", &["dot"], 2);

    // 边界：必须逐个字母接力（ab→ad→cd→cf→ef）
    check("ab", "ef", &["ad", "cd", "cf", "ef"], 5);

    // 边界：endWord 在字典里，但整体不可达
    check("hit", "cog", &["hot", "dog", "cog"], 0);

    // 边界：字典只有一个与两端都无关的词
    check("hit", "cog", &["xyz"], 0);

    println!("all tests passed");
}
