/*
============================================================================
字典树（Trie / 前缀树）
分类：手写数据结构 · 树
操作复杂度：
  insert       O(L)   L 为单词长度，沿字符一路向下建节点
  search       O(L)   整词：走完 L 步后看 end > 0
  starts_with  O(L)   前缀：走完 L 步只要节点存在即可
  count_prefix O(L)   返回该节点被多少单词经过
  count_word   O(L)
  erase        O(L)   沿路径 pass 减一，可选自底向上剪枝
  空间 O(总字符数 * 26)；共享前缀的单词越多越省
============================================================================

用途：
  前缀匹配、自动补全、敏感词过滤、单词搜索题的加速结构。
  面试常考：实现 Trie、前缀计数、支持重复插入与删除。

核心思想：
  把字符串按字符拆成一条路径，根到某节点的路径就是一个前缀。每个
  节点用 26 个槽位指向子节点，节点上记两个计数：pass 表示有多少个
  单词经过这里，end 表示有多少个单词在这里结束。查询时沿路径走，
  走不动就说明没这个前缀。

关键点（不变量）：
  · 用计数而不是 bool：同一个单词插入两次，end 就是 2；删除一次只
    减一，还能查到。用 bool 就无法区分「插入过一次」和「插入过三次」，
    删一次就没了，重复插入的信息直接丢失；
  · pass 与 end 的含义不能混：search 看 end，starts_with 看 pass
    （节点存在即 pass > 0），否则 "app" 会被误判成 "apple" 的整词；
  · 删除必须先确认 count_word > 0，否则会把不存在的路径减成负数；
  · 剪枝只能自底向上，且一旦某层 pass > 0 就必须停：上层还有别的
    单词经过，继续剪会把它们一起删掉。

手推一遍：
  依次插入 app(1 次)、apple(2 次)：
  1. 路径 a-p-p 上每个节点 pass 都加 1，最后把 p 的 end 加 1；
  2. 再插两次 apple：a、p、p、l、e 的 pass 各加 2，e 的 end 变 2；
     此时 root.pass=3、app 节点 pass=3/end=1、e 节点 pass=2/end=2；
  3. count_prefix("app") 返回 3，count_prefix("appl") 返回 2，
     search("app") 为 true，search("appl") 为 false；
  4. erase("app") 只把路径上 pass 各减一、end 减到 0，因为两个
     apple 还经过这层，pass 仍是 2，不触发剪枝。

复杂度分析：
  时间 O(L)：每次操作只走单词长度条边，与树里已有多少单词无关。
  空间 O(总字符数 * 26)：26 叉数组是定长开销，前缀共享能显著省空间。
*/
package main

import "fmt"

type trieNode struct {
	child [26]*trieNode // 空指针表示没有这条边
	pass  int           // 有多少个单词经过这个节点
	end   int           // 有多少个单词在这个节点结束
}

type trie struct {
	root  *trieNode
	words int // 单词总数，含重复
}

func newTrie() *trie { return &trie{root: &trieNode{}} }

// 只接受非空的全小写单词
func validWord(w string) bool {
	if len(w) == 0 {
		return false
	}
	for i := 0; i < len(w); i++ {
		if w[i] < 'a' || w[i] > 'z' {
			return false
		}
	}
	return true
}

func (t *trie) insert(w string) bool {
	if !validWord(w) {
		return false
	}
	cur := t.root
	cur.pass++ // 根节点也要记：空前缀的计数靠它
	for i := 0; i < len(w); i++ {
		c := int(w[i] - 'a')
		if cur.child[c] == nil {
			cur.child[c] = &trieNode{}
		}
		cur = cur.child[c]
		cur.pass++
	}
	cur.end++
	t.words++
	return true
}

// 整词查询：必须走完且该节点有单词结束
func (t *trie) search(w string) bool {
	if !validWord(w) {
		return false
	}
	n := t.walk(w)
	return n != nil && n.end > 0
}

// 前缀查询：只要路径走得通就算存在
func (t *trie) startsWith(p string) bool {
	if len(p) == 0 {
		return t.words > 0
	}
	if !validWord(p) {
		return false
	}
	return t.walk(p) != nil
}

// 有多少个单词以 p 为前缀；空前缀返回单词总数
func (t *trie) countPrefix(p string) int {
	if len(p) == 0 {
		return t.words
	}
	if !validWord(p) {
		return 0
	}
	n := t.walk(p)
	if n == nil {
		return 0
	}
	return n.pass
}

// 该词被插入了几次
func (t *trie) countWord(w string) int {
	if !validWord(w) {
		return 0
	}
	n := t.walk(w)
	if n == nil {
		return 0
	}
	return n.end
}

// 删除一次出现；没插过则返回 false
func (t *trie) erase(w string) bool {
	if !validWord(w) || t.countWord(w) == 0 {
		return false
	}
	cur := t.root
	// 记下整条路径，方便自底向上剪枝
	nodes := []*trieNode{cur}
	letters := []int{}
	cur.pass--
	for i := 0; i < len(w); i++ {
		c := int(w[i] - 'a')
		cur = cur.child[c]
		cur.pass--
		nodes = append(nodes, cur)
		letters = append(letters, c)
	}
	cur.end--
	t.words--
	for i := len(w) - 1; i >= 0; i-- {
		if nodes[i+1].pass == 0 {
			// 没有单词再经过这个节点了，剪掉它
			nodes[i].child[letters[i]] = nil
		} else {
			break // 上层还有别的单词，继续剪会误伤
		}
	}
	return true
}

func (t *trie) walk(w string) *trieNode {
	cur := t.root
	for i := 0; i < len(w); i++ {
		c := int(w[i] - 'a')
		if cur.child[c] == nil {
			return nil
		}
		cur = cur.child[c]
	}
	return cur
}

func check(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
}

func main() {
	// 1. 插入 / 整词查询 / 前缀查询 / 前缀计数
	{
		t := newTrie()
		check(t.insert("apple"), "insert apple")
		check(t.insert("app"), "insert app")
		check(t.insert("apply"), "insert apply")
		check(t.search("apple"), "search apple")
		check(t.search("app"), "search app")
		check(t.startsWith("app"), "prefix app")
		check(t.startsWith("appl"), "prefix appl")
		check(t.countPrefix("app") == 3, "count app")
		check(t.countPrefix("appl") == 2, "count appl")
		check(t.countPrefix("a") == 3, "count a")
		check(t.countWord("app") == 1, "count word app")
		// 前缀存在但整个词不存在
		check(!t.search("appl"), "appl is not a word")
		check(!t.search("ap"), "ap is not a word")
		check(!t.startsWith("b"), "no b prefix")
		fmt.Println("prefix app count:", t.countPrefix("app"))
	}

	// 2. 重复插入：计数累加，search 只看 end > 0
	{
		t := newTrie()
		check(t.insert("abc"), "insert 1")
		check(t.insert("abc"), "insert 2")
		check(t.insert("abc"), "insert 3")
		check(t.countWord("abc") == 3, "count word 3")
		check(t.countPrefix("ab") == 3, "count prefix 3")
		check(t.countPrefix("abc") == 3, "count prefix abc")
		check(t.search("abc"), "search abc")
		// 删一次还有两次
		check(t.erase("abc"), "erase 1")
		check(t.countWord("abc") == 2, "count word 2")
		check(t.search("abc"), "still found")
		check(t.countPrefix("ab") == 2, "count prefix 2")
		check(t.erase("abc"), "erase 2")
		check(t.erase("abc"), "erase 3")
		check(!t.search("abc"), "gone")
		check(!t.startsWith("ab"), "path pruned")
		check(t.countPrefix("ab") == 0, "count 0")
		check(t.countWord("abc") == 0, "count word 0")
		// 删光了再删一次必须失败
		check(!t.erase("abc"), "erase after empty")
	}

	// 3. 空前缀：存在的口径是「树里还有单词」
	{
		t := newTrie()
		check(!t.startsWith(""), "empty prefix empty tree")
		check(t.countPrefix("") == 0, "empty prefix count")
		check(!t.search(""), "empty word")
		t.insert("x")
		check(t.startsWith(""), "empty prefix non empty tree")
		check(t.countPrefix("") == 1, "empty prefix count 1")
		check(!t.search(""), "empty word still absent")
		t.erase("x")
		check(!t.startsWith(""), "empty prefix after erase")
		check(t.countPrefix("") == 0, "empty prefix count after erase")
	}

	// 4. 非小写字母：一律拒绝
	{
		t := newTrie()
		check(!t.insert("Abc"), "reject Abc")
		check(!t.insert("aBc"), "reject aBc")
		check(!t.insert("ab1"), "reject ab1")
		check(!t.insert(""), "reject empty")
		check(!t.search("Abc"), "search Abc")
		check(!t.startsWith("aB"), "prefix aB")
		check(t.countPrefix("a1") == 0, "count a1")
		check(!t.erase("a b"), "erase with space")
		check(t.countPrefix("") == 0, "still empty")
	}

	// 5. 空树删除 / 删除不存在的词
	{
		t := newTrie()
		check(!t.erase("a"), "erase from empty")
		check(!t.search("a"), "search empty trie")
		check(t.insert("ab"), "insert ab")
		check(!t.erase("a"), "erase prefix only")
		check(t.search("ab"), "ab kept")
		check(!t.erase("abc"), "erase missing word")
		check(t.countWord("ab") == 1, "ab count kept")
	}

	// 6. 剪枝后仍可复用同一层
	{
		t := newTrie()
		check(t.insert("ab"), "insert ab")
		check(t.erase("ab"), "erase ab")
		check(!t.startsWith("a"), "a pruned")
		check(t.insert("ac"), "insert ac")
		check(t.search("ac"), "search ac")
		check(!t.search("ab"), "ab gone")
		check(t.countPrefix("a") == 1, "count a 1")
	}

	fmt.Println("all tests passed")
}
