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
#include <cassert>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

/*
解法：孩子数组建树（每条边对应一个小写字母）【最优解】

接口契约：
  · insert 不要求 word 原本不在树中，重复插入同一个词必须幂等
  · search 要求整条路径存在，且终点带「词尾」标记
  · startsWith 只要求整条路径存在，不关心终点是否收尾
  · 三个操作都只读 word / prefix，不改动调用方传入的字符串

数据结构选型：
  · 每个节点存 26 个孩子指针，用 ch - 'a' 直接定位，单步 O(1)
  · 孩子数组的内存是「节点数 × 26 个指针」，稀疏时浪费明显；换成哈希表
    能省内存，但每次查找多一次哈希计算，常数更大
  · 本题字符集固定为 26 个小写字母、调用次数不超过 3 × 10^4，
    数组的常数优势更重要，因此选数组

关键点：
  · 路径上的节点可能在多次 insert 中共享，缺孩子时才新建，已有就复用
    —— 覆盖掉旧孩子会丢掉之前插入的词
  · isEnd 只在 insert 走完整个 word 后置 true，中途不停留
  · search 与 startsWith 的唯一区别就是最后是否检查 isEnd
  · 空串不会出现（长度至少 1），根节点本身不承载任何词

手推一遍：
  以示例序列为例：
  insert("apple") 建出 a → p → p → l → e，只在 e 上置 isEnd
  search("apple") 走完整条路径，终点 isEnd 为 true → true
  search("app") 路径存在，但停在 p 上，isEnd 为 false → false
  startsWith("app") 只查路径 → true
  insert("app") 复用 a → p → p，只把第二个 p 的 isEnd 置 true
  search("app") 终点 isEnd 已为 true → true

复杂度分析：
  时间 O(L)：每次操作只沿字符串长度 L 走一遍，每步 O(1)
  空间 O(总字符数 × 26)：每个新建节点固定 26 个指针，
    最坏是所有词两两没有公共前缀
*/
class Trie
{
public:
    Trie() : root(new Node()) {}

    void insert(const string &word)
    {
        Node *node = root;
        for (char ch : word)
        {
            int idx = ch - 'a';
            // 共享已有前缀：只有缺孩子时才新建
            if (node->child[idx] == nullptr)
            {
                node->child[idx] = new Node();
            }
            node = node->child[idx];
        }
        // 只有完整插入过的词尾才置 true
        node->isEnd = true;
    }

    bool search(const string &word) const
    {
        const Node *node = find(word);
        // 路径存在不代表是一个词，必须曾经在这里收尾
        return node != nullptr && node->isEnd;
    }

    bool startsWith(const string &prefix) const
    {
        // 前缀只要求路径存在，不要求收尾标记
        return find(prefix) != nullptr;
    }

private:
    struct Node
    {
        Node *child[26];
        bool isEnd;

        Node() : isEnd(false)
        {
            for (Node *&p : child)
            {
                p = nullptr;
            }
        }
    };

    Node *root;

    // 沿 s 往下走，返回终点；中途缺孩子就说明路径不存在
    const Node *find(const string &s) const
    {
        const Node *node = root;
        for (char ch : s)
        {
            node = node->child[ch - 'a'];
            if (node == nullptr) return nullptr;
        }
        return node;
    }
};

namespace
{
struct Call
{
    string op;
    string arg;
    string want; // "null" / "true" / "false"
};

// 按题目的调用序列回放；序列开头的 "Trie" 对应这里的局部对象构造
void replay(const vector<Call> &calls)
{
    Trie trie;
    for (const Call &c : calls)
    {
        if (c.op == "insert")
        {
            trie.insert(c.arg);
            assert(c.want == "null");
        }
        else if (c.op == "search")
        {
            assert((trie.search(c.arg) ? "true" : "false") == c.want);
        }
        else
        {
            assert(c.op == "startsWith");
            assert((trie.startsWith(c.arg) ? "true" : "false") == c.want);
        }
    }
}
} // namespace

int main()
{
    // 示例：按题目的调用序列逐条回放
    replay({
        {"insert", "apple", "null"},
        {"search", "apple", "true"},
        {"search", "app", "false"},
        {"startsWith", "app", "true"},
        {"insert", "app", "null"},
        {"search", "app", "true"},
    });

    // 边界：重复插入同一个词不改变语义
    replay({
        {"insert", "abc", "null"},
        {"insert", "abc", "null"},
        {"search", "abc", "true"},
        {"search", "ab", "false"},
        {"startsWith", "ab", "true"},
    });

    // 边界：查询从未插入过的首字母，路径第一步就断
    replay({
        {"insert", "a", "null"},
        {"search", "b", "false"},
        {"startsWith", "b", "false"},
    });

    // 边界：前一个词是后一个词的前缀，词尾标记不能互相污染
    replay({
        {"insert", "app", "null"},
        {"insert", "apple", "null"},
        {"search", "app", "true"},
        {"search", "appl", "false"},
        {"startsWith", "appl", "true"},
        {"search", "apples", "false"},
    });

    // 边界：26 个首字母各插一个单字符词，验证下标换算覆盖整个字母表
    {
        Trie trie;
        for (char ch = 'a'; ch <= 'z'; ++ch)
        {
            trie.insert(string(1, ch));
        }
        for (char ch = 'a'; ch <= 'z'; ++ch)
        {
            assert(trie.search(string(1, ch)));
            assert(trie.startsWith(string(1, ch)));
        }
        // 空串不是任何已插入的词（根节点没有词尾标记）
        assert(!trie.search(""));
    }

    // 边界：长度 2000 的词，验证长路径上的补建与查找都没有越界
    {
        string longWord(2000, 'x');
        Trie trie;
        trie.insert(longWord);
        assert(trie.search(longWord));
        assert(trie.startsWith(string(1999, 'x')));
        assert(!trie.search(string(1999, 'x')));
    }

    cout << "all tests passed" << endl;
    return 0;
}
