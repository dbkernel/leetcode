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

#include <cassert>

#include "lock.h"
#include "row.h"

using namespace std;

Row::Row() {
    rwlock = new ReadFirstRWLock();
    assert(rwlock);
}

Row::Row(int value, bool readFirst) {
    // 支持两种读写锁，一种是读优先，另一种是写优先
    if (readFirst)
        rwlock = new ReadFirstRWLock();
    else
        rwlock = new WriteFirstRWLock();
    this->value = value;
    assert(rwlock);
}

Row::~Row() {
    if (rwlock)
        delete rwlock;
}
