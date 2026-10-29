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
#include <algorithm>
#include <cassert>
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;

// 字典树节点：child 存 26 个孩子，word 非空表示这里是一个单词的结尾
struct TrieNode
{
    TrieNode *child[26];
    string word;

    TrieNode() : word()
    {
        for (TrieNode *&p : child) p = nullptr;
    }

    ~TrieNode()
    {
        for (TrieNode *p : child) delete p;
    }
};

namespace
{
void insertWord(TrieNode *root, const string &word)
{
    TrieNode *node = root;
    for (char c : word)
    {
        int d = c - 'a';
        if (node->child[d] == nullptr) node->child[d] = new TrieNode();
        node = node->child[d];
    }
    node->word = word;
}

bool hasChild(const TrieNode *node)
{
    for (const TrieNode *p : node->child)
        if (p != nullptr) return true;
    return false;
}

const int DX[4] = {1, -1, 0, 0};
const int DY[4] = {0, 0, 1, -1};
} // namespace

/*
解法一：字典树 + 回溯（结果用集合去重）【最优解】

为什么慢：如果对 words 里的每个词单独在网格里做一次 DFS，最坏要
O(k * m * n * 3^l)，k 是词数。大量词共享前缀时，这些前缀路径被反复搜索。

换个角度：把所有词插入 Trie，再从每个格子出发做一次 DFS。DFS 的每一步都
沿着 Trie 的孩子走，走不通就剪枝；一条路径同时覆盖所有以它为前缀的词，
前缀只需要展开一次。

关键点：
  · 进入格子前先看 Trie 有没有对应孩子，没有就直接返回，避免无谓递归
  · 同一个单词可能由多条不同路径拼出（如多个 "a"），所以结果用 set 去重
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
搜很多遍，只靠 set 去重并不能减少搜索量。

换个角度：一个单词一旦被找到，就从 Trie 里把它的终止标记清空；如果某个
节点既没有单词也没有孩子，就把它从父节点上摘掉。这样后续 DFS 走到这条
分支会立刻被剪掉，每个单词最多被完整匹配一次。

关键点：
  · 命中后把 node.word 清空，而不是只往结果里塞；否则同一词会被反复命中
  · 清空后如果 node 还有孩子（如 "ab" 是 "abc" 的前缀），不能删节点
  · 递归返回后要再检查一次孩子是否被删空，空了才摘掉并释放
  · 因为每个词只命中一次，结果不需要 set 去重，顺序也不要求
  · 摘节点时先把父指针置空再 delete，避免析构时重复释放

以示例 1 为例：
  命中 "eat" 后清空终止标记；"oath" 同理
  若输入是 ["a","aa","aaa"] 且网格全为 'a'，命中 "a" 后不再重复收集，
  命中 "aaa" 后其叶节点被摘掉，下一条路径到 "aa" 就停

复杂度分析：
  时间 O(m * n * 3^(l-1))：上界与解法一相同，但已匹配的分支被剪掉，
  重复词多的数据上实际搜索量显著下降
  空间 O(k * l)：Trie 节点总数与所有单词总长同阶；递归栈 O(l)
*/
class Solution1
{
public:
    vector<string> findWords(vector<vector<char>> &board, vector<string> &words)
    {
        TrieNode *root = new TrieNode();
        for (const string &word : words) insertWord(root, word);

        set<string> found;
        int m = static_cast<int>(board.size());
        int n = static_cast<int>(board[0].size());
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j) dfs(board, i, j, root, found);

        vector<string> ans(found.begin(), found.end());
        delete root;
        return ans;
    }

private:
    void dfs(vector<vector<char>> &board, int x, int y, TrieNode *node,
             set<string> &found)
    {
        char ch = board[x][y];
        TrieNode *next = node->child[ch - 'a'];
        // 当前路径不是任何单词的前缀，直接剪枝
        if (next == nullptr) return;

        if (!next->word.empty()) found.insert(next->word);

        board[x][y] = '#';
        for (int k = 0; k < 4; ++k)
        {
            int nx = x + DX[k];
            int ny = y + DY[k];
            if (nx < 0 || nx >= static_cast<int>(board.size())) continue;
            if (ny < 0 || ny >= static_cast<int>(board[0].size())) continue;
            if (board[nx][ny] == '#') continue;
            dfs(board, nx, ny, next, found);
        }
        board[x][y] = ch;
    }
};

class Solution2
{
public:
    vector<string> findWords(vector<vector<char>> &board, vector<string> &words)
    {
        TrieNode *root = new TrieNode();
        for (const string &word : words) insertWord(root, word);

        vector<string> ans;
        int m = static_cast<int>(board.size());
        int n = static_cast<int>(board[0].size());
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j) dfs(board, i, j, root, ans);

        delete root;
        return ans;
    }

private:
    void dfs(vector<vector<char>> &board, int x, int y, TrieNode *node,
             vector<string> &ans)
    {
        char ch = board[x][y];
        int d = ch - 'a';
        TrieNode *next = node->child[d];
        if (next == nullptr) return;

        if (!next->word.empty())
        {
            ans.push_back(next->word);
            // 清空终止标记，保证这个单词只被收集一次
            next->word.clear();
        }

        // 已经没有更长的单词挂在这个节点下，整条分支可以摘掉
        if (!hasChild(next))
        {
            node->child[d] = nullptr;
            delete next;
            return;
        }

        board[x][y] = '#';
        for (int k = 0; k < 4; ++k)
        {
            int nx = x + DX[k];
            int ny = y + DY[k];
            if (nx < 0 || nx >= static_cast<int>(board.size())) continue;
            if (ny < 0 || ny >= static_cast<int>(board[0].size())) continue;
            if (board[nx][ny] == '#') continue;
            dfs(board, nx, ny, next, ans);
        }
        board[x][y] = ch;

        // 递归中可能把 next 的孩子全部摘空，返回后补一次清理
        if (!hasChild(next))
        {
            node->child[d] = nullptr;
            delete next;
        }
    }
};

namespace
{
vector<string> sortedCopy(vector<string> values)
{
    sort(values.begin(), values.end());
    return values;
}

void check(const vector<vector<char>> &board, const vector<string> &words,
           const vector<string> &want)
{
    vector<vector<char>> a = board;
    vector<string> wa = words;
    assert(sortedCopy(Solution1().findWords(a, wa)) == sortedCopy(want));

    vector<vector<char>> b = board;
    vector<string> wb = words;
    assert(sortedCopy(Solution2().findWords(b, wb)) == sortedCopy(want));
}
} // namespace

int main()
{
    // 示例 1
    check({{'o', 'a', 'a', 'n'}, {'e', 't', 'a', 'e'}, {'i', 'h', 'k', 'r'},
           {'i', 'f', 'l', 'v'}},
          {"oath", "pea", "eat", "rain"}, {"eat", "oath"});

    // 示例 2：同一个格子不能重复使用，"abcb" 不可能
    check({{'a', 'b'}, {'c', 'd'}}, {"abcb"}, {});

    // 边界：单格网格
    check({{'a'}}, {"a"}, {"a"});

    // 边界：单词比可走路径长
    check({{'a', 'b'}, {'c', 'd'}}, {"abcde"}, {});

    // 边界：短词是长词的前缀，三个都要输出（2x2 可以走出 3 格 L 形）
    check({{'a', 'a'}, {'a', 'a'}}, {"a", "aa", "aaa"}, {"a", "aa", "aaa"});

    // 边界：多条路径拼出同一个词，结果只能出现一次
    check({{'a', 'a'}, {'a', 'a'}}, {"aa"}, {"aa"});

    // 边界：拐弯路径，验证四个方向都走到
    check({{'a', 'b'}, {'c', 'd'}}, {"ab", "ac", "abd", "dca"},
          {"ab", "ac", "abd", "dca"});

    // 边界：单词在网格里完全不存在
    check({{'a', 'b'}, {'c', 'd'}}, {"xy", "zz"}, {});

    // 边界：3x3 全 'a'，验证前缀词与长词可以同时命中
    check({{'a', 'a', 'a'}, {'a', 'a', 'a'}, {'a', 'a', 'a'}},
          {"a", "aa", "aaaa", "aaaaaaaaaa"}, {"a", "aa", "aaaa"});

    cout << "all tests passed" << endl;
    return 0;
}
