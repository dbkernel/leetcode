/*
随着移动互联网的发展，人们的出行方式也变得越来越方便，相信很多人都用过手机打车软件，请你设计一个简单的订单调度系统，
当用户打车时能够计算出最快接到乘客的司机。

乘客叫车时，系统获取出当前区域内司机情况如下:
1. 乘客到各个司机的路程长度分别是D1、D2. .DN,单位是米。
2. 这些路程中可能存在拥堵路段，各司机路程中的拥堵长度分别是C1、C2 ... CN, 如果到某司机的路段不存在拥堵，则值为0。
3. 路程中还可能存在红绿灯路口，各司机路程经过的红绿灯的个数分别是L1、L2 ..... LN, 如果无红绿灯则为0，
   假设每个红绿灯路口碰到红灯的概率都为50%，每个红灯要等待15秒。
4. 所有司机在正常路段的行驶速度是10m/s,在拥堵路段的行驶速度是2m/s.
5. 各个司机评分级别分别是Q1、Q2、... QN. 评分等级反映服务质量，是1-10的整数。

调度系统需综合考虑接载速度和服务质量，假设通过以上数据计算得到最快接载司机是M司机，其用时为T，单位是秒，
如果在[T,T+60)时间范围内存在评分级别更高的司机，则安排该时间范围内评分更高的司机接载。

输入描述:
输入包括4行数据，格式说明如下:
- 第1行是乘客到各司机的路程长度，都是整数，用英文半角逗号分隔。第1个整数表示1号司机的距离，第2个整数表示2号司机的距离，以此类推
- 第2行是各司机路程中的拥堵路段长度，都是整数，用英文半角逗号分隔。与第一行对应，按照司机编号排列
- 第3行是各司机路程中红绿灯路口的个数，都是整数，用英文半角逗号分隔。与第一行对应，按照司机编号排列
- 第4行是各司机的评分等级，共有10级，分别用整数1到10表示，数字越大表示级别越高，服务质量越好。与第一行对应，按照司机编号排列

输出描述:
输出系统所计算出的接单的司机编号(司机编号从1开始)及其接载所用时长(单位是秒)，用逗号分隔.例如: 4,398

输入
1530,1760,2300,1990
240,320,0,0
2,1,0,0
8,10,10,9

输出
3,230
*/

#include <limits.h>
// #include <math.h>
// #include <stdio.h>
#include <algorithm>
#include <iostream>
// #include <map>
// #include <queue>
// #include <stack>
#include <string>
#include <vector>
using namespace std;

vector<int> get_input() {
    string s;
    cin >> s;

    int sum = 0;
    s = s + ",";
    vector<int> v;
    for (int i = 0; i < s.size(); i++) {
        if (s[i] != ',') {
            sum = sum * 10 + (s[i] - '0');
        } else {
            v.push_back(sum);
            sum = 0;
        }
    }

    return v;
}
bool cmp(vector<int> v1, vector<int> v2) {
    // 对应于本题，则优先比较司机耗时，当两个司机耗时相同时，再比较司机评分，整体以升序排序
    if (v1[0] != v2[0])
        return v1[0] < v2[0];
    else
        return v1[1] < v2[1];
}
int main() {
    vector<int> distance = get_input(); // 乘客到各个司机的路程长度，单位：米
    vector<int> busy = get_input(); // 各司机路程中的拥堵长度
    vector<int> light = get_input(); // 各司机路程经过的红绿灯的个数
    vector<int> quality = get_input(); // 各个司机评分级别
    int n = busy.size(); // 司机数量
    vector<vector<int> > time(n, vector<int>(3, 0)); // 二维数组
    int min_time = INT_MAX; // 临时记录最终输出的司机耗时
    int q; // 记录最终输出的司机评分等级
    int res_n; // 最终输出的司机序号

    for (int i = 0; i < n; i++) {
        time[i][0] = (busy[i]) * 1.0 / 2 + (distance[i] - busy[i]) * 1.0 / 10 +
                     light[i] * 15; // 司机耗时
        time[i][1] = quality[i]; // 司机评分级别
        time[i][2] = i; // 司机序号（实际序号需要 + 1）
        if (time[i][0] < min_time) { // 查找并记录耗时最少的司机信息
            min_time = time[i][0];
            q = time[i][1];
            res_n = time[i][2];
        }
    }

    // 按 <司机耗时，司机评分级别> 维度，以升序排序
    sort(time.begin(), time.end(), cmp);

    int res_time = min_time; // 记录最终输出的司机耗时
    // 如果在 [T,T+60) 时间范围内存在评分级别更高的司机，则安排评分更高的司机接载
    for (int i = 1; i < n; i++) {
        if (time[i][0] < min_time + 60) {
            if (time[i][1] > q) { // 是否有更高评分的司机
                res_time = time[i][0];
                res_n = time[i][2];
                q = time[i][1];
            }

        } else
            break;
    }

    cout << res_n + 1 << "," << res_time << endl; // 司机序号需要 + 1
}
