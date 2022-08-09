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

# 第一阶段（1 周）

> [详细速览版](Docs/stage01.md)

## 纯编程

- [x] [1.两数之和（简单）](https://leetcode-cn.com/problems/two-sum/)<br>
  - [x] [0001-two-sum.cpp](Problems/0001-0100/0001-two-sum/0001-two-sum.cpp)
- [x] [1108. IP 地址无效化（简单）](https://leetcode-cn.com/problems/defanging-an-ip-address/) <br>
  - [x] [1108-defanging-an-ip-address.cpp](Problems/1101-1200/1108-defanging-an-ip-address/1108-defanging-an-ip-address.cpp)
- [x] [344. 反转字符串（简单）](https://leetcode-cn.com/problems/reverse-string/)<br>
  - [x] [0344-reverse-string.cpp](Problems/0301-0400/0344-reverse-string/0344-reverse-string.cpp)
- [x] [剑指 Offer 58 - I. 翻转单词顺序（简单）](https://leetcode-cn.com/problems/fan-zhuan-dan-ci-shun-xu-lcof/) <br>
  - [x] [58-I-翻转单词顺序.cpp](JianZhiOffer/58-I-翻转单词顺序.cpp)
- [x] [125. 验证回文串（简单）](https://leetcode-cn.com/problems/valid-palindrome/) <br>
  - [x] [0125-valid-palindrome.cpp](Problems/0101-0200/0125-valid-palindrome/0125-valid-palindrome.cpp)
- [ ] [9. 回文数（简单）](https://leetcode-cn.com/problems/palindrome-number/)<br>
- [ ] [58. 最后一个单词的长度（简单）](https://leetcode-cn.com/problems/length-of-last-word/)<br>
- [ ] [剑指 Offer 05. 替换空格（简单）](https://leetcode-cn.com/problems/ti-huan-kong-ge-lcof/)<br>
- [ ] [剑指 Offer 58 - II. 左旋转字符串（简单）](https://leetcode-cn.com/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/)<br>
- [ ] [26. 删除排序数组中的重复项（简单）](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/)<br>
- [ ] [剑指 Offer 67. 把字符串转换成整数（中等）](https://leetcode-cn.com/problems/ba-zi-fu-chuan-zhuan-huan-cheng-zheng-shu-lcof/)经典 atoi()，注意范围越界处理<br>

## 找规律

- [ ] [面试题 01.08. 零矩阵 （简单）](https://leetcode-cn.com/problems/zero-matrix-lcci/)<br>
- [ ] [剑指 Offer 61. 扑克牌中的顺子(中等)](https://leetcode-cn.com/problems/bu-ke-pai-zhong-de-shun-zi-lcof/)<br>
- [ ] [面试题 16.11. 跳水板（简单）](https://leetcode-cn.com/problems/diving-board-lcci/)<br>
- [ ] [面试题 01.05. 一次编辑（中等）](https://leetcode-cn.com/problems/one-away-lcci/)<br>
- [ ] [面试题 16.15. 珠玑妙算 （中等）](https://leetcode-cn.com/problems/master-mind-lcci/)<br>
- [ ] [面试题 16.04. 井字游戏（中等）](https://leetcode-cn.com/problems/tic-tac-toe-lcci/)<br>
- [ ] [55. 跳跃游戏 （中等）](https://leetcode-cn.com/problems/jump-game/)<br>
- [ ] [48. 旋转图像 （中等）](https://leetcode-cn.com/problems/rotate-image/)<br>
- [ ] [54. 螺旋矩阵（中等，字节电商 22 届暑期实习，出处）](https://leetcode-cn.com/problems/spiral-matrix/)<br>
- [ ] [240. 搜索二维矩阵 II （中等）](https://leetcode-cn.com/problems/search-a-2d-matrix-ii/)<br>

# 第二阶段（1 周）

## 链表

- [x] [203. 移除链表元素 （简单）](https://leetcode-cn.com/problems/remove-linked-list-elements/)<br>
  - [0203-remove-linked-list-elements.cpp](Problems/0201-0300/0203-remove-linked-list-elements/0203-remove-linked-list-elements.cpp)
- [x] [876. 链表的中间结点（简单）](https://leetcode-cn.com/problems/middle-of-the-linked-list/)<br>
  - [0876-middle-of-the-linked-list.cpp](Problems/0801-0900/0876-middle-of-the-linked-list/0876-middle-of-the-linked-list.cpp)
- [x] [83. 删除排序链表中的重复元素（简单）](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/)<br>
  - [0083-remove-duplicates-from-sorted-list.cpp](Problems/0001-0100/0083-remove-duplicates-from-sorted-list/0083-remove-duplicates-from-sorted-list.cpp)
- [x] [2. 两数相加 （中等，腾讯笔试题）](https://leetcode-cn.com/problems/add-two-numbers/)<br>
  - [0002-two-numbers.cpp](Problems/0001-0100/0002-two-numbers/0002-two-numbers.cpp)
- [x] [剑指 Offer 25. 合并两个排序的链表 （中等）](https://leetcode-cn.com/problems/he-bing-liang-ge-pai-xu-de-lian-biao-lcof/)<br>
  - [25-合并两个排序的链表.cpp](JianZhiOffer/25-合并两个排序的链表.cpp)
- [ ] [369. 给单链表加一（中等）力扣会员](https://leetcode-cn.com/problems/plus-one-linked-list/)<br>

  > **题目：** 比如链表[6,8,1,9]，加 1 后，为[6,8,2,0]
  >
  > **思路：** 先翻转链表为[9,1,8,6]，就可以对 9 加 1，之后根据按照从左往右的顺序进位，直到无位可进，之后再次翻转链表。

- [x] [剑指 Offer 22. 链表中倒数第 k 个节点 （简单，百度笔试题）](https://leetcode-cn.com/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/)<br>
  - [22-链表中倒数第 k 个节点.cpp](JianZhiOffer/22-链表中倒数第k个节点.cpp)
- [x] [19. 删除链表的倒数第 N 个结点 （中等）](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)<br>
  - [0019-remove-nth-node-from-end-of-list.cpp](Problems/0001-0100/0019-remove-nth-node-from-end-of-list/0019-remove-nth-node-from-end-of-list.cpp)
- [x] [160. 相交链表（简单，新华智云笔试题）](https://leetcode-cn.com/problems/intersection-of-two-linked-lists/)`步骤简单，但思路清奇`<br>
  - [0160-intersection-of-two-linked-lists.cpp](Problems/0101-0200/0160-intersection-of-two-linked-lists/0160-intersection-of-two-linked-lists.cpp)
- [x] [141. 环形链表（简单） 判断链表中是否存在环 ](https://leetcode-cn.com/problems/linked-list-cycle/)<br>
  - [0141-linked-list-cycle.cpp](Problems/0101-0200/0141-linked-list-cycle/0141-linked-list-cycle.cpp)
- [x] [206. 反转链表 （中等，阿里云笔试题）](https://leetcode-cn.com/problems/reverse-linked-list/)`重点复习`<br>
  - [0206-reverse-linked-list.cpp](Problems/0201-0300/0206-reverse-linked-list/0206-reverse-linked-list.cpp)
- [x] [234. 回文链表 （中等，蚂蚁金服笔试题）](https://leetcode-cn.com/problems/palindrome-linked-list/)`重点复习`<br>
  - [0234-palindrome-linked-list.cpp](Problems/0201-0300/0234-palindrome-linked-list/0234-palindrome-linked-list.cpp)
- [x] [328. 奇偶链表（中等，腾讯笔试题](https://leetcode-cn.com/problems/odd-even-linked-list/)`典型`<br>
  - [0328-odd-even-linked-list.cpp](Problems/0301-0400/0328-odd-even-linked-list/0328-odd-even-linked-list.cpp)
- [x] [25. K 个一组翻转链表（困难，腾讯笔试题）](https://leetcode-cn.com/problems/reverse-nodes-in-k-group/)`很好，但并不难`<br>
  - [0025-reverse-nodes-in-k-group.cpp](Problems/0001-0100/0025-reverse-nodes-in-k-group/0025-reverse-nodes-in-k-group.cpp)

## 栈

- [x] [剑指 Offer 09. 用两个栈实现队列（简单）](https://leetcode-cn.com/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/)<br>
  - [09-用两个栈实现队列.cpp](./JianZhiOffer/09-用两个栈实现队列.cpp)
- [x] [225. 用队列实现栈（简单，字节实习，出处）](https://leetcode-cn.com/problems/implement-stack-using-queues/)<br>
  - [0225-implement-stack-using-queues.cpp](Problems/0201-0300/0225-implement-stack-using-queues/0225-implement-stack-using-queues.cpp)
- [x] [面试题 03.01. 三合一（简单）](https://leetcode-cn.com/problems/three-in-one-lcci/)<br>
  - [03.01-three-in-one-lcci.cpp](InterviewQuestions/03.01-three-in-one-lcci.cpp)
- [x] [20. 有效的括号（简单）](https://leetcode-cn.com/problems/valid-parentheses/)<br>
  - [0020-valid-parentheses.cpp](Problems/0001-0100/0020-valid-parentheses/0020-valid-parentheses.cpp)
- [x] [面试题 16.26. 计算器（中等）](https://leetcode-cn.com/problems/calculator-lcci/)<br>
  - [16.26-calculator-lcci.cpp](InterviewQuestions/16.26-calculator-lcci.cpp)
- [ ] [772. 基本计算器 III（困难 包含括号）力扣会员](https://leetcode-cn.com/problems/basic-calculator-iii/)<br>
- [x] [1047. 删除字符串中的所有相邻重复项（简单）](https://leetcode-cn.com/problems/remove-all-adjacent-duplicates-in-string/)<br>
  - [1047-remove-all-adjacent-duplicates-in-string.cpp](Problems/1001-1100/1047-remove-all-adjacent-duplicates-in-string/1047-remove-all-adjacent-duplicates-in-string.cpp)
- [x] [面试题 03.05. 栈排序（中等）](https://leetcode-cn.com/problems/sort-of-stacks-lcci/)<br>
  - [03.05-sort-of-stacks-lcci.cpp](InterviewQuestions/03.05-sort-of-stacks-lcci.cpp)
- [x] [155. 最小栈（简单）](https://leetcode-cn.com/problems/min-stack/)<br>
  - [0155-min-stack.cpp](Problems/0101-0200/0155-min-stack/0155-min-stack.cpp)
- [x] [剑指 Offer 31. 栈的压入、弹出序列（中等）](https://leetcode-cn.com/problems/zhan-de-ya-ru-dan-chu-xu-lie-lcof/)<br>
  - [31-栈的压入、弹出序列.cpp](JianZhiOffer/31-栈的压入、弹出序列.cpp)

**附加题：**

- [ ] [739. 每日温度（中等）单调栈](https://leetcode-cn.com/problems/daily-temperatures/)<br>
- [ ] [42. 接雨水（困难）单调栈](https://leetcode-cn.com/problems/trapping-rain-water/)<br>
- [ ] [84. 柱状图中最大的矩形（困难）单调栈](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/)<br>
- [ ] [面试题 03.06. 动物收容所（中等） 队列](https://leetcode-cn.com/problems/animal-shelter-lcci/)<br>
- [ ] [剑指 Offer 59 - II. 队列的最大值（中等） 单调队列](https://leetcode-cn.com/problems/dui-lie-de-zui-da-zhi-lcof/)<br>
- [ ] [剑指 Offer 59 - I. 滑动窗口的最大值 （困难）单调队列](https://leetcode-cn.com/problems/hua-dong-chuang-kou-de-zui-da-zhi-lcof/)<br>

# 第三阶段（1 周）

## 递归 + 分治

- [x] [剑指 Offer 10- I. 斐波那契数列 （简单）](https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof/)`矩阵方式比较难理解`<br>
  - [10-1-斐波那契数列.cpp](JianZhiOffer/10-1-斐波那契数列.cpp)
- [x] [剑指 Offer 10- II. 青蛙跳台阶问题（简单）](https://leetcode-cn.com/problems/qing-wa-tiao-tai-jie-wen-ti-lcof/)`类似斐波那契`<br>
  - [10-2-青蛙跳台阶问题.cpp](JianZhiOffer/10-2-青蛙跳台阶问题.cpp)
- [ ] [面试题 08.01. 三步问题 （简单）](https://leetcode-cn.com/problems/three-steps-problem-lcci/)<br>
- [ ] [剑指 Offer 06. 从尾到头打印链表 （简单）](https://leetcode-cn.com/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/)<br>
- [ ] [剑指 Offer 24. 反转链表 （简单）](https://leetcode-cn.com/problems/fan-zhuan-lian-biao-lcof/)<br>
- [ ] [剑指 Offer 16. 数值的整数次方 （中等）](https://leetcode-cn.com/problems/shu-zhi-de-zheng-shu-ci-fang-lcof/)<br>
- [ ] [面试题 08.05. 递归乘法 （中等）](https://leetcode-cn.com/problems/recursive-mulitply-lcci/)<br>

## 排序

- [x] [面试题 10.01. 合并排序的数组 （简单）](https://leetcode-cn.com/problems/sorted-merge-lcci/)`非常巧妙`<br>
  - [10.01-sorted-merge-lcci.cpp](InterviewQuestions/10.01-sorted-merge-lcci.cpp)
- [x] [21. 合并两个有序链表 （简单）](https://leetcode-cn.com/problems/merge-two-sorted-lists/)<br>
  - [与剑指 Offer 25-合并两个排序的链表.cpp 一致](JianZhiOffer/25-合并两个排序的链表.cpp)
  - [0021-merge-two-sorted-lists.cpp](Problems/0001-0100/0021-merge-two-sorted-lists/0021-merge-two-sorted-lists.cpp)
- [x] [242. 有效的字母异位词（简单）](https://leetcode-cn.com/problems/valid-anagram/)`需要字母数组来计数`<br>
  - [0242-valid-anagram.cpp](Problems/0201-0300/0242-valid-anagram/0242-valid-anagram.cpp)
- [x] [1502. 判断能否形成等差数列（简单）](https://leetcode-cn.com/problems/can-make-arithmetic-progression-from-sequence/)先排序后判断<br>
  - [1502-can-make-arithmetic-progression-from-sequence.cpp](Problems/1501-1600/1502-can-make-arithmetic-progression-from-sequence/1502-can-make-arithmetic-progression-from-sequence.cpp)
- [ ] [252. 会议室 （简单）力扣会员](https://leetcode-cn.com/problems/meeting-rooms/)<br>
- [x] [56. 合并区间（中等） ](https://leetcode-cn.com/problems/merge-intervals/)`先按区间左值排序，再判断是否需要合并`<br>
  - [0056-merge-intervals.cpp](Problems/0001-0100/0056-merge-intervals/0056-merge-intervals.cpp)
- [x] [剑指 Offer 21. 调整数组顺序使奇数位于偶数前面 （简单）](https://leetcode-cn.com/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof/)`首尾双指针法，非常巧妙`<br>
  - [21-调整数组顺序使奇数位于偶数前面.cpp](JianZhiOffer/21-调整数组顺序使奇数位于偶数前面.cpp)
- [x] [75. 颜色分类（中等）](https://leetcode-cn.com/problems/sort-colors/)`首尾双指针法，非常巧妙`**【精品】**<br>
  - [0075-sort-colors.cpp](Problems/0001-0100/0075-sort-colors/0075-sort-colors.cpp)
- [ ] [147. 对链表进行插入排序（中等）](https://leetcode-cn.com/problems/insertion-sort-list/)<br>
- [ ] [148. 排序链表（中等） 链表上的归并排序](https://leetcode-cn.com/problems/sort-list/)<br>
- [x] [215. 数组中的第 K 个最大元素（中等） ](https://leetcode-cn.com/problems/kth-largest-element-in-an-array/)`快速排序，非常巧妙`**【精品】**<br>
  - [0215-kth-largest-element-in-an-array.cpp](Problems/0201-0300/0215-kth-largest-element-in-an-array/0215-kth-largest-element-in-an-array.cpp)
- [x] [面试题 17.14. 最小 K 个数 （中等）](https://leetcode-cn.com/problems/smallest-k-lcci/)`快速排序，非常巧妙，与上一题类似。不过还有个堆的方法`**【精品】**<br>
  - [17.14-smallest-k-lcci.cpp](InterviewQuestions/17.14-smallest-k-lcci.cpp)
- [ ] [剑指 Offer 51. 数组中的逆序对（困难）](https://leetcode-cn.com/problems/shu-zu-zhong-de-ni-xu-dui-lcof/)<br>
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

- [x] [704. 二分查找（简单） 标准二分查找](https://leetcode-cn.com/problems/binary-search/)<br>
  - [0704-binary-search.cpp](Problems/0701-0800/0704-binary-search/0704-binary-search.cpp)
- [x] [374. 猜数字大小（简单）](https://leetcode-cn.com/problems/guess-number-higher-or-lower/)<br>
  - [0374-guess-number-higher-or-lower.cpp](Problems/0301-0400/0374-guess-number-higher-or-lower/0374-guess-number-higher-or-lower.cpp)
- [ ] [744. 寻找比目标字母大的最小字母（简单）](https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target/)<br>
- [ ] [35. 搜索插入位置（简单）](https://leetcode-cn.com/problems/search-insert-position/)<br>
- [ ] [34. 在排序数组中查找元素的第一个和最后一个位置 （中等）](https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/)<br>
- [ ] [面试题 10.05. 稀疏数组搜索（简单） ](https://leetcode-cn.com/problems/sparse-array-search-lcci/)<br>
- [ ] [33. 搜索旋转排序数组（中等）无重复数据 ](https://leetcode-cn.com/problems/search-in-rotated-sorted-array/)<br>
- [ ] [81. 搜索旋转排序数组 II（中等）有重复数据](https://leetcode-cn.com/problems/search-in-rotated-sorted-array-ii/)<br>
- [ ] [153. 寻找旋转排序数组中的最小值（中等） 无重复数据](https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/)<br>
- [ ] [154. 寻找旋转排序数组中的最小值 II （困难） 有重复数据](https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array-ii/)<br>
- [ ] [852. 山脉数组的峰顶索引（简单）峰值二分](https://leetcode-cn.com/problems/peak-index-in-a-mountain-array/)<br>
- [ ] [162. 寻找峰值（中等）峰值二分](https://leetcode-cn.com/problems/find-peak-element/)<br>
- [ ] [367. 有效的完全平方数（简单）二分答案](https://leetcode-cn.com/problems/valid-perfect-square/)<br>
- [ ] [69. x 的平方根（简单）二分答案](https://leetcode-cn.com/problems/sqrtx/)<br>
- [ ] [875. 爱吃香蕉的珂珂（中等）二分答案](https://leetcode-cn.com/problems/koko-eating-bananas/)<br>
- [ ] [658. 找到 K 个最接近的元素（中等）](https://leetcode-cn.com/problems/find-k-closest-elements/)<br>
- [ ] [74. 搜索二维矩阵（中等） 二维转一维，二分查找](https://leetcode-cn.com/problems/search-a-2d-matrix/)<br>

## 哈希表

- [x] [两数之和 （简单） 两数之和](https://leetcode-cn.com/problems/two-sum/)`unordered_map<int, int> hashtable 存储数值及下表`**【精品】**<br>
  - [0001-two-sum.cpp](Problems/0001-0100/0001-two-sum/0001-two-sum.cpp)
- [ ] [15. 三数之和（中等）三数之和](https://leetcode-cn.com/problems/3sum/)<br>
- [x] [160. 相交链表（简单，新华智云 22 届暑期实习） ](https://leetcode-cn.com/problems/intersection-of-two-linked-lists/)`双指针最好，哈希集合 unordered_set 空间复杂度高`<br>
  - [0160-intersection-of-two-linked-lists.cpp](Problems/0101-0200/0160-intersection-of-two-linked-lists/0160-intersection-of-two-linked-lists.cpp)
- [ ] [141. 环形链表（简单） 判断链表中是否存在环 ](https://leetcode-cn.com/problems/linked-list-cycle/)<br>
- [ ] [面试题 02.01. 移除重复节点（中等） 借助哈希表](https://leetcode-cn.com/problems/remove-duplicate-node-lcci/)<br>
- [ ] [面试题 16.02. 单词频率 （简单）](https://leetcode-cn.com/problems/words-frequency-lcci/)<br>
- [ ] [面试题 01.02. 判定是否互为字符重排（简单）](https://leetcode-cn.com/problems/check-permutation-lcci/)<br>
- [ ] [剑指 Offer 03. 数组中重复的数字 （简单） ](https://leetcode-cn.com/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof/)<br>
- [ ] [242. 有效的字母异位词（简单）](https://leetcode-cn.com/problems/valid-anagram/)<br>
- [ ] [49. 字母异位词分组（中等）](https://leetcode-cn.com/problems/group-anagrams/)<br>
- [ ] [136. 只出现一次的数字（简单）](https://leetcode-cn.com/problems/single-number/)<br>
- [ ] [349. 两个数组的交集 （简单）](https://leetcode-cn.com/problems/intersection-of-two-arrays/)<br>
- [ ] [1122. 数组的相对排序（中等）](https://leetcode-cn.com/problems/relative-sort-array/)<br>
- [ ] [706. 设计哈希映射（简单）](https://leetcode-cn.com/problems/design-hashmap/)<br>
- [ ] [146. LRU 缓存机制 （中等）标准的 LRU](https://leetcode-cn.com/problems/lru-cache/)<br>
- [ ] [面试题 16.21. 交换和（中等） 找规律，可以借助哈希表优化](https://leetcode-cn.com/problems/sum-swap-lcci/)<br>

# 第五阶段（2 周）

## 二叉树

### 题型 1：二叉树前中后序遍历

- [x] [144. 二叉树的前序遍历（简单）](https://leetcode-cn.com/problems/binary-tree-preorder-traversal/)`递归方式简单，迭代方式需要引入辅助栈和一个临时指针，Morris方式有些难`<br>
  - [0144-binary-tree-preorder-traversal.cpp](Problems/0101-0200/0144-binary-tree-preorder-traversal/0144-binary-tree-preorder-traversal.cpp)
- [x] [94. 二叉树的中序遍历 （简单）](https://leetcode-cn.com/problems/binary-tree-inorder-traversal/)`递归方式简单，迭代方式需要引入辅助栈和一个临时指针，Morris方式有些难`<br>
  - [0094-binary-tree-inorder-traversal.cpp](Problems/0001-0100/0094-binary-tree-inorder-traversal/0094-binary-tree-inorder-traversal.cpp)
- [x] [145. 二叉树的后序遍历（简单）](https://leetcode-cn.com/problems/binary-tree-postorder-traversal/)`递归方式简单，迭代方式需要引入辅助栈和两个临时指针，比前序、中序要难一些，Morris方式更难`<br>
  - [0145-binary-tree-postorder-traversal.cpp](Problems/0101-0200/0145-binary-tree-postorder-traversal/0145-binary-tree-postorder-traversal.cpp)
- [x] [589. N 叉树的前序遍历（简单）](https://leetcode-cn.com/problems/n-ary-tree-preorder-traversal/)`递归方式简单，迭代方式需要引入辅助栈`<br>
  - [0589-n-ary-tree-preorder-traversal.cpp](Problems/0501-0600/0589-n-ary-tree-preorder-traversal/0589-n-ary-tree-preorder-traversal.cpp)
- [x] N(>2) 叉树没法做中序遍历
- [x] [590. N 叉树的后序遍历（简单）](https://leetcode-cn.com/problems/n-ary-tree-postorder-traversal/)`递归方式简单，迭代方式需要引入辅助栈，很巧妙`**【精品】**<br>
  - [0590-n-ary-tree-postorder-traversal.cpp](Problems/0501-0600/0590-n-ary-tree-postorder-traversal/0590-n-ary-tree-postorder-traversal.cpp)

### 题型 2：二叉树按层遍历

- [ ] [剑指 Offer 32 - I. 从上到下打印二叉树（中等） ](https://leetcode-cn.com/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/)<br>
- [ ] [102. 二叉树的层序遍历（中等） ](https://leetcode-cn.com/problems/binary-tree-level-order-traversal/)<br>
- [ ] [剑指 Offer 32 - III. 从上到下打印二叉树 III （中等）](https://leetcode-cn.com/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/)<br>
- [ ] [429. N 叉树的层序遍历（中等）](https://leetcode-cn.com/problems/n-ary-tree-level-order-traversal/)<br>
- [ ] [513. 找树左下角的值（中等）](https://leetcode-cn.com/problems/find-bottom-left-tree-value/)<br>

### 题型 3：二叉树上的递归

- [ ] [104. 二叉树的最大深度（简单） ](https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/)<br>
- [ ] [543. 二叉树的直径（简单）](https://leetcode-cn.com/problems/diameter-of-binary-tree/)<br>
- [ ] [559. N 叉树的最大深度（简单）](https://leetcode-cn.com/problems/maximum-depth-of-n-ary-tree/)<br>
- [ ] [剑指 Offer 55 - II. 平衡二叉树（中等）](https://leetcode-cn.com/problems/ping-heng-er-cha-shu-lcof/)<br>
- [ ] [617. 合并二叉树（简单）](https://leetcode-cn.com/problems/merge-two-binary-trees/)<br>
- [ ] [226. 翻转二叉树 （简单）](https://leetcode-cn.com/problems/invert-binary-tree/)<br>
- [ ] [101. 对称二叉树（中等）](https://leetcode-cn.com/problems/symmetric-tree/)<br>
- [ ] [98. 验证二叉搜索树（中等）](https://leetcode-cn.com/problems/validate-binary-search-tree/)<br>

### 题型 4：二叉查找树

- [ ] [剑指 Offer 54. 二叉搜索树的第 k 大节点（中等）](https://leetcode-cn.com/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/)<br>
- [ ] [538. 把二叉搜索树转换为累加树 （中等）](https://leetcode-cn.com/problems/convert-bst-to-greater-tree/)<br>
- [ ] [面试题 04.06. 后继者（中等）](https://leetcode-cn.com/problems/successor-lcci/)<br>

### 题型 5：LCA 最近公共祖先

- [ ] [236. 二叉树的最近公共祖先（中等） ](https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-tree/)<br>
- [ ] [剑指 Offer 68 - I. 二叉搜索树的最近公共祖先（中等） ](https://leetcode-cn.com/problems/er-cha-sou-suo-shu-de-zui-jin-gong-gong-zu-xian-lcof/)<br>

### 题型 6：二叉树转单、双、循环链表

- [ ] [114. 二叉树展开为链表（中等）](https://leetcode-cn.com/problems/flatten-binary-tree-to-linked-list/)<br>
- [ ] [面试题 17.12. BiNode（中等）](https://leetcode-cn.com/problems/binode-lcci/)<br>
- [ ] [剑指 Offer 36. 二叉搜索树与双向链表（中等）](https://leetcode-cn.com/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/)<br>
- [ ] [面试题 04.03. 特定深度节点链表（中等）](https://leetcode-cn.com/problems/list-of-depth-lcci/)<br>

### 题型 7：按照遍历结果反向构建二叉树

- [ ] [105. 从前序与中序遍历序列构造二叉树（中等）](https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)<br>
- [ ] [889. 根据前序和后序遍历构造二叉树（中等）](https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/)<br>
- [ ] [106. 从中序与后序遍历序列构造二叉树（中等）](https://leetcode-cn.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)<br>
- [ ] [剑指 Offer 33. 二叉搜索树的后序遍历序列（中等）](https://leetcode-cn.com/problems/er-cha-sou-suo-shu-de-hou-xu-bian-li-xu-lie-lcof/)<br>

### 题型 8：二叉树上的最长路径和

- [ ] [剑指 Offer 34. 二叉树中和为某一值的路径（中等）](https://leetcode-cn.com/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/)<br>
- [ ] [124. 二叉树中的最大路径和 （困难）](https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/)<br>
- [ ] [437. 路径总和 III （困难）](https://leetcode-cn.com/problems/path-sum-iii/)<br>

# 第六阶段（1 周）

## 堆

- [ ] [23. 合并 K 个升序链表(困难) ](https://leetcode-cn.com/problems/merge-k-sorted-lists/)<br>
- [ ] [347. 前 K 个高频元素（中等） ](https://leetcode-cn.com/problems/top-k-frequent-elements/)<br>
- [ ] [973. 最接近原点的 K 个点（中等）](https://leetcode-cn.com/problems/k-closest-points-to-origin/)<br>
- [ ] [295. 数据流的中位数（困难）](https://leetcode-cn.com/problems/find-median-from-data-stream/)<br>
- [ ] [313. 超级丑数（中等）](https://leetcode-cn.com/problems/super-ugly-number/)<br>

## Trie 树

- [ ] [208. 实现 Trie (前缀树) （中等） 标准 Trie 树](https://leetcode-cn.com/problems/implement-trie-prefix-tree/)<br>
- [ ] 以下为选做题目：<br>
- [ ] [面试题 17.17. 多次搜索（中等） 标准 AC 自动机，不过写 AC 自动机太复杂，Trie 树搞定](https://leetcode-cn.com/problems/multi-search-lcci/)<br>
- [ ] [212. 单词搜索 II（困难）](https://leetcode-cn.com/problems/word-search-ii/)<br>

# 第七阶段（1 周）

## 回溯

- [ ] [面试题 08.12. 八皇后（困难）](https://leetcode-cn.com/problems/eight-queens-lcci/)<br>
- [ ] [51. N 皇后（困难）](https://leetcode-cn.com/problems/n-queens/)<br>
- [ ] [37. 解数独](https://leetcode-cn.com/problems/sudoku-solver/)<br>
- [ ] [17. 电话号码的字母组合 （中等）](https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/)<br>
- [ ] [77. 组合 （中等） 给 n 个数返回所有 k 个数的组合](https://leetcode-cn.com/problems/combinations/)<br>
- [ ] [78. 子集（中等） 所有的组合](https://leetcode-cn.com/problems/subsets/)<br>
- [ ] [90. 子集 II（中等）有重复数据](https://leetcode-cn.com/problems/subsets-ii/)<br>
- [ ] [46. 全排列 （中等） 所有排列](https://leetcode-cn.com/problems/permutations/)<br>
- [ ] [47. 全排列 II（中等） 有重复数据](https://leetcode-cn.com/problems/permutations-ii/)<br>
- [ ] [39. 组合总和 （中等） 选出某几个数相加为给定和，无重复数据，可以使用多次，不能有重复答案](https://leetcode-cn.com/problems/combination-sum/)<br>
- [ ] [40. 组合总和 II（中等）选出某几个数相加为给定和，有重复数据，只能使用一次，不能有重复答案](https://leetcode-cn.com/problems/combination-sum-ii/)<br>
- [ ] [216. 组合总和 III（中等） 选出 k 个数相加为给定和，没有重复数据，只能使用一次](https://leetcode-cn.com/problems/combination-sum-iii/)<br>
- [ ] [131. 分割回文串（中等）](https://leetcode-cn.com/problems/palindrome-partitioning/)<br>
- [ ] [93. 复原 IP 地址（中等）](https://leetcode-cn.com/problems/restore-ip-addresses/)<br>
- [ ] [22. 括号生成（中等） ](https://leetcode-cn.com/problems/generate-parentheses/)<br>

## DFS

- [ ] [剑指 Offer 13. 机器人的运动范围（中等）](https://leetcode-cn.com/problems/ji-qi-ren-de-yun-dong-fan-wei-lcof/)<br>
- [ ] [面试题 08.10. 颜色填充（简单）](https://leetcode-cn.com/problems/color-fill-lcci/)<br>
- [ ] [面试题 04.01. 节点间通路（中等）](https://leetcode-cn.com/problems/route-between-nodes-lcci/)<br>
- [ ] [200. 岛屿数量（中等）](https://leetcode-cn.com/problems/number-of-islands/)<br>
- [ ] [面试题 16.19. 水域大小（中等）](https://leetcode-cn.com/problems/pond-sizes-lcci/)<br>
- [ ] [207. 课程表（中等）](https://leetcode-cn.com/problems/course-schedule/)<br>
- [ ] [79. 单词搜索（中等）](https://leetcode-cn.com/problems/word-search/)<br>
- [ ] [1306. 跳跃游戏 III（中等）](https://leetcode-cn.com/problems/jump-game-iii/)<br>
- [ ] [752. 打开转盘锁（中等）](https://leetcode-cn.com/problems/open-the-lock/)<br>

个人补充（不一定是放当前板块）：

- [ ] [207. 课程表（中等）](https://leetcode.cn/problems/course-schedule/)<br>
- [ ] [210. 课程表 II（中等）](https://leetcode.cn/problems/course-schedule-ii/)<br>
- [ ] [630. 课程表 III（困难）](https://leetcode.cn/problems/course-schedule-iii/)<br>
- [ ] [1462. 课程表 IV（中等）](https://leetcode.cn/problems/course-schedule-iv/)<br>
- [ ] [1136. 并行课程（力扣会员）](https://leetcode.cn/problems/parallel-courses/)<br>
- [ ] [1494. 并行课程 II（困难）](https://leetcode.cn/problems/parallel-courses-ii/)<br>
- [ ] [2050. 并行课程 III（困难）](https://leetcode.cn/problems/parallel-courses-iii/)<br>

**附加题：**

- [ ] [面试题 17.22. 单词转换（困难） ](https://leetcode-cn.com/problems/word-transformer-lcci/)<br>
- [ ] [面试题 17.07. 婴儿名字（困难） ](https://leetcode-cn.com/problems/baby-names-lcci/)<br>
- [ ] [529. 扫雷游戏（困难）](https://leetcode-cn.com/problems/minesweeper/)<br>
- [ ] [127. 单词接龙（困难）](https://leetcode-cn.com/problems/word-ladder/)<br>
- [ ] [126. 单词接龙 II（困难）](https://leetcode-cn.com/problems/word-ladder-ii/)<br>

# 第八阶段（1 周）

## 背包：

- [ ] [416. 分割等和子集](https://leetcode-cn.com/problems/partition-equal-subset-sum/)<br>
- [ ] [494. 目标和](https://leetcode-cn.com/problems/target-sum/)<br>
- [ ] [279. 完全平方数](https://leetcode-cn.com/problems/perfect-squares/)<br>
- [ ] [322. 零钱兑换 ](https://leetcode-cn.com/problems/coin-change/)<br>
- [ ] [518. 零钱兑换 II](https://leetcode-cn.com/problems/coin-change-2/)<br>
- [ ] [377. 组合总和 Ⅳ](https://leetcode-cn.com/problems/combination-sum-iv/)<br>

## 打家劫舍：

- [ ] [198. 打家劫舍](https://leetcode-cn.com/problems/house-robber/)<br>
- [ ] [337. 打家劫舍 III ](https://leetcode-cn.com/problems/house-robber-iii/)<br>
- [ ] [面试题 17.16. 按摩师](https://leetcode-cn.com/problems/the-masseuse-lcci/)<br>

## 买卖股票：

- [ ] [121. 买卖股票的最佳时机](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/)<br>
- [ ] [122. 买卖股票的最佳时机 II](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii/)<br>
- [ ] [714. 买卖股票的最佳时机含手续](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/)<br>
- [ ] [309. 最佳买卖股票时机含冷冻期](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)<br>
- [ ] [300. 最长递增子序列](https://leetcode-cn.com/problems/longest-increasing-subsequence/)<br>
- [ ] [62. 不同路径](https://leetcode-cn.com/problems/unique-paths/)<br>
- [ ] [64. 最小路径和](https://leetcode-cn.com/problems/minimum-path-sum/)<br>
- [ ] [剑指 Offer 47. 礼物的最大价值](https://leetcode-cn.com/problems/li-wu-de-zui-da-jie-zhi-lcof/) <br>
- [ ] [1143. 最长公共子序列](https://leetcode-cn.com/problems/longest-common-subsequence/)<br>
- [ ] [72. 编辑距离 ](https://leetcode-cn.com/problems/edit-distance/)<br>
- [ ] [10. 正则表达式匹配](https://leetcode-cn.com/problems/regular-expression-matching/)<br>
- [ ] [剑指 Offer 14- I. 剪绳子](https://leetcode-cn.com/problems/jian-sheng-zi-lcof/)<br>
- [ ] [剑指 Offer 46. 把数字翻译成字符串](https://leetcode-cn.com/problems/ba-shu-zi-fan-yi-cheng-zi-fu-chuan-lcof/)<br>
- [ ] [139. 单词拆分](https://leetcode-cn.com/problems/word-break/)<br>
- [ ] [437. 路径总和 III](https://leetcode-cn.com/problems/path-sum-iii/)<br>

# 第九阶段（1 周）

## 双指针

- [ ] [344. 反转字符串](https://leetcode-cn.com/problems/reverse-string/)<br>
- [ ] [面试题 16.24. 数对和（例题 1）](https://leetcode-cn.com/problems/pairs-with-sum-lcci/)<br>
- [ ] [1. 两数之和](https://leetcode-cn.com/problems/two-sum/)<br>
- [ ] [15. 三数之和](https://leetcode-cn.com/problems/3sum/)<br>
- [ ] [剑指 Offer 21. 调整数组顺序使奇数位于偶数前面 ](https://leetcode-cn.com/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof/)<br>
- [ ] [75. 颜色分类 ](https://leetcode-cn.com/problems/sort-colors/)<br>
- [ ] [283. 移动零 已排序未排序指针（例题 2）](https://leetcode-cn.com/problems/move-zeroes/)<br>
- [ ] [面试题 16.06. 最小差 类似合并两个有序数组（例题 3）](https://leetcode-cn.com/problems/smallest-difference-lcci/)<br>
- [ ] [面试题 17.11. 单词距离 类似合并两个有序数组](https://leetcode-cn.com/problems/find-closest-lcci/)<br>

## 滑动窗口

- [ ] [剑指 Offer 57 - II. 和为 s 的连续正数序列 （例题 1）](https://leetcode-cn.com/problems/he-wei-sde-lian-xu-zheng-shu-xu-lie-lcof/)<br>
- [ ] [剑指 Offer 48. 最长不含重复字符的子字符串 （例题 2）](https://leetcode-cn.com/problems/zui-chang-bu-han-zhong-fu-zi-fu-de-zi-zi-fu-chuan-lcof/)<br>
- [ ] [438. 找到字符串中所有字母异位词 ](https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/)<br>
- [ ] [76. 最小覆盖子串 ](https://leetcode-cn.com/problems/minimum-window-substring/)<br>

## 前缀后缀统计

- [ ] [53. 最大子序和](https://leetcode-cn.com/problems/maximum-subarray/)<br>
- [ ] [238. 除自身以外数组的乘积](https://leetcode-cn.com/problems/product-of-array-except-self/)<br>
- [ ] [面试题 05.03. 翻转数位](https://leetcode-cn.com/problems/reverse-bits-lcci/)<br>
- [ ] [42. 接雨水](https://leetcode-cn.com/problems/trapping-rain-water/)<br>

## 位运算

- [ ] [191. 位 1 的个数 （例题 1）](https://leetcode-cn.com/problems/number-of-1-bits/)<br>
- [ ] [461. 汉明距离（例题 2）](https://leetcode-cn.com/problems/hamming-distance/)<br>
- [ ] [面试题 05.06. 整数转换](https://leetcode-cn.com/problems/convert-integer-lcci/)<br>
- [ ] [面试题 05.07. 配对交换 ](https://leetcode-cn.com/problems/exchange-lcci/)<br>
- [ ] [面试题 05.01. 插入 ](https://leetcode-cn.com/problems/insert-into-bits-lcci/)<br>
- [ ] [面试题 17.04. 消失的数字](https://leetcode-cn.com/problems/missing-number-lcci/)<br>
- [ ] [剑指 Offer 56 - I. 数组中数字出现的次数](https://leetcode-cn.com/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-lcof/)<br>
- [ ] [剑指 Offer 56 - II. 数组中数字出现的次数 II ](https://leetcode-cn.com/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-ii-lcof/)<br>
- [ ] [面试题 16.01. 交换数字](https://leetcode-cn.com/problems/swap-numbers-lcci/)<br>
- [ ] [231. 2 的幂](https://leetcode-cn.com/problems/power-of-two/)<br>

# 综合示例

- [x] [⾼效处理海量数据,](Examples/BigData/README.md)
  - [x] [BigData](Examples/BigData)
- [ ] [生成 1 亿行数据存文件，从该文件中找出最大值/最小值](Examples/BigDataFile/README.md)
  - [ ] [BigDataFile](Examples/BigDataFile)
- [x] [两阶段加锁实现高并发访问](Examples/MultiWorkers/README.md)
  - [x] [MultiWorkers](Examples/MultiWorkers)
