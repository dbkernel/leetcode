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

// 100万行整数存文件
// 取出来求最大、最小值
// 1亿行效率
#include <algorithm>
#include <fstream>
#include <iostream>
using namespace std;

void generateRows(string filename, int len) {
    srand(time(0));
    ofstream outfile;
    outfile.open(filename, ios::app);
    for (int i = 0; i < len; i++) {
        outfile << rand() % 1000000 << endl;
    }
    outfile.close();
}

void getMinMaxSum(string filename, int len) {
    ifstream infile(filename);
    int arr[len];
    int64_t sum = 0;
    int min = -1, max = -1;

    for (int i = 0; i < len; i++) {
        infile >> arr[i];
        if (min == -1)
            min = arr[i];
        else
            min = (min > arr[i] ? arr[i] : min);

        if (max == -1)
            max = arr[i];
        else
            max = (max < arr[i] ? arr[i] : max);

        sum += arr[i];
    }

    printf("min=%d, max=%d, sum=%lld\n", min, max, sum);

    infile.close();
}

int main() {
    string filename = "/tmp/testdata";
    int len = 10000 * 10000;
    generateRows(filename, len);
    // getMinMaxSum(filename, len);
}
