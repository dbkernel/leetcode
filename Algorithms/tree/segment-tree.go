/*
============================================================================
线段树（Segment Tree）
分类：手写数据结构 · 树
操作复杂度：
  build      O(n)      每个节点只算一次
  update     O(log n)  单点改值，回溯 log n 个祖先
  query      O(log n)  区间和，最多拆成 O(log n) 段
  range_add  O(log n)  懒标记版本：整段覆盖时只打标记，不下钻
  range_sum  O(log n)  懒标记版本
  空间 O(4n)：下标从 1 开始，2i / 2i+1 是孩子
============================================================================

用途：
  区间查询 + 单点 / 区间修改的标准结构。面试里常以「区间和」「区间
  最值」「区间加后求和」出现，考的是分治边界与懒标记下推时机。

核心思想：
  把数组切成二叉树的区间：根管 [0, n-1]，每个节点管一段，孩子各管
  一半，叶子管一个元素。节点存这段区间的和。查询时把目标区间拆成
  若干个「被节点完整覆盖」的段相加，所以只需 O(log n) 段。单点更新
  改叶子后沿父亲一路重算。

  懒标记版本多存一个 add：整段被覆盖时只把 tree[node] 加上
  val * 区间长度、lazy[node] 加上 val，不往下走；等以后真的需要
  进入孩子时再把标记推给两个孩子。

关键点（不变量）：
  · 完全覆盖 (ql <= l && r <= qr) 立刻返回，绝不再下钻，这是
    O(log n) 的来源；只按「相交」判断会退化成 O(n)；
  · push_down 只能在「部分覆盖、必须进孩子」之前调用；整段覆盖时
    调用会把标记提前散掉，白干一趟；
  · add 是叠加：lazy[孩子] += lazy[父亲]，tree[孩子] +=
    lazy[父亲] * 孩子区间长度；写成赋值会丢掉之前的加法；
  · 下推后必须 lazy[node] = 0，否则下次进孩子会重复加一遍；
  · 孩子区间长度分别是 mid-l+1 和 r-mid，不能都乘总长度。

手推一遍：
  以 {1,2,3,4,5} 建树：
  1. 根 [0,4] 存 15，左 [0,2] 存 6，右 [3,4] 存 9，
     [0,1] 存 3，叶子分别是 1 2 3 4 5；
  2. query(1,3)：根不被完全覆盖，左 [0,2] 只取 [1,2] 得 5，
     右 [3,4] 只取 3 得 4，合计 9；
  3. update(2,10)：叶子 3 -> 10，回溯 [0,2] 变 13、根变 22。

  懒标记版本以 {0,0,0,0} 为例，先 range_add(0,2,5)，再
  range_add(1,3,2)：
  1. 第一次：左 [0,1] 被整段覆盖，tree 加 5*2=10、lazy 加 5；
     右 [2,3] 只覆盖叶子 2，tree 加 5；
  2. 第二次：[0,1] 部分覆盖，先把 lazy=5 推给两个叶子（各 +5、
     清零），再给叶子 1 加 2 得 7；右半边同理，叶子 2 得 7、
     叶子 3 得 2；
  3. 四个叶子 5 7 7 2，区间和 21，正好等于 5*3 + 2*3。

复杂度分析：
  时间：build O(n)；update / query / range_add / range_sum
        都是 O(log n)，因为每层最多访问常数个节点。
  空间：tree 开 4n，lazy 再开 4n，共 O(n)。
*/
package main

import "fmt"

// ---------- 单点改值 + 区间和 ----------
type segmentTree struct {
	n    int
	tree []int // 4n 够用：区间二分的节点数上界
}

func newSegmentTree(a []int) *segmentTree {
	if len(a) == 0 {
		panic("empty array")
	}
	st := &segmentTree{n: len(a), tree: make([]int, 4*len(a)+4)}
	st.build(1, 0, len(a)-1, a)
	return st
}

func (st *segmentTree) build(node, l, r int, a []int) {
	if l == r {
		st.tree[node] = a[l]
		return
	}
	mid := l + (r-l)/2 // 防溢出写法
	st.build(node*2, l, mid, a)
	st.build(node*2+1, mid+1, r, a)
	st.tree[node] = st.tree[node*2] + st.tree[node*2+1]
}

// 单点赋值：把下标 idx 的值改成 value
func (st *segmentTree) update(idx, value int) {
	st.updateRec(1, 0, st.n-1, idx, value)
}

func (st *segmentTree) updateRec(node, l, r, idx, value int) {
	if l == r {
		st.tree[node] = value
		return
	}
	mid := l + (r-l)/2
	if idx <= mid {
		st.updateRec(node*2, l, mid, idx, value)
	} else {
		st.updateRec(node*2+1, mid+1, r, idx, value)
	}
	st.tree[node] = st.tree[node*2] + st.tree[node*2+1]
}

// 区间和 [ql, qr]；ql > qr 时返回 0
func (st *segmentTree) query(ql, qr int) int {
	if ql > qr {
		return 0
	}
	return st.queryRec(1, 0, st.n-1, ql, qr)
}

func (st *segmentTree) queryRec(node, l, r, ql, qr int) int {
	if ql <= l && r <= qr {
		return st.tree[node] // 完全覆盖，直接返回
	}
	mid := l + (r-l)/2
	s := 0
	if ql <= mid {
		s += st.queryRec(node*2, l, mid, ql, qr)
	}
	if qr > mid {
		s += st.queryRec(node*2+1, mid+1, r, ql, qr)
	}
	return s
}

// ---------- 区间加 + 区间和（懒标记） ----------
type lazySegmentTree struct {
	n    int
	tree []int
	lazy []int // 待下推到孩子的加法
}

func newLazySegmentTree(a []int) *lazySegmentTree {
	if len(a) == 0 {
		panic("empty array")
	}
	st := &lazySegmentTree{
		n:    len(a),
		tree: make([]int, 4*len(a)+4),
		lazy: make([]int, 4*len(a)+4),
	}
	st.build(1, 0, len(a)-1, a)
	return st
}

func (st *lazySegmentTree) build(node, l, r int, a []int) {
	st.lazy[node] = 0
	if l == r {
		st.tree[node] = a[l]
		return
	}
	mid := l + (r-l)/2
	st.build(node*2, l, mid, a)
	st.build(node*2+1, mid+1, r, a)
	st.tree[node] = st.tree[node*2] + st.tree[node*2+1]
}

// 把 node 的标记推给两个孩子，再清空自己
func (st *lazySegmentTree) pushDown(node, l, r int) {
	if st.lazy[node] == 0 || l == r {
		return // 叶子没有孩子可推
	}
	mid := l + (r-l)/2
	lc := node * 2
	rc := node*2 + 1
	st.lazy[lc] += st.lazy[node] // 叠加：新加法叠在旧标记上面
	st.tree[lc] += st.lazy[node] * (mid - l + 1)
	st.lazy[rc] += st.lazy[node]
	st.tree[rc] += st.lazy[node] * (r - mid)
	st.lazy[node] = 0 // 不清零会在下次下推时重复加
}

func (st *lazySegmentTree) rangeAdd(ql, qr, val int) {
	if ql > qr {
		return
	}
	st.addRec(1, 0, st.n-1, ql, qr, val)
}

func (st *lazySegmentTree) addRec(node, l, r, ql, qr, val int) {
	if ql <= l && r <= qr {
		// 整段覆盖：只改本节点的和与标记，绝不往下走
		st.tree[node] += val * (r - l + 1)
		st.lazy[node] += val
		return
	}
	st.pushDown(node, l, r) // 只有需要进孩子时才下推
	mid := l + (r-l)/2
	if ql <= mid {
		st.addRec(node*2, l, mid, ql, qr, val)
	}
	if qr > mid {
		st.addRec(node*2+1, mid+1, r, ql, qr, val)
	}
	st.tree[node] = st.tree[node*2] + st.tree[node*2+1]
}

func (st *lazySegmentTree) rangeSum(ql, qr int) int {
	if ql > qr {
		return 0
	}
	return st.sumRec(1, 0, st.n-1, ql, qr)
}

func (st *lazySegmentTree) sumRec(node, l, r, ql, qr int) int {
	if ql <= l && r <= qr {
		return st.tree[node]
	}
	// 查询也要下推：孩子的 tree 还没包含父节点挂着的标记
	st.pushDown(node, l, r)
	mid := l + (r-l)/2
	s := 0
	if ql <= mid {
		s += st.sumRec(node*2, l, mid, ql, qr)
	}
	if qr > mid {
		s += st.sumRec(node*2+1, mid+1, r, ql, qr)
	}
	return s
}

func bruteSum(a []int, l, r int) int {
	s := 0
	for i := l; i <= r; i++ {
		s += a[i]
	}
	return s
}

func check(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
}

func main() {
	// 1. 小数组的手算用例：全区间 / 子区间 / 单点
	{
		a := []int{1, 2, 3, 4, 5}
		st := newSegmentTree(a)
		check(st.query(0, 4) == 15, "query all")
		check(st.query(1, 3) == 9, "query 1..3")
		check(st.query(2, 2) == 3, "query single")
		check(st.query(0, 0) == 1, "query first")
		check(st.query(4, 4) == 5, "query last")
		check(st.query(3, 2) == 0, "empty range")
		st.update(2, 10)
		check(st.query(0, 4) == 22, "after update all")
		check(st.query(1, 3) == 16, "after update 1..3")
		check(st.query(2, 2) == 10, "after update single")
		fmt.Println("after update total:", st.query(0, 4))
	}

	// 2. n = 1：区间就是那一个点
	{
		st := newSegmentTree([]int{7})
		check(st.query(0, 0) == 7, "n=1 query")
		st.update(0, 3)
		check(st.query(0, 0) == 3, "n=1 update")
		check(st.query(0, -1) == 0, "n=1 empty range")
	}

	// 3. 负数与全 0
	{
		st := newSegmentTree([]int{0, 0, 0, 0})
		check(st.query(0, 3) == 0, "all zero")
		st.update(1, -5)
		st.update(3, -7)
		check(st.query(0, 3) == -12, "neg sum")
		check(st.query(1, 1) == -5, "neg single")
		check(st.query(1, 2) == -5, "neg range")
	}

	// 4. 与暴力对拍：固定种子随机单点修改 + 随机区间查询
	{
		const n = 40
		a := make([]int, n)
		var seed uint32 = 987654321
		for i := 0; i < n; i++ {
			seed = seed*1103515245 + 12345
			a[i] = int((seed>>16)%201) - 100
		}
		st := newSegmentTree(a)
		check(st.query(0, n-1) == bruteSum(a, 0, n-1), "initial total")
		for step := 0; step < 300; step++ {
			seed = seed*1103515245 + 12345
			x := int((seed >> 16) % 1000)
			seed = seed*1103515245 + 12345
			y := int((seed >> 16) % 1000)
			if step%3 == 0 {
				idx := x % n
				value := y%401 - 200
				a[idx] = value // 影子数组，只用于暴力对照
				st.update(idx, value)
			} else {
				l, r := x%n, y%n
				if l > r {
					l, r = r, l
				}
				check(st.query(l, r) == bruteSum(a, l, r), "random query")
			}
		}
		fmt.Println("random point-update total:", st.query(0, n-1))
	}

	// 5. 懒标记版本：区间加 + 区间和，与暴力对拍
	{
		const n = 30
		a := make([]int, n)
		var seed uint32 = 13579
		for i := 0; i < n; i++ {
			seed = seed*1103515245 + 12345
			a[i] = int((seed>>16)%41) - 20
		}
		st := newLazySegmentTree(a)
		b := make([]int, n)
		copy(b, a) // 暴力的影子数组
		check(st.rangeSum(0, n-1) == bruteSum(b, 0, n-1), "lazy initial")
		for step := 0; step < 300; step++ {
			seed = seed*1103515245 + 12345
			x := int((seed >> 16) % 1000)
			seed = seed*1103515245 + 12345
			y := int((seed >> 16) % 1000)
			l, r := x%n, y%n
			if l > r {
				l, r = r, l
			}
			seed = seed*1103515245 + 12345
			val := int((seed>>16)%21) - 10
			if step%2 == 0 {
				st.rangeAdd(l, r, val)
				for i := l; i <= r; i++ {
					b[i] += val
				}
			} else {
				check(st.rangeSum(l, r) == bruteSum(b, l, r), "lazy query")
			}
		}
		// 最后整体核对一次，确保懒标记没有漏推
		check(st.rangeSum(0, n-1) == bruteSum(b, 0, n-1), "lazy total")
		fmt.Println("lazy total:", st.rangeSum(0, n-1))
	}

	// 6. 懒标记边界：n = 1、整段加、负值、l == r
	{
		st := newLazySegmentTree([]int{5})
		st.rangeAdd(0, 0, -3)
		check(st.rangeSum(0, 0) == 2, "lazy n=1 add")
		st.rangeAdd(0, 0, 10)
		check(st.rangeSum(0, 0) == 12, "lazy n=1 add again")

		t2 := newLazySegmentTree([]int{1, 2, 3, 4})
		t2.rangeAdd(0, 3, 5) // 整段覆盖根，只打标记
		check(t2.rangeSum(0, 3) == 30, "full add")
		t2.rangeAdd(1, 2, -4) // 此时必须先把根的标记推下去
		check(t2.rangeSum(0, 3) == 22, "after partial add")
		check(t2.rangeSum(1, 1) == 3, "leaf 1")
		check(t2.rangeSum(2, 2) == 4, "leaf 2")
		check(t2.rangeSum(0, 0) == 6, "leaf 0")
		check(t2.rangeSum(3, 3) == 9, "leaf 3")
	}

	fmt.Println("all tests passed")
}
