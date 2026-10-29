r"""
============================================================================
字典树（Trie / 前缀树）
分类：手写数据结构 · 树
操作复杂度：
  insert       O(L)   L 为单词长度，沿字符一路向下建节点
  search       O(L)   整词：走完 L 步后看 end > 0
  starts_with  O(L)   前缀：走完 L 步只要节点存在即可
  count_prefix O(L)   返回该节点被多少单词经过
  count_word   O(L)
  erase        O(L)   沿路径 pass 减一，可选自底向上剪枝
  空间 O(总字符数 * 26)；共享前缀的单词越多越省
============================================================================

用途：
  前缀匹配、自动补全、敏感词过滤、单词搜索题的加速结构。
  面试常考：实现 Trie、前缀计数、支持重复插入与删除。

核心思想：
  把字符串按字符拆成一条路径，根到某节点的路径就是一个前缀。每个
  节点用 26 个槽位指向子节点，节点上记两个计数：pass 表示有多少个
  单词经过这里，end 表示有多少个单词在这里结束。查询时沿路径走，
  走不动就说明没这个前缀。

关键点（不变量）：
  · 用计数而不是 bool：同一个单词插入两次，end 就是 2；删除一次只
    减一，还能查到。用 bool 就无法区分「插入过一次」和「插入过三次」，
    删一次就没了，重复插入的信息直接丢失；
  · pass 与 end 的含义不能混：search 看 end，starts_with 看 pass
    （节点存在即 pass > 0），否则 "app" 会被误判成 "apple" 的整词；
  · 删除必须先确认 count_word > 0，否则会把不存在的路径减成负数；
  · 剪枝只能自底向上，且一旦某层 pass > 0 就必须停：上层还有别的
    单词经过，继续剪会把它们一起删掉。

手推一遍：
  依次插入 app(1 次)、apple(2 次)：
  1. 路径 a-p-p 上每个节点 pass 都加 1，最后把 p 的 end 加 1；
  2. 再插两次 apple：a、p、p、l、e 的 pass 各加 2，e 的 end 变 2；
     此时 root.pass=3、app 节点 pass=3/end=1、e 节点 pass=2/end=2；
  3. count_prefix("app") 返回 3，count_prefix("appl") 返回 2，
     search("app") 为 true，search("appl") 为 false；
  4. erase("app") 只把路径上 pass 各减一、end 减到 0，因为两个
     apple 还经过这层，pass 仍是 2，不触发剪枝。

复杂度分析：
  时间 O(L)：每次操作只走单词长度条边，与树里已有多少单词无关。
  空间 O(总字符数 * 26)：26 叉数组是定长开销，前缀共享能显著省空间。
"""


class TrieNode:
    # pass 是 Python 关键字，属性改名 pass_cnt，含义完全相同
    __slots__ = ("child", "pass_cnt", "end")

    def __init__(self):
        self.child = [None] * 26  # None 表示没有这条边
        self.pass_cnt = 0  # 有多少个单词经过这个节点
        self.end = 0       # 有多少个单词在这个节点结束


def valid_word(w):
    """只接受非空的全小写单词"""
    if not w:
        return False
    for ch in w:
        if ch < "a" or ch > "z":
            return False
    return True


class Trie:
    def __init__(self):
        self.root = TrieNode()
        self.words = 0  # 单词总数，含重复

    def insert(self, w):
        if not valid_word(w):
            return False
        cur = self.root
        cur.pass_cnt += 1  # 根节点也要记：空前缀的计数靠它
        for ch in w:
            c = ord(ch) - ord("a")
            if cur.child[c] is None:
                cur.child[c] = TrieNode()
            cur = cur.child[c]
            cur.pass_cnt += 1
        cur.end += 1
        self.words += 1
        return True

    def search(self, w):
        """整词查询：必须走完且该节点有单词结束"""
        if not valid_word(w):
            return False
        n = self._walk(w)
        return n is not None and n.end > 0

    def starts_with(self, p):
        """前缀查询：只要路径走得通就算存在"""
        if not p:
            return self.words > 0
        if not valid_word(p):
            return False
        return self._walk(p) is not None

    def count_prefix(self, p):
        """有多少个单词以 p 为前缀；空前缀返回单词总数"""
        if not p:
            return self.words
        if not valid_word(p):
            return 0
        n = self._walk(p)
        return 0 if n is None else n.pass_cnt

    def count_word(self, w):
        """该词被插入了几次"""
        if not valid_word(w):
            return 0
        n = self._walk(w)
        return 0 if n is None else n.end

    def erase(self, w):
        """删除一次出现；没插过则返回 False"""
        if not valid_word(w) or self.count_word(w) == 0:
            return False
        cur = self.root
        # 记下整条路径，方便自底向上剪枝
        nodes = [cur]
        letters = []
        cur.pass_cnt -= 1
        for ch in w:
            c = ord(ch) - ord("a")
            cur = cur.child[c]
            cur.pass_cnt -= 1
            nodes.append(cur)
            letters.append(c)
        cur.end -= 1
        self.words -= 1
        for i in range(len(w) - 1, -1, -1):
            if nodes[i + 1].pass_cnt == 0:
                # 没有单词再经过这个节点了，剪掉它
                nodes[i].child[letters[i]] = None
            else:
                break  # 上层还有别的单词，继续剪会误伤
        return True

    def _walk(self, w):
        cur = self.root
        for ch in w:
            c = ord(ch) - ord("a")
            if cur.child[c] is None:
                return None
            cur = cur.child[c]
        return cur


def main():
    # 1. 插入 / 整词查询 / 前缀查询 / 前缀计数
    t = Trie()
    assert t.insert("apple")
    assert t.insert("app")
    assert t.insert("apply")
    assert t.search("apple")
    assert t.search("app")
    assert t.starts_with("app")
    assert t.starts_with("appl")
    assert t.count_prefix("app") == 3
    assert t.count_prefix("appl") == 2
    assert t.count_prefix("a") == 3
    assert t.count_word("app") == 1
    # 前缀存在但整个词不存在
    assert not t.search("appl")
    assert not t.search("ap")
    assert not t.starts_with("b")
    print("prefix app count:", t.count_prefix("app"))

    # 2. 重复插入：计数累加，search 只看 end > 0
    t = Trie()
    assert t.insert("abc")
    assert t.insert("abc")
    assert t.insert("abc")
    assert t.count_word("abc") == 3
    assert t.count_prefix("ab") == 3
    assert t.count_prefix("abc") == 3
    assert t.search("abc")
    # 删一次还有两次
    assert t.erase("abc")
    assert t.count_word("abc") == 2
    assert t.search("abc")
    assert t.count_prefix("ab") == 2
    assert t.erase("abc")
    assert t.erase("abc")
    assert not t.search("abc")
    assert not t.starts_with("ab")  # 剪枝后路径不存在
    assert t.count_prefix("ab") == 0
    assert t.count_word("abc") == 0
    # 删光了再删一次必须失败
    assert not t.erase("abc")

    # 3. 空前缀：存在的口径是「树里还有单词」
    t = Trie()
    assert not t.starts_with("")
    assert t.count_prefix("") == 0
    assert not t.search("")
    t.insert("x")
    assert t.starts_with("")
    assert t.count_prefix("") == 1
    assert not t.search("")  # 没插入过空串
    t.erase("x")
    assert not t.starts_with("")
    assert t.count_prefix("") == 0

    # 4. 非小写字母：一律拒绝
    t = Trie()
    assert not t.insert("Abc")
    assert not t.insert("aBc")
    assert not t.insert("ab1")
    assert not t.insert("")
    assert not t.search("Abc")
    assert not t.starts_with("aB")
    assert t.count_prefix("a1") == 0
    assert not t.erase("a b")
    assert t.count_prefix("") == 0

    # 5. 空树删除 / 删除不存在的词
    t = Trie()
    assert not t.erase("a")
    assert not t.search("a")
    assert t.insert("ab")
    assert not t.erase("a")  # 只是前缀，不是单词
    assert t.search("ab")    # 不能误删
    assert not t.erase("abc")
    assert t.count_word("ab") == 1

    # 6. 剪枝后仍可复用同一层
    t = Trie()
    assert t.insert("ab")
    assert t.erase("ab")
    assert not t.starts_with("a")
    assert t.insert("ac")  # 重新建 a 这一层
    assert t.search("ac")
    assert not t.search("ab")
    assert t.count_prefix("a") == 1

    print("all tests passed")


if __name__ == "__main__":
    main()
