/*
============================================================================
45. 跳跃游戏 II（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/jump-game-ii/
分类：贪心 · 区间分层

给定一个长度为 n 的 0 索引整数数组 nums。初始位置在下标 0。

每个元素 nums[i] 表示从索引 i 向后跳转的最大长度。换句话说，如果你在
索引 i 处，你可以跳转到任意 (i + j) 处：

    0 <= j <= nums[i] 且
    i + j < n

返回到达 n - 1 的最小跳跃次数。测试用例保证可以到达 n - 1。

示例 1：
输入: nums = [2,3,1,1,4]
输出: 2
解释: 跳到最后一个位置的最小跳跃数是 2。
     从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后
     一个位置。

示例 2：
输入: nums = [2,3,0,1,4]
输出: 2

提示：
1 <= nums.length <= 10^4
0 <= nums[i] <= 1000
题目保证可以到达 n - 1
*/
package main

import "fmt"

/*
解法一：反向查找出发位置（每轮选最靠左的可行前驱）【非最优】

为什么慢：每确定一个前驱都要从左到右重新扫一遍 [0, position)，
最坏（每次只前进一格）要扫 O(n^2) 次。瓶颈是每一层都重新找「最靠左」。

换个角度：与其从起点往后猜该跳多远，不如从终点往回推：最后一步之前
一定站在某个 i，满足 i + nums[i] >= n-1。这样的 i 可能有很多个，
选最靠左的那个，剩下的路最短。

关键点：
  · 每轮从 i = 0 开始找第一个满足 i + nums[i] >= position 的下标
  · 找到后把 position 收缩到 i，并让 steps + 1，再继续往左推
  · 必须从 0 开始找（而不是从 position-1 往左找），才能取到最小下标
  · 题目保证可达，所以内层循环一定能命中，不必处理找不到的情况

为什么贪心是对的（反向支配）：
  · 设当前要到达 position，i* 是满足 i + nums[i] >= position 的最小下标，
    i' 是任意另一个可行的前驱（i* < i' < position）。
  · i* 能一步跨到 position，自然也能一步跨到 i'（跳短不违反限制）。
    于是「先到 i*，再按 i' 之后的方案走」是一条合法路线，跳数不增加。
  · 所以把上一跳的起点从 i' 换成 i* 不会让答案变差；反复替换，
    必存在一个最优方案每一层都从最靠左的可行前驱出发。
  · 每轮 steps + 1 也是必要的：position 还没有到 0，至少要再退一跳。

以示例 1（nums = [2,3,1,1,4]）为例：
  position = 4：i=1 时 1+3 >= 4，第一个命中 → position=1，steps=1
  position = 1：i=0 时 0+2 >= 1 → position=0，steps=2
  position = 0，停止，返回 2 ✔

复杂度分析：
  时间 O(n^2)：每轮线性扫描，最多 n 轮
  空间 O(1)：只用了 position 与 steps
*/
func jumpReverse(nums []int) int {
	position := len(nums) - 1
	steps := 0

	// position 每轮严格变小，最多 n 轮
	for position > 0 {
		// 从左往右找第一个能一步跨到 position 的下标
		for i := 0; i < position; i++ {
			if i+nums[i] >= position {
				position = i
				steps++
				break
			}
		}
	}
	return steps
}

/*
解法二：贪心 + 分层（维护本层右边界与下一层右边界）【最优解】

为什么慢：枚举所有跳跃方案是指数级；退一步用 dp[i] 表示到达 i 的最少跳数，
需要枚举每个 i 之前所有能跳到 i 的位置，时间 O(n^2)。瓶颈是「每个位置都
重新回头看一遍」。

换个角度：把「跳 k 次能到哪」看成一圈一圈向外扩的层。因为从 i 能跳到
i+1..i+nums[i] 中的任意下标，「能到达」的集合天然是一个前缀，所以第 k 层
只需要用一个右边界 R_k 就能描述，不必记录层内每个位置。

关键点：
  · curEnd 是第 k 层的右边界，farthest 是扫描本层时能扩到的下一层右边界
  · 先更新 farthest，再判断 i == curEnd：顺序反了会把当前元素算进上一层
  · 跳数在「跨层」时才 +1，不是每访问一个下标就 +1
  · 答案在 curEnd >= n - 1 时即可停止，最后一跳落在终点右侧也算到达
  · n == 1 直接返回 0，否则循环条件 i < n - 1 永远不触发跨层

为什么贪心是对的（分层可达性，不是「每步跳最远」）：
  · 记 R_k 为跳 k 次能到达的最远下标。可达集合是前缀 [0, R_k]：
    若 j 可达，那么任何 j' < j 也可达 —— 把最后一跳跳短即可。
  · 因此 R_{k+1} = max_{i <= R_k} (i + nums[i])，只依赖整个第 k 层前缀，
    与「落在层内哪一个位置」无关。层内所有位置给出的下一层上界是同一个值，
    所以不存在「这一步跳近了会吃亏」的情况。
  · 代码扫到 curEnd 时，[0, curEnd] 已全部看过，farthest 恰好等于 R_{k+1}；
    每跨一层跳数 +1，层数就是最小跳数。
  · 反证：若贪心跳了 m 次到达终点，而最优只要 m-1 次，则 R_{m-1} >= n-1；
    但贪心第 m-1 层的右边界正是 R_{m-1}，它会在该层结束时停止，矛盾。

以示例 1（nums = [2,3,1,1,4]）为例：
  初始 curEnd = 0，farthest = 0
  i = 0：farthest = max(0, 0+2) = 2；i == curEnd → 跳数 1，curEnd = 2
  i = 1：farthest = max(2, 1+3) = 4
  i = 2：farthest = max(4, 2+1) = 4；i == curEnd → 跳数 2，curEnd = 4
  curEnd = 4 >= n-1 = 4，停止，返回 2 ✔

复杂度分析：
  时间 O(n)：每个下标只被访问一次，farthest 只做常数次比较
  空间 O(1)：只用了 jumps / curEnd / farthest 三个变量
*/
func jump(nums []int) int {
	n := len(nums)
	// 起点就是终点，一次都不用跳
	if n == 1 {
		return 0
	}

	jumps := 0
	curEnd := 0   // 跳 jumps 次能到达的最远下标
	farthest := 0 // 再跳一次能到达的最远下标

	// 最后一个下标不需要再作为起跳点，i < n-1 已经足够
	for i := 0; i < n-1; i++ {
		// 先扩张下一层：本层内每个位置都要参与取 max
		if i+nums[i] > farthest {
			farthest = i + nums[i]
		}

		// 本层扫描完毕，必须再跳一次，落点就是 farthest
		if i == curEnd {
			jumps++
			curEnd = farthest
			// 终点已在可达范围内，后面的元素不必再扫
			if curEnd >= n-1 {
				break
			}
		}
	}
	return jumps
}

// 传值接收（切片本身按引用传递，这里只读不写，不会影响调用方）
func check(nums []int, want int) {
	if got := jumpReverse(nums); got != want {
		panic(fmt.Sprintf("reverse(%v) = %d, want %d", nums, got, want))
	}
	if got := jump(nums); got != want {
		panic(fmt.Sprintf("greedy(%v) = %d, want %d", nums, got, want))
	}
}

func main() {
	// 示例 1：第一跳落在 1 而不是最远的 2，说明分层不等于每步跳最远
	check([]int{2, 3, 1, 1, 4}, 2)

	// 示例 2：下标 2 的 0 不能起跳，本层右边界仍要覆盖到它
	check([]int{2, 3, 0, 1, 4}, 2)

	// 边界：单元素，起点即终点
	check([]int{0}, 0)

	// 边界：一步直接到终点
	check([]int{5, 1, 1, 1}, 1)

	// 边界：每步只能走一格，跳数等于 n-1
	check([]int{1, 1, 1, 1, 1}, 4)

	// 反直觉：中间连续 0，只要上一层能越过它们就不影响跳数
	check([]int{3, 0, 0, 1, 0}, 2)

	fmt.Println("all tests passed")
}
