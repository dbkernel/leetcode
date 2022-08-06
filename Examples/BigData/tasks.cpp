/*
 * Copyright (c) 2021-present, DBKernel (Wenshuang Lu). All rights reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * Maintainer: Wenshuang Lu (wenshuang_lu@163.com)
*/

#include "tasks.h"
#include "skiplist.h"

#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

/*!
 * Task 1.
 *
 * Find out all the rows that sastify below conditions:
 *
 * ((b >= 10 && b < 50) &&
 * (a == 1000 || a == 2000 || a == 3000))
 *
 * Print them to the terminal, one row per line, for example:
 *
 * 1000,20
 * 1000,23
 * 2000,16
 *
 * @param nrows The total number of rows.
 * @param rows The rows, for example rows[0] is the first row.
 */
void task1(const Row* rows, int nrows) {
    for (int i = 0; i < nrows; i++) {
        if ((rows[i].a == 1000 || rows[i].a == 2000 || rows[i].a == 3000) &&
            (rows[i].b >= 10 && rows[i].b < 50)) {
            cout << rows[i].a << ", " << rows[i].b << endl;
        }
    }
}

// 使用二分查找的方式找到第一个大于等于 target 的行
int binarySearch(const Row* rows, int nrows, const Row target) {
    if (rows == nullptr)
        return 0;

    int left = 0;
    int right = nrows - 1;
    int mid;
    while (left < right) {
        mid = (left + right) / 2;
        int res = compareAB(rows[mid], target);
        if (res)
            left = mid + 1;
        else
            right = mid;  // 表明 mid >= target
    }
    return right;
}

/*
 * Based on task 1, assume that the parameter rows has been sorted by (a,b).
 *
 * Input:
 * Row sorted_rows[] =
 * {
 *     { 1000, 31 },
 *     { 1000, 72 },
 *     { 1500, 12 },
 *     { 1500, 34 },
 *     { 2000, 22 },
 *     { 2000, 33 },
 * };
 *
 * Output:
 * 1000,31
 * 2000,22
 * 2000,33
 */
// 二分查找
void task2(const Row* rows, int nrows) {
    for (int i = 1; i < 4; i++) {
        Row min = {i * 1000, 10}, max = {i * 1000, 50};
        int minIndex = binarySearch(rows, nrows, min);
        int maxIndex = binarySearch(rows, nrows, max);
        while (minIndex < maxIndex) {
            cout << rows[minIndex].a << ", " << rows[minIndex].b << endl;
            minIndex++;
        }
    }
}

/*
 * Based on task 2, the printed matching rows are sorted by column b.
 *
 * Output:
 * 2000,22
 * 1000,31
 * 2000,33
 */
// 快速排序
void task3(Row* rows, int nrows) {
    int len = 100;

    for (int i = 1; i < len; i++) {
        Row min = {i * 1000, 10}, max = {i * 1000, 50};
        // 使用二分查找确定边界
        int minIndex = binarySearch(rows, nrows, min);
        int maxIndex = binarySearch(rows, nrows, max);

        // 由于内存可以放下，按快速排序方式对 b 列排序后输出结果集
        sort(rows + minIndex, rows + maxIndex, compareB);
        // sort(rows + minIndex, rows + maxIndex, compareBA);
        while (minIndex < maxIndex) {
            cout << rows[minIndex].a << ", " << rows[minIndex].b << endl;
            minIndex++;
        }
    }
}

/*
 * Based on task 3, assume that the match condition on column a is not just
 * 1000,2000,3000, but expanded to 1000,2000,3000... , 98000, 99000.
 */
// 跳表
// 实测在1亿行数据以下，效率并没有 task3 的快速排序高，我分析有两方面原因：
// 1. task3 仅需比较B列，而 task4
// 中所有跳表结构不允许重复元素，因此按（B，A）列排序，排序代价更高；
// 2. 跳表、红黑树、B+
// 树的优势在于查找速度会很快，但是构建、维护这些结构是有一些代价的。
void task4(Row* rows, int nrows) {
    int len = 100;
    int minIndexs[len], maxIndexs[len];

    skiplist* list = skiplistCreate(compareBAForRowNode);

    for (int i = 1; i < len; i++) {
        Row min = {i * 1000, 10}, max = {i * 1000, 50};
        // 使用二分查找确定边界
        int minIndex = binarySearch(rows, nrows, min);
        int maxIndex = binarySearch(rows, nrows, max);
        minIndexs[i] = minIndex;
        maxIndexs[i] = maxIndex;
        // printf("minIndexs[%d]=%d, maxIndexs[%d]=%d\n", i, minIndexs[i], i,
        // maxIndexs[i]);
        if (minIndex < maxIndex) {
            RowNode* node = new RowNode(i, rows + minIndex);
            skiplistInsert(
                list,
                node);  // 插入后按照 B
                        // 列升序构建跳表，如果元素已存在，则不再插入，因此，适合结果集唯一或需要去重的情况
        }
    }

    while (1) {
        RowNode* head = (RowNode*)skiplistPopHead(list);
        if (head == nullptr)
            break;
        cout << head->row->a << ", " << head->row->b << endl;
        int minIndex = ++minIndexs[head->index];  // 最小边界后移
        int maxIndex = maxIndexs[head->index];
        // printf("skiplistLength=%ld, minIndexs[%d]=%d, maxIndexs[%d]=%d\n",
        //        skiplistLength(list), head->index, minIndexs[head->index],
        //        head->index, maxIndexs[head->index]);
        if (minIndex < maxIndex) {
            RowNode* node = new RowNode(head->index, rows + minIndex);
            skiplistInsert(list, node);  // 插入后按照 B 列升序构建跳表
        }
        free(head);
    }

    skiplistFree(list);
}