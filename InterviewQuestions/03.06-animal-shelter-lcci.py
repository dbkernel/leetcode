"""
============================================================================
面试题 03.06. 动物收容所（简单）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/animal-shelter-lcci/
分类：队列 · 设计（先进先出）

动物收容所。有家动物收容所只收容狗与猫，且严格遵守“先进先出”的原则。在收
养该收容所的动物时，收养人只能收养所有动物中“最老”（由其进入收容所的时间
长短而定）的动物，或者可以挑选猫或狗（同时必须收养此类动物中“最老”的）。
换言之，收养人不能自由挑选想收养的对象。请创建适用于这个系统的数据结构，
实现各种操作方法，比如enqueue、dequeueAny、dequeueDog和dequeueCat。允许
使用Java内置的LinkedList数据结构。

enqueue方法有一个animal参数，animal[0]代表动物编号，animal[1]代表动物种
类，其中 0 代表猫，1 代表狗。

dequeue*方法返回一个列表[动物编号, 动物种类]，若没有可以收养的动物，则返
回[-1,-1]。

示例 1：
输入：
["AnimalShelf", "enqueue", "enqueue", "dequeueCat", "dequeueDog",
 "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [], [], []]
输出：
[null,null,null,[0,0],[-1,-1],[1,0]]

示例 2：
输入：
["AnimalShelf", "enqueue", "enqueue", "enqueue", "dequeueDog",
 "dequeueCat", "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [[2, 1]], [], [], []]
输出：
[null,null,null,null,[2,1],[0,0],[1,0]]

说明：
收纳所的最大容量为20000
"""
from collections import deque
from typing import List


"""
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出标准实现并说明取舍。
另一种常见写法是「一条总队列 + 按类型线性跳过最老的异类动物」，它让
dequeueDog / dequeueCat 最坏 O(n) 且需要额外的临时结构，所以不取。

解法：猫狗各一条队列 + 全局入所序号【最优解】

接口契约：
  · enqueue(animal)：animal = [编号, 种类]，0 是猫、1 是狗，返回 None
  · dequeueAny()：取所有动物中最老的一只，返回 [编号, 种类]
  · dequeueDog() / dequeueCat()：取该类里最老的一只，返回 [编号, 种类]
  · 没有可取动物时统一返回 [-1, -1]

数据结构选型：
  「最老」按入所先后排序，所以每种动物各用一条 FIFO 队列，队首天然就是该
  类最老的一只。dequeueAny 需要跨类比较谁更老，而动物编号并不保证递增，
  不能拿编号当时间戳，于是额外维护一个全局自增的序号，入所时一起入队。
  若用一条总队列，dequeueDog 就得跳过排在前面的猫，只能逐个弹出暂存再放回，
  最坏 O(n)；两条队列把四个操作都压到 O(1)。

各操作的不变量：
  · 每条队列只放同类动物，队首是该类最老的一只
  · 每条队列里的序号严格递增，所以比较两条队首的序号就是比较年龄
  · dequeueAny 从序号更小的那条队列出队，序号相同不可能出现（全局自增）
  · 三个出队操作都只动队首，不会破坏队列内的顺序
  · 两条队列都空时返回 [-1, -1]，与"这一类没有动物"的返回值一致
  · Python 用 collections.deque，popleft 才是 O(1)；list.pop(0) 是 O(n)

手推一遍（示例 2）：
  enqueue([0,0])：猫队列 [(0, 0)]
  enqueue([1,0])：猫队列 [(0, 0), (1, 1)]
  enqueue([2,1])：狗队列 [(2, 2)]
  dequeueDog  → 狗队首 → [2,1]
  dequeueCat  → 猫队首 → [0,0]
  dequeueAny  → 猫队首序号更小 → [1,0]

复杂度分析：
  时间：enqueue / dequeueAny / dequeueDog / dequeueCat 均 O(1)
  空间 O(n)：n 为当前在所的动物数，每只动物只存一个 (序号, 编号)
"""


class AnimalShelf:
    def __init__(self):
        # 元素是 (入所序号, 动物编号)
        self.cats = deque()
        self.dogs = deque()
        self.seq = 0

    # 平台签名：animal[0] 编号、animal[1] 种类
    def enqueue(self, animal: List[int]) -> None:
        # 序号与编号分开存：编号不保证递增，不能用它判断谁更老
        if animal[1] == 0:
            self.cats.append((self.seq, animal[0]))
        else:
            self.dogs.append((self.seq, animal[0]))
        self.seq += 1

    def dequeueAny(self) -> List[int]:
        # 两条都空才算没有动物；只有一类空时取另一类的最老
        if not self.cats and not self.dogs:
            return [-1, -1]
        if not self.dogs:
            return self.dequeueCat()
        if not self.cats:
            return self.dequeueDog()
        # 序号小的入所更早，先被收养
        if self.cats[0][0] < self.dogs[0][0]:
            return self.dequeueCat()
        return self.dequeueDog()

    def dequeueDog(self) -> List[int]:
        if not self.dogs:
            return [-1, -1]
        return [self.dogs.popleft()[1], 1]

    def dequeueCat(self) -> List[int]:
        if not self.cats:
            return [-1, -1]
        return [self.cats.popleft()[1], 0]


# 按题目给的调用序列回放；void 方法在结果里留 None
def run(ops: List[tuple]) -> list:
    shelf = AnimalShelf()
    out = []
    for name, arg in ops:
        if name == "enqueue":
            shelf.enqueue(arg)
            out.append(None)
        elif name == "dequeueAny":
            out.append(shelf.dequeueAny())
        elif name == "dequeueDog":
            out.append(shelf.dequeueDog())
        else:
            out.append(shelf.dequeueCat())
    return out


def check(ops: List[tuple], want: list) -> None:
    got = run(ops)
    assert got == want, f"got {got}, want {want}"


# 对照实现：一条按入所顺序的线性表，删除时直接搬元素，用来验证最优解
class Reference:
    def __init__(self):
        self.all = []  # (编号, 种类)，下标即入所顺序

    def enqueue(self, animal: List[int]) -> None:
        self.all.append((animal[0], animal[1]))

    # kind 取 -1 表示任意种类
    def dequeue(self, kind: int) -> List[int]:
        for i, (animal_id, animal_type) in enumerate(self.all):
            if kind >= 0 and animal_type != kind:
                continue
            del self.all[i]
            return [animal_id, animal_type]
        return [-1, -1]


# 随机交叉验证：编号故意乱序，专门验证"按入所顺序"而不是"按编号"
def cross_check() -> None:
    shelf = AnimalShelf()
    ref = Reference()
    seed = 20240607
    for step in range(3000):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        if (seed >> 16) % 10 < 4:
            seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
            # 编号乱序：先入所的动物编号可能更大
            animal = [(seed >> 16) % 100, (seed >> 8) % 2]
            shelf.enqueue(animal)
            ref.enqueue(animal)
            continue

        kind = (seed >> 16) % 3
        if kind == 0:
            got, want = shelf.dequeueAny(), ref.dequeue(-1)
        elif kind == 1:
            got, want = shelf.dequeueDog(), ref.dequeue(1)
        else:
            got, want = shelf.dequeueCat(), ref.dequeue(0)
        assert got == want, f"第 {step} 步：got {got}, want {want}"


if __name__ == "__main__":
    # 示例 1：两只猫，dequeueDog 先返回空，再取走剩下的那只猫
    check([("enqueue", [0, 0]), ("enqueue", [1, 0]), ("dequeueCat", None),
           ("dequeueDog", None), ("dequeueAny", None)],
          [None, None, [0, 0], [-1, -1], [1, 0]])

    # 示例 2：先取狗不影响猫的先后顺序
    check([("enqueue", [0, 0]), ("enqueue", [1, 0]), ("enqueue", [2, 1]),
           ("dequeueDog", None), ("dequeueCat", None), ("dequeueAny", None)],
          [None, None, None, [2, 1], [0, 0], [1, 0]])

    # 边界：空所连续出队，全部返回 [-1,-1]
    check([("dequeueAny", None), ("dequeueDog", None), ("dequeueCat", None)],
          [[-1, -1], [-1, -1], [-1, -1]])

    # 反直觉：dequeueDog 必须跳过排在前面的猫，但不能把猫弄丢
    check([("enqueue", [0, 0]), ("enqueue", [1, 1]), ("enqueue", [2, 0]),
           ("enqueue", [3, 1]), ("dequeueDog", None), ("dequeueAny", None),
           ("dequeueAny", None), ("dequeueAny", None), ("dequeueAny", None)],
          [None, None, None, None, [1, 1], [0, 0], [2, 0], [3, 1], [-1, -1]])

    # 反直觉：编号不按入所顺序时，dequeueAny 看的是入所先后而不是编号
    check([("enqueue", [7, 1]), ("enqueue", [2, 0]), ("dequeueAny", None),
           ("dequeueCat", None)],
          [None, None, [7, 1], [2, 0]])

    # 边界：只有狗，猫的那一路始终返回 [-1,-1]
    check([("enqueue", [5, 1]), ("dequeueCat", None), ("dequeueAny", None),
           ("dequeueAny", None)],
          [None, [-1, -1], [5, 1], [-1, -1]])

    # 边界：取空之后还能继续入所并按新顺序出队
    check([("enqueue", [1, 0]), ("dequeueAny", None), ("enqueue", [2, 1]),
           ("enqueue", [3, 0]), ("dequeueAny", None), ("dequeueCat", None)],
          [None, [1, 0], None, None, [2, 1], [3, 0]])

    # 随机交叉验证
    cross_check()

    print("all tests passed")
