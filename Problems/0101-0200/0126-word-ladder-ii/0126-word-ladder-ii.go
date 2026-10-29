/*
============================================================================
126. 单词接龙 II（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/word-ladder-ii/
分类：广度优先搜索 · 分层图 + 回溯

按字典 wordList 完成从单词 beginWord 到单词 endWord 转化，一个表示此
过程的 转换序列 是形式上像 beginWord -> s1 -> s2 -> ... -> sk 这样的
单词序列，并满足：

- 每对相邻的单词之间仅有单个字母不同。
- 转换过程中的每个单词 si（1 <= i <= k）必须是字典 wordList 中的单词。
  注意，beginWord 不必是字典 wordList 中的单词。
- sk == endWord

给你两个单词 beginWord 和 endWord ，以及一个字典 wordList 。请你找出
并返回所有从 beginWord 到 endWord 的 最短转换序列 ，如果不存在这样的
转换序列，返回一个空列表。每个序列都应该以单词列表
[beginWord, s1, s2, ..., sk] 的形式返回。

示例 1：
输入：beginWord = "hit", endWord = "cog",
      wordList = ["hot","dot","dog","lot","log","cog"]
输出：[["hit","hot","dot","dog","cog"],
       ["hit","hot","lot","log","cog"]]
解释：存在 2 种最短的转换序列：
"hit" -> "hot" -> "dot" -> "dog" -> "cog"
"hit" -> "hot" -> "lot" -> "log" -> "cog"

示例 2：
输入：beginWord = "hit", endWord = "cog",
      wordList = ["hot","dot","dog","lot","log"]
输出：[]
解释：endWord "cog" 不在字典 wordList 中，所以不存在符合要求的转换序列。

提示：
- 1 <= beginWord.length <= 5
- endWord.length == beginWord.length
- 1 <= wordList.length <= 500
- wordList[i].length == beginWord.length
- beginWord、endWord 和 wordList[i] 由小写英文字母组成
- beginWord != endWord
- wordList 中的所有字符串 互不相同
*/
package main

import (
	"fmt"
	"sort"
	"strings"
)

/*
解法：BFS 建分层 DAG + DFS 回溯所有最短路径【最优解】

为什么慢：最直观的做法是队列里直接存"整条路径"，每扩展一步复制一次。一条
路径要被复制 O(L) 次，队列里还会堆满走不到 endWord 的死路前缀，这些前缀的
数量随层数指数增长，时间与空间都被输出之外的中间状态吃掉；而本题要输出的
只是最短路径本身。

换个角度：答案由两部分信息决定——"哪些边位于最短路上"和"这些边怎么串起来"。
前者一次 BFS 分层就能定下：按到 beginWord 的距离给单词分层，只保留从第 k 层
指向第 k+1 层的边，得到一张 DAG；后者在 DAG 上从 beginWord 做一次 DFS，
每条能到 endWord 的路径必然是最短路径，且只需要走一遍。

为什么必须先 BFS 再 DFS：单独的 DFS 无法判断"当前这条路已经绕远了"，
只能把整张图走完再比长度，指数级的路径根本枚举不完；BFS 分层天然给出每个
单词的最短层号，DAG 上的所有路径等长，DFS 只做枚举、不做比较。

关键点：
  · 一层里发现的词不能立刻从字典删除 —— 同层其他词仍可能连到它，提前删就
    丢掉"一个词有多个同层前驱"的那些路径（例如 ted、rex 同层，都通向 tex）。
    必须整层扫完后统一删除本层发现的词。
  · 记录边时只认"邻居还在字典里"：已被更早层用过的词已经短了一步，连过去
    只会得到非最短路径。
  · 命中 endWord 也要记这条边，但不要把 endWord 放进下一层继续展开，
    否则路径会被继续延长成非最短。
  · 用 `reached` 标记而非 break：同一层里其他词可能也能连到 endWord，
    要跑完本层才算收集齐所有最短路径。
  · Go 的 DFS 用闭包递归，存入答案前必须复制一份 path，否则后续回溯会把
    已保存的路径改掉

手推一遍：
以示例 1（hit → cog）为例：
  第 0 层 {hit}：发现 hot
  第 1 层 {hot}：发现 dot、lot
  第 2 层 {dot, lot}：分别发现 dog、log
  第 3 层 {dog, log}：都发现 cog，reached = true，本层扫完即停
  DAG 为 hit→hot→{dot,lot}→{dog,log}→cog，DFS 走出两条路径 ✔
  若在第 2 层发现 dog 时立刻把 dog 删出字典，同层的 lot 就再也连不上它，
  示例 1 的两条路径只剩一条。

复杂度分析：
  时间 O(N·L·26 + 输出字符数)：每个单词只在自己那一层展开一次，DFS 对每条
    路径只走一遍
  空间 O(N·L + 输出字符数)：DAG 的边只来自真实命中的邻居，递归栈深度不超过
    最短路径的单词数
*/
func findLadders(beginWord string, endWord string,
	wordList []string) [][]string {
	dict := make(map[string]bool, len(wordList))
	for _, w := range wordList {
		dict[w] = true
	}
	// 终点不在字典里，连一条边都建不出来
	if !dict[endWord] {
		return [][]string{}
	}

	// beginWord 不属于任何一层，删掉后"绕回起点"的边自然被排除
	delete(dict, beginWord)

	// next[w]：分层 DAG 中从 w 指向下一层的全部单词
	next := make(map[string][]string)
	layer := map[string]bool{beginWord: true}
	reached := false

	for len(layer) > 0 && !reached {
		// 本层新发现的词先攒着，层末才从字典里删
		var discovered []string
		nextLayer := make(map[string]bool)

		for word := range layer {
			b := []byte(word)
			for pos := 0; pos < len(b); pos++ {
				origin := b[pos]
				for c := byte('a'); c <= 'z'; c++ {
					if c == origin {
						continue
					}
					b[pos] = c
					s := string(b)

					// 不在字典里：更早的层用过，或就是 beginWord
					if !dict[s] {
						continue
					}

					// 命中终点也要留边，只是不再从它往下展开
					next[word] = append(next[word], s)
					if s == endWord {
						reached = true
					}

					// 同层第二次发现同一个词时仍要留边，这正是多路径的来源
					if !nextLayer[s] {
						nextLayer[s] = true
						discovered = append(discovered, s)
					}
				}
				b[pos] = origin
			}
		}

		// 整层扫完才删：本层后面的词仍需要看到这些同层邻居
		for _, w := range discovered {
			delete(dict, w)
		}
		layer = nextLayer
	}

	// 终点在字典里却始终没被连上，说明不连通
	if !reached {
		return [][]string{}
	}

	ans := [][]string{}
	path := []string{beginWord}

	// 在分层 DAG 上枚举：凡是能走到 endWord 的路径都恰好是最短路径
	var dfs func(w string)
	dfs = func(w string) {
		if w == endWord {
			snapshot := make([]string, len(path))
			copy(snapshot, path)
			ans = append(ans, snapshot)
			return
		}
		for _, nxt := range next[w] {
			path = append(path, nxt)
			dfs(nxt)
			path = path[:len(path)-1]
		}
	}
	dfs(beginWord)

	return ans
}

// 题面不承诺路径间的顺序，统一转成"每条路径按字典序排好、再拼成一个串"
func canonical126(paths [][]string) string {
	lines := make([]string, 0, len(paths))
	for _, p := range paths {
		lines = append(lines, strings.Join(p, ">"))
	}
	sort.Strings(lines)
	return strings.Join(lines, "|")
}

func check126(beginWord string, endWord string, wordList []string,
	want [][]string) {
	got := canonical126(findLadders(beginWord, endWord, wordList))
	expect := canonical126(want)
	if got != expect {
		panic(fmt.Sprintf("findLadders(%s -> %s)\n got: %s\nwant: %s",
			beginWord, endWord, got, expect))
	}
}

func main() {
	// 示例 1：两条最短路径，长度都是 5
	check126("hit", "cog", []string{"hot", "dot", "dog", "lot", "log", "cog"},
		[][]string{
			{"hit", "hot", "dot", "dog", "cog"},
			{"hit", "hot", "lot", "log", "cog"},
		})

	// 示例 2：终点不在字典里
	check126("hit", "cog", []string{"hot", "dot", "dog", "lot", "log"},
		[][]string{})

	// 关键用例：tex 在同一个层里同时由 ted、rex 发现，两条父边都不能丢；
	// 若实现里在本层中途就删词，这里会少一条路径
	check126("red", "tax",
		[]string{"ted", "tex", "red", "tax", "tad", "den", "rex", "pee"},
		[][]string{
			{"red", "rex", "tex", "tax"},
			{"red", "ted", "tex", "tax"},
			{"red", "ted", "tad", "tax"},
		})

	// 边界：一步直达，路径只有两个单词
	check126("hot", "dot", []string{"dot"}, [][]string{{"hot", "dot"}})

	// 边界：终点在字典里但整体不连通
	check126("hit", "cog", []string{"hot", "dog", "cog"}, [][]string{})

	// 边界：唯一的一条长链（ab→ad→cd→cf→ef）
	check126("ab", "ef", []string{"ad", "cd", "cf", "ef"},
		[][]string{{"ab", "ad", "cd", "cf", "ef"}})

	fmt.Println("all tests passed")
}
