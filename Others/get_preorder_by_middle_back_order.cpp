/*
天翼云社招题：基于二叉树中序、后序遍历序列，求先序序列

输入：
后序： DGJHEBIFCA
中序： DBGEHJACIF

输出：
DGJHEBIFCA
*/
#include <iostream>
#include <string>
using namespace std;

/*
方法一：递归方式
*/
void getPreorder(string mid, string back) {
    int mlen = mid.length();
    int blen = back.length();
    if (blen <= 0)
        return;

    // 输出当前子树根节点
    char c = back[blen - 1];
    cout << c;

    // 得到当前根节点在中序序列中的位置
    int pos = mid.find(c);
    // 解析根节点左边序列
    getPreorder(mid.substr(0, pos), back.substr(0, pos));
    // 解析根节点右边序列
    getPreorder(mid.substr(pos + 1, mlen - pos - 1),
                back.substr(pos, blen - pos - 1));
}

int main() {
    string mid = "DBGEHJACIF";
    string back = "DGJHEBIFCA";
    getPreorder(mid, back);
    cout << endl;
    return 0;
}