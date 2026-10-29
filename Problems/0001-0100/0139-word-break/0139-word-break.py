"""
============================================================================
139. 单词拆分（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/word-break/
分类：动态规划 · 字符串切分（可达性判定）

给你一个字符串 s 和一个字符串列表 wordDict 作为字典。如果可以利用字典中
出现的一个或多个单词拼接出 s 则返回 true。

注意：不要求字典中出现的单词全部都使用，并且字典中的单词可以重复使用。

示例 1：
输入: s = "leetcode", wordDict = ["leet", "code"]
输出: true
解释: 返回 true 因为 "leetcode" 可以由 "leet" 和 "code" 拼接成。

示例 2：
输入: s = "applepenapple", wordDict = ["apple", "pen"]
输出: true
解释: 返回 true 因为 "applepenapple" 可以由 "apple" "pen" "apple" 拼接成。
     注意，你可以重复使用字典中的单词。

示例 3：
输入: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
输出: false

提示：
1 <= s.length <= 300
1 <= wordDict.length <= 1000
1 <= wordDict[i].length <= 20
s 和 wordDict[i] 仅由小写英文字母组成
wordDict 中的所有字符串 互不相同
"""

"""
解法一：动态规划（内层倒序 + 命中即停）【最优解】

为什么慢：把「在哪些位置切开」直接枚举，n 个间隙每个可切可不切，一共
2^(n-1) 种切法，逐个去查字典显然不行。慢在把「怎么切」当成了要枚举的对象。

换个角度：真正需要知道的只是「前缀 s[0..i) 能不能拼出来」这一件事。它是
一个布尔量，并且只依赖更小的前缀：只要存在一个切点 j，s[0..j) 可拼、且
s[j..i) 是字典里的词，s[0..i) 就可拼。

关键点：
  · dp[0] = True：空前缀天然可拼，它是唯一不需要切点的起点
  · 内层 j 从 i-1 往 0 倒着走，命中就 break：正确性不依赖遍历顺序，但只要有
    一个切点成立就可以停，倒序让「末尾那个词较短」的常见情形一两次就比较中
  · 单词长度超过 i 时 s[j:i] 的长度对不上，自然查不到，不必特判
  · 字典用 set 存：题面保证 wordDict 内不重复，即使用例里混入重复词也只会
    命中一次，不会重复计数
  · 外层必须从 1 递推到 n，dp[i] 只依赖下标更小的 dp[j]

以示例 1（s = "leetcode"，wordDict = ["leet","code"]）为例：
  i=1..3：没有任何后缀是词 → dp 全为 False
  i=4：j=0 时 "leet" 命中且 dp[0] 为真 → dp[4] = True
  i=5..7：以 "l" 开头的前缀不可拼，无从接续 → False
  i=8：j=4 时 "code" 命中且 dp[4] 为真 → dp[8] = True ✔

复杂度分析：
  时间 O(n^2)：两层循环最多 n^2/2 次子串查询，每次哈希 O(L)，L <= 20 视为常数
  空间 O(n)：布尔表 n+1 格；字典另占 O(字典总长)
"""


class Solution1:
    def wordBreak(self, s: str, wordDict: list) -> bool:
        dict_set = set(wordDict)
        n = len(s)

        # dp[i]：s 的前 i 个字符能否由字典拼出
        dp = [False] * (n + 1)
        dp[0] = True  # 空前缀天然可拼

        for i in range(1, n + 1):
            for j in range(i - 1, -1, -1):
                if dp[j] and s[j:i] in dict_set:
                    dp[i] = True
                    break  # 有一个切点成立就够了，剩下的不必再看
        return dp[n]


"""
解法二：记忆化回溯【最优解，与解法一并列】

为什么慢：回溯本身是对的 —— 从位置 0 出发，枚举这一刀切在哪里，再递归
处理剩下的部分。但它会从不同的路径反复到达同一个起点，同一个子问题被重
算多次。

换个角度：把「从位置 pos 开始的剩余串能否拼出」当成唯一的状态。状态只有
n+1 个，第一次算完就记下来，后面直接查。

关键点：
  · 备忘录用 None 表示未知：False 也是合法结果，不能用假值判断
  · pos == n 返回 True，这是唯一的成功出口
  · 某个 end 成功就立刻返回，不必试完剩下的切点
  · 递归深度最多 n（300），不会触及默认栈上限

复杂度分析：
  时间 O(n^2)：每个状态最多枚举 O(n) 个切点，每个只算一次
  空间 O(n)：备忘录 n+1 格，另加 O(n) 的递归栈
"""


class Solution2:
    def wordBreak(self, s: str, wordDict: list) -> bool:
        dict_set = set(wordDict)
        n = len(s)
        memo = [None] * (n + 1)

        def dfs(pos: int) -> bool:
            if pos == n:
                return True
            if memo[pos] is not None:
                return memo[pos]

            for end in range(pos + 1, n + 1):
                if s[pos:end] in dict_set and dfs(end):
                    memo[pos] = True
                    return True
            memo[pos] = False
            return False

        return dfs(0)


"""
解法三：BFS（按可达位置建图）【最优解，与解法一、二并列】

换个角度：把每个下标当成图上的点，若 s[start:end] 是字典里的词，就从
start 连一条有向边到 end。原问题就变成「从 0 出发能否走到 n」，直接 BFS。

关键点：
  · 入队时就要标记 visited，不能等出队再标：否则同一个位置会被多个前驱
    重复入队，队列膨胀
  · 一旦发现 end == n 立刻返回 True，不必把队列跑空
  · 只从「可达的位置」往后扩展，天然跳过了 dp 里那些 dp[j] 为假的下标

复杂度分析：
  时间 O(n^2)：每个位置出队一次，每次扫描 O(n) 个终点
  空间 O(n)：visited 与队列最多各存 n+1 个下标

三解时间同为 O(n^2)、空间同为 O(n)，并列最优。取舍：DP 不占递归栈、常数
最小，是最稳的一版；记忆化回溯只展开从 0 真正走得到的位置，字典很"长"时
实际比较次数更少，但占 O(n) 的递归栈；BFS 与记忆化工作量同阶，好处是完全
迭代、没有栈，并且天然按「切了几刀」分层，题目若改问最少切几刀，只有它能
直接给出层数。
"""


class Solution3:
    def wordBreak(self, s: str, wordDict: list) -> bool:
        dict_set = set(wordDict)
        n = len(s)

        seen = [False] * (n + 1)
        queue = [0]
        seen[0] = True

        head = 0
        while head < len(queue):
            start = queue[head]
            head += 1
            for end in range(start + 1, n + 1):
                if s[start:end] not in dict_set:
                    continue
                if end == n:
                    return True
                if not seen[end]:
                    seen[end] = True
                    queue.append(end)
        return False


# 三个解法的期望值一致，统一在这里断言
def check(s: str, wordDict: list, want: bool) -> None:
    assert Solution1().wordBreak(s, wordDict) == want, (s, "dp")
    assert Solution2().wordBreak(s, wordDict) == want, (s, "memo")
    assert Solution3().wordBreak(s, wordDict) == want, (s, "bfs")


if __name__ == "__main__":
    # 示例 1
    check("leetcode", ["leet", "code"], True)

    # 示例 2：同一个词可以重复使用
    check("applepenapple", ["apple", "pen"], True)

    # 示例 3：前缀能拼出一半，后半段接不上
    check("catsandog", ["cats", "dog", "sand", "and", "cat"], False)

    # 边界：单字符命中 / 落空
    check("a", ["a"], True)
    check("a", ["b"], False)

    # 边界：字典里的词比 s 还长
    check("ab", ["abc"], False)

    # 关键点用例：多种切法，短词优先也能拼成功
    check("aaaaaa", ["aaaa", "aa"], True)

    # 关键点用例：最后一位落单，任何切法都接不上
    check("aaaaab", ["aaaa", "aa"], False)

    # 反直觉：字典里混入重复词，不应该重复计数
    check("aa", ["a", "a"], True)

    # 反直觉：更长的词能覆盖短词拼不出的位置
    check("goalspecial", ["go", "goal", "goals", "special"], True)

    # 规模上界：300 个 a 全用短词拼
    check("a" * 300, ["a", "aa"], True)

    print("all tests passed")
