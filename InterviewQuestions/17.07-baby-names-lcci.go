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
package main

import (
	"fmt"
	"reflect"
	"strconv"
	"strings"
)

// names 的元素形如 "John(15)"，从最后一个 '(' 切分
func parseName(s string) (string, int64) {
	lp := strings.LastIndexByte(s, '(')
	rp := strings.LastIndexByte(s, ')')
	freq, _ := strconv.ParseInt(strings.TrimSpace(s[lp+1:rp]), 10, 64)
	return strings.TrimSpace(s[:lp]), freq
}

// synonyms 的元素形如 "(Jon,John)"，去掉括号后按唯一的逗号切分
func parseSynonym(s string) (string, string) {
	lp := strings.IndexByte(s, '(')
	rp := strings.LastIndexByte(s, ')')
	inner := s[lp+1 : rp]
	comma := strings.IndexByte(inner, ',')
	return strings.TrimSpace(inner[:comma]), strings.TrimSpace(inner[comma+1:])
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

Go 里平台函数必须放在顶层，并查集用结构体承载。
*/

type babyDSU struct {
	id     map[string]int
	name   []string
	parent []int
	size   []int
	total  []int64
	best   []string
}

func newBabyDSU() *babyDSU {
	return &babyDSU{id: make(map[string]int)}
}

func (d *babyDSU) idOf(s string) int {
	if idx, ok := d.id[s]; ok {
		return idx
	}
	idx := len(d.name)
	d.id[s] = idx
	d.name = append(d.name, s)
	d.parent = append(d.parent, idx)
	d.size = append(d.size, 1)
	d.total = append(d.total, 0)
	d.best = append(d.best, s)
	return idx
}

func (d *babyDSU) find(x int) int {
	// 路径减半：把节点挂到祖父上，摊还代价接近常数
	for d.parent[x] != x {
		d.parent[x] = d.parent[d.parent[x]]
		x = d.parent[x]
	}
	return x
}

func (d *babyDSU) unite(a, b int) {
	a = d.find(a)
	b = d.find(b)
	if a == b {
		return
	}

	// 结构上按大小合并，保证树高为 O(log n)，配合路径压缩达到 α
	if d.size[a] < d.size[b] {
		a, b = b, a
	}
	d.parent[b] = a
	d.size[a] += d.size[b]
	d.total[a] += d.total[b]
	// 代表名独立于结构根：只保留字典序最小，与合并方向无关
	if d.best[b] < d.best[a] {
		d.best[a] = d.best[b]
	}
}

func trulyMostPopular(names []string, synonyms []string) []string {
	dsu := newBabyDSU()
	type parsedName struct {
		name string
		freq int64
	}
	parsed := make([]parsedName, 0, len(names))

	for _, s := range names {
		name, freq := parseName(s)
		idx := dsu.idOf(name)
		dsu.total[idx] += freq
		parsed = append(parsed, parsedName{name: name, freq: freq})
	}

	for _, s := range synonyms {
		a, b := parseSynonym(s)
		dsu.unite(dsu.idOf(a), dsu.idOf(b))
	}

	result := []string{}
	emitted := map[int]bool{}
	for _, entry := range parsed {
		root := dsu.find(dsu.id[entry.name])
		if emitted[root] {
			continue
		}
		emitted[root] = true
		result = append(result,
			dsu.best[root]+"("+strconv.FormatInt(dsu.total[root], 10)+")")
	}
	return result
}

// ===== 测试辅助：比较完整结果，并覆盖顺序稳定与频率累加 =====
func check(names, synonyms, want []string) {
	got := trulyMostPopular(names, synonyms)
	if !reflect.DeepEqual(got, want) {
		panic(fmt.Sprintf("trulyMostPopular(%v, %v) = %v, want %v",
			names, synonyms, got, want))
	}
}

func main() {
	// 示例：Christopher 频率更高，但代表名取字典序更小的 Chris
	check(
		[]string{"John(15)", "Jon(12)", "Chris(13)", "Kris(4)", "Christopher(19)"},
		[]string{
			"(Jon,John)", "(John,Johnny)",
			"(Chris,Kris)", "(Chris,Christopher)",
		},
		[]string{"John(27)", "Chris(36)"},
	)

	// 边界：没有同义词对
	check([]string{"Alice(5)"}, []string{}, []string{"Alice(5)"})

	// 边界：空输入
	check([]string{}, []string{}, []string{})

	// 边界：传递关系必须把三个名字并成一个分量
	check([]string{"A(1)", "B(2)", "C(3)"}, []string{"(A,B)", "(B,C)"},
		[]string{"A(6)"})

	// 反直觉：字典序胜过频率，不能按频率选代表名
	check([]string{"Chris(13)", "Christopher(19)"},
		[]string{"(Chris,Christopher)"}, []string{"Chris(32)"})

	// 反直觉：只出现在 synonyms 里的名字也可能成为代表名
	check([]string{"John(15)"}, []string{"(John,Aaron)"},
		[]string{"Aaron(15)"})

	// 边界：多个分量按 names 首次出现的顺序输出
	check([]string{"Zed(1)", "Amy(2)", "Bob(3)"}, []string{"(Amy,Bob)"},
		[]string{"Zed(1)", "Amy(5)"})

	// 边界：重复与反向的同义词对不应重复计数
	check([]string{"A(1)", "B(2)"}, []string{"(A,B)", "(B,A)"},
		[]string{"A(3)"})

	// 边界：频率之和超过 32 位整数范围
	check([]string{"Big(1000000000)", "Bigger(1000000000)"},
		[]string{"(Big,Bigger)"}, []string{"Big(2000000000)"})

	// 边界：同义词两侧带空格，解析时要先 trim
	check([]string{"John(15)", "Jon(12)"}, []string{"( Jon , John )"},
		[]string{"John(27)"})

	// 边界：未公布的名字作为中间桥梁
	check([]string{"Mike(5)", "John(10)"},
		[]string{"(Mike,Johnny)", "(Johnny,John)"}, []string{"John(15)"})

	// 边界：同名重复出现时频率累加，只输出一行
	check([]string{"Sam(2)", "Sam(3)"}, []string{}, []string{"Sam(5)"})

	fmt.Println("all tests passed")
}
