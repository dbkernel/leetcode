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
use std::collections::VecDeque;

/*
说明：本题在 leetcode.cn 上没有官方题解文章，下面给出标准实现并说明取舍。
另一种常见写法是「一条总队列 + 按类型线性跳过最老的异类动物」，它让
dequeueDog / dequeueCat 最坏 O(n) 且需要额外的临时结构，所以不取。

解法：猫狗各一条队列 + 全局入所序号【最优解】

接口契约：
  · enqueue(animal)：animal = [编号, 种类]，0 是猫、1 是狗，返回 ()
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
  · 平台模板把方法写成 &self，改动内部状态需要 &mut self（模板注释也允许改）

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

// rustc 单文件编译时平台预置的 AnimalShelf 不存在，这里按平台模板显式定义
struct AnimalShelf {
    // 队列元素是 (入所序号, 动物编号)
    cats: VecDeque<(i32, i32)>,
    dogs: VecDeque<(i32, i32)>,
    seq: i32,
}

impl AnimalShelf {
    fn new() -> Self {
        AnimalShelf {
            cats: VecDeque::new(),
            dogs: VecDeque::new(),
            seq: 0,
        }
    }

    // 平台签名：入参是 Vec<i32>，animal[0] 编号、animal[1] 种类
    fn enqueue(&mut self, animal: Vec<i32>) {
        // 序号与编号分开存：编号不保证递增，不能用它判断谁更老
        let item = (self.seq, animal[0]);
        if animal[1] == 0 {
            self.cats.push_back(item);
        } else {
            self.dogs.push_back(item);
        }
        self.seq += 1;
    }

    fn dequeue_any(&mut self) -> Vec<i32> {
        // 两条都空才算没有动物；只有一类空时取另一类的最老
        match (self.cats.is_empty(), self.dogs.is_empty()) {
            (true, true) => vec![-1, -1],
            (false, true) => self.dequeue_cat(),
            (true, false) => self.dequeue_dog(),
            (false, false) => {
                // 序号小的入所更早，先被收养
                if self.cats[0].0 < self.dogs[0].0 {
                    self.dequeue_cat()
                } else {
                    self.dequeue_dog()
                }
            }
        }
    }

    fn dequeue_dog(&mut self) -> Vec<i32> {
        match self.dogs.pop_front() {
            Some((_, id)) => vec![id, 1],
            None => vec![-1, -1],
        }
    }

    fn dequeue_cat(&mut self) -> Vec<i32> {
        match self.cats.pop_front() {
            Some((_, id)) => vec![id, 0],
            None => vec![-1, -1],
        }
    }
}

// 按题目给的调用序列回放；void 方法在结果里留一个空 vector
fn run(ops: &[(&str, Vec<i32>)]) -> Vec<Vec<i32>> {
    let mut shelf = AnimalShelf::new();
    let mut out = Vec::with_capacity(ops.len());
    for (name, arg) in ops {
        match *name {
            "enqueue" => {
                shelf.enqueue(arg.clone());
                out.push(Vec::new());
            }
            "dequeueAny" => out.push(shelf.dequeue_any()),
            "dequeueDog" => out.push(shelf.dequeue_dog()),
            _ => out.push(shelf.dequeue_cat()),
        }
    }
    out
}

fn check(ops: &[(&str, Vec<i32>)], want: &[Vec<i32>]) {
    let got = run(ops);
    assert_eq!(got, want, "调用序列回放结果不符");
}

// 对照实现：一条按入所顺序的线性表，删除时直接搬元素，用来验证最优解
struct Reference {
    all: Vec<(i32, i32)>, // (编号, 种类)，下标即入所顺序
}

impl Reference {
    fn new() -> Self {
        Reference { all: Vec::new() }
    }

    fn enqueue(&mut self, animal: &[i32]) {
        self.all.push((animal[0], animal[1]));
    }

    // kind 取 -1 表示任意种类
    fn dequeue(&mut self, kind: i32) -> Vec<i32> {
        let mut found = None;
        for (i, &(_, animal_type)) in self.all.iter().enumerate() {
            if kind >= 0 && animal_type != kind {
                continue;
            }
            found = Some(i);
            break;
        }
        match found {
            Some(i) => {
                let (id, animal_type) = self.all.remove(i);
                vec![id, animal_type]
            }
            None => vec![-1, -1],
        }
    }
}

// 随机交叉验证：编号故意乱序，专门验证"按入所顺序"而不是"按编号"
fn cross_check() {
    let mut shelf = AnimalShelf::new();
    let mut reference = Reference::new();
    let mut seed: u32 = 20240607;
    for step in 0..3000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        if (seed >> 16) % 10 < 4 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            // 编号乱序：先入所的动物编号可能更大
            let animal = vec![
                ((seed >> 16) % 100) as i32,
                ((seed >> 8) % 2) as i32,
            ];
            shelf.enqueue(animal.clone());
            reference.enqueue(&animal);
            continue;
        }

        let (got, want) = match (seed >> 16) % 3 {
            0 => (shelf.dequeue_any(), reference.dequeue(-1)),
            1 => (shelf.dequeue_dog(), reference.dequeue(1)),
            _ => (shelf.dequeue_cat(), reference.dequeue(0)),
        };
        assert_eq!(got, want, "第 {} 步结果不符", step);
    }
}

fn main() {
    // 示例 1：两只猫，dequeueDog 先返回空，再取走剩下的那只猫
    check(
        &[
            ("enqueue", vec![0, 0]),
            ("enqueue", vec![1, 0]),
            ("dequeueCat", vec![]),
            ("dequeueDog", vec![]),
            ("dequeueAny", vec![]),
        ],
        &[
            vec![],
            vec![],
            vec![0, 0],
            vec![-1, -1],
            vec![1, 0],
        ],
    );

    // 示例 2：先取狗不影响猫的先后顺序
    check(
        &[
            ("enqueue", vec![0, 0]),
            ("enqueue", vec![1, 0]),
            ("enqueue", vec![2, 1]),
            ("dequeueDog", vec![]),
            ("dequeueCat", vec![]),
            ("dequeueAny", vec![]),
        ],
        &[
            vec![],
            vec![],
            vec![],
            vec![2, 1],
            vec![0, 0],
            vec![1, 0],
        ],
    );

    // 边界：空所连续出队，全部返回 [-1,-1]
    check(
        &[
            ("dequeueAny", vec![]),
            ("dequeueDog", vec![]),
            ("dequeueCat", vec![]),
        ],
        &[vec![-1, -1], vec![-1, -1], vec![-1, -1]],
    );

    // 反直觉：dequeueDog 必须跳过排在前面的猫，但不能把猫弄丢
    check(
        &[
            ("enqueue", vec![0, 0]),
            ("enqueue", vec![1, 1]),
            ("enqueue", vec![2, 0]),
            ("enqueue", vec![3, 1]),
            ("dequeueDog", vec![]),
            ("dequeueAny", vec![]),
            ("dequeueAny", vec![]),
            ("dequeueAny", vec![]),
            ("dequeueAny", vec![]),
        ],
        &[
            vec![],
            vec![],
            vec![],
            vec![],
            vec![1, 1],
            vec![0, 0],
            vec![2, 0],
            vec![3, 1],
            vec![-1, -1],
        ],
    );

    // 反直觉：编号不按入所顺序时，dequeueAny 看的是入所先后而不是编号
    check(
        &[
            ("enqueue", vec![7, 1]),
            ("enqueue", vec![2, 0]),
            ("dequeueAny", vec![]),
            ("dequeueCat", vec![]),
        ],
        &[vec![], vec![], vec![7, 1], vec![2, 0]],
    );

    // 边界：只有狗，猫的那一路始终返回 [-1,-1]
    check(
        &[
            ("enqueue", vec![5, 1]),
            ("dequeueCat", vec![]),
            ("dequeueAny", vec![]),
            ("dequeueAny", vec![]),
        ],
        &[vec![], vec![-1, -1], vec![5, 1], vec![-1, -1]],
    );

    // 边界：取空之后还能继续入所并按新顺序出队
    check(
        &[
            ("enqueue", vec![1, 0]),
            ("dequeueAny", vec![]),
            ("enqueue", vec![2, 1]),
            ("enqueue", vec![3, 0]),
            ("dequeueAny", vec![]),
            ("dequeueCat", vec![]),
        ],
        &[
            vec![],
            vec![1, 0],
            vec![],
            vec![],
            vec![2, 1],
            vec![3, 0],
        ],
    );

    // 随机交叉验证
    cross_check();

    println!("all tests passed");
}
