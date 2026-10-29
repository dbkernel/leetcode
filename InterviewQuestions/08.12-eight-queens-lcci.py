"""
============================================================================
面试题 08.12. 八皇后（困难）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/eight-queens-lcci/
分类：回溯 · 逐行放置与冲突集合

设计一种算法，打印 N 皇后在 N × N 棋盘上的各种摆法，其中每个皇后都不同行、
不同列，也不在对角线上。这里的“对角线”指的是所有的对角线，不只是主对角线。

示例:
 输入：4
 输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 解释: 4 皇后问题存在如下两个不同的解法。
[
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]

提示：
- 1 <= n <= 9
"""
from typing import List


"""
解法：逐行放置 + 列/主对角/副对角三个冲突集合【最优解】

本题与 51. N 皇后 的返回结构一致，都是"全部方案的棋盘列表"，题面差异只在
措辞（这里强调"所有的对角线"，即两条斜向都要查）。因此和解法、代码可以逐字
对照 51 阅读；唯一要留神的是别把副对角线漏掉。

为什么慢：最直白的写法是往 n×n 个格子里硬塞 n 个皇后，再逐一检查两两是否
同行、同列、同对角线，枚举量约 C(n^2, n)，n = 8 时已经上亿；而且大量分支在
放下第二个皇后时就注定失败，全都白枚举了。

换个角度：每行必须恰好一个皇后，于是"放哪个格子"退化成"第 row 行的皇后
放在哪一列"，搜索树变成 n 叉、深度 n。冲突判断也不必两两比较，只要让列、
主对角线、副对角线各用一个集合记账，放置前 O(1) 查一次即可。

关键点：
  · 主对角线（左上到右下）上 col - row 恒定，加 n - 1 偏移后落在 [0, 2n-2]
  · 副对角线（右上到左下）上 col + row 恒定，直接落在 [0, 2n-2]
  · 先查三个集合再落子，递归返回时三个集合必须一起撤销，
    漏撤任何一个都会让后续分支误判为冲突
  · 递归到 row == n 才收集棋盘：此时每行一个皇后且无冲突，必然合法
  · 输出顺序不固定，测试必须按集合比较，不能按列表下标比较

以示例（n = 4）为例：
  row 0 放 col 1，占用列 1、主对角 1-0 = 1、副对角 1+0 = 1
  row 1 只能放 col 3（col 0、col 2 分别撞列与对角线）
  row 2 放 col 0，row 3 放 col 2，得到 ".Q.."/"...Q"/"Q..."/"..Q."
  回溯到 row 0 改放 col 2，对称地得到第二个解 ✔

复杂度分析：
  时间 O(n!)：第 row 行最多 n - row 个可选列（实际更少），搜索树规模不超过
    排列数 n!，每个叶子再花 O(n^2) 拼棋盘；n <= 9 时完全够用
  空间 O(n)：三个集合各 O(n)，递归栈 O(n)；结果集按题目要求不计入
"""


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        cols = [False] * n
        diag1 = [False] * (2 * n - 1)  # col - row + n - 1
        diag2 = [False] * (2 * n - 1)  # col + row
        board = [["."] * n for _ in range(n)]
        answer: List[List[str]] = []

        def dfs(row: int) -> None:
            # 每行恰好一个皇后，走满 n 行就一定是一个完整合法解
            if row == n:
                answer.append(["".join(r) for r in board])
                return

            for col in range(n):
                d1 = col - row + n - 1
                d2 = col + row
                # 三个集合任一命中都能直接剪掉整棵子树
                if cols[col] or diag1[d1] or diag2[d2]:
                    continue

                cols[col] = diag1[d1] = diag2[d2] = True
                board[row][col] = "Q"

                dfs(row + 1)

                # 撤销必须与置位一一对应，否则后续分支会继承脏状态
                board[row][col] = "."
                cols[col] = diag1[d1] = diag2[d2] = False

        dfs(0)
        return answer


# 校验一份棋盘：形状正确、恰好 n 个皇后、且任意两皇后不共线
def valid_board(board: List[str]) -> bool:
    n = len(board)
    count = 0
    col = [0] * n
    d1 = [0] * (2 * n - 1)
    d2 = [0] * (2 * n - 1)

    for r in range(n):
        if len(board[r]) != n:
            return False
        for c in range(n):
            if board[r][c] == ".":
                continue
            if board[r][c] != "Q":
                return False
            count += 1
            # 同行由「每行只写一个 Q」的构造保证，这里只查剩余三类
            col[c] += 1
            d1[c - r + n - 1] += 1
            d2[c + r] += 1
            if col[c] > 1 or d1[c - r + n - 1] > 1 or d2[c + r] > 1:
                return False
    return count == n


def check(n: int, want: int) -> None:
    got = Solution().solveNQueens(n)
    assert len(got) == want, f"n={n} 得到 {len(got)} 个解，期望 {want} 个"
    for board in got:
        assert valid_board(board), f"n={n} 的方案不合法: {board}"
    # 方案不能重复：数量对但内容重复说明回溯路径有 bug
    assert len({tuple(b) for b in got}) == len(got)


if __name__ == "__main__":
    # 示例：n = 4 有两个解
    check(4, 2)

    # 边界：n = 1 只有一个解
    check(1, 1)

    # 边界：n = 2、n = 3 无解
    check(2, 0)
    check(3, 0)

    # 经典规模：八皇后 92 解
    check(8, 92)
    check(5, 10)
    check(6, 4)
    check(7, 40)

    # 约束上界：n = 9，解数 352
    check(9, 352)

    print("all tests passed")
