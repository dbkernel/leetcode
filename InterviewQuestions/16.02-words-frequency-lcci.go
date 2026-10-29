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
package main

import "fmt"

/*
解法：哈希表计数（构造时一次遍历）【最优解】

接口契约：
  · 构造函数接收整本书，构造完成后不再读取 book
  · Get(word) 返回 word 在书中出现的次数，没出现返回 0
  · 查询不修改任何状态，可以被反复调用

数据结构选型：
  · 哈希表：构造 O(n)，Get 平均 O(1)，空间 O(m)，m 为不同单词数
  · 字典树：Get O(L)，L 为单词长度（<=10），空间 O(总字符数)；
    查询渐进复杂度比哈希表差，本题也不需要前缀检索
  · 排序 + 二分：构造 O(n log n)，Get O(log n)，还要额外保存单词
  因此哈希表在时间和实现复杂度上都占优。

关键点：
  · 构造时对每个单词计数，重复单词累加
  · Get 只读不写：Go 读取 map 中不存在的键返回零值，不会插入新键
  · 不用先判断键是否存在，直接取值即可
  · 单词只含小写字母，长度最多 10，按字符串哈希的代价很小

手推一遍（示例）：
  book 中 have 出现 2 次，an/apple/pen 各 1 次，you 没有出现
  Get("you")=0，Get("have")=2，Get("an")=1，
  Get("apple")=1，Get("pen")=1 ✔

复杂度分析：
  构造：时间 O(n)，空间 O(m)
  Get：平均时间 O(1)，最坏 O(m)（哈希冲突）
*/
type WordsFrequency struct {
	count map[string]int
}

func Constructor(book []string) WordsFrequency {
	count := make(map[string]int, len(book))
	for _, word := range book {
		count[word]++
	}
	return WordsFrequency{count: count}
}

func (this *WordsFrequency) Get(word string) int {
	// 读取不存在的键返回 0，且不会像插入那样扩大 map
	return this.count[word]
}

func checkWordsFrequency(book, queries []string, want []int) {
	obj := Constructor(book)
	if len(queries) != len(want) {
		panic("queries and want length mismatch")
	}
	for i, q := range queries {
		if got := obj.Get(q); got != want[i] {
			panic(fmt.Sprintf("get(%q) = %d, want %d", q, got, want[i]))
		}
	}
}

func main() {
	// 示例：构造函数 + 五次 Get
	checkWordsFrequency(
		[]string{"i", "have", "an", "apple", "he", "have", "a", "pen"},
		[]string{"you", "have", "an", "apple", "pen"},
		[]int{0, 2, 1, 1, 1})

	// 边界：空书（题面保证至少 1 个单词，这里只验证健壮性）
	checkWordsFrequency([]string{}, []string{"a", ""}, []int{0, 0})

	// 边界：只有一个单词，重复查询与未命中交替
	checkWordsFrequency([]string{"apple"},
		[]string{"apple", "app", "apple", ""},
		[]int{1, 0, 1, 0})

	// 边界：同一单词出现多次
	checkWordsFrequency([]string{"a", "a", "a", "b", "b", "c"},
		[]string{"a", "b", "c", "d"}, []int{3, 2, 1, 0})

	// 规模：1000 个单词，a/b/c 分别 500/300/200 次
	book := make([]string, 0, 1000)
	for i := 0; i < 500; i++ {
		book = append(book, "a")
	}
	for i := 0; i < 300; i++ {
		book = append(book, "b")
	}
	for i := 0; i < 200; i++ {
		book = append(book, "c")
	}
	checkWordsFrequency(book, []string{"a", "b", "c", "d"},
		[]int{500, 300, 200, 0})

	// 边界：查询未出现的单词后，再查已出现的单词仍然正确
	checkWordsFrequency([]string{"hello", "world", "hello"},
		[]string{"absent", "hello", "absent", "world"},
		[]int{0, 2, 0, 1})

	fmt.Println("all tests passed")
}
