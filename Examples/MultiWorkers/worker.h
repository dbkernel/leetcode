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

#ifndef __WORKER_H__
#define __WORKER_H__

#define ROWS_NUM 10000

#include <array>
#include <cassert>
#include <string>

#include "lock.h"
#include "row.h"

class Worker {
   public:
    Worker() {}
    Worker(std::string name, std::vector<Row*>* rows) {
        this->name = name;
        this->rows = rows;
    }
    ~Worker() {}

    void Run();

   public:
    std::string name;

   private:
    std::vector<Row*>* rows;
};

bool WorkersInit(std::vector<Worker*>& workers,
                 int workerNum,
                 std::vector<Row*>* rows);
void WorkersRun(std::vector<Worker*>& workers,
                std::vector<std::thread>& threads);
void WorkerWait(std::vector<std::thread>& threads);
void WorkersRelease(std::vector<Worker*>& workers);

#endif