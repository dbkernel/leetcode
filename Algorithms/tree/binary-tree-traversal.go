/*
============================================================================
二叉树遍历（Binary Tree Traversal）
分类：手写数据结构 · 树
复杂度：时间 O(n)：四种遍历都恰好访问每个节点一次；
        空间 O(h)：递归栈或显式栈，h 为树高；Morris 中序 O(1)
============================================================================

用途：
  树的入门动作，也是序列化 / 反序列化、树形 DP 的底座。面试常问
  「不用递归写后序」「不用栈和递归遍历，空间 O(1)」。

核心思想：
  三种深度优先序的差别只在「输出根」放在两次递归的哪一侧：前序在
  进左子树之前，中序在左子树回来之后，后序在右子树回来之后。把递归
  栈换成一个自己维护的数组就是迭代版，层序把栈换成先进先出的循环
  队列。Morris 中序借叶子结点空闲的右指针指向「中序后继」，走完左
  子树后顺着这根线回到祖先，用完立刻拆掉。

关键点（不变量）：
  · 迭代前序：先压右孩子再压左孩子，栈顶才是左孩子；
  · 迭代后序：last 记录上一个真正输出的节点，只有右孩子为空或
    右孩子 == last 时才允许输出栈顶，否则先把右子树走完；
  · Morris：prev->right == cur 表示第二次到达 cur，必须把
    prev->right 还原成空，否则树里留下环；
  · 层序队列用 count 记录元素个数：回绕后 head == tail 无法区分
    空与满，只看 head / tail 会误判。

手推一遍：
  以层序数组 {1,2,3,4,5,-1,6,-1,-1,7} 建出的树：

            1
          /   \
         2     3
        / \     \
       4   5     6
          /
         7

  1. 前序：输出 1，进左 2，进左 4 输出 4，回 2 进右 5，进左 7
     输出 7，回到 5 输出 5，回到 2 输出 2，回根进右 3，进右 6，
     得 1 2 4 5 7 3 6；
  2. 中序：一路向左到 4 输出，回 2 输出，进 5 先到 7 输出，
     得 4 2 7 5 1 3 6；
  3. 后序得 4 7 5 2 6 3 1；层序按出队顺序得 1 2 3 4 5 6 7。

复杂度分析：
  时间 O(n)：每节点进栈、出栈各一次；Morris 每条边最多走两遍。
  空间 O(h)：递归栈 / 显式栈；层序 O(w)，w 为最大层宽；
        Morris 只用两个指针，O(1)。
*/
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

// 自己写的循环队列：count 记录元素个数，回绕后也能区分空与满。
type ringQueue struct {
	buf   []*Node
	head  int
	tail  int
	count int
}

func newRingQueue() *ringQueue {
	return &ringQueue{buf: make([]*Node, 8)}
}

func (q *ringQueue) empty() bool { return q.count == 0 }

func (q *ringQueue) push(x *Node) {
	if q.count == len(q.buf) {
		// 扩容：按 head 起的逻辑顺序搬移，搬完 head 回到 0
		nb := make([]*Node, len(q.buf)*2)
		for i := 0; i < q.count; i++ {
			nb[i] = q.buf[(q.head+i)%len(q.buf)]
		}
		q.buf = nb
		q.head = 0
		q.tail = q.count
	}
	q.buf[q.tail] = x
	q.tail = (q.tail + 1) % len(q.buf)
	q.count++
}

func (q *ringQueue) pop() *Node {
	x := q.buf[q.head]
	q.head = (q.head + 1) % len(q.buf)
	q.count--
	return x
}

type binTree struct {
	root *Node
}

// 层序数组建树，-1 表示空位。仅用于构造。
func (t *binTree) build(a []int) {
	t.root = nil
	if len(a) == 0 || a[0] == -1 {
		return
	}
	t.root = &Node{val: a[0]}
	q := newRingQueue()
	q.push(t.root)
	i := 1
	for !q.empty() && i < len(a) {
		cur := q.pop()
		if a[i] != -1 {
			cur.left = &Node{val: a[i]}
			q.push(cur.left)
		}
		i++
		if i < len(a) {
			if a[i] != -1 {
				cur.right = &Node{val: a[i]}
				q.push(cur.right)
			}
			i++
		}
	}
}

// 只有左孩子的链，值从根往下是 n, n-1, ..., 1
func (t *binTree) buildLeftChain(n int) {
	t.root = nil
	for v := 1; v <= n; v++ {
		t.root = &Node{val: v, left: t.root}
	}
}

// 只有右孩子的链，值从根往下同样是 n, n-1, ..., 1
func (t *binTree) buildRightChain(n int) {
	t.root = nil
	for v := 1; v <= n; v++ {
		t.root = &Node{val: v, right: t.root}
	}
}

// 层序序列化，空位写 -1 并去掉尾部空位，用来确认遍历没改坏树
func (t *binTree) toLevelArray() []int {
	out := []int{}
	q := newRingQueue()
	q.push(t.root)
	for !q.empty() {
		x := q.pop()
		if x == nil {
			out = append(out, -1)
			continue
		}
		out = append(out, x.val)
		q.push(x.left)
		q.push(x.right)
	}
	for len(out) > 0 && out[len(out)-1] == -1 {
		out = out[:len(out)-1]
	}
	return out
}

// ---------- 递归版 ----------
func preRec(t *Node, out *[]int) {
	if t == nil {
		return
	}
	*out = append(*out, t.val)
	preRec(t.left, out)
	preRec(t.right, out)
}

func inRec(t *Node, out *[]int) {
	if t == nil {
		return
	}
	inRec(t.left, out)
	*out = append(*out, t.val)
	inRec(t.right, out)
}

func postRec(t *Node, out *[]int) {
	if t == nil {
		return
	}
	postRec(t.left, out)
	postRec(t.right, out)
	*out = append(*out, t.val)
}

func (t *binTree) preorder() []int {
	out := []int{}
	preRec(t.root, &out)
	return out
}

func (t *binTree) inorder() []int {
	out := []int{}
	inRec(t.root, &out)
	return out
}

func (t *binTree) postorder() []int {
	out := []int{}
	postRec(t.root, &out)
	return out
}

// ---------- 显式栈迭代版 ----------
func (t *binTree) preorderIter() []int {
	out := []int{}
	if t.root == nil {
		return out
	}
	st := []*Node{t.root}
	for len(st) > 0 {
		x := st[len(st)-1]
		st = st[:len(st)-1]
		out = append(out, x.val)
		// 先压右再压左：后进先出，左孩子先被处理
		if x.right != nil {
			st = append(st, x.right)
		}
		if x.left != nil {
			st = append(st, x.left)
		}
	}
	return out
}

func (t *binTree) inorderIter() []int {
	out := []int{}
	st := []*Node{}
	cur := t.root
	for cur != nil || len(st) > 0 {
		for cur != nil {
			st = append(st, cur)
			cur = cur.left
		}
		cur = st[len(st)-1]
		st = st[:len(st)-1]
		out = append(out, cur.val)
		cur = cur.right
	}
	return out
}

func (t *binTree) postorderIter() []int {
	out := []int{}
	st := []*Node{}
	cur := t.root
	var last *Node // 上一个真正输出的节点
	for cur != nil || len(st) > 0 {
		for cur != nil {
			st = append(st, cur)
			cur = cur.left
		}
		top := st[len(st)-1]
		// 右子树存在且还没走完，先转过去，top 留在栈上
		if top.right != nil && top.right != last {
			cur = top.right
		} else {
			out = append(out, top.val)
			last = top
			st = st[:len(st)-1]
		}
	}
	return out
}

// ---------- 层序：自己写的循环队列 ----------
func (t *binTree) levelOrder() []int {
	out := []int{}
	if t.root == nil {
		return out
	}
	q := newRingQueue()
	q.push(t.root)
	for !q.empty() {
		x := q.pop()
		out = append(out, x.val)
		if x.left != nil {
			q.push(x.left)
		}
		if x.right != nil {
			q.push(x.right)
		}
	}
	return out
}

// ---------- Morris 中序：O(1) 额外空间 ----------
func (t *binTree) morrisInorder() []int {
	out := []int{}
	cur := t.root
	for cur != nil {
		if cur.left == nil {
			out = append(out, cur.val)
			cur = cur.right
			continue
		}
		prev := cur.left
		for prev.right != nil && prev.right != cur {
			prev = prev.right
		}
		if prev.right == nil {
			prev.right = cur // 穿线
			cur = cur.left
		} else {
			prev.right = nil // 拆线，树恢复原状
			out = append(out, cur.val)
			cur = cur.right
		}
	}
	return out
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
	// 1. 固定形状的树，验证四种遍历序列
	{
		t := &binTree{}
		t.build([]int{1, 2, 3, 4, 5, -1, 6, -1, -1, 7})
		pre := []int{1, 2, 4, 5, 7, 3, 6}
		in := []int{4, 2, 7, 5, 1, 3, 6}
		post := []int{4, 7, 5, 2, 6, 3, 1}
		lvl := []int{1, 2, 3, 4, 5, 6, 7}
		check(eq(t.preorder(), pre), "preorder")
		check(eq(t.inorder(), in), "inorder")
		check(eq(t.postorder(), post), "postorder")
		check(eq(t.preorderIter(), pre), "preorderIter")
		check(eq(t.inorderIter(), in), "inorderIter")
		check(eq(t.postorderIter(), post), "postorderIter")
		check(eq(t.levelOrder(), lvl), "levelOrder")
		check(eq(t.morrisInorder(), in), "morris != recursive")
		// 遍历（含 Morris 穿线拆线）之后树结构不能变
		check(eq(t.toLevelArray(),
			[]int{1, 2, 3, 4, 5, -1, 6, -1, -1, 7}), "shape changed")
		check(eq(t.preorder(), pre), "preorder after morris")
		fmt.Println("fixed tree preorder:", t.preorder())
	}

	// 2. 单节点
	{
		t := &binTree{}
		t.build([]int{5})
		one := []int{5}
		check(eq(t.preorder(), one), "single preorder")
		check(eq(t.inorder(), one), "single inorder")
		check(eq(t.postorder(), one), "single postorder")
		check(eq(t.preorderIter(), one), "single preorderIter")
		check(eq(t.inorderIter(), one), "single inorderIter")
		check(eq(t.postorderIter(), one), "single postorderIter")
		check(eq(t.levelOrder(), one), "single levelOrder")
		check(eq(t.morrisInorder(), one), "single morris")
		check(eq(t.toLevelArray(), one), "single shape")
	}

	// 3. 空树
	{
		t := &binTree{}
		check(len(t.preorder()) == 0, "empty preorder")
		check(len(t.postorder()) == 0, "empty postorder")
		check(len(t.preorderIter()) == 0, "empty preorderIter")
		check(len(t.inorderIter()) == 0, "empty inorderIter")
		check(len(t.postorderIter()) == 0, "empty postorderIter")
		check(len(t.levelOrder()) == 0, "empty levelOrder")
		check(len(t.morrisInorder()) == 0, "empty morris")
		check(len(t.toLevelArray()) == 0, "empty shape")
		// 空树里 -1 只表示空位，不产生节点
		t.build([]int{-1, 1, 2})
		check(len(t.toLevelArray()) == 0, "root -1 should be empty")
	}

	// 4. 只有左 / 右孩子的长链：验证显式栈不塌
	{
		const n = 500
		asc := make([]int, n)
		desc := make([]int, n)
		for v := 1; v <= n; v++ {
			asc[v-1] = v
			desc[v-1] = n - v + 1
		}
		lt := &binTree{}
		lt.buildLeftChain(n)
		check(eq(lt.preorderIter(), desc), "left chain preorder")
		check(eq(lt.inorderIter(), asc), "left chain inorder")
		check(eq(lt.postorderIter(), asc), "left chain postorder")
		check(eq(lt.levelOrder(), desc), "left chain level")
		check(eq(lt.morrisInorder(), asc), "left chain morris")

		rt := &binTree{}
		rt.buildRightChain(n)
		check(eq(rt.preorderIter(), desc), "right chain preorder")
		check(eq(rt.inorderIter(), desc), "right chain inorder")
		check(eq(rt.postorderIter(), asc), "right chain postorder")
		check(eq(rt.levelOrder(), desc), "right chain level")
		check(eq(rt.morrisInorder(), desc), "right chain morris")
	}

	fmt.Println("all tests passed")
}
