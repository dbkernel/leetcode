/*
============================================================================
面试题 03.06. 动物收容所（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/animal-shelter-lcci/
分类：队列 · 设计（先进先出）

动物收容所。有家动物收容所只收容狗与猫，且严格遵守“先进先出”的原则。在收
养该收容所的动物时，收养人只能收养所有动物中“最老”（由其进入收容所的时间
长短而定）的动物，或者可以挑选猫或狗（同时必须收养此类动物中“最老”的）。
换言之，收养人不能自由挑选想收养的对象。请创建适用于这个系统的数据结构，
实现各种操作方法，比如enqueue、dequeueAny、dequeueDog和dequeueCat。允许
使用Java内置的LinkedList数据结构。

enqueue方法有一个animal参数，animal[0]代表动物编号，animal[1]代表动物种
类，其中 0 代表猫，1 代表狗。

dequeue*方法返回一个列表[动物编号, 动物种类]，若没有可以收养的动物，则返
回[-1,-1]。

示例 1：
输入：
["AnimalShelf", "enqueue", "enqueue", "dequeueCat",
"dequeueDog", "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [], [], []]
输出：
[null,null,null,[0,0],[-1,-1],[1,0]]

示例 2：
输入：
["AnimalShelf", "enqueue", "enqueue", "enqueue", "dequeueDog",
"dequeueCat", "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [[2, 1]], [], [], []]
输出：
[null,null,null,null,[2,1],[0,0],[1,0]]

说明：
收纳所的最大容量为20000
*/
package main

import (
	"fmt"
	"slices"
)

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出标准实现并说明取舍。
另一种常见写法是「一条总队列 + 按类型线性跳过最老的异类动物」，它让
dequeueDog / dequeueCat 最坏 O(n) 且需要额外的临时结构，所以不取。

解法：猫狗各一条队列 + 全局入所序号【最优解】

接口契约：
  · enqueue(animal)：animal = [编号, 种类]，0 是猫、1 是狗，返回 void
  · dequeueAny()：取所有动物中最老的一只，返回 [编号, 种类]
  · dequeueDog() / dequeueCat()：取该类里最老的一只，返回 [编号, 种类]
  · 没有可取动物时统一返回 [-1, -1]

数据结构选型：
  「最老」按入所先后排序，所以每种动物各用一条 FIFO 队列，队首天然就是该
  类最老的一只。dequeueAny 需要跨类比较谁更老，而动物编号并不保证递增，
  不能拿编号当时间戳，于是额外维护一个全局自增的序号，入所时一起入队。
  若用一条总队列，dequeueDog 就得跳过排在前面的猫，只能逐个弹出暂存再放回，
  最坏 O(n)；两条队列把四个操作都压到 O(1)。

各操作的不变量：
  · 每条队列只放同类动物，队首是该类最老的一只
  · 每条队列里的序号严格递增，所以比较两条队首的序号就是比较年龄
  · dequeueAny 从序号更小的那条队列出队，序号相同不可能出现（全局自增）
  · 三个出队操作都只动队首，不会破坏队列内的顺序
  · 两条队列都空时返回 [-1, -1]，与"这一类没有动物"的返回值一致
  · Go 用切片模拟队列，出队写成 q = q[1:]，不再拷贝元素

手推一遍（示例 2）：
  enqueue([0,0])：猫队列 [(0, 0)]
  enqueue([1,0])：猫队列 [(0, 0), (1, 1)]
  enqueue([2,1])：狗队列 [(2, 2)]
  dequeueDog  → 狗队首 → [2,1]
  dequeueCat  → 猫队首 → [0,0]
  dequeueAny  → 猫队首序号更小 → [1,0]

复杂度分析：
  时间：enqueue / dequeueAny / dequeueDog / dequeueCat 均 O(1)
  空间 O(n)：n 为当前在所的动物数，每只动物只存一个 (序号, 编号)
*/

// 队列元素：入所序号 + 动物编号
type shelfAnimal struct {
	seq int
	id  int
}

type AnimalShelf struct {
	cats []shelfAnimal
	dogs []shelfAnimal
	seq  int
}

func Constructor() AnimalShelf {
	return AnimalShelf{}
}

// 平台签名：入参是 []int，animal[0] 编号、animal[1] 种类
func (this *AnimalShelf) Enqueue(animal []int) {
	// 序号与编号分开存：编号不保证递增，不能用它判断谁更老
	item := shelfAnimal{seq: this.seq, id: animal[0]}
	if animal[1] == 0 {
		this.cats = append(this.cats, item)
	} else {
		this.dogs = append(this.dogs, item)
	}
	this.seq++
}

func (this *AnimalShelf) DequeueAny() []int {
	// 两条都空才算没有动物；只有一类空时取另一类的最老
	if len(this.cats) == 0 && len(this.dogs) == 0 {
		return []int{-1, -1}
	}
	if len(this.dogs) == 0 {
		return this.DequeueCat()
	}
	if len(this.cats) == 0 {
		return this.DequeueDog()
	}
	// 序号小的入所更早，先被收养
	if this.cats[0].seq < this.dogs[0].seq {
		return this.DequeueCat()
	}
	return this.DequeueDog()
}

func (this *AnimalShelf) DequeueDog() []int {
	if len(this.dogs) == 0 {
		return []int{-1, -1}
	}
	id := this.dogs[0].id
	this.dogs = this.dogs[1:]
	return []int{id, 1}
}

func (this *AnimalShelf) DequeueCat() []int {
	if len(this.cats) == 0 {
		return []int{-1, -1}
	}
	id := this.cats[0].id
	this.cats = this.cats[1:]
	return []int{id, 0}
}

type shelfOp struct {
	name string
	arg  []int
}

// 按题目给的调用序列回放；void 方法在结果里留一个 nil
func runShelf(ops []shelfOp) [][]int {
	shelf := Constructor()
	out := make([][]int, 0, len(ops))
	for _, op := range ops {
		switch op.name {
		case "enqueue":
			shelf.Enqueue(op.arg)
			out = append(out, nil)
		case "dequeueAny":
			out = append(out, shelf.DequeueAny())
		case "dequeueDog":
			out = append(out, shelf.DequeueDog())
		default:
			out = append(out, shelf.DequeueCat())
		}
	}
	return out
}

func checkShelf(ops []shelfOp, want [][]int) {
	got := runShelf(ops)
	if len(got) != len(want) {
		panic(fmt.Sprintf("长度不符：got %v, want %v", got, want))
	}
	for i := range got {
		if !slices.Equal(got[i], want[i]) {
			panic(fmt.Sprintf("第 %d 步：got %v, want %v", i, got, want))
		}
	}
}

// 对照实现：一条按入所顺序的线性表，删除时直接搬元素，用来验证最优解
type shelfReference struct {
	all [][2]int // (编号, 种类)，下标即入所顺序
}

func (r *shelfReference) enqueue(animal []int) {
	r.all = append(r.all, [2]int{animal[0], animal[1]})
}

// type 取 -1 表示任意种类
func (r *shelfReference) dequeue(kind int) []int {
	for i := 0; i < len(r.all); i++ {
		if kind >= 0 && r.all[i][1] != kind {
			continue
		}
		id, t := r.all[i][0], r.all[i][1]
		r.all = append(r.all[:i], r.all[i+1:]...)
		return []int{id, t}
	}
	return []int{-1, -1}
}

// 随机交叉验证：编号故意乱序，专门验证"按入所顺序"而不是"按编号"
func crossCheckShelf() {
	shelf := Constructor()
	ref := &shelfReference{}
	seed := uint32(20240607)
	for i := 0; i < 3000; i++ {
		seed = seed*1103515245 + 12345
		r := int((seed >> 16) % 10)
		if r < 4 {
			seed = seed*1103515245 + 12345
			// 编号乱序：先入所的动物编号可能更大
			animal := []int{int((seed >> 16) % 100), int((seed >> 8) % 2)}
			shelf.Enqueue(animal)
			ref.enqueue(animal)
			continue
		}

		var got, want []int
		switch int((seed >> 16) % 3) {
		case 0:
			got, want = shelf.DequeueAny(), ref.dequeue(-1)
		case 1:
			got, want = shelf.DequeueDog(), ref.dequeue(1)
		default:
			got, want = shelf.DequeueCat(), ref.dequeue(0)
		}
		if !slices.Equal(got, want) {
			panic(fmt.Sprintf("第 %d 步：got %v, want %v", i, got, want))
		}
	}
}

func main() {
	// 示例 1：两只猫，dequeueDog 先返回空，再取走剩下的那只猫
	checkShelf([]shelfOp{
		{"enqueue", []int{0, 0}},
		{"enqueue", []int{1, 0}},
		{"dequeueCat", nil},
		{"dequeueDog", nil},
		{"dequeueAny", nil},
	}, [][]int{nil, nil, {0, 0}, {-1, -1}, {1, 0}})

	// 示例 2：先取狗不影响猫的先后顺序
	checkShelf([]shelfOp{
		{"enqueue", []int{0, 0}},
		{"enqueue", []int{1, 0}},
		{"enqueue", []int{2, 1}},
		{"dequeueDog", nil},
		{"dequeueCat", nil},
		{"dequeueAny", nil},
	}, [][]int{nil, nil, nil, {2, 1}, {0, 0}, {1, 0}})

	// 边界：空所连续出队，全部返回 [-1,-1]
	checkShelf([]shelfOp{
		{"dequeueAny", nil},
		{"dequeueDog", nil},
		{"dequeueCat", nil},
	}, [][]int{{-1, -1}, {-1, -1}, {-1, -1}})

	// 反直觉：dequeueDog 必须跳过排在前面的猫，但不能把猫弄丢
	checkShelf([]shelfOp{
		{"enqueue", []int{0, 0}},
		{"enqueue", []int{1, 1}},
		{"enqueue", []int{2, 0}},
		{"enqueue", []int{3, 1}},
		{"dequeueDog", nil},
		{"dequeueAny", nil},
		{"dequeueAny", nil},
		{"dequeueAny", nil},
		{"dequeueAny", nil},
	}, [][]int{nil, nil, nil, nil, {1, 1}, {0, 0}, {2, 0}, {3, 1}, {-1, -1}})

	// 反直觉：编号不按入所顺序时，dequeueAny 看的是入所先后而不是编号
	checkShelf([]shelfOp{
		{"enqueue", []int{7, 1}},
		{"enqueue", []int{2, 0}},
		{"dequeueAny", nil},
		{"dequeueCat", nil},
	}, [][]int{nil, nil, {7, 1}, {2, 0}})

	// 边界：只有狗，猫的那一路始终返回 [-1,-1]
	checkShelf([]shelfOp{
		{"enqueue", []int{5, 1}},
		{"dequeueCat", nil},
		{"dequeueAny", nil},
		{"dequeueAny", nil},
	}, [][]int{nil, {-1, -1}, {5, 1}, {-1, -1}})

	// 边界：取空之后还能继续入所并按新顺序出队
	checkShelf([]shelfOp{
		{"enqueue", []int{1, 0}},
		{"dequeueAny", nil},
		{"enqueue", []int{2, 1}},
		{"enqueue", []int{3, 0}},
		{"dequeueAny", nil},
		{"dequeueCat", nil},
	}, [][]int{nil, {1, 0}, nil, nil, {2, 1}, {3, 0}})

	// 随机交叉验证
	crossCheckShelf()

	fmt.Println("all tests passed")
}
