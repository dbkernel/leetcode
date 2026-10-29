# Table of Contents

- [Table of Contents](#table-of-contents)
- [第一阶段（1 周）](#第一阶段1-周)
  - [纯编程](#纯编程)
  - [找规律](#找规律)
- [第二阶段（1 周）](#第二阶段1-周)
  - [链表](#链表)
  - [栈](#栈)
- [第三阶段（1 周）](#第三阶段1-周)
  - [递归 + 分治](#递归--分治)
  - [排序](#排序)
- [第四阶段（1 周）](#第四阶段1-周)
  - [二分查找](#二分查找)
  - [哈希表](#哈希表)
- [第五阶段（2 周）](#第五阶段2-周)
  - [二叉树](#二叉树)
    - [题型 1：二叉树前中后序遍历](#题型-1二叉树前中后序遍历)
    - [题型 2：二叉树按层遍历](#题型-2二叉树按层遍历)
    - [题型 3：二叉树上的递归](#题型-3二叉树上的递归)
    - [题型 4：二叉查找树](#题型-4二叉查找树)
    - [题型 5：LCA 最近公共祖先](#题型-5lca-最近公共祖先)
    - [题型 6：二叉树转单、双、循环链表](#题型-6二叉树转单双循环链表)
    - [题型 7：按照遍历结果反向构建二叉树](#题型-7按照遍历结果反向构建二叉树)
    - [题型 8：二叉树上的最长路径和](#题型-8二叉树上的最长路径和)
- [第六阶段（1 周）](#第六阶段1-周)
  - [堆](#堆)
  - [Trie 树](#trie-树)
- [第七阶段（1 周）](#第七阶段1-周)
  - [回溯](#回溯)
  - [DFS](#dfs)
- [第八阶段（1 周）](#第八阶段1-周)
  - [背包：](#背包)
  - [打家劫舍：](#打家劫舍)
  - [买卖股票：](#买卖股票)
- [第九阶段（1 周）](#第九阶段1-周)
  - [双指针](#双指针)
  - [滑动窗口](#滑动窗口)
  - [前缀后缀统计](#前缀后缀统计)
  - [位运算](#位运算)
- [综合示例](#综合示例)
- [其他](#其他)

# 第一阶段（1 周）

> [详细速览版](Docs/stage01.md)

## 纯编程

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [1.两数之和（简单，虾皮社招）](https://leetcode.cn/problems/two-sum/) | [cpp](Problems/0001-0100/0001-two-sum/0001-two-sum.cpp) | [go](Problems/0001-0100/0001-two-sum/0001-two-sum.go) | [py](Problems/0001-0100/0001-two-sum/0001-two-sum.py) | [rs](Problems/0001-0100/0001-two-sum/0001-two-sum.rs) | 官方两解：暴力枚举 O(n^2) / 哈希表 O(n)；最优解为哈希表 |
| ✅ | [1108. IP 地址无效化（简单）](https://leetcode.cn/problems/defanging-an-ip-address/) | [cpp](Problems/1101-1200/1108-defanging-an-ip-address/1108-defanging-an-ip-address.cpp) | [go](Problems/1101-1200/1108-defanging-an-ip-address/1108-defanging-an-ip-address.go) | [py](Problems/1101-1200/1108-defanging-an-ip-address/1108-defanging-an-ip-address.py) | [rs](Problems/1101-1200/1108-defanging-an-ip-address/1108-defanging-an-ip-address.rs) | 官方仅一解：一次遍历替换 O(n)，即最优解 |
| ✅ | [344. 反转字符串（简单）](https://leetcode.cn/problems/reverse-string/) | [cpp](Problems/0301-0400/0344-reverse-string/0344-reverse-string.cpp) | [go](Problems/0301-0400/0344-reverse-string/0344-reverse-string.go) | [py](Problems/0301-0400/0344-reverse-string/0344-reverse-string.py) | [rs](Problems/0301-0400/0344-reverse-string/0344-reverse-string.rs) | 官方仅一解：双指针原地交换 O(n)，即最优解 |
| ✅ | [LCR 181 / 剑指 Offer 58 - I. 字符串中的单词反转（简单）](https://leetcode.cn/problems/fan-zhuan-dan-ci-shun-xu-lcof/) | [cpp](JianZhiOffer/58-I-翻转单词顺序.cpp) | [go](JianZhiOffer/58-I-翻转单词顺序.go) | [py](JianZhiOffer/58-I-翻转单词顺序.py) | [rs](JianZhiOffer/58-I-翻转单词顺序.rs) | 官方两解：倒序双指针 / 分割后倒序；最优解为双指针（不依赖内置切分函数） |
| ✅ | [125. 验证回文串（简单）](https://leetcode.cn/problems/valid-palindrome/) | [cpp](Problems/0101-0200/0125-valid-palindrome/0125-valid-palindrome.cpp) | [go](Problems/0101-0200/0125-valid-palindrome/0125-valid-palindrome.go) | [py](Problems/0101-0200/0125-valid-palindrome/0125-valid-palindrome.py) | [rs](Problems/0101-0200/0125-valid-palindrome/0125-valid-palindrome.rs) | 官方两解：筛选后判断 O(n) 空间 / 原串双指针 O(1) 空间；最优解为原串双指针 |
| ✅ | [9. 回文数（简单）](https://leetcode.cn/problems/palindrome-number/) | [cpp](Problems/0001-0100/0009-palindrome-number/0009-palindrome-number.cpp) | [go](Problems/0001-0100/0009-palindrome-number/0009-palindrome-number.go) | [py](Problems/0001-0100/0009-palindrome-number/0009-palindrome-number.py) | [rs](Problems/0001-0100/0009-palindrome-number/0009-palindrome-number.rs) | 官方仅一解：反转一半数字 O(log n)，即最优解；规避整体反转溢出 |
| ✅ | [58. 最后一个单词的长度（简单）](https://leetcode.cn/problems/length-of-last-word/) | [cpp](Problems/0001-0100/0058-length-of-last-word/0058-length-of-last-word.cpp) | [go](Problems/0001-0100/0058-length-of-last-word/0058-length-of-last-word.go) | [py](Problems/0001-0100/0058-length-of-last-word/0058-length-of-last-word.py) | [rs](Problems/0001-0100/0058-length-of-last-word/0058-length-of-last-word.rs) | 官方仅一解：反向遍历 O(n)，即最优解；先跳尾空格再数 |
| ✅ | [LCR 122 / 剑指 Offer 05. 路径加密（简单）](https://leetcode.cn/problems/ti-huan-kong-ge-lcof/) | [cpp](JianZhiOffer/05-替换空格.cpp) | [go](JianZhiOffer/05-替换空格.go) | [py](JianZhiOffer/05-替换空格.py) | [rs](JianZhiOffer/05-替换空格.rs) | 官方仅一解：逐字符遍历替换 O(n)，即最优解（原题「替换空格」现改名「路径加密」，把 `.` 换成空格） |
| ✅ | [LCR 182 / 剑指 Offer 58 - II. 动态口令（简单）](https://leetcode.cn/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/) | [cpp](JianZhiOffer/58-II-左旋转字符串.cpp) | [go](JianZhiOffer/58-II-左旋转字符串.go) | [py](JianZhiOffer/58-II-左旋转字符串.py) | [rs](JianZhiOffer/58-II-左旋转字符串.rs) | 官方四解：切片 / 列表拼接 / 字符串拼接 / 三次翻转；最优解为三次翻转 O(1) 空间 |
| ✅ | [26. 删除排序数组中的重复项（简单）](https://leetcode.cn/problems/remove-duplicates-from-sorted-array/) | [cpp](Problems/0001-0100/0026-remove-duplicates-from-sorted-array/0026-remove-duplicates-from-sorted-array.cpp) | [go](Problems/0001-0100/0026-remove-duplicates-from-sorted-array/0026-remove-duplicates-from-sorted-array.go) | [py](Problems/0001-0100/0026-remove-duplicates-from-sorted-array/0026-remove-duplicates-from-sorted-array.py) | [rs](Problems/0001-0100/0026-remove-duplicates-from-sorted-array/0026-remove-duplicates-from-sorted-array.rs) | 官方仅一解：快慢双指针原地去重 O(n)，即最优解 |
| ✅ | [LCR 192 / 剑指 Offer 67. 把字符串转换成整数 (atoi)（中等）](https://leetcode.cn/problems/ba-zi-fu-chuan-zhuan-huan-cheng-zheng-shu-lcof/) | [cpp](JianZhiOffer/67-把字符串转换成整数.cpp) | [go](JianZhiOffer/67-把字符串转换成整数.go) | [py](JianZhiOffer/67-把字符串转换成整数.py) | [rs](JianZhiOffer/67-把字符串转换成整数.rs) | 官方仅一解：逐位扫描 + 拼接前越界预判 O(n)，即最优解；注意只跳前导空格与符号位必须紧跟数字 |

## 找规律

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [面试题 01.08. 零矩阵 （简单）](https://leetcode.cn/problems/zero-matrix-lcci/) |  |  |  |  |  |
| ☐ | [剑指 Offer 61. 扑克牌中的顺子(中等)](https://leetcode.cn/problems/bu-ke-pai-zhong-de-shun-zi-lcof/) |  |  |  |  |  |
| ☐ | [面试题 16.11. 跳水板（简单）](https://leetcode.cn/problems/diving-board-lcci/) |  |  |  |  |  |
| ☐ | [面试题 01.05. 一次编辑（中等）](https://leetcode.cn/problems/one-away-lcci/) |  |  |  |  |  |
| ☐ | [面试题 16.15. 珠玑妙算 （中等）](https://leetcode.cn/problems/master-mind-lcci/) |  |  |  |  |  |
| ☐ | [面试题 16.04. 井字游戏（中等）](https://leetcode.cn/problems/tic-tac-toe-lcci/) |  |  |  |  |  |
| ☐ | [55. 跳跃游戏 （中等）](https://leetcode.cn/problems/jump-game/) |  |  |  |  |  |
| ☐ | [48. 旋转图像 （中等）](https://leetcode.cn/problems/rotate-image/) |  |  |  |  |  |
| ☐ | [54. 螺旋矩阵（中等，字节电商 22 届暑期实习，出处）](https://leetcode.cn/problems/spiral-matrix/) |  |  |  |  |  |
| ☐ | [240. 搜索二维矩阵 II （中等）](https://leetcode.cn/problems/search-a-2d-matrix-ii/) |  |  |  |  |  |

# 第二阶段（1 周）

## 链表

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [203. 移除链表元素 （简单）](https://leetcode.cn/problems/remove-linked-list-elements/) | [cpp](Problems/0201-0300/0203-remove-linked-list-elements/0203-remove-linked-list-elements.cpp) | [go](Problems/0201-0300/0203-remove-linked-list-elements/0203-remove-linked-list-elements.go) | [py](Problems/0201-0300/0203-remove-linked-list-elements/0203-remove-linked-list-elements.py) | [rs](Problems/0201-0300/0203-remove-linked-list-elements/0203-remove-linked-list-elements.rs) | 官方两解：递归 O(n) 栈 / 迭代 O(1)；最优解为迭代 + 哨兵 |
| ✅ | [876. 链表的中间结点（简单）](https://leetcode.cn/problems/middle-of-the-linked-list/) | [cpp](Problems/0801-0900/0876-middle-of-the-linked-list/0876-middle-of-the-linked-list.cpp) | [go](Problems/0801-0900/0876-middle-of-the-linked-list/0876-middle-of-the-linked-list.go) | [py](Problems/0801-0900/0876-middle-of-the-linked-list/0876-middle-of-the-linked-list.py) | [rs](Problems/0801-0900/0876-middle-of-the-linked-list/0876-middle-of-the-linked-list.rs) | 官方三解：数组 O(n) 空间 / 单指针两趟 / 快慢指针一趟；最优解为快慢指针 |
| ✅ | [83. 删除排序链表中的重复元素（简单）](https://leetcode.cn/problems/remove-duplicates-from-sorted-list/) | [cpp](Problems/0001-0100/0083-remove-duplicates-from-sorted-list/0083-remove-duplicates-from-sorted-list.cpp) | [go](Problems/0001-0100/0083-remove-duplicates-from-sorted-list/0083-remove-duplicates-from-sorted-list.go) | [py](Problems/0001-0100/0083-remove-duplicates-from-sorted-list/0083-remove-duplicates-from-sorted-list.py) | [rs](Problems/0001-0100/0083-remove-duplicates-from-sorted-list/0083-remove-duplicates-from-sorted-list.rs) | 官方仅一解：一次遍历去重 O(n)，即最优解；删除时当前节点不动 |
| ✅ | [2. 两数相加 （中等，腾讯笔试题）](https://leetcode.cn/problems/add-two-numbers/) | [cpp](Problems/0001-0100/0002-two-numbers/0002-two-numbers.cpp) | [go](Problems/0001-0100/0002-two-numbers/0002-two-numbers.go) | [py](Problems/0001-0100/0002-two-numbers/0002-two-numbers.py) | [rs](Problems/0001-0100/0002-two-numbers/0002-two-numbers.rs) | 官方仅一解：逐位模拟 O(max(m,n))，即最优解；注意进位与末尾补位 |
| ✅ | [LCR 142 / 剑指 Offer 25. 训练计划 IV （简单）](https://leetcode.cn/problems/he-bing-liang-ge-pai-xu-de-lian-biao-lcof/) | [cpp](JianZhiOffer/25-合并两个排序的链表.cpp) | [go](JianZhiOffer/25-合并两个排序的链表.go) | [py](JianZhiOffer/25-合并两个排序的链表.py) | [rs](JianZhiOffer/25-合并两个排序的链表.rs) | 官方两解：递归 O(n+m) 栈空间 / 迭代 O(1) 空间；最优解为迭代 + 哨兵（原题「合并两个排序的链表」） |
| ☐ | [369. 给单链表加一（中等）力扣会员](https://leetcode.cn/problems/plus-one-linked-list/) |  |  |  |  |  |

> **题目：** 比如链表[6,8,1,9]，加 1 后，为[6,8,2,0]
>
> **思路：** 先翻转链表为[9,1,8,6]，就可以对 9 加 1，之后根据按照从左往右的顺序进位，直到无位可进，之后再次翻转链表。

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [LCR 140 / 剑指 Offer 22. 训练计划 II （简单）](https://leetcode.cn/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/) | [cpp](JianZhiOffer/22-链表中倒数第k个节点.cpp) | [go](JianZhiOffer/22-链表中倒数第k个节点.go) | [py](JianZhiOffer/22-链表中倒数第k个节点.py) | [rs](JianZhiOffer/22-链表中倒数第k个节点.rs) | 官方两解：先数长度再定位 / 快慢双指针一趟；最优解为双指针（原题「链表中倒数第 k 个节点」） |
| ✅ | [19. 删除链表的倒数第 N 个结点 （中等）](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/) | [cpp](Problems/0001-0100/0019-remove-nth-node-from-end-of-list/0019-remove-nth-node-from-end-of-list.cpp) | [go](Problems/0001-0100/0019-remove-nth-node-from-end-of-list/0019-remove-nth-node-from-end-of-list.go) | [py](Problems/0001-0100/0019-remove-nth-node-from-end-of-list/0019-remove-nth-node-from-end-of-list.py) | [rs](Problems/0001-0100/0019-remove-nth-node-from-end-of-list/0019-remove-nth-node-from-end-of-list.rs) | 官方三解：算长度 O(L) 两趟 / 栈 O(L) 空间 / 快慢指针一趟；最优解为快慢指针 |
| ✅ | [160. 相交链表（简单，新华智云笔试题）](https://leetcode.cn/problems/intersection-of-two-linked-lists/) | [cpp](Problems/0101-0200/0160-intersection-of-two-linked-lists/0160-intersection-of-two-linked-lists.cpp) | [go](Problems/0101-0200/0160-intersection-of-two-linked-lists/0160-intersection-of-two-linked-lists.go) | [py](Problems/0101-0200/0160-intersection-of-two-linked-lists/0160-intersection-of-two-linked-lists.py) | [rs](Problems/0101-0200/0160-intersection-of-two-linked-lists/0160-intersection-of-two-linked-lists.rs) | 官方两解：哈希集合 O(m) 空间 / 双指针走对方的路 O(1) 空间；最优解为双指针（Rust 版平台暂无模板，按裸指针建模） |
| ✅ | [141. 环形链表（简单） 判断链表中是否存在环 ](https://leetcode.cn/problems/linked-list-cycle/) | [cpp](Problems/0101-0200/0141-linked-list-cycle/0141-linked-list-cycle.cpp) | [go](Problems/0101-0200/0141-linked-list-cycle/0141-linked-list-cycle.go) | [py](Problems/0101-0200/0141-linked-list-cycle/0141-linked-list-cycle.py) | [rs](Problems/0101-0200/0141-linked-list-cycle/0141-linked-list-cycle.rs) | 官方两解：哈希表 O(n) 空间 / Floyd 快慢指针 O(1) 空间；最优解为快慢指针（Rust 版平台暂无模板，按裸指针建模） |
| ✅ | [206. 反转链表 （中等，阿里云笔试题）](https://leetcode.cn/problems/reverse-linked-list/) | [cpp](Problems/0201-0300/0206-reverse-linked-list/0206-reverse-linked-list.cpp) | [go](Problems/0201-0300/0206-reverse-linked-list/0206-reverse-linked-list.go) | [py](Problems/0201-0300/0206-reverse-linked-list/0206-reverse-linked-list.py) | [rs](Problems/0201-0300/0206-reverse-linked-list/0206-reverse-linked-list.rs) | 官方两解：迭代 O(1) 空间 / 递归 O(n) 栈空间；最优解为迭代 |
| ✅ | [234. 回文链表 （中等，蚂蚁金服笔试题）](https://leetcode.cn/problems/palindrome-linked-list/) | [cpp](Problems/0201-0300/0234-palindrome-linked-list/0234-palindrome-linked-list.cpp) | [go](Problems/0201-0300/0234-palindrome-linked-list/0234-palindrome-linked-list.go) | [py](Problems/0201-0300/0234-palindrome-linked-list/0234-palindrome-linked-list.py) | [rs](Problems/0201-0300/0234-palindrome-linked-list/0234-palindrome-linked-list.rs) | 官方三解：复制到数组 / 递归 / 快慢指针+反转后半；最优解为后者 O(1) 空间 |
| ✅ | [328. 奇偶链表（中等，腾讯笔试题](https://leetcode.cn/problems/odd-even-linked-list/) | [cpp](Problems/0301-0400/0328-odd-even-linked-list/0328-odd-even-linked-list.cpp) | [go](Problems/0301-0400/0328-odd-even-linked-list/0328-odd-even-linked-list.go) | [py](Problems/0301-0400/0328-odd-even-linked-list/0328-odd-even-linked-list.py) | [rs](Problems/0301-0400/0328-odd-even-linked-list/0328-odd-even-linked-list.rs) | 官方仅一解：分离奇偶两条链再拼接 O(n)、O(1)，即最优解 |
| ✅ | [25. K 个一组翻转链表（困难，腾讯笔试题）](https://leetcode.cn/problems/reverse-nodes-in-k-group/) | [cpp](Problems/0001-0100/0025-reverse-nodes-in-k-group/0025-reverse-nodes-in-k-group.cpp) | [go](Problems/0001-0100/0025-reverse-nodes-in-k-group/0025-reverse-nodes-in-k-group.go) | [py](Problems/0001-0100/0025-reverse-nodes-in-k-group/0025-reverse-nodes-in-k-group.py) | [rs](Problems/0001-0100/0025-reverse-nodes-in-k-group/0025-reverse-nodes-in-k-group.rs) | 官方仅一解：哑节点 + 按组反转再拼接 O(n)、O(1)，即最优解 |

## 栈

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [LCR 125 / 剑指 Offer 09. 图书整理 II（简单）](https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/) | [cpp](JianZhiOffer/09-用两个栈实现队列.cpp) | [go](JianZhiOffer/09-用两个栈实现队列.go) | [py](JianZhiOffer/09-用两个栈实现队列.py) | [rs](JianZhiOffer/09-用两个栈实现队列.rs) | 官方仅一解：双栈模拟队列，均摊 O(1) 出队、O(n) 空间，即最优解；输出栈非空时不能倒栈 |
| ✅ | [225. 用队列实现栈（简单，字节实习，出处）](https://leetcode.cn/problems/implement-stack-using-queues/) | [cpp](Problems/0201-0300/0225-implement-stack-using-queues/0225-implement-stack-using-queues.cpp) | [go](Problems/0201-0300/0225-implement-stack-using-queues/0225-implement-stack-using-queues.go) | [py](Problems/0201-0300/0225-implement-stack-using-queues/0225-implement-stack-using-queues.py) | [rs](Problems/0201-0300/0225-implement-stack-using-queues/0225-implement-stack-using-queues.rs) | 官方两解：两个队列 / 一个队列（进阶）；两者入栈均 O(n)、出栈 O(1)，最优解为一个队列，空间常数更小 |
| ✅ | [面试题 03.01. 三合一（简单）](https://leetcode.cn/problems/three-in-one-lcci/) | [cpp](InterviewQuestions/03.01-three-in-one-lcci.cpp) | [go](InterviewQuestions/03.01-three-in-one-lcci.go) | [py](InterviewQuestions/03.01-three-in-one-lcci.py) | [rs](InterviewQuestions/03.01-three-in-one-lcci.rs) | 平台无官方题解；按金典标准做法给两种实现：三个独立数组（不满足题意）/ 一个数组分三段 + 三个栈顶指针，均为 O(1) 操作，最优解为后者 |
| ✅ | [20. 有效的括号（简单）](https://leetcode.cn/problems/valid-parentheses/) | [cpp](Problems/0001-0100/0020-valid-parentheses/0020-valid-parentheses.cpp) | [go](Problems/0001-0100/0020-valid-parentheses/0020-valid-parentheses.go) | [py](Problems/0001-0100/0020-valid-parentheses/0020-valid-parentheses.py) | [rs](Problems/0001-0100/0020-valid-parentheses/0020-valid-parentheses.rs) | 官方仅一解：栈匹配 O(n)，即最优解；注意栈空与遍历后非空两种边界 |
| ✅ | [面试题 16.26. 计算器（中等）](https://leetcode.cn/problems/calculator-lcci/) | [cpp](InterviewQuestions/16.26-calculator-lcci.cpp) | [go](InterviewQuestions/16.26-calculator-lcci.go) | [py](InterviewQuestions/16.26-calculator-lcci.py) | [rs](InterviewQuestions/16.26-calculator-lcci.rs) | 官方仅一解：栈 + preSign，乘除就地结算、加减入栈最后求和 O(n)，即最优解；注意除法向零截断 |
| ☐ | [772. 基本计算器 III（困难 包含括号）力扣会员](https://leetcode.cn/problems/basic-calculator-iii/) |  |  |  |  |  |
| ✅ | [1047. 删除字符串中的所有相邻重复项（简单）](https://leetcode.cn/problems/remove-all-adjacent-duplicates-in-string/) | [cpp](Problems/1001-1100/1047-remove-all-adjacent-duplicates-in-string/1047-remove-all-adjacent-duplicates-in-string.cpp) | [go](Problems/1001-1100/1047-remove-all-adjacent-duplicates-in-string/1047-remove-all-adjacent-duplicates-in-string.go) | [py](Problems/1001-1100/1047-remove-all-adjacent-duplicates-in-string/1047-remove-all-adjacent-duplicates-in-string.py) | [rs](Problems/1001-1100/1047-remove-all-adjacent-duplicates-in-string/1047-remove-all-adjacent-duplicates-in-string.rs) | 官方仅一解：栈消除 O(n)，即最优解；结果串直接当栈用 |
| ✅ | [面试题 03.05. 栈排序（中等）](https://leetcode.cn/problems/sort-of-stacks-lcci/) | [cpp](InterviewQuestions/03.05-sort-of-stacks-lcci.cpp) | [go](InterviewQuestions/03.05-sort-of-stacks-lcci.go) | [py](InterviewQuestions/03.05-sort-of-stacks-lcci.py) | [rs](InterviewQuestions/03.05-sort-of-stacks-lcci.rs) | 平台无官方题解；给两种实现（均只用一个临时栈）：push 时插入排序（O(n) 入栈、O(1) 取数）/ 取数时惰性排序，最优解为前者 |
| ✅ | [155. 最小栈（简单）](https://leetcode.cn/problems/min-stack/) | [cpp](Problems/0101-0200/0155-min-stack/0155-min-stack.cpp) | [go](Problems/0101-0200/0155-min-stack/0155-min-stack.go) | [py](Problems/0101-0200/0155-min-stack/0155-min-stack.py) | [rs](Problems/0101-0200/0155-min-stack/0155-min-stack.rs) | 官方仅一解：辅助栈与数据栈同步记录前缀最小值，四操作均 O(1)，即最优解 |
| ✅ | [LCR 148 / 剑指 Offer 31. 验证图书取出顺序（中等）](https://leetcode.cn/problems/zhan-de-ya-ru-dan-chu-xu-lie-lcof/) | [cpp](JianZhiOffer/31-栈的压入、弹出序列.cpp) | [go](JianZhiOffer/31-栈的压入、弹出序列.go) | [py](JianZhiOffer/31-栈的压入、弹出序列.py) | [rs](JianZhiOffer/31-栈的压入、弹出序列.rs) | 平台无官方题解（同主站 946）；标准做法为辅助栈模拟：边入栈边弹出栈顶，最终栈空即合法，O(n) |

**附加题：**

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [739. 每日温度（中等）单调栈](https://leetcode.cn/problems/daily-temperatures/) |  |  |  |  |  |
| ☐ | [42. 接雨水（困难）单调栈](https://leetcode.cn/problems/trapping-rain-water/) |  |  |  |  |  |
| ☐ | [84. 柱状图中最大的矩形（困难）单调栈](https://leetcode.cn/problems/largest-rectangle-in-histogram/) |  |  |  |  |  |
| ☐ | [面试题 03.06. 动物收容所（中等） 队列](https://leetcode.cn/problems/animal-shelter-lcci/) |  |  |  |  |  |
| ☐ | [剑指 Offer 59 - II. 队列的最大值（中等） 单调队列](https://leetcode.cn/problems/dui-lie-de-zui-da-zhi-lcof/) |  |  |  |  |  |
| ☐ | [剑指 Offer 59 - I. 滑动窗口的最大值 （困难）单调队列](https://leetcode.cn/problems/hua-dong-chuang-kou-de-zui-da-zhi-lcof/) |  |  |  |  |  |

# 第三阶段（1 周）

## 递归 + 分治

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [LCR 126 / 剑指 Offer 10- I. 斐波那契数（简单）](https://leetcode.cn/problems/fei-bo-na-qi-shu-lie-lcof/) | [cpp](JianZhiOffer/10-1-斐波那契数列.cpp) | [go](JianZhiOffer/10-1-斐波那契数列.go) | [py](JianZhiOffer/10-1-斐波那契数列.py) | [rs](JianZhiOffer/10-1-斐波那契数列.rs) | 官方仅一解：动态规划 + 滚动数组，O(n) 时间、O(1) 空间，即最优解；每步取模 1e9+7 |
| ✅ | [LCR 127 / 剑指 Offer 10- II. 跳跃训练（简单）](https://leetcode.cn/problems/qing-wa-tiao-tai-jie-wen-ti-lcof/) | [cpp](JianZhiOffer/10-2-青蛙跳台阶问题.cpp) | [go](JianZhiOffer/10-2-青蛙跳台阶问题.go) | [py](JianZhiOffer/10-2-青蛙跳台阶问题.py) | [rs](JianZhiOffer/10-2-青蛙跳台阶问题.rs) | 平台无官方题解（同主站 70）；标准做法为动态规划 + 滚动数组 O(n)、O(1)，注意 f(0)=1 的边界 |
| ☐ | [面试题 08.01. 三步问题 （简单）](https://leetcode.cn/problems/three-steps-problem-lcci/) |  |  |  |  |  |
| ☐ | [剑指 Offer 06. 从尾到头打印链表 （简单）](https://leetcode.cn/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/) |  |  |  |  |  |
| ☐ | [剑指 Offer 24. 反转链表 （简单）](https://leetcode.cn/problems/fan-zhuan-lian-biao-lcof/) |  |  |  |  |  |
| ☐ | [剑指 Offer 16. 数值的整数次方 （中等）](https://leetcode.cn/problems/shu-zhi-de-zheng-shu-ci-fang-lcof/) |  |  |  |  |  |
| ☐ | [面试题 08.05. 递归乘法 （中等）](https://leetcode.cn/problems/recursive-mulitply-lcci/) |  |  |  |  |  |

## 排序

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [面试题 10.01. 合并排序的数组（简单）](https://leetcode.cn/problems/sorted-merge-lcci/) | [cpp](InterviewQuestions/10.01-sorted-merge-lcci.cpp) | [go](InterviewQuestions/10.01-sorted-merge-lcci.go) | [py](InterviewQuestions/10.01-sorted-merge-lcci.py) | [rs](InterviewQuestions/10.01-sorted-merge-lcci.rs) | 官方三解：拼接后排序 / 双指针 + 临时数组 / 逆向双指针原地归并；最优解为逆向双指针 O(m+n)、O(1) |
| ✅ | [21. 合并两个有序链表 （简单）](https://leetcode.cn/problems/merge-two-sorted-lists/) | [25-合并两个排序的链表.cpp](JianZhiOffer/25-合并两个排序的链表.cpp)<br>[0021-merge-two-sorted-lists.cpp](Problems/0001-0100/0021-merge-two-sorted-lists/0021-merge-two-sorted-lists.cpp) | [go](Problems/0001-0100/0021-merge-two-sorted-lists/0021-merge-two-sorted-lists.go) | [py](Problems/0001-0100/0021-merge-two-sorted-lists/0021-merge-two-sorted-lists.py) | [rs](Problems/0001-0100/0021-merge-two-sorted-lists/0021-merge-two-sorted-lists.rs) | 官方两解：递归 O(n+m) 栈空间 / 迭代 O(1) 空间；最优解为迭代 + 哨兵 |
| ✅ | [242. 有效的字母异位词（简单）](https://leetcode.cn/problems/valid-anagram/) | [cpp](Problems/0201-0300/0242-valid-anagram/0242-valid-anagram.cpp) | [go](Problems/0201-0300/0242-valid-anagram/0242-valid-anagram.go) | [py](Problems/0201-0300/0242-valid-anagram/0242-valid-anagram.py) | [rs](Problems/0201-0300/0242-valid-anagram/0242-valid-anagram.rs) | 官方两解：排序后比较 O(n log n) / 频次数组计数 O(n)；最优解为频次数组（含 unicode 时换哈希表） |
| ✅ | [1502. 判断能否形成等差数列（简单）](https://leetcode.cn/problems/can-make-arithmetic-progression-from-sequence/) | [cpp](Problems/1501-1600/1502-can-make-arithmetic-progression-from-sequence/1502-can-make-arithmetic-progression-from-sequence.cpp) | [go](Problems/1501-1600/1502-can-make-arithmetic-progression-from-sequence/1502-can-make-arithmetic-progression-from-sequence.go) | [py](Problems/1501-1600/1502-can-make-arithmetic-progression-from-sequence/1502-can-make-arithmetic-progression-from-sequence.py) | [rs](Problems/1501-1600/1502-can-make-arithmetic-progression-from-sequence/1502-can-make-arithmetic-progression-from-sequence.rs) | 官方一解：排序后检查相邻公差是否一致，O(n log n) |
| ☐ | [252. 会议室 （简单）力扣会员](https://leetcode.cn/problems/meeting-rooms/) |  |  |  |  |  |
| ✅ | [56. 合并区间（中等） ](https://leetcode.cn/problems/merge-intervals/) | [cpp](Problems/0001-0100/0056-merge-intervals/0056-merge-intervals.cpp) | [go](Problems/0001-0100/0056-merge-intervals/0056-merge-intervals.go) | [py](Problems/0001-0100/0056-merge-intervals/0056-merge-intervals.py) | [rs](Problems/0001-0100/0056-merge-intervals/0056-merge-intervals.rs) | 官方一解：按左端点排序后一次扫描贪心合并，O(n log n) |
| ✅ | [剑指 Offer 21. 调整数组顺序使奇数位于偶数前面 （简单）](https://leetcode.cn/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof/) | [cpp](JianZhiOffer/21-调整数组顺序使奇数位于偶数前面.cpp) | [go](JianZhiOffer/21-调整数组顺序使奇数位于偶数前面.go) | [py](JianZhiOffer/21-调整数组顺序使奇数位于偶数前面.py) | [rs](JianZhiOffer/21-调整数组顺序使奇数位于偶数前面.rs) | 现收录为 LCR 139「训练计划 I」；官方一解：首尾双指针原地交换，O(n) 时间 O(1) 空间；不保证稳定性 |
| ✅ | [75. 颜色分类（中等）](https://leetcode.cn/problems/sort-colors/) | [cpp](Problems/0001-0100/0075-sort-colors/0075-sort-colors.cpp) | [go](Problems/0001-0100/0075-sort-colors/0075-sort-colors.go) | [py](Problems/0001-0100/0075-sort-colors/0075-sort-colors.py) | [rs](Problems/0001-0100/0075-sort-colors/0075-sort-colors.rs) | 荷兰国旗问题。官方三解：单指针两趟 / 双指针 p0+p1 / 双指针 p0+p2 一趟扫描；最优解为 p0+p2 一趟扫描，O(n) 时间 O(1) 空间 |
| ☐ | [147. 对链表进行插入排序（中等）](https://leetcode.cn/problems/insertion-sort-list/) |  |  |  |  |  |
| ☐ | [148. 排序链表（中等） 链表上的归并排序](https://leetcode.cn/problems/sort-list/) |  |  |  |  |  |
| ✅ | [215. 数组中的第 K 个最大元素（中等） ](https://leetcode.cn/problems/kth-largest-element-in-an-array/) | [cpp](Problems/0201-0300/0215-kth-largest-element-in-an-array/0215-kth-largest-element-in-an-array.cpp) | [go](Problems/0201-0300/0215-kth-largest-element-in-an-array/0215-kth-largest-element-in-an-array.go) | [py](Problems/0201-0300/0215-kth-largest-element-in-an-array/0215-kth-largest-element-in-an-array.py) | [rs](Problems/0201-0300/0215-kth-largest-element-in-an-array/0215-kth-largest-element-in-an-array.rs) | 官方两解：快速选择（期望 O(n)）/ 大根堆（O(n log n)）；最优解为快速选择；【精品】 |
| ✅ | [面试题 17.14. 最小 K 个数 （中等）](https://leetcode.cn/problems/smallest-k-lcci/) | [cpp](InterviewQuestions/17.14-smallest-k-lcci.cpp) | [go](InterviewQuestions/17.14-smallest-k-lcci.go) | [py](InterviewQuestions/17.14-smallest-k-lcci.py) | [rs](InterviewQuestions/17.14-smallest-k-lcci.rs) | 官方三解：排序 O(n log n) / 大小 k 的大根堆 O(n log k) / 随机化快速选择期望 O(n)；最优解为快速选择；【精品】 |
| ☐ | [剑指 Offer 51. 数组中的逆序对（困难）](https://leetcode.cn/problems/shu-zu-zhong-de-ni-xu-dui-lcof/) |  |  |  |  |  |

- [ ] **补充各类排序算法：**
  - [x] 快速排序
    - [quick_sort.cpp](Algorithms/sort/quick_sort.cpp)
  - [ ] 归并排序
  - [ ] 冒泡排序
  - [ ] 插入排序
  - [ ] 堆排序
  - [ ] 希尔排序
  - [ ] ......

# 第四阶段（1 周）

## 二分查找

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [704. 二分查找（简单） 标准二分查找](https://leetcode.cn/problems/binary-search/) | [cpp](Problems/0701-0800/0704-binary-search/0704-binary-search.cpp) | [go](Problems/0701-0800/0704-binary-search/0704-binary-search.go) | [py](Problems/0701-0800/0704-binary-search/0704-binary-search.py) | [rs](Problems/0701-0800/0704-binary-search/0704-binary-search.rs) | 官方一解：闭区间二分，O(log n) 时间 O(1) 空间 |
| ✅ | [374. 猜数字大小（简单）](https://leetcode.cn/problems/guess-number-higher-or-lower/) | [cpp](Problems/0301-0400/0374-guess-number-higher-or-lower/0374-guess-number-higher-or-lower.cpp) | [go](Problems/0301-0400/0374-guess-number-higher-or-lower/0374-guess-number-higher-or-lower.go) | [py](Problems/0301-0400/0374-guess-number-higher-or-lower/0374-guess-number-higher-or-lower.py) | [rs](Problems/0301-0400/0374-guess-number-higher-or-lower/0374-guess-number-higher-or-lower.rs) | 交互题。官方一解：二分并把区间收敛到一点，O(log n)；mid 需写成 left+(right-left)/2 防溢出 |
| ✅ | [744. 寻找比目标字母大的最小字母（简单）](https://leetcode.cn/problems/find-smallest-letter-greater-than-target/) | [cpp](Problems/0701-0800/0744-find-smallest-letter-greater-than-target/0744-find-smallest-letter-greater-than-target.cpp) | [go](Problems/0701-0800/0744-find-smallest-letter-greater-than-target/0744-find-smallest-letter-greater-than-target.go) | [py](Problems/0701-0800/0744-find-smallest-letter-greater-than-target/0744-find-smallest-letter-greater-than-target.py) | [rs](Problems/0701-0800/0744-find-smallest-letter-greater-than-target/0744-find-smallest-letter-greater-than-target.rs) | 官方仅一解：二分找第一个大于 target 的位置，越界时取模回绕到首字母；O(log n) 时间，即最优解 |
| ✅ | [35. 搜索插入位置（简单）](https://leetcode.cn/problems/search-insert-position/) | [cpp](Problems/0001-0100/0035-search-insert-position/0035-search-insert-position.cpp) | [go](Problems/0001-0100/0035-search-insert-position/0035-search-insert-position.go) | [py](Problems/0001-0100/0035-search-insert-position/0035-search-insert-position.py) | [rs](Problems/0001-0100/0035-search-insert-position/0035-search-insert-position.rs) | 官方仅一解：左闭右开二分返回 left，即最优解；答案可等于 n，即插到末尾 |
| ✅ | [34. 在排序数组中查找元素的第一个和最后一个位置 （中等）](https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/) | [cpp](Problems/0001-0100/0034-find-first-and-last-position-of-element-in-sorted-array/0034-find-first-and-last-position-of-element-in-sorted-array.cpp) | [go](Problems/0001-0100/0034-find-first-and-last-position-of-element-in-sorted-array/0034-find-first-and-last-position-of-element-in-sorted-array.go) | [py](Problems/0001-0100/0034-find-first-and-last-position-of-element-in-sorted-array/0034-find-first-and-last-position-of-element-in-sorted-array.py) | [rs](Problems/0001-0100/0034-find-first-and-last-position-of-element-in-sorted-array/0034-find-first-and-last-position-of-element-in-sorted-array.rs) | 官方两解：命中后向两侧线性扩展 / 两次二分分别找左右边界；最优解为两次二分，O(log n) 时间；注意空数组与全重复 |
| ✅ | [面试题 10.05. 稀疏数组搜索（简单） ](https://leetcode.cn/problems/sparse-array-search-lcci/) | [cpp](InterviewQuestions/10.05-sparse-array-search-lcci.cpp) | [go](InterviewQuestions/10.05-sparse-array-search-lcci.go) | [py](InterviewQuestions/10.05-sparse-array-search-lcci.py) | [rs](InterviewQuestions/10.05-sparse-array-search-lcci.rs) | 官方仅一解：二分，中点为空格时向右探测或缩右界；最坏 O(n)（全空串），即最优解 |
| ✅ | [33. 搜索旋转排序数组（中等）无重复数据 ](https://leetcode.cn/problems/search-in-rotated-sorted-array/) | [cpp](Problems/0001-0100/0033-search-in-rotated-sorted-array/0033-search-in-rotated-sorted-array.cpp) | [go](Problems/0001-0100/0033-search-in-rotated-sorted-array/0033-search-in-rotated-sorted-array.go) | [py](Problems/0001-0100/0033-search-in-rotated-sorted-array/0033-search-in-rotated-sorted-array.py) | [rs](Problems/0001-0100/0033-search-in-rotated-sorted-array/0033-search-in-rotated-sorted-array.rs) | 官方一解：一次二分先判哪半有序再决定收缩方向；O(log n) 时间，即最优解（另附「先找旋转点再二分」的同阶对照） |
| ✅ | [81. 搜索旋转排序数组 II（中等）有重复数据](https://leetcode.cn/problems/search-in-rotated-sorted-array-ii/) | [cpp](Problems/0001-0100/0081-search-in-rotated-sorted-array-ii/0081-search-in-rotated-sorted-array-ii.cpp) | [go](Problems/0001-0100/0081-search-in-rotated-sorted-array-ii/0081-search-in-rotated-sorted-array-ii.go) | [py](Problems/0001-0100/0081-search-in-rotated-sorted-array-ii/0081-search-in-rotated-sorted-array-ii.py) | [rs](Problems/0001-0100/0081-search-in-rotated-sorted-array-ii/0081-search-in-rotated-sorted-array-ii.rs) | 官方仅一解：二分，nums[left]==nums[mid] 判不出有序侧时 left++ 缩小；全相等时最坏 O(n)，即最优解 |
| ✅ | [153. 寻找旋转排序数组中的最小值（中等） 无重复数据](https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/) | [cpp](Problems/0101-0200/0153-find-minimum-in-rotated-sorted-array/0153-find-minimum-in-rotated-sorted-array.cpp) | [go](Problems/0101-0200/0153-find-minimum-in-rotated-sorted-array/0153-find-minimum-in-rotated-sorted-array.go) | [py](Problems/0101-0200/0153-find-minimum-in-rotated-sorted-array/0153-find-minimum-in-rotated-sorted-array.py) | [rs](Problems/0101-0200/0153-find-minimum-in-rotated-sorted-array/0153-find-minimum-in-rotated-sorted-array.rs) | 官方仅一解：mid 与右端点比较收缩 left<right，收敛到一点后返回 nums[left]；O(log n) 时间，即最优解 |
| ✅ | [154. 寻找旋转排序数组中的最小值 II （困难） 有重复数据](https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array-ii/) | [cpp](Problems/0101-0200/0154-find-minimum-in-rotated-sorted-array-ii/0154-find-minimum-in-rotated-sorted-array-ii.cpp) | [go](Problems/0101-0200/0154-find-minimum-in-rotated-sorted-array-ii/0154-find-minimum-in-rotated-sorted-array-ii.go) | [py](Problems/0101-0200/0154-find-minimum-in-rotated-sorted-array-ii/0154-find-minimum-in-rotated-sorted-array-ii.py) | [rs](Problems/0101-0200/0154-find-minimum-in-rotated-sorted-array-ii/0154-find-minimum-in-rotated-sorted-array-ii.rs) | 官方仅一解：相等时只 right--，不能二分；全相等时最坏 O(n)，即最优解 |
| ✅ | [852. 山脉数组的峰顶索引（简单）峰值二分](https://leetcode.cn/problems/peak-index-in-a-mountain-array/) | [cpp](Problems/0801-0900/0852-peak-index-in-a-mountain-array/0852-peak-index-in-a-mountain-array.cpp) | [go](Problems/0801-0900/0852-peak-index-in-a-mountain-array/0852-peak-index-in-a-mountain-array.go) | [py](Problems/0801-0900/0852-peak-index-in-a-mountain-array/0852-peak-index-in-a-mountain-array.py) | [rs](Problems/0801-0900/0852-peak-index-in-a-mountain-array/0852-peak-index-in-a-mountain-array.rs) | 官方两解：线性扫描 O(n) / 二分比较 arr[mid] 与 arr[mid+1]；最优解为二分，O(log n)，右端取 n-2 |
| ✅ | [162. 寻找峰值（中等）峰值二分](https://leetcode.cn/problems/find-peak-element/) | [cpp](Problems/0101-0200/0162-find-peak-element/0162-find-peak-element.cpp) | [go](Problems/0101-0200/0162-find-peak-element/0162-find-peak-element.go) | [py](Problems/0101-0200/0162-find-peak-element/0162-find-peak-element.py) | [rs](Problems/0101-0200/0162-find-peak-element/0162-find-peak-element.rs) | 官方仅一解：二分向更高的一侧收缩，端点视为外侧 -∞ 所以一定有峰；O(log n) 时间，即最优解（另附迭代爬坡的 O(n) 对照） |
| ✅ | [367. 有效的完全平方数（简单）二分答案](https://leetcode.cn/problems/valid-perfect-square/) | [cpp](Problems/0301-0400/0367-valid-perfect-square/0367-valid-perfect-square.cpp) | [go](Problems/0301-0400/0367-valid-perfect-square/0367-valid-perfect-square.go) | [py](Problems/0301-0400/0367-valid-perfect-square/0367-valid-perfect-square.py) | [rs](Problems/0301-0400/0367-valid-perfect-square/0367-valid-perfect-square.rs) | 官方三解：二分答案 / 牛顿迭代 / 内置 sqrt；最优解为二分与牛顿并列；注意 46340^2 与 2^31-1 的溢出 |
| ✅ | [69. x 的平方根（简单）二分答案](https://leetcode.cn/problems/sqrtx/) | [cpp](Problems/0001-0100/0069-sqrtx/0069-sqrtx.cpp) | [go](Problems/0001-0100/0069-sqrtx/0069-sqrtx.go) | [py](Problems/0001-0100/0069-sqrtx/0069-sqrtx.py) | [rs](Problems/0001-0100/0069-sqrtx/0069-sqrtx.rs) | 官方三解：二分 / 牛顿迭代 / 内置 exp-log；最优解为牛顿迭代；注意 x=0 不能除、mid*mid 用 64 位 |
| ✅ | [875. 爱吃香蕉的珂珂（中等）二分答案](https://leetcode.cn/problems/koko-eating-bananas/) | [cpp](Problems/0801-0900/0875-koko-eating-bananas/0875-koko-eating-bananas.cpp) | [go](Problems/0801-0900/0875-koko-eating-bananas/0875-koko-eating-bananas.go) | [py](Problems/0801-0900/0875-koko-eating-bananas/0875-koko-eating-bananas.py) | [rs](Problems/0801-0900/0875-koko-eating-bananas/0875-koko-eating-bananas.rs) | 官方仅一解：二分答案，下界 1、上界 max(piles)，耗时按 (p+k-1)/k 上取整；O(n log max) 即最优解 |
| ✅ | [658. 找到 K 个最接近的元素（中等）](https://leetcode.cn/problems/find-k-closest-elements/) | [cpp](Problems/0601-0700/0658-find-k-closest-elements/0658-find-k-closest-elements.cpp) | [go](Problems/0601-0700/0658-find-k-closest-elements/0658-find-k-closest-elements.go) | [py](Problems/0601-0700/0658-find-k-closest-elements/0658-find-k-closest-elements.py) | [rs](Problems/0601-0700/0658-find-k-closest-elements/0658-find-k-closest-elements.rs) | 官方两解：双指针从两端删远端 O(n) / 二分窗口左端点 O(log(n-k)+k)；最优解为二分；平局取较小值，区间是 [0, n-k] |
| ✅ | [74. 搜索二维矩阵（中等） 二维转一维，二分查找](https://leetcode.cn/problems/search-a-2d-matrix/) | [cpp](Problems/0001-0100/0074-search-a-2d-matrix/0074-search-a-2d-matrix.cpp) | [go](Problems/0001-0100/0074-search-a-2d-matrix/0074-search-a-2d-matrix.go) | [py](Problems/0001-0100/0074-search-a-2d-matrix/0074-search-a-2d-matrix.py) | [rs](Problems/0001-0100/0074-search-a-2d-matrix/0074-search-a-2d-matrix.rs) | 官方两解：二维展平成 1 维一次二分 / 先二分定位行再行内二分；log m + log n 与 log(m·n) 同阶，两者并列最优 |

## 哈希表

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [两数之和 （简单，虾皮社招）](https://leetcode.cn/problems/two-sum/) | [cpp](Problems/0001-0100/0001-two-sum/0001-two-sum.cpp) | [go](Problems/0001-0100/0001-two-sum/0001-two-sum.go) | [py](Problems/0001-0100/0001-two-sum/0001-two-sum.py) | [rs](Problems/0001-0100/0001-two-sum/0001-two-sum.rs) | 官方两解：暴力枚举 O(n^2) / 哈希表 O(n)；最优解为哈希表，「先查后插」；【精品】 |
| ☐ | [15. 三数之和（中等，墨奇社招）](https://leetcode.cn/problems/3sum/) |  |  |  |  |  |
| ✅ | [160. 相交链表（简单，新华智云 22 届暑期实习） ](https://leetcode.cn/problems/intersection-of-two-linked-lists/) | [cpp](Problems/0101-0200/0160-intersection-of-two-linked-lists/0160-intersection-of-two-linked-lists.cpp) |  |  |  | 双指针最好，哈希集合 unordered_set 空间复杂度高 |
| ☐ | [141. 环形链表（简单） 判断链表中是否存在环 ](https://leetcode.cn/problems/linked-list-cycle/) |  |  |  |  |  |
| ☐ | [面试题 02.01. 移除重复节点（中等） 借助哈希表](https://leetcode.cn/problems/remove-duplicate-node-lcci/) |  |  |  |  |  |
| ☐ | [面试题 16.02. 单词频率 （简单）](https://leetcode.cn/problems/words-frequency-lcci/) |  |  |  |  |  |
| ☐ | [面试题 01.02. 判定是否互为字符重排（简单）](https://leetcode.cn/problems/check-permutation-lcci/) |  |  |  |  |  |
| ✅ | [LCR 120 / 剑指 Offer 03. 数组中重复的数字 （简单） ](https://leetcode.cn/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof/) | [cpp](JianZhiOffer/03-数组中重复的数字.cpp) | [go](JianZhiOffer/03-数组中重复的数字.go) | [py](JianZhiOffer/03-数组中重复的数字.py) | [rs](JianZhiOffer/03-数组中重复的数字.rs) | 现收录为 LCR 120「寻找文件副本」，签名改为 findRepeatDocument；官方三解：哈希集合 / 排序 / 原地置换；最优解为原地置换，O(n) 时间 O(1) 空间 |
| ☐ | [242. 有效的字母异位词（简单）](https://leetcode.cn/problems/valid-anagram/) |  |  |  |  |  |
| ☐ | [49. 字母异位词分组（中等）](https://leetcode.cn/problems/group-anagrams/) |  |  |  |  |  |
| ☐ | [136. 只出现一次的数字（简单）](https://leetcode.cn/problems/single-number/) |  |  |  |  |  |
| ☐ | [349. 两个数组的交集 （简单）](https://leetcode.cn/problems/intersection-of-two-arrays/) |  |  |  |  |  |
| ☐ | [1122. 数组的相对排序（中等）](https://leetcode.cn/problems/relative-sort-array/) |  |  |  |  |  |
| ☐ | [706. 设计哈希映射（简单）](https://leetcode.cn/problems/design-hashmap/) |  |  |  |  |  |
| ☐ | [146. LRU 缓存机制 （中等）标准的 LRU](https://leetcode.cn/problems/lru-cache/) |  |  |  |  |  |
| ☐ | [面试题 16.21. 交换和（中等） 找规律，可以借助哈希表优化](https://leetcode.cn/problems/sum-swap-lcci/) |  |  |  |  |  |

# 第五阶段（2 周）

## 二叉树

### 题型 1：二叉树前中后序遍历

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [144. 二叉树的前序遍历（简单）](https://leetcode.cn/problems/binary-tree-preorder-traversal/) | [cpp](Problems/0101-0200/0144-binary-tree-preorder-traversal/0144-binary-tree-preorder-traversal.cpp) | [go](Problems/0101-0200/0144-binary-tree-preorder-traversal/0144-binary-tree-preorder-traversal.go) | [py](Problems/0101-0200/0144-binary-tree-preorder-traversal/0144-binary-tree-preorder-traversal.py) | [rs](Problems/0101-0200/0144-binary-tree-preorder-traversal/0144-binary-tree-preorder-traversal.rs) | 官方三解：递归 / 显式栈迭代 / Morris 遍历；最优解为 Morris，O(n) 时间 O(1) 空间 |
| ✅ | [94. 二叉树的中序遍历 （简单）](https://leetcode.cn/problems/binary-tree-inorder-traversal/) | [cpp](Problems/0001-0100/0094-binary-tree-inorder-traversal/0094-binary-tree-inorder-traversal.cpp) | [go](Problems/0001-0100/0094-binary-tree-inorder-traversal/0094-binary-tree-inorder-traversal.go) | [py](Problems/0001-0100/0094-binary-tree-inorder-traversal/0094-binary-tree-inorder-traversal.py) | [rs](Problems/0001-0100/0094-binary-tree-inorder-traversal/0094-binary-tree-inorder-traversal.rs) | 官方三解：递归 / 显式栈迭代 / Morris 遍历；最优解为 Morris，O(n) 时间 O(1) 空间；中序在拆线索时才访问 |
| ✅ | [145. 二叉树的后序遍历（简单）](https://leetcode.cn/problems/binary-tree-postorder-traversal/) | [cpp](Problems/0101-0200/0145-binary-tree-postorder-traversal/0145-binary-tree-postorder-traversal.cpp) | [go](Problems/0101-0200/0145-binary-tree-postorder-traversal/0145-binary-tree-postorder-traversal.go) | [py](Problems/0101-0200/0145-binary-tree-postorder-traversal/0145-binary-tree-postorder-traversal.py) | [rs](Problems/0101-0200/0145-binary-tree-postorder-traversal/0145-binary-tree-postorder-traversal.rs) | 官方三解：递归 / 显式栈 + prev 指针迭代 / Morris 遍历；最优解为 Morris，O(n) 时间 O(1) 空间；拆线索时把左子树的右脊倒序输出 |
| ✅ | [589. N 叉树的前序遍历（简单）](https://leetcode.cn/problems/n-ary-tree-preorder-traversal/) | [cpp](Problems/0501-0600/0589-n-ary-tree-preorder-traversal/0589-n-ary-tree-preorder-traversal.cpp) | [go](Problems/0501-0600/0589-n-ary-tree-preorder-traversal/0589-n-ary-tree-preorder-traversal.go) | [py](Problems/0501-0600/0589-n-ary-tree-preorder-traversal/0589-n-ary-tree-preorder-traversal.py) | [rs](Problems/0501-0600/0589-n-ary-tree-preorder-traversal/0589-n-ary-tree-preorder-traversal.rs) | 官方三解：递归 / 显式栈 + 哈希表记录孩子下标 / 逆序压栈迭代；最优解为递归，O(m) 时间 O(h) 空间 |
| ✅ | N(>2) 叉树没法做中序遍历 |  |  |  |  |  |
| ✅ | [590. N 叉树的后序遍历（简单）](https://leetcode.cn/problems/n-ary-tree-postorder-traversal/) | [cpp](Problems/0501-0600/0590-n-ary-tree-postorder-traversal/0590-n-ary-tree-postorder-traversal.cpp) | [go](Problems/0501-0600/0590-n-ary-tree-postorder-traversal/0590-n-ary-tree-postorder-traversal.go) | [py](Problems/0501-0600/0590-n-ary-tree-postorder-traversal/0590-n-ary-tree-postorder-traversal.py) | [rs](Problems/0501-0600/0590-n-ary-tree-postorder-traversal/0590-n-ary-tree-postorder-traversal.rs) | 官方四解：递归 / 显式栈 + 哈希表记录下标 / 显式栈 + visited 集合 / 前序反转；最优解为递归，O(m) 时间 O(h) 空间；前序反转最巧但空间 O(m) |

### 题型 2：二叉树按层遍历

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [LCR 149 / 剑指 Offer 32 - I. 从上到下打印二叉树（中等）](https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/) | [cpp](JianZhiOffer/32-I-从上到下打印二叉树.cpp) | [go](JianZhiOffer/32-I-从上到下打印二叉树.go) | [py](JianZhiOffer/32-I-从上到下打印二叉树.py) | [rs](JianZhiOffer/32-I-从上到下打印二叉树.rs) | 现收录为 LCR 149「彩灯装饰记录 I」；BFS + 队列，输出一维层序，O(n) 时间 O(w) 空间 |
| ✅ | [102. 二叉树的层序遍历（中等） ](https://leetcode.cn/problems/binary-tree-level-order-traversal/) | [cpp](Problems/0101-0200/0102-binary-tree-level-order-traversal/0102-binary-tree-level-order-traversal.cpp) | [go](Problems/0101-0200/0102-binary-tree-level-order-traversal/0102-binary-tree-level-order-traversal.go) | [py](Problems/0101-0200/0102-binary-tree-level-order-traversal/0102-binary-tree-level-order-traversal.py) | [rs](Problems/0101-0200/0102-binary-tree-level-order-traversal/0102-binary-tree-level-order-traversal.rs) | 官方一解：BFS + 按层定长出队；O(n) 时间 O(w) 空间，不需要额外记录层号 |
| ✅ | [LCR 151 / 剑指 Offer 32 - III. 从上到下打印二叉树 III （中等）](https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/) | [cpp](JianZhiOffer/32-III-从上到下打印二叉树 III.cpp) | [go](JianZhiOffer/32-III-从上到下打印二叉树 III.go) | [py](JianZhiOffer/32-III-从上到下打印二叉树 III.py) | [rs](JianZhiOffer/32-III-从上到下打印二叉树 III.rs) | 现收录为 LCR 151「彩灯装饰记录 III」；两解：按层收集后反转奇数层 / 双端填充；最优解为双端填充，O(n) 时间 O(w) 空间 |
| ✅ | [429. N 叉树的层序遍历（中等）](https://leetcode.cn/problems/n-ary-tree-level-order-traversal/) | [cpp](Problems/0401-0500/0429-n-ary-tree-level-order-traversal/0429-n-ary-tree-level-order-traversal.cpp) | [go](Problems/0401-0500/0429-n-ary-tree-level-order-traversal/0429-n-ary-tree-level-order-traversal.go) | [py](Problems/0401-0500/0429-n-ary-tree-level-order-traversal/0429-n-ary-tree-level-order-traversal.py) | [rs](Problems/0401-0500/0429-n-ary-tree-level-order-traversal/0429-n-ary-tree-level-order-traversal.rs) | 官方一解：BFS + 按层定长出队；O(m) 时间 O(w) 空间，孩子按 children 下标顺序入队 |
| ✅ | [513. 找树左下角的值（中等）](https://leetcode.cn/problems/find-bottom-left-tree-value/) | [cpp](Problems/0501-0600/0513-find-bottom-left-tree-value/0513-find-bottom-left-tree-value.cpp) | [go](Problems/0501-0600/0513-find-bottom-left-tree-value/0513-find-bottom-left-tree-value.go) | [py](Problems/0501-0600/0513-find-bottom-left-tree-value/0513-find-bottom-left-tree-value.py) | [rs](Problems/0501-0600/0513-find-bottom-left-tree-value/0513-find-bottom-left-tree-value.rs) | 官方两解：DFS 先左后右记录最大深度 / BFS 先右后左取最后出队；最优解为 DFS，O(n) 时间 O(h) 空间 |

### 题型 3：二叉树上的递归

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [104. 二叉树的最大深度（简单） ](https://leetcode.cn/problems/maximum-depth-of-binary-tree/) | [cpp](Problems/0101-0200/0104-maximum-depth-of-binary-tree/0104-maximum-depth-of-binary-tree.cpp) | [go](Problems/0101-0200/0104-maximum-depth-of-binary-tree/0104-maximum-depth-of-binary-tree.go) | [py](Problems/0101-0200/0104-maximum-depth-of-binary-tree/0104-maximum-depth-of-binary-tree.py) | [rs](Problems/0101-0200/0104-maximum-depth-of-binary-tree/0104-maximum-depth-of-binary-tree.rs) | 官方两解：后序递归 1 + max(左, 右) / BFS 按层计数；最优解为 DFS，O(n) 时间 O(h) 空间 |
| ✅ | [543. 二叉树的直径（简单）](https://leetcode.cn/problems/diameter-of-binary-tree/) | [cpp](Problems/0501-0600/0543-diameter-of-binary-tree/0543-diameter-of-binary-tree.cpp) | [go](Problems/0501-0600/0543-diameter-of-binary-tree/0543-diameter-of-binary-tree.go) | [py](Problems/0501-0600/0543-diameter-of-binary-tree/0543-diameter-of-binary-tree.py) | [rs](Problems/0501-0600/0543-diameter-of-binary-tree/0543-diameter-of-binary-tree.rs) | 官方一解：后序递归一次遍历，边求深度边用 左+右+2 更新答案；O(n) 时间 O(h) 空间，另附 O(n^2) 暴力对照 |
| ✅ | [559. N 叉树的最大深度（简单）](https://leetcode.cn/problems/maximum-depth-of-n-ary-tree/) | [cpp](Problems/0501-0600/0559-maximum-depth-of-n-ary-tree/0559-maximum-depth-of-n-ary-tree.cpp) | [go](Problems/0501-0600/0559-maximum-depth-of-n-ary-tree/0559-maximum-depth-of-n-ary-tree.go) | [py](Problems/0501-0600/0559-maximum-depth-of-n-ary-tree/0559-maximum-depth-of-n-ary-tree.py) | [rs](Problems/0501-0600/0559-maximum-depth-of-n-ary-tree/0559-maximum-depth-of-n-ary-tree.rs) | 官方两解：DFS 后序 1 + max(children) / BFS 按层计数；最优解为 DFS，O(m) 时间 O(h) 空间 |
| ✅ | [LCR 176 / 剑指 Offer 55 - II. 判断是否为平衡二叉树（简单）](https://leetcode.cn/problems/ping-heng-er-cha-shu-lcof/) | [cpp](JianZhiOffer/55-II-平衡二叉树.cpp) | [go](JianZhiOffer/55-II-平衡二叉树.go) | [py](JianZhiOffer/55-II-平衡二叉树.py) | [rs](JianZhiOffer/55-II-平衡二叉树.rs) | 现收录为 LCR 176「判断是否为平衡二叉树」；官方两解：后序剪枝带高度 / 先序 + 求深度；最优解为后序剪枝，O(n) 时间 O(n) 空间 |
| ✅ | [617. 合并二叉树（简单）](https://leetcode.cn/problems/merge-two-binary-trees/) | [cpp](Problems/0601-0700/0617-merge-two-binary-trees/0617-merge-two-binary-trees.cpp) | [go](Problems/0601-0700/0617-merge-two-binary-trees/0617-merge-two-binary-trees.go) | [py](Problems/0601-0700/0617-merge-two-binary-trees/0617-merge-two-binary-trees.py) | [rs](Problems/0601-0700/0617-merge-two-binary-trees/0617-merge-two-binary-trees.rs) | 官方两解：DFS 同步递归 / BFS 成对出队并嫁接空侧；最优解为 DFS，O(m+n) 时间 O(h) 空间 |
| ✅ | [226. 翻转二叉树 （简单）](https://leetcode.cn/problems/invert-binary-tree/) | [cpp](Problems/0201-0300/0226-invert-binary-tree/0226-invert-binary-tree.cpp) | [go](Problems/0201-0300/0226-invert-binary-tree/0226-invert-binary-tree.go) | [py](Problems/0201-0300/0226-invert-binary-tree/0226-invert-binary-tree.py) | [rs](Problems/0201-0300/0226-invert-binary-tree/0226-invert-binary-tree.rs) | 官方两解：递归交换左右孩子 / BFS 层序逐个交换；最优解为递归，O(n) 时间 O(h) 空间，原地翻转无需新建节点 |
| ✅ | [101. 对称二叉树（简单）](https://leetcode.cn/problems/symmetric-tree/) | [cpp](Problems/0101-0200/0101-symmetric-tree/0101-symmetric-tree.cpp) | [go](Problems/0101-0200/0101-symmetric-tree/0101-symmetric-tree.go) | [py](Problems/0101-0200/0101-symmetric-tree/0101-symmetric-tree.py) | [rs](Problems/0101-0200/0101-symmetric-tree/0101-symmetric-tree.rs) | 官方两解：递归镜像比较 / 队列成对出队；最优解为递归，O(n) 时间 O(h) 空间，注意左对右而非左对左 |
| ✅ | [98. 验证二叉搜索树（中等）](https://leetcode.cn/problems/validate-binary-search-tree/) | [cpp](Problems/0001-0100/0098-validate-binary-search-tree/0098-validate-binary-search-tree.cpp) | [go](Problems/0001-0100/0098-validate-binary-search-tree/0098-validate-binary-search-tree.go) | [py](Problems/0001-0100/0098-validate-binary-search-tree/0098-validate-binary-search-tree.py) | [rs](Problems/0001-0100/0098-validate-binary-search-tree/0098-validate-binary-search-tree.rs) | 官方三解：递归传上下界 / 中序栈验证递增 / Morris 中序；最优解为 Morris，O(n) 时间 O(1) 空间 |

### 题型 4：二叉查找树

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [LCR 174 / 剑指 Offer 54. 寻找二叉搜索树中的目标节点（简单）](https://leetcode.cn/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/) | [cpp](JianZhiOffer/54-二叉搜索树的第k大节点.cpp) | [go](JianZhiOffer/54-二叉搜索树的第k大节点.go) | [py](JianZhiOffer/54-二叉搜索树的第k大节点.py) | [rs](JianZhiOffer/54-二叉搜索树的第k大节点.rs) | 现收录为 LCR 174；官方一解：右根左反序中序，数到第 cnt 个即停；O(n) 时间 O(h) 空间，另附迭代栈对照 |
| ✅ | [538. 把二叉搜索树转换为累加树 （中等）](https://leetcode.cn/problems/convert-bst-to-greater-tree/) | [cpp](Problems/0501-0600/0538-convert-bst-to-greater-tree/0538-convert-bst-to-greater-tree.cpp) | [go](Problems/0501-0600/0538-convert-bst-to-greater-tree/0538-convert-bst-to-greater-tree.go) | [py](Problems/0501-0600/0538-convert-bst-to-greater-tree/0538-convert-bst-to-greater-tree.py) | [rs](Problems/0501-0600/0538-convert-bst-to-greater-tree/0538-convert-bst-to-greater-tree.rs) | 官方两解：递归反序中序累加 / Morris 反序中序；最优解为 Morris，O(n) 时间 O(1) 空间；与 1038 同题 |
| ✅ | [面试题 04.06. 后继者（中等）](https://leetcode.cn/problems/successor-lcci/) | [cpp](InterviewQuestions/04.06-successor-lcci.cpp) | [go](InterviewQuestions/04.06-successor-lcci.go) | [py](InterviewQuestions/04.06-successor-lcci.py) | [rs](InterviewQuestions/04.06-successor-lcci.rs) | 官方两解：中序遍历收集全部节点 / 利用 BST 性质找"右子树最左"或"大于 p 的最小祖先"；最优解为后者，O(h) 时间 O(1) 空间 |

### 题型 5：LCA 最近公共祖先

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [236. 二叉树的最近公共祖先（中等） ](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/) | [cpp](Problems/0201-0300/0236-lowest-common-ancestor-of-a-binary-tree/0236-lowest-common-ancestor-of-a-binary-tree.cpp) | [go](Problems/0201-0300/0236-lowest-common-ancestor-of-a-binary-tree/0236-lowest-common-ancestor-of-a-binary-tree.go) | [py](Problems/0201-0300/0236-lowest-common-ancestor-of-a-binary-tree/0236-lowest-common-ancestor-of-a-binary-tree.py) | [rs](Problems/0201-0300/0236-lowest-common-ancestor-of-a-binary-tree/0236-lowest-common-ancestor-of-a-binary-tree.rs) | 官方仅一解：后序递归，左右子树各自上报，当前节点两侧都非空（或等于 p/q）即返回；O(n) 时间 O(h) 空间，即最优解 |
| ✅ | [剑指 Offer 68 - I. 二叉搜索树的最近公共祖先（简单） ](https://leetcode.cn/problems/er-cha-sou-suo-shu-de-zui-jin-gong-gong-zu-xian-lcof/) | [cpp](JianZhiOffer/68-I-二叉搜索树的最近公共祖先.cpp) | [go](JianZhiOffer/68-I-二叉搜索树的最近公共祖先.go) | [py](JianZhiOffer/68-I-二叉搜索树的最近公共祖先.py) | [rs](JianZhiOffer/68-I-二叉搜索树的最近公共祖先.rs) | 现收录为 LCR 193；官方两解：两次遍历求路径取公共前缀 / 一次遍历利用 BST 大小关系；最优解为一次遍历，O(h) 时间 O(1) 空间 |

### 题型 6：二叉树转单、双、循环链表

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [114. 二叉树展开为链表（中等）](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/) | [cpp](Problems/0101-0200/0114-flatten-binary-tree-to-linked-list/0114-flatten-binary-tree-to-linked-list.cpp) | [go](Problems/0101-0200/0114-flatten-binary-tree-to-linked-list/0114-flatten-binary-tree-to-linked-list.go) | [py](Problems/0101-0200/0114-flatten-binary-tree-to-linked-list/0114-flatten-binary-tree-to-linked-list.py) | [rs](Problems/0101-0200/0114-flatten-binary-tree-to-linked-list/0114-flatten-binary-tree-to-linked-list.rs) | 官方三解：前序收集后重建 / 前序 + 显式栈 / 找左子树最右节点作前驱的原地展开；最优解为原地展开，O(n) 时间 O(1) 空间 |
| ✅ | [面试题 17.12. BiNode（简单）](https://leetcode.cn/problems/binode-lcci/) | [cpp](InterviewQuestions/17.12-binode-lcci.cpp) | [go](InterviewQuestions/17.12-binode-lcci.go) | [py](InterviewQuestions/17.12-binode-lcci.py) | [rs](InterviewQuestions/17.12-binode-lcci.rs) | 官方两解：递归中序返回 (头, 尾) 拼接 / Morris 中序原地改指针；最优解为 Morris，O(n) 时间 O(1) 空间 |
| ✅ | [剑指 Offer 36. 二叉搜索树与双向链表（中等）](https://leetcode.cn/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/) | [cpp](JianZhiOffer/36-二叉搜索树与双向链表.cpp) | [go](JianZhiOffer/36-二叉搜索树与双向链表.go) | [py](JianZhiOffer/36-二叉搜索树与双向链表.py) | [rs](JianZhiOffer/36-二叉搜索树与双向链表.rs) | 现收录为 LCR 155（与主站 426 同题）；官方两解：中序递归 + 前驱指针 / 显式栈中序；最优解为递归，最后必须把首尾互指成循环链表 |
| ✅ | [面试题 04.03. 特定深度节点链表（中等）](https://leetcode.cn/problems/list-of-depth-lcci/) | [cpp](InterviewQuestions/04.03-list-of-depth-lcci.cpp) | [go](InterviewQuestions/04.03-list-of-depth-lcci.go) | [py](InterviewQuestions/04.03-list-of-depth-lcci.py) | [rs](InterviewQuestions/04.03-list-of-depth-lcci.rs) | 官方仅一解：BFS 按层建链表，每轮先固定层宽；O(n) 时间 O(w) 空间，即最优解 |

### 题型 7：按照遍历结果反向构建二叉树

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [105. 从前序与中序遍历序列构造二叉树（中等）](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) | [cpp](Problems/0101-0200/0105-construct-binary-tree-from-preorder-and-inorder-traversal/0105-construct-binary-tree-from-preorder-and-inorder-traversal.cpp) | [go](Problems/0101-0200/0105-construct-binary-tree-from-preorder-and-inorder-traversal/0105-construct-binary-tree-from-preorder-and-inorder-traversal.go) | [py](Problems/0101-0200/0105-construct-binary-tree-from-preorder-and-inorder-traversal/0105-construct-binary-tree-from-preorder-and-inorder-traversal.py) | [rs](Problems/0101-0200/0105-construct-binary-tree-from-preorder-and-inorder-traversal/0105-construct-binary-tree-from-preorder-and-inorder-traversal.rs) | 官方两解：递归分治 + 中序下标哈希表 / 迭代栈；两者时间同为 O(n)、并列最优，迭代栈省掉哈希表把空间降到 O(h) |
| ✅ | [889. 根据前序和后序遍历构造二叉树（中等）](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-postorder-traversal/) | [cpp](Problems/0801-0900/0889-construct-binary-tree-from-preorder-and-postorder-traversal/0889-construct-binary-tree-from-preorder-and-postorder-traversal.cpp) | [go](Problems/0801-0900/0889-construct-binary-tree-from-preorder-and-postorder-traversal/0889-construct-binary-tree-from-preorder-and-postorder-traversal.go) | [py](Problems/0801-0900/0889-construct-binary-tree-from-preorder-and-postorder-traversal/0889-construct-binary-tree-from-preorder-and-postorder-traversal.py) | [rs](Problems/0801-0900/0889-construct-binary-tree-from-preorder-and-postorder-traversal/0889-construct-binary-tree-from-preorder-and-postorder-traversal.rs) | 官方仅一解：递归 + 后序下标哈希表，O(n) 时间 O(n) 空间，即最优解；答案不唯一，单孩子一律挂左 |
| ✅ | [106. 从中序与后序遍历序列构造二叉树（中等）](https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/) | [cpp](Problems/0101-0200/0106-construct-binary-tree-from-inorder-and-postorder-traversal/0106-construct-binary-tree-from-inorder-and-postorder-traversal.cpp) | [go](Problems/0101-0200/0106-construct-binary-tree-from-inorder-and-postorder-traversal/0106-construct-binary-tree-from-inorder-and-postorder-traversal.go) | [py](Problems/0101-0200/0106-construct-binary-tree-from-inorder-and-postorder-traversal/0106-construct-binary-tree-from-inorder-and-postorder-traversal.py) | [rs](Problems/0101-0200/0106-construct-binary-tree-from-inorder-and-postorder-traversal/0106-construct-binary-tree-from-inorder-and-postorder-traversal.rs) | 官方两解：递归分治 + 中序下标哈希表 / 迭代栈；两者时间同为 O(n)、并列最优，注意根在后序区间右端；另见 [根据中序和后序序列构建先序序列（天翼云社招）](Others/get_preorder_by_middle_back_order.cpp) |
| ✅ | [LCR 152 / 剑指 Offer 33. 二叉搜索树的后序遍历序列（中等）](https://leetcode.cn/problems/er-cha-sou-suo-shu-de-hou-xu-bian-li-xu-lie-lcof/) | [cpp](JianZhiOffer/33-二叉搜索树的后序遍历序列.cpp) | [go](JianZhiOffer/33-二叉搜索树的后序遍历序列.go) | [py](JianZhiOffer/33-二叉搜索树的后序遍历序列.py) | [rs](JianZhiOffer/33-二叉搜索树的后序遍历序列.rs) | 现收录为 LCR 152；官方两解：递归分治判断（左斜链最差 O(n^2)）/ 单调栈靠祖先上界剪枝；最优解为单调栈，O(n) 时间 O(h) 空间 |

### 题型 8：二叉树上的最长路径和

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [LCR 153 / 剑指 Offer 34. 二叉树中和为某一值的路径（中等）](https://leetcode.cn/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/) | [cpp](JianZhiOffer/34-二叉树中和为某一值的路径.cpp) | [go](JianZhiOffer/34-二叉树中和为某一值的路径.go) | [py](JianZhiOffer/34-二叉树中和为某一值的路径.py) | [rs](JianZhiOffer/34-二叉树中和为某一值的路径.rs) | 现收录为 LCR 153；官方仅一解：DFS 回溯，只收「叶子且和相等」的路径，回溯必须弹出末尾元素；O(n) 时间 O(h) 空间，即最优解 |
| ✅ | [124. 二叉树中的最大路径和 （困难）](https://leetcode.cn/problems/binary-tree-maximum-path-sum/) | [cpp](Problems/0101-0200/0124-binary-tree-maximum-path-sum/0124-binary-tree-maximum-path-sum.cpp) | [go](Problems/0101-0200/0124-binary-tree-maximum-path-sum/0124-binary-tree-maximum-path-sum.go) | [py](Problems/0101-0200/0124-binary-tree-maximum-path-sum/0124-binary-tree-maximum-path-sum.py) | [rs](Problems/0101-0200/0124-binary-tree-maximum-path-sum/0124-binary-tree-maximum-path-sum.rs) | 官方仅一解：后序递归，返回给父节点的一侧贡献要与 0 取 max，答案用「左+右+当前值」更新；O(n) 时间 O(h) 空间，即最优解 |
| ✅ | [437. 路径总和 III （中等）](https://leetcode.cn/problems/path-sum-iii/) | [cpp](Problems/0401-0500/0437-path-sum-iii/0437-path-sum-iii.cpp) | [go](Problems/0401-0500/0437-path-sum-iii/0437-path-sum-iii.go) | [py](Problems/0401-0500/0437-path-sum-iii/0437-path-sum-iii.py) | [rs](Problems/0401-0500/0437-path-sum-iii/0437-path-sum-iii.rs) | 官方两解：DFS 枚举每个起点 O(n^2) / 前缀和 + 哈希表回溯 O(n)；最优解为前缀和，先查后插、退出时回滚，累加用 64 位 |

# 第六阶段（1 周）

## 堆

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [23. 合并 K 个升序链表(困难) ](https://leetcode.cn/problems/merge-k-sorted-lists/) | [cpp](Problems/0001-0100/0023-merge-k-sorted-lists/0023-merge-k-sorted-lists.cpp) | [go](Problems/0001-0100/0023-merge-k-sorted-lists/0023-merge-k-sorted-lists.go) | [py](Problems/0001-0100/0023-merge-k-sorted-lists/0023-merge-k-sorted-lists.py) | [rs](Problems/0001-0100/0023-merge-k-sorted-lists/0023-merge-k-sorted-lists.rs) | 官方三解：顺序合并 O(k^2 n) / 分治两两合并 / 优先队列；最优解为分治与优先队列并列，均 O(n log k)（分治省空间，堆支持流式输入）；注意 k=0 与含空链表 |
| ✅ | [347. 前 K 个高频元素（中等） ](https://leetcode.cn/problems/top-k-frequent-elements/) | [cpp](Problems/0301-0400/0347-top-k-frequent-elements/0347-top-k-frequent-elements.cpp) | [go](Problems/0301-0400/0347-top-k-frequent-elements/0347-top-k-frequent-elements.go) | [py](Problems/0301-0400/0347-top-k-frequent-elements/0347-top-k-frequent-elements.py) | [rs](Problems/0301-0400/0347-top-k-frequent-elements/0347-top-k-frequent-elements.rs) | 官方三解：小根堆 O(n log k) / 基于快速排序的快选（期望 O(n)、最坏 O(n^2)）/ 桶排序；最优解为桶排序，最坏也是 O(n) |
| ✅ | [973. 最接近原点的 K 个点（中等）](https://leetcode.cn/problems/k-closest-points-to-origin/) | [cpp](Problems/0901-1000/0973-k-closest-points-to-origin/0973-k-closest-points-to-origin.cpp) | [go](Problems/0901-1000/0973-k-closest-points-to-origin/0973-k-closest-points-to-origin.go) | [py](Problems/0901-1000/0973-k-closest-points-to-origin/0973-k-closest-points-to-origin.py) | [rs](Problems/0901-1000/0973-k-closest-points-to-origin/0973-k-closest-points-to-origin.rs) | 官方两解：全部排序 O(n log n) / 大小为 k 的大根堆；最优解为大根堆 O(n log k)，比较用距离平方避免浮点误差与溢出 |
| ✅ | [295. 数据流的中位数（困难）](https://leetcode.cn/problems/find-median-from-data-stream/) | [cpp](Problems/0201-0300/0295-find-median-from-data-stream/0295-find-median-from-data-stream.cpp) | [go](Problems/0201-0300/0295-find-median-from-data-stream/0295-find-median-from-data-stream.go) | [py](Problems/0201-0300/0295-find-median-from-data-stream/0295-find-median-from-data-stream.py) | [rs](Problems/0201-0300/0295-find-median-from-data-stream/0295-find-median-from-data-stream.rs) | 官方两解：有序数组 + 二分插入（addNum O(n)）/ 对顶双堆（addNum O(log n)）；最优解为对顶双堆，两堆大小差不超过 1 |
| ✅ | [313. 超级丑数（中等）](https://leetcode.cn/problems/super-ugly-number/) | [cpp](Problems/0301-0400/0313-super-ugly-number/0313-super-ugly-number.cpp) | [go](Problems/0301-0400/0313-super-ugly-number/0313-super-ugly-number.go) | [py](Problems/0301-0400/0313-super-ugly-number/0313-super-ugly-number.py) | [rs](Problems/0301-0400/0313-super-ugly-number/0313-super-ugly-number.rs) | 官方两解：最小堆 O(n·k·log) / 多指针动态规划 O(n·k)；最优解为多指针 DP（另附缓存候选值的同阶变体，每轮少一半乘法）；等值指针要一次性全推进去重 |

## Trie 树

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [208. 实现 Trie (前缀树) （中等） 标准 Trie 树](https://leetcode.cn/problems/implement-trie-prefix-tree/) | [cpp](Problems/0201-0300/0208-implement-trie-prefix-tree/0208-implement-trie-prefix-tree.cpp) | [go](Problems/0201-0300/0208-implement-trie-prefix-tree/0208-implement-trie-prefix-tree.go) | [py](Problems/0201-0300/0208-implement-trie-prefix-tree/0208-implement-trie-prefix-tree.py) | [rs](Problems/0201-0300/0208-implement-trie-prefix-tree/0208-implement-trie-prefix-tree.rs) | 官方仅一解：孩子用 26 叉数组存储，四操作均 O(L)；即最优解；词尾标记区分 search 与 startsWith |
| ☐ | 以下为选做题目： |  |  |  |  |  |
| ☐ | [面试题 17.17. 多次搜索（中等） 标准 AC 自动机，不过写 AC 自动机太复杂，Trie 树搞定](https://leetcode.cn/problems/multi-search-lcci/) |  |  |  |  |  |
| ☐ | [212. 单词搜索 II（困难）](https://leetcode.cn/problems/word-search-ii/) |  |  |  |  |  |

# 第七阶段（1 周）

## 回溯

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [面试题 08.12. 八皇后（困难）](https://leetcode.cn/problems/eight-queens-lcci/) | [cpp](InterviewQuestions/08.12-eight-queens-lcci.cpp) | [go](InterviewQuestions/08.12-eight-queens-lcci.go) | [py](InterviewQuestions/08.12-eight-queens-lcci.py) | [rs](InterviewQuestions/08.12-eight-queens-lcci.rs) | 官方一解：逐行放置 + 列/主对角/副对角三个冲突集合回溯；与 51 题同构，本题返回全部方案的棋盘列表（n=8 时 92 组）；O(n!) 时间、O(n) 空间 |
| ☐ | [51. N 皇后（困难）](https://leetcode.cn/problems/n-queens/) |  |  |  |  |  |
| ☐ | [37. 解数独](https://leetcode.cn/problems/sudoku-solver/) |  |  |  |  |  |
| ☐ | [17. 电话号码的字母组合 （中等）](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/) |  |  |  |  |  |
| ☐ | [77. 组合 （中等） 给 n 个数返回所有 k 个数的组合](https://leetcode.cn/problems/combinations/) |  |  |  |  |  |
| ☐ | [78. 子集（中等） 所有的组合](https://leetcode.cn/problems/subsets/) |  |  |  |  |  |
| ☐ | [90. 子集 II（中等）有重复数据](https://leetcode.cn/problems/subsets-ii/) |  |  |  |  |  |
| ☐ | [46. 全排列 （中等） 所有排列](https://leetcode.cn/problems/permutations/) |  |  |  |  |  |
| ☐ | [47. 全排列 II（中等） 有重复数据](https://leetcode.cn/problems/permutations-ii/) |  |  |  |  |  |
| ☐ | [39. 组合总和 （中等） 选出某几个数相加为给定和，无重复数据，可以使用多次，不能有重复答案](https://leetcode.cn/problems/combination-sum/) |  |  |  |  |  |
| ☐ | [40. 组合总和 II（中等）选出某几个数相加为给定和，有重复数据，只能使用一次，不能有重复答案](https://leetcode.cn/problems/combination-sum-ii/) |  |  |  |  |  |
| ☐ | [216. 组合总和 III（中等） 选出 k 个数相加为给定和，没有重复数据，只能使用一次](https://leetcode.cn/problems/combination-sum-iii/) |  |  |  |  |  |
| ☐ | [131. 分割回文串（中等）](https://leetcode.cn/problems/palindrome-partitioning/) |  |  |  |  |  |
| ☐ | [93. 复原 IP 地址（中等）](https://leetcode.cn/problems/restore-ip-addresses/) |  |  |  |  |  |
| ☐ | [22. 括号生成（中等） ](https://leetcode.cn/problems/generate-parentheses/) |  |  |  |  |  |

## DFS

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [剑指 Offer 13. 机器人的运动范围（中等）](https://leetcode.cn/problems/ji-qi-ren-de-yun-dong-fan-wei-lcof/) |  |  |  |  |  |
| ☐ | [面试题 08.10. 颜色填充（简单）](https://leetcode.cn/problems/color-fill-lcci/) |  |  |  |  |  |
| ☐ | [面试题 04.01. 节点间通路（中等）](https://leetcode.cn/problems/route-between-nodes-lcci/) |  |  |  |  |  |
| ☐ | [200. 岛屿数量（中等）](https://leetcode.cn/problems/number-of-islands/) |  |  |  |  |  |
| ☐ | [面试题 16.19. 水域大小（中等）](https://leetcode.cn/problems/pond-sizes-lcci/) |  |  |  |  |  |
| ☐ | [207. 课程表（中等）](https://leetcode.cn/problems/course-schedule/) |  |  |  |  |  |
| ☐ | [79. 单词搜索（中等）](https://leetcode.cn/problems/word-search/) |  |  |  |  |  |
| ☐ | [1306. 跳跃游戏 III（中等）](https://leetcode.cn/problems/jump-game-iii/) |  |  |  |  |  |
| ☐ | [752. 打开转盘锁（中等）](https://leetcode.cn/problems/open-the-lock/) |  |  |  |  |  |

个人补充（不一定是放当前板块）：

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [207. 课程表（中等）](https://leetcode.cn/problems/course-schedule/) |  |  |  |  |  |
| ☐ | [210. 课程表 II（中等）](https://leetcode.cn/problems/course-schedule-ii/) |  |  |  |  |  |
| ☐ | [630. 课程表 III（困难）](https://leetcode.cn/problems/course-schedule-iii/) |  |  |  |  |  |
| ☐ | [1462. 课程表 IV（中等）](https://leetcode.cn/problems/course-schedule-iv/) |  |  |  |  |  |
| ☐ | [1136. 并行课程（力扣会员）](https://leetcode.cn/problems/parallel-courses/) |  |  |  |  |  |
| ☐ | [1494. 并行课程 II（困难）](https://leetcode.cn/problems/parallel-courses-ii/) |  |  |  |  |  |
| ☐ | [2050. 并行课程 III（困难）](https://leetcode.cn/problems/parallel-courses-iii/) |  |  |  |  |  |

个人补充（DFS 其他题）：

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [841. 钥匙和房间（中等）](https://leetcode.cn/problems/keys-and-rooms/) | [cpp](Problems/0801-0900/0841-keys-and-rooms/0841-keys-and-rooms.cpp) |  |  |  |  |

**附加题：**

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [面试题 17.22. 单词转换（困难） ](https://leetcode.cn/problems/word-transformer-lcci/) |  |  |  |  |  |
| ☐ | [面试题 17.07. 婴儿名字（困难） ](https://leetcode.cn/problems/baby-names-lcci/) |  |  |  |  |  |
| ☐ | [529. 扫雷游戏（困难）](https://leetcode.cn/problems/minesweeper/) |  |  |  |  |  |
| ☐ | [127. 单词接龙（困难）](https://leetcode.cn/problems/word-ladder/) |  |  |  |  |  |
| ☐ | [126. 单词接龙 II（困难）](https://leetcode.cn/problems/word-ladder-ii/) |  |  |  |  |  |

# 第八阶段（1 周）

## 背包：

这个系列是动态规划问题。

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [416. 分割等和子集](https://leetcode.cn/problems/partition-equal-subset-sum/) |  |  |  |  |  |
| ☐ | [494. 目标和](https://leetcode.cn/problems/target-sum/) |  |  |  |  |  |
| ☐ | [279. 完全平方数](https://leetcode.cn/problems/perfect-squares/) |  |  |  |  |  |
| ☐ | [322. 零钱兑换 ](https://leetcode.cn/problems/coin-change/) |  |  |  |  |  |
| ☐ | [518. 零钱兑换 II](https://leetcode.cn/problems/coin-change-2/) |  |  |  |  |  |
| ☐ | [377. 组合总和 Ⅳ](https://leetcode.cn/problems/combination-sum-iv/) |  |  |  |  |  |

华为机考题：

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [HJ16 购物单](https://www.nowcoder.com/practice/f9c6f980eeec43ef85be20755ddbeaf4) | [cpp](HuaweiJikao/HJ16-购物单/HJ16-购物单.cpp) |  |  |  |  |

## 打家劫舍：

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [198. 打家劫舍](https://leetcode.cn/problems/house-robber/) |  |  |  |  |  |
| ☐ | [337. 打家劫舍 III ](https://leetcode.cn/problems/house-robber-iii/) |  |  |  |  |  |
| ☐ | [面试题 17.16. 按摩师](https://leetcode.cn/problems/the-masseuse-lcci/) |  |  |  |  |  |

## 买卖股票：

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [121. 买卖股票的最佳时机](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/) |  |  |  |  |  |
| ☐ | [122. 买卖股票的最佳时机 II](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/) |  |  |  |  |  |
| ☐ | [714. 买卖股票的最佳时机含手续](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/) |  |  |  |  |  |
| ☐ | [309. 最佳买卖股票时机含冷冻期](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/) |  |  |  |  |  |
| ☐ | [300. 最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/) |  |  |  |  |  |
| ☐ | [62. 不同路径](https://leetcode.cn/problems/unique-paths/) |  |  |  |  |  |
| ☐ | [64. 最小路径和](https://leetcode.cn/problems/minimum-path-sum/) |  |  |  |  |  |
| ☐ | [剑指 Offer 47. 礼物的最大价值](https://leetcode.cn/problems/li-wu-de-zui-da-jie-zhi-lcof/) |  |  |  |  |  |
| ☐ | [1143. 最长公共子序列](https://leetcode.cn/problems/longest-common-subsequence/) |  |  |  |  |  |
| ☐ | [72. 编辑距离 ](https://leetcode.cn/problems/edit-distance/) |  |  |  |  |  |
| ☐ | [10. 正则表达式匹配](https://leetcode.cn/problems/regular-expression-matching/) |  |  |  |  |  |
| ☐ | [剑指 Offer 14- I. 剪绳子](https://leetcode.cn/problems/jian-sheng-zi-lcof/) |  |  |  |  |  |
| ☐ | [剑指 Offer 46. 把数字翻译成字符串](https://leetcode.cn/problems/ba-shu-zi-fan-yi-cheng-zi-fu-chuan-lcof/) |  |  |  |  |  |
| ☐ | [139. 单词拆分](https://leetcode.cn/problems/word-break/) |  |  |  |  |  |
| ☐ | [437. 路径总和 III](https://leetcode.cn/problems/path-sum-iii/) |  |  |  |  |  |

# 第九阶段（1 周）

## 双指针

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [344. 反转字符串](https://leetcode.cn/problems/reverse-string/) | [cpp](Problems/0301-0400/0344-reverse-string/0344-reverse-string.cpp) | [go](Problems/0301-0400/0344-reverse-string/0344-reverse-string.go) | [py](Problems/0301-0400/0344-reverse-string/0344-reverse-string.py) | [rs](Problems/0301-0400/0344-reverse-string/0344-reverse-string.rs) | 双指针首尾交换 |
| ☐ | [面试题 16.24. 数对和（例题 1）](https://leetcode.cn/problems/pairs-with-sum-lcci/) |  |  |  |  |  |
| ☐ | [1. 两数之和（简单，虾皮社招）](https://leetcode.cn/problems/two-sum/) |  |  |  |  | 双指针需先排序，非官方解法；可参考 三数之和 内层循环 |
| ✅ | [15. 三数之和（中等，墨奇社招）](https://leetcode.cn/problems/3sum/) | [cpp](Problems/0001-0100/0015-3sum/0015-3sum.cpp) |  |  |  |  |
| ☐ | [剑指 Offer 21. 调整数组顺序使奇数位于偶数前面 ](https://leetcode.cn/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof/) |  |  |  |  |  |
| ☐ | [75. 颜色分类 ](https://leetcode.cn/problems/sort-colors/) |  |  |  |  |  |
| ☐ | [283. 移动零 已排序未排序指针（例题 2）](https://leetcode.cn/problems/move-zeroes/) |  |  |  |  |  |
| ☐ | [面试题 16.06. 最小差 类似合并两个有序数组（例题 3）](https://leetcode.cn/problems/smallest-difference-lcci/) |  |  |  |  |  |
| ☐ | [面试题 17.11. 单词距离 类似合并两个有序数组](https://leetcode.cn/problems/find-closest-lcci/) |  |  |  |  |  |

## 滑动窗口

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [剑指 Offer 57 - II. 和为 s 的连续正数序列 （例题 1）](https://leetcode.cn/problems/he-wei-sde-lian-xu-zheng-shu-xu-lie-lcof/) |  |  |  |  |  |
| ☐ | [剑指 Offer 48. 最长不含重复字符的子字符串 （例题 2）](https://leetcode.cn/problems/zui-chang-bu-han-zhong-fu-zi-fu-de-zi-zi-fu-chuan-lcof/) |  |  |  |  |  |
| ☐ | [438. 找到字符串中所有字母异位词 ](https://leetcode.cn/problems/find-all-anagrams-in-a-string/) |  |  |  |  |  |
| ☐ | [76. 最小覆盖子串 ](https://leetcode.cn/problems/minimum-window-substring/) |  |  |  |  |  |

## 前缀后缀统计

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [53. 最大子序和](https://leetcode.cn/problems/maximum-subarray/) |  |  |  |  |  |
| ☐ | [238. 除自身以外数组的乘积](https://leetcode.cn/problems/product-of-array-except-self/) |  |  |  |  |  |
| ☐ | [面试题 05.03. 翻转数位](https://leetcode.cn/problems/reverse-bits-lcci/) |  |  |  |  |  |
| ☐ | [42. 接雨水](https://leetcode.cn/problems/trapping-rain-water/) |  |  |  |  |  |

## 位运算

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ☐ | [191. 位 1 的个数 （例题 1）](https://leetcode.cn/problems/number-of-1-bits/) |  |  |  |  |  |
| ☐ | [461. 汉明距离（例题 2）](https://leetcode.cn/problems/hamming-distance/) |  |  |  |  |  |
| ☐ | [面试题 05.06. 整数转换](https://leetcode.cn/problems/convert-integer-lcci/) |  |  |  |  |  |
| ☐ | [面试题 05.07. 配对交换 ](https://leetcode.cn/problems/exchange-lcci/) |  |  |  |  |  |
| ☐ | [面试题 05.01. 插入 ](https://leetcode.cn/problems/insert-into-bits-lcci/) |  |  |  |  |  |
| ☐ | [面试题 17.04. 消失的数字](https://leetcode.cn/problems/missing-number-lcci/) |  |  |  |  |  |
| ☐ | [剑指 Offer 56 - I. 数组中数字出现的次数](https://leetcode.cn/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-lcof/) |  |  |  |  |  |
| ☐ | [剑指 Offer 56 - II. 数组中数字出现的次数 II ](https://leetcode.cn/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-ii-lcof/) |  |  |  |  |  |
| ☐ | [面试题 16.01. 交换数字](https://leetcode.cn/problems/swap-numbers-lcci/) |  |  |  |  |  |
| ☐ | [231. 2 的幂](https://leetcode.cn/problems/power-of-two/) |  |  |  |  |  |

# 综合示例

| 状态 | 题目 | C++ | Go | Py | Rust | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| ✅ | [⾼效处理海量数据（四维纵横面试前线下做的题，基本无时间限制）](Examples/BigData/README.md) |  |  |  |  | [BigData](Examples/BigData) |
| ☐ | [生成 1 亿行数据存文件，从该文件中找出最大值/最小值（四维纵横一面题，在面试官的监督下完成）](Examples/BigDataFile/README.md) |  |  |  |  | [BigDataFile](Examples/BigDataFile) |
| ✅ | [两阶段加锁实现高并发访问（成章数据面试前线下做的题，基本无时间限制）](Examples/MultiWorkers/README.md) |  |  |  |  | [MultiWorkers](Examples/MultiWorkers) |
| ✅ | [打车软件的订单调度系统（困难，华为机考）](HuaweiJikao/taxi-order-scheduling-system/taxi-order-scheduling-system.cpp) | [cpp](HuaweiJikao/taxi-order-scheduling-system/taxi-order-scheduling-system.cpp) |  |  |  |  |

# 其他

- [华为系公司机考部分试题合集，共 103 道](https://www.nowcoder.com/exam/oj/ta?tpId=37)
  - 本人参加华为机考时，抽到的两道题在该合集中都没有，可见这里只是部分试题

- 程序员大彬公众号刷题心得：
  - [LeetCode刷题手册.pdf](./Docs/pdf/LeetCode刷题手册.pdf)
  - [LeetCode 101 - A LeetCode Grinding Guide (C++ Version).pdf](./Docs/pdf/LeetCode-101_A-LeetCode-Grinding-Guide_C++_Version.pdf)
  - [LeetCode题解（java语言实现）.pdf](./Docs/pdf/LeetCode题解（java语言实现）.pdf)
