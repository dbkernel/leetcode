/*
题目描述：详见 https://www.nowcoder.com/practice/f9c6f980eeec43ef85be20755ddbeaf4
*/

#include <iostream>
#include <vector>
using namespace std;
int main() {
    int money_n, num_m;
    cin >> money_n >> num_m;
    money_n /= 10; // 除以10，为的是提高计算速度

    // 每行的三个值表示：主件 附件一 附件二。其中第0行是空出的
    // 以题中示例来说，最终数组内容如下。
    // price: [ [0,0,0],[800/10, 400/10, 300/10],[0,0,0],[0,0,0],[400/10,0,0],[500/10,0,0] ]
    // value: [ [0,0,0],[800*2/10, 400*5/10, 300*5/10],[0,0,0],[0,0,0],[400*3/10,0,0],[500*2/10,0,0] ]
    vector<vector<int>> price(num_m + 1, vector<int>(3, 0)); // 存放未加权的价值
    vector<vector<int>> value(num_m + 1, vector<int>(3, 0)); // 存放加权后的价值
    int v, p, q;  // 价格 重要度 编号

    // 以下先更新附件一，后更新附件二，因此，只有附件一不为空才会更新附件二。
    for (int i = 1; i <= num_m; ++i) {
        cin >> v >> p >> q;
        if (q == 0) {              // 主件
            price[i][0] = v / 10;  // 除以10，为的是提高计算速度
            value[i][0] = v * p / 10; // 除以10，为的是提高计算速度
        } else if (price[q][1] == 0) { // 附件一还未更新
            price[q][1] = v / 10;
            value[q][1] = v * p / 10;
        } else { // 附件二还未更新
            price[q][2] = v / 10;
            value[q][2] = v * p / 10;
        }
    }

    // 动态规划数组
    // money_n 除以 10，可以节省动态数组空间
    // 假定总钱数等于 1000，则 money_n 等于 100，则动态规划后只有下标为10的倍数时才有值
    //
    // 举例：dp[1][10]，表示花费 10（最终结果需要再乘以10）元，只买物品1及其部分或全部附件，能得到的满意度。
    // 如果买不起，满意度等于上一物品在同一价钱的情况下的满意度，即 dp[0][10] 。
    // 类似的，dp[2][10]，表示花费 10 元，尝试购买 物品1和物品2及二者部分或全部附件，能得到的满意度。
    vector<vector<int>> dp(num_m + 1, vector<int>(money_n + 1, 0));

    // 区分几种情况：
    // 只买主件；
    // 买主件+附件一；
    // 买主件+附件二；
    // 买主件+附件一+附件二
    for (int i = 1; i <= num_m; ++i) {
        for (int j = 1; j <= money_n; ++j) {
            if (j >= price[i][0]) // 只买主件
                dp[i][j] =
                        max(dp[i - 1][j - price[i][0]] + value[i][0],
                            dp[i - 1][j]); // 花费同样金额的情况下，最大重要度
            else
                dp[i][j] = dp[i - 1][j]; // 买不起

            if (j >= price[i][0] + price[i][1]) // 主件+附件一
                dp[i][j] = max(dp[i - 1][j - price[i][0] - price[i][1]] +
                               value[i][0] + value[i][1],
                               dp[i][j]); // 花费同样金额的情况下，最大重要度
            else
                dp[i][j] = dp[i][j]; // 买不起

            if (j >= price[i][0] + price[i][2]) // 主件+附件二
                dp[i][j] = max(dp[i - 1][j - price[i][0] - price[i][2]] +
                               value[i][0] + value[i][2],
                               dp[i][j]); // 花费同样金额的情况下，最大重要度
            else
                dp[i][j] = dp[i][j]; // 买不起

            if (j >=
                price[i][0] + price[i][1] + price[i][2]) //主件+附件一+附件二
                dp[i][j] =
                        max(dp[i - 1][j - price[i][0] - price[i][1] - price[i][2]] +
                            value[i][0] + value[i][1] + value[i][2],
                            dp[i][j]); // 花费同样金额的情况下，最大重要度
            else
                dp[i][j] = dp[i][j]; // 买不起
        }
    }

    // 尝试以 money_n（实际钱数需要乘以10）购买 num_m 号及前述物品（包括部分全部附件）时所得到的最大满意度。
    cout << dp[num_m][money_n] * 10 << endl;
}