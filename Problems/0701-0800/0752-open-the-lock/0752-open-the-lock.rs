/*
============================================================================
752. 打开转盘锁（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/open-the-lock/
分类：图 · 隐式图最短路（BFS / 双向 BFS）

你有一个带有四个圆形拨轮的转盘锁。每个拨轮都有 10 个数字：
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9' 。每个拨轮可以自由
旋转：例如把 '9' 变为 '0'，'0' 变为 '9' 。每次旋转都只能旋转一个拨轮
的一位数字。

锁的初始数字为 '0000' ，一个代表四个拨轮的数字的字符串。

列表 deadends 包含了一组死亡数字，一旦拨轮的数字和列表里的任何一个
元素相同，这个锁将会被永久锁定，无法再旋转。

字符串 target 代表可以解锁的数字，你需要给出解锁需要的最小旋转次数，
如果无论如何不能解锁，返回 -1 。

示例 1:
输入: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
输出：6
解释：
可能的移动序列为 "0000" -> "1000" -> "1100" -> "1200" -> "1201" ->
"1202" -> "0202"。
注意 "0000" -> "0001" -> "0002" -> "0102" -> "0202" 这样的序列是
不能解锁的，因为当拨动到 "0102" 时这个锁就会被锁定。

示例 2:
输入: deadends = ["8888"], target = "0009"
输出：1
解释：
把最后一位反向旋转一次即可 "0000" -> "0009"。

示例 3:
输入: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"],
target = "8888"
输出：-1
解释：
无法旋转到目标数字且不被锁定。

提示：
- 1 <= deadends.length <= 500
- deadends[i].length == 4
- target.length == 4
- target 不在 deadends 之中
- target != "0000"
- deadends[i] 仅由若干位数字组成
*/
use std::collections::{HashSet, VecDeque};

/*
解法一：BFS 逐层扩展【最优解】

为什么慢：从 "0000" 出发每步有 8 种拨法（4 个拨轮各 ±1），想找最少步数
就得枚举长度为 1、2、3……的所有序列。暴力枚举的序列数按 8^k 增长，而
不同序列会落到同一个数字上，重复计算几乎无法避免。

换个角度：把每个四位数字串看成一个节点，两串只差一位（且按模 10 相邻）
就有一条边，问题变成“从 "0000" 到 target 的最短路边数”。无权图最短路
就是 BFS，第一次碰到目标时的层号就是最小旋转次数。

关键点：
  · 先把 "0000" 在 deadends 里的情况挡掉：起点本身被锁死，一步都走不
    了，直接返回 -1
  · 邻居的生成用 (d + 1) % 10 与 (d + 9) % 10，'9' 加一变 '0'、'0'
    减一变 '9'，不能写成 d - 1 后不管负数
  · 必须按层计数：一次只把当前层的全部节点扩展完，steps 才等于层号
  · 入队时立刻写进已访问集合，否则同一数字会被多条路径反复入队
  · 命中目标要在入队时判断，别等出队，否则会多算一层
  · Rust 里用 HashSet<String> 当集合，front 与 back 靠 mem::swap 互换

以示例 2（deadends = ["8888"]，target = "0009"）为例：
  第 1 层从 "0000" 展开 8 个邻居，其中 "0009" 就是目标
  入队时命中，返回 steps = 1 ✔

复杂度分析：
  时间 O(10^4 * 4 * 2)：状态数封顶 10^4，每个状态生成 8 个邻居
  空间 O(10^4)：已访问集合与队列最多各存全部状态

解法二：双向 BFS【最优解，与解法一并列】

为什么慢：单向 BFS 从起点向外扩散，搜索半径等于答案本身。当答案需要
十几步时，扩散到第 k 层就要访问约 8^k 个状态，虽然状态总数封顶 10^4，
但实际展开量仍然可观。

换个角度：既然已知起点和终点，就从两端同时扩散。两侧各走 k 步能覆盖
半径 k 的球，相遇时总路径约 2k 步 —— 同样的步数覆盖更多状态，等价于
把需要展开的状态数从 b^d 压到 b^(d/2)。

关键点：
  · 每次扩展当前较小的一侧，两侧规模接近时相遇最快
  · steps 记录“两侧已扩展的层数之和”，命中时答案是 steps + 1
  · 相遇判断要在“已访问”判断之前：目标侧的点也在 seen 里
  · "0000" 在 deadends 里仍然要第一时间返回 -1，双向也救不了锁死的起点
  · 状态总数固定为 10^4，所以两者最坏都是 O(10^4)；双向只是把实际
    展开的状态数压到约平方根量级，渐近上界相同，故并列最优

以示例 1（target = "0202"，答案 6）为例：
  扩展 "0000" 一层后换成扩展目标侧，两侧交替、每轮扩展较小集合
  第 6 步时某侧新邻居落在另一侧集合里，返回 6 ✔

复杂度分析：
  时间 O(10^4 * 4 * 2)：最坏仍要处理全部状态，只是通常远小于单向
  空间 O(10^4)：两个集合加 seen 都是状态数级别
*/

// turn 把第 pos 位循环加 delta（+1 或 +9），其余位不动
fn turn(s: &str, pos: usize, delta: i32) -> String {
    let mut bytes = s.as_bytes().to_vec();
    let digit = (i32::from(bytes[pos]) - i32::from(b'0') + delta) % 10;
    bytes[pos] = b'0' + digit as u8;
    String::from_utf8(bytes).expect("只有数字字符，转换不会失败")
}

// rustc 单文件编译时平台上的 Solution 不存在，这里显式定义结构体
pub struct Solution1;

impl Solution1 {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let dead: HashSet<String> = deadends.into_iter().collect();
        // 起点被锁死时一步都走不了
        if dead.contains("0000") {
            return -1;
        }
        // 题面保证 target != "0000"，这里保留是为了接口自洽
        if target == "0000" {
            return 0;
        }

        let mut seen: HashSet<String> = HashSet::new();
        seen.insert("0000".to_string());
        let mut queue = VecDeque::new();
        queue.push_back("0000".to_string());
        let mut steps = 0;

        while !queue.is_empty() {
            // 先固定本层边界，循环里追加的下一层节点不会被提前处理
            let level_size = queue.len();
            steps += 1;
            for _ in 0..level_size {
                let cur = queue.pop_front().unwrap();
                for pos in 0..4 {
                    for &delta in &[1, 9] {
                        let nxt = turn(&cur, pos, delta);
                        if dead.contains(&nxt) || seen.contains(&nxt) {
                            continue;
                        }
                        // 入队时命中，层号才是最小步数
                        if nxt == target {
                            return steps;
                        }
                        seen.insert(nxt.clone());
                        queue.push_back(nxt);
                    }
                }
            }
        }
        -1
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let dead: HashSet<String> = deadends.into_iter().collect();
        if dead.contains("0000") {
            return -1;
        }
        if target == "0000" {
            return 0;
        }

        let mut front: HashSet<String> = HashSet::new();
        front.insert("0000".to_string());
        let mut back: HashSet<String> = HashSet::new();
        back.insert(target.clone());
        let mut seen: HashSet<String> = HashSet::new();
        seen.insert("0000".to_string());
        seen.insert(target);
        let mut steps = 0;

        while !front.is_empty() && !back.is_empty() {
            // 总扩展较小的一侧：两侧规模接近时相遇最早
            if front.len() > back.len() {
                std::mem::swap(&mut front, &mut back);
            }

            let mut next: HashSet<String> = HashSet::new();
            for cur in &front {
                for pos in 0..4 {
                    for &delta in &[1, 9] {
                        let nxt = turn(cur, pos, delta);
                        if dead.contains(&nxt) {
                            continue;
                        }
                        // 先判相遇：另一侧的点也在 seen 里，先查 seen 会漏
                        if back.contains(&nxt) {
                            return steps + 1;
                        }
                        if seen.contains(&nxt) {
                            continue;
                        }
                        seen.insert(nxt.clone());
                        next.insert(nxt);
                    }
                }
            }
            steps += 1;
            front = next;
        }
        -1
    }
}

// ===== 自测辅助：两个解法必须给出同一答案 =====
fn both(deadends: &[&str], target: &str) -> i32 {
    let dead: Vec<String> = deadends.iter().map(|s| s.to_string()).collect();
    let by_bfs = Solution1::open_lock(dead.clone(), target.to_string());
    let by_bi = Solution2::open_lock(dead, target.to_string());
    assert_eq!(by_bfs, by_bi, "target={} 两解法不一致", target);
    by_bfs
}

fn check(deadends: &[&str], target: &str, want: i32) {
    assert_eq!(both(deadends, target), want, "target={}", target);
}

fn main() {
    // 示例 1：只能绕开 "0102" 等死锁，答案 6
    check(&["0201", "0101", "0102", "1212", "2002"], "0202", 6);

    // 示例 2：最后一位反向转一次
    check(&["8888"], "0009", 1);

    // 示例 3：目标被 8 个死锁完全包住
    check(
        &[
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ],
        "8888",
        -1,
    );

    // 反直觉：起点 "0000" 自己在 deadends 里，一步都走不了
    check(&["0000"], "0001", -1);

    // 边界：没有任何死锁，target 需要两位各转一次
    check(&[], "0002", 2);

    // 边界：四个拨轮各转一次
    check(&[], "1111", 4);

    // 反直觉：8 个邻居全被死锁占住，起点被围死
    check(
        &[
            "0001", "0009", "0010", "0090", "0100", "0900", "1000", "9000",
        ],
        "1111",
        -1,
    );

    // 边界：题目保证 target != "0000"，这里只验证代码不崩
    check(&[], "0000", 0);

    println!("all tests passed");
}
