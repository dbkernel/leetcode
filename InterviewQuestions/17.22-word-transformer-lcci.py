"""
============================================================================
面试题 17.22. 单词转换（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/word-transformer-lcci/
分类：广度优先搜索 · 隐式图路径

给定字典中的两个词，长度相等。写一个方法，把一个词转换成另一个词，
但是一次只能改变一个字符。每一步得到的新词都必须能在字典中找到。

编写一个程序，返回一个可能的转换序列。如有多个可能的转换序列，
你可以返回任何一个。

示例 1：
输入：
beginWord = "hit",
endWord = "cog",
wordList = ["hot","dot","dog","lot","log","cog"]
输出：
["hit","hot","dot","lot","log","cog"]

示例 2：
输入：
beginWord = "hit"
endWord = "cog"
wordList = ["hot","dot","dog","lot","log"]
输出：
[]
解释：
endWord "cog" 不在字典中，所以不存在符合要求的转换序列。

提示：
原题未给出额外约束。
"""
from collections import deque
from typing import List


def build_path(parent: dict, begin: str, end: str) -> List[str]:
    """沿 parent 表从 begin 走到 end，再反转成起点在前的序列。"""
    path = []
    cur = end
    while cur != begin:
        path.append(cur)
        cur = parent[cur]
    path.append(begin)
    path.reverse()
    return path


"""
解法一：单向广度优先搜索【最优解】

为什么慢：如果从起点做深度优先搜索，一条分支走不通就要回溯换一条，
最坏会枚举出指数级的路径；即便加 visited 剪枝，先撞到的终点也不保证
序列短，重新搜索的代价仍高。

换个角度：把每个单词看成图上的点，相差一个字母的两个词之间连一条边。
问题变成"从起点走到终点，输出一条路径"。BFS 按步数一层层向外扩展，
第一次到达终点时，沿 parent 表倒推就是一条合法序列。

关键点：
  · endWord 不在字典里直接返回空数组，这是示例 2 的判定入口
  · 只有字典中的词可以入队；beginWord 是起点，不必在字典里
  · 逐位替换成 26 个字母再查哈希表，比两两比较所有词快一个量级
  · 入队时就写 parent，同一层里多个词指向它也不会重复入队
  · 到达 endWord 立即返回，此时路径已经最短

手推一遍（示例 1）：
  hit -> hot -> dot -> dog -> cog 是一条 5 个词的路径；
  官方示例给的是 hit-hot-dot-lot-log-cog（6 个词），说明题目只要求
  序列合法，不要求最短。BFS 顺带返回了最短的那条，同样被接受。
  parent 记录 hot<-hit、dot<-hot、dog<-dot、cog<-dog，
  从 cog 倒推到 hit 再反转即得 ["hit","hot","dot","dog","cog"] ✔

复杂度分析：
  时间 O(V·L²)：V 为字典词数、L 为词长；每个词出队一次，每次枚举
    L·26 个候选，每个候选要花 O(L) 构造并哈希，26 视为常数
  空间 O(V·L)：字典与 parent 表各存 V 个长度为 L 的字符串

解法二：双向广度优先搜索【最优解，与解法一并列】

为什么慢：单向 BFS 从起点向外铺，分支因子约为 25L；终点在 d 层时，
要先把前 d-1 层几乎铺满。两个方向各走一半，访问量会小得多。

换个角度：同时维护起点侧与终点侧两张 parent 表，每轮扩展较小的那一侧。
新词一旦出现在另一侧的表里，就找到了相遇点，把两段路径拼起来即可。

关键点：
  · 每轮比较两侧 frontier 大小，只扩展较小的一侧，避免一边跑太深
  · 命中条件是"新词已在另一侧 parent 中"，而不是等到出队
  · 拼接时起点侧要反转（parent 指向起点），终点侧顺着 parent 走向终点
  · 两侧的树在相遇前互不相交，拼出的序列不会出现重复词
  · 最坏仍与解法一同阶，优势在实际访问量；因此两条并列最优

手推一遍（示例 1）：
  起点侧 {hit}，终点侧 {cog}，先扩较小的一侧
  hit 的邻居有 hot -> 记 parentA[hot] = hit
  之后从 hot 扩出 dot、lot；终点侧从 cog 扩出 dog、log
  某一轮 next 同时出现在两张表里，例如 dot，拼接得
  hit-hot-dot-dog-cog ✔

复杂度分析：
  时间 O(V·L²)：两侧合计至多访问每个词一次，单次扩展代价同上
  空间 O(V·L)：两张 parent 表与两个 frontier
"""


class Solution1:
    def findLadders(
        self, beginWord: str, endWord: str, wordList: List[str]
    ) -> List[str]:
        dict_set = set(wordList)

        # 示例 2：终点不在字典里，任何序列都不成立
        if endWord not in dict_set:
            return []
        # 起点即终点时，零步序列本身就是合法答案
        if beginWord == endWord:
            return [beginWord]

        parent = {beginWord: ""}
        queue = deque([beginWord])

        while queue:
            word = queue.popleft()
            chars = list(word)
            for i in range(len(chars)):
                old = chars[i]
                for code in range(ord("a"), ord("z") + 1):
                    c = chr(code)
                    if c == old:
                        continue
                    chars[i] = c
                    nxt = "".join(chars)

                    # 不在字典里的词不能作为中间步骤
                    if nxt not in dict_set:
                        continue
                    # 已经入队过的词不再改写 parent，保证同一层不重复
                    if nxt in parent:
                        continue

                    parent[nxt] = word
                    if nxt == endWord:
                        return build_path(parent, beginWord, endWord)
                    queue.append(nxt)
                # chars 是复用的缓冲区，本位置试完后恢复
                chars[i] = old

        return []


class Solution2:
    def findLadders(
        self, beginWord: str, endWord: str, wordList: List[str]
    ) -> List[str]:
        dict_set = set(wordList)

        if endWord not in dict_set:
            return []
        if beginWord == endWord:
            return [beginWord]

        parent_a = {beginWord: ""}
        parent_b = {endWord: ""}
        front_a = {beginWord}
        front_b = {endWord}

        while front_a and front_b:
            # 扩较小的一侧，让两边的深度尽量同步
            expand_a = len(front_a) <= len(front_b)
            if expand_a:
                front, parent, other = front_a, parent_a, parent_b
            else:
                front, parent, other = front_b, parent_b, parent_a

            next_front = set()
            for word in front:
                chars = list(word)
                for i in range(len(chars)):
                    old = chars[i]
                    for code in range(ord("a"), ord("z") + 1):
                        c = chr(code)
                        if c == old:
                            continue
                        chars[i] = c
                        nxt = "".join(chars)

                        if nxt not in dict_set:
                            continue
                        if nxt in parent:
                            continue

                        parent[nxt] = word
                        if nxt in other:
                            return self._join(parent_a, parent_b, beginWord,
                                             endWord, nxt)
                        next_front.add(nxt)
                    chars[i] = old

            if expand_a:
                front_a = next_front
            else:
                front_b = next_front

        return []

    def _join(self, parent_a: dict, parent_b: dict, begin: str, end: str,
              meet: str) -> List[str]:
        # 起点侧：从相遇点沿 parent 走到 begin，再反转
        path = []
        cur = meet
        while cur != begin:
            path.append(cur)
            cur = parent_a[cur]
        path.append(begin)
        path.reverse()

        # 终点侧：相遇点已经写入，从它的下一个词接到 end
        cur = meet
        while cur != end:
            cur = parent_b[cur]
            path.append(cur)
        return path


# ===== 测试辅助：校验序列是否合法，而不是比对某一条固定路径 =====
def one_char_apart(a: str, b: str) -> bool:
    diff = 0
    for x, y in zip(a, b):
        if x != y:
            diff += 1
            # 第二个不同字符出现即可判定失败
            if diff > 1:
                return False
    return diff == 1


def valid_path(path: List[str], begin: str, end: str,
               word_list: List[str]) -> bool:
    if not path:
        return False
    if path[0] != begin or path[-1] != end:
        return False

    dict_set = set(word_list)
    # 起点不需要在字典里，之后每一步都必须是字典中的词
    for i in range(1, len(path)):
        if not one_char_apart(path[i - 1], path[i]) or path[i] not in dict_set:
            return False
    return True


def check(begin: str, end: str, word_list: List[str], has_path: bool) -> None:
    got_bfs = Solution1().findLadders(begin, end, word_list)
    if has_path:
        assert valid_path(got_bfs, begin, end, word_list), (begin, end, got_bfs)
    else:
        assert got_bfs == [], (begin, end, got_bfs)

    got_bi = Solution2().findLadders(begin, end, word_list)
    if has_path:
        assert valid_path(got_bi, begin, end, word_list), (begin, end, got_bi)
    else:
        assert got_bi == [], (begin, end, got_bi)


if __name__ == "__main__":
    # 示例 1：多条合法路径，只校验合法性
    check("hit", "cog", ["hot", "dot", "dog", "lot", "log", "cog"], True)

    # 示例 2：endWord 不在字典中
    check("hit", "cog", ["hot", "dot", "dog", "lot", "log"], False)

    # 边界：起点即终点，零步序列
    check("hot", "hot", ["hot"], True)

    # 边界：字典里只有终点，和起点差多个字符
    check("hit", "cog", ["cog"], False)

    # 边界：单字符词，a -> b -> c
    check("a", "c", ["b", "c"], True)

    # 边界：需要连续改四个位置
    check("aaaa", "dddd", ["aaad", "aadd", "addd", "dddd"], True)

    # 反直觉：存在多条等长路径，不能假定某一条固定顺序
    check("ab", "cd", ["ac", "ad", "cc", "cd"], True)

    # 反直觉：终点可达但必须经过中间词，不能跳到 "abcd"
    check("ab", "cd", ["abcd", "cd"], False)

    # 边界：词表中混入与起点相同的词，不应影响 parent
    check("hit", "cog", ["hit", "hot", "dot", "dog", "cog"], True)

    # 边界：26 个单字母词构成的完全图，任意两点一步可达
    letters = [chr(code) for code in range(ord("a"), ord("z") + 1)]
    check("a", "z", letters, True)

    print("all tests passed")
