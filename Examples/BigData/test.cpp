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

#include <algorithm>
#include <cstdlib>
#include <ctime>

using namespace std;

long getTimeNs() {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);

    return ts.tv_sec * 1000000000 + ts.tv_nsec;
}

Row* generateRows(int rows_num) {
    Row* rows = new Row[rows_num];
    int div = (rows_num > 100000 ? rows_num : 100000);
    for (int i = 0; i < rows_num; i++) {
        rows[i].a = rand() % div / 100 * 100;
        rows[i].b = rand() % 100;
    }
    return rows;
}

void runTask(enum TaskNum num, Row* rows, int row_nums) {
    time_t start = getTimeNs();
    srand(time(0));
    printf("==== task%d ====\n", num);
    switch (num) {
        case TASK1:
            task1(rows, row_nums);  // 不适合海量数据
            break;
        case TASK2:
            task2(rows, row_nums);
            break;
        case TASK3:
            task3(rows, row_nums);
            break;
        case TASK4:
            task4(rows, row_nums);
            break;
        default:
            break;
    }
    printf("task%d takes %ld nano-seconds\n", num, getTimeNs() - start);
}
/*
task1 takes 27256931 nano-seconds
task2 takes 163552 nano-seconds
task3 takes 5082328 nano-seconds
task4 takes 5898344 nano-seconds
*/
int main(int argc, char** argv) {
    int row_nums = 1000 * 10000;  // 1千万行

    // task1 & task2
    {
        Row* rows = generateRows(row_nums);
        runTask(TASK1, rows, row_nums);
        sort(rows, rows + row_nums, compareAB);
        runTask(TASK2, rows, row_nums);
        free(rows);
    }

    // task3
    {
        Row* rows = generateRows(row_nums);
        sort(rows, rows + row_nums, compareAB);
        runTask(TASK3, rows, row_nums);
        free(rows);
    }

    // task4
    {
        Row* rows = generateRows(row_nums);
        sort(rows, rows + row_nums, compareAB);
        runTask(TASK4, rows, row_nums);
        free(rows);
    }

    return 0;
}
