---
name: git-commit-msg
description: |
  本仓库（LeetCode 题解集）的 commit message 规范。题解类提交以「平台(题号)」开头，
  非题解类提交沿用 Conventional Commits。
  当准备提交题解、更新 README 目录、修改题解模板或做批量整理时使用；
  涉及 git commit、写提交信息、拆分提交、review 待提交内容时也应触发。
---

# Git Commit Message 规范（本仓库）

本文件是本仓库的提交规范，也是唯一标准。

---

## 一句话规则

仓库里的提交分两类，格式不同：

```text
<平台>(<题号>): <动词> <对象>        ← 题解类提交（写代码、写注释、换写法）
<type>(<scope>): <subject>           ← 非题解类提交（README、模板、配置、批量整理）
```

```text
LeetCode(0001): add solutions in C++, Go and Python
LeetCode(0015): optimize from O(n^3) to O(n^2)
JianZhiOffer(58-I): fix off-by-one in the loop
docs(README): add Rust link for 0001-two-sum
chore(*): unify leetcode.cn domain across 43 files
```

---

## 为什么要和 Conventional Commits 分开

按 Conventional Commits，题解类提交只能写成 `feat(Problems): add 0001-two-sum.cpp`。
在这个仓库里这是最差的选择，因为：

- **scope 恒定无用**：`Problems` 占掉了 scope 位，而 95% 的提交 scope 都是它，
  等于没有 scope。真正有价值的定位信息是**题号**。
- **type 区分不成立**：新增一门语言是 `feat`，改注释是 `docs`，断言写错是 `fix` ——
  这三件事在读者眼里是同一件事（「动了 0001 这道题」），却要在三个 type 之间反复判断，
  判断结果对 `git log --oneline` 的可读性毫无贡献。
- **检索方向反了**：查历史时人想按题号找（「0001 这题我改过几次」），
  而不是按 type 找（「把所有 feat 列出来」）。

所以题解类提交把「平台 + 题号」放到最前面，改动的性质交给动词表达。
非题解类提交（说明书、配置、批量替换）没有题号可言，继续用 Conventional Commits。

**注意**：本规范不兼容 `commitlint` 的 `type-enum` 默认值。若将来引入 commitlint，
需要把 `LeetCode`、`JianZhiOffer`、`InterviewQuestions`、`HuaweiJikao`、`Others`
加进允许的 type 列表，否则会被判为非法 type。

---

## 平台与题号

平台名与目录一一对应，拼写照抄，不要自创：

| 平台名 | 对应目录 | 题号写法 | 示例 |
| --- | --- | --- | --- |
| `LeetCode` | `Problems/` | 4 位补零，与目录名一致 | `LeetCode(0001)`、`LeetCode(1502)` |
| `JianZhiOffer` | `JianZhiOffer/` | 与文件名中的题号一致，含罗马数字后缀 | `JianZhiOffer(10-2)`、`JianZhiOffer(58-I)`、`JianZhiOffer(05)` |
| `InterviewQuestions` | `InterviewQuestions/` | 面试金典的点分题号 | `InterviewQuestions(03.05)`、`InterviewQuestions(17.14)` |
| `HuaweiJikao` | `HuaweiJikao/` | `HJ` + 编号；无编号时用目录 slug | `HuaweiJikao(HJ16)`、`HuaweiJikao(taxi-order-scheduling-system)` |
| `Others` | `Others/` | 无官方题号，**省略括号** | `Others: add get_preorder_by_middle_back_order.cpp` |

规则：

- **`LeetCode` 的题号必须补零到 4 位**。`LeetCode(1)` 与 `LeetCode(0001)` 混用会让
  `git log --grep 'LeetCode(0001)'` 漏掉一半结果，也会让按题号排序失效。
- 题号只写数字，不写题目 slug：`LeetCode(0001)`，不是 `LeetCode(two-sum)`。
  slug 和文件名放在 subject 里。
- 无官方题号时不硬造编号，省略括号即可（`Others: ...`）。

---

## 动词

题解类与非题解类共用同一套动词，英文祈使句、首字母小写、结尾不加句号：

| 动词 | 用法 | 示例 |
| --- | --- | --- |
| `add` | 新增文件、新增一门语言、新增条目 | `LeetCode(0001): add solutions in C++, Go and Python` |
| `update` | 修改已有内容，未改变行为 | `LeetCode(0002): update problem statement to the new wording` |
| `fix` | 修正错误（代码错、注释与行为不符、链接失效、复杂度标错） | `LeetCode(0007): fix overflow in the reversal loop` |
| `remove` | 删除 | `LeetCode(0160): remove redundant idea comment` |
| `rewrite` | 换写法，行为等价 | `LeetCode(0001): rewrite Java solution with hash map` |
| `rename` | 纯改名 / 纯移动 | `refactor(Problems): rename Leetcode directory to Problems` |
| `optimize` | 改动目的就是降复杂度 | `LeetCode(0015): optimize from O(n^3) to O(n^2)` |

前四个是常用词。后三个只在需要区分「改法」时使用——`update` 太笼统时，
`rewrite` 说明换写法、`optimize` 说明为性能而改，二者比 `refactor` 更直白。

subject 里允许出现中文（剑指 Offer 的题名本身就是中文），但动词保持英文：

```text
JianZhiOffer(67): add 把字符串转换成整数
JianZhiOffer(21): add 调整数组顺序使奇数位于偶数前面
```

---

## 非题解类提交的 type 与 scope

| type | 判定标准 | 示例 |
| --- | --- | --- |
| `docs` | 只动 README、模板、规范、pdf 索引，代码行为不变 | `docs(README): rebuild problem tables with status columns` |
| `chore` | 构建配置、`.vscode`、`.gitignore`、依赖、批量机械替换 | `chore(Config): add VS Code build and debug configuration` |
| `refactor` | 目录重构、批量改名等结构整理，不带解题内容 | `refactor(Problems): rename Leetcode directory to Problems` |
| `style` | 只动空行、缩进、对齐、行宽 | `style(Problems): reformat 0020-valid-parentheses.cpp` |
| `test` | 只增删 `main` 里的自测用例或断言 | `test(LeetCode-0001): add duplicate-value case` 见下注 |
| `revert` | 回滚某次提交 | `revert: LeetCode(0001): add solutions in C++, Go and Python` |

scope 词表（只在这些值里选，拼写照抄）：

| scope | 对应路径 |
| --- | --- |
| `README` | 根目录 `README.md` |
| `Guidelines` | `Problems/TEMPLATE.md`、`.agent/skills/` |
| `Config` | `.vscode/`、`.gitignore` |
| `Docs` | `Docs/` |
| `Examples` | `Examples/` |
| `Algorithms` | `Algorithms/` |
| `Problems` | `Problems/` 目录本身（结构重构、批量替换） |
| `*` | 跨多个目录的机械操作，仅 `chore` 与 `refactor` 使用 |

**注**：只改某道题的自测用例，仍属题解类提交，写成
`LeetCode(0001): add duplicate-value test case`，不要为了用 `test` 而退回
`test(Problems)` —— 题号比 type 更有检索价值。

### 一条提交同时动题解和 README 怎么办

拆成两条：`LeetCode(0001): ...` 之后再跟一条 `docs(README): ...`。
README 的链接更新量大、变更频繁，混进题解提交会让那道题的 diff 被目录表淹没。

---

## 多题 / 多平台提交

一次提交覆盖多个题号时，把题号列进括号；跨平台时用 `*`：

```text
LeetCode(0002,0020,0225): add three problems from stage 1
```

```text
*: add 0841-keys-and-rooms and HJ16-购物单
```

括号内的题号按升序排列，用逗号分隔，不加空格。跨平台的提交必须在正文里列出
每个平台对应的文件，否则事后无法从 subject 还原范围。

---

## 正文（可选）

只在需要解释「为什么」或列出清单时使用，与 subject 之间空一行。

```text
LeetCode(0001): add solutions in C++, Go, Python and Rust

四种语言均按 leetcode.cn 官方题解的哈希表写法实现，统一为五段式注释结构；
C++ 版按模板重写，修正 insert/emplace 与 operator[] 语义混淆的说明。
```

正文用中文撰写（与本仓库注释语言一致），每行不超过 72 字符。

---

## 禁止事项

### 1. 不标注联合开发者

commit message 中禁止出现任何形式的协助者、联合开发者或生成工具署名，包括：

- footer 行：`Co-authored-by:`、`Signed-off-by:`、`Reviewed-by:`、`Assisted-by:`、`Generated-by:`、`Made-with:`
- 正文表述：「由 XX 协助完成」「与 XX 联合开发」「由 AI 生成」「Generated with ...」

提交只体现改动本身；作者信息由 git 的 author 字段承载，协作过程留在 PR 或 issue 里。

### 2. 不把多件事塞进一个 commit

下面这些都必须拆开：

- 新增题解 + 更新 README 目录
- 修改题解代码 + 批量替换域名
- 新增模板/规范 + 用新模板重写某道题
- 修 bug + 顺手格式化整个文件

### 3. 不提交半成品

- 代码必须能编译、能跑通自测（见 `Problems/TEMPLATE.md` 的自检清单）
- 不留 `TODO`、调试输出、注释掉的旧代码
- 提交前 `git status` 确认没有误加的临时文件

---

## 拆分规则

一个 commit 对应一个「可以用一句话说清、且回滚时希望整体回滚」的单元。

| 场景 | 拆分方式 |
| --- | --- |
| 新增一道题的多语言解 | 一个 `LeetCode(<题号>)`，包含该题所有语言文件 |
| 同一道题补一门新语言 | 同上，另起一条 `LeetCode(<题号>): add Rust solution` |
| 用模板重写一道旧题 | 一个 `LeetCode(<题号>): rewrite ...`，只含该题目录下的文件 |
| 引入模板或规范 | 一个 `docs(Guidelines)`，不带任何题解改动 |
| 批量替换（域名、路径、命名） | 一个 `chore(*): ...` 或 `refactor(...)`，只含机械替换 |
| README 目录更新 | 一个 `docs(README)`，无论涉及几道题 |

---

## 工作流程

1. `git status` + `git diff` 看清变更范围，按「拆分规则」列出分组
2. 每组确定平台 / 题号 / 动词（或 type / scope / subject）
3. 逐组 `git add <精确路径>`（不用 `git add -A`，避免误带文件）
4. 提交前核对下方的自检清单
5. `git commit`，然后在输出中展示 `git log --oneline -n` 的结果

---

## 自检清单

- [ ] 题解类提交以 `LeetCode(` / `JianZhiOffer(` / `InterviewQuestions(` / `HuaweiJikao(` 开头
- [ ] `LeetCode` 题号为 4 位补零，与目录名一致
- [ ] 多题号写成 `(0002,0020,0225)`，升序、逗号分隔、无空格
- [ ] 非题解类提交的 type 与 scope 在词表内
- [ ] subject 动词开头、小写、无句号、≤ 80 字符
- [ ] 一个 commit 只做一件事，回滚时不需要挑选文件
- [ ] 已确认 message 中没有 `Co-authored-by` 等任何协助者署名
- [ ] 代码类提交已编译并跑通自测
- [ ] `git status` 中没有遗留在暂存区的无关文件

提交后用 `git log -1 --pretty=%B` 再看一眼：部分 IDE / 提交工具会自动追加
`Co-authored-by` 之类的 trailer，命令行里看不到，只有落到 message 里才暴露。
一旦发现，用 `git commit --amend -m` 重写（尚未推送时）或
`git filter-branch --msg-filter` 批量清除（已产生多次提交时）。

---

## 历史提交的处理

历史提交已统一按本规范重写（76 条）。早期提交的原貌保留在
`backup/pre-msg-normalize` 分支中，可随时对比：

```bash
git log --oneline backup/pre-msg-normalize    # 重写前的原始信息
git log --oneline master                      # 规范化之后
```

---

## 示例

本次模板化改造的拆分结果：

```text
docs(Guidelines): add solution template and commit message convention
chore(*): unify leetcode.cn domain across 43 files
LeetCode(0001): add solutions in C++, Go, Python and Rust
```

三条按「规范 / 批量替换 / 题解」分开。README 里新增的 Rust 链接若与题解同批产生，
另起一条 `docs(README): add Rust link for 0001-two-sum`，不要并进题解提交。
