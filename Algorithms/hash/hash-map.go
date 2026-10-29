/*
============================================================================
手写哈希表（Hash Map，链地址法）
分类：手写数据结构 · 哈希表
操作复杂度：

	put      O(1) 均摊  负载因子超过 0.75 就 2 倍扩容 rehash
	get      O(1) 均摊  最坏 O(n)，所有 key 挤在同一个桶
	remove   O(1) 均摊  单链表摘除，最坏 O(n)
	clear    O(n)       逐个释放节点

============================================================================

用途：

	需要 O(1) 增删查的 key-value 场景。面试里常要求手写链地址法，
	并追问扩容、rehash、负载因子、为什么容量取 2 的幂。

核心思想：

	桶数组的每个位置挂一条单链表，哈希值决定挂在哪个桶。容量是 2 的幂，
	所以取下标用 h & (cap-1) 代替取模（取模是整数除法，慢一个量级）。
	元素多了就 2 倍扩容并把每个节点重新挂到新桶，元素少了就缩容。

关键点（不变量）：

	· 任何时刻 size <= cap * 0.75，超了就扩容，保证链不太长。
	· 扩容后每个节点必须重新算下标：mask 从 cap-1 变成 2*cap-1，
	  同一个 key 的低位变了，旧桶号完全可能失效，照搬就会丢数据。
	· 头插会让同一个桶里的顺序与插入顺序相反，但查找是「整条链扫完」，
	  顺序不影响正确性；只有需要遍历有序性时才要改成尾插。
	· remove 必须拿到前驱再改 next，别把后面的整条链一起丢掉。

手推一遍：

	初始容量 8，mask = 7，依次 put(1,10)、put(3,30)、put(10,100)：
	1. mix(1) & 7 = 0，桶 0 空，挂上 -> 桶 0: 1。
	2. mix(3) & 7 = 5，桶 5 空，挂上 -> 桶 5: 3。
	3. mix(10) & 7 = 5，与 3 撞桶，头插 -> 桶 5: 10 -> 3。
	get(3)：先算到桶 5，沿链比 10 不等，再比 3 命中，返回 30。
	注意链上是 10 在前、3 在后，与插入顺序相反，但结果一样。

复杂度分析：

	时间 O(1) 均摊：负载因子封顶 0.75，链长期望长度是常数；扩容代价
	分摊到每次 put，等价 O(1)。最坏 O(n) 是所有 key 同桶。
	空间 O(n)：桶数组 cap 个指针 + 每个元素一个节点，cap <= 4n/3 左右。
	稳定性：不涉及顺序，遍历顺序由哈希值决定，不可依赖。
*/
package main

import "fmt"

type hashNode struct {
	key   int
	value int
	next  *hashNode
}

type HashMap struct {
	buckets []*hashNode
	cap     int
	size    int
}

const minCapacity = 8

func NewHashMap() *HashMap {
	return &HashMap{buckets: make([]*hashNode, minCapacity), cap: minCapacity}
}

func (m *HashMap) Size() int     { return m.size }
func (m *HashMap) Capacity() int { return m.cap }

// 已存在就覆盖值，不算新键，也不触发扩容。
func (m *HashMap) Put(key, value int) {
	index := m.indexOf(key)
	for node := m.buckets[index]; node != nil; node = node.next {
		if node.key == key {
			node.value = value
			return
		}
	}
	// 先判满再插：size+1 个元素必须装得下 0.75 的负载因子。
	if (m.size+1)*4 > m.cap*3 {
		m.rehash(m.cap * 2)
		index = m.indexOf(key)
	}
	m.buckets[index] = &hashNode{key: key, value: value, next: m.buckets[index]}
	m.size++
}

func (m *HashMap) Get(key int) (int, bool) {
	for node := m.buckets[m.indexOf(key)]; node != nil; node = node.next {
		if node.key == key {
			return node.value, true
		}
	}
	return 0, false
}

func (m *HashMap) Contains(key int) bool {
	_, ok := m.Get(key)
	return ok
}

func (m *HashMap) Remove(key int) bool {
	index := m.indexOf(key)
	var prev *hashNode
	node := m.buckets[index]
	for node != nil {
		if node.key == key {
			if prev != nil {
				prev.next = node.next
			} else {
				m.buckets[index] = node.next
			}
			m.size--
			m.maybeShrink()
			return true
		}
		prev = node
		node = node.next
	}
	return false
}

func (m *HashMap) Clear() {
	for i := 0; i < m.cap; i++ {
		m.buckets[i] = nil
	}
	m.size = 0
}

// 仅供自测：扫全部桶，检查每个节点都在正确的桶里、节点数等于 size。
func (m *HashMap) CheckInvariants() bool {
	counted := 0
	for i := 0; i < m.cap; i++ {
		for node := m.buckets[i]; node != nil; node = node.next {
			counted++
			if m.indexOf(node.key) != i {
				return false
			}
		}
	}
	if counted != m.size {
		return false
	}
	return m.size*4 <= m.cap*3
}

// 对 int 的 32 位模式做乘法散列（murmur 风格的 finalizer）。
// 先当无符号数看，符号位不再有特殊含义，负数 key 也能散匀。
func mix(key int) uint32 {
	h := uint32(key)
	h ^= h >> 16
	h *= 0x7feb352d
	h ^= h >> 15
	h *= 0x846ca68b
	h ^= h >> 16
	return h
}

// 容量是 2 的幂，用 & 代替 %。
func (m *HashMap) indexOf(key int) int {
	return int(mix(key) & uint32(m.cap-1))
}

func (m *HashMap) rehash(newCap int) {
	fresh := make([]*hashNode, newCap)
	mask := uint32(newCap - 1)
	for i := 0; i < m.cap; i++ {
		for node := m.buckets[i]; node != nil; {
			next := node.next
			// mask 变了，下标必须重算；沿用旧下标会把节点放错桶。
			index := mix(node.key) & mask
			node.next = fresh[index]
			fresh[index] = node
			node = next
		}
	}
	m.buckets = fresh
	m.cap = newCap
}

func (m *HashMap) maybeShrink() {
	// 低于 0.25 才缩，且不低于最小容量；否则 Put/Remove 交替抖动。
	if m.cap > minCapacity && m.size*4 < m.cap {
		m.rehash(m.cap / 2)
	}
}

func isPowerOfTwo(value int) bool {
	return value > 0 && value&(value-1) == 0
}

func check(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
}

func main() {
	// 1. 正常用例：Put/Get/Contains，重复 Put 只覆盖不新增。
	{
		m := NewHashMap()
		check(m.Size() == 0 && m.Capacity() == 8, "fresh map state")

		m.Put(42, 420)
		m.Put(7, 70)
		check(m.Size() == 2, "size after two puts")

		v, ok := m.Get(42)
		check(ok && v == 420, "get 42")
		v, ok = m.Get(7)
		check(ok && v == 70, "get 7")
		check(m.Contains(7), "contains 7")
		check(!m.Contains(8), "not contains 8")

		m.Put(42, 999)
		check(m.Size() == 2, "overwrite keeps size")
		v, ok = m.Get(42)
		check(ok && v == 999, "overwrite value")
		check(m.CheckInvariants(), "invariants after basics")
	}

	// 2. 边界用例：负数 key、int 两端极值。
	{
		m := NewHashMap()
		extremes := []int{0, -1, -1024, 1024,
			-2147483648, 2147483647, -2147483647}
		for _, key := range extremes {
			// 用异或造值，避免最小负数再乘 2 溢出。
			m.Put(key, key^0x5a5a)
		}
		check(m.Size() == len(extremes), "extremes size")
		for _, key := range extremes {
			v, ok := m.Get(key)
			check(ok, "extremes get")
			check(v == key^0x5a5a, "extremes value")
		}
		if _, ok := m.Get(1); ok {
			panic("key 1 should be missing")
		}
		check(m.CheckInvariants(), "invariants after extremes")
	}

	// 3. 关键点用例：同桶链表（头插逆行）也能查到、摘除中间节点不断链。
	{
		// 容量 8 时 mix(3)、mix(10)、mix(17) 的低三位都是 5，必然同桶。
		m := NewHashMap()
		check(m.Capacity() == 8, "initial capacity")
		m.Put(3, 30)
		m.Put(10, 100)
		m.Put(17, 170)
		check(m.Capacity() == 8, "no growth for 3 keys")
		check(m.Size() == 3, "three colliding keys")
		check(m.CheckInvariants(), "collision invariants")

		v, ok := m.Get(3)
		check(ok && v == 30, "collision get 3")
		v, ok = m.Get(10)
		check(ok && v == 100, "collision get 10")
		v, ok = m.Get(17)
		check(ok && v == 170, "collision get 17")

		// 10 在链中间，摘掉它必须把前后接上。
		check(m.Remove(10), "remove middle node")
		check(!m.Contains(10), "removed key gone")
		check(m.Size() == 2, "size after middle remove")
		v, ok = m.Get(3)
		check(ok && v == 30, "get 3 after remove")
		v, ok = m.Get(17)
		check(ok && v == 170, "get 17 after remove")
		check(m.CheckInvariants(), "invariants after middle remove")

		// Remove 不存在的 key 必须返回 false 且不动 size。
		check(!m.Remove(10), "remove again")
		check(!m.Remove(12345), "remove missing")
		check(m.Size() == 2, "size unchanged by failed remove")
		check(m.CheckInvariants(), "invariants after failed remove")
	}

	// 4. 大量 key：触发多次扩容，全部仍可查到；随后缩容也不丢数据。
	{
		m := NewHashMap()
		const total = 5000
		for i := 0; i < total; i++ {
			m.Put(i, i*3+1)
		}
		check(m.Size() == total, "bulk size")
		check(isPowerOfTwo(m.Capacity()), "capacity is power of two")
		check(m.Capacity() >= 8192, "capacity grew enough")
		check(m.CheckInvariants(), "bulk invariants")

		for i := 0; i < total; i++ {
			v, ok := m.Get(i)
			check(ok && v == i*3+1, "bulk get")
		}

		// 删掉一半，剩下的必须还能查到（扩容/缩容都要重算下标）。
		for i := 0; i < total; i += 2 {
			check(m.Remove(i), "bulk remove even")
		}
		check(m.Size() == total/2, "half size")
		for i := 0; i < total; i++ {
			v, ok := m.Get(i)
			check(ok == (i%2 == 1), "half presence")
			if ok {
				check(v == i*3+1, "half value")
			}
		}
		check(m.CheckInvariants(), "half invariants")

		bigCapacity := m.Capacity()
		for i := 1; i < total; i += 2 {
			if i >= 1000 {
				check(m.Remove(i), "shrink remove")
			}
		}
		check(m.Size() == 500, "shrink size")
		check(m.Capacity() < bigCapacity, "capacity shrank")
		check(isPowerOfTwo(m.Capacity()), "shrunk capacity power of two")
		for i := 1; i < total; i += 2 {
			check(m.Contains(i) == (i < 1000), "post shrink presence")
		}
		check(m.CheckInvariants(), "post shrink invariants")

		// Clear 之后 size 归零、旧 key 全部查不到，且能继续使用。
		m.Clear()
		check(m.Size() == 0, "clear size")
		check(!m.Contains(1), "clear removed keys")
		check(m.CheckInvariants(), "clear invariants")

		m.Put(123, 456)
		check(m.Size() == 1, "reuse after clear")
		v, ok := m.Get(123)
		check(ok && v == 456, "reuse value")
		check(m.CheckInvariants(), "reuse invariants")
	}

	fmt.Println("all tests passed")
}
