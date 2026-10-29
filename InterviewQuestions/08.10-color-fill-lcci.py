"""
============================================================================
面试题 08.10. 颜色填充（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/color-fill-lcci/
分类：网格 · 泛洪填充（DFS / BFS）

编写函数，实现许多图片编辑软件都支持的「颜色填充」功能。

待填充的图像用二维数组 image 表示，元素为初始颜色值。初始坐标点的
行坐标为 sr 列坐标为 sc。需要填充的新颜色为 newColor 。

「周围区域」是指颜色相同且在上、下、左、右四个方向上存在相连情况的
若干元素。

请用新颜色填充初始坐标点的周围区域，并返回填充后的图像。

示例：
输入：
image = [[1,1,1],[1,1,0],[1,0,1]]
sr = 1, sc = 1, newColor = 2
输出：[[2,2,2],[2,2,0],[2,0,1]]
解释:
初始坐标点位于图像的正中间，坐标 (sr,sc)=(1,1) 。
初始坐标点周围区域上所有符合条件的像素点的颜色都被更改成 2 。
注意，右下角的像素没有更改为 2 ，因为它不属于初始坐标点的周围区域。

提示：
- image 和 image[0] 的长度均在范围 [1, 50] 内。
- 初始坐标点 (sr,sc) 满足 0 <= sr < image.length 和
  0 <= sc < image[0].length 。
- image[i][j] 和 newColor 表示的颜色值在范围 [0, 65535] 内。
"""
import sys
from typing import List

# 50×50 全同色时 DFS 深度可达 2500，默认 1000 上限不够用
sys.setrecursionlimit(300000)

"""
解法一：DFS 泛洪【最优解】

为什么慢：本题没有"只往右、下扩"的限制，同色区域可能绕回来，所以靠线性
扫描解决不了：一维扫描既判断不了"是否与起点连通"，又会把不同区域里的
同色像素混在一起。

换个角度：要填充的就是"与起点同色且四连通"的像素集合。以起点为源做一次
图遍历，只走进颜色等于 oldColor 的格子；走过的格子立刻涂成 newColor，
它自然不再等于 oldColor，于是"上色"本身就充当了 visited。

关键点：
  · 先取 oldColor = image[sr][sc]；若 oldColor == newColor 必须立刻返回，
    否则涂完色格子"仍是旧颜色"，递归会原地打转直到栈溢出——这是本题
    唯一的死循环来源，必须前置判断
  · 递归里先判越界、再判"颜色是否等于 oldColor"，两个条件缺一不可：
    少了前者会越界，少了后者会涂到不属于周围区域的像素
  · 涂色要放在递归之前：先涂再扩散，同一格不会被两个邻居重复进入
  · 返回值就是入参 image 本身（原地修改），不需要深拷贝
  · Python 默认递归上限 1000，而 50×50 全同色时深度可到 2500，自测里
    用 sys.setrecursionlimit 放宽；也可以直接用解法二的迭代写法

手推一遍：以示例（sr=1, sc=1, newColor=2）为例：
  oldColor = 1；起点 (1,1) 涂成 2，向四邻扩散
  (0,0)、(0,1)、(0,2)、(1,0) 依次被涂成 2
  (1,2) 与 (2,1) 的颜色是 0，不等于 oldColor → 停住
  (2,2) 因此永远走不到，最终仍是 1 ✔

复杂度分析：
  时间 O(mn)：每格至多被涂一次，涂过就不再等于 oldColor
  空间 O(mn)：递归栈最深 mn（整张图同色时最坏）

解法二：BFS 泛洪【最优解，与解法一并列】

官方题解给的写法：把递归换成队列，起点涂色后入队，出队时把四邻中仍是
oldColor 的格子涂色并入队，直到队列为空。
  · 时间同为 O(mn)，空间同为 O(mn)
  · 取舍：BFS 不占递归栈，也不受 Python 递归上限约束，是本题更稳的写法；
    DFS 少一个队列对象，代码更短
  · oldColor == newColor 的前置判断同样不能省，理由与解法一相同

复杂度分析：
  时间 O(mn)
  空间 O(mn)：队列最坏装满整张图
"""


class Solution1:
    def floodFill(self, image: List[List[int]], sr: int, sc: int,
                  newColor: int) -> List[List[int]]:
        old_color = image[sr][sc]
        # 新旧颜色相同就无事可做；不判这一条会无限递归
        new_color = newColor
        if old_color == new_color:
            return image

        rows, cols = len(image), len(image[0])

        def fill(r: int, c: int) -> None:
            if r < 0 or r >= rows or c < 0 or c >= cols:
                return
            if image[r][c] != old_color:
                return
            # 先上色再扩散：涂过的格子不再等于 oldColor，等价于 visited
            image[r][c] = new_color
            fill(r - 1, c)
            fill(r + 1, c)
            fill(r, c - 1)
            fill(r, c + 1)

        fill(sr, sc)
        return image


class Solution2:
    def floodFill(self, image: List[List[int]], sr: int, sc: int,
                  newColor: int) -> List[List[int]]:
        from collections import deque

        old_color = image[sr][sc]
        new_color = newColor
        if old_color == new_color:
            return image

        rows, cols = len(image), len(image[0])
        directions = ((-1, 0), (1, 0), (0, -1), (0, 1))
        image[sr][sc] = new_color
        pending = deque([(sr, sc)])

        while pending:
            r, c = pending.popleft()
            for dr, dc in directions:
                nr, nc = r + dr, c + dc
                if nr < 0 or nr >= rows or nc < 0 or nc >= cols:
                    continue
                if image[nr][nc] != old_color:
                    continue
                # 入队前上色，避免同一格被两个邻居重复入队
                image[nr][nc] = new_color
                pending.append((nr, nc))

        return image


# ===== 测试辅助：双解法对照 =====
# 解法会就地改图，两次调用各用一份副本
def check(image: List[List[int]], sr: int, sc: int, newColor: int,
          want: List[List[int]]) -> None:
    assert Solution1().floodFill([row[:] for row in image], sr, sc,
                                 newColor) == want
    assert Solution2().floodFill([row[:] for row in image], sr, sc,
                                 newColor) == want


if __name__ == "__main__":
    # 示例：中间的四连通 1 区域被涂成 2，右下角的 1 保持不动
    check([[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 2,
          [[2, 2, 2], [2, 2, 0], [2, 0, 1]])

    # 坑一：新颜色与旧颜色相同 —— 不提前返回就会无限递归
    check([[0, 0, 0], [0, 1, 1]], 0, 0, 0,
          [[0, 0, 0], [0, 1, 1]])

    # 坑二：起点旧颜色本来就等于新颜色，整片区域都不该变
    check([[1, 1], [1, 1]], 0, 0, 1,
          [[1, 1], [1, 1]])

    # 边界：单像素图
    check([[5]], 0, 0, 9, [[9]])

    # 边界：起点四周颜色都不同，只改起点自己
    check([[1, 2], [3, 4]], 0, 0, 7,
          [[7, 2], [3, 4]])

    # 边界：整张图同色，一次泛洪涂满
    check([[1, 1], [1, 1]], 0, 0, 3,
          [[3, 3], [3, 3]])

    # 反直觉：只有一条拐弯的通道相连，绕远的格子也必须涂到
    check([[1, 1, 0], [0, 1, 0], [0, 1, 1]], 0, 0, 9,
          [[9, 9, 0], [0, 9, 0], [0, 9, 9]])

    # 边界：颜色取到上界 65535
    check([[65535, 0]], 0, 1, 65535, [[65535, 65535]])

    # 边界：50×50 全同色，递归深度到 2500，靠放宽后的上限才跑得动
    big = [[7] * 50 for _ in range(50)]
    check(big, 0, 0, 8, [[8] * 50 for _ in range(50)])

    print("all tests passed")
