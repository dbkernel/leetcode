/*
============================================================================
面试题 16.02. 单词频率（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/words-frequency-lcci/
分类：哈希表 · 设计（词频统计）

设计一个方法，找出任意指定单词在一本书中的出现频率。

你的实现应该支持如下操作：

- WordsFrequency(book)构造函数，参数为字符串数组构成的一本书
- get(word)查询指定单词在书中出现的频率

示例：
WordsFrequency wordsFrequency = new WordsFrequency(
    {"i", "have", "an", "apple", "he", "have", "a", "pen"});
wordsFrequency.get("you"); //返回0，"you"没有出现过
wordsFrequency.get("have"); //返回2，"have"出现2次
wordsFrequency.get("an"); //返回1
wordsFrequency.get("apple"); //返回1
wordsFrequency.get("pen"); //返回1

提示：
- book[i]中只包含小写字母
- 1 <= book.length <= 100000
- 1 <= book[i].length <= 10
- get函数的调用次数不会超过100000
*/
#include <cassert>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

/*
解法：哈希表计数（构造时一次遍历）【最优解】

接口契约：
  · 构造函数接收整本书，构造完成后不再读取 book
  · get(word) 返回 word 在书中出现的次数，没出现返回 0
  · 查询不修改任何状态，可以被反复调用

数据结构选型：
  · 哈希表：构造 O(n)，get 平均 O(1)，空间 O(m)，m 为不同单词数
  · 字典树：get O(L)，L 为单词长度（<=10），空间 O(总字符数)；
    查询渐进复杂度比哈希表差，本题也不需要前缀检索
  · 排序 + 二分：构造 O(n log n)，get O(log n)，还要额外保存单词
  因此哈希表在时间和实现复杂度上都占优。

关键点：
  · 构造时对每个单词计数，重复单词累加
  · get 只查不插：不能用 operator[]，否则未出现的单词会被插进表里，
    查询次数一多，表会无谓地膨胀
  · 用 find 判断是否命中，未命中返回 0
  · 单词只含小写字母，长度最多 10，按字符串哈希的代价很小

手推一遍（示例）：
  book 中 have 出现 2 次，an/apple/pen 各 1 次，you 没有出现
  get("you")=0，get("have")=2，get("an")=1，
  get("apple")=1，get("pen")=1 ✔

复杂度分析：
  构造：时间 O(n)，空间 O(m)
  get：平均时间 O(1)，最坏 O(m)（哈希冲突）
*/
class WordsFrequency
{
public:
    WordsFrequency(vector<string> &book)
    {
        for (const string &word : book) ++count_[word];
    }

    int get(string word)
    {
        // 只查不插：operator[] 会把未命中的词写进表里
        auto it = count_.find(word);
        return it == count_.end() ? 0 : it->second;
    }

private:
    unordered_map<string, int> count_;
};

namespace
{
// 按题目给出的调用序列回放：先构造，再依次 get 并核对返回值
void check(const vector<string> &book, const vector<string> &queries,
           const vector<int> &want)
{
    vector<string> input = book;
    WordsFrequency wf(input);
    assert(queries.size() == want.size());
    for (size_t i = 0; i < queries.size(); ++i)
        assert(wf.get(queries[i]) == want[i]);
}

} // namespace

int main()
{
    // 示例：构造函数 + 五次 get
    check({"i", "have", "an", "apple", "he", "have", "a", "pen"},
          {"you", "have", "an", "apple", "pen"},
          {0, 2, 1, 1, 1});

    // 边界：空书（题面保证至少 1 个单词，这里只验证健壮性）
    check({}, {"a", ""}, {0, 0});

    // 边界：只有一个单词，重复查询与未命中交替
    check({"apple"}, {"apple", "app", "apple", ""},
          {1, 0, 1, 0});

    // 边界：同一单词出现多次
    check({"a", "a", "a", "b", "b", "c"},
          {"a", "b", "c", "d"}, {3, 2, 1, 0});

    // 规模：1000 个单词，a/b/c 分别 500/300/200 次
    {
        vector<string> book;
        for (int i = 0; i < 500; ++i) book.push_back("a");
        for (int i = 0; i < 300; ++i) book.push_back("b");
        for (int i = 0; i < 200; ++i) book.push_back("c");
        check(book, {"a", "b", "c", "d"}, {500, 300, 200, 0});
    }

    // 边界：查询未出现的单词后，再查已出现的单词仍然正确
    check({"hello", "world", "hello"},
          {"absent", "hello", "absent", "world"},
          {0, 2, 0, 1});

    cout << "all tests passed" << endl;
    return 0;
}
