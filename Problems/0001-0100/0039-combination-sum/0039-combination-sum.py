"""
============================================================================
39. 组合总和（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/combination-sum/
分类：回溯 · 组合树（元素可重复选取）

给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，
找出 candidates 中可以使数字和为目标数 target 的 所有 不同组合 ，
并以列表形式返回。你可以按 任意顺序 返回这些组合。

candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的
被选数量不同，则两种组合是不同的。

对于给定的输入，保证和为 target 的不同组合数少于 150 个。

示例 1：
输入：candidates = [2,3,6,7], target = 7
输出：[[2,2,3],[7]]
解释：
2 和 3 可以形成一组候选，2 + 2 + 3 = 7 。注意 2 可以使用多次。
7 也是一个候选， 7 = 7 。
仅有这两种组合。

示例 2：
输入: candidates = [2,3,5], target = 8
输出: [[2,2,2,2],[2,3,3],[3,5]]

示例 3：
输入: candidates = [2], target = 1
输出: []

提示：
- 1 <= candidates.length <= 30
- 2 <= candidates[i] <= 40
- candidates 的所有元素 互不相同
- 1 <= target <= 40
"""
"""
解法：排序 + 回溯，下一层从当前下标开始【最优解】

为什么慢：如果按「长度从 1 开始，每层都在全部候选里挑一个」去枚举，会生成
大量顺序不同但元素相同的序列（[2,3,2]、[3,2,2] 与 [2,2,3] 是同一个组合），
还要再把和超过 target 的分支走完才知道无效，长度没有上界，搜索树指数膨胀。

换个角度：强制组合按非递减顺序生成。每一层从 start 开始选，选完 cand[i]
后，下一层仍可再选 cand[i]（允许重复），但绝不回头选下标小于 i 的数。这样
每个组合只对应一条路径，顺序不同的写法根本不会出现。

关键点：
  · 递归下一层的下标是 i，不是 i + 1 —— 这是「同一个数可以重复取」的唯一
    表达；写成 i + 1 就退化成 40 题的「每数只用一次」
  · 不需要任何额外去重：下标非递减已经保证 [2,3,2] 不会被生成，只有 [2,2,3]
  · 排序是为了剪枝：只有升序时，「当前候选已大于剩余和」才能推出「后面
    所有候选都更大」，从而 break 整层
  · remain == 0 在递归入口收集，而不是在循环里等到 remain < 0 再判
  · 用 break 而不是 continue：continue 只跳过当前值，后面更大的值仍会被
    无谓地试一遍，剪枝等于没做

手推一遍：
  以示例 1（candidates = [2,3,6,7]，target = 7）为例：
  start = 0 选 2 → remain = 5
    再选 2 → remain = 3
      再选 2 > 3，break
      改选 3 → remain = 0，收集 [2,2,3]
      改选 6 > 0，break
  回到 start = 0 选 3 → remain = 4
    再选 3 → remain = 1，接着选 3 就超过，break，该分支无解
  选 6 → remain = 1，下一个 6 就超过，break
  选 7 → remain = 0，收集 [7]
  结果 [[2,2,3],[7]] ✔

复杂度分析：
  时间：与搜索树节点数同阶；最坏上界 O(n^(target / min + 1))，其中 n 是
    候选个数、min 是最小候选值，指数项就是递归深度。排序 + break 会把
    「剩余和装不下最小候选」的分支整片砍掉，实际远小于这个上界
  空间 O(target / min)：递归深度不超过「一直选最小候选」的层数，path 同阶；
    返回结果不计入
"""


class Solution:
    def combinationSum(self, candidates: list, target: int) -> list:
        # 升序是 break 剪枝的前提
        candidates = sorted(candidates)
        path = []
        ans = []

        def dfs(remain: int, start: int) -> None:
            if remain == 0:
                ans.append(path[:])
                return
            for i in range(start, len(candidates)):
                # 升序下 candidates[i] 已超过剩余和，后面的只会更大
                if candidates[i] > remain:
                    break
                path.append(candidates[i])
                # 传 i：下一层还能再选自己，这是「可重复取」的唯一区别
                dfs(remain - candidates[i], i)
                path.pop()

        dfs(target, 0)
        return ans


# ===== 测试辅助 =====
def normalize(v: list) -> list:
    # 组合内部顺序无意义：先给每个组合排序，再给列表排序
    return sorted(sorted(comb) for comb in v)


def check(candidates: list, target: int, want: list) -> None:
    got = normalize(Solution().combinationSum(candidates, target))
    assert got == normalize(want), f"got {got}, want {want}"


if __name__ == "__main__":
    # 示例 1：既验证 2 可重复取，也验证单个数直接命中
    check([2, 3, 6, 7], 7, [[2, 2, 3], [7]])

    # 示例 2：多组解，含四连 2 与跨值组合
    check([2, 3, 5], 8, [[2, 2, 2, 2], [2, 3, 3], [3, 5]])

    # 示例 3：最小候选都超过 target，答案为空
    check([2], 1, [])

    # 边界：入参乱序，排序后才有的 break 仍要正确
    check([7, 3, 2], 7, [[2, 2, 3], [7]])

    # 边界：只有一个候选，恰好等于 target
    check([2], 2, [[2]])

    # 边界：只有一个候选，必须重复取两次 —— start = i 写错就只剩空集
    check([2], 4, [[2, 2]])

    # 边界：两个候选组成唯一解，验证不会生成 5,3 这样的反向重复
    check([3, 5], 8, [[3, 5]])

    # 边界：恰好只有一个数命中，组合里可以含 1 个元素
    check([8], 8, [[8]])

    # 边界：三种长度的解同时存在
    check([2, 4, 6], 6, [[2, 2, 2], [2, 4], [6]])

    # 边界：target 小于所有候选
    check([5, 6], 3, [])

    print("all tests passed")
