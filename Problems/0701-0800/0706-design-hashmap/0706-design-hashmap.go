/*
============================================================================
706. 设计哈希映射（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/design-hashmap/
分类：哈希表 · 设计（链地址法，自实现桶数组与链表）

不使用任何内建的哈希表库设计一个哈希映射（HashMap）。

实现 MyHashMap 类：

- MyHashMap() 用空映射初始化对象
- void put(int key, int value) 向 HashMap 插入一个键值对 (key, value) 。
  如果 key 已经存在于映射中，则更新其对应的值 value 。
- int get(int key) 返回特定的 key 所映射的 value ；如果映射中不包含 key 的映射，
  返回 -1 。
- void remove(key) 如果映射中存在 key 的映射，则移除 key 和它所对应的 value 。

示例：
输入：
["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
[[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
输出：
[null, null, null, 1, -1, null, 1, null, -1]

解释：
MyHashMap myHashMap = new MyHashMap();
myHashMap.put(1, 1); // myHashMap 现在为 [[1,1]]
myHashMap.put(2, 2); // myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(1);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
myHashMap.get(3);    // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
myHashMap.put(2, 1); // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
myHashMap.get(2);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
myHashMap.remove(2); // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
myHashMap.get(2);    // 返回 -1（未找到），myHashMap 现在为 [[1,1]]

提示：
- 0 <= key, value <= 10^6
- 最多调用 10^4 次 put、get 和 remove 方法
*/
package main

import "fmt"

/*
解法：链地址法（自实现桶数组 + 单向链表）【最优解】

官方题解只给出这一种方法，也就是本题的最优解：单次操作平均 O(1)，空间 O(n+b)。

接口契约：
  · put 是「有则改、无则插」，不能插出重复 key
  · get 用 -1 表示不存在 —— value 非负，所以 -1 不会与合法值冲突
  · remove 只删 key，不存在时什么也不做，不能报错
  · 全程不许用语言内建的哈希表，只能用数组加自己写的链表

数据结构选型：
  · 直接把 key 当下标开数组，单次 O(1)，但要开 10^6 + 1 个格子，而调用最多
    10^4 次，空间利用率不到百分之一
  · 有序数组加二分查找，单次 O(log n)，插入还要搬动元素
  · 桶数组加链表（链地址法）：769 个桶，10^4 个 key 平均每桶约 13 个节点，
    单次 O(n/b)；桶数取质数是为了让 key % b 分布更均匀

不变式：
  · 同一个 key 在整张表里最多出现一次：Put 先查后插，查不到才新建节点
  · 每个节点的 next 要么为空，要么指向同一个桶里的另一个节点
  · buckets[h] 始终指向该桶链表的头节点；删除时改的是「指向这条边的指针」
    而不是节点本身，于是头节点和中间节点可以共用一段代码

关键点：
  · Put 命中已有 key 时只改 value，绝不新建节点，否则同一个 key 会出现两次
  · Remove 用二级指针沿链前进：命中的那一刻 *link 正好是「指向该节点的边」，
    把它改成 node.next 即可，不需要单独记住前驱
  · Get 与 Remove 必须沿 next 走完整条链才能断定 key 不存在
  · 空映射时每个桶头都是 nil，哈希函数只做取模，不依赖 key 是否出现过
  · Go 的切片本身可以就地修改，桶数组用 make 一次分配好即可

以题面示例（Put 1→1、Put 2→2、Get 1、Get 3、Put 2→1、Get 2、Remove 2、Get 2）：
  Put(1,1)、Put(2,2)：两个桶各挂一个节点
  Get(1) → 1
  Get(3) → 该桶为空 → -1
  Put(2,1)：命中已有节点，只把 value 改成 1
  Get(2) → 1
  Remove(2)：从链上摘掉该节点
  Get(2) → -1 ✔

复杂度分析：
  时间：Put / Get / Remove 平均 O(n/b)，b = 769 固定，10^4 次调用下平均每桶约
    13 个节点，可视为 O(1)；最坏（key 全部同余）退化为 O(n)
  空间 O(n + b)：节点数等于不同 key 的个数，另加 769 个桶头指针
*/
const base = 769

// entry：链地址法里的链表节点，同时存 key 与 value
type entry struct {
	key, value int
	next       *entry
}

type MyHashMap struct {
	buckets []*entry
}

func hashKey(key int) int {
	return key % base
}

// 平台签名：func Constructor() MyHashMap
func Constructor() MyHashMap {
	return MyHashMap{buckets: make([]*entry, base)}
}

func (m *MyHashMap) Put(key int, value int) {
	h := hashKey(key)
	for cur := m.buckets[h]; cur != nil; cur = cur.next {
		if cur.key == key {
			// 命中也只改值：key 在表里必须唯一
			cur.value = value
			return
		}
	}
	// 头插：新节点放在链首，省掉找链尾的遍历
	m.buckets[h] = &entry{key: key, value: value, next: m.buckets[h]}
}

func (m *MyHashMap) Get(key int) int {
	for cur := m.buckets[hashKey(key)]; cur != nil; cur = cur.next {
		if cur.key == key {
			return cur.value
		}
	}
	// 链走完仍未命中，才说明 key 不存在
	return -1
}

func (m *MyHashMap) Remove(key int) {
	h := hashKey(key)
	// link 指「指向当前节点的边」，命中时改它即可跳过该节点
	link := &m.buckets[h]
	for *link != nil {
		if (*link).key == key {
			*link = (*link).next
			return
		}
		link = &(*link).next
	}
}

// ===== 测试辅助 =====
// 测试对照：用线性查找的键值对列表模拟，只负责核对结果
type naiveMap struct {
	items [][2]int
}

func (n *naiveMap) put(key, value int) {
	for i := range n.items {
		if n.items[i][0] == key {
			n.items[i][1] = value
			return
		}
	}
	n.items = append(n.items, [2]int{key, value})
}

func (n *naiveMap) get(key int) int {
	for _, kv := range n.items {
		if kv[0] == key {
			return kv[1]
		}
	}
	return -1
}

func (n *naiveMap) remove(key int) {
	for i, kv := range n.items {
		if kv[0] == key {
			n.items = append(n.items[:i], n.items[i+1:]...)
			return
		}
	}
}

func mustEqual(got, want int, what string) {
	if got != want {
		panic(fmt.Sprintf("%s = %d, want %d", what, got, want))
	}
}

func replayExample() {
	m := Constructor()
	m.Put(1, 1)
	m.Put(2, 2)
	mustEqual(m.Get(1), 1, "get(1)")
	mustEqual(m.Get(3), -1, "get(3)")
	m.Put(2, 1)
	mustEqual(m.Get(2), 1, "get(2) after overwrite")
	m.Remove(2)
	mustEqual(m.Get(2), -1, "get(2) after remove")
}

func checkCollisions() {
	m := Constructor()
	// 三个 key 对 769 同余，必然落在同一个桶里
	m.Put(0, 10)
	m.Put(769, 20)
	m.Put(1538, 30)
	mustEqual(m.Get(0), 10, "get(0)")
	mustEqual(m.Get(769), 20, "get(769)")
	mustEqual(m.Get(1538), 30, "get(1538)")

	// 删桶头
	m.Remove(0)
	mustEqual(m.Get(0), -1, "get(0) after remove")
	mustEqual(m.Get(769), 20, "get(769) after head remove")
	mustEqual(m.Get(1538), 30, "get(1538) after head remove")

	// 删链中间
	m.Remove(769)
	mustEqual(m.Get(769), -1, "get(769) after middle remove")
	mustEqual(m.Get(1538), 30, "get(1538) after middle remove")

	// 删不存在的 key 不应报错，也不应影响其它节点
	m.Remove(769)
	m.Remove(1000000)
	mustEqual(m.Get(1538), 30, "get(1538) after missing remove")
}

func crossCheck() {
	fast := Constructor()
	slow := naiveMap{}
	seed := uint32(20240607)
	for step := 0; step < 3000; step++ {
		seed = seed*1103515245 + 12345
		op := int((seed >> 16) % 3)
		// 让 key 集中在几个同余类上，制造长链
		key := int((seed>>8)%8)*769 + int((seed>>20)%3)
		value := int((seed >> 4) % 1000)

		switch op {
		case 0:
			fast.Put(key, value)
			slow.put(key, value)
		case 1:
			mustEqual(fast.Get(key), slow.get(key), "cross get")
		default:
			fast.Remove(key)
			slow.remove(key)
		}
	}
}

func main() {
	// 示例：按题目给的调用序列回放
	replayExample()

	// 边界：同桶多节点的增删
	checkCollisions()

	// 边界：value 为 0 与「不存在」的 -1 必须区分开
	{
		m := Constructor()
		m.Put(1000000, 0)
		mustEqual(m.Get(1000000), 0, "get(1000000)")
		mustEqual(m.Get(0), -1, "get(0) on empty bucket")
	}

	// 边界：同一个 key 反复覆盖，值只保留最后一次
	{
		m := Constructor()
		for v := 0; v < 5; v++ {
			m.Put(42, v)
		}
		mustEqual(m.Get(42), 4, "get(42) after overwrites")
	}

	// 随机交叉验证
	crossCheck()

	fmt.Println("all tests passed")
}
