/*
============================================================================
LCR 184 / 剑指 Offer 59 - II. 设计自助结算系统（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/dui-lie-de-zui-da-zhi-lcof/
分类：队列 · 设计（单调双端队列）

请设计一个自助结账系统，该系统需要通过一个队列来模拟顾客通过购物车的结算
过程，需要实现的功能有：

- get_max()：获取结算商品中的最高价格，如果队列为空，则返回 -1
- add(value)：将价格为 value 的商品加入待结算商品队列的尾部
- remove()：移除第一个待结算的商品价格，如果队列为空，则返回 -1

注意，为保证该系统运转高效性，以上函数的均摊时间复杂度均为 O(1)

示例 1：
输入：
["Checkout","add","add","get_max","remove","get_max"]
[[],[4],[7],[],[],[]]
输出: [null,null,null,7,4,7]

示例 2：
输入：
["Checkout","remove","get_max"]
[[],[],[]]
输出: [null,-1,-1]

提示：
- 1 <= get_max, add, remove 的总操作数 <= 10000
- 1 <= value <= 10^5

说明：本题在 leetcode.cn 上已改号为 LCR 184，题名由「队列的最大值」改为
「设计自助结算系统」，平台方法名也从 max_value / push_back / pop_front
改为 get_max / add / remove，下面按平台给出的签名实现。
*/
package main

import "fmt"

/*
解法一：普通队列 + 线性扫描求最大值【非最优】

为什么慢：队列只允许两头操作，没有任何"局部信息"能直接给出最大值，于是
get_max 每次都得从头扫到尾，单次 O(n)。m 次操作里有大量 get_max 时退化
成 O(m·n)。

换个角度：先把 FIFO 语义做对 —— 用一个只前进的 head 游标记录队首，避免
remove 时整体搬移元素；最大值则老老实实扫描。这是正确但慢的基线，后面
的单调队列正是在它的基础上把"扫描"换成了"增量维护"。

关键点：
  · remove 返回被移除的那个价格，而不是无返回值；空队列返回 -1
  · get_max 在空队列上也返回 -1，与"最大值恰好是 -1"不会混淆（价格 ≥ 1）
  · 用 head 游标记录队首，remove 时只把 head 前移，不必搬移元素
  · head 只前进，data 只追加，所以扫描区间是 data[head:]

以示例 1（add 4、add 7、get_max、remove、get_max）为例：
  add 4：data = [4]
  add 7：data = [4, 7]
  get_max：扫描得 7
  remove：返回 4，head 移到 1
  get_max：扫描 data[1:] 得 7 ✔

复杂度分析：
  时间：add O(1)、remove O(1)、get_max O(n)（n 为当前队列长度）
  空间 O(n)：保存所有尚未出队的元素；head 只前进，最坏存下全部历史元素

解法二：单调双端队列，与普通队列同步进出【最优解】

为什么慢：解法一每次都重扫，但两次 get_max 之间队列只可能"尾部加一个"或
"头部删一个"，真正影响最大值的信息变化极小。重扫等于把没变的部分又算一遍。

换个角度：若 i 排在 j 前面且 value[i] <= value[j]，那么 j 一定比 i 晚出队，
i 在 j 存在期间永远当不了最大值，可以直接丢弃。反复丢弃后，候选集合里剩
下的值必然"从队首到队尾严格递减"，队首就是当前最大值。

关键点：
  · add 时从队尾弹掉所有**严格小于**新值的元素：相等必须保留 —— 重复的
    最大值要一个一个等着被 remove 走，弹掉就少了一份
  · remove 时先按 FIFO 弹出队首元素，只有它恰好等于单调队列的队首，才同步
    弹出单调队列的队首；不相等说明它早被"更大的后来者"压掉了
  · 单调队列只存值就够了，不需要下标：remove 的顺序与值无关，靠"相等"就能
    对齐两个队列
  · 两个队列都空时 remove / get_max 返回 -1；只要普通队列非空，单调队列必非空
  · Go 的切片两头都弹：尾部 q = q[:len(q)-1]，首部 q = q[1:]

以示例 1（add 4、add 7、get_max、remove、get_max）为例：
  add 4：普通队列 [4]，单调队列 [4]
  add 7：7 比队尾 4 大，先弹掉 4；普通队列 [4,7]，单调队列 [7]
  get_max：队首 7
  remove：普通队列弹出 4，与单调队列队首 7 不等，单调队列不动 → 返回 4
  get_max：仍是 7 ✔（这正是示例输出 4、7 的来源）

复杂度分析：
  时间：add / remove / get_max 均摊 O(1)：每个值最多进、出单调队列各一次
  空间 O(n)：普通队列存所有待结算价格，单调队列最坏也存 n 个（全相等时）
*/

// 解法一：普通队列 + 线性扫描
type CheckoutBrute struct {
	data []int
	head int // 队首位置，只前进
}

func NewCheckoutBrute() CheckoutBrute {
	return CheckoutBrute{}
}

func (this *CheckoutBrute) Get_max() int {
	best := -1
	for i := this.head; i < len(this.data); i++ {
		if this.data[i] > best {
			best = this.data[i]
		}
	}
	return best
}

func (this *CheckoutBrute) Add(value int) {
	this.data = append(this.data, value)
}

func (this *CheckoutBrute) Remove() int {
	if this.head == len(this.data) {
		return -1
	}
	value := this.data[this.head]
	this.head++
	return value
}

// 解法二：单调双端队列（平台入口签名）
type Checkout struct {
	q []int // 普通 FIFO 队列，保证 remove 的顺序
	d []int // 单调队列，队首恒为当前最大值
}

func Constructor() Checkout {
	return Checkout{}
}

func (this *Checkout) Get_max() int {
	if len(this.d) == 0 {
		return -1
	}
	return this.d[0]
}

func (this *Checkout) Add(value int) {
	// 严格小于才弹：相等要保留，否则重复的最大值会少一份
	for len(this.d) > 0 && this.d[len(this.d)-1] < value {
		this.d = this.d[:len(this.d)-1]
	}
	this.d = append(this.d, value)
	this.q = append(this.q, value)
}

func (this *Checkout) Remove() int {
	if len(this.q) == 0 {
		return -1
	}
	value := this.q[0]
	this.q = this.q[1:]
	// 只有真正等于最大值时才同步弹出单调队列的队首
	if value == this.d[0] {
		this.d = this.d[1:]
	}
	return value
}

type checkoutOp struct {
	name  string
	value int // add 用；其余操作忽略
}

// 按题目给的调用序列回放；void 方法记 -2 表示 null（价格 >= 1）
func runCheckoutBrute(ops []checkoutOp) []int {
	shelf := NewCheckoutBrute()
	out := make([]int, 0, len(ops))
	for _, op := range ops {
		switch op.name {
		case "add":
			shelf.Add(op.value)
			out = append(out, -2)
		case "get_max":
			out = append(out, shelf.Get_max())
		default:
			out = append(out, shelf.Remove())
		}
	}
	return out
}

func runCheckout(ops []checkoutOp) []int {
	shelf := Constructor()
	out := make([]int, 0, len(ops))
	for _, op := range ops {
		switch op.name {
		case "add":
			shelf.Add(op.value)
			out = append(out, -2)
		case "get_max":
			out = append(out, shelf.Get_max())
		default:
			out = append(out, shelf.Remove())
		}
	}
	return out
}

func checkCheckout(ops []checkoutOp, want []int) {
	got := runCheckout(ops)
	if len(got) != len(want) {
		panic(fmt.Sprintf("长度不符：got %v, want %v", got, want))
	}
	for i := range got {
		if got[i] != want[i] {
			panic(fmt.Sprintf("单调队列 %v = %v, want %v", ops, got, want))
		}
	}
	got = runCheckoutBrute(ops)
	for i := range got {
		if got[i] != want[i] {
			panic(fmt.Sprintf("暴力 %v = %v, want %v", ops, got, want))
		}
	}
}

// 随机交叉验证：用切片当队列，最大值直接扫描，作为独立基准
func crossCheckCheckout() {
	shelf := Constructor()
	brute := NewCheckoutBrute()
	reference := []int{}
	seed := uint32(20240607)
	for i := 0; i < 3000; i++ {
		seed = seed*1103515245 + 12345
		r := int((seed >> 16) % 10)
		switch {
		case r < 5:
			seed = seed*1103515245 + 12345
			// 值域 1..5，专门制造大量重复值
			value := int((seed>>16)%5) + 1
			shelf.Add(value)
			brute.Add(value)
			reference = append(reference, value)
		case r < 8:
			want := -1
			if len(reference) > 0 {
				want = reference[0]
				reference = reference[1:]
			}
			if got := shelf.Remove(); got != want {
				panic(fmt.Sprintf("第 %d 步 remove = %d, want %d", i, got, want))
			}
			if got := brute.Remove(); got != want {
				panic(fmt.Sprintf("第 %d 步 remove(brute) = %d, want %d", i, got, want))
			}
		default:
			want := -1
			for _, v := range reference {
				if v > want {
					want = v
				}
			}
			if got := shelf.Get_max(); got != want {
				panic(fmt.Sprintf("第 %d 步 get_max = %d, want %d", i, got, want))
			}
			if got := brute.Get_max(); got != want {
				panic(fmt.Sprintf("第 %d 步 get_max(brute) = %d, want %d", i, got, want))
			}
		}
	}
}

func main() {
	// 示例 1：remove 返回被移除的价格 4，之后最大值仍是 7
	checkCheckout([]checkoutOp{
		{"add", 4}, {"add", 7}, {"get_max", 0}, {"remove", 0},
		{"get_max", 0},
	}, []int{-2, -2, 7, 4, 7})

	// 示例 2：空队列上的 remove 与 get_max 都返回 -1
	checkCheckout([]checkoutOp{{"remove", 0}, {"get_max", 0}}, []int{-1, -1})

	// 边界：取空后继续操作，状态不能残留
	checkCheckout([]checkoutOp{
		{"add", 5}, {"remove", 0}, {"get_max", 0}, {"remove", 0},
	}, []int{-2, 5, -1, -1})

	// 反直觉：最大值被取走后要回落到次大值，而不是继续报 5
	checkCheckout([]checkoutOp{
		{"add", 5}, {"add", 3}, {"add", 4}, {"get_max", 0},
		{"remove", 0}, {"get_max", 0}, {"remove", 0}, {"get_max", 0},
		{"remove", 0}, {"get_max", 0},
	}, []int{-2, -2, -2, 5, 5, 4, 3, 4, 4, -1})

	// 反直觉：两个相同的最大值必须分别等着被 remove，弹队尾用 < 而不是 <=
	checkCheckout([]checkoutOp{
		{"add", 5}, {"add", 5}, {"get_max", 0}, {"remove", 0},
		{"get_max", 0}, {"remove", 0}, {"get_max", 0},
	}, []int{-2, -2, 5, 5, 5, 5, -1})

	// 反直觉：新来的更大值要立刻接管最大值，且旧最大值已不在单调队列里
	checkCheckout([]checkoutOp{
		{"add", 3}, {"get_max", 0}, {"add", 9}, {"get_max", 0},
		{"remove", 0}, {"get_max", 0}, {"remove", 0}, {"get_max", 0},
	}, []int{-2, 3, -2, 9, 3, 9, 9, -1})

	// 边界：单个元素反复 get_max，不应改变状态
	checkCheckout([]checkoutOp{
		{"add", 1}, {"get_max", 0}, {"get_max", 0}, {"remove", 0},
		{"get_max", 0},
	}, []int{-2, 1, 1, 1, -1})

	// 随机交叉验证
	crossCheckCheckout()

	fmt.Println("all tests passed")
}
