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
#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

namespace
{
// 沿 parent 表从 begin 走到 end，再反转成起点在前的序列
vector<string> buildPath(const unordered_map<string, string> &parent,
                         const string &begin, const string &end)
{
    vector<string> path;
    string cur = end;
    while (cur != begin)
    {
        path.push_back(cur);
        cur = parent.at(cur);
    }
    path.push_back(begin);
    reverse(path.begin(), path.end());
    return path;
}
} // namespace

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
*/
class Solution1
{
public:
    vector<string> findLadders(string beginWord, string endWord,
                               vector<string> &wordList)
    {
        unordered_set<string> dict(wordList.begin(), wordList.end());
        if (dict.find(endWord) == dict.end())
        {
            return {};
        }
        // 起点即终点时，零步序列本身就是合法答案
        if (beginWord == endWord)
        {
            return {beginWord};
        }

        unordered_map<string, string> parent;
        parent[beginWord] = "";
        queue<string> q;
        q.push(beginWord);

        while (!q.empty())
        {
            string word = q.front();
            q.pop();

            string next = word;
            for (size_t i = 0; i < word.size(); ++i)
            {
                char old = word[i];
                for (char c = 'a'; c <= 'z'; ++c)
                {
                    if (c == old) continue;
                    next[i] = c;

                    // 不在字典里的词不能作为中间步骤
                    if (dict.count(next) == 0) continue;
                    // 已经入队过的词不再改写 parent，保证同一层不重复
                    if (parent.count(next) != 0) continue;

                    parent[next] = word;
                    if (next == endWord)
                    {
                        return buildPath(parent, beginWord, endWord);
                    }
                    q.push(next);
                }
                // next 是复用的缓冲区，本位置试完后恢复
                next[i] = old;
            }
        }

        return {};
    }
};

/*
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
*/

class Solution2
{
public:
    vector<string> findLadders(string beginWord, string endWord,
                               vector<string> &wordList)
    {
        unordered_set<string> dict(wordList.begin(), wordList.end());
        if (dict.find(endWord) == dict.end())
        {
            return {};
        }
        if (beginWord == endWord)
        {
            return {beginWord};
        }

        unordered_map<string, string> parentA;
        unordered_map<string, string> parentB;
        parentA[beginWord] = "";
        parentB[endWord] = "";

        unordered_set<string> frontA{beginWord};
        unordered_set<string> frontB{endWord};

        while (!frontA.empty() && !frontB.empty())
        {
            // 扩较小的一侧，让两边的深度尽量同步
            bool expandA = frontA.size() <= frontB.size();
            unordered_set<string> &front = expandA ? frontA : frontB;
            unordered_map<string, string> &parent = expandA ? parentA : parentB;
            const unordered_map<string, string> &other =
                expandA ? parentB : parentA;

            unordered_set<string> nextFront;
            for (const string &word : front)
            {
                string next = word;
                for (size_t i = 0; i < word.size(); ++i)
                {
                    char old = word[i];
                    for (char c = 'a'; c <= 'z'; ++c)
                    {
                        if (c == old) continue;
                        next[i] = c;

                        if (dict.count(next) == 0) continue;
                        if (parent.count(next) != 0) continue;

                        parent[next] = word;
                        if (other.count(next) != 0)
                        {
                            return joinPath(parentA, parentB, beginWord,
                                            endWord, next);
                        }
                        nextFront.insert(next);
                    }
                    next[i] = old;
                }
            }
            front.swap(nextFront);
        }

        return {};
    }

private:
    vector<string> joinPath(const unordered_map<string, string> &parentA,
                            const unordered_map<string, string> &parentB,
                            const string &begin, const string &end,
                            const string &meet) const
    {
        // 起点侧：从相遇点沿 parent 走到 begin，再反转
        vector<string> path;
        string cur = meet;
        while (cur != begin)
        {
            path.push_back(cur);
            cur = parentA.at(cur);
        }
        path.push_back(begin);
        reverse(path.begin(), path.end());

        // 终点侧：相遇点已经写入，从它的下一个词接到 end
        cur = meet;
        while (cur != end)
        {
            cur = parentB.at(cur);
            path.push_back(cur);
        }
        return path;
    }
};

// ===== 测试辅助：校验序列是否合法，而不是比对某一条固定路径 =====
namespace
{
bool oneCharApart(const string &a, const string &b)
{
    int diff = 0;
    for (size_t i = 0; i < a.size(); ++i)
    {
        if (a[i] != b[i])
        {
            ++diff;
            // 第二个不同字符出现即可判定失败
            if (diff > 1) return false;
        }
    }
    return diff == 1;
}

bool inWordList(const vector<string> &wordList, const string &word)
{
    return find(wordList.begin(), wordList.end(), word) != wordList.end();
}

bool validPath(const vector<string> &path, const string &begin,
               const string &end, const vector<string> &wordList)
{
    if (path.empty()) return false;
    if (path.front() != begin || path.back() != end) return false;

    // 起点不需要在字典里，之后每一步都必须是字典中的词
    for (size_t i = 1; i < path.size(); ++i)
    {
        if (!oneCharApart(path[i - 1], path[i])) return false;
        if (!inWordList(wordList, path[i])) return false;
    }
    return true;
}

void check(const string &begin, const string &end,
           const vector<string> &wordList, bool hasPath)
{
    vector<string> list1 = wordList;
    vector<string> got1 = Solution1().findLadders(begin, end, list1);
    assert(hasPath ? validPath(got1, begin, end, wordList) : got1.empty());

    vector<string> list2 = wordList;
    vector<string> got2 = Solution2().findLadders(begin, end, list2);
    assert(hasPath ? validPath(got2, begin, end, wordList) : got2.empty());
}
} // namespace

int main()
{
    // 示例 1：多条合法路径，只校验合法性
    check("hit", "cog", {"hot", "dot", "dog", "lot", "log", "cog"}, true);

    // 示例 2：endWord 不在字典中
    check("hit", "cog", {"hot", "dot", "dog", "lot", "log"}, false);

    // 边界：起点即终点，零步序列
    check("hot", "hot", {"hot"}, true);

    // 边界：字典里只有终点，和起点差多个字符
    check("hit", "cog", {"cog"}, false);

    // 边界：单字符词，a -> b -> c
    check("a", "c", {"b", "c"}, true);

    // 边界：需要连续改四个位置
    check("aaaa", "dddd", {"aaad", "aadd", "addd", "dddd"}, true);

    // 反直觉：存在多条等长路径，不能假定某一条固定顺序
    check("ab", "cd", {"ac", "ad", "cc", "cd"}, true);

    // 反直觉：终点可达但必须经过中间词，不能跳到 "abcd"
    check("ab", "cd", {"abcd", "cd"}, false);

    // 边界：词表中混入与起点相同的词，不应影响 parent
    check("hit", "cog", {"hit", "hot", "dot", "dog", "cog"}, true);

    // 边界：26 个单字母词构成的完全图，任意两点一步可达
    {
        vector<string> letters;
        for (char c = 'a'; c <= 'z'; ++c) letters.push_back(string(1, c));
        check("a", "z", letters, true);
    }

    cout << "all tests passed" << endl;
    return 0;
}
