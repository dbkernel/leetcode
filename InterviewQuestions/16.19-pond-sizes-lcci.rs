/*
============================================================================
面试题 16.19. 水域大小（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/pond-sizes-lcci/
分类：网格 · 深度优先搜索（八连通块）

你有一个用于表示一片土地的整数矩阵 land，该矩阵中每个点的值代表对应
地点的海拔高度。若值为0则表示水域。由垂直、水平或对角连接的水域为池塘。
池塘的大小是指相连接的水域的个数。编写一个方法来计算矩阵中所有池塘的
大小，返回值需要从小到大排序。

示例：
输入：
[
  [0,2,1,0],
  [0,1,0,1],
  [1,1,0,1],
  [0,1,0,1]
]
输出： [1,2,4]

提示：
- 0 < len(land) <= 1000
- 0 < len(land[i]) <= 1000
*/

/*
解法：DFS 统计八连通块大小【最优解】

为什么慢：池塘大小要"数清一块连通水域里有多少格"。逐行扫描时不知道当前
水格属于哪一块，也无法避免同一块被反复计数；若先把水格坐标全部收集起来
再两两判断是否相邻，更是 O((mn)^2)。

换个角度：每个水格只属于一个池塘。以每个尚未访问的水格为起点做一次八
方向遍历，遍历中累加格子数，这个数就是该池塘的大小；所有水格各起一次，
就得到全部池塘。每格只被数一次，总代价仍是线性于格子数。

关键点：
  · 方向是八连通：上下左右加四个对角，共 8 个偏移；漏掉对角会把斜着
    连成一片的池塘拆成好几块
  · 访问标记就地做：遍历时把 0 改成任意非 0 值（这里用 -1），"是否为 0"
    就同时充当了 visited，不必另开一张布尔矩阵
  · 计数在进入格子时加一，且与标记同时发生，否则同一格会被多个邻居
    重复计数
  · 结果必须升序：收集完统一 sort_unstable，不要依赖遍历顺序（按行扫描
    天然无序）
  · 没有水时返回空 Vec，不能返回长度为 1、内容为 0 的数组
  · 递归深度最坏是 mn（1000×1000 全水时可达 10^6 层），C++ / Python 都会
    先爆栈；工程上把递归换成显式栈即可，本题自测用的是小尺寸用例
  · Rust 的 8 个偏移用 i32 算完再转 usize，避免 usize 减法下溢

手推一遍：以示例为例：
  (0,0) 起搜 → 连到 (1,0)，大小 2
  (0,3) 起搜 → 经 (1,2)、(2,2) 连到 (3,2)，大小 4
  (3,0) 起搜 → 八邻全是非 0，大小 1
  收集 {2,4,1} → 排序 → [1,2,4] ✔

复杂度分析：
  时间 O(mn + p log p)：每格至多访问一次；p 为池塘个数且 p ≤ mn，
    排序 p 项，上界可记作 O(mn log(mn))
  空间 O(mn)：递归栈最深 mn（整张图都是水时最坏）
*/

// rustc 单文件编译时没有平台预置的 Solution，这里显式声明
pub struct Solution;

impl Solution {
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        if land.is_empty() || land[0].is_empty() {
            return Vec::new();
        }
        let (rows, cols) = (land.len(), land[0].len());
        let mut sizes = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                // 只从未访问过的水格起搜，每块池塘只会被起一次
                if land[r][c] != 0 {
                    continue;
                }
                sizes.push(drain(&mut land, rows, cols, r, c));
            }
        }

        // 题目要求升序，遍历顺序本身给不出顺序
        sizes.sort_unstable();
        sizes
    }
}

// 调用前已确认 land[r][c] == 0；入口先标记再递归，避免重复计数
fn drain(
    land: &mut [Vec<i32>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
) -> i32 {
    land[r][c] = -1; // 就地标记：非 0 即"已访问"
    let mut size = 1;

    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if land[nr][nc] == 0 {
                size += drain(land, rows, cols, nr, nc);
            }
        }
    }
    size
}

// ===== 测试辅助：断言结果完全等于期望的升序数组 =====
fn check(land: Vec<Vec<i32>>, want: Vec<i32>) {
    assert_eq!(Solution::pond_sizes(land.clone()), want, "land = {:?}", land);
}

fn main() {
    // 示例：三块池塘，大小分别是 2、4、1
    check(
        vec![
            vec![0, 2, 1, 0],
            vec![0, 1, 0, 1],
            vec![1, 1, 0, 1],
            vec![0, 1, 0, 1],
        ],
        vec![1, 2, 4],
    );

    // 反直觉：四种大小都要出现，且结果必须严格升序
    check(
        vec![
            vec![0, 0, 0, 1, 1, 1],
            vec![0, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 0, 1],
            vec![0, 1, 1, 1, 1, 1],
        ],
        vec![1, 2, 3, 5],
    );

    // 边界：全水，只有一块
    check(vec![vec![0, 0], vec![0, 0]], vec![4]);
    check(
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        vec![9],
    );

    // 边界：完全没有水，返回空数组
    check(vec![vec![1, 2], vec![3, 4]], vec![]);

    // 边界：单格
    check(vec![vec![0]], vec![1]);
    check(vec![vec![7]], vec![]);

    // 反直觉：只有对角相连也算一个池塘
    check(vec![vec![0, 1], vec![1, 0]], vec![2]);

    // 边界：一维排列，池塘被陆地点开
    check(vec![vec![0, 0, 1, 0]], vec![1, 2]);

    println!("all tests passed");
}
