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
#ifndef __LOCK_H__
#define __LOCK_H__

#include <mutex>
#include <shared_mutex>

class RWLock {
   protected:
    std::shared_mutex readMtx;
    std::mutex writeMtx;
    int readCnt = 0;  // 已加读锁个数

   public:
    RWLock() : readCnt(0) {}
    virtual ~RWLock() {}  // 必须定义抽象基类的虚析构函数

    virtual void readLock() = 0;
    virtual void readUnlock() = 0;
    virtual void writeLock() = 0;
    virtual void writeUnlock() = 0;
};

// 该类是用两个 mutex 实现的读写锁，策略是读优先，可能会导致写线程饿死。
// 另外，还有一种读写锁是使用 一个互斥锁 + 一个条件变量 来实现的，
// 虽然读写并不区分优先级，但也会导致写线程等待时间过长。
class ReadFirstRWLock : public RWLock {
   public:
    ReadFirstRWLock() {}
    ~ReadFirstRWLock() {}

    virtual void readLock();
    virtual void readUnlock();
    virtual void writeLock();
    virtual void writeUnlock();
};

// 该类是用 一个互斥锁 + 两个条件变量 实现的读写锁，这个版本是写优先的
class WriteFirstRWLock : public RWLock {
   public:
    WriteFirstRWLock() {}
    ~WriteFirstRWLock() {}

    virtual void readLock();
    virtual void readUnlock();
    virtual void writeLock();
    virtual void writeUnlock();
};

#endif