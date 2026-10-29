---
name: git-commit-msg
description: |
  本仓库（LeetCode 题解集）的 commit message 规范。基于 Conventional Commits，
  并按本仓库历史提交习惯固定了 scope 词表与 subject 写法。
  当准备提交题解、更新 README 目录、修改题解模板或做批量整理时使用；
  涉及 git commit、写提交信息、拆分提交、review 待提交内容时也应触发。
---

# Git Commit Message 规范（本仓库）

本文件是本仓库的提交规范，也是唯一标准。与通用 Conventional Commits 文档冲突时，
以本文件为准；本文件未覆盖的部分按通用规范处理。

---

## 一句话规则

```text
<type>(<scope>): <subject>
```

`type` 必填且为下列之一；`scope` 取自[固定词表](#scope-词表)；`subject` 用英文祈使句，
首字母小写，结尾不加句号。

```text
feat(Problems): add 0001-two-sum.cpp
docs(README): add Rust link for 0001-two-sum
chore(*): unify leetcode.cn domain across 45 files
```

---

## 与历史提交的关系

本仓库历史提交多为 `Problems: add 0026-...cpp` 的形式，即「目录名 + 冒号 + add + 文件名」，
没有 type 前缀。为保持 `git log` 的可检索性，本规范**保留历史 scope 拼写**
（`Problems`、`JianZhiOffer` 等按下表原样书写，不改成小写，也不改成连字符形式），
只补上 type 前缀，并把 subject 的动词固定为 `add` / `update` / `fix` / `remove`。

历史提交不回改。新提交一律按本规范执行。

---

## type 词表

| type | 本仓库中的判定标准 | 示例 |
| --- | --- | --- |
| `feat` | 新增题解或为已有题目新增一门语言实现，仓库内容变多 | `feat(Problems): add 0001-two-sum.go` |
| `fix` | 题解代码本身有错、行内注释与 API 行为不符、链接失效、复杂度标注错误 | `fix(Problems): correct overflow handling in 0007-reverse-integer.cpp` |
| `docs` | 只动 README、模板、规范、题面文字或注释文字，代码行为不变 | `docs(Problems): add solution template` |
| `style` | 只动空行、缩进、对齐、行宽，逻辑与文案都不变 | `style(Problems): reformat 0020-valid-parentheses.cpp` |
| `refactor` | 同一题换写法，行为等价且无性能目的 | `refactor(JianZhiOffer): rewrite 09-用两个栈实现队列.cpp with std::stack` |
| `perf` | 改动的直接目的是降复杂度 | `perf(Problems): optimize 0015-3sum from O(n^3) to O(n^2)` |
| `test` | 只增删 `main` 里的自测用例或断言 | `test(Problems): add duplicate-value case for 0001-two-sum` |
| `chore` | 构建配置、`.vscode`、依赖、批量机械替换等与题解内容无关的整理 | `chore(*): unify leetcode.cn domain across 45 files` |
| `revert` | 回滚某次提交 | `revert: feat(Problems): add 0001-two-sum.rs` |

判断要点：

- **新增一门语言的实现是 `feat` 还是 `docs`**：新增可运行代码 → `feat`；
  只补 README 里的链接 → `docs`。两者通常分属两个 commit。
- **改注释是 `docs` 还是 `fix`**：注释说的与代码实际行为不一致 → `fix`（它会误导读者）；
  只是措辞更清楚 → `docs`。
- **`chore(*): ...` 是最后的退路**：只有当改动跨多个目录且无法归属单一 scope 时才用，
  且必须是一次机械操作（域名替换、重命名），不能在里夹带任何解题内容。

---

## scope 词表

scope 必须是下列值之一，拼写照抄，不要自创、不要改成小写：

| scope | 对应目录 |
| --- | --- |
| `Problems` | `Problems/` 下所有 LeetCode 官方题号 |
| `JianZhiOffer` | `JianZhiOffer/` 剑指 Offer 系列 |
| `InterviewQuestions` | `InterviewQuestions/` 程序员面试金典系列 |
| `HuaweiJikao` | `HuaweiJikao/` 华为机考题目 |
| `Algorithms` | `Algorithms/` 算法专题实现 |
| `Examples` | `Examples/` 综合示例 |
| `Others` | `Others/` 零散题目 |
| `Docs` | `Docs/` 阶段性文档与 pdf |
| `Guidelines` | 模板与开发规范：`Problems/TEMPLATE.md`、`.cursor/skills/` 下的 skill |
| `README` | 根目录 `README.md` |
| `Config` | `.vscode/`、`.gitignore` 等工程配置 |
| `*` | 跨目录的机械操作，仅 `chore` 使用 |

一次提交跨越两个 scope（如同时改 `Problems` 和 `README`）时，按下面的「拆分规则」拆开，
不要写成 `Problems+README`。

---

## subject 写法

固定为「动词 + 对象」，动词只用这四个：

| 动词 | 用法 | 对象形式 |
| --- | --- | --- |
| `add` | 新增文件或新增条目 | 文件名带扩展名，如 `0001-two-sum.cpp`；或 `Rust link for 0001-two-sum` |
| `update` | 修改已有内容 | `README.md`、`solution template`、`complexity analysis for ...` |
| `fix` | 修正错误 | 错误现象，如 `overflow handling in 0007-reverse-integer.cpp` |
| `remove` | 删除 | 被删对象，如 `unused include in 0020-valid-parentheses.cpp` |

规则：

- 英文，动词开头，首字母小写（`add` 不写 `Add`）
- 一行写完，不超过 72 字符
- 不写「修改」「更新了」这类模糊词，也不写成「add 代码」
- 文件名保留仓库中的真实拼写，包括中文文件名：
  `feat(JianZhiOffer): add 67-把字符串转换成整数.cpp`

---

## 正文（可选）

只在需要解释「为什么」或列出清单时使用，与 subject 之间空一行。

```text
feat(Problems): add Go, Python and Rust solutions for 0001-two-sum

三种语言均按 leetcode.cn 官方题解的哈希表写法实现，
沿用 0001-two-sum.cpp 的五段式注释结构。
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

本仓库常见的拆分粒度：

| 场景 | 拆分方式 |
| --- | --- |
| 新增一道题的多语言解 | 一个 `feat(Problems)`，包含该题所有语言文件；README 链接另起 `docs(README)` |
| 用模板重写一道旧题 | 一个 `fix` 或 `refactor`，只含该题目录下的文件 |
| 引入模板或规范 | 一个 `docs`，只含模板/规范文件本身，不带任何题解改动 |
| 批量替换（域名、路径、命名） | 一个 `chore(*): ...`，只含机械替换，不带语义改动 |
| 同一道题出现在 README 多处 | 合并到一个 `docs(README)`，不要按阶段拆成多次提交 |

---

## 工作流程

1. `git status` + `git diff` 看清变更范围，按「拆分规则」列出分组
2. 每组确定 type / scope / subject，组装 message
3. 逐组 `git add <精确路径>`（不用 `git add -A`，避免误带文件）
4. 提交前核对下方的自检清单
5. `git commit`，然后在输出中展示 `git log --oneline -n` 的结果

---

## 自检清单

- [ ] type 在词表内，scope 拼写与词表一致
- [ ] subject 为英文、动词开头、首字母小写、无句号、≤ 72 字符
- [ ] 一个 commit 只做一件事，回滚时不需要挑选文件
- [ ] 已确认 message 中没有 `Co-authored-by` 等任何协助者署名
- [ ] 代码类提交已编译并跑通自测
- [ ] `git status` 中没有遗留在暂存区的无关文件

提交后用 `git log -1 --pretty=%B` 再看一眼：部分 IDE / 提交工具会自动追加
`Co-authored-by` 之类的 trailer，命令行里看不到，只有落到 message 里才暴露。
一旦发现，用 `git commit --amend -m` 重写（尚未推送时）或
`git filter-branch --msg-filter` 批量清除（已产生多次提交时）。

---

## 示例

本次模板化改造的拆分结果：

```text
docs(Guidelines): add solution template and commit message convention
chore(*): unify leetcode.cn domain across 44 files
feat(Problems): add Go, Python and Rust solutions for 0001-two-sum
docs(README): add Rust link for 0001-two-sum
```

前四条按「规范 / 批量替换 / 题解」分开；README 的链接更新若与题解同批产生，
可并入题解那条 `feat` 的正文说明，但文件仍需在同一个 commit 内。

