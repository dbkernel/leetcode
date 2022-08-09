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

#include "lock.h"

void ReadFirstRWLock::readLock() {
    readMtx.lock();
    if (++readCnt == 1) {
        writeMtx.lock();  // 存在线程读操作时，写加锁（只加一次）
    }
    readMtx.unlock();
}

void ReadFirstRWLock::readUnlock() {
    readMtx.lock();
    if (--readCnt == 0) {  // 没有线程读操作时，释放写锁
        writeMtx.unlock();
    }
    readMtx.unlock();
}

void ReadFirstRWLock::writeLock() {
    writeMtx.lock();
}

void ReadFirstRWLock::writeUnlock() {
    writeMtx.unlock();
}

void WriteFirstRWLock::readLock() {}

void WriteFirstRWLock::readUnlock() {}

void WriteFirstRWLock::writeLock() {}

void WriteFirstRWLock::writeUnlock() {}
