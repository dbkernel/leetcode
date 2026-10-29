/*
============================================================================
面试题 16.15. 珠玑妙算（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/master-mind-lcci/
分类：哈希表 · 计数配对

珠玑妙算游戏（the game of master mind）的玩法如下。

计算机有4个槽，每个槽放一个球，颜色可能是红色（R）、黄色（Y）、绿色（G）
或蓝色（B）。例如，计算机可能有RGGB 4种（槽1为红色，槽2、3为绿色，槽4
为蓝色）。作为用户，你试图猜出颜色组合。打个比方，你可能会猜YRGB。要是
猜对某个槽的颜色，则算一次“猜中”；要是只猜对颜色但槽位猜错了，则算一次
“伪猜中”。注意，“猜中”不能算入“伪猜中”。

给定一种颜色组合solution和一个猜测guess，编写一个方法，返回猜中和伪猜中
的次数answer，其中answer[0]为猜中的次数，answer[1]为伪猜中的次数。

示例：
输入： solution="RGBY",guess="GGRR"
输出： [1,1]
解释： 猜中1次，伪猜中1次。

提示：
- len(solution) = len(guess) = 4
- solution和guess仅包含"R", "G", "B", "Y"这4种字符
*/
package main

import "fmt"

/*
力扣官方未发布本题题解，下面两种做法都是 O(n) 时间、O(1) 空间，
并列最优。

解法一：先扣除猜中，再按颜色统计剩余频次【最优解】

为什么慢：如果对每个 guess 位置去 solution 里找一个同色且未使用的位置，
单次要 O(n)，总共 O(n^2)；而且"已经配过对"的状态要靠额外的 visited 数组
维护，稍不留神就会把同一个位置重复计数。

换个角度：猜中的位置先单独扣掉，剩下的字符只关心"每种颜色各剩多少个"。
伪猜中的数量就是每种颜色在两边剩余数量中的较小值 —— 一边多出来的部分
没有对手，配不成对。

关键点：
  · 先统计猜中，并把猜中的位置从两边频次中排除，否则会重复计数
  · 伪猜中只在非猜中的位置上统计
  · 颜色种类固定为 4 种，频次数组是常数空间
  · 每个颜色的贡献是 min(solution 剩余数, guess 剩余数)，不能直接相加

以示例（solution = "RGBY"，guess = "GGRR"）为例：
  位置 1 的 G 猜中，answer[0] = 1
  剩余 solution：R、B、Y；剩余 guess：G、R、R
  只有 R 两边都有，取 min(1,2) = 1，answer[1] = 1 ✔

复杂度分析：
  时间 O(n)：两遍线性扫描，n 为字符串长度
  空间 O(1)：4 种颜色对应常数大小的频次数组
*/

// masterMind 是平台入口，采用两遍计数的写法
func masterMind(solution string, guess string) []int {
	exact := 0
	var cntSolution [26]int
	var cntGuess [26]int

	for i := 0; i < len(solution); i++ {
		if solution[i] == guess[i] {
			exact++
		} else {
			// 猜中的位置不进入频次，避免被二次统计成伪猜中
			cntSolution[solution[i]-'A']++
			cntGuess[guess[i]-'A']++
		}
	}

	pseudo := 0
	for c := 0; c < 26; c++ {
		if cntSolution[c] < cntGuess[c] {
			pseudo += cntSolution[c]
		} else {
			pseudo += cntGuess[c]
		}
	}
	return []int{exact, pseudo}
}

/*
解法二：一次遍历，用有符号差值边扫边配对【最优解，与解法一并列】
【最优解，与解法一并列】

为什么慢：解法一要先把两边剩余频次都统计完，再回头求 min，需要两遍。
可配对这件事在扫描时就能确定：当前颜色在对面是否还有"欠账"，有就立即
配掉。

换个角度：维护 balance[c] = 未配对的 solution 颜色 c 数量 减去 未配对的
guess 颜色 c 数量。遇到非猜中的位置时：
  · 若 balance[solution[i]] < 0，说明之前有 guess 的颜色在等它 → 伪猜中
  · 若 balance[guess[i]] > 0，说明之前有 solution 的颜色在等它 → 伪猜中

关键点：
  · 猜中的位置直接跳过，不进入 balance，天然避免重复计数
  · 先判断再修改 balance，顺序反了会把自己刚加进去的数量配给自己
  · 两个颜色不同（否则已经在猜中分支），所以两次判断互不干扰
  · 颜色用 26 个字母的下标，空间仍是常数

以示例（solution = "RGBY"，guess = "GGRR"）为例：
  位置 0：R 与 G，balance[R] 0→1，balance[G] 0→-1
  位置 1：G 与 G，猜中，exact = 1
  位置 2：B 与 R，balance[R] = 1 > 0 → 伪猜中 +1，balance[R] → 0
  位置 3：Y 与 R，balance[Y] → 1，balance[R] → -1
  结果 [1,1] ✔

复杂度分析：
  时间 O(n)：只扫描一遍
  空间 O(1)：26 个字母的差值数组
*/
func masterMindOnePass(solution string, guess string) []int {
	exact := 0
	pseudo := 0
	var balance [26]int

	for i := 0; i < len(solution); i++ {
		s := solution[i] - 'A'
		g := guess[i] - 'A'
		if s == g {
			exact++
			continue
		}

		// 先看欠账再记账，否则会把自己刚提供的数量配掉
		if balance[s] < 0 {
			pseudo++
		}
		balance[s]++
		if balance[g] > 0 {
			pseudo++
		}
		balance[g]--
	}
	return []int{exact, pseudo}
}

// ---- 以下为自测代码，不属于题解主体 ----

// 与计数解独立的参照实现：先标记猜中，再对剩余位置逐个配对
func bruteMasterMind(solution, guess string) []int {
	n := len(solution)
	usedSolution := make([]bool, n)
	usedGuess := make([]bool, n)
	exact := 0

	for i := 0; i < n; i++ {
		if solution[i] == guess[i] {
			exact++
			usedSolution[i] = true
			usedGuess[i] = true
		}
	}

	pseudo := 0
	for i := 0; i < n; i++ {
		if usedGuess[i] {
			continue
		}
		for j := 0; j < n; j++ {
			if usedSolution[j] {
				continue
			}
			if guess[i] == solution[j] {
				usedSolution[j] = true
				usedGuess[i] = true
				pseudo++
				break
			}
		}
	}
	return []int{exact, pseudo}
}

func checkMasterMind(solution, guess string, want [2]int) {
	cases := []struct {
		name string
		fn   func(string, string) []int
	}{
		{"count", masterMind},
		{"onePass", masterMindOnePass},
	}
	for _, c := range cases {
		got := c.fn(solution, guess)
		if got[0] != want[0] || got[1] != want[1] {
			panic(fmt.Sprintf("%s(%q,%q) = %v, want %v",
				c.name, solution, guess, got, want))
		}
	}

	brute := bruteMasterMind(solution, guess)
	if brute[0] != want[0] || brute[1] != want[1] {
		panic(fmt.Sprintf("brute(%q,%q) = %v, want %v",
			solution, guess, brute, want))
	}
}

func allColorCodes() []string {
	const colors = "RGBY"
	out := make([]string, 0, 256)
	for code := 0; code < 256; code++ {
		buf := make([]byte, 0, 4)
		x := code
		for i := 0; i < 4; i++ {
			buf = append(buf, colors[x%4])
			x /= 4
		}
		out = append(out, string(buf))
	}
	return out
}

func main() {
	// 示例：一个猜中，一个伪猜中
	checkMasterMind("RGBY", "GGRR", [2]int{1, 1})

	// 全部猜中
	checkMasterMind("RGBY", "RGBY", [2]int{4, 0})

	// 颜色全对、位置全错：四种颜色都只算伪猜中
	checkMasterMind("RGBY", "YRGB", [2]int{0, 4})
	checkMasterMind("RGBY", "BYRG", [2]int{0, 4})

	// 重复颜色只按较少的一边配对：Y 猜了三次只算一次
	checkMasterMind("RGBY", "YYYR", [2]int{0, 2})

	// 四个位置都是同一颜色，猜中四个
	checkMasterMind("RRRR", "RRRR", [2]int{4, 0})

	// 颜色完全不同
	checkMasterMind("RRRR", "YYYY", [2]int{0, 0})

	// 一真三假：剩余猜测颜色与 solution 剩余颜色对不上
	checkMasterMind("RRRR", "RYYY", [2]int{1, 0})

	// 重复颜色：两边各两个 R、两个 G，位置全错
	checkMasterMind("RRGG", "GGRR", [2]int{0, 4})

	// 两真两伪
	checkMasterMind("RGBY", "BGRY", [2]int{2, 2})
	checkMasterMind("RGBY", "GRBY", [2]int{2, 2})

	// 三真，剩余颜色不匹配
	checkMasterMind("RGBY", "RGBB", [2]int{3, 0})

	// 两真一伪，剩余颜色有一个能配上
	checkMasterMind("RGBY", "RGGB", [2]int{2, 1})

	// 反直觉：猜中位置不能再次贡献伪猜中
	checkMasterMind("RRRR", "RRRY", [2]int{3, 0})

	// 穷举 4^4 × 4^4 组 solution/guess，与逐位配对的参照实现比对
	codes := allColorCodes()
	for _, solution := range codes {
		for _, guess := range codes {
			want := bruteMasterMind(solution, guess)
			got1 := masterMind(solution, guess)
			got2 := masterMindOnePass(solution, guess)
			if got1[0] != want[0] || got1[1] != want[1] ||
				got2[0] != want[0] || got2[1] != want[1] {
				panic(fmt.Sprintf("exhaustive mismatch at %q,%q",
					solution, guess))
			}
		}
	}

	fmt.Println("all tests passed")
}
