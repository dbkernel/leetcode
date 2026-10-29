/*
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
*/
use std::collections::VecDeque;

/*
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
  · Rust 下标是 usize：越界判断必须先比较再减法，`r - 1 >= 0` 会下溢 panic

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
  · 取舍：BFS 不占递归栈，50×50 的图两种写法都够用；若把规模放大到
    1000×1000 全同色，DFS 会先死在递归深度上，BFS 只是队列变长
  · oldColor == newColor 的前置判断同样不能省，理由与解法一相同

复杂度分析：
  时间 O(mn)
  空间 O(mn)：队列最坏装满整张图
*/

// ---------- 解法一：DFS 泛洪 ----------
pub struct Solution1;

impl Solution1 {
    pub fn flood_fill(
        mut image: Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        new_color: i32,
    ) -> Vec<Vec<i32>> {
        let (r, c) = (sr as usize, sc as usize);
        let old_color = image[r][c];
        // 新旧颜色相同就无事可做；不判这一条会无限递归
        if old_color == new_color {
            return image;
        }
        let (rows, cols) = (image.len(), image[0].len());
        fill(&mut image, rows, cols, r, c, old_color, new_color);
        image
    }
}

// 调用前已确认 image[r][c] == old_color；入口先涂色再递归，避免重复进入
fn fill(
    image: &mut [Vec<i32>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    old_color: i32,
    new_color: i32,
) {
    image[r][c] = new_color;
    if r > 0 && image[r - 1][c] == old_color {
        fill(image, rows, cols, r - 1, c, old_color, new_color);
    }
    if r + 1 < rows && image[r + 1][c] == old_color {
        fill(image, rows, cols, r + 1, c, old_color, new_color);
    }
    if c > 0 && image[r][c - 1] == old_color {
        fill(image, rows, cols, r, c - 1, old_color, new_color);
    }
    if c + 1 < cols && image[r][c + 1] == old_color {
        fill(image, rows, cols, r, c + 1, old_color, new_color);
    }
}

// ---------- 解法二：BFS 泛洪 ----------
pub struct Solution2;

impl Solution2 {
    pub fn flood_fill(
        mut image: Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        new_color: i32,
    ) -> Vec<Vec<i32>> {
        let (start_r, start_c) = (sr as usize, sc as usize);
        let old_color = image[start_r][start_c];
        if old_color == new_color {
            return image;
        }
        let (rows, cols) = (image.len(), image[0].len());
        let mut pending: VecDeque<(usize, usize)> = VecDeque::new();

        image[start_r][start_c] = new_color;
        pending.push_back((start_r, start_c));

        while let Some((r, c)) = pending.pop_front() {
            // 先判"下标存在"再访问，无符号减法才不会下溢
            if r > 0 && image[r - 1][c] == old_color {
                image[r - 1][c] = new_color;
                pending.push_back((r - 1, c));
            }
            if r + 1 < rows && image[r + 1][c] == old_color {
                image[r + 1][c] = new_color;
                pending.push_back((r + 1, c));
            }
            if c > 0 && image[r][c - 1] == old_color {
                image[r][c - 1] = new_color;
                pending.push_back((r, c - 1));
            }
            if c + 1 < cols && image[r][c + 1] == old_color {
                image[r][c + 1] = new_color;
                pending.push_back((r, c + 1));
            }
        }
        image
    }
}

// ===== 测试辅助：双解法对照 =====
// 解法会就地改图，两次调用各用一份副本
fn check(
    image: Vec<Vec<i32>>,
    sr: i32,
    sc: i32,
    new_color: i32,
    want: Vec<Vec<i32>>,
) {
    assert_eq!(
        Solution1::flood_fill(image.clone(), sr, sc, new_color),
        want,
        "dfs({:?}, {}, {}, {})",
        image,
        sr,
        sc,
        new_color
    );
    assert_eq!(
        Solution2::flood_fill(image.clone(), sr, sc, new_color),
        want,
        "bfs({:?}, {}, {}, {})",
        image,
        sr,
        sc,
        new_color
    );
}

fn main() {
    // 示例：中间的四连通 1 区域被涂成 2，右下角的 1 保持不动
    check(
        vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]],
        1,
        1,
        2,
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
    );

    // 坑一：新颜色与旧颜色相同 —— 不提前返回就会无限递归
    check(
        vec![vec![0, 0, 0], vec![0, 1, 1]],
        0,
        0,
        0,
        vec![vec![0, 0, 0], vec![0, 1, 1]],
    );

    // 坑二：起点旧颜色本来就等于新颜色，整片区域都不该变
    check(
        vec![vec![1, 1], vec![1, 1]],
        0,
        0,
        1,
        vec![vec![1, 1], vec![1, 1]],
    );

    // 边界：单像素图
    check(vec![vec![5]], 0, 0, 9, vec![vec![9]]);

    // 边界：起点四周颜色都不同，只改起点自己
    check(
        vec![vec![1, 2], vec![3, 4]],
        0,
        0,
        7,
        vec![vec![7, 2], vec![3, 4]],
    );

    // 边界：整张图同色，一次泛洪涂满
    check(
        vec![vec![1, 1], vec![1, 1]],
        0,
        0,
        3,
        vec![vec![3, 3], vec![3, 3]],
    );

    // 反直觉：只有一条拐弯的通道相连，绕远的格子也必须涂到
    check(
        vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 1]],
        0,
        0,
        9,
        vec![vec![9, 9, 0], vec![0, 9, 0], vec![0, 9, 9]],
    );

    // 边界：颜色取到上界 65535
    check(vec![vec![65535, 0]], 0, 1, 65535, vec![vec![65535, 65535]]);

    println!("all tests passed");
}
