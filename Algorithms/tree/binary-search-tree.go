/*
============================================================================
二叉搜索树（Binary Search Tree）
分类：手写数据结构 · 树
操作复杂度：
  insert    O(h)  沿一条路径下降；重复值只把 cnt 加一
  find      O(h)
  erase     O(h)  双孩子要沿右子树最左一路找后继
  min/max   O(h)
  height    O(n)  递归量一遍（缓存子树高度可做到 O(1)）
  size      O(n)  递归把 cnt 累加（维护计数器可做到 O(1)）
  h 为树高，随机形态期望 O(log n)，退化成链时 O(n)
============================================================================

用途：
  「有序 + 动态插删」的最小可用结构，也是 AVL / 红黑树的原型。
  面试常考：插入、查找、删除节点（尤其双孩子）、验证 BST、第 k 小。

核心思想：
  对每个节点，左子树所有值都比它小，右子树所有值都比它大，于是查找
  每次都能砍掉一半子树。插入和删除都只沿一条从根到叶的路径走，改动
  只发生在路径上。删除双孩子节点时，把中序后继（右子树最小值）搬到
  当前位置，问题就退化成「删一个最多只有一个孩子的节点」。

关键点（不变量）：
  · 重复值策略选「计数」：相等时只把 cnt 加一，不新建节点，因此
    每个不同的值在树里恰好占一个节点，中序遍历按 cnt 重复输出；
  · 双孩子删除必须用中序后继：它是右子树最小值，天然大于左子树
    全部、小于右子树其余全部；且它没有左孩子，物理摘除退化成
    「至多一个孩子」的最简情况。随便挑一个节点会破坏 BST 有序性；
  · cnt > 1 时删除只减计数、绝不动结构，否则会把同值的其它出现
    一起删掉；
  · erase 找不到值时原树必须原样返回，不能顺手改指针。

手推一遍：
  依次插入 50 30 70 20 40 60 80，得到：

            50
          /    \
        30      70
       /  \    /  \
      20  40  60  80

  1. 插入 40 时走 50 -> 30 -> 40，40 比 30 大且 30 的右孩子为空，
     挂成 30 的右孩子；
  2. 删除 50：它有两个孩子，进右子树一路向左找到 60，把 60 搬到
     根，70 的左孩子置空；中序仍是 20 30 40 60 70 80，高度 3；
  3. 再插入一个 40：走到已有节点 40，只把它的 cnt 变成 2，节点数
     不变，中序变成 20 30 40 40 60 70 80。

复杂度分析：
  时间 O(h)：insert / find / erase / min / max 都只走一条路径；
        顺序插入会退化成链，h = n，这就是要 AVL 的原因。
  空间 O(h)：递归栈；节点本身 O(n)。
*/
package main

import "fmt"

type Node struct {
	val         int
	cnt         int // 该值出现次数，重复值不新建节点
	left, right *Node
}

func newNode(v int) *Node { return &Node{val: v, cnt: 1} }

type bst struct {
	root *Node
}

// 返回 true 表示新建了节点；false 表示只是 cnt 加一
func (t *bst) insert(v int) bool {
	created := false
	t.root = insertRec(t.root, v, &created)
	return created
}

func insertRec(n *Node, v int, created *bool) *Node {
	if n == nil {
		*created = true
		return newNode(v)
	}
	if v < n.val {
		n.left = insertRec(n.left, v, created)
	} else if v > n.val {
		n.right = insertRec(n.right, v, created)
	} else {
		n.cnt++ // 重复值只计数，保持「一值一节点」
	}
	return n
}

func (t *bst) find(v int) bool {
	n := t.root
	for n != nil {
		if v < n.val {
			n = n.left
		} else if v > n.val {
			n = n.right
		} else {
			return true
		}
	}
	return false
}

// 返回该值出现次数，不存在返回 0
func (t *bst) count(v int) int {
	n := t.root
	for n != nil {
		if v < n.val {
			n = n.left
		} else if v > n.val {
			n = n.right
		} else {
			return n.cnt
		}
	}
	return 0
}

func (t *bst) minVal() (int, bool) {
	if t.root == nil {
		return 0, false
	}
	n := t.root
	for n.left != nil {
		n = n.left
	}
	return n.val, true
}

func (t *bst) maxVal() (int, bool) {
	if t.root == nil {
		return 0, false
	}
	n := t.root
	for n.right != nil {
		n = n.right
	}
	return n.val, true
}

// 摘掉子树里最小的节点：它一定没有左孩子，直接拿右孩子顶替
func popMin(n *Node) (*Node, *Node) {
	if n.left == nil {
		return n.right, n
	}
	next, out := popMin(n.left)
	n.left = next
	return n, out
}

// 返回 true 表示确实删掉了一次出现
func (t *bst) erase(v int) bool {
	removed := false
	t.root = eraseRec(t.root, v, &removed)
	return removed
}

func eraseRec(n *Node, v int, removed *bool) *Node {
	if n == nil {
		return nil // 空树删除：原样返回，不算删除成功
	}
	if v < n.val {
		n.left = eraseRec(n.left, v, removed)
		return n
	}
	if v > n.val {
		n.right = eraseRec(n.right, v, removed)
		return n
	}
	*removed = true
	if n.cnt > 1 {
		n.cnt-- // 还有同值的其它出现，只减少一次计数
		return n
	}
	if n.left == nil {
		return n.right
	}
	if n.right == nil {
		return n.left
	}
	// 双孩子：中序后继没有左孩子，摘除它只需把它的右孩子接上
	var s *Node
	n.right, s = popMin(n.right)
	n.val = s.val
	n.cnt = s.cnt // 后继可能自己也带着重复计数，整份搬过来
	return n
}

func (t *bst) height() int { return heightRec(t.root) }

func heightRec(n *Node) int {
	if n == nil {
		return 0
	}
	l := heightRec(n.left)
	r := heightRec(n.right)
	if l > r {
		return l + 1
	}
	return r + 1
}

func (t *bst) size() int { return sizeRec(t.root) }

func sizeRec(n *Node) int {
	if n == nil {
		return 0
	}
	return n.cnt + sizeRec(n.left) + sizeRec(n.right)
}

func (t *bst) nodeCount() int { return nodeCountRec(t.root) }

func nodeCountRec(n *Node) int {
	if n == nil {
		return 0
	}
	return 1 + nodeCountRec(n.left) + nodeCountRec(n.right)
}

// 中序：每个值按 cnt 重复输出，所以整体非递减
func (t *bst) inorder() []int {
	out := []int{}
	inorderRec(t.root, &out)
	return out
}

func inorderRec(n *Node, out *[]int) {
	if n == nil {
		return
	}
	inorderRec(n.left, out)
	for i := 0; i < n.cnt; i++ {
		*out = append(*out, n.val)
	}
	inorderRec(n.right, out)
}

// 校验 BST 不变量：严格区间夹逼 + cnt >= 1
func (t *bst) check() bool {
	return checkRec(t.root, -1<<62, 1<<62)
}

func checkRec(n *Node, lo, hi int) bool {
	if n == nil {
		return true
	}
	if n.cnt < 1 {
		return false
	}
	if !(lo < n.val && n.val < hi) {
		return false
	}
	return checkRec(n.left, lo, n.val) && checkRec(n.right, n.val, hi)
}

func eq(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func check(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
}

func refInsert(ref []int, v int) []int {
	at := 0
	for at < len(ref) && ref[at] < v {
		at++
	}
	ref = append(ref, 0)
	copy(ref[at+1:], ref[at:])
	ref[at] = v
	return ref
}

func refErase(ref []int, v int) ([]int, bool) {
	for i := range ref {
		if ref[i] == v {
			return append(ref[:i], ref[i+1:]...), true
		}
	}
	return ref, false
}

func main() {
	// 1. 正常用例：插入、查找、极值、高度、中序
	{
		t := &bst{}
		for _, v := range []int{50, 30, 70, 20, 40, 60, 80} {
			check(t.insert(v), "insert should create node")
		}
		check(t.size() == 7, "size")
		check(t.nodeCount() == 7, "nodeCount")
		check(t.height() == 3, "height")
		check(t.find(40), "find 40")
		check(!t.find(41), "find 41")
		mn, ok1 := t.minVal()
		mx, ok2 := t.maxVal()
		check(ok1 && mn == 20, "min")
		check(ok2 && mx == 80, "max")
		check(t.check(), "check")
		check(eq(t.inorder(), []int{20, 30, 40, 50, 60, 70, 80}), "inorder")
		fmt.Println("inorder:", t.inorder())
	}

	// 2. 重复值：只加计数，不新建节点
	{
		t := &bst{}
		t.insert(40)
		check(!t.insert(40), "dup should not create")
		check(!t.insert(40), "dup should not create")
		check(t.count(40) == 3, "count 40")
		check(t.nodeCount() == 1, "nodeCount dup")
		check(t.size() == 3, "size dup")
		check(t.erase(40), "erase one")
		check(t.count(40) == 2, "count after erase")
		check(t.find(40), "still found")
		check(t.erase(40), "erase two")
		check(t.erase(40), "erase three")
		check(!t.find(40), "gone")
		check(!t.erase(40), "erase missing")
		check(t.nodeCount() == 0, "empty after")
		check(t.height() == 0, "height 0")
	}

	// 3. 删除叶子 / 单孩子 / 根 / 双孩子
	{
		t := &bst{}
		for _, v := range []int{50, 30, 70, 20, 40, 60, 80} {
			t.insert(v)
		}
		check(t.erase(20), "erase leaf")
		check(!t.find(20), "leaf gone")
		check(t.check(), "check after leaf")
		check(t.erase(60), "erase 60")
		check(t.erase(70), "erase 70")
		check(t.find(80), "80 remains")
		check(t.check(), "check after single child")
		check(t.erase(50), "erase root")
		check(t.find(80), "successor up")
		check(t.check(), "check after root")
		check(eq(t.inorder(), []int{30, 40, 80}), "inorder after")
		// 80 上位后左边还挂着 30 -> 40，所以高度是 3
		check(t.height() == 3, "height after")
	}

	// 4. 单孩子删根：根被唯一孩子顶替
	{
		t := &bst{}
		t.insert(10)
		t.insert(5)
		t.insert(3)
		check(t.erase(10), "erase root with one child")
		check(eq(t.inorder(), []int{3, 5}), "inorder root one child")
		check(t.check(), "check root one child")
		check(t.height() == 2, "height root one child")
		mn, _ := t.minVal()
		mx, _ := t.maxVal()
		check(mn == 3 && mx == 5, "min max root one child")
	}

	// 5. 空树删除：返回 false，树保持为空
	{
		t := &bst{}
		check(!t.erase(1), "erase empty")
		check(len(t.inorder()) == 0, "empty inorder")
		check(t.height() == 0, "empty height")
		_, ok1 := t.minVal()
		_, ok2 := t.maxVal()
		check(!ok1 && !ok2, "empty min max")
		check(t.size() == 0, "empty size")
		check(t.check(), "empty check")
	}

	// 6. 双孩子 + 后继自身带重复计数：计数必须整份搬上来
	{
		t := &bst{}
		t.insert(10)
		t.insert(5)
		t.insert(20)
		t.insert(20)
		check(t.count(20) == 2, "count 20")
		check(t.erase(10), "erase 10")
		check(eq(t.inorder(), []int{5, 20, 20}), "dup successor")
		check(t.count(20) == 2, "count 20 kept")
		check(t.nodeCount() == 2, "nodeCount dup successor")
		check(t.check(), "check dup successor")
	}

	// 7. 中序严格递增（无重复时）
	{
		t := &bst{}
		for i := 0; i < 64; i++ {
			t.insert(i*3 - 50)
		}
		seq := t.inorder()
		for i := 1; i < len(seq); i++ {
			check(seq[i-1] < seq[i], "not strictly increasing")
		}
		check(t.check(), "check strict")
	}

	// 8. 操作序列对拍：固定种子随机插删，与有序数组对照
	{
		t := &bst{}
		ref := []int{}
		var seed uint32 = 12345
		for step := 0; step < 400; step++ {
			seed = seed*1103515245 + 12345
			v := int((seed>>16)%201) - 100
			seed = seed*1103515245 + 12345
			doErase := (seed>>16)%2 == 0
			if doErase {
				removed := t.erase(v)
				var has bool
				ref, has = refErase(ref, v)
				check(removed == has, "erase mismatch")
			} else {
				t.insert(v)
				ref = refInsert(ref, v)
			}
			check(t.check(), "random check")
			check(t.size() == len(ref), "random size")
			check(eq(t.inorder(), ref), "random inorder")
		}
		fmt.Println("random ops size:", t.size())
	}

	fmt.Println("all tests passed")
}
