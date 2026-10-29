/*
============================================================================
AVL 树（AVL Tree）
分类：手写数据结构 · 树
操作复杂度：
  height   O(1)     每个节点缓存子树高度
  find     O(log n)
  insert   O(log n) 回溯更新高度，至多一次旋转（单旋或双旋）
  erase    O(log n) 双孩子先搬后继，再从后继父节点回溯
  rotate   O(1)     单次旋转只改三条指针、修正两个高度
  空间 O(n)；树高恒为 O(log n)，最坏也是，不会退化成链
============================================================================

用途：
  需要「有序 + 稳定 O(log n)」时的首选手写结构；面试里常用来考
  「你知不知道旋转之后要回溯修高度」「删除为什么比重插入难」。

核心思想：
  AVL 在 BST 的有序性之上再加一条：每个节点左右子树高度差不超过 1。
  插入 / 删除后只有从改动点到根这一条路径上的节点可能失衡，于是回溯
  时自底向上修高度，一旦某节点 bf 变成 +2 或 -2，就按失衡形态做
  一次旋转把高度压回去。旋转不改变中序序列，所以有序性自动保持。

关键点（不变量）：
  · 先修自己的高度，再看 bf；旋转返回新的子树根，调用方必须接住；
  · 单旋后两个节点的高度都要重算：先算子节点，再算新根，顺序反了
    就会把旧高度写回去；
  · 删除双孩子节点后，必须从「后继的父节点」开始回溯，不能只修到
    被删节点为止。这一段是 AVL 删除最容易漏的地方：后继在右子树
    深处，它的祖先高度全变了，漏掉就会出现 bf 已经越界却没人旋转；
  · 重复值直接返回、不计入、不旋转，保证树里每个值唯一。

手推一遍：
  依次插入 30 20 10：
  1. 插 30 后 h=1；插 20 挂在左边，30 的 bf=1；
  2. 插 10 仍往左，30 的 bf=2，且左孩子 20 的 bf=1，属于 LL：
     对 30 右旋，20 升为根，30 成为 20 的右孩子，20 原来的右子树
     （空）挂到 30 左边；先修 30 的高度为 1，再修 20 为 2；
  3. 若插入序列是 30 10 20，则 30 的 bf=2 而左孩子 10 的 bf=-1，
     属于 LR：先对 10 左旋得到 30 -> 20 -> 10 的形状，再对 30
     右旋，最终 20 为根，10、30 为两个孩子。

复杂度分析：
  时间 O(log n)：树高被 bf <= 1 卡在 1.44 log2(n+2) 附近；
        insert / erase 各走一条根到叶的路径，旋转 O(1)。
  空间 O(n) 存节点，O(log n) 递归栈。
*/
package main

import "fmt"

type avlNode struct {
	val         int
	height      int // 以该节点为根的子树高度，空树记 0，叶子记 1
	left, right *avlNode
}

func newAVLNode(v int) *avlNode {
	return &avlNode{val: v, height: 1}
}

func h(n *avlNode) int {
	if n == nil {
		return 0
	}
	return n.height
}

func bf(n *avlNode) int {
	if n == nil {
		return 0
	}
	return h(n.left) - h(n.right)
}

func fix(n *avlNode) {
	l := h(n.left)
	r := h(n.right)
	if l > r {
		n.height = l + 1
	} else {
		n.height = r + 1
	}
}

// LL 失衡：左孩子的左子树过高，对 t 右旋
func rotateLL(t *avlNode) *avlNode {
	x := t.left
	t.left = x.right // x 的右子树夹在 t 和 x 之间，仍然有序
	x.right = t
	fix(t) // 先算子节点，再算新根
	fix(x)
	return x
}

// RR 失衡：右孩子的右子树过高，对 t 左旋
func rotateRR(t *avlNode) *avlNode {
	x := t.right
	t.right = x.left
	x.left = t
	fix(t)
	fix(x)
	return x
}

// LR 失衡：先对左孩子左旋，把它变成 LL，再对自己右旋
func rotateLR(t *avlNode) *avlNode {
	t.left = rotateRR(t.left)
	return rotateLL(t)
}

// RL 失衡：先对右孩子右旋，把它变成 RR，再对自己左旋
func rotateRL(t *avlNode) *avlNode {
	t.right = rotateLL(t.right)
	return rotateRR(t)
}

func rebalance(t *avlNode) *avlNode {
	fix(t)
	b := bf(t)
	if b > 1 {
		// 左孩子 bf >= 0 是 LL；< 0 说明新节点插在左孩子的右边
		if bf(t.left) >= 0 {
			return rotateLL(t)
		}
		return rotateLR(t)
	}
	if b < -1 {
		if bf(t.right) <= 0 {
			return rotateRR(t)
		}
		return rotateRL(t)
	}
	return t
}

func insertRec(t *avlNode, v int, inserted *bool) *avlNode {
	if t == nil {
		*inserted = true
		return newAVLNode(v)
	}
	if v < t.val {
		t.left = insertRec(t.left, v, inserted)
	} else if v > t.val {
		t.right = insertRec(t.right, v, inserted)
	} else {
		return t // 重复值不插入，也不必修高度
	}
	return rebalance(t)
}

func eraseRec(t *avlNode, v int, removed *bool) *avlNode {
	if t == nil {
		return nil
	}
	if v < t.val {
		t.left = eraseRec(t.left, v, removed)
	} else if v > t.val {
		t.right = eraseRec(t.right, v, removed)
	} else {
		*removed = true
		if t.left == nil || t.right == nil {
			// 叶子或单孩子：孩子顶上，高度由调用方 rebalance 修
			if t.left != nil {
				return t.left
			}
			return t.right
		}
		// 双孩子：中序后继（右子树最小值）的值搬上来
		s := t.right
		for s.left != nil {
			s = s.left
		}
		t.val = s.val
		successor := s.val
		// 关键：真正删的是右子树里的后继，回溯必须从右子树内部
		// 一路 rebalance 上来，否则后继的祖先高度会停在旧值
		t.right = eraseRec(t.right, successor, removed)
	}
	return rebalance(t)
}

type avl struct {
	root *avlNode
}

func (t *avl) insert(v int) bool {
	inserted := false
	t.root = insertRec(t.root, v, &inserted)
	return inserted
}

func (t *avl) erase(v int) bool {
	removed := false
	t.root = eraseRec(t.root, v, &removed)
	return removed
}

func (t *avl) find(v int) bool {
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

func (t *avl) height() int { return h(t.root) }

func (t *avl) rootVal() int { return t.root.val } // 调用前保证非空

func (t *avl) size() int { return sizeRec(t.root) }

func sizeRec(n *avlNode) int {
	if n == nil {
		return 0
	}
	return 1 + sizeRec(n.left) + sizeRec(n.right)
}

func (t *avl) inorder() []int {
	out := []int{}
	inorderRec(t.root, &out)
	return out
}

func inorderRec(n *avlNode, out *[]int) {
	if n == nil {
		return
	}
	inorderRec(n.left, out)
	*out = append(*out, n.val)
	inorderRec(n.right, out)
}

// 校验函数：同时验证 BST 严格递增、|bf| <= 1、缓存高度没写错
func (t *avl) checkBalanced() bool {
	hasPrev := false
	prev := 0
	return checkRec(t.root, &hasPrev, &prev)
}

func checkRec(n *avlNode, hasPrev *bool, prev *int) bool {
	if n == nil {
		return true
	}
	if !checkRec(n.left, hasPrev, prev) {
		return false
	}
	if *hasPrev && !(*prev < n.val) {
		return false // BST 必须严格递增
	}
	*hasPrev = true
	*prev = n.val
	if !checkRec(n.right, hasPrev, prev) {
		return false
	}
	diff := h(n.left) - h(n.right)
	if diff > 1 || diff < -1 {
		return false
	}
	l := h(n.left)
	r := h(n.right)
	want := r + 1
	if l > r {
		want = l + 1
	}
	return n.height == want // 缓存高度也不能说谎
}

func check(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
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

func main() {
	// 1. 顺序插入 1..100，高度不能超过理论上界 9
	{
		t := &avl{}
		for v := 1; v <= 100; v++ {
			check(t.insert(v), "insert sequential")
			check(t.checkBalanced(), "balanced after insert")
		}
		check(t.size() == 100, "size 100")
		// 高度为 h 的 AVL 最少节点数 N(h)=N(h-1)+N(h-2)+1，
		// N(9)=88 <= 100 < N(10)=143，所以高度最多 9
		check(t.height() <= 9, "height bound")
		check(t.height() >= 7, "height lower bound")
		expect := make([]int, 100)
		for v := 1; v <= 100; v++ {
			expect[v-1] = v
		}
		check(eq(t.inorder(), expect), "inorder 1..100")
		fmt.Println("1..100 height:", t.height())
	}

	// 2. 四种旋转各一个最小用例，最终都应是 20 为根、高度 2
	{
		ll := &avl{}
		ll.insert(30)
		ll.insert(20)
		ll.insert(10)
		check(ll.rootVal() == 20 && ll.height() == 2, "LL rotate")
		check(eq(ll.inorder(), []int{10, 20, 30}), "LL inorder")
		check(ll.checkBalanced(), "LL balanced")

		rr := &avl{}
		rr.insert(10)
		rr.insert(20)
		rr.insert(30)
		check(rr.rootVal() == 20 && rr.height() == 2, "RR rotate")
		check(eq(rr.inorder(), []int{10, 20, 30}), "RR inorder")
		check(rr.checkBalanced(), "RR balanced")

		lr := &avl{}
		lr.insert(30)
		lr.insert(10)
		lr.insert(20)
		check(lr.rootVal() == 20 && lr.height() == 2, "LR rotate")
		check(eq(lr.inorder(), []int{10, 20, 30}), "LR inorder")
		check(lr.checkBalanced(), "LR balanced")

		rl := &avl{}
		rl.insert(10)
		rl.insert(30)
		rl.insert(20)
		check(rl.rootVal() == 20 && rl.height() == 2, "RL rotate")
		check(eq(rl.inorder(), []int{10, 20, 30}), "RL inorder")
		check(rl.checkBalanced(), "RL balanced")
	}

	// 3. 重复值不插入
	{
		t := &avl{}
		check(t.insert(5), "insert 5")
		check(!t.insert(5), "dup rejected")
		check(!t.insert(5), "dup rejected again")
		check(t.size() == 1, "size dup")
		check(t.height() == 1, "height dup")
		check(eq(t.inorder(), []int{5}), "inorder dup")
		check(t.checkBalanced(), "balanced dup")
		check(!t.erase(4), "erase missing")
		check(t.size() == 1, "size after missing erase")
	}

	// 4. 删除：叶子 / 单孩子 / 根 / 空树
	{
		t := &avl{}
		for v := 1; v <= 20; v++ {
			t.insert(v)
		}
		check(t.erase(1), "erase leaf")
		check(!t.find(1), "leaf gone")
		check(t.checkBalanced(), "balanced after leaf")
		for t.size() > 1 { // 逐个删到只剩根
			v := t.inorder()[0]
			check(t.erase(v), "erase loop")
			check(t.checkBalanced(), "balanced in loop")
		}
		check(t.erase(t.rootVal()), "erase root")
		check(t.size() == 0 && t.height() == 0, "empty after root")
		check(t.checkBalanced(), "balanced empty")
		check(!t.erase(42), "erase empty tree")
		check(t.size() == 0, "still empty")
	}

	// 5. 双孩子删除：后继在右子树深处，回溯必须覆盖后继的祖先
	{
		t := &avl{}
		for _, v := range []int{50, 25, 75, 10, 30, 60, 90, 5, 15, 27,
			35, 55, 65, 80, 95, 1, 7, 12, 18} {
			t.insert(v)
		}
		check(t.checkBalanced(), "balanced before")
		// 50 有双孩子，后继是 55，删完从 55 的父节点一路修回根
		check(t.erase(50), "erase two children")
		check(!t.find(50), "50 gone")
		check(t.find(55), "55 kept")
		check(t.checkBalanced(), "balanced after two children")
		check(t.size() == 18, "size after two children")
	}

	// 6. 随机插入 / 删除对拍：固定种子，与有序数组对照
	{
		t := &avl{}
		ref := []int{}
		var seed uint32 = 20240601
		for step := 0; step < 1600; step++ {
			seed = seed*1103515245 + 12345
			v := int((seed>>16)%201) - 100
			seed = seed*1103515245 + 12345
			doErase := (seed>>16)%2 == 0
			if doErase {
				removed := t.erase(v)
				at := -1
				for i := range ref {
					if ref[i] == v {
						at = i
						break
					}
				}
				check(removed == (at >= 0), "erase mismatch")
				if at >= 0 {
					ref = append(ref[:at], ref[at+1:]...)
				}
			} else {
				inserted := t.insert(v)
				at := 0
				for at < len(ref) && ref[at] < v {
					at++
				}
				has := at < len(ref) && ref[at] == v
				check(inserted == !has, "insert mismatch")
				if !has {
					ref = append(ref, 0)
					copy(ref[at+1:], ref[at:])
					ref[at] = v
				}
			}
			// 每一步都必须满足：BST 有序 + |bf| <= 1 + 高度缓存正确
			check(t.checkBalanced(), "random balanced")
			check(t.size() == len(ref), "random size")
			check(eq(t.inorder(), ref), "random inorder")
		}
		fmt.Println("random ops size:", t.size(), "height:", t.height())
		// 再把剩下的按升序删空，验证连续的删除路径也能保持平衡
		for len(ref) > 0 {
			v := ref[0]
			ref = ref[1:]
			check(t.erase(v), "erase ascending")
			check(t.checkBalanced(), "balanced ascending")
			check(eq(t.inorder(), ref), "inorder ascending")
		}
		check(t.size() == 0 && t.height() == 0, "empty at end")
	}

	fmt.Println("all tests passed")
}
