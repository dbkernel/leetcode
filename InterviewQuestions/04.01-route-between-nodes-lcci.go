/*
============================================================================
面试题 04.01. 节点间通路（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/route-between-nodes-lcci/
分类：图 · 可达性（BFS / DFS）

节点间通路。给定有向图，设计一个算法，找出两个节点之间是否存在一条
路径。

示例 1:
输入：n = 3, graph = [[0, 1], [0, 2], [1, 2], [1, 2]], start = 0, target = 2
输出：true

示例 2:
输入：n = 5, graph = [[0, 1], [0, 2], [1, 2], [1, 2]], start = 0, target = 4
输出：false

提示：
- 节点数量 n 在 [0, 1e5] 范围内。
- 节点编号大于等于 0 小于 n。
- 图中可能有自环和重边。
*/
package main

import "fmt"

/*
解法一：BFS 逐层扩展【最优解】

为什么慢：想判断可达性，可以枚举长度为 1、2、3……的所有路径，逐层拼接
边表。但同一对节点会被不同路径重复走到，路径数随层数指数增长，代价远超
图本身的规模。

换个角度：只关心“能不能到”，不关心“怎么到”。把从 start 出发走 k 步能
到达的点看成一个集合，集合只会扩张不会收缩，且每个点最多进集合一次。
于是可以像水波扩散一样按层推进，每个点只处理一次。

关键点：
  · 先判 start == target：图里可能一条边都没有，但起点就是终点也算有
    通路（长度为 0 的路径）
  · 邻接表按 n 建，不要按边表里出现过的最大编号建：n 可能远大于所有
    边涉及的节点，按边的上界建会漏掉“空节点”
  · 入队时就标记已访问，不要等出队再标记：重边和自环会让同一节点反复
    入队，队列膨胀到 O(E)
  · 边是有向的，graph[i] = [from, to] 只能从 from 走到 to
  · Go 用切片加 head 下标当队列，避免 container/list 的接口开销

以示例 2（n = 5，start = 0，target = 4）为例：
  0 的邻居是 1、2，入队并标记
  依次弹出 1、2，它们的邻居仍是 1、2，都已在集合里，不再入队
  队列空，4 从未进过集合 → 返回 false ✔

复杂度分析：
  时间 O(V+E)：每个节点入队一次，每条边被检查一次（重边也只多走一次）
  空间 O(V+E)：邻接表 O(V+E)，访问标记与队列各 O(V)

解法二：DFS 显式栈【最优解，与解法一并列】

为什么慢：递归版 DFS 一样是 O(V+E)，但递归深度等于路径长度，n 可达
1e5，深链输入会把调用栈打爆；而可达性问题本身不需要“回溯”，用显式栈
就能表达“顺着一条路走到底，走不通再换下一条”。

换个角度：DFS 与 BFS 的差别只在“下一个扩展谁” —— 栈取栈顶（最近发现
的），队列取队头（最早发现的）。可达性只问存在性，两者答案必然一致。

关键点：
  · 压栈时就标记，出栈时不再判重：同一节点可能被多个前驱指向
  · 自环只会让 cur 再次指向自己，被标记挡住，不会死循环
  · 显式栈就是普通切片，n = 1e5 的深链也不会栈溢出
  · start == target 同样要在建图前先返回 true

以示例 1（n = 3，graph 含 0->1、0->2、1->2，start = 0，target = 2）为例：
  栈 [0]，弹出 0，压入 1、2 并标记
  弹出 2，等于 target → 返回 true ✔

复杂度分析：
  时间 O(V+E)：每个节点入栈一次，每条边检查一次
  空间 O(V+E)：邻接表 O(V+E)，标记数组与栈各 O(V)

两解法同为 O(V+E)，并列最优。取舍：BFS 先到先扩展，目标离起点近时
更快命中；DFS 的栈比 BFS 的队列更容易命中“一条长链直达”的情形，且
不必存整层。双向 BFS 是同一思路的另一写法：同时从 start 与 target
扩展、哪侧待扩展集合小就扩哪侧，最坏仍是 O(V+E)，但要额外维护反向
邻接表，空间常数更大，故不单独成解。
*/

// 解法一：BFS 逐层扩展
func findWhetherExistsPath(n int, graph [][]int, start int, target int) bool {
	// 长度为 0 的路径不需要任何边，空图也成立
	if start == target {
		return true
	}
	if n <= 0 {
		return false
	}

	// 按节点总数建表：n 可能大于边里出现过的最大编号
	next := make([][]int, n)
	for _, e := range graph {
		next[e[0]] = append(next[e[0]], e[1])
	}

	seen := make([]bool, n)
	queue := make([]int, 0, n)
	seen[start] = true
	queue = append(queue, start)

	for head := 0; head < len(queue); head++ {
		cur := queue[head]
		for _, nxt := range next[cur] {
			// 一旦邻接点就是目标，不必再入队
			if nxt == target {
				return true
			}
			// 入队即标记：重边与自环都不会让同一节点二次入队
			if !seen[nxt] {
				seen[nxt] = true
				queue = append(queue, nxt)
			}
		}
	}
	return false
}

// 解法二：DFS 显式栈
func findWhetherExistsPathDFS(n int, graph [][]int, start int,
	target int) bool {
	if start == target {
		return true
	}
	if n <= 0 {
		return false
	}

	next := make([][]int, n)
	for _, e := range graph {
		next[e[0]] = append(next[e[0]], e[1])
	}

	seen := make([]bool, n)
	stack := []int{start}
	seen[start] = true

	for len(stack) > 0 {
		cur := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if cur == target {
			return true
		}
		for _, nxt := range next[cur] {
			// 压栈时标记：多个前驱指向同一节点时不会重复入栈
			if !seen[nxt] {
				seen[nxt] = true
				stack = append(stack, nxt)
			}
		}
	}
	return false
}

// ===== 自测辅助：两个解法必须给出同一答案 =====
func both(n int, graph [][]int, start int, target int) bool {
	byBfs := findWhetherExistsPath(n, graph, start, target)
	byDfs := findWhetherExistsPathDFS(n, graph, start, target)
	if byBfs != byDfs {
		panic(fmt.Sprintf("%d->%d: bfs=%v, dfs=%v", start, target, byBfs, byDfs))
	}
	return byBfs
}

func check(n int, graph [][]int, start int, target int, want bool) {
	if got := both(n, graph, start, target); got != want {
		panic(fmt.Sprintf("n=%d, %d->%d: got %v, want %v",
			n, start, target, got, want))
	}
}

func main() {
	// 示例 1：0 -> 2 有直达边，同时 1 -> 2 是重边
	check(3, [][]int{{0, 1}, {0, 2}, {1, 2}, {1, 2}}, 0, 2, true)

	// 示例 2：4 从未在边里出现，不可达
	check(5, [][]int{{0, 1}, {0, 2}, {1, 2}, {1, 2}}, 0, 4, false)

	// 反直觉：一条边都没有，但起点等于终点仍是通路
	check(5, [][]int{}, 4, 4, true)

	// 边界：空边表且起终点不同，必然不可达
	check(5, [][]int{}, 0, 4, false)

	// 边界：n = 1，节点 0 只能到达自己
	check(1, [][]int{}, 0, 0, true)

	// 关键点：边有向，0 -> 1 不代表 1 -> 0
	check(2, [][]int{{0, 1}}, 1, 0, false)

	// 反直觉：自环与重边都不能让搜索死循环
	check(2, [][]int{{0, 0}, {0, 1}, {0, 1}}, 0, 1, true)

	// 关键点：n 远大于边涉及的最大编号，邻接表要按 n 开
	check(1000, [][]int{{0, 1}, {1, 2}}, 0, 999, false)

	// 边界：长链要一路走到底
	check(5, [][]int{{0, 1}, {1, 2}, {2, 3}, {3, 4}}, 0, 4, true)

	fmt.Println("all tests passed")
}
