/*
============================================================================
37. 解数独（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sudoku-solver/
分类：回溯 · 位运算掩码与 MRV 剪枝

编写一个程序，通过填充空格来解决数独问题。

数独的解法需 遵循如下规则：

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）

数独部分空格内已填入了数字，空白格用 '.' 表示。

示例 1：
输入：board = [["5","3",".",".","7",".",".",".","."],
["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],
["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],
["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],
[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
输出：[["5","3","4","6","7","8","9","1","2"],
["6","7","2","1","9","5","3","4","8"],
["1","9","8","3","4","2","5","6","7"],
["8","5","9","7","6","1","4","2","3"],
["4","2","6","8","5","3","7","9","1"],
["7","1","3","9","2","4","8","5","6"],
["9","6","1","5","3","7","2","8","4"],
["2","8","7","4","1","9","6","3","5"],
["3","4","5","2","8","6","1","7","9"]]
解释：输入的数独如上图所示，唯一有效的解决方案如下所示：

提示：
- board.length == 9
- board[i].length == 9
- board[i][j] 是一位数字或者 '.'
- 题目数据 保证 输入数独仅有一个解
*/

// 力扣提交时平台已提供 `impl Solution`，这里显式声明是为了单文件 rustc 编译。
pub struct Solution1;
pub struct Solution2;

/*
解法一：回溯 + 行/列/宫三个位掩码【最优解】

为什么慢：最朴素的写法是给每个空格依次试 1-9，每试一个都扫一遍所在行、列、
宫判断冲突，单次判断 O(27)；递归时又从头扫棋盘找下一个空格，每层还要再花
O(81)。节点数一样，但每个节点都带着常数很大的重复扫描。

换个角度：一行、一列、一个宫各自只需要 9 个比特就能表示"哪些数字已用"。
把冲突判断压成三次按位或，一次位运算就得到当前格的全部候选；再让 pos 单调
向前，每层从上次的位置继续，空格只扫描一遍而不是每层重扫。

关键点：
  · 数字 d（0-8）对应 bit `1 << d`，三个掩码的并集就是"已占用"
  · 候选集 = `!(行 | 列 | 宫) & 0x1FF`，0x1FF 把无用的高位截掉
  · 落子与撤销必须成对：置 rows/cols/boxes 的同时改 board，
    回溯时两者一起还原，否则后续分支会读到脏掩码
  · 找到一个解立即逐层返回 true，不能继续搜索（题目保证唯一解，多搜只会变慢）
  · 空格用 pos 一维编号，跳过题面已给的数字，回溯时传 pos + 1 而不是 0
  · Rust 无符号数取最低位用 `x & x.wrapping_neg()`，再由
    `trailing_zeros` 直接得到数字下标

以示例 1 的第一个空格 (0, 2) 为例：
  第 0 行已用 {5,3,7}，第 2 列已用 {8}，左上宫已用 {5,3,6,9,8}
  并集 {3,5,6,7,8,9}，候选 {1,2,4}，先试 4，后续一路递归即可填满 ✔

复杂度分析：
  时间 O(9^(空格数))：最坏要枚举每个空格的候选，实际被掩码剪到很小；
    每个节点只做 O(1) 位运算，远小于每步 O(27) 扫描的写法
  空间 O(1)：三个掩码各 9 个 u16，递归深度不超过 81，均与 n 无关
*/
impl Solution1 {
    // 契约：原地修改 board，返回时棋盘已填满且满足数独规则
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        // 先把题面给定的数字登记进掩码，后续只处理空格
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    let d = board[r][c] as u32 - '1' as u32;
                    flip(&mut rows, &mut cols, &mut boxes, r, c, d, true);
                }
            }
        }

        dfs_mask(board, &mut rows, &mut cols, &mut boxes, 0);
    }
}

fn flip(
    rows: &mut [u16],
    cols: &mut [u16],
    boxes: &mut [u16],
    r: usize,
    c: usize,
    d: u32,
    on: bool,
) {
    let bit = 1u16 << d;
    let b = (r / 3) * 3 + c / 3;
    // 置位与清位成对出现，撤销时不会误伤同一宫里的其它数字
    if on {
        rows[r] |= bit;
        cols[c] |= bit;
        boxes[b] |= bit;
    } else {
        rows[r] &= !bit;
        cols[c] &= !bit;
        boxes[b] &= !bit;
    }
}

fn dfs_mask(
    board: &mut [Vec<char>],
    rows: &mut [u16],
    cols: &mut [u16],
    boxes: &mut [u16],
    mut pos: usize,
) -> bool {
    // pos 一直向前，题面给定的数字直接跳过，不重复扫描
    while pos < 81 && board[pos / 9][pos % 9] != '.' {
        pos += 1;
    }
    if pos == 81 {
        return true;
    }

    let (r, c) = (pos / 9, pos % 9);
    let b = (r / 3) * 3 + c / 3;
    // 三个掩码取并就是被禁用的数字，取反后与 0x1FF 得到候选位
    let mut avail = !(rows[r] | cols[c] | boxes[b]) & 0x1FF;

    while avail != 0 {
        let bit = avail & avail.wrapping_neg();
        avail ^= bit;
        let d = bit.trailing_zeros();

        flip(rows, cols, boxes, r, c, d, true);
        board[r][c] = char::from_digit(d + 1, 10).unwrap();

        if dfs_mask(board, rows, cols, boxes, pos + 1) {
            return true;
        }

        // 这一支失败，格子必须恢复成空格，掩码同步撤销
        board[r][c] = '.';
        flip(rows, cols, boxes, r, c, d, false);
    }
    false
}

/*
解法二：回溯 + 每层挑候选最少的空格（MRV）【最优解，与解法一并列】

动机：解法一按固定顺序填空格，遇到"候选很多但后面会死"的格子时，
前面几个分支都是白试。数独里候选最少的空格往往就是最容易证伪的位置，
先把它们填掉能更早剪枝。

换个角度：每层不再取"下一个空格"，而是扫描全盘、算每个空格的合法候选数，
挑候选数最小的那个来展开（Minimum Remaining Values 启发式）。

关键点：
  · 扫描要算满 9 个候选才能比较，单层代价 O(81 × 27)，比解法一的 O(1) 高得多
  · 候选数为 0 要立刻返回 false，这是当前分支已死的直接证据
  · 找不到空格（best_r 为 None）说明棋盘已填满，返回 true
  · 递归返回 false 后要把 (best_r, best_c) 恢复成 '.'，否则上一层会读到残留数字

复杂度分析：
  时间 O(9^(空格数))：与解法一同一量级的上界，但每层多了 O(81 × 27) 的
    扫描成本；启发式只缩小常数级的搜索树，没有改变指数上界
  空间 O(1)：只用几个下标变量，递归深度不超过 81

  取舍：两解的最坏情况上界相同（都是指数级），额外空间都是 O(1)，
  按渐近复杂度属于并列最优。解法一单节点只做 O(1) 位运算，常数更小；
  解法二单节点多出 O(81 × 27) 扫描，但优先展开候选最少的格子，
  在"线索极少"的盘面上搜索树小得多。顺序位掩码胜在常数，
  MRV 胜在实践剪枝，按盘面形态各有所长。
*/
impl Solution2 {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        dfs_mrv(board);
    }
}

fn can_place(board: &[Vec<char>], r: usize, c: usize, ch: char) -> bool {
    for k in 0..9 {
        if board[r][k] == ch || board[k][c] == ch {
            return false;
        }
    }
    let (br, bc) = ((r / 3) * 3, (c / 3) * 3);
    for i in 0..3 {
        for j in 0..3 {
            if board[br + i][bc + j] == ch {
                return false;
            }
        }
    }
    true
}

fn dfs_mrv(board: &mut [Vec<char>]) -> bool {
    let mut best: Option<(usize, usize, usize)> = None;

    // best_cnt 一旦归零就不必再扫，直接认定当前分支无解
    'scan: for r in 0..9 {
        for c in 0..9 {
            if board[r][c] != '.' {
                continue;
            }
            let cnt = ('1'..='9')
                .filter(|&ch| can_place(board, r, c, ch))
                .count();
            if best.map_or(true, |(_, _, b)| cnt < b) {
                best = Some((r, c, cnt));
            }
            if cnt == 0 {
                break 'scan;
            }
        }
    }

    let Some((r, c, cnt)) = best else {
        return true; // 没有空格，棋盘已填满
    };
    if cnt == 0 {
        return false; // 有空格却无候选，当前分支死了
    }

    for ch in '1'..='9' {
        if !can_place(board, r, c, ch) {
            continue;
        }
        board[r][c] = ch;
        if dfs_mrv(board) {
            return true;
        }
        board[r][c] = '.';
    }
    false
}

// ===== 测试辅助 =====

fn parse_board(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn dump_board(board: &[Vec<char>]) -> Vec<String> {
    board.iter().map(|row| row.iter().collect()).collect()
}

// 校验终局：每行、每列、每宫都恰好是 1-9 各一次
fn is_solved(board: &[Vec<char>]) -> bool {
    for i in 0..9 {
        let (mut row_mask, mut col_mask, mut box_mask) = (0u16, 0u16, 0u16);
        for j in 0..9 {
            let (br, bc) = ((i / 3) * 3 + j / 3, (i % 3) * 3 + j % 3);
            for &ch in &[board[i][j], board[j][i], board[br][bc]] {
                if !('1'..='9').contains(&ch) {
                    return false;
                }
            }
            row_mask |= 1 << (board[i][j] as u16 - '1' as u16);
            col_mask |= 1 << (board[j][i] as u16 - '1' as u16);
            box_mask |= 1 << (board[br][bc] as u16 - '1' as u16);
        }
        if row_mask != 0x1FF || col_mask != 0x1FF || box_mask != 0x1FF {
            return false;
        }
    }
    true
}

// 终局必须原样保留题面给出的数字
fn keeps_clues(board: &[Vec<char>], puzzle: &[&str]) -> bool {
    for r in 0..9 {
        let clue: Vec<char> = puzzle[r].chars().collect();
        for c in 0..9 {
            if clue[c] != '.' && board[r][c] != clue[c] {
                return false;
            }
        }
    }
    true
}

fn check_sudoku(puzzle: &[&str], want: &[&str]) {
    let mut b1 = parse_board(puzzle);
    Solution1::solve_sudoku(&mut b1);
    assert_eq!(dump_board(&b1), want, "位掩码解错");
    assert!(is_solved(&b1) && keeps_clues(&b1, puzzle));

    let mut b2 = parse_board(puzzle);
    Solution2::solve_sudoku(&mut b2);
    assert_eq!(dump_board(&b2), want, "MRV 解错");
    assert!(is_solved(&b2) && keeps_clues(&b2, puzzle));
}

fn main() {
    // 示例 1：题面给定的盘面与唯一解
    check_sudoku(
        &[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ],
        &[
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ],
    );

    // 已知的高难度盘面：验证两条回溯路线都能解出同一终局
    check_sudoku(
        &[
            "8........",
            "..36.....",
            ".7..9.2..",
            ".5...7...",
            "....457..",
            "...1...3.",
            "..1....68",
            "..85...1.",
            ".9....4..",
        ],
        &[
            "812753649",
            "943682175",
            "675491283",
            "154237896",
            "369845721",
            "287169534",
            "521974368",
            "438526917",
            "796318452",
        ],
    );

    // 边界：输入已经填满，两条路线都应原样返回（不能把已填的数擦掉）
    let full = [
        "534678912",
        "672195348",
        "198342567",
        "859761423",
        "426853791",
        "713924856",
        "961537284",
        "287419635",
        "345286179",
    ];

    let mut b1 = parse_board(&full);
    Solution1::solve_sudoku(&mut b1);
    assert_eq!(dump_board(&b1), full);

    let mut b2 = parse_board(&full);
    Solution2::solve_sudoku(&mut b2);
    assert_eq!(dump_board(&b2), full);

    // 边界：只有一个空格，掩码必须能精确挑出唯一候选
    check_sudoku(
        &[
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "28741.635",
            "345286179",
        ],
        &full,
    );

    println!("all tests passed");
}
