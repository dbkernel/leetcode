/*
============================================================================
面试题 17.07. 婴儿名字（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/baby-names-lcci/
分类：并查集 · 连通分量

每年，政府都会公布一万个最常见的婴儿名字和它们出现的频率，也就是同名
婴儿的数量。有些名字有多种拼法，例如，John 和 Jon 本质上是相同的名字，
但被当成了两个名字公布出来。给定两个列表，一个是名字及对应的频率，
另一个是本质相同的名字对。设计一个算法打印出每个真实名字的实际频率。
注意，如果 John 和 Jon 是相同的，并且 Jon 和 Johnny 相同，则 John 与
Johnny 也相同，即它们有传递和对称性。

在结果列表中，选择 字典序最小 的名字作为真实名字。

示例：
输入：names = ["John(15)","Jon(12)","Chris(13)","Kris(4)",
"Christopher(19)"], synonyms = ["(Jon,John)","(John,Johnny)",
"(Chris,Kris)","(Chris,Christopher)"]
输出：["John(27)","Chris(36)"]

提示：
- names.length <= 100000
*/
use std::collections::{HashMap, HashSet};

// names 的元素形如 "John(15)"，从最后一个 '(' 切分
fn parse_name(s: &str) -> (String, i64) {
    let lp = s.rfind('(').unwrap();
    let rp = s.rfind(')').unwrap();
    let name = s[..lp].trim().to_string();
    let freq = s[lp + 1..rp].trim().parse::<i64>().unwrap();
    (name, freq)
}

// synonyms 的元素形如 "(Jon,John)"，去掉括号后按唯一的逗号切分
fn parse_synonym(s: &str) -> (String, String) {
    let lp = s.find('(').unwrap();
    let rp = s.rfind(')').unwrap();
    let inner = &s[lp + 1..rp];
    let comma = inner.find(',').unwrap();
    let a = inner[..comma].trim().to_string();
    let b = inner[comma + 1..].trim().to_string();
    (a, b)
}

/*
解法：并查集（同义词连通分量 + 字典序代表名）【最优解】

为什么慢：如果只把同义词对两两相加，遇到 (A,B)、(B,C) 这种链式关系时，
A 和 C 的合并会被漏掉；若每次都重新扫描全部已合并集合，最坏 O(n·m)。
问题的本质是"等价类合并"，需要的正是能处理传递闭包的并查集。

换个角度：把每个名字看成点，每条同义词对看成一条无向边，求连通分量。
同一个分量里的名字本质相同：频率在分量内求和，真实名字取分量内
字典序最小的那个。边读边合并，最后每个分量只剩一个根。

关键点：
  · 代表名只看字典序，不看频率：示例里 Christopher 的频率 19 高于
    Chris 的 13，输出却是 Chris(36)。按"更频繁者当名字"会得到
    Christopher(36)，与官方示例不符
  · 频率只在分量内累加，累加值存在根上，合并时一并相加
  · 只出现在 synonyms 里的名字也要建点（频率记 0），它可能把两个已
    公布的名字连起来，也可能本身就是字典序更小的代表名
  · 合并按集合大小进行，代表名单独用 best 维护，两者互不干扰
  · 输出只遍历 names 里出现过的名字，按首次出现顺序去重，结果稳定
  · 解析 names 用最后一个 '('，解析 synonyms 去掉首尾括号后按逗号切

手推一遍（示例）：
  (Jon,John)          -> 分量 {John(15), Jon(12)}，best = John，和 27
  (John,Johnny)       -> Johnny 频率 0 并入，best 仍是 John
  (Chris,Kris)        -> best = Chris，和 17
  (Chris,Christopher) -> Christopher 频率 19 并入，best 仍是 Chris，和 36
  按 names 首次出现顺序输出 John(27)、Chris(36) ✔

复杂度分析：
  时间 O((N + M)·α(N)·L)：N 为不同名字数、M 为同义词对数；路径压缩
    与按大小合并的摊还代价为反阿克曼函数，字符串哈希/比较再乘 L
  空间 O(N·L)：哈希表、并查集数组与名字副本

rustc 单文件编译时没有平台预置的 Solution，故显式写出该结构体。
*/

struct Dsu {
    id: HashMap<String, usize>,
    name: Vec<String>,
    parent: Vec<usize>,
    size: Vec<usize>,
    total: Vec<i64>,
    best: Vec<String>,
}

impl Dsu {
    fn new() -> Self {
        Dsu {
            id: HashMap::new(),
            name: Vec::new(),
            parent: Vec::new(),
            size: Vec::new(),
            total: Vec::new(),
            best: Vec::new(),
        }
    }

    fn id_of(&mut self, s: &str) -> usize {
        if let Some(&idx) = self.id.get(s) {
            return idx;
        }
        let idx = self.name.len();
        self.id.insert(s.to_string(), idx);
        self.name.push(s.to_string());
        self.parent.push(idx);
        self.size.push(1);
        self.total.push(0);
        self.best.push(s.to_string());
        idx
    }

    fn find(&mut self, mut x: usize) -> usize {
        // 路径减半：把节点挂到祖父上，摊还代价接近常数
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return;
        }

        // 结构上按大小合并，保证树高为 O(log n)，配合路径压缩达到 α
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.total[a] += self.total[b];
        // 代表名独立于结构根：只保留字典序最小，与合并方向无关
        let candidate = self.best[b].clone();
        if candidate < self.best[a] {
            self.best[a] = candidate;
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn truly_most_popular(
        names: Vec<String>,
        synonyms: Vec<String>,
    ) -> Vec<String> {
        let mut dsu = Dsu::new();
        let mut parsed: Vec<(String, i64)> = Vec::new();

        for s in &names {
            let (name, freq) = parse_name(s);
            let idx = dsu.id_of(&name);
            dsu.total[idx] += freq;
            parsed.push((name, freq));
        }

        for s in &synonyms {
            let (a, b) = parse_synonym(s);
            let ia = dsu.id_of(&a);
            let ib = dsu.id_of(&b);
            dsu.unite(ia, ib);
        }

        let mut result = Vec::new();
        let mut emitted: HashSet<usize> = HashSet::new();
        for (name, _) in &parsed {
            let id = *dsu.id.get(name.as_str()).unwrap();
            let root = dsu.find(id);
            if !emitted.insert(root) {
                continue;
            }
            result.push(format!("{}({})", dsu.best[root], dsu.total[root]));
        }
        result
    }
}

// ===== 测试辅助：比较完整结果，并覆盖顺序稳定与频率累加 =====
fn check(names: &[&str], synonyms: &[&str], want: &[&str]) {
    let name_list: Vec<String> = names.iter().map(|s| s.to_string()).collect();
    let synonym_list: Vec<String> =
        synonyms.iter().map(|s| s.to_string()).collect();
    let want_list: Vec<String> = want.iter().map(|s| s.to_string()).collect();
    assert_eq!(
        Solution::truly_most_popular(name_list, synonym_list),
        want_list
    );
}

fn main() {
    // 示例：Christopher 频率更高，但代表名取字典序更小的 Chris
    check(
        &[
            "John(15)",
            "Jon(12)",
            "Chris(13)",
            "Kris(4)",
            "Christopher(19)",
        ],
        &[
            "(Jon,John)",
            "(John,Johnny)",
            "(Chris,Kris)",
            "(Chris,Christopher)",
        ],
        &["John(27)", "Chris(36)"],
    );

    // 边界：没有同义词对
    check(&["Alice(5)"], &[], &["Alice(5)"]);

    // 边界：空输入
    check(&[], &[], &[]);

    // 边界：传递关系必须把三个名字并成一个分量
    check(&["A(1)", "B(2)", "C(3)"], &["(A,B)", "(B,C)"], &["A(6)"]);

    // 反直觉：字典序胜过频率，不能按频率选代表名
    check(
        &["Chris(13)", "Christopher(19)"],
        &["(Chris,Christopher)"],
        &["Chris(32)"],
    );

    // 反直觉：只出现在 synonyms 里的名字也可能成为代表名
    check(&["John(15)"], &["(John,Aaron)"], &["Aaron(15)"]);

    // 边界：多个分量按 names 首次出现的顺序输出
    check(
        &["Zed(1)", "Amy(2)", "Bob(3)"],
        &["(Amy,Bob)"],
        &["Zed(1)", "Amy(5)"],
    );

    // 边界：重复与反向的同义词对不应重复计数
    check(&["A(1)", "B(2)"], &["(A,B)", "(B,A)"], &["A(3)"]);

    // 边界：频率之和超过 32 位整数范围
    check(
        &["Big(1000000000)", "Bigger(1000000000)"],
        &["(Big,Bigger)"],
        &["Big(2000000000)"],
    );

    // 边界：同义词两侧带空格，解析时要先 trim
    check(&["John(15)", "Jon(12)"], &["( Jon , John )"], &["John(27)"]);

    // 边界：未公布的名字作为中间桥梁
    check(
        &["Mike(5)", "John(10)"],
        &["(Mike,Johnny)", "(Johnny,John)"],
        &["John(15)"],
    );

    // 边界：同名重复出现时频率累加，只输出一行
    check(&["Sam(2)", "Sam(3)"], &[], &["Sam(5)"]);

    println!("all tests passed");
}
