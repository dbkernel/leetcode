/*
============================================================================
1306. 跳跃游戏 III（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/jump-game-iii/
分类：图 · 可达性（隐式图 BFS / DFS）

这里有一个非负整数数组 arr，你最开始位于该数组的起始下标 start 处。
当你位于下标 i 处时，你可以跳到 i + arr[i] 或者 i - arr[i]。

请你判断自己是否能够跳到对应元素值为 0 的 任意 下标处。

注意，不管是什么情况下，你都无法跳到数组之外。

示例 1：
输入：arr = [4,2,3,0,3,1,2], start = 5
输出：true
解释：
到达值为 0 的下标 3 有以下可能方案：
下标 5 -> 下标 4 -> 下标 1 -> 下标 3

示例 2：
输入：arr = [4,2,3,0,3,1,2], start = 0
输出：true
解释：
到达值为 0 的下标 3 有以下可能方案：
下标 0 -> 下标 3

示例 3：
输入：arr = [3,0,2,1,2], start = 2
输出：false
解释：无法到达值为 0 的下标 1 处。

提示：
- 1 <= arr.length <= 5 * 10^4
- 0 <= arr[i] < arr.length
- 0 <= start < arr.length
*/
use std::collections::VecDeque;

/*
解法一：BFS 求最短路可达性【最优解】

为什么慢：想枚举所有跳法，每一步都有两个分叉（往左、往右），路径数按
2^k 增长，同一批下标会被不同路径反复访问；而“能否到达”只关心下标集合
是否扩张，与走了几步无关。

换个角度：把下标看成点，把 i -> i ± arr[i] 看成边，题目要求的正是
“start 所在的连通分量里有没有值为 0 的点”。图是隐式的（边由数组即时
算出），用 BFS 从 start 扩散一遍即可，每个点只入队一次。

关键点：
  · 访问后立刻标记：arr[i] == 0 时两个“邻居”都是 i 自己，不标记就
    会在原地死循环
  · 出队时先判 arr[cur] == 0 再扩展，这样起点自己就是 0 也能立刻返回
  · 两个方向都要做越界检查，而不是只检查一个方向
  · arr[i] 可能让 i ± arr[i] 正好跳出数组，越界的跳法直接丢弃

以示例 1（arr = [4,2,3,0,3,1,2]，start = 5）为例：
  5 出队，arr[5] = 1，邻居 4、6 入队
  4 出队，arr[4] = 3，邻居 1、7（7 越界）
  6 出队，arr[6] = 2，邻居 4（已访问）、8（越界）
  1 出队，arr[1] = 2，邻居 -1（越界）、3
  3 出队，arr[3] = 0 → 返回 true ✔

复杂度分析：
  时间 O(n)：每个下标最多入队一次，每次只算两个邻居
  空间 O(n)：访问数组与队列最坏各存 n 个下标

解法二：DFS 显式栈【最优解，与解法一并列】

为什么慢：递归版 DFS 也能 O(n)，但递归深度最坏等于 n（5 * 10^4），
斜着连成一条链时调用栈压力大；可达性判定不需要回溯，显式栈就够。

换个角度：BFS 与 DFS 对“连通分量”的覆盖范围完全一致，只是扩展顺序
不同。目标只是“分量里有没有 0”，所以谁先谁后不影响答案，只影响命中
的早晚。

关键点：
  · 压栈时标记，不要等出栈再标记：一个点可能被左右两侧同时指向
  · arr[cur] == 0 时立刻返回 true，不要把它再压回栈里
  · 栈里存的是下标，邻居在弹出时现算，不需要先建图
  · 起点本身就是 0 时，第一轮就命中

以示例 3（arr = [3,0,2,1,2]，start = 2）为例：
  2 出栈，arr[2] = 2，邻居 0、4 入栈
  4 出栈，arr[4] = 2，邻居 2（已访问）、6（越界）
  0 出栈，arr[0] = 3，邻居 3、-3（越界）
  3 出栈，arr[3] = 1，邻居 4（已访问）、2（已访问）
  栈空，从未遇到值为 0 的下标 → 返回 false ✔

复杂度分析：
  时间 O(n)：每个下标入栈一次，每条隐式边检查一次
  空间 O(n)：访问数组 O(n)，栈最坏 O(n)

两解法同为 O(n)，并列最优。取舍：BFS 天然按跳跃次数分层，若题目改成
“最少跳几次到 0”，只有 BFS 能直接给出答案；DFS 不需要队列、常数更小，
但拿不到步数信息。
*/

// rustc 单文件编译时平台上的 Solution 不存在，这里显式定义结构体
pub struct Solution1;

impl Solution1 {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut seen = vec![false; n];
        let mut queue = VecDeque::new();
        seen[start as usize] = true;
        queue.push_back(start as usize);

        while let Some(cur) = queue.pop_front() {
            // 落到值为 0 的下标即成功，起点自己为 0 也在这里命中
            if arr[cur] == 0 {
                return true;
            }
            let step = arr[cur] as i64;
            for nxt in [cur as i64 + step, cur as i64 - step] {
                // 越界的跳法不合法，直接丢弃
                if nxt < 0 || nxt >= n as i64 {
                    continue;
                }
                let nxt = nxt as usize;
                // 入队即标记：arr[cur] == 0 时邻居是自己，不标记会死循环
                if !seen[nxt] {
                    seen[nxt] = true;
                    queue.push_back(nxt);
                }
            }
        }
        false
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut seen = vec![false; n];
        seen[start as usize] = true;
        let mut stack = vec![start as usize];

        while let Some(cur) = stack.pop() {
            if arr[cur] == 0 {
                return true;
            }
            let step = arr[cur] as i64;
            for nxt in [cur as i64 + step, cur as i64 - step] {
                if nxt < 0 || nxt >= n as i64 {
                    continue;
                }
                let nxt = nxt as usize;
                // 压栈即标记：左右两条路汇到同一点时不会重复入栈
                if !seen[nxt] {
                    seen[nxt] = true;
                    stack.push(nxt);
                }
            }
        }
        false
    }
}

// ===== 自测辅助：两个解法必须给出同一答案 =====
fn both(arr: &[i32], start: i32) -> bool {
    let by_bfs = Solution1::can_reach(arr.to_vec(), start);
    let by_dfs = Solution2::can_reach(arr.to_vec(), start);
    assert_eq!(by_bfs, by_dfs, "arr={:?} start={}", arr, start);
    by_bfs
}

fn check(arr: &[i32], start: i32, want: bool) {
    assert_eq!(both(arr, start), want, "arr={:?} start={}", arr, start);
}

fn main() {
    // 示例 1：5 -> 4 -> 1 -> 3，能落到值为 0 的下标
    check(&[4, 2, 3, 0, 3, 1, 2], 5, true);

    // 示例 2：0 -> 3 一步到位
    check(&[4, 2, 3, 0, 3, 1, 2], 0, true);

    // 示例 3：能到达的下标都跳不到 1
    check(&[3, 0, 2, 1, 2], 2, false);

    // 边界：只有一个元素且值为 0，起点即答案
    check(&[0], 0, true);

    // 边界：只有一个元素，两个方向都跳出数组
    check(&[1], 0, false);

    // 反直觉：[1,1,1,1] 在 0、1 之间来回跳，没有 0 且不标记会死循环
    check(&[1, 1, 1, 1], 0, false);

    // 反直觉：步长恰好等于数组长度，两个方向都越界
    check(&[2, 0], 0, false);

    // 边界：只能向右一步命中
    check(&[1, 0], 0, true);

    // 边界：值为 0 的下标就在起点右侧，但必须先绕过左边的越界跳法
    check(&[2, 0, 0], 0, true);

    println!("all tests passed");
}
