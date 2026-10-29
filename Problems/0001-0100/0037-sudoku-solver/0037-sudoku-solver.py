"""
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
"""
from typing import List


"""
解法一：回溯 + 行/列/宫三个位掩码【最优解】

为什么慢：最朴素的写法是给每个空格依次试 1-9，每试一个都扫一遍所在行、列、
宫判断冲突，单次判断 O(27)；递归时又从头扫棋盘找下一个空格，每层还要再花
O(81)。节点数一样，但每个节点都带着常数很大的重复扫描。

换个角度：一行、一列、一个宫各自只需要 9 个比特就能表示"哪些数字已用"。
把冲突判断压成三次按位或，一次位运算就得到当前格的全部候选；再让 pos 单调
向前，每层从上次的位置继续，空格只扫描一遍而不是每层重扫。

关键点：
  · 数字 d（0-8）对应 bit `1 << d`，三个掩码的并集就是"已占用"
  · 候选集 = `~(行 | 列 | 宫) & 0x1FF`，0x1FF 把高位的符号位与无用的位截掉
  · 落子与撤销必须成对：置 rows/cols/boxes 的同时改 board，
    回溯时两者一起还原，否则后续分支会读到脏掩码
  · 找到一个解立即逐层返回 True，不能继续搜索（题目保证唯一解，多搜只会变慢）
  · 空格用 pos 一维编号，跳过题面已给的数字，回溯时传 pos + 1 而不是 0
  · Python 的负整数按补码无限延伸，所以掩码取反后必须显式 `& 0x1FF`

以示例 1 的第一个空格 (0, 2) 为例：
  第 0 行已用 {5,3,7}，第 2 列已用 {8}，左上宫已用 {5,3,6,9,8}
  并集 {3,5,6,7,8,9}，候选 {1,2,4}，先试 4，后续一路递归即可填满 ✔

复杂度分析：
  时间 O(9^(空格数))：最坏要枚举每个空格的候选，实际被掩码剪到很小；
    每个节点只做 O(1) 位运算，远小于每步 O(27) 扫描的写法
  空间 O(1)：三个掩码各 9 个整数，递归深度不超过 81，均与 n 无关
"""


class Solution1:
    def solveSudoku(self, board: List[List[str]]) -> None:
        rows = [0] * 9
        cols = [0] * 9
        boxes = [0] * 9

        def flip(r: int, c: int, d: int, on: bool) -> None:
            bit = 1 << d
            b = (r // 3) * 3 + c // 3
            # 置位与清位成对出现，撤销时不会误伤同一宫里的其它数字
            if on:
                rows[r] |= bit
                cols[c] |= bit
                boxes[b] |= bit
            else:
                rows[r] &= ~bit
                cols[c] &= ~bit
                boxes[b] &= ~bit

        # 先把题面给定的数字登记进掩码，后续只处理空格
        for r in range(9):
            for c in range(9):
                if board[r][c] != ".":
                    flip(r, c, int(board[r][c]) - 1, True)

        def dfs(pos: int) -> bool:
            # pos 一直向前，题面给定的数字直接跳过，不重复扫描
            while pos < 81 and board[pos // 9][pos % 9] != ".":
                pos += 1
            if pos == 81:
                return True

            r, c = pos // 9, pos % 9
            b = (r // 3) * 3 + c // 3
            # 三个掩码取并就是被禁用的数字，取反后与 0x1FF 得到候选位
            avail = ~(rows[r] | cols[c] | boxes[b]) & 0x1FF

            while avail:
                bit = avail & (-avail)
                avail ^= bit
                d = bit.bit_length() - 1

                flip(r, c, d, True)
                board[r][c] = str(d + 1)

                if dfs(pos + 1):
                    return True

                # 这一支失败，格子必须恢复成空格，掩码同步撤销
                board[r][c] = "."
                flip(r, c, d, False)

            return False

        dfs(0)


"""
解法二：回溯 + 每层挑候选最少的空格（MRV）【最优解，与解法一并列】

动机：解法一按固定顺序填空格，遇到"候选很多但后面会死"的格子时，
前面几个分支都是白试。数独里候选最少的空格往往就是最容易证伪的位置，
先把它们填掉能更早剪枝。

换个角度：每层不再取"下一个空格"，而是扫描全盘、算每个空格的合法候选数，
挑候选数最小的那个来展开（Minimum Remaining Values 启发式）。

关键点：
  · 扫描要算满 9 个候选才能比较，单层代价 O(81 × 27)，比解法一的 O(1) 高得多
  · 候选数为 0 要立刻返回 False，这是当前分支已死的直接证据
  · 找不到空格（best_r == -1）说明棋盘已填满，返回 True
  · 递归返回 False 后要把 best_r/best_c 恢复成 '.'，否则上一层会读到残留数字

复杂度分析：
  时间 O(9^(空格数))：与解法一同一量级的上界，但每层多了 O(81 × 27) 的
    扫描成本；启发式只缩小常数级的搜索树，没有改变指数上界
  空间 O(1)：只用几个下标变量，递归深度不超过 81

  取舍：两解的最坏情况上界相同（都是指数级），额外空间都是 O(1)，
  按渐近复杂度属于并列最优。解法一单节点只做 O(1) 位运算，常数更小；
  解法二单节点多出 O(81 × 27) 扫描，但优先展开候选最少的格子，
  在"线索极少"的盘面上搜索树小得多。顺序位掩码胜在常数，
  MRV 胜在实践剪枝，按盘面形态各有所长。
"""


class Solution2:
    def solveSudoku(self, board: List[List[str]]) -> None:
        def can_place(r: int, c: int, ch: str) -> bool:
            for k in range(9):
                if board[r][k] == ch or board[k][c] == ch:
                    return False
            br, bc = (r // 3) * 3, (c // 3) * 3
            for i in range(3):
                for j in range(3):
                    if board[br + i][bc + j] == ch:
                        return False
            return True

        def dfs() -> bool:
            best_r, best_c, best_cnt = -1, -1, 10

            # best_cnt 一旦归零就不必再扫，直接认定当前分支无解
            r = 0
            while r < 9 and best_cnt > 0:
                for c in range(9):
                    if board[r][c] != ".":
                        continue
                    cnt = sum(1 for ch in "123456789" if can_place(r, c, ch))
                    if cnt < best_cnt:
                        best_cnt, best_r, best_c = cnt, r, c
                r += 1

            if best_r == -1:
                return True  # 没有空格，棋盘已填满
            if best_cnt == 0:
                return False  # 有空格却无候选，当前分支死了

            for ch in "123456789":
                if not can_place(best_r, best_c, ch):
                    continue
                board[best_r][best_c] = ch
                if dfs():
                    return True
                board[best_r][best_c] = "."
            return False

        dfs()


# ===== 测试辅助 =====


def parse_board(rows: List[str]) -> List[List[str]]:
    return [list(row) for row in rows]


def dump_board(board: List[List[str]]) -> List[str]:
    return ["".join(row) for row in board]


def is_solved(board: List[List[str]]) -> bool:
    """校验终局：每行、每列、每宫都恰好是 1-9 各一次"""
    for i in range(9):
        row_mask = col_mask = box_mask = 0
        for j in range(9):
            for ch in (board[i][j], board[j][i]):
                if not ("1" <= ch <= "9"):
                    return False
            br, bc = (i // 3) * 3 + j // 3, (i % 3) * 3 + j % 3
            if not ("1" <= board[br][bc] <= "9"):
                return False

            row_mask |= 1 << (ord(board[i][j]) - ord("1"))
            col_mask |= 1 << (ord(board[j][i]) - ord("1"))
            box_mask |= 1 << (ord(board[br][bc]) - ord("1"))
        if row_mask != 0x1FF or col_mask != 0x1FF or box_mask != 0x1FF:
            return False
    return True


def keeps_clues(board: List[List[str]], puzzle: List[str]) -> bool:
    """终局必须原样保留题面给出的数字"""
    for r in range(9):
        for c in range(9):
            if puzzle[r][c] != "." and board[r][c] != puzzle[r][c]:
                return False
    return True


def check_sudoku(puzzle: List[str], want: List[str]) -> None:
    b1 = parse_board(puzzle)
    Solution1().solveSudoku(b1)
    assert dump_board(b1) == want, f"位掩码解错: {dump_board(b1)}"
    assert is_solved(b1) and keeps_clues(b1, puzzle)

    b2 = parse_board(puzzle)
    Solution2().solveSudoku(b2)
    assert dump_board(b2) == want, f"MRV 解错: {dump_board(b2)}"
    assert is_solved(b2) and keeps_clues(b2, puzzle)


if __name__ == "__main__":
    # 示例 1：题面给定的盘面与唯一解
    check_sudoku(
        ["53..7....", "6..195...", ".98....6.",
         "8...6...3", "4..8.3..1", "7...2...6",
         ".6....28.", "...419..5", "....8..79"],
        ["534678912", "672195348", "198342567",
         "859761423", "426853791", "713924856",
         "961537284", "287419635", "345286179"],
    )

    # 已知的高难度盘面：验证两条回溯路线都能解出同一终局
    check_sudoku(
        ["8........", "..36.....", ".7..9.2..",
         ".5...7...", "....457..", "...1...3.",
         "..1....68", "..85...1.", ".9....4.."],
        ["812753649", "943682175", "675491283",
         "154237896", "369845721", "287169534",
         "521974368", "438526917", "796318452"],
    )

    # 边界：输入已经填满，两条路线都应原样返回（不能把已填的数擦掉）
    full = ["534678912", "672195348", "198342567",
            "859761423", "426853791", "713924856",
            "961537284", "287419635", "345286179"]

    b1 = parse_board(full)
    Solution1().solveSudoku(b1)
    assert dump_board(b1) == full

    b2 = parse_board(full)
    Solution2().solveSudoku(b2)
    assert dump_board(b2) == full

    # 边界：只有一个空格，掩码必须能精确挑出唯一候选
    check_sudoku(
        ["534678912", "672195348", "198342567",
         "859761423", "426853791", "713924856",
         "961537284", "28741.635", "345286179"],
        full,
    )

    print("all tests passed")
