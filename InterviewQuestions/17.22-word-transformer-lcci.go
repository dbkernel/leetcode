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
package main

import "fmt"

// 沿 parent 表从 begin 走到 end，再反转成起点在前的序列
func buildPath(parent map[string]string, begin, end string) []string {
	path := []string{}
	for cur := end; cur != begin; cur = parent[cur] {
		path = append(path, cur)
	}
	path = append(path, begin)
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}
	return path
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

Go 里平台函数必须放在顶层，多解法用不同函数名区分：
findLaddersBFS 对应解法一，findLaddersBidirectional 对应解法二。
*/

func findLaddersBFS(
	beginWord string, endWord string, wordList []string,
) []string {
	dict := make(map[string]bool, len(wordList))
	for _, word := range wordList {
		dict[word] = true
	}

	// 示例 2：终点不在字典里，任何序列都不成立
	if !dict[endWord] {
		return []string{}
	}
	// 起点即终点时，零步序列本身就是合法答案
	if beginWord == endWord {
		return []string{beginWord}
	}

	parent := map[string]string{beginWord: ""}
	queue := []string{beginWord}

	for head := 0; head < len(queue); head++ {
		word := queue[head]
		b := []byte(word)
		for i := range b {
			old := b[i]
			for c := byte('a'); c <= 'z'; c++ {
				if c == old {
					continue
				}
				b[i] = c
				next := string(b)

				// 不在字典里的词不能作为中间步骤
				if !dict[next] {
					continue
				}
				// 已经入队过的词不再改写 parent，保证同一层不重复
				if _, seen := parent[next]; seen {
					continue
				}

				parent[next] = word
				if next == endWord {
					return buildPath(parent, beginWord, endWord)
				}
				queue = append(queue, next)
			}
			// b 是复用的缓冲区，本位置试完后恢复
			b[i] = old
		}
	}

	return []string{}
}

func findLaddersBidirectional(
	beginWord string, endWord string, wordList []string,
) []string {
	dict := make(map[string]bool, len(wordList))
	for _, word := range wordList {
		dict[word] = true
	}

	if !dict[endWord] {
		return []string{}
	}
	if beginWord == endWord {
		return []string{beginWord}
	}

	parentA := map[string]string{beginWord: ""}
	parentB := map[string]string{endWord: ""}
	frontA := map[string]bool{beginWord: true}
	frontB := map[string]bool{endWord: true}

	for len(frontA) > 0 && len(frontB) > 0 {
		// 扩较小的一侧，让两边的深度尽量同步
		expandA := len(frontA) <= len(frontB)
		front, parent, other := frontA, parentA, parentB
		if !expandA {
			front, parent, other = frontB, parentB, parentA
		}

		nextFront := make(map[string]bool)
		for word := range front {
			b := []byte(word)
			for i := range b {
				old := b[i]
				for c := byte('a'); c <= 'z'; c++ {
					if c == old {
						continue
					}
					b[i] = c
					next := string(b)

					if !dict[next] {
						continue
					}
					if _, seen := parent[next]; seen {
						continue
					}

					parent[next] = word
					if _, hit := other[next]; hit {
						return joinLadders(parentA, parentB, beginWord, endWord, next)
					}
					nextFront[next] = true
				}
				b[i] = old
			}
		}

		if expandA {
			frontA = nextFront
		} else {
			frontB = nextFront
		}
	}

	return []string{}
}

func joinLadders(
	parentA, parentB map[string]string, begin, end, meet string,
) []string {
	// 起点侧：从相遇点沿 parent 走到 begin，再反转
	path := []string{}
	for cur := meet; cur != begin; cur = parentA[cur] {
		path = append(path, cur)
	}
	path = append(path, begin)
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}

	// 终点侧：相遇点已经写入，从它的下一个词接到 end
	for cur := meet; cur != end; {
		cur = parentB[cur]
		path = append(path, cur)
	}
	return path
}

// ===== 测试辅助：校验序列是否合法，而不是比对某一条固定路径 =====
func oneCharApart(a, b string) bool {
	diff := 0
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			diff++
			// 第二个不同字符出现即可判定失败
			if diff > 1 {
				return false
			}
		}
	}
	return diff == 1
}

func validPath(path []string, begin, end string, wordList []string) bool {
	if len(path) == 0 {
		return false
	}
	if path[0] != begin || path[len(path)-1] != end {
		return false
	}

	dict := make(map[string]bool, len(wordList))
	for _, word := range wordList {
		dict[word] = true
	}
	// 起点不需要在字典里，之后每一步都必须是字典中的词
	for i := 1; i < len(path); i++ {
		if !oneCharApart(path[i-1], path[i]) || !dict[path[i]] {
			return false
		}
	}
	return true
}

func check(begin, end string, wordList []string, hasPath bool) {
	gotBFS := findLaddersBFS(begin, end, wordList)
	if hasPath {
		if !validPath(gotBFS, begin, end, wordList) {
			panic(fmt.Sprintf("bfs(%s, %s) = %v, invalid path", begin, end, gotBFS))
		}
	} else if len(gotBFS) != 0 {
		panic(fmt.Sprintf("bfs(%s, %s) = %v, want empty", begin, end, gotBFS))
	}

	gotBi := findLaddersBidirectional(begin, end, wordList)
	if hasPath {
		if !validPath(gotBi, begin, end, wordList) {
			panic(fmt.Sprintf(
				"bidirectional(%s, %s) = %v, invalid path",
				begin, end, gotBi,
			))
		}
	} else if len(gotBi) != 0 {
		panic(fmt.Sprintf(
			"bidirectional(%s, %s) = %v, want empty",
			begin, end, gotBi,
		))
	}
}

func main() {
	// 示例 1：多条合法路径，只校验合法性
	check("hit", "cog", []string{"hot", "dot", "dog", "lot", "log", "cog"}, true)

	// 示例 2：endWord 不在字典中
	check("hit", "cog", []string{"hot", "dot", "dog", "lot", "log"}, false)

	// 边界：起点即终点，零步序列
	check("hot", "hot", []string{"hot"}, true)

	// 边界：字典里只有终点，和起点差多个字符
	check("hit", "cog", []string{"cog"}, false)

	// 边界：单字符词，a -> b -> c
	check("a", "c", []string{"b", "c"}, true)

	// 边界：需要连续改四个位置
	check("aaaa", "dddd", []string{"aaad", "aadd", "addd", "dddd"}, true)

	// 反直觉：存在多条等长路径，不能假定某一条固定顺序
	check("ab", "cd", []string{"ac", "ad", "cc", "cd"}, true)

	// 反直觉：终点可达但必须经过中间词，不能跳到 "abcd"
	check("ab", "cd", []string{"abcd", "cd"}, false)

	// 边界：词表中混入与起点相同的词，不应影响 parent
	check("hit", "cog", []string{"hit", "hot", "dot", "dog", "cog"}, true)

	// 边界：26 个单字母词构成的完全图，任意两点一步可达
	letters := make([]string, 0, 26)
	for c := byte('a'); c <= 'z'; c++ {
		letters = append(letters, string(c))
	}
	check("a", "z", letters, true)

	fmt.Println("all tests passed")
}
