# LeetCode Solutions · A Study Roadmap

English | [简体中文](README.md)

> **For the full problem index, see the [Chinese README](README.md).** This page is an overview,
> not a translation: the index is maintained in one place only, so the two never drift apart.

## What this repository is

A staged study plan covering 240+ LeetCode problems, plus a separate chapter on hand-written
data structures and algorithms.

- Each solution is written against the official leetcode.cn editorial. When the editorial offers
  several approaches, **all of them are implemented and the optimal one is marked** — the notes
  after each row say why one wins on time, space, or both.
- Most problems ship in C++17, Go, Python 3 and Rust. A few rows (notably the Huawei OA problems)
  are C++ only, and the tables say so.
- Every solution file is self-contained: one file, one `main`, standard library only, no build
  system. Running it prints `all tests passed`.
- The `Algorithms/` chapter re-implements 31 foundational topics (sorting, linear structures,
  heaps, trees, graphs, a hash table) across 124 files, using no library containers and no
  library sorting. The point is to be able to write `sort` on a whiteboard, not to call it.

## The nine stages

Stages 1-6 are organised by **the structure that holds the data**. Stage 1 (1 week) is warm-up
programming and pattern spotting. Stage 2 (2 weeks) covers linked lists and stacks, including the
monotonic stack and monotonic queue. Stage 4 (2 weeks) pairs binary search with hash tables,
because both are "look something up faster than linear" tools. Stage 5 (2 weeks) is binary trees,
split into eight recurring patterns from traversal to rebuilding a tree out of its traversal
results. Stage 6 (1 week) is heaps and tries. Stage 3 (1 week) — recursion, divide and conquer,
sorting — sits in the middle on purpose: it is the bridge between the early structure chapters and
every recursive pattern that follows.

Stages 7-9 switch the axis to **the technique you apply**. Stage 7 (4 weeks) is graphs and search:
backtracking, DFS, topological sort, union-find and BFS, each as its own section because each has
a distinct correctness argument. Stage 8 (2 weeks) is dynamic programming, grouped by the *shape*
of the state — knapsack, linear DP, path DP, interval and sequence DP, state machine DP — rather
than by the story in the problem statement. Stage 9 (2 weeks) is linear scans and common tricks:
two pointers, sliding window, prefix/suffix statistics, bit manipulation, and greedy, the last of
which is included for its proofs rather than its templates.

Week counts are estimated rather than measured: easy counts 1 load unit, medium 2, hard 4, and a
week holds 30 units (about three medium problems a day), rounded up. The nine stages total about
17 weeks starting from zero; with prior experience that can halve, and at one hour a day it
roughly doubles.

## Running a single file

```bash
# C++
g++ -std=c++17 Problems/0001-0100/0001-two-sum/0001-two-sum.cpp -o /tmp/two-sum && /tmp/two-sum

# Go
go run Problems/0001-0100/0001-two-sum/0001-two-sum.go

# Python 3
python3 Problems/0001-0100/0001-two-sum/0001-two-sum.py

# Rust
rustc Problems/0001-0100/0001-two-sum/0001-two-sum.rs -o /tmp/two-sum && /tmp/two-sum
```

Each file ends by printing `all tests passed`; if it prints anything else, or panics, the
assertions inside caught something.

## Repository layout

- `Problems/<bucket>/<id>-<slug>/<id>-<slug>.<ext>` — the numeric problem set. `bucket` is a
  zero-padded id range such as `0001-0100`, and the file name repeats the id and the English slug,
  so a path is self-describing without the README. `Problems/TEMPLATE.md` is the skeleton new
  solutions are copied from.
- `JianZhiOffer/` — 35 problems from 剑指 Offer / LCR. **These file names are in Chinese and stay
  that way**, because they mirror the titles on leetcode.cn; renaming them would break both the
  index and the author's mapping back to the Chinese statement.
- `InterviewQuestions/` — Cracking the Coding Interview problems, named `<number>-<slug>-lcci`.
- `HuaweiJikao/` — Huawei online-assessment problems, one folder each (one folder name is Chinese
  for the same reason as above).
- `Others/` — one-off problems that do not belong to a set.
- `Algorithms/` — the hand-written chapter, split into `sort/`, `linear/`, `heap/`, `tree/`,
  `graph/` and `hash/`.
- `Examples/` — larger engineering examples (`BigData`, `BigDataFile`, `MultiWorkers`) with their
  own READMEs and Makefiles.
- `Docs/pdf/` — third-party PDF references, kept in Chinese.
- `README.md` — the complete index: every problem, its four language links, complexity notes and
  cross-references between sections.

If a link in the Chinese index points at a file whose name is Chinese, that is intentional, not a
translation gap: the repository keeps the platform's original titles as the single source of truth
and translates everything around them.

## License

The solutions and the hand-written implementations are released under the MIT License; see
[LICENSE](LICENSE). Problem statements quoted in file headers remain the property of LeetCode, and
the license covers only the code written in this repository.

Code under `Examples/` keeps its own license, as declared in each file header: Apache License 2.0
for `Examples/BigData/`, `Examples/BigDataFile/` and `Examples/MultiWorkers/` (except the two
`skiplist` files), and BSD 3-Clause for `Examples/BigData/skiplist.cpp` and `skiplist.h`, which
originate from Redis and are copyright Salvatore Sanfilippo and Pieter Noordhuis.
