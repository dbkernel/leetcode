/*
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
*/
#include <cassert>
#include <iostream>
#include <queue>
#include <string>
#include <utility>
#include <vector>
using namespace std;

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出标准实现并说明取舍。
另一种常见写法是「一条总队列 + 按类型线性跳过最老的异类动物」，它让
dequeueDog / dequeueCat 最坏 O(n) 且需要额外的临时结构，所以不取。

解法：猫狗各一条队列 + 全局入所序号【最优解】

接口契约：
  · enqueue(animal)：animal = [编号, 种类]，0 是猫、1 是狗，返回 void
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
*/
class AnimalShelf
{
public:
    AnimalShelf() {}

    // 平台签名：入参是 vector<int>，animal[0] 编号、animal[1] 种类
    void enqueue(vector<int> animal)
    {
        // 序号与编号分开存：编号不保证递增，不能用它判断谁更老
        if (animal[1] == 0)
        {
            cats.push({seq, animal[0]});
        }
        else
        {
            dogs.push({seq, animal[0]});
        }
        ++seq;
    }

    vector<int> dequeueAny()
    {
        // 两条都空才算没有动物；只有一类空时取另一类的最老
        if (cats.empty() && dogs.empty()) return {-1, -1};
        if (dogs.empty()) return dequeueCat();
        if (cats.empty()) return dequeueDog();
        // 序号小的入所更早，先被收养
        return cats.front().first < dogs.front().first ? dequeueCat()
                                                       : dequeueDog();
    }

    vector<int> dequeueDog()
    {
        if (dogs.empty()) return {-1, -1};
        int id = dogs.front().second;
        dogs.pop();
        return {id, 1};
    }

    vector<int> dequeueCat()
    {
        if (cats.empty()) return {-1, -1};
        int id = cats.front().second;
        cats.pop();
        return {id, 0};
    }

private:
    // 队列元素是 (入所序号, 动物编号)
    queue<pair<int, int>> cats;
    queue<pair<int, int>> dogs;
    int seq = 0;
};

namespace
{
struct Op
{
    string name;
    vector<int> arg;
};

// 按题目给的调用序列回放；void 方法在结果里占一个空 vector
vector<vector<int>> run(const vector<Op> &ops)
{
    AnimalShelf shelf;
    vector<vector<int>> out;
    for (const auto &op : ops)
    {
        if (op.name == "enqueue")
        {
            shelf.enqueue(op.arg);
            out.push_back({});
        }
        else if (op.name == "dequeueAny")
        {
            out.push_back(shelf.dequeueAny());
        }
        else if (op.name == "dequeueDog")
        {
            out.push_back(shelf.dequeueDog());
        }
        else
        {
            out.push_back(shelf.dequeueCat());
        }
    }
    return out;
}

void check(const vector<Op> &ops, const vector<vector<int>> &want)
{
    vector<vector<int>> got = run(ops);
    assert(got == want);
}

// 对照实现：一条按入所顺序的线性表，删除时直接搬元素，用来验证最优解
struct Reference
{
    vector<pair<int, int>> all; // (编号, 种类)，下标即入所顺序

    void enqueue(const vector<int> &animal)
    {
        all.push_back({animal[0], animal[1]});
    }

    vector<int> dequeue(int type)
    {
        for (size_t i = 0; i < all.size(); ++i)
        {
            if (type >= 0 && all[i].second != type) continue;
            int id = all[i].first;
            int t = all[i].second;
            all.erase(all.begin() + static_cast<long>(i));
            return {id, t};
        }
        return {-1, -1};
    }
};

// 随机交叉验证：编号故意乱序，专门验证"按入所顺序"而不是"按编号"
void crossCheck()
{
    AnimalShelf shelf;
    Reference ref;
    unsigned seed = 20240607;
    for (int i = 0; i < 3000; ++i)
    {
        seed = seed * 1103515245 + 12345;
        int r = (seed >> 16) % 10;
        if (r < 4)
        {
            seed = seed * 1103515245 + 12345;
            // 编号乱序：先入所的动物编号可能更大
            vector<int> animal = {static_cast<int>((seed >> 16) % 100),
                                  static_cast<int>((seed >> 8) % 2)};
            shelf.enqueue(animal);
            ref.enqueue(animal);
        }
        else
        {
            vector<int> got;
            vector<int> want;
            int kind = static_cast<int>((seed >> 16) % 3);
            if (kind == 0)
            {
                got = shelf.dequeueAny();
                want = ref.dequeue(-1);
            }
            else if (kind == 1)
            {
                got = shelf.dequeueDog();
                want = ref.dequeue(1);
            }
            else
            {
                got = shelf.dequeueCat();
                want = ref.dequeue(0);
            }
            assert(got == want);
        }
    }
}
} // namespace

int main()
{
    // 示例 1：两只猫，dequeueDog 先返回空，再取走剩下的那只猫
    check({{"enqueue", {0, 0}},
           {"enqueue", {1, 0}},
           {"dequeueCat", {}},
           {"dequeueDog", {}},
           {"dequeueAny", {}}},
          {{}, {}, {0, 0}, {-1, -1}, {1, 0}});

    // 示例 2：先取狗不影响猫的先后顺序
    check({{"enqueue", {0, 0}},
           {"enqueue", {1, 0}},
           {"enqueue", {2, 1}},
           {"dequeueDog", {}},
           {"dequeueCat", {}},
           {"dequeueAny", {}}},
          {{}, {}, {}, {2, 1}, {0, 0}, {1, 0}});

    // 边界：空所连续出队，全部返回 [-1,-1]
    check({{"dequeueAny", {}}, {"dequeueDog", {}}, {"dequeueCat", {}}},
          {{-1, -1}, {-1, -1}, {-1, -1}});

    // 反直觉：dequeueDog 必须跳过排在前面的猫，但不能把猫弄丢
    check({{"enqueue", {0, 0}},
           {"enqueue", {1, 1}},
           {"enqueue", {2, 0}},
           {"enqueue", {3, 1}},
           {"dequeueDog", {}},
           {"dequeueAny", {}},
           {"dequeueAny", {}},
           {"dequeueAny", {}},
           {"dequeueAny", {}}},
          {{}, {}, {}, {}, {1, 1}, {0, 0}, {2, 0}, {3, 1}, {-1, -1}});

    // 反直觉：编号不按入所顺序时，dequeueAny 看的是入所先后而不是编号
    check({{"enqueue", {7, 1}}, {"enqueue", {2, 0}}, {"dequeueAny", {}},
           {"dequeueCat", {}}},
          {{}, {}, {7, 1}, {2, 0}});

    // 边界：只有狗，猫的那一路始终返回 [-1,-1]
    check({{"enqueue", {5, 1}},
           {"dequeueCat", {}},
           {"dequeueAny", {}},
           {"dequeueAny", {}}},
          {{}, {-1, -1}, {5, 1}, {-1, -1}});

    // 边界：取空之后还能继续入所并按新顺序出队
    check({{"enqueue", {1, 0}},
           {"dequeueAny", {}},
           {"enqueue", {2, 1}},
           {"enqueue", {3, 0}},
           {"dequeueAny", {}},
           {"dequeueCat", {}}},
          {{}, {1, 0}, {}, {}, {2, 1}, {3, 0}});

    // 随机交叉验证
    crossCheck();

    cout << "all tests passed" << endl;
    return 0;
}
