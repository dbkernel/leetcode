/*
============================================================================
树状数组（Fenwick Tree / Binary Indexed Tree）
分类：手写数据结构 · 树
操作复杂度：
  lowbit     O(1)     x & (-x)，取出最低位的 1
  build      O(n)     线性建树，不是 n 次 update 的 O(n log n)
  update     O(log n) 单点加，沿 i += lowbit(i) 往上跳
  query      O(log n) 前缀和，沿 i -= lowbit(i) 往下跳
  range_sum  O(log n) 两个前缀和相减
  空间 O(n)：tree 下标从 1 开始
============================================================================

用途：
  单点修改 + 前缀和 / 区间和的高频结构，比线段树短得多。
  面试常问：lowbit 什么意思、为什么 O(n) 建树、能不能求区间最值。

核心思想：
  把下标 i 的管辖范围定义成 (i - lowbit(i), i] 这一段，tree[i] 存
  这段的和。比如 i=6 (110)，lowbit=2，管 5..6；i=8 (1000) 管
  1..8。查询前缀和时每次去掉最低位的 1，最多走 log n 步；单点更新
  时每次加上最低位的 1，把改动推给所有管到它的节点。

关键点（不变量）：
  · tree[i] 的区间是左开右闭 (i - lowbit(i), i]，所以树状数组必须
    1 基下标；用 0 基会 lowbit(0) = 0 死循环；
  · update 用 i += lowbit(i) 往上跳，query 用 i -= lowbit(i) 往下
    跳，两个方向写反就是死循环或漏算；
  · O(n) 建树必须按 i 从小到大：走到 i 时，所有 j + lowbit(j) = i
    的孩子都已经把值推给了 tree[i]，此时 tree[i] 才是这段的真和；
    逆序时父节点先被加了一次「半成品」，孩子再来加就重复且不准；
  · 区间和是 query(r) - query(l-1)，l=0 时不能查 -1，要单独处理。

手推一遍：
  以 a = {1,2,3,4,5} 为例，线性建树：
  1. i=1: tree[1] += 1，j=2，tree[2] += tree[1] 得 1；
  2. i=2: tree[2] += 2 得 3，j=4，tree[4] += 3 得 3；
  3. i=3: tree[3] += 3 得 3，j=4，tree[4] += 3 得 6；
  4. i=4: tree[4] += 4 得 10，j=8 越界；
  5. i=5: tree[5] += 5 得 5，j=6 越界；
  最终 tree = [_,1,3,3,10,5]；
  6. query(4) 求 0..4：i=5 取 5，i=4 取 10，合计 15，正确。

复杂度分析：
  时间：update / query / range_sum 都是 O(log n)，因为每次至少
        去掉或加上一个二进制位，最多 log2(n) 步；build O(n)。
  空间：O(n)，只比原数组多一个 n+1 的数组。
*/
package main

import "fmt"

type fenwick struct {
	n    int
	tree []int // 1 基下标，tree[i] 管 (i-lowbit(i), i]
}

func lowbit(x int) int { return x & (-x) }

// O(n) 建树：i 从小到大，tree[i] 此时已经是自己那段的真和
func newFenwick(a []int) *fenwick {
	f := &fenwick{n: len(a), tree: make([]int, len(a)+1)}
	for i := 1; i <= f.n; i++ {
		f.tree[i] += a[i-1]
		j := i + lowbit(i)
		if j <= f.n {
			f.tree[j] += f.tree[i]
		}
	}
	return f
}

// 全 0 数组，之后用 update 逐个填
func newFenwickZeros(n int) *fenwick {
	return &fenwick{n: n, tree: make([]int, n+1)}
}

// 单点加：idx 是 0 基下标，delta 可正可负
func (f *fenwick) update(idx, delta int) {
	for i := idx + 1; i <= f.n; i += lowbit(i) {
		f.tree[i] += delta
	}
}

// 前缀和 [0, idx]
func (f *fenwick) query(idx int) int {
	s := 0
	for i := idx + 1; i > 0; i -= lowbit(i) {
		s += f.tree[i]
	}
	return s
}

// 区间和 [l, r]，l > r 时返回 0
func (f *fenwick) rangeSum(l, r int) int {
	if l > r {
		return 0
	}
	s := f.query(r)
	if l > 0 {
		s -= f.query(l - 1) // l=0 时没有前缀要减
	}
	return s
}

func brutePrefix(a []int, idx int) int {
	s := 0
	for i := 0; i <= idx; i++ {
		s += a[i]
	}
	return s
}

func bruteRange(a []int, l, r int) int {
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
	// 1. 基本用例：前缀和 / 区间和
	{
		f := newFenwick([]int{1, 2, 3, 4, 5})
		check(f.query(0) == 1, "prefix 0")
		check(f.query(2) == 6, "prefix 2")
		check(f.query(4) == 15, "prefix 4")
		check(f.rangeSum(1, 3) == 9, "range 1..3")
		check(f.rangeSum(0, 4) == 15, "range all")
		check(f.rangeSum(2, 2) == 3, "range single")
		check(f.rangeSum(3, 2) == 0, "empty range")
		fmt.Println("total:", f.query(4))
	}

	// 2. O(n) 建树必须与逐个 update 的结果完全一致
	{
		a := []int{4, -3, 7, 0, 2, -8, 5, 1, 9, -6, 3}
		fast := newFenwick(a)
		slow := newFenwickZeros(len(a))
		for i := 0; i < len(a); i++ {
			slow.update(i, a[i])
		}
		for i := 0; i < len(a); i++ {
			check(fast.query(i) == slow.query(i), "build vs update")
			check(fast.query(i) == brutePrefix(a, i), "build vs brute")
		}
	}

	// 3. n = 1
	{
		f := newFenwick([]int{7})
		check(f.query(0) == 7, "n=1 prefix")
		check(f.rangeSum(0, 0) == 7, "n=1 range")
		f.update(0, 3)
		check(f.query(0) == 10, "n=1 add")
		f.update(0, -10)
		check(f.query(0) == 0, "n=1 subtract")
	}

	// 4. 全 0 与负数
	{
		f := newFenwick([]int{0, 0, 0, 0})
		check(f.query(3) == 0, "zeros prefix")
		check(f.rangeSum(0, 3) == 0, "zeros range")
		f.update(2, 5)
		check(f.rangeSum(0, 3) == 5, "zeros after update")

		g := newFenwick([]int{-1, -2, -3})
		check(g.query(2) == -6, "neg prefix")
		check(g.rangeSum(0, 1) == -3, "neg range")
		check(g.rangeSum(1, 2) == -5, "neg range 2")
		g.update(1, 2) // -2 变成 0
		check(g.query(2) == -4, "neg after update")
	}

	// 5. 连续更新后的区间和：加加减减必须对得上
	{
		a := []int{10, 20, 30, 40, 50, 60}
		f := newFenwick(a)
		f.update(0, -10)
		f.update(5, 5)
		f.update(2, 100)
		f.update(2, -100)
		a[0] -= 10
		a[5] += 5
		for l := 0; l < 6; l++ {
			for r := l; r < 6; r++ {
				check(f.rangeSum(l, r) == bruteRange(a, l, r), "range loop")
			}
		}
	}

	// 6. 与暴力对拍：固定种子随机单点加 + 随机前缀 / 区间查询
	{
		const n = 37
		a := make([]int, n)
		f := newFenwickZeros(n)
		var seed uint32 = 24680
		for step := 0; step < 500; step++ {
			seed = seed*1103515245 + 12345
			x := int((seed >> 16) % 1000)
			seed = seed*1103515245 + 12345
			y := int((seed >> 16) % 1000)
			if step%2 == 0 {
				idx := x % n
				delta := y%200 - 100
				a[idx] += delta
				f.update(idx, delta)
			} else if step%4 == 1 {
				idx := x % n
				check(f.query(idx) == brutePrefix(a, idx), "random prefix")
			} else {
				l, r := x%n, y%n
				if l > r {
					l, r = r, l
				}
				check(f.rangeSum(l, r) == bruteRange(a, l, r), "random range")
			}
		}
		// 最后整体核对每一个前缀
		for i := 0; i < n; i++ {
			check(f.query(i) == brutePrefix(a, i), "final prefix")
		}
		fmt.Println("random total:", f.query(n-1))
	}

	fmt.Println("all tests passed")
}
