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

#ifndef TASKS_H
#define TASKS_H

typedef struct Row {
    int a;
    int b;
} Row;

typedef struct RowNode {
    RowNode() {}
    RowNode(int index, const Row* row) : index(index), row(row) {}
    int index;
    const Row* row;
} RowNode;

enum TaskNum { TASK1 = 1, TASK2, TASK3, TASK4 };

// 升序：如果 left < right，返回 true
inline bool compareAB(Row left, Row right) {
    if (left.a < right.a || (left.a == right.a && left.b < right.b))
        return true;
    else
        return false;
}

// 升序
inline bool compareB(Row left, Row right) {
    return left.b < right.b;
}

// 升序
inline bool compareBA(Row left, Row right) {
    if (left.b < right.b || (left.b == right.b && left.a < right.a))
        return true;
    else
        return false;
}

// 升序
inline int compareBAForRowNode(const void* left, const void* right) {
    int res = ((RowNode*)left)->row->b - ((RowNode*)right)->row->b;
    if (res != 0)
        return res;
    else
        return ((RowNode*)left)->row->a - ((RowNode*)right)->row->a;
}

void task1(const Row* rows, int nrows);
void task2(const Row* rows, int nrows);
void task3(Row* rows, int nrows);
void task4(Row* rows, int nrows);

#endif