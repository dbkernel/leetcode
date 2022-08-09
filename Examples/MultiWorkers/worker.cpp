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

#include "worker.h"
#include "lock.h"

#include <cstdlib>
#include <ctime>
#include <iostream>
#include <thread>
#include <vector>

using namespace std;

void Worker::Run() {
    srand((int)time(0));
    for (int n = 0; n < ROWS_NUM; ++n) {
        int i = rand() % ROWS_NUM;
        int j = rand() % ROWS_NUM;
        int r1 = (i + 1) % ROWS_NUM;
        int r2 = (i + 2) % ROWS_NUM;

        printf("%s    i=%05d, j=%05d\n", name.c_str(), i, j);

        rows[i]->rwlock->readLock();
        rows[r1]->rwlock->readLock();
        rows[r2]->rwlock->readLock();
        rows[j]->rwlock->writeLock();

        // TODO ==== 需要解决
        /*
        (d) j 有可能落在 [i, i+2] 区间。
        (e) 附加思考：会出现死锁吗？如何避免？
        */

        rows[j]->value = rows[i]->value + rows[r1]->value + rows[r2]->value;

        rows[i]->rwlock->readUnLock();
        rows[r1]->rwlock->readUnLock();
        rows[r2]->rwlock->readUnLock();
        rows[j]->rwlock->writeUnLock();
    }
}

bool WorkersInit(vector<Worker*>& workers, int workerNum, vector<Row*>* rows) {
    if (workerNum < 2)
        return false;
    for (int i = 0; i < workerNum; ++i) {
        char workerName[32];
        sprintf(workerName, "worker[%03d]", i);
        Worker* worker = new Worker(workerName, rows);
        assert(worker);
        workers.emplace_back(worker);
    }
    return true;
}

static bool workerRun(Worker* worker) {
    if (!worker)
        return false;
    cout << "====" << worker->name << "====" << endl;
    worker->Run();
    return true;
}

void WorkersRun(vector<Worker*>& workers, vector<thread>& threads) {
    for (int i = 0; i < workers.size(); ++i)
        threads.push_back(thread(workerRun, workers[i]));
}

void WorkerWait(vector<thread>& threads) {
    for_each(threads.begin(), threads.end(),
             mem_fn(&thread::join));  // 对每个线程调用join()
}

void WorkersRelease(vector<Worker*>& workers) {
    for (auto worker : workers)
        if (worker) {
            delete worker;
            worker = NULL;
        }
}
