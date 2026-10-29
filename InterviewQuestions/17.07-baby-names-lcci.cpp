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
#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>
using namespace std;

namespace
{
string trim(const string &s)
{
    size_t begin = s.find_first_not_of(" \t\r\n");
    if (begin == string::npos) return "";
    size_t end = s.find_last_not_of(" \t\r\n");
    return s.substr(begin, end - begin + 1);
}

// names 的元素形如 "John(15)"，从最后一个 '(' 切分
pair<string, long long> parseName(const string &s)
{
    size_t lp = s.rfind('(');
    size_t rp = s.rfind(')');
    string name = trim(s.substr(0, lp));
    long long freq = stoll(trim(s.substr(lp + 1, rp - lp - 1)));
    return {name, freq};
}

// synonyms 的元素形如 "(Jon,John)"，去掉括号后按唯一的逗号切分
pair<string, string> parseSynonym(const string &s)
{
    size_t lp = s.find('(');
    size_t rp = s.rfind(')');
    string inner = s.substr(lp + 1, rp - lp - 1);
    size_t comma = inner.find(',');
    return {trim(inner.substr(0, comma)), trim(inner.substr(comma + 1))};
}

// 并查集：结构按大小合并，代表名与频率和挂在根上
struct DSU
{
    unordered_map<string, int> id;
    vector<string> name;
    vector<int> parent;
    vector<int> size;
    vector<long long> total;
    vector<string> best;

    int idOf(const string &s)
    {
        auto it = id.find(s);
        if (it != id.end()) return it->second;

        int idx = static_cast<int>(name.size());
        id[s] = idx;
        name.push_back(s);
        parent.push_back(idx);
        size.push_back(1);
        total.push_back(0);
        best.push_back(s);
        return idx;
    }

    int find(int x)
    {
        // 路径减半：把节点挂到祖父上，摊还代价接近常数
        while (parent[x] != x)
        {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    void unite(int a, int b)
    {
        a = find(a);
        b = find(b);
        if (a == b) return;

        // 结构上按大小合并，保证树高为 O(log n)，配合路径压缩达到 α
        if (size[a] < size[b]) swap(a, b);
        parent[b] = a;
        size[a] += size[b];
        total[a] += total[b];
        // 代表名独立于结构根：只保留字典序最小，与合并方向无关
        if (best[b] < best[a]) best[a] = best[b];
    }
};
} // namespace

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
*/
class Solution
{
public:
    vector<string> trulyMostPopular(vector<string> &names,
                                    vector<string> &synonyms)
    {
        DSU dsu;
        vector<pair<string, long long>> parsed;
        parsed.reserve(names.size());

        for (const string &s : names)
        {
            auto [name, freq] = parseName(s);
            int idx = dsu.idOf(name);
            dsu.total[idx] += freq;
            parsed.push_back({name, freq});
        }

        for (const string &s : synonyms)
        {
            auto [a, b] = parseSynonym(s);
            dsu.unite(dsu.idOf(a), dsu.idOf(b));
        }

        vector<string> result;
        unordered_set<int> emitted;
        for (const auto &entry : parsed)
        {
            const string &name = entry.first;
            int root = dsu.find(dsu.id.at(name));
            if (!emitted.insert(root).second) continue;
            result.push_back(dsu.best[root] + "(" +
                             to_string(dsu.total[root]) + ")");
        }
        return result;
    }
};

// ===== 测试辅助：比较完整结果，并覆盖顺序稳定与频率累加 =====
namespace
{
void check(vector<string> names, vector<string> synonyms,
           const vector<string> &want)
{
    Solution s;
    vector<string> got = s.trulyMostPopular(names, synonyms);
    assert(got == want);
}
} // namespace

int main()
{
    // 示例：Christopher 频率更高，但代表名取字典序更小的 Chris
    check({"John(15)", "Jon(12)", "Chris(13)", "Kris(4)", "Christopher(19)"},
          {"(Jon,John)", "(John,Johnny)", "(Chris,Kris)",
           "(Chris,Christopher)"},
          {"John(27)", "Chris(36)"});

    // 边界：没有同义词对
    check({"Alice(5)"}, {}, {"Alice(5)"});

    // 边界：空输入
    check({}, {}, {});

    // 边界：传递关系必须把三个名字并成一个分量
    check({"A(1)", "B(2)", "C(3)"}, {"(A,B)", "(B,C)"}, {"A(6)"});

    // 反直觉：字典序胜过频率，不能按频率选代表名
    check({"Chris(13)", "Christopher(19)"}, {"(Chris,Christopher)"},
          {"Chris(32)"});

    // 反直觉：只出现在 synonyms 里的名字也可能成为代表名
    check({"John(15)"}, {"(John,Aaron)"}, {"Aaron(15)"});

    // 边界：多个分量按 names 首次出现的顺序输出
    check({"Zed(1)", "Amy(2)", "Bob(3)"}, {"(Amy,Bob)"},
          {"Zed(1)", "Amy(5)"});

    // 边界：重复与反向的同义词对不应重复计数
    check({"A(1)", "B(2)"}, {"(A,B)", "(B,A)"}, {"A(3)"});

    // 边界：频率之和超过 32 位整数范围
    check({"Big(1000000000)", "Bigger(1000000000)"}, {"(Big,Bigger)"},
          {"Big(2000000000)"});

    // 边界：同义词两侧带空格，解析时要先 trim
    check({"John(15)", "Jon(12)"}, {"( Jon , John )"}, {"John(27)"});

    // 边界：未公布的名字作为中间桥梁
    check({"Mike(5)", "John(10)"}, {"(Mike,Johnny)", "(Johnny,John)"},
          {"John(15)"});

    // 边界：同名重复出现时频率累加，只输出一行
    check({"Sam(2)", "Sam(3)"}, {}, {"Sam(5)"});

    cout << "all tests passed" << endl;
    return 0;
}
